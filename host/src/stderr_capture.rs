//! Cross-platform stderr redirection used to capture VM console output.
//!
//! On Unix: dup2-based redirect to a temp file.
//! On Windows: no-op (VM output goes to inherited stderr, which the
//! kraftkit subprocess driver captures via exec.Command).

#[cfg(unix)]
mod imp {
    use anyhow::Result;
    use nix::unistd;
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    use std::path::Path;
    use std::sync::{Mutex, MutexGuard};

    /// Process-wide lock protecting fd 2 (stderr) manipulation.
    /// Held from `redirect_to_file` until `restore` to prevent concurrent
    /// VMs from racing on dup2.
    static STDERR_LOCK: Mutex<()> = Mutex::new(());

    pub struct Capture {
        original_stderr: Option<OwnedFd>,
        _guard: MutexGuard<'static, ()>,
    }

    impl Capture {
        pub fn redirect_to_file(path: &Path) -> Result<Self> {
            let guard = STDERR_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let capture_file = std::fs::File::create(path)?;
            let original_stderr = unistd::dup(2).map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })?;
            unistd::dup2(capture_file.as_raw_fd(), 2)?;
            // capture_file dropped here — its OwnedFd closes the fd via RAII
            Ok(Self {
                original_stderr: Some(original_stderr),
                _guard: guard,
            })
        }

        pub fn restore(self) -> Result<()> {
            if let Some(ref fd) = self.original_stderr {
                unistd::dup2(fd.as_raw_fd(), 2)?;
            }
            // Drop restores via Drop impl if restore() wasn't called,
            // but explicit call lets caller handle errors.
            Ok(())
        }
    }

    impl Drop for Capture {
        fn drop(&mut self) {
            if let Some(ref fd) = self.original_stderr.take() {
                let _ = unistd::dup2(fd.as_raw_fd(), 2);
            }
        }
    }
}

#[cfg(windows)]
mod imp {
    use anyhow::Result;
    use std::path::Path;

    /// No-op on Windows. VM console output goes to inherited stderr,
    /// which the kraftkit subprocess driver captures.
    pub struct Capture;

    impl Capture {
        pub fn redirect_to_file(_path: &Path) -> Result<Self> {
            Ok(Self)
        }

        pub fn restore(self) -> Result<()> {
            Ok(())
        }
    }
}

pub use imp::Capture;
