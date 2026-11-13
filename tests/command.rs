// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{
    FutureExt,
    io::{AsyncReadExt, AsyncWriteExt},
};
use hyperlight_unikraft::{Mount, SandboxBuilder, command::Stdio};

mod common;

fn builder() -> SandboxBuilder {
    common::require_bin("c", "command");
    SandboxBuilder::from_initrd(common::require_rootfs("c"))
        .mounts(vec![Mount::ro(common::require_bins("c"), "/mnt/bin")])
        .scratch_mb(128)
        .echo_output(false)
}

#[tokio::test]
async fn structured_command_and_distinct_streams() {
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let out = tokio::time::timeout(
        Duration::from_secs(5),
        sandbox
            .command("/mnt/bin/command")
            .args(["args", "a b", "", "quoted\"text"])
            .env_clear()
            .env("COMMAND_TEST", "one value")
            .current_dir("/tmp")
            .output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(out.status.code(), Some(17));
    assert_eq!(
        out.stdout,
        b"cwd=/tmp\nenv=one value\narg=a b\narg=\narg=quoted\"text\n"
    );
    assert_eq!(out.stderr, b"stderr\n");
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
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut a = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[1]"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let b = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[2]"])
        .stdout(Stdio::piped())
        .spawn()
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
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    for mode in ["sleep", "sleep-tree"] {
        let mut a = sandbox
            .command("/mnt/bin/command")
            .args([mode, "20", "a"])
            .spawn()
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
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("tree")
        .stdout(Stdio::piped())
        .spawn()
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
async fn dropping_a_child_cancels_calls_and_restoring_retries_with_eintr() {
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
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    assert!(sandbox.restore_child(id).is_err());
    assert!(child.wait().now_or_never().is_none());
    assert_eq!(count.load(Ordering::SeqCst), 1);
    drop(child);
    assert!(sender.is_canceled());
    assert_eq!(sandbox.parked_children(), [id]);
    assert!(sandbox.forget_child(id).is_err());
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
    let child = sandbox.restore_child(id).unwrap();
    drop(child);
    let child = sandbox.restore_child(id).unwrap();
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"interrupted\nretried");
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert!(
        sandbox
            .restore_child(id)
            .unwrap()
            .try_wait()
            .unwrap()
            .unwrap()
            .success()
    );
    sandbox.forget_child(id).unwrap();
    assert!(sandbox.restore_child(id).is_err());
}

#[tokio::test]
async fn parking_preserves_backpressured_output_and_invalidates_taken_pipes() {
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    let mut old_stdout = child.stdout.take().unwrap();
    for _ in 0..40 {
        assert!(child.wait().now_or_never().is_none());
    }
    let mut first = [0];
    old_stdout.read_exact(&mut first).await.unwrap();
    assert!(child.wait().now_or_never().is_none());
    drop(child);
    assert!(old_stdout.read(&mut first).await.is_err());
    let snapshot = sandbox.snapshot().unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let child = sandbox.restore_child(id).unwrap();
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
        fork.restore_child(id).unwrap().wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(fork_output.status.success());
    assert_eq!(fork_output.stdout, output.stdout);
    assert_eq!(fork_output.stderr, output.stderr);
}

#[tokio::test]
async fn parking_closes_host_connections_without_entering_the_vm() {
    use tokio::io::AsyncRead as _;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let sandbox = builder()
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["park-socket", &port])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    let mut output = child.stdout.take().unwrap();
    let mut ready = [0; 6];
    tokio::time::timeout(Duration::from_secs(5), output.read_exact(&mut ready))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&ready, b"ready\n");
    let (mut socket, _) = listener.accept().await.unwrap();
    drop(child);
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
    let snapshot = sandbox.snapshot().unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
        .echo_output(false)
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let child = sandbox.restore_child(id).unwrap();
    drop(output);
    let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"closed\n");
    let fork_output = tokio::time::timeout(
        Duration::from_secs(5),
        fork.restore_child(id).unwrap().wait_with_output(),
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
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["park-listener", &address.port().to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    stdout.read_exact(&mut [0; 6]).await.unwrap();
    drop(child);
    assert!(sender.is_canceled());
    let reservation = std::net::TcpListener::bind(address).unwrap();
    let mut child = sandbox.restore_child(id).unwrap();
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
fn parked_child_can_be_restored_and_killed_without_a_runtime() {
    let sandbox = builder()
        .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
            panic!("an undriven call must never be polled");
        })
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .unwrap();
    let id = child.execution_id();
    drop(child);
    let mut child = sandbox.restore_child(id).unwrap();
    child.kill().unwrap();
    assert_eq!(
        child.wait().now_or_never().unwrap().unwrap().signal(),
        Some(9)
    );
}

#[tokio::test]
async fn restoring_a_parked_timer_does_not_restart_its_duration() {
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep-tree", "200"])
        .spawn()
        .unwrap();
    let id = child.execution_id();
    drop(child);
    let snapshot = sandbox.snapshot().unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .echo_output(false)
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    tokio::time::sleep(Duration::from_millis(300)).await;
    for restored in [&sandbox, &fork] {
        let mut child = restored.restore_child(id).unwrap();
        assert!(child.try_wait().unwrap().is_none());
        let _ = child.wait().now_or_never();
        assert!(child.try_wait().unwrap().unwrap().success());
    }
}

#[tokio::test]
async fn host_future_can_drop_its_own_child_without_deadlocking() {
    let slot = Arc::new(Mutex::new(None));
    let owned = slot.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            drop(owned.lock().unwrap().take());
            std::future::ready(Ok("completed".into()))
        })
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    *slot.lock().unwrap() = Some(child);
    assert!(stdout.read(&mut [0]).await.is_err());
    let snapshot = sandbox.snapshot().unwrap();
    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
            panic!("a completed reply must survive the snapshot");
        })
        .echo_output(false)
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let fork_output = tokio::time::timeout(
        Duration::from_secs(5),
        fork.restore_child(id).unwrap().wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(fork_output.status.success());
    assert_eq!(fork_output.stdout, b"completed");
    let output = tokio::time::timeout(
        Duration::from_secs(5),
        sandbox.restore_child(id).unwrap().wait_with_output(),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"completed");
}

#[tokio::test]
async fn snapshots_restore_parked_commands_streams_and_completed_status() {
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut echo = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let echo_id = echo.execution_id();
    assert!(sandbox.snapshot().is_err());
    let mut old_stdin = echo.stdin.take().unwrap();
    old_stdin.write_all(b"retained input\n").await.unwrap();
    let mut done = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "retained output"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let done_id = done.execution_id();
    assert!(done.wait().await.unwrap().success());
    drop(echo);
    drop(done);
    let snapshot = sandbox.snapshot().unwrap();

    let fork = SandboxBuilder::from_command_snapshot(&snapshot)
        .mount(Mount::ro(common::require_bins("c"), "/mnt/bin"))
        .echo_output(false)
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    assert_eq!(fork.parked_children(), [echo_id, done_id]);
    let active = sandbox.restore_child(echo_id).unwrap();
    assert!(sandbox.restore(&snapshot).is_err());
    drop(active);
    for restored in [&fork, &sandbox] {
        let echo = restored.restore_child(echo_id).unwrap();
        let done = restored.restore_child(done_id).unwrap();
        assert!(done.try_wait().unwrap().unwrap().success());
        let (echo, done) = tokio::time::timeout(Duration::from_secs(5), async {
            futures::try_join!(echo.wait_with_output(), done.wait_with_output())
        })
        .await
        .unwrap()
        .unwrap();
        assert_eq!(echo.stdout, b"retained input\n");
        assert_eq!(done.stdout, b"retained output\n");
        assert!(echo.status.success() && done.status.success());
    }

    let later = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "later"])
        .spawn()
        .unwrap();
    let later_id = later.execution_id();
    drop(later);
    sandbox.restore(&snapshot).unwrap();
    assert_eq!(sandbox.parked_children(), [echo_id, done_id]);
    assert!(sandbox.restore_child(later_id).is_err());
    assert!(old_stdin.write_all(b"stale").await.is_err());
    drop(old_stdin);
    let echo = sandbox
        .restore_child(echo_id)
        .unwrap()
        .wait_with_output()
        .await
        .unwrap();
    assert_eq!(echo.stdout, b"retained input\n");
    let fresh = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "fresh"])
        .spawn()
        .unwrap();
    assert!(fresh.execution_id() > later_id);
}

#[tokio::test]
async fn snapshot_forks_interrupt_pending_calls_without_starting_handlers_at_boot() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let original_count = Arc::new(AtomicUsize::new(0));
    let called = original_count.clone();
    let sandbox = builder()
        .async_host_function("gate", move |_| {
            let first = called.fetch_add(1, Ordering::SeqCst) == 0;
            async move {
                if first {
                    std::future::pending::<()>().await;
                }
                Ok("original".into())
            }
        })
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    assert!(child.wait().now_or_never().is_none());
    assert_eq!(original_count.load(Ordering::SeqCst), 1);
    drop(child);
    let snapshot = sandbox.snapshot().unwrap();
    for _ in 0..2 {
        let count = Arc::new(AtomicUsize::new(0));
        let called = count.clone();
        let fork = SandboxBuilder::from_command_snapshot(&snapshot)
            .async_host_function("gate", move |_| {
                called.fetch_add(1, Ordering::SeqCst);
                std::future::ready(Ok("fork".into()))
            })
            .echo_output(false)
            .boot()
            .unwrap()
            .into_commands()
            .unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        let child = fork.restore_child(id).unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        let output = tokio::time::timeout(Duration::from_secs(5), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"interrupted\nfork");
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
    let output = sandbox
        .restore_child(id)
        .unwrap()
        .wait_with_output()
        .await
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"interrupted\noriginal");
}

#[test]
fn snapshot_and_reattachment_reject_a_poll_still_finishing_detachment() {
    use std::sync::{
        Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    let (entered, receive) = mpsc::channel();
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
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let id = child.execution_id();
    let mut stdout = child.stdout.take().unwrap();
    let poller = std::thread::spawn(move || stdout.read(&mut [0]).now_or_never());
    receive.recv_timeout(Duration::from_secs(5)).unwrap();
    drop(child);
    for result in [
        sandbox.restore_child(id).map(|_| ()),
        sandbox.snapshot().map(|_| ()),
    ] {
        assert!(
            matches!(result, Err(hyperlight_unikraft::Error::Wait(error))
            if error.kind() == std::io::ErrorKind::WouldBlock)
        );
    }
    let mut other = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "0", "other"])
        .spawn()
        .unwrap();
    assert!(other.wait().now_or_never().unwrap().unwrap().success());
    drop(other);
    release.wait();
    assert!(poller.join().unwrap().unwrap().is_err());
    assert_eq!(count.load(Ordering::SeqCst), 1);
    sandbox.snapshot().unwrap();
    let mut child = sandbox.restore_child(id).unwrap();
    child.kill().unwrap();
}

#[tokio::test]
async fn root_exit_cancels_descendant_host_calls_without_polling_them() {
    for collect_output in [false, true] {
        let sandbox = builder()
            .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
                panic!("a descendant must not be driven after root exit");
            })
            .boot()
            .unwrap()
            .into_commands()
            .unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .arg("tree-host")
            .spawn()
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
            child.kill().unwrap();
        }
    }
}

#[tokio::test]
async fn bidirectional_pipes_and_bounded_output() {
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
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
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    assert!(sandbox.command("/missing").spawn().is_err());
    assert!(sandbox.command("relative").spawn().is_err());
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .current_dir("")
            .spawn()
            .is_err()
    );
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .arg("bad\0argument")
            .spawn()
            .is_err()
    );
    assert!(
        sandbox
            .command("/mnt/bin/command")
            .env("BAD=KEY", "value")
            .spawn()
            .is_err()
    );
}

#[tokio::test]
async fn completed_contexts_do_not_exhaust_the_live_execution_limit() {
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        let mut completed = Vec::new();
        for _ in 0..1030 {
            let mut child = sandbox
                .command("/mnt/bin/command")
                .args(["sleep", "0", ""])
                .spawn()
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

#[test]
fn interpreter_sandboxes_reject_the_command_api() {
    assert!(
        SandboxBuilder::from_initrd(common::require_rootfs("python"))
            .scratch_mb(256)
            .echo_output(false)
            .boot()
            .unwrap()
            .into_commands()
            .is_err()
    );
}

#[tokio::test]
async fn compiled_runtime_drivers_launch_independent_executables() {
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
            .boot()
            .unwrap()
            .into_commands()
            .unwrap();
        let path = format!("/mnt/bin/{program}");
        let a = sandbox
            .command(&path)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let b = sandbox
            .command(&path)
            .stdout(Stdio::piped())
            .spawn()
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
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "60000", "late"])
        .spawn()
        .unwrap();
    assert!(child.wait().now_or_never().is_none());
    child.kill().unwrap();
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
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .spawn()
        .unwrap();
    assert!(child.wait().now_or_never().is_none());
    assert!(!dropped.load(Ordering::SeqCst));
    child.kill().unwrap();
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
            .boot()
            .unwrap()
            .into_commands()
            .unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .args(["kill-tree", root_mode])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
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
                child.kill().unwrap();
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
            child.kill().unwrap();
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

    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
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
        child.kill().unwrap();
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
        child.kill().unwrap();
        assert_eq!(entries.load(Ordering::SeqCst), 1);
    });
    for output in [out, err] {
        assert_eq!(output.len(), 64 * 1024);
        assert!(output.iter().all(|byte| *byte == b'x'));
    }
}

#[test]
fn kill_without_a_runtime_does_not_poll_queued_host_functions() {
    let sandbox = builder()
        .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
            panic!("kill must cancel, not poll, the host function");
        })
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["host", "[]"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    child.kill().unwrap();
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

#[tokio::test]
async fn kill_closes_only_its_trees_inherited_socket() {
    tokio::time::timeout(Duration::from_secs(10), async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port().to_string();
        let sandbox = builder()
            .network(hyperlight_unikraft::NetworkPolicy::AllowAll)
            .boot()
            .unwrap()
            .into_commands()
            .unwrap();
        let mut a = sandbox
            .command("/mnt/bin/command")
            .args(["net-tree", &port])
            .spawn()
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
            .unwrap();
        let peer_b = match futures::future::select(Box::pin(b.wait()), Box::pin(listener.accept()))
            .await
        {
            futures::future::Either::Right((peer, _)) => peer.unwrap().0,
            futures::future::Either::Left((status, _)) => panic!("root exited early: {status:?}"),
        };

        assert!(a.wait().now_or_never().is_none());
        a.kill().unwrap();
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
    let sandbox = builder().boot().unwrap().into_commands().unwrap();
    let mut child = sandbox
        .command("/mnt/bin/command")
        .arg("large")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
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
        .boot()
        .unwrap()
        .into_commands()
        .unwrap();
    let mut a = sandbox
        .command("/mnt/bin/command")
        .args(["net-tree", &port])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut b = sandbox
        .command("/mnt/bin/command")
        .args(["net-tree", &port])
        .stdout(Stdio::piped())
        .spawn()
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
