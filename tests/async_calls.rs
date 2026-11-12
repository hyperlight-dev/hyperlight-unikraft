// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use hyperlight_unikraft::{Error, NetworkPolicy, SandboxBuilder};

mod common;
use common::RAW_HOST_CALL;

fn python() -> SandboxBuilder {
    SandboxBuilder::from_initrd(common::require_rootfs("python"))
        .scratch_mb(256)
        .echo_output(false)
}

#[test]
fn bounded_sync_step_retains_pending_async_work() {
    let mut sandbox = python()
        .async_host_function("later", |_| async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok("7".into())
        })
        .boot()
        .unwrap();
    sandbox
        .submit("import hyperlight; assert hyperlight.call('later') == 7")
        .unwrap();
    assert!(matches!(
        sandbox.step(Duration::from_millis(1)).unwrap(),
        hyperlight_unikraft::Yield::Blocked { .. }
    ));
    sandbox.snapshot().unwrap();
    assert_eq!(
        sandbox.step(Duration::from_secs(2)).unwrap(),
        hyperlight_unikraft::Yield::CallDone
    );
    sandbox.run("print('still usable')").unwrap();
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
        .run_async("import hyperlight; assert hyperlight.call('sync') == 42")
        .await
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
async fn blocklist_connect_and_sendto_share_completions_with_embedder_calls() {
    let mut sandbox = python()
        .network(NetworkPolicy::BlockList(
            hyperlight_unikraft::BlockList::from_hosts(&["localhost"]).unwrap(),
        ))
        .profile(true)
        .async_host_function("pause", |_| async {
            tokio::time::sleep(Duration::from_millis(20)).await;
            Ok("7".into())
        })
        .boot()
        .unwrap();
    let script = r#"
import socket, threading, hyperlight, errno
errors = []
def datagram():
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
            try:
                assert s.sendto(b"probe", ("192.0.2.1", 9)) == 5
            except OSError as e:
                assert e.errno in (errno.ENETUNREACH, errno.EHOSTUNREACH, errno.ECONNREFUSED), e
    except BaseException as e:
        errors.append(e)
t = threading.Thread(target=datagram)
t.start()
assert hyperlight.call("pause") == 7
with socket.socket() as s:
    s.settimeout(0.05)
    try:
        s.connect(("192.0.2.1", 9))
    except OSError as e:
        assert e.errno not in (errno.EACCES, errno.EIO, errno.EOPNOTSUPP), e
t.join()
assert not errors, errors
"#;
    tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(script))
        .await
        .unwrap()
        .unwrap();
    let report = sandbox.profile().report();
    for label in [
        "host net_connect_async ",
        "host net_sendto_async ",
        "host net_wait_async ",
        "entry step_async ",
    ] {
        assert!(report.contains(label), "{report}");
    }
    sandbox.snapshot().unwrap();
    tokio::time::timeout(Duration::from_secs(1), sandbox.step_async())
        .await
        .unwrap()
        .unwrap();
    sandbox.snapshot().unwrap();
    // A subsequent call can perform another deferred policy check.
    sandbox.run_async("import socket\ns = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)\ns.connect(('192.0.2.1', 9))\ns.close()").await.unwrap();
}

#[test]
fn synchronous_setup_and_restore_queue_network_waits_for_async_driving() {
    use hyperlight_unikraft::Yield;
    use std::io::{Read, Write};

    let reservation = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = reservation.local_addr().unwrap().port();
    drop(reservation);
    let mut sandbox = python()
        .network(NetworkPolicy::AllowAll)
        .listen_ports(hyperlight_unikraft::ListenPorts::from_ports([port]))
        .boot()
        .unwrap();
    sandbox
        .submit(format!(
            "import socket\ns = socket.socket(); s.bind(('127.0.0.1', {port})); s.listen(1)\n\
         print(s.getsockname()[1], flush=True)\n\
         c, _ = s.accept()\nassert c.recv(4) == b'ping'\nc.sendall(b'pong')\n\
         c.close(); s.close()"
        ))
        .unwrap();
    assert_eq!(sandbox.drain_output().trim().parse::<u16>().unwrap(), port);
    sandbox.snapshot().unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    drop(sandbox);
    let mut sandbox = SandboxBuilder::from_snapshot(snapshot)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(hyperlight_unikraft::ListenPorts::from_ports([port]))
        .echo_output(false)
        .boot()
        .unwrap();
    sandbox.snapshot().unwrap();
    let mut peer = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    peer.write_all(b"ping").unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match sandbox.step_async().await.unwrap() {
                    Yield::CallDone => break,
                    Yield::Blocked { .. } => {}
                    other => panic!("unexpected yield: {other:?}"),
                }
            }
        })
        .await
        .unwrap();
    });
    let mut reply = [0; 4];
    peer.read_exact(&mut reply).unwrap();
    assert_eq!(&reply, b"pong");
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
    sandbox.snapshot().unwrap();
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
    poisoned(sandbox.snapshot_with_scratch_mb(512));
    poisoned(sandbox.step_async().await);
    poisoned(sandbox.submit("print('no')"));
    poisoned(sandbox.join_async().await);
    sandbox.restore(snapshot).unwrap();
    sandbox.run_async("print('recovered')").await.unwrap();
    sandbox.snapshot().unwrap();
}

#[tokio::test]
async fn backpressured_send_leaves_executor_and_host_calls_runnable() {
    use std::io::Read;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port();
    let (release, wait) = std::sync::mpsc::channel();
    let mut sandbox = python()
        .network(NetworkPolicy::AllowAll)
        .async_host_function("release_reader", move |_| {
            let release = release.clone();
            async move {
                tokio::time::sleep(Duration::from_millis(40)).await;
                release.send(()).map_err(|e| e.to_string())?;
                Ok("null".into())
            }
        })
        .boot()
        .unwrap();
    sandbox
        .submit(format!(
            r#"
import socket, threading, hyperlight
s = socket.create_connection(("127.0.0.1", {port}))
s.setsockopt(socket.SOL_SOCKET, socket.SO_SNDBUF, 4096)
errors = []
def send():
    try:
        s.sendall(b"x" * (4 * 1024 * 1024))
    except BaseException as e:
        errors.append(e)
    finally:
        s.close()
t = threading.Thread(target=send)
t.start()
hyperlight.call("release_reader")
t.join()
assert not errors, errors
"#
        ))
        .unwrap();
    let (mut peer, _) = listener.accept().unwrap();
    let reader = std::thread::spawn(move || {
        // If a send blocks the vCPU/executor, release_reader cannot complete.
        // The deadline drops the peer so even that regression fails boundedly.
        wait.recv_timeout(Duration::from_secs(2)).unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut data = Vec::new();
        peer.read_to_end(&mut data).unwrap();
        data.len()
    });
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match sandbox.step_async().await.unwrap() {
                hyperlight_unikraft::Yield::CallDone => break,
                hyperlight_unikraft::Yield::Blocked { .. } => {}
                other => panic!("unexpected yield: {other:?}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(reader.join().unwrap(), 4 * 1024 * 1024);
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
async fn maximum_results_share_batches_across_guest_threads() {
    let mut sandbox = python()
        .profile(true)
        .async_host_function("large", |_| async { Ok("7".repeat(69_619)) })
        .boot()
        .unwrap();
    sandbox.profile().reset();
    let code = format!(
        r#"{RAW_HOST_CALL}
results, errors = [], []
def work():
    try:
        result = raw_call("large", capacity=69620)
        assert result == b"7" * 69619, len(result)
        results.append(len(result))
    except BaseException as e:
        errors.append(str(e))
threads = [threading.Thread(target=work) for _ in range(32)]
for thread in threads:
    thread.start()
for thread in threads:
    thread.join()
assert not errors, errors
assert len(results) == 32, len(results)
os.close(fd)
"#
    );
    tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(code))
        .await
        .unwrap()
        .unwrap();
    let report = sandbox.profile().report();
    let entries: usize = report
        .lines()
        .find_map(|line| line.strip_prefix("entry step_async "))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(entries, 3, "{report}");
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
async fn many_guest_waiters_complete_once_across_multiple_batches() {
    let mut sandbox = python()
        .profile(true)
        .async_host_function("echo", |args| async move { Ok(args) })
        .boot()
        .unwrap();
    let code = format!(
        r#"{RAW_HOST_CALL}
import json
results = []
def worker(i):
    results.append(json.loads(raw_call("echo", f"[{{i}}]".encode()))[0])
threads = [threading.Thread(target=worker, args=(i,)) for i in range(130)]
for t in threads: t.start()
for t in threads: t.join()
assert sorted(results) == list(range(130)), results
os.close(fd)
"#
    );
    for _ in 0..3 {
        sandbox.profile().reset();
        tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(code.as_str()))
            .await
            .unwrap()
            .unwrap();
        let report = sandbox.profile().report();
        let batches = report
            .lines()
            .find_map(|line| line.strip_prefix("entry step_async "))
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(batches, 3, "{report}");
        sandbox.snapshot().unwrap();
    }
}

#[tokio::test]
async fn repeated_fanout_cancellation_and_restore_drops_every_future() {
    let senders = Arc::new(Mutex::new(Vec::new()));
    let pending = senders.clone();
    let mut sandbox = python()
        .async_host_function("wait", move |_| {
            let (send, recv) = futures::channel::oneshot::channel::<()>();
            pending.lock().unwrap().push(send);
            async move {
                recv.await.map_err(|e| e.to_string())?;
                Ok("7".into())
            }
        })
        .boot()
        .unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    let code = format!(
        r#"{RAW_HOST_CALL}
threads = [threading.Thread(target=lambda: raw_call("wait")) for _ in range(16)]
for t in threads: t.start()
for t in threads: t.join()
"#
    );
    for _ in 0..8 {
        assert!(
            tokio::time::timeout(Duration::from_millis(20), sandbox.run_async(code.as_str()))
                .await
                .is_err()
        );
        let cancelled = std::mem::take(&mut *senders.lock().unwrap());
        assert_eq!(cancelled.len(), 16);
        assert!(cancelled.iter().all(|send| send.is_canceled()));
        assert!(matches!(
            sandbox.snapshot(),
            Err(Error::Hyperlight(
                hyperlight_unikraft::hyperlight_host::HyperlightError::PoisonedSandbox
            ))
        ));
        sandbox.restore(snapshot.clone()).unwrap();
        sandbox.run_async("pass").await.unwrap();
        sandbox.snapshot().unwrap();
    }
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
    sandbox.run_async("print('sync recovery')").await.unwrap();
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
        .run_async("import hyperlight\nfor _ in range(10): hyperlight.call('sync')")
        .await
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
async fn async_tcp_server_accepts_repeatedly_and_restore_releases_cancelled_listener() {
    use std::io::{Read, Write};

    let (send, mut recv) = std::sync::mpsc::channel::<u16>();
    let mut sandbox = python()
        .network(NetworkPolicy::AllowAll)
        .host_function("listening", move |args| {
            let [port]: [u16; 1] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            send.send(port).map_err(|e| e.to_string())?;
            Ok("null".into())
        })
        .boot()
        .unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    let server = r#"
import hyperlight, socket
listener = socket.socket()
listener.bind(("127.0.0.1", 0))
listener.listen(1)
hyperlight.call("listening", listener.getsockname()[1])
conn, _ = listener.accept()
assert conn.recv(4) == b"ping"
conn.sendall(b"pong")
conn.close()
listener.close()
"#;
    for _ in 0..2 {
        let client = std::thread::spawn(move || {
            let port = recv.recv_timeout(Duration::from_secs(5)).unwrap();
            std::thread::sleep(Duration::from_millis(20));
            let mut socket = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            std::thread::sleep(Duration::from_millis(20));
            socket.write_all(b"ping").unwrap();
            let mut response = [0; 4];
            socket.read_exact(&mut response).unwrap();
            assert_eq!(&response, b"pong");
            recv
        });
        tokio::time::timeout(Duration::from_secs(5), sandbox.run_async(server))
            .await
            .unwrap()
            .unwrap();
        recv = client.join().unwrap();
        sandbox.restore(snapshot.clone()).unwrap();
    }

    assert!(
        tokio::time::timeout(Duration::from_millis(100), sandbox.run_async(server))
            .await
            .is_err()
    );
    let port = recv.recv_timeout(Duration::from_secs(1)).unwrap();
    assert!(matches!(
        sandbox.step_async().await,
        Err(Error::Hyperlight(
            hyperlight_unikraft::hyperlight_host::HyperlightError::PoisonedSandbox
        ))
    ));
    sandbox.restore(snapshot).unwrap();
    let _rebound = std::net::TcpListener::bind(("127.0.0.1", port)).unwrap();
    sandbox.run_async("print('recovered')").await.unwrap();
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

#[tokio::test]
async fn scheduler_timers_keep_stable_entry_and_exit_counts() {
    let mut sandbox = python().profile(true).boot().unwrap();
    let script = "import time\nfor _ in range(3): time.sleep(0.02)";
    // Populate the guest's environment cache before comparing VM exits.
    sandbox.run_async("import time").await.unwrap();
    sandbox.profile().reset();
    sandbox.run_async(script).await.unwrap();
    let first = sandbox.profile().report();
    sandbox.profile().reset();
    sandbox.run_async(script).await.unwrap();
    let asynchronous = sandbox.profile().report();
    let counts = |report: &str| {
        ["entry Exec ", "entry step ", "host Yield "].map(|prefix| {
            let row = report
                .lines()
                .find_map(|line| line.strip_prefix(prefix))
                .unwrap_or_else(|| panic!("missing {prefix}: {report}"));
            let fields: Vec<_> = row.split_whitespace().collect();
            let count = fields[0].parse::<u64>().unwrap();
            let exits = if prefix.starts_with("entry ") {
                fields[4].parse::<f64>().unwrap()
            } else {
                0.0
            };
            (count, exits)
        })
    };
    assert_eq!(
        counts(&first),
        counts(&asynchronous),
        "{first}\n{asynchronous}"
    );
    assert_eq!(counts(&asynchronous)[1].0, 3);
    assert!(!asynchronous.contains("entry step_async"), "{asynchronous}");
}

#[tokio::test]
async fn timer_snapshots_restore_between_async_steps() {
    use hyperlight_unikraft::Yield;

    let mut sandbox = python().boot().unwrap();
    let idle = sandbox.snapshot().unwrap();
    sandbox
        .submit("import time; time.sleep(0.03); print('first'); time.sleep(0.1); print('second')")
        .unwrap();
    assert!(matches!(
        sandbox.step_async().await.unwrap(),
        Yield::Blocked { .. }
    ));
    assert!(sandbox.drain_output().contains("first"));
    // A scheduler timer alone must not trigger the async host-call snapshot restriction.
    let sleeping = sandbox.snapshot().unwrap();
    assert_eq!(sandbox.step_async().await.unwrap(), Yield::CallDone);
    assert!(sandbox.drain_output().contains("second"));
    sandbox.restore(sleeping).unwrap();
    assert_eq!(sandbox.step_async().await.unwrap(), Yield::CallDone);
    assert!(sandbox.drain_output().contains("second"));
    sandbox.restore(idle).unwrap();
    assert_eq!(
        sandbox.step_async().await.unwrap(),
        Yield::Blocked { until: None }
    );
    sandbox.run_async("print('new timeline')").await.unwrap();
}
