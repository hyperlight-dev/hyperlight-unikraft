// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use hyperlight_unikraft::{AppSandbox, Error, SandboxBuilder, Yield};

fn python() -> SandboxBuilder {
    SandboxBuilder::from_initrd(common::require_rootfs("python"))
        .scratch_mb(256)
        .echo_output(false)
}

fn script() -> String {
    format!(
        r#"{}
import errno
try:
    value = raw_call("work", b"[7]")
    print("value=" + value.decode(), flush=True)
except OSError as e:
    assert e.errno == errno.EINTR, e
    print("interrupted", flush=True)
os.close(fd)
"#,
        common::RAW_HOST_CALL
    )
}

async fn finish(sandbox: &mut AppSandbox) {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match sandbox.step().await.unwrap() {
                Yield::CallDone => break,
                Yield::Blocked { .. } => {}
                other => panic!("{other:?}: {}", sandbox.drain_output()),
            }
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn snapshot_leaves_original_work_and_default_restore_interrupts() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let mut sandbox = python()
        .async_host_function("work", move |args| {
            assert_eq!(args, "[7]");
            count.fetch_add(1, Ordering::SeqCst);
            async { Ok("7".into()) }
        })
        .boot()
        .unwrap();
    sandbox.submit(script()).unwrap();
    let snapshots = [
        sandbox.snapshot().unwrap(),
        sandbox.snapshot_with_scratch_mb(512).unwrap(),
    ];
    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "snapshot polled the factory"
    );
    finish(&mut sandbox).await;
    assert!(sandbox.drain_output().contains("value=7"));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    for snapshot in snapshots.iter().cycle().take(6) {
        sandbox.restore(snapshot.clone()).unwrap();
        finish(&mut sandbox).await;
        assert!(sandbox.drain_output().contains("interrupted"));
        assert_eq!(calls.load(Ordering::SeqCst), 1, "original call replayed");
        sandbox.snapshot().unwrap();
    }
    sandbox
        .run("import hyperlight; assert hyperlight.call('work', 7) == 7")
        .await
        .unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn delivered_results_survive_beside_pending_calls_on_a_new_host() {
    let mut sandbox = python()
        .async_host_function("done", |_| async { Ok("7".into()) })
        .async_host_function("work", |_| async { panic!("must not poll original") })
        .boot()
        .unwrap();
    sandbox
        .submit(format!(
            r#"{}
import errno
value = raw_call("done", b"[]")
print("delivered", flush=True)
try:
    raw_call("work", b"[7]")
    raise AssertionError("not interrupted")
except OSError as e:
    assert e.errno == errno.EINTR, e
assert value == b"7", value
os.close(fd)
print("preserved", flush=True)
"#,
            common::RAW_HOST_CALL
        ))
        .unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        while !sandbox.drain_output().contains("delivered") {
            assert!(matches!(
                sandbox.step().await.unwrap(),
                Yield::Blocked { .. }
            ));
        }
    })
    .await
    .unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    drop(sandbox);
    let mut restored = SandboxBuilder::from_snapshot(snapshot)
        .async_host_function("done", |_| async {
            panic!("must not replay delivered call")
        })
        .async_host_function("work", |_| async { panic!("must not replay pending call") })
        .echo_output(false)
        .boot()
        .unwrap();
    finish(&mut restored).await;
    assert!(restored.drain_output().contains("preserved"));
    restored.snapshot().unwrap();
}

struct Active(Arc<AtomicUsize>);

impl Drop for Active {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn restore_drops_the_old_running_future_without_replaying() {
    let calls = Arc::new(AtomicUsize::new(0));
    let active = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let running = active.clone();
    let mut sandbox = python()
        .async_host_function("work", move |args| {
            assert_eq!(args, "[7]");
            count.fetch_add(1, Ordering::SeqCst);
            running.fetch_add(1, Ordering::SeqCst);
            let active = Active(running.clone());
            async move {
                let _active = active;
                std::future::pending().await
            }
        })
        .boot()
        .unwrap();
    let code = format!(
        "import threading, time\n\
         def tick():\n    time.sleep(0.02)\n    print('checkpoint', flush=True)\n\
         threading.Thread(target=tick).start()\n{}",
        script()
    );
    sandbox.submit(code).unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        while !sandbox.drain_output().contains("checkpoint") {
            sandbox.step().await.unwrap();
        }
    })
    .await
    .unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(active.load(Ordering::SeqCst), 1);
    let snapshot = sandbox.snapshot().unwrap();
    assert_eq!(active.load(Ordering::SeqCst), 1);
    for _ in 0..3 {
        sandbox.restore(snapshot.clone()).unwrap();
        assert_eq!(active.load(Ordering::SeqCst), 0);
        sandbox.snapshot().unwrap();
        finish(&mut sandbox).await;
        assert!(sandbox.drain_output().contains("interrupted"));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn pending_snapshot_restores_in_a_fresh_process() {
    let dir = common::temp_dir("pending-call-snapshot");
    let mut sandbox = python()
        .async_host_function("work", |_| async {
            panic!("snapshot must not poll original")
        })
        .boot()
        .unwrap();
    sandbox.submit(script()).unwrap();
    sandbox.snapshot_to(dir.path()).unwrap();
    drop(sandbox);
    for mode in ["default", "missing", "sync"] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "restore_child", "--ignored", "--nocapture"])
            .env("HLUK_RESTORE_TEST_SNAPSHOT", dir.path())
            .env("HLUK_RESTORE_TEST_MODE", mode)
            .output()
            .unwrap();
        assert!(output.status.success(), "{mode}: {output:?}");
    }
}

#[test]
#[ignore = "invoked by pending_snapshot_restores_in_a_fresh_process with a saved snapshot"]
fn restore_child() {
    let path = std::env::var_os("HLUK_RESTORE_TEST_SNAPSHOT").expect("child snapshot");
    let mode = std::env::var("HLUK_RESTORE_TEST_MODE").expect("child mode");
    let builder = SandboxBuilder::from_snapshot_dir(path)
        .unwrap()
        .echo_output(false);
    let builder = match mode.as_str() {
        "default" => builder.async_host_function("work", |_| async { panic!("must not replay") }),
        "missing" => builder,
        "sync" => builder.host_function("work", |_| panic!("must not replay synchronously")),
        _ => panic!("unknown child mode"),
    };
    // Interruption needs neither a handler invocation nor an async runtime.
    let mut sandbox = builder.boot().unwrap();
    assert!(sandbox.drain_output().contains("interrupted"));
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    assert_eq!(runtime.block_on(sandbox.step()).unwrap(), Yield::CallDone);
    sandbox.snapshot().unwrap();
}

#[tokio::test]
async fn restore_interrupts_all_waiters_without_completion_batches() {
    let mut sandbox = python()
        .async_host_function("work", |_| async { panic!("must not replay") })
        .profile(true)
        .boot()
        .unwrap();
    let code = format!(
        r#"{}
import errno, json
results = []
def worker(i):
    try:
        raw_call("work", json.dumps([i, "z" * 2048]).encode(), 4096)
        raise AssertionError("not interrupted")
    except OSError as e:
        assert e.errno == errno.EINTR, e
        results.append(i)
threads = [threading.Thread(target=worker, args=(i,)) for i in range(130)]
for t in threads: t.start()
for t in threads: t.join()
assert sorted(results) == list(range(130)), results
os.close(fd)
print("all interrupted")
"#,
        common::RAW_HOST_CALL
    );
    sandbox.submit(code).unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    for _ in 0..3 {
        sandbox.profile().reset();
        sandbox.restore(snapshot.clone()).unwrap();
        finish(&mut sandbox).await;
        assert!(sandbox.drain_output().contains("all interrupted"));
        let report = sandbox.profile().report();
        assert!(!report.contains("entry step_async "), "{report}");
        assert!(!report.contains("RestoreHostCall"), "{report}");
        sandbox.snapshot().unwrap();
    }
}

#[tokio::test]
async fn interruption_reaches_the_language_driver_as_a_transport_error() {
    let mut sandbox = python()
        .async_host_function("work", |_| async { panic!("must not replay") })
        .boot()
        .unwrap();
    sandbox.submit(
        "import hyperlight\ntry:\n    hyperlight.call('work')\n    raise AssertionError('expected failure')\n\
         except hyperlight.HostError:\n    raise AssertionError('not a host result error')\n\
         except RuntimeError as e:\n    assert 'Interrupted system call' in str(e), e\nprint('interrupted')"
    ).unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    sandbox.restore(snapshot).unwrap();
    finish(&mut sandbox).await;
    assert!(sandbox.drain_output().contains("interrupted"));
    sandbox.snapshot().unwrap();
}

#[tokio::test]
async fn pending_snapshot_recovers_a_cancelled_drive_by_interrupting() {
    let active = Arc::new(AtomicUsize::new(0));
    let running = active.clone();
    let mut sandbox = python()
        .async_host_function("work", move |_| {
            running.fetch_add(1, Ordering::SeqCst);
            let active = Active(running.clone());
            async move {
                let _active = active;
                std::future::pending().await
            }
        })
        .boot()
        .unwrap();
    sandbox.submit(script()).unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    let mut drive = Box::pin(sandbox.step());
    assert!(futures::poll!(drive.as_mut()).is_pending());
    assert_eq!(active.load(Ordering::SeqCst), 1);
    drop(drive);
    assert_eq!(active.load(Ordering::SeqCst), 0);
    assert!(matches!(
        sandbox.snapshot(),
        Err(Error::Hyperlight(
            hyperlight_unikraft::hyperlight_host::HyperlightError::PoisonedSandbox
        ))
    ));
    sandbox.restore(snapshot).unwrap();
    finish(&mut sandbox).await;
    assert_eq!(active.load(Ordering::SeqCst), 0);
    assert!(sandbox.drain_output().contains("interrupted"));
    sandbox.snapshot().unwrap();
}

#[tokio::test]
async fn pending_listener_wait_restores_on_a_new_host_and_keeps_serving() {
    use hyperlight_unikraft::NetworkPolicy;

    let port = {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        listener.local_addr().unwrap().port()
    };
    let mut sandbox = python()
        .network(NetworkPolicy::AllowAll)
        .listen_ports(hyperlight_unikraft::ListenPorts::from_ports([port]))
        .boot()
        .unwrap();
    sandbox
        .run(format!(
            r#"
import socket, threading
listener = socket.socket()
listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
listener.bind(("127.0.0.1", {port}))
listener.listen(1)
def serve():
    conn, _ = listener.accept()
    conn.sendall(b"pong")
    conn.close()
worker = threading.Thread(target=serve)
worker.start()
"#
        ))
        .await
        .unwrap();
    sandbox.snapshot().unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    drop(sandbox);
    for _ in 0..3 {
        let mut restored = SandboxBuilder::from_snapshot(snapshot.clone())
            .network(NetworkPolicy::AllowAll)
            .listen_ports(hyperlight_unikraft::ListenPorts::from_ports([port]))
            .echo_output(false)
            .profile(true)
            .boot()
            .unwrap();
        tokio::time::timeout(Duration::from_secs(5), async {
            let (guest, ()) =
                tokio::join!(restored.run("worker.join(); listener.close()"), async {
                    let client = tokio::net::TcpStream::connect(("127.0.0.1", port))
                        .await
                        .unwrap();
                    let mut response = [0; 4];
                    let mut read = 0;
                    while read < response.len() {
                        client.readable().await.unwrap();
                        match client.try_read(&mut response[read..]) {
                            Ok(0) => panic!("server closed before replying"),
                            Ok(n) => read += n,
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                            Err(e) => panic!("reading server response: {e}"),
                        }
                    }
                    assert_eq!(&response, b"pong");
                });
            guest.unwrap();
            restored.step().await.unwrap();
        })
        .await
        .unwrap();
        let report = restored.profile().report();
        assert!(report.contains("host net_wait_async "), "{report}");
        assert!(report.contains("entry step_async "), "{report}");
        restored.snapshot().unwrap();
    }
}
