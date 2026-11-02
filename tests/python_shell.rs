// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;

use common::{require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, SandboxBuilder};

#[test]
fn python_shell_hello() {
    let rootfs = require_rootfs("python-shell");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/hello.py");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello from the Hyperlight agent"),
        "expected agent hello.py to run on python-shell, got: {output:?}",
    );
}

#[test]
fn python_shell_ssl_available() {
    let rootfs = require_rootfs("python-shell");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
import ssl
print(f'ssl={ssl.OPENSSL_VERSION}')
print('ca-certs', ssl.create_default_context().cert_store_stats()['x509_ca'] > 0)
import sqlite3
print('sqlite3-ok')
import ctypes
print('ctypes-ok')
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("ssl="),
        "SSL not available in python-shell: {output:?}"
    );
    assert!(
        output.contains("ca-certs True"),
        "OpenSSL's default verify paths hold no CA in python-shell: {output:?}"
    );
    assert!(
        output.contains("sqlite3-ok"),
        "sqlite3 not available in python-shell: {output:?}"
    );
    assert!(
        output.contains("ctypes-ok"),
        "ctypes not available in python-shell: {output:?}"
    );
}

#[test]
fn python_shell_shell_subprocess() {
    let rootfs = require_rootfs("python-shell");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/shell_commands.py");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello from hush shell!"),
        "expected shell_commands.py to work on python-shell, got: {output:?}",
    );
}

#[test]
fn python_shell_no_numpy() {
    let rootfs = require_rootfs("python-shell");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    // python-shell should NOT have numpy — it's the slim rootfs
    sandbox
        .run(
            r#"
try:
    import numpy
    print('numpy-found')
except ImportError:
    print('numpy-missing-ok')
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("numpy-missing-ok"),
        "python-shell should NOT have numpy, got: {output:?}",
    );
}

#[test]
fn python_shell_snapshot_round_trip() {
    let rootfs = require_rootfs("python-shell");
    let snap_dir = temp_dir("python-shell-snap");

    // Save
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    // Restore + run hello.py from snapshot
    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/hello.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello from the Hyperlight agent"),
        "expected hello.py to work after python-shell snapshot restore, got: {output:?}",
    );
}

/// A pipe and a `$(...)` through `subprocess.run(..., shell=True)`: hush
/// starts each process with vfork, whose child runs on its parent's TLS.
#[test]
fn python_shell_subprocess_pipe() {
    let rootfs = require_rootfs("python-shell");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
import subprocess
r = subprocess.run('echo hello | tr a-z A-Z; echo "n=$(echo abc | wc -c)"', shell=True, capture_output=True, text=True)
print(repr(r.stdout), r.returncode)
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("'HELLO\\nn=4\\n' 0"),
        "expected the pipe and substitution to run, got: {output:?}",
    );

    // A program that fails to execute, then a blocking call: the vfork
    // child that failed must not have freed the parent's stack.
    sandbox
        .run("import subprocess, time\ntry:\n    subprocess.run(['/nope/x'])\nexcept OSError:\n    print('oserror')\ntime.sleep(0.2)\nprint('after')")
        .unwrap();
    assert!(sandbox.drain_output().ends_with("oserror\nafter\n"));

    // From a thread, whose stack its caller made: the child's program
    // still gets a stack of its own.
    sandbox
        .run("import subprocess, threading\nr = []\nt = threading.Thread(target=lambda: r.append(subprocess.run(['echo', 'from-thread'], capture_output=True, text=True).stdout))\nt.start()\nt.join()\nprint(repr(r))")
        .unwrap();
    assert!(sandbox.drain_output().ends_with("['from-thread\\n']\n"));
}

/// Where hluk maps the initrd (`INITRD_MAP_BASE` in src/lib.rs); the
/// kernel maps it at the same virtual address.
#[cfg(target_arch = "x86_64")]
const INITRD_MAP_BASE: u64 = 0xFEF0_0000;
#[cfg(target_arch = "aarch64")]
const INITRD_MAP_BASE: u64 = 0x1_0000_0000;

/// An mmap() hint that would cover part of the initrd (running into it
/// from below, or at its first, middle or last page) is moved clear of
/// it, and the files extracted from the initrd, which borrow its memory,
/// read the same after the new mappings are written.  HotSpot's 4
/// GiB-aligned hints would otherwise land on the arm64 initrd, at 4 GiB,
/// and replace the pages under libzip.so.
#[test]
fn python_shell_mmap_hint_avoids_the_initrd() {
    let rootfs = require_rootfs("python-shell");
    let len = std::fs::metadata(&rootfs).unwrap().len();
    // A hint below the initrd whose 16 MiB run into it, then its first,
    // middle and last pages.
    let hints = [INITRD_MAP_BASE - (8 << 20)]
        .into_iter()
        .chain([0, len / 2, len - 1].map(|off| INITRD_MAP_BASE + (off & !0xfff)))
        .collect::<Vec<_>>();
    // The kernel reserves whole 4 KiB pages.
    let initrd_end = (INITRD_MAP_BASE + len).next_multiple_of(0x1000);
    // Free in every guest: below the heap (16 GiB) and far past the initrd.
    let control: u64 = 0x2_0000_0000;
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(format!(
            r#"
import ctypes, hashlib, os

def digest():
    h = hashlib.sha256()
    for root, dirs, files in os.walk('/'):
        if root == '/':
            dirs[:] = [d for d in dirs if d not in ('dev', 'proc', 'sys', 'tmp')]
        dirs.sort()
        for name in sorted(files):
            path = os.path.join(root, name)
            if os.path.isfile(path) and not os.path.islink(path):
                h.update(f'{{path}} {{os.path.getsize(path)}}'.encode())
                with open(path, 'rb') as f:
                    while chunk := f.read(1 << 20):
                        h.update(chunk)
    return h.hexdigest()

before = digest()
libc = ctypes.CDLL(None)
libc.mmap.restype = ctypes.c_void_p
libc.mmap.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int,
                      ctypes.c_int, ctypes.c_int, ctypes.c_long]
libc.munmap.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
size = 16 << 20

def clear(addr):
    return addr >= {initrd_end:#x} or addr + size <= {INITRD_MAP_BASE:#x}

def mapped_at(hint):
    # PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS
    addr = libc.mmap(hint, size, 3, 0x22, -1, 0)
    if addr in (None, 2**64 - 1):
        raise SystemExit(f'mmap({{hint:#x}}) failed')
    print(f'hint {{hint:#x}} got {{addr:#x}}')
    # Writing or unmapping a mapping on the initrd would bring the guest
    # down before the test could say so.
    if clear(addr):
        ctypes.memset(addr, 0xa5, size)
        libc.munmap(addr, size)
    return addr

# A hint clear of the initrd is taken as given.
honoured = mapped_at({control:#x}) == {control:#x}
moved = all([clear(mapped_at(hint)) for hint in {hints:?}])
print('honoured', honoured, 'moved', moved, 'files', digest() == before)
"#
        ))
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("honoured True moved True files True"),
        "expected the hints moved off the initrd and its files intact, got: {output:?}",
    );
}
