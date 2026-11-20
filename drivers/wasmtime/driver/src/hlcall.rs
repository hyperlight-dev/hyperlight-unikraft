// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The driver side of `/dev/hlcall`, as `drivers/hl_driver.h`,
//! `hl_fc.h` and `hl_env.h` have it for the C drivers: open the device,
//! size the call buffer, read one call at a time, report a failure, and
//! refresh the environment the host set.  See docs/driver.md.

use std::ffi::CString;
use std::io;

const DEVICE: &str = "/dev/hlcall";

/// `_IOR('H', 1, uint64_t)` and `_IOWR('H', 2, struct hlcall_env)`: the
/// argument's size is part of the number, so these must track
/// `hl_driver.h` (and the kernel's `step.h`) exactly.
const fn ioc(dir: u64, nr: u64, size: u64) -> u64 {
    (dir << 30) | (size << 16) | ((b'H' as u64) << 8) | nr
}
const IOC_READ: u64 = 2;
const IOC_READ_WRITE: u64 = 3;
const HLCALL_IOC_MAXLEN: u64 = ioc(IOC_READ, 1, 8);
const HLCALL_IOC_GETENV: u64 = ioc(IOC_READ_WRITE, 2, size_of::<EnvArg>() as u64);
const HLCALL_IOC_HOSTCALL: u64 = ioc(IOC_READ_WRITE, 3, size_of::<HostCallArg>() as u64);

/// The host library's HostCall reply: a tag byte, then the result or the
/// error message (src/lib.rs, `dispatch_host_call`).
const HOST_CALL_OK: u8 = 0;

#[repr(C)]
struct HostCallArg {
    name: *const u8,
    name_len: u64,
    args: *const u8,
    args_len: u64,
    out: *mut u8,
    out_cap: u64,
    out_len: u64,
}

/// The device, for host calls made from inside Wasmtime (a host function
/// the guest imports), which cannot borrow the `Device` serving the call.
static DEVICE_FD: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);
static CALL_CAP: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Call the host function the embedder registered as `name` with `args`
/// (JSON), during a call: `Ok` with its result, `Err` with its error, or
/// with why the call could not be made.
pub fn host_call(name: &str, args: &str) -> Result<String, String> {
    // One call-sized buffer for every reply, as hl_driver.h keeps: a host
    // function a module calls in a loop should cost a VM exit, not an
    // allocation as well.
    thread_local! {
        static REPLY: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    REPLY.with_borrow_mut(|out| host_call_into(name, args, out))
}

fn host_call_into(name: &str, args: &str, out: &mut Vec<u8>) -> Result<String, String> {
    use std::sync::atomic::Ordering::Relaxed;
    let fd = DEVICE_FD.load(Relaxed);
    out.resize(CALL_CAP.load(Relaxed), 0);
    let mut arg = HostCallArg {
        name: name.as_ptr(),
        name_len: name.len() as u64,
        args: args.as_ptr(),
        args_len: args.len() as u64,
        out: out.as_mut_ptr(),
        out_cap: out.len() as u64,
        out_len: 0,
    };
    // SAFETY: the kernel reads the name and args and writes at most
    // `out_cap` bytes into `out`, all live for the call.
    if unsafe { libc::ioctl(fd, HLCALL_IOC_HOSTCALL as _, &mut arg) } < 0 {
        return Err(io::Error::last_os_error().to_string());
    }
    let (tag, text) = out[..arg.out_len as usize]
        .split_first()
        .ok_or_else(|| "an empty reply".to_string())?;
    let text = String::from_utf8_lossy(text).into_owned();
    if *tag == HOST_CALL_OK {
        Ok(text)
    } else {
        Err(text)
    }
}

#[repr(C)]
struct EnvArg {
    buf: *mut u8,
    cap: u64,
    len: u64,
}

/// The host's environment for a call: `KEY=VALUE` pairs.
pub type Env = Vec<(String, String)>;

/// The open call queue and a buffer as large as the largest call.
pub struct Device {
    fd: libc::c_int,
    call: Vec<u8>,
    env: Vec<u8>,
}

impl Device {
    pub fn open() -> io::Result<Device> {
        let path = CString::new(DEVICE).unwrap();
        // SAFETY: a valid NUL-terminated path; the fd is owned from here.
        let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDWR | libc::O_CLOEXEC) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let mut cap: u64 = 0;
        // SAFETY: MAXLEN writes one u64 through the pointer.
        if unsafe { libc::ioctl(fd, HLCALL_IOC_MAXLEN as _, &mut cap) } < 0 {
            return Err(io::Error::last_os_error());
        }
        if cap == 0 {
            return Err(io::Error::other("the kernel reports no call queue"));
        }
        DEVICE_FD.store(fd, std::sync::atomic::Ordering::Relaxed);
        CALL_CAP.store(cap as usize, std::sync::atomic::Ordering::Relaxed);
        Ok(Device {
            fd,
            call: vec![0; cap as usize],
            env: Vec::new(),
        })
    }

    /// Block until the host issues a call, and return it with the host's
    /// environment as it is for that call (read once the call is in: the
    /// host may have changed it since the last one).  Reading again is
    /// what completes the previous call.
    pub fn next_call(&mut self) -> io::Result<(&[u8], Env)> {
        let n = loop {
            // SAFETY: reads at most call.len() bytes into the buffer.
            let n = unsafe { libc::read(self.fd, self.call.as_mut_ptr().cast(), self.call.len()) };
            if n < 0 {
                let e = io::Error::last_os_error();
                if e.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(e);
            }
            if n > 0 {
                break n as usize;
            }
        };
        if self.env.is_empty() {
            self.env = vec![0; self.call.len()];
        }
        let env = host_env(self.fd, &mut self.env);
        Ok((&self.call[..n], env))
    }

    /// Report how the call being served went: its status, and what it
    /// returned.  A success with no result needs no report.
    pub fn finish(&self, status: i32, result: Option<&[u8]>) {
        // A failed call reports its status alone: a result is not its.
        let result = if status == 0 { result } else { None };
        if status == 0 && result.is_none() {
            return;
        }
        // The status and the result as two iovecs, which the kernel
        // gathers: no copy of the result.
        let status_bytes = status.to_ne_bytes();
        let result = result.unwrap_or(&[]);
        let iov = [
            libc::iovec {
                iov_base: status_bytes.as_ptr() as *mut _,
                iov_len: status_bytes.len(),
            },
            libc::iovec {
                iov_base: result.as_ptr() as *mut _,
                iov_len: result.len(),
            },
        ];
        // SAFETY: both iovecs point at live buffers of the lengths given.
        if unsafe { libc::writev(self.fd, iov.as_ptr(), iov.len() as _) } < 0 {
            eprintln!(
                "hl_wasmtimedriver: cannot report the call: {}",
                io::Error::last_os_error()
            );
            // A result the kernel refused (too large to send) must not
            // pass for no result: fail the call.
            if !result.is_empty() {
                let failed = (-1i32).to_ne_bytes();
                // SAFETY: as above.
                unsafe { libc::write(self.fd, failed.as_ptr().cast(), failed.len()) };
            }
        }
    }
}

/// The host's environment as `KEY=VALUE` pairs, read into `buf`; empty on
/// a kernel without the ioctl.
fn host_env(fd: libc::c_int, buf: &mut [u8]) -> Env {
    let mut arg = EnvArg {
        buf: buf.as_mut_ptr(),
        cap: buf.len() as u64,
        len: 0,
    };
    // SAFETY: the kernel writes at most `cap` bytes into `buf`.
    if unsafe { libc::ioctl(fd, HLCALL_IOC_GETENV as _, &mut arg) } < 0 {
        return Vec::new();
    }
    buf[..arg.len as usize]
        .split(|b| *b == 0)
        .filter_map(|entry| {
            let (k, v) = std::str::from_utf8(entry).ok()?.split_once('=')?;
            (!k.is_empty()).then(|| (k.to_string(), v.to_string()))
        })
        .collect()
}

#[path = "call.rs"]
mod call;
pub use call::Call;

/// Split a guest command line on whitespace, as `hl_split_ws` does
/// (urunc's `strings.Fields`).
pub fn split_ws(line: &str) -> Vec<String> {
    line.split_whitespace().map(str::to_string).collect()
}
