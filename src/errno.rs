// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Linux `errno` values for the guest.
//!
//! The guest is a Linux-ABI unikernel: every error a host function
//! reports to it must be a *Linux* errno number, whatever the host OS.
//! On Linux the host's own errno already is that number and passes
//! through untouched.  Elsewhere the raw OS code is translated, with
//! `io::ErrorKind` as the fallback: on Windows the Win32 and Winsock
//! codes, on macOS the BSD numbering (the same as Linux up to `ERANGE`,
//! except `EDEADLK`, and different after).
//!
//! Callers get the errno number itself (`ECONNREFUSED` is 111) and
//! negate it on the wire, the usual `-errno` convention.  Only the
//! constants the translation and the host functions use are defined;
//! some exist only for the Windows and macOS tables.

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
pub const EOVERFLOW: i32 = 75;
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
pub const EISCONN: i32 = 106;
pub const ENOTCONN: i32 = 107;
pub const ETIMEDOUT: i32 = 110;
pub const ECONNREFUSED: i32 = 111;
pub const EHOSTUNREACH: i32 = 113;
pub const EINPROGRESS: i32 = 115;
pub const ESTALE: i32 = 116;
pub const EDQUOT: i32 = 122;

// Only the Windows and macOS translation tables need these.
#[cfg(target_vendor = "apple")]
pub const ENOSYS: i32 = 38;
#[cfg(target_vendor = "apple")]
pub const EILSEQ: i32 = 84;
#[cfg(target_vendor = "apple")]
pub const ENOMSG: i32 = 42;
#[cfg(target_vendor = "apple")]
pub const EIDRM: i32 = 43;
#[cfg(target_vendor = "apple")]
pub const ENOLCK: i32 = 37;
#[cfg(target_vendor = "apple")]
pub const ENODATA: i32 = 61;
#[cfg(target_vendor = "apple")]
pub const ETIME: i32 = 62;
#[cfg(target_vendor = "apple")]
pub const EREMOTE: i32 = 66;
#[cfg(target_vendor = "apple")]
pub const EPROTO: i32 = 71;
#[cfg(target_vendor = "apple")]
pub const EBADMSG: i32 = 74;
#[cfg(target_vendor = "apple")]
pub const EUSERS: i32 = 87;
#[cfg(target_vendor = "apple")]
pub const ETOOMANYREFS: i32 = 109;
#[cfg(target_vendor = "apple")]
pub const ECANCELED: i32 = 125;
#[cfg(target_vendor = "apple")]
pub const EOWNERDEAD: i32 = 130;
#[cfg(target_vendor = "apple")]
pub const ENOTRECOVERABLE: i32 = 131;
#[cfg(windows)]
pub const EPERM: i32 = 1;
#[cfg(windows)]
pub const EFAULT: i32 = 14;
#[cfg(any(windows, target_vendor = "apple"))]
pub const ENAMETOOLONG: i32 = 36;
#[cfg(any(windows, target_vendor = "apple"))]
pub const ELOOP: i32 = 40;
#[cfg(any(windows, target_vendor = "apple"))]
pub const ENOTSOCK: i32 = 88;
#[cfg(any(windows, target_vendor = "apple"))]
pub const EDESTADDRREQ: i32 = 89;
#[cfg(any(windows, target_vendor = "apple"))]
pub const EMSGSIZE: i32 = 90;
#[cfg(any(windows, target_vendor = "apple"))]
pub const EPROTOTYPE: i32 = 91;
#[cfg(any(windows, target_vendor = "apple"))]
pub const ESOCKTNOSUPPORT: i32 = 94;
#[cfg(any(windows, target_vendor = "apple"))]
pub const EPFNOSUPPORT: i32 = 96;
#[cfg(any(windows, target_vendor = "apple"))]
pub const ENETRESET: i32 = 102;
#[cfg(any(windows, target_vendor = "apple"))]
pub const ENOBUFS: i32 = 105;
#[cfg(any(windows, target_vendor = "apple"))]
pub const EHOSTDOWN: i32 = 112;
#[cfg(any(windows, target_vendor = "apple"))]
pub const EALREADY: i32 = 114;

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

        #[cfg(target_vendor = "apple")]
        if let Some(code) = from_bsd(raw) {
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

/// macOS errno codes with an exact Linux equivalent.  Written against
/// rustix's names, which carry the host's numbers, so the table reads as
/// name to name.
#[cfg(target_vendor = "apple")]
fn from_bsd(code: i32) -> Option<i32> {
    use rustix::io::Errno as E;

    // EPERM (1) to ERANGE (34) share their numbers, but for 11: EAGAIN on
    // Linux, EDEADLK on the BSDs (whose EAGAIN is 35).
    if (1..=34).contains(&code) && code != E::DEADLK.raw_os_error() {
        return Some(code);
    }
    const TABLE: &[(E, i32)] = &[
        (E::DEADLK, EDEADLK),
        (E::AGAIN, EAGAIN),
        (E::INPROGRESS, EINPROGRESS),
        (E::ALREADY, EALREADY),
        (E::NOTSOCK, ENOTSOCK),
        (E::DESTADDRREQ, EDESTADDRREQ),
        (E::MSGSIZE, EMSGSIZE),
        (E::PROTOTYPE, EPROTOTYPE),
        (E::NOPROTOOPT, ENOPROTOOPT),
        (E::PROTONOSUPPORT, EPROTONOSUPPORT),
        (E::SOCKTNOSUPPORT, ESOCKTNOSUPPORT),
        (E::OPNOTSUPP, EOPNOTSUPP),
        (E::PFNOSUPPORT, EPFNOSUPPORT),
        (E::AFNOSUPPORT, EAFNOSUPPORT),
        (E::ADDRINUSE, EADDRINUSE),
        (E::ADDRNOTAVAIL, EADDRNOTAVAIL),
        (E::NETDOWN, ENETDOWN),
        (E::NETUNREACH, ENETUNREACH),
        (E::NETRESET, ENETRESET),
        (E::CONNABORTED, ECONNABORTED),
        (E::CONNRESET, ECONNRESET),
        (E::NOBUFS, ENOBUFS),
        (E::ISCONN, EISCONN),
        (E::NOTCONN, ENOTCONN),
        (E::SHUTDOWN, EPIPE), // Linux reports send-after-shutdown as EPIPE
        (E::TIMEDOUT, ETIMEDOUT),
        (E::CONNREFUSED, ECONNREFUSED),
        (E::LOOP, ELOOP),
        (E::NAMETOOLONG, ENAMETOOLONG),
        (E::HOSTDOWN, EHOSTDOWN),
        (E::HOSTUNREACH, EHOSTUNREACH),
        (E::NOTEMPTY, ENOTEMPTY),
        (E::DQUOT, EDQUOT),
        (E::STALE, ESTALE),
        (E::NOSYS, ENOSYS),
        (E::OVERFLOW, EOVERFLOW),
        (E::ILSEQ, EILSEQ),
        (E::NOTSUP, EOPNOTSUPP),
        (E::NOLCK, ENOLCK),
        (E::CANCELED, ECANCELED),
        (E::IDRM, EIDRM),
        (E::NOMSG, ENOMSG),
        (E::NOATTR, ENODATA), // Linux reports a missing xattr as ENODATA
        (E::BADMSG, EBADMSG),
        (E::NODATA, ENODATA),
        (E::PROTO, EPROTO),
        (E::TIME, ETIME),
        (E::TOOMANYREFS, ETOOMANYREFS),
        (E::USERS, EUSERS),
        (E::REMOTE, EREMOTE),
        (E::NOTRECOVERABLE, ENOTRECOVERABLE),
        (E::OWNERDEAD, EOWNERDEAD),
    ];
    TABLE
        .iter()
        .find(|(e, _)| e.raw_os_error() == code)
        .map(|&(_, linux)| linux)
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
        let e = io::Error::from_raw_os_error(84); // EILSEQ
        assert_eq!(from_io(&e), 84);
    }

    #[cfg(target_vendor = "apple")]
    #[test]
    fn bsd_codes_translate() {
        use rustix::io::Errno as E;
        // Shared numbering passes through...
        assert_eq!(from_io(&io::Error::from_raw_os_error(2)), ENOENT);
        assert_eq!(from_io(&io::Error::from_raw_os_error(13)), EACCES);
        // ...but not where the BSDs diverge.
        let raw = |e: E| io::Error::from_raw_os_error(e.raw_os_error());
        assert_eq!(from_io(&raw(E::DEADLK)), EDEADLK);
        assert_eq!(from_io(&raw(E::AGAIN)), EAGAIN);
        assert_eq!(from_io(&raw(E::INPROGRESS)), EINPROGRESS);
        assert_eq!(from_io(&raw(E::CONNREFUSED)), ECONNREFUSED);
        assert_eq!(from_io(&raw(E::NOTEMPTY)), ENOTEMPTY);
        assert_eq!(from_io(&raw(E::NOLCK)), ENOLCK);
        assert_eq!(from_io(&raw(E::NOATTR)), ENODATA);
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
