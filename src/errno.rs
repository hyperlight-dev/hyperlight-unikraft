// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Linux `errno` values for the guest.
//!
//! The guest is a Linux-ABI unikernel: every error a host function
//! reports to it must be a *Linux* errno number, whatever the host OS.
//! On Linux the host's own errno already is that number and passes
//! through untouched.  On Windows the raw OS code is translated: Win32
//! and Winsock codes first, then `io::ErrorKind` as the fallback.
//!
//! Callers get the errno number itself (`ECONNREFUSED` is 111) and
//! negate it on the wire, the usual `-errno` convention.  Only the
//! constants the translation and the host functions use are defined;
//! a few exist only for the Windows table.

use std::io;

pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const E2BIG: i32 = 7;
pub const EBADF: i32 = 9;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const EMFILE: i32 = 24;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDEADLK: i32 = 35;
pub const ENOTEMPTY: i32 = 39;
pub const ENOPROTOOPT: i32 = 92;
pub const EPROTONOSUPPORT: i32 = 93;
pub const EOPNOTSUPP: i32 = 95;
pub const EAFNOSUPPORT: i32 = 97;
pub const EADDRINUSE: i32 = 98;
pub const EADDRNOTAVAIL: i32 = 99;
pub const ENETDOWN: i32 = 100;
pub const ENETUNREACH: i32 = 101;
pub const ECONNABORTED: i32 = 103;
pub const ECONNRESET: i32 = 104;
pub const ENOTCONN: i32 = 107;
pub const ETIMEDOUT: i32 = 110;
pub const ECONNREFUSED: i32 = 111;
pub const EHOSTUNREACH: i32 = 113;
pub const ESTALE: i32 = 116;
pub const EDQUOT: i32 = 122;

// Only the Windows translation table (and one unit test) needs these.
#[cfg(windows)]
pub const EPERM: i32 = 1;
#[cfg(windows)]
pub const EFAULT: i32 = 14;
#[cfg(windows)]
pub const ENAMETOOLONG: i32 = 36;
#[cfg(windows)]
pub const ELOOP: i32 = 40;
#[cfg(windows)]
pub const ENOTSOCK: i32 = 88;
#[cfg(windows)]
pub const EDESTADDRREQ: i32 = 89;
#[cfg(windows)]
pub const EMSGSIZE: i32 = 90;
#[cfg(windows)]
pub const EPROTOTYPE: i32 = 91;
#[cfg(windows)]
pub const ESOCKTNOSUPPORT: i32 = 94;
#[cfg(windows)]
pub const EPFNOSUPPORT: i32 = 96;
#[cfg(windows)]
pub const ENETRESET: i32 = 102;
#[cfg(windows)]
pub const ENOBUFS: i32 = 105;
#[cfg(any(windows, test))]
pub const EISCONN: i32 = 106;
#[cfg(windows)]
pub const EHOSTDOWN: i32 = 112;
#[cfg(windows)]
pub const EALREADY: i32 = 114;
#[cfg(windows)]
pub const EINPROGRESS: i32 = 115;

/// Linux errno for a host I/O error.
pub fn from_io(e: &io::Error) -> i32 {
    if let Some(raw) = e.raw_os_error() {
        // A Linux host's errno *is* the guest ABI.
        #[cfg(target_os = "linux")]
        return raw;

        #[cfg(windows)]
        if let Some(code) = from_win32(raw) {
            return code;
        }
    }
    from_kind(e.kind())
}

/// Linux errno for a rustix error.
pub fn from_rustix(e: rustix::io::Errno) -> i32 {
    from_io(&io::Error::from(e))
}

/// Closest Linux errno for an `io::ErrorKind`: the portable fallback
/// for errors without a raw OS code, or with one we don't know.
fn from_kind(kind: io::ErrorKind) -> i32 {
    use io::ErrorKind as K;
    match kind {
        K::NotFound => ENOENT,
        K::PermissionDenied => EACCES,
        K::ConnectionRefused => ECONNREFUSED,
        K::ConnectionReset => ECONNRESET,
        K::HostUnreachable => EHOSTUNREACH,
        K::NetworkUnreachable => ENETUNREACH,
        K::ConnectionAborted => ECONNABORTED,
        K::NotConnected => ENOTCONN,
        K::AddrInUse => EADDRINUSE,
        K::AddrNotAvailable => EADDRNOTAVAIL,
        K::NetworkDown => ENETDOWN,
        K::BrokenPipe => EPIPE,
        K::AlreadyExists => EEXIST,
        K::WouldBlock => EAGAIN,
        K::NotADirectory => ENOTDIR,
        K::IsADirectory => EISDIR,
        K::DirectoryNotEmpty => ENOTEMPTY,
        K::ReadOnlyFilesystem => EROFS,
        K::StaleNetworkFileHandle => ESTALE,
        K::InvalidInput | K::InvalidData | K::InvalidFilename => EINVAL,
        K::TimedOut => ETIMEDOUT,
        K::StorageFull => ENOSPC,
        K::NotSeekable => ESPIPE,
        K::QuotaExceeded => EDQUOT,
        K::FileTooLarge => EFBIG,
        K::ResourceBusy => EBUSY,
        K::ExecutableFileBusy => ETXTBSY,
        K::Deadlock => EDEADLK,
        K::CrossesDevices => EXDEV,
        K::TooManyLinks => EMLINK,
        K::ArgumentListTooLong => E2BIG,
        K::Interrupted => EINTR,
        K::Unsupported => EOPNOTSUPP,
        K::OutOfMemory => ENOMEM,
        _ => EIO,
    }
}

/// Win32 and Winsock codes with an exact Linux equivalent.
#[cfg(windows)]
fn from_win32(code: i32) -> Option<i32> {
    Some(match code {
        // ── Win32 (file system) ──────────────────────────────────
        2 | 3 => ENOENT,     // FILE_NOT_FOUND, PATH_NOT_FOUND
        4 => EMFILE,         // TOO_MANY_OPEN_FILES
        5 => EACCES,         // ACCESS_DENIED
        6 => EBADF,          // INVALID_HANDLE
        8 | 14 => ENOMEM,    // NOT_ENOUGH_MEMORY, OUTOFMEMORY
        17 => EXDEV,         // NOT_SAME_DEVICE
        19 => EROFS,         // WRITE_PROTECT
        32 | 33 => EBUSY,    // SHARING_VIOLATION, LOCK_VIOLATION
        39 | 112 => ENOSPC,  // HANDLE_DISK_FULL, DISK_FULL
        80 | 183 => EEXIST,  // FILE_EXISTS, ALREADY_EXISTS
        87 | 123 => EINVAL,  // INVALID_PARAMETER, INVALID_NAME
        145 => ENOTEMPTY,    // DIR_NOT_EMPTY
        206 => ENAMETOOLONG, // file name too long
        267 => ENOTDIR,      // DIRECTORY
        1142 => ELOOP,       // TOO_MANY_LINKS (cap-std's symlink-loop limit)
        1314 => EPERM,       // PRIVILEGE_NOT_HELD (symlink without privilege)
        1921 => ELOOP,       // CANT_RESOLVE_FILENAME
        4390 => EINVAL,      // NOT_A_REPARSE_POINT (readlink on a non-link)
        // ── Winsock ─────────────────────────────────────────────
        10004 => EINTR,
        10009 => EBADF,
        10013 => EACCES,
        10014 => EFAULT,
        10022 => EINVAL,
        10024 => EMFILE,
        10035 => EAGAIN,
        10036 => EINPROGRESS,
        10037 => EALREADY,
        10038 => ENOTSOCK,
        10039 => EDESTADDRREQ,
        10040 => EMSGSIZE,
        10041 => EPROTOTYPE,
        10042 => ENOPROTOOPT,
        10043 => EPROTONOSUPPORT,
        10044 => ESOCKTNOSUPPORT,
        10045 => EOPNOTSUPP,
        10046 => EPFNOSUPPORT,
        10047 => EAFNOSUPPORT,
        10048 => EADDRINUSE,
        10049 => EADDRNOTAVAIL,
        10050 => ENETDOWN,
        10051 => ENETUNREACH,
        10052 => ENETRESET,
        10053 => ECONNABORTED,
        10054 => ECONNRESET,
        10055 => ENOBUFS,
        10056 => EISCONN,
        10057 => ENOTCONN,
        10058 => EPIPE, // WSAESHUTDOWN: Linux reports send-after-shutdown as EPIPE
        10060 => ETIMEDOUT,
        10061 => ECONNREFUSED,
        10062 => ELOOP,
        10063 => ENAMETOOLONG,
        10064 => EHOSTDOWN,
        10065 => EHOSTUNREACH,
        10066 => ENOTEMPTY,
        10069 => EDQUOT,
        10070 => ESTALE,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_fallback_covers_common_cases() {
        let e = io::Error::new(io::ErrorKind::NotFound, "x");
        assert_eq!(from_io(&e), ENOENT);
        let e = io::Error::new(io::ErrorKind::ConnectionRefused, "x");
        assert_eq!(from_io(&e), ECONNREFUSED);
        let e = io::Error::new(io::ErrorKind::WouldBlock, "x");
        assert_eq!(from_io(&e), EAGAIN);
        let e = io::Error::other("x");
        assert_eq!(from_io(&e), EIO);
    }

    #[test]
    fn rustix_errno_maps_to_linux_values() {
        assert_eq!(from_rustix(rustix::io::Errno::CONNREFUSED), ECONNREFUSED);
        assert_eq!(from_rustix(rustix::io::Errno::ACCESS), EACCES);
        assert_eq!(from_rustix(rustix::io::Errno::NOTCONN), ENOTCONN);
        assert_eq!(from_rustix(rustix::io::Errno::ADDRINUSE), EADDRINUSE);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_raw_errno_passes_through() {
        let e = io::Error::from_raw_os_error(ECONNREFUSED);
        assert_eq!(from_io(&e), ECONNREFUSED);
        // Even codes we don't name are passed through verbatim.
        let e = io::Error::from_raw_os_error(75); // EOVERFLOW
        assert_eq!(from_io(&e), 75);
    }

    #[cfg(windows)]
    #[test]
    fn win32_codes_translate() {
        assert_eq!(from_io(&io::Error::from_raw_os_error(2)), ENOENT);
        assert_eq!(from_io(&io::Error::from_raw_os_error(5)), EACCES);
        assert_eq!(from_io(&io::Error::from_raw_os_error(183)), EEXIST);
        assert_eq!(from_io(&io::Error::from_raw_os_error(145)), ENOTEMPTY);
        assert_eq!(from_io(&io::Error::from_raw_os_error(10061)), ECONNREFUSED);
        assert_eq!(from_io(&io::Error::from_raw_os_error(10035)), EAGAIN);
        // Unknown raw codes fall back to the ErrorKind mapping.
        assert_eq!(from_io(&io::Error::from_raw_os_error(0x7fff_0000)), EIO);
    }
}
