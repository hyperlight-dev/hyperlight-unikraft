// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use hyperlight_unikraft::{Error, NetworkPolicy, SandboxBuilder};

mod common;

fn python() -> SandboxBuilder {
    SandboxBuilder::from_initrd(common::require_rootfs("python"))
        .scratch_mb(256)
        .echo_output(false)
}

#[tokio::test]
async fn async_values_errors_and_sync_fast_path() {
    let mut sandbox = python()
        .host_function("sync", |_| Ok("42".into()))
        .async_host_function("later", |args| async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok(args)
        })
        .async_host_function("fail", |_| async { Err("async failure".into()) })
        .boot()
        .unwrap();
    // A synchronous call does not become async merely by registering an async handler.
    sandbox
        .run("import hyperlight; assert hyperlight.call('sync') == 42")
        .unwrap();
    tokio::time::timeout(
        Duration::from_secs(5),
        sandbox.run_async(
            r#"
import hyperlight
assert hyperlight.call("later", 7, "value") == [7, "value"]
try:
    hyperlight.call("fail")
    raise AssertionError("expected a host error")
except hyperlight.HostError as e:
    assert "async failure" in str(e)
def echo(value):
    return hyperlight.call("later", value)[0]
"#,
        ),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        sandbox.call_async("echo", r#""hello""#).await.unwrap(),
        r#""hello""#
    );
}

#[tokio::test]
async fn dropping_drive_future_poisons_and_drops_work() {
    let (send, recv) = futures::channel::oneshot::channel::<String>();
    let recv = Arc::new(Mutex::new(Some(recv)));
    let mut sandbox = python()
        .async_host_function("wait", move |_| {
            let recv = recv.lock().unwrap().take().expect("invoked only once");
            async move { recv.await.map_err(|e| e.to_string()) }
        })
        .boot()
        .unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    // An unpolled future has not started driving and must not poison.
    drop(sandbox.run_async("print('never submitted')"));
    sandbox.snapshot().unwrap();
    sandbox
        .submit("import hyperlight; assert hyperlight.call('wait') == 7")
        .unwrap();
    assert!(matches!(
        sandbox.snapshot(),
        Err(Error::AsyncSnapshotPending)
    ));
    assert!(matches!(
        sandbox.step(Duration::ZERO),
        Err(Error::AsyncHostCallsPending)
    ));
    assert!(
        tokio::time::timeout(Duration::from_millis(30), sandbox.step_async(),)
            .await
            .is_err()
    );
    assert!(send.is_canceled());
    fn poisoned<T>(result: Result<T, Error>) {
        match result {
            Err(error) => assert!(
                matches!(
                    error,
                    Error::Hyperlight(
                        hyperlight_unikraft::hyperlight_host::HyperlightError::PoisonedSandbox
                    )
                ),
                "{error:?}"
            ),
            Ok(_) => panic!("poisoned sandbox accepted an operation"),
        }
    }
    poisoned(sandbox.snapshot());
    poisoned(sandbox.step(Duration::ZERO));
    poisoned(sandbox.step_async().await);
    poisoned(sandbox.submit("print('no')"));
    poisoned(sandbox.join());
    poisoned(sandbox.join_async().await);
    sandbox.restore(snapshot).unwrap();
    sandbox.run_async("print('recovered')").await.unwrap();
    sandbox.snapshot().unwrap();
}

#[tokio::test]
async fn restore_discards_pending_operations_without_replaying() {
    let (send, recv) = futures::channel::oneshot::channel::<String>();
    let recv = Arc::new(Mutex::new(Some(recv)));
    let mut sandbox = python()
        .async_host_function("wait", move |_| {
            let recv = recv.lock().unwrap().take().expect("must not replay");
            async move { recv.await.map_err(|e| e.to_string()) }
        })
        .boot()
        .unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    assert!(
        tokio::time::timeout(
            Duration::from_millis(30),
            sandbox.run_async("import hyperlight; hyperlight.call('wait')"),
        )
        .await
        .is_err()
    );
    sandbox.restore(snapshot.clone()).unwrap();
    assert!(send.is_canceled());
    sandbox.run_async("print('restored')").await.unwrap();
    assert!(sandbox.drain_output().contains("restored"));
    let mut fresh = SandboxBuilder::from_snapshot(snapshot)
        .async_host_function("wait", |_| async { Ok("42".into()) })
        .echo_output(false)
        .boot()
        .unwrap();
    fresh
        .run_async("import hyperlight; assert hyperlight.call('wait') == 42")
        .await
        .unwrap();
}

// Python's high-level host-call helper intentionally only accepts its driver
// thread. Use the unchanged ioctl ABI to exercise independent guest waiters.
const RAW_HOST_CALL: &str = r#"
import array, fcntl, os, struct, threading
fd = os.open("/dev/hlcall", os.O_RDWR)
def raw_call(name, args=b"[]", capacity=1024):
    name = array.array("B", name.encode())
    args = array.array("B", args)
    out = array.array("B", b"\0" * capacity)
    request = bytearray(struct.pack("=7Q", name.buffer_info()[0], len(name),
                        args.buffer_info()[0], len(args), out.buffer_info()[0], len(out), 0))
    fcntl.ioctl(fd, 0xc0384803, request)
    n = struct.unpack("=7Q", request)[6]
    result = out.tobytes()[:n]
    assert result[0] == 0, result
    return result[1:]
"#;

#[tokio::test]
async fn two_guest_threads_park_independently_and_complete_out_of_order() {
    let (send, recv) = futures::channel::oneshot::channel::<()>();
    let send = Arc::new(Mutex::new(Some(send)));
    let recv = Arc::new(Mutex::new(Some(recv)));
    let mut sandbox = python()
        .async_host_function("first", move |_| {
            let recv = recv.lock().unwrap().take().unwrap();
            async move {
                recv.await.map_err(|e| e.to_string())?;
                tokio::time::sleep(Duration::from_millis(20)).await;
                Ok("1".into())
            }
        })
        .async_host_function("second", move |_| {
            let send = send.lock().unwrap().take().unwrap();
            async move {
                send.send(()).map_err(|_| "first was cancelled")?;
                Ok("2".into())
            }
        })
        .boot()
        .unwrap();
    let code = format!(
        r#"{RAW_HOST_CALL}
results = []
def worker(name):
    results.append(raw_call(name))
first = threading.Thread(target=worker, args=("first",))
second = threading.Thread(target=worker, args=("second",))
first.start()
second.start()
first.join()
second.join()
assert results == [b"2", b"1"], results
os.close(fd)
"#
    );
    tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(code))
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn maximum_results_and_small_guest_buffers() {
    let mut sandbox = python()
        .async_host_function("large", |_| async { Ok("7".repeat(69_619)) })
        .async_host_function("too_large", |_| async { Ok("7".repeat(69_620)) })
        .boot()
        .unwrap();
    let code = format!(
        r#"{RAW_HOST_CALL}
assert len(raw_call("large", capacity=69620)) == 69619
import errno
try:
    raw_call("large", capacity=1)
    raise AssertionError("expected ENOBUFS")
except OSError as e:
    assert e.errno == errno.ENOBUFS, e
import hyperlight
try:
    hyperlight.call("too_large")
    raise AssertionError("expected a host error")
except hyperlight.HostError as e:
    assert "exceeds" in str(e)
os.close(fd)
"#
    );
    tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(code))
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn multiple_sandboxes_share_one_executor() {
    let make = || {
        python()
            .async_host_function("wait", |_| async {
                tokio::time::sleep(Duration::from_millis(25)).await;
                Ok("null".into())
            })
            .boot()
            .unwrap()
    };
    let mut first = make();
    let mut second = make();
    let code = "import hyperlight; hyperlight.call('wait')";
    let (a, b) = futures::join!(first.run_async(code), second.run_async(code));
    a.unwrap();
    b.unwrap();
}

#[tokio::test]
async fn dropping_run_with_only_a_guest_timer_also_poisons() {
    let mut sandbox = python().boot().unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    assert!(
        tokio::time::timeout(
            Duration::from_millis(25),
            sandbox.run_async("import time; time.sleep(60)")
        )
        .await
        .is_err()
    );
    assert!(matches!(
        sandbox.snapshot(),
        Err(Error::Hyperlight(
            hyperlight_unikraft::hyperlight_host::HyperlightError::PoisonedSandbox
        ))
    ));
    sandbox.restore(snapshot).unwrap();
    sandbox.run("print('sync recovery')").unwrap();
}

#[tokio::test]
async fn ready_completions_share_an_entry_and_sync_calls_do_not_yield() {
    let mut sandbox = python()
        .profile(true)
        .host_function("sync", |_| Ok("1".into()))
        .async_host_function("ready", |_| async { Ok("2".into()) })
        .boot()
        .unwrap();
    sandbox.profile().reset();
    sandbox
        .run("import hyperlight\nfor _ in range(10): hyperlight.call('sync')")
        .unwrap();
    let sync = sandbox.profile().report();
    assert!(!sync.contains("entry step"), "{sync}");
    let calls = sync
        .lines()
        .find_map(|line| line.strip_prefix("host HostCall "))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();
    assert_eq!(calls, "10", "{sync}");

    sandbox.profile().reset();
    let code = format!(
        r#"{RAW_HOST_CALL}
results = []
threads = [threading.Thread(target=lambda: results.append(raw_call("ready"))) for _ in range(2)]
for t in threads: t.start()
for t in threads: t.join()
assert results == [b"2", b"2"], results
os.close(fd)
"#
    );
    sandbox.run_async(code).await.unwrap();
    let profile = sandbox.profile().report();
    let entries = profile
        .lines()
        .find_map(|line| line.strip_prefix("entry step_async "))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();
    assert_eq!(entries, "1", "{profile}");
}

#[tokio::test]
async fn host_calls_do_not_starve_guest_socket_or_timer_waits() {
    use std::io::Write;
    use std::sync::atomic::{AtomicBool, Ordering};

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        std::thread::sleep(Duration::from_millis(60));
        stream.write_all(b"wake").unwrap();
    });
    let (send, recv) = futures::channel::oneshot::channel::<String>();
    let send = Arc::new(Mutex::new(Some(send)));
    let recv = Arc::new(Mutex::new(Some(recv)));
    let timer_fired = Arc::new(AtomicBool::new(false));
    let timer = timer_fired.clone();
    let mut sandbox = python()
        .network(NetworkPolicy::AllowAll)
        .host_function("socket_ready", move |_| {
            send.lock()
                .unwrap()
                .take()
                .unwrap()
                .send("7".into())
                .unwrap();
            Ok("null".into())
        })
        .host_function("timer_ready", move |_| {
            timer.store(true, Ordering::Relaxed);
            Ok("null".into())
        })
        .async_host_function("wait", move |_| {
            let recv = recv.lock().unwrap().take().unwrap();
            async move { recv.await.map_err(|e| e.to_string()) }
        })
        .boot()
        .unwrap();
    let code = format!(
        r#"{RAW_HOST_CALL}
import hyperlight, socket, time
sock = socket.create_connection(("127.0.0.1", {port}))
def receive():
    assert sock.recv(4) == b"wake"
    raw_call("socket_ready")
def timer():
    time.sleep(0.01)
    raw_call("timer_ready")
threads = [threading.Thread(target=receive), threading.Thread(target=timer)]
for t in threads: t.start()
assert hyperlight.call("wait") == 7
for t in threads: t.join()
sock.close()
os.close(fd)
"#
    );
    tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(code))
        .await
        .unwrap()
        .unwrap();
    assert!(timer_fired.load(Ordering::Relaxed));
    server.join().unwrap();
}

#[tokio::test]
async fn join_async_preserves_exit_and_idle_driver_semantics() {
    let mut sandbox = python().boot().unwrap();
    assert!(matches!(
        sandbox.join_async().await,
        Err(Error::NothingToJoin)
    ));
    sandbox
        .submit("import time, os; time.sleep(0.01); os._exit(7)")
        .unwrap();
    assert_eq!(sandbox.join_async().await.unwrap(), 7);
    assert_eq!(sandbox.join_async().await.unwrap(), 7);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn async_registration_rejects_kernel_without_negotiation() {
    let kernel = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/native-kernel/helloworld-native_hyperlight-x86_64");
    let result = SandboxBuilder::from_kernel(kernel)
        .scratch_mb(64)
        .echo_output(false)
        .async_host_function("unused", |_| async { Ok("null".into()) })
        .boot();
    assert!(matches!(result, Err(Error::AsyncHostCallsUnsupported)));
}
