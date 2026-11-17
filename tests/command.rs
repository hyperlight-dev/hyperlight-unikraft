// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{
    FutureExt,
    io::{AsyncReadExt, AsyncWriteExt},
};
use hyperlight_unikraft::{
    Mount, SandboxBuilder, command::Stdio, load_command_snapshot, save_command_snapshot,
};

mod common;

fn builder() -> SandboxBuilder {
    common::require_bin("c", "command");
    SandboxBuilder::from_initrd(common::require_rootfs("c"))
        .mounts(vec![Mount::ro(common::require_bins("c"), "/mnt/bin")])
        .scratch_mb(128)
        .echo_output(false)
}

async fn read_while_running(
    child: &mut hyperlight_unikraft::command::Child,
    stdout: &mut (impl futures::io::AsyncRead + Unpin),
    bytes: &mut [u8],
) {
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        futures::future::select(Box::pin(stdout.read_exact(bytes)), Box::pin(child.wait())),
    )
    .await
    .unwrap();
    match result {
        futures::future::Either::Left((read, _)) => read.unwrap(),
        futures::future::Either::Right((status, _)) => {
            panic!("command exited before producing expected output: {status:?}")
        }
    }
}

#[tokio::test]
async fn output_modes_complete_without_readers_and_preserve_drop_counts() {
    let sandbox = builder().boot_commands().unwrap();
    for mode in [
        Stdio::head(0),
        Stdio::tail(0),
        Stdio::head(5000),
        Stdio::tail(5000),
        Stdio::unbounded(),
    ] {
        let mut child = sandbox
            .command("/mnt/bin/command")
            .arg("large")
            .stdout(mode)
            .stderr(mode)
            .spawn()
            .await
            .unwrap();
        assert!(
            tokio::time::timeout(Duration::from_secs(5), child.wait())
                .await
                .unwrap()
                .unwrap()
                .success()
        );
        let retained = match mode {
            Stdio::Head(n) | Stdio::Tail(n) => n,
            _ => 320 * 1024,
        };
        let dropped = (320 * 1024 - retained) as u64;
        assert_eq!(child.stdout.as_ref().unwrap().dropped_bytes(), dropped);
        assert_eq!(child.stderr.as_ref().unwrap().dropped_bytes(), dropped);
        let output = child.wait_with_output().await.unwrap();
        assert_eq!(output.stdout, vec![b'x'; retained]);
        assert_eq!(output.stderr, output.stdout);
        assert_eq!(output.stdout_dropped, dropped);
        assert_eq!(output.stderr_dropped, dropped);
    }
}

#[tokio::test]
async fn output_preserves_configured_modes_and_rejects_output_only_stdin() {
    let sandbox = builder().boot_commands().unwrap();
    let output = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "abcdef"])
        .stdout(Stdio::head(3))
        .stderr(Stdio::tail(0))
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"abc");
    assert_eq!(output.stdout_dropped, 4);
    assert_eq!(output.stderr_dropped, 0);
    for mode in [Stdio::head(3), Stdio::tail(3), Stdio::unbounded()] {
        assert!(matches!(
            sandbox.command("/mnt/bin/command").stdin(mode).spawn().await,
            Err(hyperlight_unikraft::Error::Wait(error))
                if error.kind() == std::io::ErrorKind::InvalidInput
        ));
    }
    assert!(sandbox.children().is_empty());
}

#[tokio::test]
async fn output_modes_and_loss_counters_survive_snapshot_restore() {
    let dir = common::temp_dir("command-output-snapshot");
    let sandbox = builder()
        .host_function("gate", |_| Ok("abcdefgh".into()))
        .boot_commands()
        .unwrap();
    for (mode, expected, dropped) in [
        (Stdio::head(3), &b"abc"[..], 8),
        (Stdio::tail(3), &b"XYZ"[..], 8),
        (Stdio::unbounded(), &b"abcdefghXYZ"[..], 0),
    ] {
        let child = sandbox
            .command("/mnt/bin/command")
            .args(["host-echo", "gate"])
            .stdin(Stdio::piped())
            .stdout(mode)
            .spawn()
            .await
            .unwrap();
        let id = child.execution_id();
        let initial_dropped = if mode == Stdio::Unbounded { 0 } else { 5 };
        assert_eq!(
            child.stdout.as_ref().unwrap().dropped_bytes(),
            initial_dropped
        );
        child.park();
        sandbox.snapshot_to(dir.path()).await.unwrap();
        sandbox.restore_from(dir.path()).await.unwrap();
        assert_eq!(sandbox.children().by_id(id).unwrap().stdio[1], mode);
        let mut child = sandbox
            .children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap();
        assert_eq!(
            child.stdout.as_ref().unwrap().dropped_bytes(),
            initial_dropped
        );
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(b"XYZ").await.unwrap();
        stdin.close().await.unwrap();
        assert!(
            tokio::time::timeout(Duration::from_secs(5), child.wait())
                .await
                .unwrap()
                .unwrap()
                .success()
        );
        let output = child.wait_with_output().await.unwrap();
        assert_eq!(output.stdout, expected);
        assert_eq!(output.stdout_dropped, dropped);
    }
}

#[tokio::test]
async fn tokio_pipes_echo_with_backpressure_and_stdin_shutdown() {
    use tokio::io::AsyncWriteExt as TokioAsyncWriteExt;

    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let input: Vec<u8> = (0..=255).cycle().take(256 * 1024).collect();
    let mut output = Vec::new();
    let (status, (), copied) = tokio::time::timeout(Duration::from_secs(5), async {
        tokio::join!(
            child.wait(),
            async {
                TokioAsyncWriteExt::write_all(&mut stdin, &input)
                    .await
                    .unwrap();
                TokioAsyncWriteExt::flush(&mut stdin).await.unwrap();
                stdin.shutdown().await.unwrap();
            },
            tokio::io::copy(&mut stdout, &mut output),
        )
    })
    .await
    .unwrap();
    assert!(status.unwrap().success());
    assert_eq!(copied.unwrap(), input.len() as u64);
    assert_eq!(output, input);
    assert_eq!(
        TokioAsyncWriteExt::write_all(&mut stdin, b"x")
            .await
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::BrokenPipe
    );
}

#[tokio::test]
async fn tokio_output_preserves_read_buf_contents_and_drains_both_streams() {
    use std::{mem::MaybeUninit, pin::Pin};
    use tokio::io::{AsyncRead, ReadBuf};

    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let mut storage = [MaybeUninit::uninit(); 16];
    let mut buffer = ReadBuf::uninit(&mut storage);
    buffer.put_slice(b"prefix");
    futures::future::poll_fn(|cx| Pin::new(&mut stdout).poll_read(cx, &mut buffer))
        .await
        .unwrap();
    assert_eq!(buffer.filled(), b"prefixxxxxxxxxxx");
    let mut output = buffer.filled()[6..].to_vec();
    let mut errors = Vec::new();
    let (status, copied, copied_errors) = tokio::time::timeout(Duration::from_secs(5), async {
        tokio::join!(
            child.wait(),
            tokio::io::copy(&mut stdout, &mut output),
            tokio::io::copy(&mut stderr, &mut errors),
        )
    })
    .await
    .unwrap();
    assert!(status.unwrap().success());
    assert_eq!(copied.unwrap(), 320 * 1024 - 10);
    assert_eq!(copied_errors.unwrap(), 320 * 1024);
    assert_eq!(output, vec![b'x'; 320 * 1024]);
    assert_eq!(errors, output);
    buffer.clear();
    buffer.put_slice(b"prefix");
    futures::future::poll_fn(|cx| Pin::new(&mut stdout).poll_read(cx, &mut buffer))
        .await
        .unwrap();
    assert_eq!(buffer.filled(), b"prefix");
}

#[tokio::test]
async fn root_exit_from_another_thread_cleans_up_without_a_supervisor() {
    let sandbox = builder().boot_commands().unwrap();
    let mut other = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "other"])
        .spawn()
        .await
        .unwrap();
    let output = tokio::time::timeout(
        Duration::from_secs(5),
        sandbox
            .command("/mnt/bin/command")
            .arg("thread-exit")
            .output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(output.status.code(), Some(23));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert_eq!(sandbox.children().len(), 1);
    assert!(other.try_wait().unwrap().is_none());
    other.kill().await.unwrap();
    assert_eq!(other.wait().await.unwrap().signal(), Some(9));
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .args(["sleep", "0", "done"])
            .status()
            .await
            .unwrap()
            .success()
    );
}

#[tokio::test]
async fn guest_stdio_epoll_and_short_vectored_reads_preserve_input() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("stdio-poll")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"abcdefghijk").await.unwrap();
    drop(stdin);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"ready\nabcdefghijk");
}

#[tokio::test]
async fn guest_stdio_nonblocking_backpressure_wakes_poll_without_committing_failed_writes() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("stdio-backpressure")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let mut marker = [0; 8];
    read_while_running(&mut child, &mut stderr, &mut marker).await;
    assert_eq!(&marker, b"blocked\n");
    assert!(child.try_wait().unwrap().is_none());
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout.len(), 64 * 1024 + 1);
    assert!(output.stdout[..64 * 1024].iter().all(|&byte| byte == b'x'));
    assert_eq!(output.stdout.last(), Some(&b'!'));
}

#[tokio::test]
async fn guest_stdio_eof_waits_for_last_duplicate_but_not_process_exit() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("stdio-dup")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    stdin.write_all(b"g").await.unwrap();
    let mut bytes = Vec::new();
    match tokio::time::timeout(
        Duration::from_secs(5),
        futures::future::select(
            Box::pin(stdout.read_to_end(&mut bytes)),
            Box::pin(child.wait()),
        ),
    )
    .await
    .unwrap()
    {
        futures::future::Either::Left((result, _)) => {
            result.unwrap();
        }
        futures::future::Either::Right((result, _)) => {
            panic!("exited before stdin EOF: {result:?}")
        }
    }
    assert_eq!(bytes, b"kept");
    assert!(child.try_wait().unwrap().is_none());
    drop(stdin);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stderr, b"redirected");
}

#[tokio::test]
async fn closing_guest_stdio_cancels_its_pending_readiness_watch() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("stdio-close-poll")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    assert_eq!(
        stdin.write_all(b"x").await.unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"closed\n");
    assert!(sandbox.children().is_empty());
}

#[tokio::test]
async fn closing_host_output_returns_epipe_to_the_guest_writer() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("stdio-broken-output")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    drop(child.stdout.take().unwrap());
    stdin.write_all(b"x").await.unwrap();
    drop(stdin);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stderr, b"broken\n");
}

#[tokio::test]
async fn passive_pipes_do_not_poll_host_calls_without_a_wait() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    for stream in ["stdin", "stdout", "stderr"] {
        let polled = Arc::new(AtomicUsize::new(0));
        let observed = polled.clone();
        let sandbox = builder()
            .async_host_function("gate", move |_| {
                observed.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok("reply".into()))
            })
            .boot_commands()
            .unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .args(["host", "[]"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let mut byte = [0];
        match stream {
            "stdin" => {
                let stdin = child.stdin.as_mut().unwrap();
                let input = vec![b'x'; 64 * 1024];
                stdin.write_all(&input).now_or_never().unwrap().unwrap();
                assert!(stdin.write_all(b"x").now_or_never().is_none());
                stdin.flush().await.unwrap();
                stdin.close().await.unwrap();
            }
            "stdout" => {
                assert!(
                    child
                        .stdout
                        .as_mut()
                        .unwrap()
                        .read(&mut byte)
                        .now_or_never()
                        .is_none()
                );
            }
            "stderr" => {
                assert!(
                    child
                        .stderr
                        .as_mut()
                        .unwrap()
                        .read(&mut byte)
                        .now_or_never()
                        .is_none()
                );
            }
            _ => unreachable!(),
        }
        assert_eq!(
            polled.load(Ordering::SeqCst),
            0,
            "{stream} drove a host call"
        );
        assert!(child.try_wait().unwrap().is_none());
        let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"reply");
        assert!(output.stderr.is_empty());
        assert_eq!(polled.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn passive_output_does_not_resume_an_expired_guest_timer() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "1", "done"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    for _ in 0..8 {
        assert!(
            child
                .stdout
                .as_mut()
                .unwrap()
                .read(&mut [0])
                .now_or_never()
                .is_none()
        );
        assert!(child.try_wait().unwrap().is_none());
    }
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"done\n");
}

#[tokio::test]
async fn dropping_last_sandbox_wakes_children_and_pipes_and_cancels_host_work() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Context;

    #[derive(Default)]
    struct WakeCount(AtomicUsize);
    impl futures::task::ArcWake for WakeCount {
        fn wake_by_ref(this: &Arc<Self>) {
            this.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let (sender, receiver) = futures::channel::oneshot::channel::<String>();
    let receiver = Mutex::new(Some(receiver));
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            let receiver = receiver.lock().unwrap().take().unwrap();
            async move { receiver.await.map_err(|error| error.to_string()) }
        })
        .boot_commands()
        .unwrap();
    let mut command = sandbox.command("/mnt/bin/command");
    let mut child = command
        .args(["host", "[]"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let info = sandbox.children().into_iter().last().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let mut wait = Box::pin(child.wait());
    let mut out = [0];
    let mut err = [0];
    let mut read = Box::pin(stdout.read(&mut out));
    let mut read_err = Box::pin(stderr.read(&mut err));
    let input = vec![b'x'; 1024 * 1024];
    let mut write = Box::pin(stdin.write_all(&input));
    let counts: Vec<_> = (0..4).map(|_| Arc::new(WakeCount::default())).collect();
    let wakers: Vec<_> = counts.iter().cloned().map(futures::task::waker).collect();
    // Settle input forwarding so none of the tasks is already scheduled to run.
    for _ in 0..128 {
        for count in &counts {
            count.0.store(0, Ordering::SeqCst);
        }
        assert!(
            wait.as_mut()
                .poll(&mut Context::from_waker(&wakers[0]))
                .is_pending()
        );
        assert!(
            read.as_mut()
                .poll(&mut Context::from_waker(&wakers[1]))
                .is_pending()
        );
        assert!(
            read_err
                .as_mut()
                .poll(&mut Context::from_waker(&wakers[2]))
                .is_pending()
        );
        assert!(
            write
                .as_mut()
                .poll(&mut Context::from_waker(&wakers[3]))
                .is_pending()
        );
        if counts
            .iter()
            .all(|count| count.0.load(Ordering::SeqCst) == 0)
        {
            break;
        }
    }
    assert!(
        counts
            .iter()
            .all(|count| count.0.load(Ordering::SeqCst) == 0)
    );
    assert!(!sender.is_canceled());
    assert!(!info.completed());
    std::thread::spawn(move || drop(sandbox)).join().unwrap();
    assert!(
        matches!(command.spawn().await, Err(hyperlight_unikraft::Error::Wait(error))
        if error.kind() == std::io::ErrorKind::NotFound)
    );
    assert!(sender.is_canceled());
    assert!(
        counts
            .iter()
            .all(|count| count.0.load(Ordering::SeqCst) > 0),
        "shutdown wake counts: {:?}",
        counts
            .iter()
            .map(|count| count.0.load(Ordering::SeqCst))
            .collect::<Vec<_>>()
    );
    assert!(info.completed() && info.parked());
    assert_eq!(wait.now_or_never().unwrap().unwrap().signal(), Some(9));
    assert_eq!(read.now_or_never().unwrap().unwrap(), 0);
    assert_eq!(read_err.now_or_never().unwrap().unwrap(), 0);
    assert_eq!(
        write.now_or_never().unwrap().unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
    assert_eq!(
        stdin.flush().await.unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
    stdin.close().await.unwrap();
    assert_eq!(child.try_wait().unwrap().unwrap().signal(), Some(9));
    child.kill().await.unwrap();
    assert!(info.restore().await.is_err());
}

#[tokio::test]
async fn sandbox_drop_preserves_completed_status_and_buffered_output() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "retained"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    assert!(child.wait().await.unwrap().success());
    let pid = child.id();
    drop(sandbox);
    assert_eq!(child.id(), pid);
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"retained\n");
}

#[tokio::test]
async fn pipe_wakeup_can_drop_the_sandbox_without_delaying_shutdown() {
    use hyperlight_unikraft::command::{ChildInfo, CommandSandbox};
    use std::task::Context;

    struct DropOnWake {
        owner: Mutex<Option<CommandSandbox>>,
        info: ChildInfo,
    }
    impl futures::task::ArcWake for DropOnWake {
        fn wake_by_ref(this: &Arc<Self>) {
            let owner = this.owner.lock().unwrap().take();
            if let Some(owner) = owner {
                drop(owner);
                assert!(this.info.completed(), "pipe operation retained the sandbox");
            }
        }
    }

    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let info = sandbox
        .children()
        .by_id(child.execution_id())
        .unwrap()
        .clone();
    let mut stdin = child.stdin.take().unwrap();
    let wake = Arc::new(DropOnWake {
        owner: Mutex::new(None),
        info,
    });
    let waker = futures::task::waker(wake.clone());
    let mut wait = Box::pin(child.wait());
    for _ in 0..4 {
        assert!(
            wait.as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
    }
    *wake.owner.lock().unwrap() = Some(sandbox);
    stdin.write_all(b"input").await.unwrap();
    assert!(wake.owner.lock().unwrap().is_none());
    assert!(wake.info.completed());
    assert_eq!(wait.await.unwrap().signal(), Some(9));
    assert_eq!(
        stdin.flush().await.unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
    stdin.close().await.unwrap();
    assert_eq!(
        child.stdout.as_mut().unwrap().read(&mut [0]).await.unwrap(),
        0
    );
}

#[tokio::test]
async fn sandbox_drop_closes_sockets_even_with_taken_pipes_alive() {
    use tokio::io::AsyncRead as _;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let sandbox = builder()
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["park-socket", &port])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut ready = [0; 6];
    read_while_running(&mut child, &mut stdout, &mut ready).await;
    assert_eq!(&ready, b"ready\n");
    let (mut socket, _) = listener.accept().await.unwrap();
    drop(sandbox);
    let mut byte = [0];
    let mut buffer = tokio::io::ReadBuf::new(&mut byte);
    tokio::time::timeout(
        Duration::from_secs(5),
        futures::future::poll_fn(|cx| std::pin::Pin::new(&mut socket).poll_read(cx, &mut buffer)),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(buffer.filled().is_empty());
    assert_eq!(child.wait().await.unwrap().signal(), Some(9));
    drop(child);
    assert_eq!(stdout.read(&mut byte).await.unwrap(), 0);
}

#[tokio::test]
async fn host_future_can_drop_the_last_sandbox_owner() {
    let owner = Arc::new(Mutex::new(None));
    let release = owner.clone();
    let (sender, receiver) = futures::channel::oneshot::channel::<String>();
    let receiver = Mutex::new(Some(receiver));
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            drop(release.lock().unwrap().take());
            let receiver = receiver.lock().unwrap().take().unwrap();
            async move { receiver.await.map_err(|error| error.to_string()) }
        })
        .boot_commands()
        .unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    *owner.lock().unwrap() = Some(sandbox);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(output.status.signal(), Some(9));
    assert!(sender.is_canceled());
    assert!(owner.lock().unwrap().is_none());
}

#[tokio::test]
async fn dropping_a_child_kills_only_its_tree_and_cancels_pending_calls() {
    let (sender, receiver) = futures::channel::oneshot::channel::<String>();
    let receiver = Mutex::new(Some(receiver));
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            let receiver = receiver.lock().unwrap().take().unwrap();
            async move { receiver.await.map_err(|error| error.to_string()) }
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    assert!(child.wait().now_or_never().is_none());
    let mut other = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "20", "other"])
        .spawn()
        .await
        .unwrap();
    drop(child);
    assert!(sender.is_canceled());
    assert!(stdout.read(&mut [0]).await.is_err());
    assert!(sandbox.children().by_id(id).is_none());
    assert!(other.try_wait().unwrap().is_none());
    assert!(other.wait().await.unwrap().success());
}

#[test]
fn dropping_a_tree_needs_no_runtime_and_does_not_drive_host_handlers() {
    async {
        let sandbox = builder()
            .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
                panic!("drop must not poll host handlers");
            })
            .boot_commands()
            .unwrap();
        for args in [vec!["host", "[]"], vec!["sleep-tree", "60000"]] {
            let child = sandbox
                .command("/mnt/bin/command")
                .args(args)
                .spawn()
                .await
                .unwrap();
            let id = child.execution_id();
            let children = sandbox.children();
            let info = children.by_id(id).unwrap();
            drop(child);
            assert!(info.completed());
            assert!(sandbox.children().by_id(id).is_none());
            assert!(info.restore().await.is_err());
        }
    }
    .now_or_never()
    .expect("uncontended lifecycle operations need no runtime");
}

#[tokio::test]
async fn cancelling_owned_output_kills_but_cancelling_borrowed_wait_does_not() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "done"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let info = sandbox.children().into_iter().last().unwrap();
    assert!(child.wait().now_or_never().is_none());
    assert!(!info.completed() && !info.parked());
    assert!(child.wait_with_output().now_or_never().is_none());
    assert!(info.completed() && info.parked());
    assert!(sandbox.children().is_empty());
    assert!(info.restore().await.is_err());
}

#[tokio::test]
async fn host_future_can_drop_another_child_and_terminate_without_deadlocking() {
    let slot = Arc::new(Mutex::new(None));
    let owned = slot.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            drop(owned.lock().unwrap().take());
            std::future::ready(Ok("not delivered".into()))
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "victim"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    *slot.lock().unwrap() = Some(child);
    let mut caller = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .await
        .unwrap();
    assert!(caller.wait().await.unwrap().success());
    assert!(stdout.read(&mut [0]).await.is_err());
    assert!(sandbox.children().by_id(id).is_none());
}

#[tokio::test]
async fn host_callback_can_drop_another_tree_during_a_vm_entry() {
    let victim = Arc::new(Mutex::new(None));
    let owned = victim.clone();
    let sandbox = builder()
        .host_function("gate", move |_| {
            drop(owned.lock().unwrap().take());
            Ok("done".into())
        })
        .boot_commands()
        .unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep-tree", "60000"])
        .spawn()
        .await
        .unwrap();
    let info = sandbox.children().into_iter().last().unwrap();
    *victim.lock().unwrap() = Some(child);
    let mut caller = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .await
        .unwrap();
    assert!(caller.wait().now_or_never().unwrap().unwrap().success());
    assert!(victim.lock().unwrap().is_none());
    assert!(info.completed());
    assert!(sandbox.children().is_empty());
    assert!(info.restore().await.is_err());
}

#[tokio::test]
async fn drop_during_snapshot_wakeup_is_terminated_before_snapshot_returns() {
    use hyperlight_unikraft::command::{Child, ChildInfo};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::Context;

    struct DropOnWake {
        armed: AtomicBool,
        child: Mutex<Option<Child>>,
        info: ChildInfo,
    }
    impl futures::task::ArcWake for DropOnWake {
        fn wake_by_ref(this: &Arc<Self>) {
            if this.armed.swap(false, Ordering::SeqCst) {
                drop(this.child.lock().unwrap().take());
                assert!(
                    !this.info.completed(),
                    "termination must wait for the snapshot lock"
                );
            }
        }
    }
    let sandbox = builder().boot_commands().unwrap();
    let victim = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "victim"])
        .spawn()
        .await
        .unwrap();
    let info = sandbox.children().into_iter().last().unwrap();
    let mut observer = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "observer"])
        .spawn()
        .await
        .unwrap();
    let wake = Arc::new(DropOnWake {
        armed: AtomicBool::new(false),
        child: Mutex::new(Some(victim)),
        info: info.clone(),
    });
    let waker = futures::task::waker(wake.clone());
    let mut wait = Box::pin(observer.wait());
    assert!(
        wait.as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    // Rearm after the initial VM entry has notified the first registration.
    assert!(
        wait.as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    wake.armed.store(true, Ordering::SeqCst);
    let snapshot = sandbox.snapshot().await.unwrap();
    assert!(info.completed());
    assert!(sandbox.children().by_id(info.execution_id).is_none());
    assert!(info.restore().await.is_err());
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    assert!(!fork.children()[0].completed());
}

#[tokio::test]
async fn named_children_restore_independently_and_release_names_on_completion() {
    use hyperlight_unikraft::Error;
    use std::io::ErrorKind;

    let sandbox = builder().boot_commands().unwrap();
    let mut command = sandbox.command("/mnt/bin/command");
    command
        .name("worker")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = command.spawn().await.unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"named\n")
        .await
        .unwrap();
    let id = child.execution_id();
    assert_eq!(child.name(), Some("worker"));
    assert_eq!(sandbox.children()[0].name.as_deref(), Some("worker"));
    let children = sandbox.children();
    let info = children.by_name("worker").unwrap();
    assert_eq!(info.execution_id, id);
    assert_eq!(info.program, "/mnt/bin/command");
    assert!(!info.parked());
    assert!(matches!(info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::AlreadyExists));
    assert!(matches!(command.spawn().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::AlreadyExists));
    assert_eq!(child.park(), id);
    assert!(info.parked());
    let snapshot = sandbox.snapshot().await.unwrap();
    assert!(matches!(command.spawn().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::AlreadyExists));
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    for source in [&sandbox, &fork] {
        assert!(source.children().by_name("Worker").is_none());
        let child = source
            .children()
            .by_name("worker")
            .unwrap()
            .restore()
            .await
            .unwrap();
        assert_eq!(child.execution_id(), id);
        assert_eq!(child.name(), Some("worker"));
        let out = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert!(out.status.success());
        assert_eq!(out.stdout, b"named\n");
        assert!(source.children().by_name("worker").is_none());
        assert!(source.children().by_id(id).is_none());
    }
    assert!(matches!(info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::NotFound));
    let replacement = command.spawn().await.unwrap();
    assert_ne!(replacement.execution_id(), id);
    drop(replacement);
    sandbox.restore(&snapshot).await.unwrap();
    let restored = sandbox
        .children()
        .by_name("worker")
        .unwrap()
        .restore()
        .await
        .unwrap();
    assert_eq!(restored.execution_id(), id);
    assert_eq!(restored.name(), Some("worker"));
}

#[tokio::test]
async fn completion_and_kill_release_names_while_old_handles_remain_usable() {
    use hyperlight_unikraft::Error;

    let sandbox = builder().boot_commands().unwrap();
    for kill in [false, true] {
        let mut child = sandbox
            .command("/mnt/bin/command")
            .name("worker")
            .arg("echo")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let id = child.execution_id();
        let children = sandbox.children();
        let info = children.by_id(id).unwrap();
        let mut stdout = child.stdout.take().unwrap();
        let mut output = Vec::new();
        if kill {
            child.kill().await.unwrap();
        } else {
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(b"done\n")
                .await
                .unwrap();
            let (status, read) = futures::join!(child.wait(), stdout.read_to_end(&mut output));
            assert!(status.unwrap().success());
            read.unwrap();
            assert_eq!(output, b"done\n");
        }
        assert!(info.completed());
        assert!(!info.parked());
        assert!(sandbox.children().is_empty());
        assert!(sandbox.children().by_id(id).is_none());
        assert!(sandbox.children().by_name("worker").is_none());
        assert!(matches!(info.restore().await, Err(Error::Wait(error))
            if error.kind() == std::io::ErrorKind::NotFound));
        let snapshot = sandbox.snapshot().await.unwrap();
        let mut replacement = sandbox
            .command("/mnt/bin/command")
            .name("worker")
            .args(["sleep", "60000", "replacement"])
            .spawn()
            .await
            .unwrap();
        assert_ne!(replacement.execution_id(), id);
        child.kill().await.unwrap();
        let status = child.wait().await.unwrap();
        if kill {
            assert_eq!(status.signal(), Some(9));
            stdout.read_to_end(&mut output).await.unwrap();
            assert!(output.is_empty());
        } else {
            assert!(status.success());
        }
        drop(child);
        assert!(replacement.try_wait().unwrap().is_none());
        assert_eq!(
            sandbox.children().by_name("worker").unwrap().execution_id,
            replacement.execution_id()
        );
        replacement.kill().await.unwrap();
        sandbox.restore(&snapshot).await.unwrap();
        assert!(sandbox.children().is_empty());
    }
}

#[tokio::test]
async fn cancelling_a_contended_kill_keeps_the_child_registered() {
    use hyperlight_unikraft::command::Child;

    let victim: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    let owned = victim.clone();
    let sandbox = builder()
        .host_function("gate", move |_| {
            let mut victim = owned.lock().unwrap();
            assert!(victim.as_mut().unwrap().kill().now_or_never().is_none());
            Ok("done".into())
        })
        .boot_commands()
        .unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .name("victim")
        .args(["sleep", "60000", "victim"])
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    *victim.lock().unwrap() = Some(child);
    let mut caller = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .await
        .unwrap();
    assert!(caller.wait().now_or_never().unwrap().unwrap().success());
    assert!(!sandbox.children().by_id(id).unwrap().completed());
    assert_eq!(
        sandbox.children().by_name("victim").unwrap().execution_id,
        id
    );
    let mut child = victim.lock().unwrap().take().unwrap();
    child.kill().await.unwrap();
    assert!(sandbox.children().by_id(id).is_none());
    assert!(sandbox.children().by_name("victim").is_none());
}

#[tokio::test]
async fn lifecycle_operations_wait_without_spinning_and_cancel_before_mutation() {
    use std::sync::{
        Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    use std::task::Context;

    #[derive(Default)]
    struct WakeCount(AtomicUsize);
    impl futures::task::ArcWake for WakeCount {
        fn wake_by_ref(this: &Arc<Self>) {
            this.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let (entered, receive) = mpsc::channel();
    let barrier = Arc::new(Barrier::new(2));
    let release = barrier.clone();
    let sandbox = builder()
        .host_function("gate", move |_| {
            entered.send(()).unwrap();
            barrier.wait();
            Ok("done".into())
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .name("victim")
        .args(["sleep", "60000", "victim"])
        .spawn()
        .await
        .unwrap();
    let children = sandbox.children();
    let checkpoint = sandbox.snapshot().await.unwrap();
    let mut caller = sandbox.command("/mnt/bin/command");
    caller.args(["host", "[]"]);
    let entry = std::thread::spawn(move || caller.spawn().now_or_never().unwrap().unwrap());
    receive.recv_timeout(Duration::from_secs(5)).unwrap();

    let mut snapshot = Box::pin(sandbox.snapshot());
    let mut restore = Box::pin(sandbox.restore(&checkpoint));
    let mut reacquire = Box::pin(children["victim"].restore());
    let mut kill = Box::pin(child.kill());
    let mut info_kill = Box::pin(children["victim"].kill());
    let mut command = sandbox.command("/mnt/bin/command");
    command
        .name("cancelled")
        .args(["sleep", "60000", "cancelled"]);
    let mut spawn = Box::pin(command.spawn());
    fn assert_send<T: Send>(_: &T) {}
    assert_send(&spawn);
    assert_send(&snapshot);
    assert_send(&restore);
    assert_send(&reacquire);
    assert_send(&kill);
    assert_send(&info_kill);
    let wakes = Arc::new(WakeCount::default());
    let waker = futures::task::waker(wakes.clone());
    let mut cx = Context::from_waker(&waker);
    assert!(spawn.as_mut().poll(&mut cx).is_pending());
    assert!(snapshot.as_mut().poll(&mut cx).is_pending());
    assert!(restore.as_mut().poll(&mut cx).is_pending());
    assert!(reacquire.as_mut().poll(&mut cx).is_pending());
    assert!(kill.as_mut().poll(&mut cx).is_pending());
    assert!(info_kill.as_mut().poll(&mut cx).is_pending());
    assert_eq!(wakes.0.load(Ordering::SeqCst), 0);
    drop(spawn);
    release.wait();
    drop(entry.join().unwrap());
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    assert!(
        matches!(reacquire.await, Err(hyperlight_unikraft::Error::Wait(error))
        if error.kind() == std::io::ErrorKind::AlreadyExists)
    );
    info_kill.await.unwrap();
    kill.await.unwrap();
    snapshot.await.unwrap();
    assert!(sandbox.children().is_empty());
    restore.await.unwrap();
    assert!(sandbox.children().by_name("cancelled").is_none());
    assert!(!sandbox.children()["victim"].completed());
}

#[tokio::test]
async fn lifecycle_waiters_wake_when_a_driver_releases_the_vm() {
    use std::sync::{
        Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    use std::task::Context;
    use tracing_subscriber::{Layer, layer::Context as TraceContext, prelude::*};

    struct HoldEntry {
        entered: Mutex<Option<mpsc::Sender<()>>>,
        release: Arc<Barrier>,
    }
    impl<S: tracing::Subscriber> Layer<S> for HoldEntry {
        fn on_event(&self, event: &tracing::Event<'_>, _: TraceContext<'_, S>) {
            if event.metadata().target() == "hyperlight_unikraft"
                && event.metadata().fields().field("yielded").is_some()
            {
                let sender = self.entered.lock().unwrap().take();
                if let Some(sender) = sender {
                    sender.send(()).unwrap();
                    self.release.wait();
                }
            }
        }
    }
    #[derive(Default)]
    struct WakeCount(AtomicUsize);
    impl futures::task::ArcWake for WakeCount {
        fn wake_by_ref(this: &Arc<Self>) {
            this.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let sandbox = builder()
        .async_host_function("gate", |_| std::future::ready(Ok("done".into())))
        .boot_commands()
        .unwrap();
    let victim = sandbox
        .command("/mnt/bin/command")
        .name("victim")
        .args(["sleep", "60000", "victim"])
        .spawn()
        .await
        .unwrap();
    let children = sandbox.children();
    let mut caller = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let (entered, receive) = mpsc::channel();
    let release = Arc::new(Barrier::new(2));
    let layer = HoldEntry {
        entered: Mutex::new(Some(entered)),
        release: release.clone(),
    };
    let entry = std::thread::spawn(move || {
        tracing::subscriber::with_default(tracing_subscriber::registry().with(layer), || {
            assert!(caller.wait().now_or_never().is_none());
        });
        caller
    });
    receive.recv_timeout(Duration::from_secs(5)).unwrap();
    let mut command = sandbox.command("/mnt/bin/command");
    command.args(["sleep", "60000", "new"]);
    let mut spawn = Box::pin(command.spawn());
    let mut kill = Box::pin(children["victim"].kill());
    let mut snapshot = Box::pin(sandbox.snapshot());
    let wakes = Arc::new(WakeCount::default());
    let waker = futures::task::waker(wakes.clone());
    let mut cx = Context::from_waker(&waker);
    assert!(spawn.as_mut().poll(&mut cx).is_pending());
    assert!(kill.as_mut().poll(&mut cx).is_pending());
    assert!(snapshot.as_mut().poll(&mut cx).is_pending());
    assert_eq!(wakes.0.load(Ordering::SeqCst), 0);
    release.wait();
    let caller = entry.join().unwrap();
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    kill.await.unwrap();
    let new_child = spawn.await.unwrap();
    snapshot.await.unwrap();
    assert_eq!(victim.try_wait().unwrap().unwrap().signal(), Some(9));
    assert!(caller.wait_with_output().await.unwrap().status.success());
    drop(new_child);
}

#[tokio::test]
async fn pending_lifecycle_futures_do_not_keep_the_sandbox_alive() {
    use std::sync::{Barrier, mpsc};
    let (entered, receive) = mpsc::channel();
    let barrier = Arc::new(Barrier::new(2));
    let release = barrier.clone();
    let sandbox = builder()
        .host_function("gate", move |_| {
            entered.send(()).unwrap();
            barrier.wait();
            Ok("done".into())
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .name("victim")
        .args(["sleep", "60000", "victim"])
        .spawn()
        .await
        .unwrap();
    let children = sandbox.children();
    let mut caller = sandbox.command("/mnt/bin/command");
    caller.args(["host", "[]"]);
    let entry = std::thread::spawn(move || caller.spawn().now_or_never().unwrap().unwrap());
    receive.recv_timeout(Duration::from_secs(5)).unwrap();
    let mut command = sandbox.command("/mnt/bin/command");
    let mut spawn = Box::pin(command.spawn());
    let mut restore = Box::pin(children["victim"].restore());
    let mut info_kill = Box::pin(children["victim"].kill());
    let mut kill = Box::pin(child.kill());
    assert!(spawn.as_mut().now_or_never().is_none());
    assert!(restore.as_mut().now_or_never().is_none());
    assert!(info_kill.as_mut().now_or_never().is_none());
    assert!(kill.as_mut().now_or_never().is_none());
    release.wait();
    drop(entry.join().unwrap());
    drop(sandbox);
    assert!(children["victim"].completed());
    assert!(
        matches!(spawn.await, Err(hyperlight_unikraft::Error::Wait(error))
        if error.kind() == std::io::ErrorKind::NotFound)
    );
    assert!(
        matches!(restore.await, Err(hyperlight_unikraft::Error::Wait(error))
        if error.kind() == std::io::ErrorKind::NotFound)
    );
    info_kill.await.unwrap();
    kill.await.unwrap();
    assert_eq!(child.try_wait().unwrap().unwrap().signal(), Some(9));
}

#[tokio::test]
async fn invalid_names_do_not_register_children_and_unnamed_children_remain_supported() {
    use hyperlight_unikraft::Error;
    use std::io::ErrorKind;
    let sandbox = builder().boot_commands().unwrap();
    for name in ["", "bad\0name"] {
        assert!(matches!(sandbox.command("/mnt/bin/command").name(name)
            .args(["sleep", "0", "done"]).spawn().await, Err(Error::Wait(error))
            if error.kind() == ErrorKind::InvalidInput));
        assert!(sandbox.children().is_empty());
    }
    for _ in 0..2 {
        let child = sandbox
            .command("/mnt/bin/command")
            .args(["sleep", "60000", "done"])
            .spawn()
            .await
            .unwrap();
        assert_eq!(child.name(), None);
        child.park();
    }
    assert_eq!(sandbox.children().len(), 2);
    assert!(sandbox.children().iter().all(|info| info.name.is_none()));
    for _ in 0..2 {
        let child = sandbox
            .command("/mnt/bin/command")
            .name("immediate")
            .args(["sleep", "0", "done"])
            .spawn()
            .await
            .unwrap();
        assert!(child.try_wait().unwrap().unwrap().success());
        assert!(sandbox.children().by_id(child.execution_id()).is_none());
        assert!(sandbox.children().by_name("immediate").is_none());
    }
}

#[tokio::test]
async fn structured_command_and_distinct_streams() {
    let sandbox = builder().boot_commands().unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["args", "a b", "", "quoted\"text"])
        .env_clear()
        .env("COMMAND_TEST", "one value")
        .current_dir("/tmp")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    assert_eq!(child.try_wait().unwrap().unwrap().code(), Some(17));
    assert!(sandbox.children().is_empty());
    let snapshot = sandbox.snapshot().await.unwrap();
    let out = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(out.status.code(), Some(17));
    assert_eq!(
        out.stdout,
        b"cwd=/tmp\nenv=one value\narg=a b\narg=\narg=quoted\"text\n"
    );
    assert_eq!(out.stderr, b"stderr\n");
    assert!(sandbox.children().is_empty());
    sandbox.restore(&snapshot).await.unwrap();
    assert!(sandbox.children().is_empty());
}

#[tokio::test]
async fn child_launch_metadata_survives_builder_reuse_and_snapshot_restore() {
    let sandbox = builder()
        .env("INHERITED", "original")
        .env("OVERRIDE", "original")
        .env("REMOVE", "original")
        .boot_commands()
        .unwrap();
    let mut command = sandbox.command("/mnt/bin/command");
    command
        .args(["echo", "first", ""])
        .stdin(Stdio::piped())
        .env("OVERRIDE", "a=b")
        .env_remove("REMOVE");
    let first = command.spawn().await.unwrap();
    let id = first.execution_id();
    let info = sandbox.children().into_iter().last().unwrap();
    assert_eq!(info.program, "/mnt/bin/command");
    assert_eq!(info.args, ["echo", "first", ""]);
    assert_eq!(info.cwd, None);
    assert_eq!(info.env["INHERITED"], "original");
    assert_eq!(info.env["OVERRIDE"], "a=b");
    assert!(!info.env.contains_key("REMOVE"));
    assert_eq!(info.stdio, [Stdio::Piped, Stdio::Null, Stdio::Null]);
    first.park();
    let snapshot = sandbox.snapshot().await.unwrap();

    let second = command
        .arg("later")
        .env_clear()
        .env("OVERRIDE", "changed")
        .current_dir("/tmp")
        .spawn()
        .await
        .unwrap();
    let second_info = sandbox.children().into_iter().last().unwrap();
    assert_eq!(second_info.args, ["echo", "first", "", "later"]);
    assert_eq!(second_info.cwd.as_deref(), Some("/tmp"));
    assert_eq!(
        second_info.env,
        [("OVERRIDE".to_owned(), "changed".to_owned())].into()
    );
    drop(second);

    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .env("INHERITED", "different fork configuration")
        .echo_output(false)
        .boot_commands()
        .unwrap();
    // Neither builder reuse nor inspecting and mutating a listing rewrites launch data.
    let mut edited = sandbox.children().into_iter().next().unwrap();
    edited.args.clear();
    edited.env.clear();
    for source in [&sandbox, &fork] {
        let child = source
            .children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap();
        let restored = source.children().into_iter().next().unwrap();
        assert_eq!(restored.program, info.program);
        assert_eq!(restored.args, info.args);
        assert_eq!(restored.cwd, info.cwd);
        assert_eq!(restored.env, info.env);
        assert_eq!(restored.stdio, info.stdio);
        child.park();
    }
    sandbox.restore(&snapshot).await.unwrap();
    let restored = sandbox.children().into_iter().last().unwrap();
    assert_eq!(restored.execution_id, id);
    assert_eq!(restored.program, info.program);
    assert_eq!(restored.args, info.args);
    assert_eq!(restored.cwd, info.cwd);
    assert_eq!(restored.env, info.env);
    assert_eq!(restored.stdio, info.stdio);
}

#[tokio::test]
async fn children_capture_stable_membership_with_live_flags_and_read_only_lookups() {
    let sandbox = builder().boot_commands().unwrap();
    let mut first = sandbox
        .command("/mnt/bin/command")
        .name("worker")
        .args(["sleep", "60000", "first"])
        .spawn()
        .await
        .unwrap();
    let second = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "second"])
        .spawn()
        .await
        .unwrap();
    let first_id = first.execution_id();
    let second_id = second.execution_id();
    let children = sandbox.children();
    assert_eq!(children.len(), 2);
    assert!(!children.is_empty());
    assert!(std::ptr::eq(&children[0], children.get(0).unwrap()));
    assert!(std::ptr::eq(&children[0], &children[first_id]));
    assert!(std::ptr::eq(&children[0], &children["worker"]));
    assert_eq!(children[second_id].name, None);
    assert!(children.get(2).is_none());
    assert!(children.by_name("Worker").is_none());
    assert_eq!(
        (&children)
            .into_iter()
            .map(|info| info.execution_id)
            .collect::<Vec<_>>(),
        [first_id, second_id]
    );

    second.park();
    assert!(children[second_id].parked());
    first.kill().await.unwrap();
    assert!(children[first_id].completed());
    assert!(children["worker"].restore().await.is_err());
    let replacement = sandbox
        .command("/mnt/bin/command")
        .name("worker")
        .args(["sleep", "60000", "replacement"])
        .spawn()
        .await
        .unwrap();
    let replacement_id = replacement.execution_id();
    assert_eq!(children.len(), 2);
    assert!(children.by_id(replacement_id).is_none());
    assert_eq!(children.by_name("worker").unwrap().execution_id, first_id);
    let fresh = sandbox.children();
    assert!(fresh.by_id(first_id).is_none());
    assert_eq!(fresh["worker"].execution_id, replacement_id);
    assert_eq!(fresh[0].execution_id, second_id);

    drop(sandbox);
    assert!(children[second_id].completed());
    let entries: Vec<_> = children.into_iter().collect();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].args, ["sleep", "60000", "first"]);
    assert_eq!(entries[1].execution_id, second_id);
}

#[tokio::test]
async fn listing_children_observes_lifecycle_and_restores_from_the_originating_sandbox() {
    use hyperlight_unikraft::Error;
    use std::io::ErrorKind;

    let gate = |_| -> std::future::Ready<Result<String, String>> {
        panic!("listing and reacquiring children must not poll host calls");
    };
    let sandbox = builder()
        .async_host_function("gate", gate)
        .boot_commands()
        .unwrap();
    assert!(sandbox.children().is_empty());
    let pending = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .await
        .unwrap();
    let pending_id = pending.execution_id();
    let mut done = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "1", "done"])
        .spawn()
        .await
        .unwrap();
    let done_id = done.execution_id();
    let before_completion = sandbox.children();
    let done_info = before_completion.by_id(done_id).unwrap();
    assert!(done.wait().await.unwrap().success());
    let done_pid = done.id();
    drop(done);

    let entries = sandbox.children();
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry.execution_id)
            .collect::<Vec<_>>(),
        [pending_id]
    );
    let pending_info = &entries[0];
    let by_id = entries.by_id(pending_id).unwrap();
    assert_eq!(by_id.execution_id, pending_info.execution_id);
    assert_eq!(by_id.program, pending_info.program);
    assert_eq!(by_id.pid, pending_info.pid);
    assert!(!by_id.parked() && !by_id.completed());
    assert_eq!(pending_info.pid, pending.id());
    let observer = pending_info.clone();
    assert!(!pending_info.parked() && !pending_info.completed());
    assert_eq!(done_info.pid, done_pid);
    assert!(done_info.parked() && done_info.completed());
    assert!(sandbox.children().by_id(done_id).is_none());
    assert!(
        matches!(pending_info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::AlreadyExists)
    );

    sandbox.snapshot().await.unwrap();
    assert!(!pending_info.parked());
    std::thread::spawn(move || pending.park()).join().unwrap();
    assert!(pending_info.parked() && observer.parked());
    assert!(by_id.parked());
    assert!(sandbox.children().iter().all(|entry| entry.parked()));
    let snapshot = sandbox.snapshot().await.unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .async_host_function("gate", gate)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let mut pending = pending_info.restore().await.unwrap();
    assert!(!pending_info.parked() && !observer.parked());
    let mut fork_pending = fork
        .children()
        .by_id(pending_id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    fork_pending.kill().await.unwrap();
    drop(fork_pending);
    assert!(pending.try_wait().unwrap().is_none());
    assert!(!pending_info.parked() && !pending_info.completed());
    assert!(
        matches!(pending_info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::AlreadyExists)
    );
    pending.kill().await.unwrap();
    assert!(pending_info.completed() && observer.completed());
    assert!(!pending_info.parked());
    pending.park();
    assert!(pending_info.parked() && observer.parked());
    assert!(sandbox.children().by_id(pending_id).is_none());
    assert!(
        matches!(pending_info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::NotFound)
    );
    assert!(sandbox.children().is_empty());
    assert!(matches!(done_info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::NotFound));
    drop(sandbox);
    assert!(done_info.parked() && done_info.completed());
    assert!(matches!(done_info.restore().await, Err(Error::Wait(error))
        if error.kind() == ErrorKind::NotFound));
}

#[tokio::test]
async fn lifecycle_flags_belong_to_the_observed_snapshot_incarnation() {
    let sandbox = builder().boot_commands().unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "done"])
        .spawn()
        .await
        .unwrap();
    let info = sandbox.children().into_iter().last().unwrap();
    child.park();
    let snapshot = sandbox.snapshot().await.unwrap();
    let mut child = info.restore().await.unwrap();
    child.kill().await.unwrap();
    assert!(info.completed());

    sandbox.restore(&snapshot).await.unwrap();
    let restored = sandbox.children().into_iter().last().unwrap();
    assert!(restored.parked() && !restored.completed());
    child.kill().await.unwrap();
    assert!(!restored.completed());
    assert_eq!(child.try_wait().unwrap().unwrap().signal(), Some(9));
    child.park();
    let mut child = restored.restore().await.unwrap();
    assert!(!restored.parked());
    assert!(info.parked() && info.completed());
    child.kill().await.unwrap();
    assert!(restored.completed());
}

#[tokio::test]
async fn independent_host_futures_and_cancelled_wait() {
    let polled = Arc::new(Mutex::new(Vec::new()));
    let observed = polled.clone();
    let completed = Arc::new(Mutex::new(Vec::new()));
    let finished = completed.clone();
    let sandbox = builder()
        .async_host_function("gate", move |args| {
            let observed = observed.clone();
            let finished = finished.clone();
            async move {
                observed.lock().unwrap().push(args.clone());
                tokio::time::sleep(Duration::from_millis(40)).await;
                finished.lock().unwrap().push(args.clone());
                Ok(args)
            }
        })
        .boot_commands()
        .unwrap();
    let mut a = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[1]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let b = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[2]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    assert!(a.wait().now_or_never().is_none());
    assert_eq!(*polled.lock().unwrap(), vec!["[1]"]);
    let out = tokio::time::timeout(Duration::from_secs(5), b.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(out.stdout, b"[2]");
    assert!(out.status.success());
    assert!(a.try_wait().unwrap().is_none());
    assert_eq!(*completed.lock().unwrap(), vec!["[2]"]);
    let out = tokio::time::timeout(Duration::from_secs(5), a.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(out.stdout, b"[1]");
    assert!(out.status.success());
    assert_eq!(*polled.lock().unwrap(), vec!["[1]", "[2]"]);
    assert_eq!(*completed.lock().unwrap(), vec!["[2]", "[1]"]);
}

#[tokio::test]
async fn driving_one_tree_does_not_schedule_another_trees_expired_timers() {
    let sandbox = builder().boot_commands().unwrap();
    for mode in ["sleep", "sleep-tree"] {
        let mut a = sandbox
            .command("/mnt/bin/command")
            .args([mode, "20", "a"])
            .spawn()
            .await
            .unwrap();
        let status = tokio::time::timeout(
            Duration::from_secs(5),
            sandbox
                .command("/mnt/bin/command")
                .args(["sleep", "80", "b"])
                .status(),
        )
        .await
        .unwrap()
        .unwrap();
        assert!(status.success());
        assert!(
            a.try_wait().unwrap().is_none(),
            "{mode} ran while driving b"
        );
        assert!(
            tokio::time::timeout(Duration::from_secs(5), a.wait())
                .await
                .unwrap()
                .unwrap()
                .success()
        );
    }
}

#[tokio::test]
async fn root_exit_kills_the_descendant_and_closes_taken_output() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("tree")
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let status = tokio::time::timeout(Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.code(), Some(7));
    let mut output = Vec::new();
    tokio::time::timeout(Duration::from_secs(5), stdout.read_to_end(&mut output))
        .await
        .unwrap()
        .unwrap();
    assert!(output.is_empty());
}

#[tokio::test]
async fn parking_a_child_cancels_calls_and_restoring_retries_with_eintr() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let (sender, receiver) = futures::channel::oneshot::channel::<String>();
    let receiver = Arc::new(Mutex::new(Some(receiver)));
    let count = Arc::new(AtomicUsize::new(0));
    let called = count.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            called.fetch_add(1, Ordering::SeqCst);
            let receiver = receiver.lock().unwrap().take();
            async move {
                match receiver {
                    Some(receiver) => receiver.await.map_err(|error| error.to_string()),
                    None => Ok("retried".into()),
                }
            }
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    assert!(
        sandbox
            .children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .is_err()
    );
    assert!(child.wait().now_or_never().is_none());
    assert_eq!(count.load(Ordering::SeqCst), 1);
    child.park();
    assert!(sender.is_canceled());
    assert_eq!(
        sandbox
            .children()
            .into_iter()
            .filter(|info| info.parked())
            .map(|info| info.execution_id)
            .collect::<Vec<_>>(),
        [id]
    );
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .args(["sleep", "0", "other"])
            .status()
            .await
            .unwrap()
            .success()
    );
    assert_eq!(count.load(Ordering::SeqCst), 1);
    let child = sandbox
        .children()
        .by_id(id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    child.park();
    let child = sandbox
        .children()
        .by_id(id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"interrupted\nretried");
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert!(sandbox.children().by_id(id).is_none());
}

#[tokio::test]
async fn parking_preserves_backpressured_output_and_invalidates_taken_pipes() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut old_stdout = child.stdout.take().unwrap();
    for _ in 0..40 {
        assert!(child.wait().now_or_never().is_none());
    }
    let mut first = [0];
    old_stdout.read_exact(&mut first).await.unwrap();
    assert!(child.wait().now_or_never().is_none());
    child.park();
    assert!(old_stdout.read(&mut first).await.is_err());
    let snapshot = sandbox.snapshot().await.unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let child = sandbox
        .children()
        .by_id(id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    assert!(old_stdout.read(&mut first).await.is_err());
    drop(old_stdout);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(first, [b'x']);
    assert_eq!(output.stdout, vec![b'x'; 80 * 4096 - 1]);
    assert_eq!(output.stderr, vec![b'x'; 80 * 4096]);
    let fork_output = tokio::time::timeout(
        Duration::from_secs(5),
        fork.children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap()
            .wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(fork_output.status.success());
    assert_eq!(fork_output.stdout, output.stdout);
    assert_eq!(fork_output.stderr, output.stderr);
}

#[tokio::test]
async fn snapshot_keeps_taken_pipes_usable_and_forks_buffered_input() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"before\n").await.unwrap();
    let snapshot = sandbox.snapshot().await.unwrap();
    stdin.write_all(b"after\n").await.unwrap();
    drop(stdin);
    let original = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(original.status.success());
    assert_eq!(original.stdout, b"before\nafter\n");

    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    assert!(fork.children()[0].parked());
    let copy = tokio::time::timeout(
        Duration::from_secs(5),
        fork.children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap()
            .wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(copy.status.success());
    assert_eq!(copy.stdout, b"before\n");
}

#[tokio::test]
async fn snapshot_of_unparked_backpressure_preserves_output_in_both_branches() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    for _ in 0..40 {
        assert!(child.wait().now_or_never().is_none());
    }
    let mut first = [0];
    stdout.read_exact(&mut first).await.unwrap();
    assert_eq!(first, [b'x']);
    assert!(child.wait().now_or_never().is_none());
    let snapshot = sandbox.snapshot().await.unwrap();
    assert!(!sandbox.children()[0].parked());
    child.stdout = Some(stdout);
    let original = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(original.status.success());
    assert_eq!(original.stdout, vec![b'x'; 80 * 4096 - 1]);
    assert_eq!(original.stderr, vec![b'x'; 80 * 4096]);
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let copy = tokio::time::timeout(
        Duration::from_secs(5),
        fork.children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap()
            .wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(copy.status.success());
    assert_eq!(copy.stdout, original.stdout);
    assert_eq!(copy.stderr, original.stderr);
}

#[tokio::test]
async fn parking_closes_host_connections_without_entering_the_vm() {
    use tokio::io::AsyncRead as _;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let sandbox = builder()
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["park-socket", &port])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut output = child.stdout.take().unwrap();
    let mut ready = [0; 6];
    read_while_running(&mut child, &mut output, &mut ready).await;
    assert_eq!(&ready, b"ready\n");
    let (mut socket, _) = listener.accept().await.unwrap();
    let snapshot = sandbox.snapshot().await.unwrap();
    let mut probe = [0];
    let mut buffer = tokio::io::ReadBuf::new(&mut probe);
    assert!(
        futures::future::poll_fn(|cx| std::pin::Pin::new(&mut socket).poll_read(cx, &mut buffer))
            .now_or_never()
            .is_none(),
        "taking a snapshot must not close the original connection"
    );
    child.park();
    let mut byte = [0];
    let mut buffer = tokio::io::ReadBuf::new(&mut byte);
    tokio::time::timeout(
        Duration::from_secs(5),
        futures::future::poll_fn(|cx| std::pin::Pin::new(&mut socket).poll_read(cx, &mut buffer)),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(buffer.filled().is_empty());
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let child = sandbox
        .children()
        .by_id(id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    drop(output);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"closed\n");
    let fork_output = tokio::time::timeout(
        Duration::from_secs(5),
        fork.children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap()
            .wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(fork_output.status.success());
    assert_eq!(fork_output.stdout, b"closed\n");
}

#[tokio::test]
async fn parking_releases_a_listener_and_restores_it_on_first_drive() {
    let reservation = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = reservation.local_addr().unwrap();
    drop(reservation);
    let (sender, receiver) = futures::channel::oneshot::channel::<String>();
    let receiver = Arc::new(Mutex::new(Some(receiver)));
    let sandbox = builder()
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .listen_ports(hyperlight_unikraft::ListenPorts::from_ports([
            address.port()
        ]))
        .async_host_function("gate", move |_| {
            let receiver = receiver.lock().unwrap().take();
            async move {
                match receiver {
                    Some(receiver) => receiver.await.map_err(|error| error.to_string()),
                    None => Ok(String::new()),
                }
            }
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["park-listener", &address.port().to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    read_while_running(&mut child, &mut stdout, &mut [0; 6]).await;
    assert!(child.wait().now_or_never().is_none());
    child.park();
    assert!(sender.is_canceled());
    let reservation = std::net::TcpListener::bind(address).unwrap();
    let mut child = sandbox
        .children()
        .by_id(id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    drop(reservation);
    assert!(child.wait().now_or_never().is_none());
    let (connection, output) = tokio::time::timeout(Duration::from_secs(5), async {
        futures::join!(
            tokio::net::TcpStream::connect(address),
            child.wait_with_output()
        )
    })
    .await
    .unwrap();
    connection.unwrap();
    let output = output.unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"interrupted\nclosed\n");
}

#[test]
fn child_info_kills_parked_and_active_trees_without_a_runtime() {
    async {
        let sandbox = builder()
            .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
                panic!("metadata kill must not poll host handlers");
            })
            .boot_commands()
            .unwrap();
        for parked in [false, true] {
            let mut child = sandbox
                .command("/mnt/bin/command")
                .name("worker")
                .args(["host", "[]"])
                .spawn()
                .await
                .unwrap();
            let children = sandbox.children();
            let info = &children["worker"];
            if parked {
                child.park();
            } else {
                let wait = child.wait();
                info.kill().await.unwrap();
                assert_eq!(wait.now_or_never().unwrap().unwrap().signal(), Some(9));
                assert!(!info.parked());
            }
            info.kill().await.unwrap();
            assert!(info.completed());
            assert!(sandbox.children().is_empty());
            assert!(info.restore().await.is_err());
        }
    }
    .now_or_never()
    .expect("uncontended lifecycle operations need no runtime");
}

#[tokio::test]
async fn child_info_kill_preserves_taken_pipes_and_wakes_a_pending_wait() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let children = sandbox.children();
    let info = &children[child.execution_id()];
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), child.wait())
            .await
            .is_err()
    );
    let mut other = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "other"])
        .spawn()
        .await
        .unwrap();
    let wait = tokio::spawn(async move {
        let status = child.wait().await.unwrap();
        (status, child)
    });
    tokio::task::yield_now().await;
    info.kill().await.unwrap();
    let (status, child) = tokio::time::timeout(Duration::from_secs(5), wait)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.signal(), Some(9));
    assert_eq!(child.try_wait().unwrap(), Some(status));
    assert!(other.try_wait().unwrap().is_none());
    assert!(stdin.write_all(b"closed").await.is_err());
    let mut out = Vec::new();
    let mut err = Vec::new();
    stdout.read_to_end(&mut out).await.unwrap();
    stderr.read_to_end(&mut err).await.unwrap();
    assert_eq!(out, vec![b'x'; 64 * 1024]);
    assert_eq!(err, out);
    assert!(sandbox.children().by_id(info.execution_id).is_none());
    info.kill().await.unwrap();
    other.kill().await.unwrap();
}

#[tokio::test]
async fn child_info_can_kill_while_its_host_future_is_being_polled() {
    use hyperlight_unikraft::command::ChildInfo;

    let target: Arc<Mutex<Option<ChildInfo>>> = Arc::new(Mutex::new(None));
    let owned = target.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            let info = owned.lock().unwrap().as_ref().unwrap().clone();
            async move {
                info.kill().await.map_err(|error| error.to_string())?;
                Ok("not delivered".into())
            }
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .await
        .unwrap();
    *target.lock().unwrap() = Some(sandbox.children()[child.execution_id()].clone());
    assert!(child.wait().now_or_never().is_none());
    assert_eq!(
        child.wait().now_or_never().unwrap().unwrap().signal(),
        Some(9)
    );
    assert!(sandbox.children().is_empty());
}

#[tokio::test]
async fn child_info_kill_never_targets_a_replacement_execution() {
    use hyperlight_unikraft::Error;

    let sandbox = builder().boot_commands().unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .name("worker")
        .args(["sleep", "60000", "worker"])
        .spawn()
        .await
        .unwrap();
    child.park();
    let old = sandbox.children();
    let snapshot = sandbox.snapshot().await.unwrap();
    sandbox.restore(&snapshot).await.unwrap();
    assert!(matches!(old["worker"].kill().await, Err(Error::Wait(error))
        if error.kind() == std::io::ErrorKind::NotFound));
    let current = sandbox.children();
    assert!(!current["worker"].completed());
    current["worker"].kill().await.unwrap();
    sandbox.restore(&snapshot).await.unwrap();
    current["worker"].kill().await.unwrap();
    assert!(!sandbox.children()["worker"].completed());
    drop(sandbox);
    old["worker"].kill().await.unwrap();
    current["worker"].kill().await.unwrap();
}

#[test]
fn parked_child_can_be_restored_and_killed_without_a_runtime() {
    async {
        let sandbox = builder()
            .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
                panic!("an undriven call must never be polled");
            })
            .boot_commands()
            .unwrap();
        let child = sandbox
            .command("/mnt/bin/command")
            .args(["host", "[]"])
            .spawn()
            .await
            .unwrap();
        let id = child.park();
        let mut child = sandbox
            .children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap();
        child.kill().await.unwrap();
        assert_eq!(
            child.wait().now_or_never().unwrap().unwrap().signal(),
            Some(9)
        );
    }
    .now_or_never()
    .expect("uncontended lifecycle operations need no runtime");
}

#[tokio::test]
async fn restoring_a_parked_timer_does_not_restart_its_duration() {
    let sandbox = builder().boot_commands().unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep-tree", "200"])
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    child.park();
    let snapshot = sandbox.snapshot().await.unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    tokio::time::sleep(Duration::from_millis(300)).await;
    for restored in [&sandbox, &fork] {
        let mut child = restored
            .children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap();
        assert!(child.try_wait().unwrap().is_none());
        let _ = child.wait().now_or_never();
        assert!(child.try_wait().unwrap().unwrap().success());
    }
}

#[tokio::test]
async fn parking_and_disk_snapshots_preserve_completed_host_replies() {
    let dir = common::temp_dir("command-reply-snapshot");
    let sandbox = builder()
        .async_host_function("gate", |_| std::future::ready(Ok("completed".into())))
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host-echo", "[]"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let _stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    assert!(child.wait().now_or_never().is_none());
    child.park();
    assert!(stdout.read(&mut [0]).await.is_err());
    let snapshot = sandbox.snapshot().await.unwrap();
    save_command_snapshot(&snapshot, dir.path()).unwrap();
    drop(snapshot);
    let snapshot = load_command_snapshot(dir.path()).unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
            panic!("a completed reply must survive the snapshot");
        })
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let fork_output = tokio::time::timeout(
        Duration::from_secs(5),
        fork.children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap()
            .wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(fork_output.status.success());
    assert_eq!(fork_output.stdout, b"completed");
    let output = tokio::time::timeout(
        Duration::from_secs(5),
        sandbox
            .children()
            .by_id(id)
            .unwrap()
            .restore()
            .await
            .unwrap()
            .wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"completed");
}

#[tokio::test]
async fn snapshots_restore_parked_streams_but_exclude_completed_children() {
    let sandbox = builder().boot_commands().unwrap();
    let mut echo = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let echo_id = echo.execution_id();
    sandbox.snapshot().await.unwrap();
    let mut old_stdin = echo.stdin.take().unwrap();
    old_stdin.write_all(b"retained input\n").await.unwrap();
    let mut done = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "retained output"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let done_id = done.execution_id();
    assert!(done.wait().await.unwrap().success());
    let snapshot = sandbox.snapshot().await.unwrap();
    assert!(sandbox.children().iter().all(|info| !info.parked()));
    echo.park();

    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .mount(Mount::ro(common::require_bins("c"), "/mnt/bin"))
        .echo_output(false)
        .boot_commands()
        .unwrap();
    assert_eq!(
        fork.children()
            .into_iter()
            .filter(|info| info.parked())
            .map(|info| info.execution_id)
            .collect::<Vec<_>>(),
        [echo_id]
    );
    let active = sandbox
        .children()
        .by_id(echo_id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    assert!(sandbox.restore(&snapshot).await.is_err());
    active.park();
    for restored in [&fork, &sandbox] {
        let echo = restored
            .children()
            .by_id(echo_id)
            .unwrap()
            .restore()
            .await
            .unwrap();
        assert!(restored.children().by_id(done_id).is_none());
        let echo = tokio::time::timeout(Duration::from_secs(5), echo.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(echo.stdout, b"retained input\n");
        assert!(echo.status.success());
    }

    let later = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "later"])
        .spawn()
        .await
        .unwrap();
    let later_id = later.execution_id();
    drop(later);
    sandbox.restore(&snapshot).await.unwrap();
    assert_eq!(
        sandbox
            .children()
            .into_iter()
            .filter(|info| info.parked())
            .map(|info| info.execution_id)
            .collect::<Vec<_>>(),
        [echo_id]
    );
    assert!(sandbox.children().by_id(done_id).is_none());
    let done = done.wait_with_output().await.unwrap();
    assert!(done.status.success());
    assert_eq!(done.stdout, b"retained output\n");
    assert!(sandbox.children().by_id(later_id).is_none());
    assert!(old_stdin.write_all(b"stale").await.is_err());
    drop(old_stdin);
    let echo = sandbox
        .children()
        .by_id(echo_id)
        .unwrap()
        .restore()
        .await
        .unwrap()
        .wait_with_output()
        .await
        .unwrap();
    assert_eq!(echo.stdout, b"retained input\n");
    let fresh = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "fresh"])
        .spawn()
        .await
        .unwrap();
    assert!(fresh.execution_id() > later_id);
}

#[tokio::test]
async fn snapshot_forks_interrupt_pending_calls_without_starting_handlers_at_boot() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let original_count = Arc::new(AtomicUsize::new(0));
    let called = original_count.clone();
    let (sender, receiver) = futures::channel::oneshot::channel::<String>();
    let receiver = Mutex::new(Some(receiver));
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            called.fetch_add(1, Ordering::SeqCst);
            let receiver = receiver.lock().unwrap().take().unwrap();
            async move { receiver.await.map_err(|error| error.to_string()) }
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    assert!(child.wait().now_or_never().is_none());
    assert_eq!(original_count.load(Ordering::SeqCst), 1);
    let snapshot = sandbox.snapshot().await.unwrap();
    assert!(!sender.is_canceled());
    assert!(!sandbox.children()[0].parked());
    assert_eq!(original_count.load(Ordering::SeqCst), 1);
    for _ in 0..2 {
        let count = Arc::new(AtomicUsize::new(0));
        let called = count.clone();
        let fork = SandboxBuilder::from_command_snapshot(&snapshot)
            .async_host_function("gate", move |_| {
                called.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok("fork".into()))
            })
            .echo_output(false)
            .boot_commands()
            .unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        let child = fork.children().by_id(id).unwrap().restore().await.unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"interrupted\nfork");
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
    sender.send("original".into()).unwrap();
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"original");
    assert_eq!(original_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn snapshot_and_child_restoration_wait_for_an_active_poll() {
    use std::sync::{
        Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    use std::task::Context;
    let (entered, receive) = mpsc::channel();
    #[derive(Default)]
    struct WakeCount(AtomicUsize);
    impl futures::task::ArcWake for WakeCount {
        fn wake_by_ref(this: &Arc<Self>) {
            this.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let barrier = Arc::new(Barrier::new(2));
    let release = barrier.clone();
    let count = Arc::new(AtomicUsize::new(0));
    let called = count.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            if called.fetch_add(1, Ordering::SeqCst) == 0 {
                entered.send(()).unwrap();
                barrier.wait();
            }
            std::future::ready(Ok("completed".into()))
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host-echo", "[]"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let _stdin = child.stdin.take().unwrap();
    let poller = std::thread::spawn(move || {
        let result = child.wait().now_or_never();
        (child, result)
    });
    receive.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(sandbox.snapshot().now_or_never().is_none());
    let children = sandbox.children();
    let mut snapshot = Box::pin(sandbox.snapshot());
    let mut restore = Box::pin(children[id].restore());
    let mut other = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "other"])
        .spawn()
        .await
        .unwrap();
    assert!(other.wait().now_or_never().unwrap().unwrap().success());
    drop(other);
    let wakes = Arc::new(WakeCount::default());
    let waker = futures::task::waker(wakes.clone());
    let mut cx = Context::from_waker(&waker);
    assert!(snapshot.as_mut().poll(&mut cx).is_pending());
    assert!(restore.as_mut().poll(&mut cx).is_pending());
    assert_eq!(
        wakes.0.load(Ordering::SeqCst),
        0,
        "contention must not busy-wake"
    );
    release.wait();
    let (child, result) = poller.join().unwrap();
    assert!(result.is_none());
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    snapshot.await.unwrap();
    assert!(
        matches!(restore.await, Err(hyperlight_unikraft::Error::Wait(error))
        if error.kind() == std::io::ErrorKind::AlreadyExists)
    );
    child.park();
    let mut child = sandbox
        .children()
        .by_id(id)
        .unwrap()
        .restore()
        .await
        .unwrap();
    child.kill().await.unwrap();
}

#[tokio::test]
async fn root_exit_cancels_descendant_host_calls_without_polling_them() {
    for collect_output in [false, true] {
        let sandbox = builder()
            .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
                panic!("a descendant must not be driven after root exit");
            })
            .boot_commands()
            .unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .arg("tree-host")
            .spawn()
            .await
            .unwrap();
        if collect_output {
            let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(output.status.code(), Some(7));
            assert!(output.stdout.is_empty() && output.stderr.is_empty());
        } else {
            let status = tokio::time::timeout(Duration::from_secs(5), child.wait())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(status.code(), Some(7));
            assert_eq!(child.try_wait().unwrap(), Some(status));
            assert_eq!(child.wait().now_or_never().unwrap().unwrap(), status);
            child.kill().await.unwrap();
        }
    }
}

#[tokio::test]
async fn bidirectional_pipes_and_bounded_output() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let input = vec![b'a'; 128 * 1024];
    let write = async {
        stdin.write_all(&input).await.unwrap();
        drop(stdin);
    };
    let (_, output) = tokio::time::timeout(Duration::from_secs(5), async {
        futures::join!(write, child.wait_with_output())
    })
    .await
    .unwrap();
    assert_eq!(output.unwrap().stdout, input);
    let out = tokio::time::timeout(
        Duration::from_secs(5),
        sandbox.command("/mnt/bin/command").arg("large").output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(out.stdout, vec![b'x'; 80 * 4096]);
    assert_eq!(out.stderr, out.stdout);
}

#[tokio::test]
async fn missing_executable_is_an_error_not_an_exit_code() {
    let sandbox = builder().boot_commands().unwrap();
    assert!(sandbox.command("/missing").spawn().await.is_err());
    assert!(sandbox.command("/dev/null").spawn().await.is_err());
    assert!(sandbox.command("relative").spawn().await.is_err());
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .current_dir("/missing")
            .spawn()
            .await
            .is_err()
    );
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .current_dir("")
            .spawn()
            .await
            .is_err()
    );
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .arg("bad\0argument")
            .spawn()
            .await
            .is_err()
    );
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .env("BAD=KEY", "value")
            .spawn()
            .await
            .is_err()
    );
    let output = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "after failed launches"])
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"after failed launches\n");
}

#[tokio::test]
async fn completed_contexts_do_not_exhaust_the_live_execution_limit() {
    let sandbox = builder().boot_commands().unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        let mut completed = Vec::new();
        for _ in 0..1030 {
            let mut child = sandbox
                .command("/mnt/bin/command")
                .args(["sleep", "0", ""])
                .spawn()
                .await
                .unwrap();
            assert!(child.wait().await.unwrap().success());
            completed.push(child);
        }
        assert!(
            completed
                .iter()
                .all(|child| child.try_wait().unwrap().unwrap().success())
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn command_only_boot_ignores_entry_and_survives_empty_intervals() {
    let sandbox = builder().entry("/missing/entry").boot_commands().unwrap();
    assert!(sandbox.children().is_empty());
    assert!(sandbox.command("/missing").spawn().await.is_err());
    for _ in 0..3 {
        let output = sandbox
            .command("/mnt/bin/command")
            .args(["sleep", "0", "command-only"])
            .output()
            .await
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"command-only\n");
        assert!(sandbox.children().is_empty());

        let mut child = sandbox
            .command("/mnt/bin/command")
            .args(["sleep", "60000", ""])
            .spawn()
            .await
            .unwrap();
        child.kill().await.unwrap();
        assert_eq!(child.wait().await.unwrap().signal(), Some(9));
        assert!(sandbox.children().is_empty());
    }
}

#[tokio::test]
async fn command_only_boot_restores_parked_commands_and_empty_snapshots() {
    let sandbox = builder().boot_commands().unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "10", "restored"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let snapshot = sandbox.snapshot().await.unwrap();
    let restored = SandboxBuilder::from_command_snapshot(&snapshot)
        .entry("/also/ignored")
        .mount(Mount::ro(common::require_bins("c"), "/mnt/bin"))
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let output = restored.children()[id]
        .restore()
        .await
        .unwrap()
        .wait_with_output()
        .await
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"restored\n");
    let empty = restored.snapshot().await.unwrap();
    let restored = SandboxBuilder::from_command_snapshot(&empty)
        .mount(Mount::ro(common::require_bins("c"), "/mnt/bin"))
        .echo_output(false)
        .boot_commands()
        .unwrap();
    assert!(restored.children().is_empty());
    assert!(
        restored
            .command("/mnt/bin/command")
            .args(["sleep", "0", ""])
            .status()
            .await
            .unwrap()
            .success()
    );
}

#[tokio::test]
async fn command_only_boot_rejects_snapshots_with_a_boot_process() {
    let mut sandbox = builder().boot().unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    let result = SandboxBuilder::from_snapshot(snapshot)
        .on_interrupt_handle(|_| panic!("must reject before entering the guest"))
        .boot_commands();
    assert!(matches!(
        result,
        Err(hyperlight_unikraft::Error::Wait(error))
            if error.kind() == std::io::ErrorKind::InvalidInput
    ));
}

#[tokio::test]
async fn command_disk_snapshots_preserve_large_buffers_metadata_and_live_handles() {
    let dir = common::temp_dir("command-disk-roundtrip");
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .name("buffered")
        .args(["echo", ""])
        .env_clear()
        .env("TEST", "a=b")
        .current_dir("/tmp")
        .stdin(Stdio::piped())
        .stdout(Stdio::unbounded())
        .stderr(Stdio::tail(3))
        .spawn()
        .await
        .unwrap();
    let id = child.execution_id();
    let pid = child.id();
    let mut stdin = child.stdin.take().unwrap();
    let bytes: Vec<u8> = (0..=255).cycle().take(384 * 1024).collect();
    match tokio::time::timeout(
        Duration::from_secs(5),
        futures::future::select(Box::pin(stdin.write_all(&bytes)), Box::pin(child.wait())),
    )
    .await
    .unwrap()
    {
        futures::future::Either::Left((write, _)) => write.unwrap(),
        futures::future::Either::Right((status, _)) => panic!("early exit: {status:?}"),
    }
    sandbox.snapshot_to(dir.path()).await.unwrap();
    let state_path = std::fs::read_dir(dir.path().join("commands"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    assert!(std::fs::metadata(state_path).unwrap().len() > 1024 * 1024);
    assert!(!sandbox.children()[id].parked());
    stdin.close().await.unwrap();
    let original = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(original.status.success());
    assert_eq!(original.stdout, bytes);
    drop(stdin);
    drop(sandbox);

    let restored = SandboxBuilder::from_command_snapshot_dir(dir.path())
        .unwrap()
        .mount(Mount::ro(common::require_bins("c"), "/mnt/bin"))
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let children = restored.children();
    let info = &children["buffered"];
    assert_eq!(info.execution_id, id);
    assert_eq!(info.pid, pid);
    assert_eq!(info.program, "/mnt/bin/command");
    assert_eq!(info.args, ["echo", ""]);
    assert_eq!(info.cwd.as_deref(), Some("/tmp"));
    assert_eq!(info.env, [("TEST".into(), "a=b".into())].into());
    assert_eq!(info.stdio, [Stdio::Piped, Stdio::Unbounded, Stdio::Tail(3)]);
    assert!(info.parked());
    let copy = tokio::time::timeout(
        Duration::from_secs(5),
        info.restore().await.unwrap().wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(copy.status.success());
    assert_eq!(copy.stdout, bytes);

    restored.snapshot_to(dir.path()).await.unwrap();
    drop(restored);
    let restored = SandboxBuilder::from_command_snapshot_dir(dir.path())
        .unwrap()
        .mount(Mount::ro(common::require_bins("c"), "/mnt/bin"))
        .echo_output(false)
        .boot_commands()
        .unwrap();
    assert!(restored.children().is_empty());
    let child = restored
        .command("/mnt/bin/command")
        .args(["sleep", "0", "new"])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    assert!(child.execution_id() > id);
    assert_eq!(child.wait_with_output().await.unwrap().stdout, b"new\n");
}

#[tokio::test]
async fn command_disk_snapshots_reject_wrong_kind_missing_state_and_corruption() {
    let app_dir = common::temp_dir("app-snapshot-kind");
    let mut app = builder().boot().unwrap();
    app.snapshot_to(app_dir.path()).unwrap();
    assert!(
        load_command_snapshot(app_dir.path())
            .err()
            .unwrap()
            .to_string()
            .contains("not a command snapshot")
    );
    drop(app);

    let dir = common::temp_dir("command-snapshot-errors");
    let sandbox = builder().boot_commands().unwrap();
    let snapshot = sandbox.snapshot_to(dir.path()).await.unwrap();
    assert!(
        hyperlight_unikraft::load_snapshot(dir.path())
            .err()
            .unwrap()
            .to_string()
            .contains("use load_command_snapshot")
    );
    let path = std::fs::read_dir(dir.path().join("commands"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[0] ^= 1;
    std::fs::write(&path, &bytes).unwrap();
    assert!(
        load_command_snapshot(dir.path())
            .err()
            .unwrap()
            .to_string()
            .contains("checksum mismatch")
    );
    std::fs::remove_file(&path).unwrap();
    assert!(load_command_snapshot(dir.path()).is_err());
    save_command_snapshot(&snapshot, dir.path()).unwrap();
    assert!(load_command_snapshot(dir.path()).is_ok());

    let child = sandbox
        .command("/mnt/bin/command")
        .name("unsaved")
        .arg("echo")
        .stdin(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let index = std::fs::read(dir.path().join("index.json")).unwrap();
    let marker_path = dir.path().join("oci-layout");
    let marker = std::fs::read(&marker_path).unwrap();
    std::fs::write(&marker_path, br#"{"imageLayoutVersion":"unsupported"}"#).unwrap();
    assert!(sandbox.snapshot_to(dir.path()).await.is_err());
    assert_eq!(std::fs::read(dir.path().join("index.json")).unwrap(), index);
    std::fs::write(&marker_path, marker).unwrap();
    let previous = SandboxBuilder::from_command_snapshot_dir(dir.path())
        .unwrap()
        .echo_output(false)
        .boot_commands()
        .unwrap();
    assert!(previous.children().is_empty());
    drop(previous);
    drop(child);

    let tag = format!(
        "{}-{}",
        env!("CARGO_PKG_VERSION"),
        hyperlight_unikraft::SNAPSHOT_KEY
    )
    .parse::<hyperlight_unikraft::hyperlight_host::sandbox::snapshot::OciTag>()
    .unwrap();
    let guest = hyperlight_unikraft::Snapshot::load(dir.path(), tag).unwrap();
    let mut metadata = guest
        .metadata::<serde_json::Value>("hluk.commands")
        .unwrap()
        .unwrap();
    metadata["version"] = serde_json::json!(999);
    let guest = guest.with_metadata("hluk.commands", &metadata).unwrap();
    hyperlight_unikraft::save_snapshot(&guest, dir.path()).unwrap();
    assert!(
        load_command_snapshot(dir.path())
            .err()
            .unwrap()
            .to_string()
            .contains("unsupported command snapshot version")
    );
}

#[tokio::test]
async fn python_rootfs_supports_separate_legacy_and_command_boots() {
    let mut sandbox = SandboxBuilder::from_initrd(common::require_rootfs("python"))
        .scratch_mb(256)
        .echo_output(false)
        .boot()
        .unwrap();
    sandbox.run("print('legacy still works')").await.unwrap();
    assert!(sandbox.take_output().text.contains("legacy still works"));
    drop(sandbox);
    let sandbox = SandboxBuilder::from_initrd(common::require_rootfs("python"))
        .scratch_mb(256)
        .echo_output(false)
        .boot_commands()
        .unwrap();
    let output = sandbox
        .command("/usr/local/bin/python3")
        .args(["-u", "-c", "print('kernel command')"])
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"kernel command\n");
}

#[tokio::test]
async fn compiled_runtime_images_launch_independent_commands() {
    for (runtime, program, greeting) in [
        ("rust", "hello", "Hello from Rust"),
        ("go", "hello", "Hello from Go"),
        ("dotnet-aot", "Hello", "Hello from .NET AOT"),
    ] {
        common::require_bin(runtime, program);
        let sandbox = SandboxBuilder::from_initrd(common::require_rootfs(runtime))
            .mounts(vec![Mount::ro(common::require_bins(runtime), "/mnt/bin")])
            .scratch_mb(256)
            .echo_output(false)
            .boot_commands()
            .unwrap();
        let path = format!("/mnt/bin/{program}");
        let a = sandbox
            .command(&path)
            .stdout(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let b = sandbox
            .command(&path)
            .stdout(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let (a, b) = tokio::time::timeout(Duration::from_secs(10), async {
            futures::try_join!(a.wait_with_output(), b.wait_with_output())
        })
        .await
        .unwrap()
        .unwrap();
        for output in [a, b] {
            assert!(output.status.success(), "{runtime}: {:?}", output.status);
            assert!(
                String::from_utf8_lossy(&output.stdout).contains(greeting),
                "{runtime}: {:?}",
                output.stdout
            );
        }
    }
}

#[tokio::test]
async fn killing_one_root_preserves_other_commands_and_cancelled_waits() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "late"])
        .spawn()
        .await
        .unwrap();
    assert!(child.wait().now_or_never().is_none());
    child.kill().await.unwrap();
    assert_eq!(child.try_wait().unwrap().unwrap().signal(), Some(9));
    let next = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "1", "alive"])
        .output()
        .await
        .unwrap();
    assert_eq!(next.stdout, b"alive\n");
    assert!(next.status.success());
}

#[tokio::test]
async fn killing_a_root_interrupts_and_drops_its_pending_host_call() {
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Dropped(Arc<AtomicBool>);
    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    let dropped = Arc::new(AtomicBool::new(false));
    let observed = dropped.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            let observed = observed.clone();
            async move {
                let _guard = Dropped(observed);
                std::future::pending::<()>().await;
                Ok("[]".into())
            }
        })
        .boot_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .await
        .unwrap();
    assert!(child.wait().now_or_never().is_none());
    assert!(!dropped.load(Ordering::SeqCst));
    child.kill().await.unwrap();
    assert_eq!(child.try_wait().unwrap().unwrap().signal(), Some(9));
    assert!(dropped.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn root_exit_and_kill_end_reparented_descendants_and_drop_their_host_calls() {
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Dropped(Arc<AtomicBool>);
    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    for root_mode in ["live", "exit"] {
        let (release_root, root_ready) = futures::channel::oneshot::channel::<()>();
        let root_ready = Arc::new(Mutex::new(Some(root_ready)));
        let started = Arc::new(AtomicBool::new(false));
        let dropped = Arc::new(AtomicBool::new(false));
        let observed_start = started.clone();
        let observed_drop = dropped.clone();
        let sandbox = builder()
            .async_host_function("root-gate", move |_| {
                let ready = root_ready.lock().unwrap().take().unwrap();
                async move {
                    ready.await.map_err(|error| error.to_string())?;
                    Ok(String::new())
                }
            })
            .async_host_function("gate", move |_| {
                let started = observed_start.clone();
                let dropped = observed_drop.clone();
                async move {
                    let _guard = Dropped(dropped);
                    started.store(true, Ordering::SeqCst);
                    std::future::pending::<()>().await;
                    Ok("[]".into())
                }
            })
            .boot_commands()
            .unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .args(["kill-tree", root_mode])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = child.stdout.take().unwrap();
        tokio::time::timeout(Duration::from_secs(5), async {
            while !started.load(Ordering::SeqCst) {
                assert!(child.wait().now_or_never().is_none());
                tokio::task::yield_now().await;
            }
            if root_mode == "exit" {
                release_root.send(()).unwrap();
                assert_eq!(child.wait().await.unwrap().code(), Some(7));
            } else {
                assert!(child.try_wait().unwrap().is_none());
                child.kill().await.unwrap();
            }
            let status = child.wait().now_or_never().unwrap().unwrap();
            let mut output = Vec::new();
            stdout
                .read_to_end(&mut output)
                .now_or_never()
                .unwrap()
                .unwrap();
            assert!(b"before kill\n".starts_with(&output));
            if root_mode == "exit" {
                assert_eq!(status.code(), Some(7));
                assert_eq!(output, b"before kill\n");
            } else {
                assert_eq!(status.signal(), Some(9));
            }
            assert!(dropped.load(Ordering::SeqCst));
            assert!(stdin.write_all(b"closed").await.is_err());
            child.kill().await.unwrap();
            let next = sandbox
                .command("/mnt/bin/command")
                .args(["sleep", "0", "still alive"])
                .output()
                .await
                .unwrap();
            assert_eq!(next.stdout, b"still alive\n");
        })
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn synchronous_kill_preserves_only_host_buffered_output_in_one_entry() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tracing_subscriber::{Layer, layer::Context, prelude::*};

    struct Entries(Arc<AtomicUsize>);
    impl<S: tracing::Subscriber> Layer<S> for Entries {
        fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
            let metadata = event.metadata();
            if metadata.target() == "hyperlight_unikraft"
                && metadata.fields().field("yielded").is_some()
                && metadata.fields().field("name").is_some()
            {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), child.wait())
            .await
            .is_err()
    );
    assert!(child.try_wait().unwrap().is_none());
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let entries = Arc::new(AtomicUsize::new(0));
    let subscriber = tracing_subscriber::registry().with(Entries(entries.clone()));
    tracing::subscriber::with_default(subscriber, || {
        child.kill().now_or_never().unwrap().unwrap();
        assert_eq!(entries.load(Ordering::SeqCst), 1);
        let status = child.wait().now_or_never().unwrap().unwrap();
        assert_eq!(status.signal(), Some(9));
        stdout
            .read_to_end(&mut out)
            .now_or_never()
            .unwrap()
            .unwrap();
        stderr
            .read_to_end(&mut err)
            .now_or_never()
            .unwrap()
            .unwrap();
        child.kill().now_or_never().unwrap().unwrap();
        assert_eq!(entries.load(Ordering::SeqCst), 1);
    });
    for output in [out, err] {
        assert_eq!(output.len(), 64 * 1024);
        assert!(output.iter().all(|byte| *byte == b'x'));
    }
}

#[test]
fn kill_without_a_runtime_does_not_poll_queued_host_functions() {
    async {
        let sandbox = builder()
            .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
                panic!("kill must cancel, not poll, the host function");
            })
            .boot_commands()
            .unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .args(["host", "[]"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = child.stdout.take().unwrap();
        child.kill().await.unwrap();
        let status = child.wait().now_or_never().unwrap().unwrap();
        assert_eq!(status.signal(), Some(9));
        assert!(stdin.write_all(b"closed").now_or_never().unwrap().is_err());
        let mut output = Vec::new();
        stdout
            .read_to_end(&mut output)
            .now_or_never()
            .unwrap()
            .unwrap();
        assert!(output.is_empty());
    }
    .now_or_never()
    .expect("uncontended lifecycle operations need no runtime");
}

#[tokio::test]
async fn kill_closes_only_its_trees_inherited_socket() {
    tokio::time::timeout(Duration::from_secs(10), async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port().to_string();
        let sandbox = builder()
            .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
            .boot_commands()
            .unwrap();
        let mut a = sandbox
            .command("/mnt/bin/command")
            .args(["net-tree", &port])
            .spawn()
            .await
            .unwrap();
        let peer_a = match futures::future::select(Box::pin(a.wait()), Box::pin(listener.accept()))
            .await
        {
            futures::future::Either::Right((peer, _)) => peer.unwrap().0,
            futures::future::Either::Left((status, _)) => panic!("root exited early: {status:?}"),
        };
        let mut b = sandbox
            .command("/mnt/bin/command")
            .args(["net-tree", &port])
            .stdout(Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let peer_b = match futures::future::select(Box::pin(b.wait()), Box::pin(listener.accept()))
            .await
        {
            futures::future::Either::Right((peer, _)) => peer.unwrap().0,
            futures::future::Either::Left((status, _)) => panic!("root exited early: {status:?}"),
        };

        assert!(a.wait().now_or_never().is_none());
        a.kill().await.unwrap();
        assert_eq!(a.wait().now_or_never().unwrap().unwrap().signal(), Some(9));
        loop {
            peer_a.readable().await.unwrap();
            match peer_a.try_read(&mut [0]) {
                Ok(count) => {
                    assert_eq!(count, 0);
                    break;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("{error}"),
            }
        }
        peer_b.writable().await.unwrap();
        assert_eq!(peer_b.try_write(b"b").unwrap(), 1);
        let output = b.wait_with_output().await.unwrap();
        assert_eq!(output.stdout, b"socket-descendant\n");
        assert_eq!(output.status.code(), Some(9));
        peer_b.readable().await.unwrap();
        let mut reply = [0];
        assert_eq!(peer_b.try_read(&mut reply).unwrap(), 1);
        assert_eq!(&reply, b"b");
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn independent_wait_and_pipe_tasks_do_not_overwrite_wakers() {
    let sandbox = builder().boot_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let out = tokio::spawn(async move {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).await.unwrap();
        bytes
    });
    let err = tokio::spawn(async move {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).await.unwrap();
        bytes
    });
    let status = tokio::spawn(async move {
        let status = child.wait().await.unwrap();
        (status, child)
    });
    let (out, err, status) = tokio::time::timeout(Duration::from_secs(5), async {
        futures::join!(out, err, status)
    })
    .await
    .unwrap();
    assert_eq!(out.unwrap(), vec![b'x'; 80 * 4096]);
    assert_eq!(err.unwrap(), vec![b'x'; 80 * 4096]);
    assert!(status.unwrap().0.success());
}

#[tokio::test]
async fn reparented_grandchildren_keep_their_trees_sockets() {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    listener.set_nonblocking(true).unwrap();
    let server = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        let mut sockets = Vec::new();
        while sockets.len() < 2 {
            match listener.accept() {
                Ok((socket, _)) => sockets.push(socket),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(
                        std::time::Instant::now() < deadline,
                        "guest did not connect"
                    );
                    std::thread::sleep(Duration::from_millis(1));
                }
                Err(error) => panic!("{error}"),
            }
        }
        for (i, socket) in sockets.iter_mut().enumerate() {
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            socket.write_all(&[b'a' + i as u8]).unwrap();
        }
        for (i, socket) in sockets.iter_mut().enumerate() {
            let mut reply = [0];
            socket.read_exact(&mut reply).unwrap();
            assert_eq!(reply, [b'a' + i as u8]);
        }
    });
    let sandbox = builder()
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .boot_commands()
        .unwrap();
    let mut a = sandbox
        .command("/mnt/bin/command")
        .args(["net-tree", &port])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let mut b = sandbox
        .command("/mnt/bin/command")
        .args(["net-tree", &port])
        .stdout(Stdio::piped())
        .spawn()
        .await
        .unwrap();
    let result = tokio::time::timeout(Duration::from_secs(5), async {
        let mut out_a = a.stdout.take().unwrap();
        let mut out_b = b.stdout.take().unwrap();
        let mut bytes_a = Vec::new();
        let mut bytes_b = Vec::new();
        let (a, b, _, _) = futures::try_join!(
            a.wait(),
            b.wait(),
            async {
                out_a
                    .read_to_end(&mut bytes_a)
                    .await
                    .map_err(hyperlight_unikraft::Error::Wait)
            },
            async {
                out_b
                    .read_to_end(&mut bytes_b)
                    .await
                    .map_err(hyperlight_unikraft::Error::Wait)
            },
        )
        .unwrap();
        assert_eq!(a.code(), Some(9));
        assert_eq!(b.code(), Some(9));
        assert_eq!(bytes_a, b"socket-descendant\n");
        assert_eq!(bytes_b, bytes_a);
    })
    .await;
    server.join().unwrap();
    result.unwrap();
}
