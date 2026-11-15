// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! What the fuzz targets share: the checks every host function's reply
//! must pass, and calls of any shape, as a guest can make them.

use hyperlight_common::flatbuffer_wrappers::function_types::{
    ParameterType, ParameterValue, ReturnValue,
};
use hyperlight_unikraft::fuzzing::{HOST_CALL_MAX, HostCalls};
use libfuzzer_sys::arbitrary::{self, Arbitrary};

/// Linux's errno values run from 1 (`EPERM`) to 133 (`EHWPOISON`).
fn is_errno(code: i32) -> bool {
    (1..=133).contains(&code)
}

/// A status or count: zero or more, or a Linux `-errno`.
pub fn check_status(name: &str, status: i32) {
    assert!(
        status >= 0 || is_errno(-status),
        "{name}: {status} is neither a count nor a Linux -errno"
    );
}

/// The four-byte status a `Vec<u8>` reply starts with, checked as one; a
/// reply too short to have one, or longer than a host call carries, fails.
pub fn reply_status(name: &str, reply: &[u8]) -> i32 {
    assert!(
        reply.len() <= HOST_CALL_MAX,
        "{name}: a {}-byte reply is more than a host call carries",
        reply.len()
    );
    let status = i32::from_le_bytes(
        reply
            .get(..4)
            .unwrap_or_else(|| panic!("{name}: a {}-byte reply has no status", reply.len()))
            .try_into()
            .unwrap(),
    );
    check_status(name, status);
    status
}

/// A reply of `-errno` alone, as every failed `Vec<u8>` reply but
/// `fs_list`'s is.
pub fn check_error_reply(name: &str, reply: &[u8], status: i32) {
    assert!(status < 0);
    assert_eq!(
        reply.len(),
        4,
        "{name}: an error reply has more than its status"
    );
}

/// Call host function `name` with arguments of its own types, which must
/// succeed.
pub fn call(calls: &HostCalls, name: &str, args: Vec<ParameterValue>) -> ReturnValue {
    calls
        .call(name, args)
        .unwrap_or_else(|e| panic!("{name}: a call of its own types failed: {e}"))
}

/// The `i32` an `i32` host function returned.
pub fn int(name: &str, value: ReturnValue) -> i32 {
    match value {
        ReturnValue::Int(v) => {
            check_status(name, v);
            v
        }
        other => panic!("{name}: returned {other:?}, not an i32"),
    }
}

/// The bytes a `Vec<u8>` host function returned.
pub fn bytes(name: &str, value: ReturnValue) -> Vec<u8> {
    match value {
        ReturnValue::VecBytes(v) => v,
        other => panic!("{name}: returned {other:?}, not bytes"),
    }
}

/// A host call of any shape: one of the host functions, or another name,
/// with any arguments, as native code in the guest can make it.
#[derive(Arbitrary, Debug)]
pub struct RawCall {
    /// One of the host functions, by its place among their sorted names, or
    /// a name of no host function.
    function: Result<u8, String>,
    args: Vec<ParameterValue>,
}

impl RawCall {
    /// Make the call: it fails, before any handler runs, unless it names a
    /// host function with arguments of its parameter types, and then it
    /// succeeds, as the host functions answer every failure with an errno.
    /// Returns the name called, the arguments and the reply when it was
    /// made.
    pub fn call(self, calls: &HostCalls) -> Option<(String, Vec<ParameterValue>, ReturnValue)> {
        let name = match self.function {
            Ok(i) => {
                let mut names: Vec<&str> = calls.names().collect();
                names.sort_unstable();
                names[usize::from(i) % names.len()].to_string()
            }
            Err(name) => name,
        };
        let types: Vec<ParameterType> = self.args.iter().map(ParameterType::from).collect();
        let fits = calls
            .parameter_types(&name)
            .is_some_and(|expected| expected == types.as_slice());
        match calls.call(&name, self.args.clone()) {
            Ok(reply) => {
                assert!(
                    fits,
                    "{name}: called with {types:?}, which it does not take"
                );
                Some((name, self.args, reply))
            }
            Err(e) => {
                assert!(!fits, "{name}: a call of the right shape failed: {e}");
                None
            }
        }
    }
}

/// The number of file descriptors the process has open: what a run must
/// leave as it found it.
#[cfg(unix)]
pub fn open_fds() -> usize {
    // /proc/self/fd on Linux; macOS has only /dev/fd.  Either lists the
    // descriptor the listing itself is read through, every time.
    std::fs::read_dir("/dev/fd")
        .expect("list the open file descriptors")
        .count()
}
