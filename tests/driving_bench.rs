// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Run explicitly with `cargo test --release --test driving_bench -- --ignored --nocapture`.
//! Async driving baseline; boot and warm-up are excluded.

mod common;

use std::io::{Read, Write};
use std::net::{TcpListener, UdpSocket};
use std::time::{Duration, Instant};

use hyperlight_unikraft::{AppSandbox, NetworkPolicy, SandboxBuilder};

const WARMUP: usize = 5;
const NETWORK_SAMPLES: usize = 200;

async fn measure(
    sandbox: &mut AppSandbox,
    label: &str,
    code: &str,
    samples: usize,
    entries_per_call: Option<u64>,
) {
    for _ in 0..WARMUP {
        sandbox.run(code).await.unwrap();
    }
    sandbox.profile().reset();
    let mut latency = Vec::with_capacity(samples);
    #[cfg(target_os = "linux")]
    let cpu = common::thread_cpu_time();
    let start = Instant::now();
    for _ in 0..samples {
        let call = Instant::now();
        sandbox.run(code).await.unwrap();
        latency.push(call.elapsed());
    }
    let wall = start.elapsed();
    #[cfg(target_os = "linux")]
    let cpu_ms = {
        let used = common::thread_cpu_time() - cpu;
        if label == "idle" {
            assert!(used < Duration::from_millis(100), "idle CPU: {used:?}");
        }
        format!("{:.1}", used.as_secs_f64() * 1000.0)
    };
    #[cfg(not(target_os = "linux"))]
    let cpu_ms = "unavailable";
    let report = sandbox.profile().report();
    let entries: u64 = report
        .lines()
        .filter(|line| line.starts_with("entry "))
        .map(|line| {
            line.split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .sum();
    if let Some(expected) = entries_per_call {
        assert_eq!(entries, expected * samples as u64, "{label}: {report}");
    }
    if label.ends_with("-wait") {
        assert!(
            entries > samples as u64,
            "{label} did not exercise a readiness wait: {report}"
        );
    }
    latency.sort_unstable();
    let median = latency[samples / 2].as_secs_f64() * 1e6;
    let p95 = latency[(samples * 95).div_ceil(100) - 1].as_secs_f64() * 1e6;
    println!(
        "BENCH drive mode=Async workload={label} samples={samples} median_us={median:.1} \
         p95_us={p95:.1} calls_per_sec={:.1} cpu_ms={cpu_ms} entries_per_call={:.2}",
        samples as f64 / wall.as_secs_f64(),
        entries as f64 / samples as f64,
    );
}

#[tokio::test(flavor = "current_thread")]
#[ignore = "release performance baseline; requires a hypervisor and Python rootfs"]
async fn async_driving_baseline() {
    println!(
        "BENCH platform os={} arch={}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    for _ in 0..2 {
        let builder = SandboxBuilder::from_initrd(common::require_rootfs("python"))
            .scratch_mb(256)
            .network(NetworkPolicy::AllowAll)
            .echo_output(false)
            .profile(true)
            .host_function("sync", |_| Ok("7".into()))
            .async_host_function("later", |_| async {
                tokio::time::sleep(Duration::from_millis(5)).await;
                Ok("7".into())
            });
        let mut sandbox = builder.boot().unwrap();
        sandbox.run(&*format!(
            r#"{}
import time, hyperlight, socket
def burst():
    results = []
    threads = [threading.Thread(target=lambda: results.append(raw_call('later'))) for _ in range(16)]
    for t in threads: t.start()
    for t in threads: t.join()
    assert results == [b'7'] * 16, results
"#,
            common::RAW_HOST_CALL,
        )).await.unwrap();
        for (label, code, samples, entries) in [
            ("immediate", "pass", 5000, Some(1)),
            (
                "sync-host",
                "assert hyperlight.call('sync') == 7",
                5000,
                Some(1),
            ),
            ("timer", "time.sleep(0.005)", 100, Some(2)),
            ("fanout-16", "burst()", 20, None),
            ("idle", "time.sleep(0.5)", 1, Some(2)),
        ] {
            measure(&mut sandbox, label, code, samples, entries).await;
        }

        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let peer = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_nodelay(true).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            for _ in 0..2 * (WARMUP + NETWORK_SAMPLES) {
                let mut data = [0; 4];
                stream.read_exact(&mut data).unwrap();
                if &data == b"wait" {
                    std::thread::sleep(Duration::from_millis(2));
                }
                stream.write_all(&data).unwrap();
            }
        });
        sandbox.run(&*format!(
            "s = socket.socket(); s.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1); s.connect(('127.0.0.1', {port})); stream = s.makefile('rb')"
        )).await.unwrap();
        measure(
            &mut sandbox,
            "tcp",
            "s.sendall(b'ping'); assert stream.read(4) == b'ping'",
            NETWORK_SAMPLES,
            None,
        )
        .await;
        measure(
            &mut sandbox,
            "tcp-wait",
            "s.sendall(b'wait'); assert stream.read(4) == b'wait'",
            NETWORK_SAMPLES,
            None,
        )
        .await;
        peer.join().unwrap();
        sandbox.run("stream.close(); s.close()").await.unwrap();

        let socket = UdpSocket::bind(("127.0.0.1", 0)).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let port = socket.local_addr().unwrap().port();
        let peer = std::thread::spawn(move || {
            for _ in 0..2 * (WARMUP + NETWORK_SAMPLES) {
                let mut data = [0; 4];
                let (n, addr) = socket.recv_from(&mut data).unwrap();
                assert_eq!(n, 4);
                if &data == b"wait" {
                    std::thread::sleep(Duration::from_millis(2));
                }
                assert_eq!(socket.send_to(&data, addr).unwrap(), 4);
            }
        });
        sandbox.run(&*format!(
            "s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM); s.connect(('127.0.0.1', {port}))"
        )).await.unwrap();
        measure(
            &mut sandbox,
            "udp",
            "assert s.send(b'ping') == 4; assert s.recv(4) == b'ping'",
            NETWORK_SAMPLES,
            None,
        )
        .await;
        measure(
            &mut sandbox,
            "udp-wait",
            "assert s.send(b'wait') == 4; assert s.recv(4) == b'wait'",
            NETWORK_SAMPLES,
            None,
        )
        .await;
        peer.join().unwrap();
        sandbox.run("s.close(); os.close(fd)").await.unwrap();
    }
}
