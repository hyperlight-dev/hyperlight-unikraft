// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::time::Instant;

pub(crate) async fn sleep_until(deadline: Instant) {
    if deadline <= Instant::now() {
        return;
    }
    #[cfg(target_os = "linux")]
    match native_sleep_until(deadline).await {
        Ok(()) => return,
        Err(error) => tracing::debug!(%error, "native timer unavailable; using Tokio timer"),
    }
    // Keep the original deadline if native setup or waiting failed.
    tokio::time::sleep_until(deadline.into()).await;
}

#[cfg(target_os = "linux")]
async fn native_sleep_until(deadline: Instant) -> std::io::Result<()> {
    use rustix::time::{
        Itimerspec, TimerfdClockId, TimerfdFlags, TimerfdTimerFlags, timerfd_create,
        timerfd_settime,
    };
    use tokio::io::{Interest, unix::AsyncFd};

    let fd = timerfd_create(
        TimerfdClockId::Monotonic,
        TimerfdFlags::CLOEXEC | TimerfdFlags::NONBLOCK,
    )?;
    // SAFETY: AsyncFd owns the descriptor, which stays open and unchanged
    // until deregistration and drop, including when the future is cancelled.
    let fd = unsafe { AsyncFd::register_with_interest(fd, Interest::READABLE) }?;
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return Ok(());
    }
    timerfd_settime(
        fd.get_ref(),
        TimerfdTimerFlags::empty(),
        &Itimerspec {
            it_interval: Default::default(),
            it_value: remaining
                .try_into()
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?,
        },
    )?;
    loop {
        let mut ready = fd.readable().await?;
        match ready.try_io(|fd| {
            let mut expirations = [0; 8];
            match rustix::io::read(fd.get_ref(), &mut expirations) {
                Ok(8) => Ok(()),
                Ok(_) => Err(std::io::ErrorKind::UnexpectedEof.into()),
                Err(error) => Err(error.into()),
            }
        }) {
            Ok(Err(error)) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Ok(result) => return result,
            Err(_) => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;
    use std::time::Duration;

    #[test]
    fn expired_deadline_needs_no_runtime() {
        assert!(sleep_until(Instant::now()).now_or_never().is_some());
    }

    #[tokio::test]
    async fn timer_is_lazy_and_does_not_fire_early() {
        let deadline = Instant::now() + Duration::from_millis(20);
        let timer = sleep_until(deadline);
        let mut timer = std::pin::pin!(timer);
        assert!(timer.as_mut().now_or_never().is_none());
        tokio::time::timeout(Duration::from_secs(1), timer)
            .await
            .unwrap();
        assert!(Instant::now() >= deadline);
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn native_timer_completes_without_the_fallback() {
        let deadline = Instant::now() + Duration::from_millis(5);
        tokio::time::timeout(Duration::from_secs(1), native_sleep_until(deadline))
            .await
            .unwrap()
            .unwrap();
        assert!(Instant::now() >= deadline);
    }
}
