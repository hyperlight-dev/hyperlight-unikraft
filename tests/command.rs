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
async fn descendant_output_outlives_root_wait() {
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
    drop(child);
    let mut output = Vec::new();
    tokio::time::timeout(Duration::from_secs(5), stdout.read_to_end(&mut output))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(output, b"descendant\n");
}

#[tokio::test]
async fn whole_tree_waits_remain_pending_after_root_exit_without_output_pipes() {
    for collect_output in [false, true] {
        let (release, ready) = futures::channel::oneshot::channel::<()>();
        let ready = Arc::new(Mutex::new(Some(ready)));
        let sandbox = builder()
            .async_host_function("gate", move |_| {
                let ready = ready.lock().unwrap().take().unwrap();
                async move {
                    ready.await.map_err(|error| error.to_string())?;
                    Ok("[]".into())
                }
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
        let root = tokio::time::timeout(Duration::from_secs(5), child.wait())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(root.code(), Some(7));
        assert_eq!(child.try_wait().unwrap(), Some(root));
        assert_eq!(child.wait().now_or_never().unwrap().unwrap(), root);
        assert!(child.wait_all().now_or_never().is_none());
        // Root-only kill must not wait on the still-blocked descendant.
        child.kill().now_or_never().unwrap().unwrap();

        if collect_output {
            let mut output = Box::pin(child.wait_with_output());
            // With no captured pipes, output waits only for the root.
            let output = output.as_mut().now_or_never().unwrap().unwrap();
            release.send(()).unwrap();
            assert_eq!(output.status, root);
            assert!(output.stdout.is_empty() && output.stderr.is_empty());
        } else {
            release.send(()).unwrap();
            let status = tokio::time::timeout(Duration::from_secs(5), child.wait_all())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(status, root);
            assert_eq!(child.wait_all().now_or_never().unwrap().unwrap(), root);
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
            assert!(child.wait_all().await.unwrap().success());
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
    tokio::time::timeout(Duration::from_secs(3), child.kill())
        .await
        .unwrap()
        .unwrap();
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
    tokio::time::timeout(Duration::from_secs(3), child.kill())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(child.try_wait().unwrap().unwrap().signal(), Some(9));
    assert!(dropped.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn kill_all_ends_reparented_descendants_and_drops_their_host_calls() {
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Dropped(Arc<AtomicBool>);
    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    for root_mode in ["live", "exit"] {
        let started = Arc::new(AtomicBool::new(false));
        let dropped = Arc::new(AtomicBool::new(false));
        let observed_start = started.clone();
        let observed_drop = dropped.clone();
        let sandbox = builder()
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
                assert!(child.wait_all().now_or_never().is_none());
                tokio::task::yield_now().await;
            }
            if root_mode == "exit" {
                assert_eq!(child.wait().await.unwrap().code(), Some(7));
            } else {
                assert!(child.try_wait().unwrap().is_none());
            }
            child.kill_all().unwrap();
            let status = child.wait_all().now_or_never().unwrap().unwrap();
            let mut output = Vec::new();
            stdout
                .read_to_end(&mut output)
                .now_or_never()
                .unwrap()
                .unwrap();
            assert!(b"before kill\n".starts_with(&output));
            if root_mode == "exit" {
                assert_eq!(status.code(), Some(7));
            } else {
                assert_eq!(status.signal(), Some(9));
            }
            assert!(dropped.load(Ordering::SeqCst));
            assert!(stdin.write_all(b"closed").await.is_err());
            child.kill_all().unwrap();
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
async fn synchronous_kill_all_preserves_only_host_buffered_output_in_one_entry() {
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
        tokio::time::timeout(Duration::from_millis(50), child.wait_all())
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
        child.kill_all().unwrap();
        assert_eq!(entries.load(Ordering::SeqCst), 1);
        let status = child.wait_all().now_or_never().unwrap().unwrap();
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
        child.kill_all().unwrap();
        assert_eq!(entries.load(Ordering::SeqCst), 1);
    });
    for output in [out, err] {
        assert_eq!(output.len(), 64 * 1024);
        assert!(output.iter().all(|byte| *byte == b'x'));
    }
}

#[test]
fn kill_all_without_a_runtime_does_not_poll_queued_host_functions() {
    let sandbox = builder()
        .async_host_function("gate", |_| -> std::future::Ready<Result<String, String>> {
            panic!("kill_all must cancel, not poll, the host function");
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
    child.kill_all().unwrap();
    let status = child.wait_all().now_or_never().unwrap().unwrap();
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
async fn kill_all_closes_only_its_trees_inherited_socket() {
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
        assert_eq!(a.wait().await.unwrap().code(), Some(9));
        let (peer_a, _) = listener.accept().await.unwrap();
        let mut b = sandbox
            .command("/mnt/bin/command")
            .args(["net-tree", &port])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        assert_eq!(b.wait().await.unwrap().code(), Some(9));
        let (peer_b, _) = listener.accept().await.unwrap();

        assert!(a.wait_all().now_or_never().is_none());
        a.kill_all().unwrap();
        assert_eq!(
            a.wait_all().now_or_never().unwrap().unwrap().code(),
            Some(9)
        );
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
    let status = tokio::spawn(async move { child.wait_all().await.unwrap() });
    let (out, err, status) = tokio::time::timeout(Duration::from_secs(5), async {
        futures::join!(out, err, status)
    })
    .await
    .unwrap();
    assert_eq!(out.unwrap(), vec![b'x'; 80 * 4096]);
    assert_eq!(err.unwrap(), vec![b'x'; 80 * 4096]);
    assert!(status.unwrap().success());
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
            a.wait_all(),
            b.wait_all(),
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
