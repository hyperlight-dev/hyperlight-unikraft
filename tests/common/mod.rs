// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Shared helpers for integration tests.
//!
//! Each test file imports from `hyperlight_unikraft` directly;
//! this module provides only test-specific helpers.
//!
//! Items are `#[allow(dead_code)]` because each test binary re-compiles
//! this module and uses only a subset of its helpers.

use std::path::PathBuf;

use tempfile::TempDir;

use hyperlight_unikraft::{Error, Exec, ListenPorts, NetworkPolicy, SandboxBuilder};

/// Run `f`, and return what the guest kernel logged meanwhile, on this
/// thread: each Hyperlight guest log message (target `hyperlight_guest`)
/// as `[<library>] <message> (<file>:<line>)`.
#[allow(dead_code)]
pub async fn guest_log<R>(f: impl std::future::Future<Output = R>) -> (R, Vec<String>) {
    use std::sync::{Arc, Mutex};
    use tracing::instrument::WithSubscriber;
    use tracing_subscriber::layer::{Context, SubscriberExt};

    #[derive(Default)]
    struct Fields {
        message: String,
        module: String,
        file: String,
        line: u64,
    }

    impl tracing::field::Visit for Fields {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            match field.name() {
                "guest_module" => self.module = value.to_string(),
                "guest_file" => self.file = value.to_string(),
                _ => {}
            }
        }

        fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
            if field.name() == "guest_line" {
                self.line = value;
            }
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            if field.name() == "message" {
                self.message = format!("{value:?}");
            }
        }
    }

    struct Keep(Arc<Mutex<Vec<String>>>);

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Keep {
        fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
            if event.metadata().target() != "hyperlight_guest" {
                return;
            }
            let mut f = Fields::default();
            event.record(&mut f);
            self.0.lock().unwrap().push(format!(
                "[{}] {} ({}:{})",
                f.module, f.message, f.file, f.line
            ));
        }
    }

    let lines = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::registry().with(Keep(lines.clone()));
    let result = f.with_subscriber(subscriber).await;
    let lines = std::mem::take(&mut *lines.lock().unwrap());
    (result, lines)
}

// Python's high-level helper only accepts its driver thread. Exercise
// independent guest waiters through the same driver ioctl ABI.
#[allow(dead_code)]
pub const RAW_HOST_CALL: &str = r#"
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

/// Calling-thread CPU time, quantized to Linux USER_HZ (10 ms) ticks.
#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn thread_cpu_time() -> std::time::Duration {
    let stat = std::fs::read_to_string("/proc/thread-self/stat").expect("procfs");
    let rest = &stat[stat.rfind(')').expect("stat format") + 2..];
    let fields: Vec<&str> = rest.split_whitespace().collect();
    // utime/stime are fields 14/15; the suffix starts at field 3.
    let ticks = fields[11].parse::<u64>().unwrap() + fields[12].parse::<u64>().unwrap();
    std::time::Duration::from_millis(ticks * 10)
}

#[allow(dead_code)]
pub fn rootfs(runtime: &str) -> Option<PathBuf> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("build-elfloader/{runtime}-rootfs.cpio"));
    if path.exists() { Some(path) } else { None }
}

#[allow(dead_code)]
pub fn require_rootfs(runtime: &str) -> PathBuf {
    match rootfs(runtime) {
        Some(p) => p,
        None => {
            panic!(
                "{}-rootfs.cpio not found — run `just build-rootfs {}`",
                runtime, runtime,
            );
        }
    }
}

/// A fresh directory under the system temp dir, named after the test.  It
/// is removed when dropped, so a failed test leaves nothing behind either.
#[allow(dead_code)]
pub fn temp_dir(label: &str) -> TempDir {
    tempfile::Builder::new()
        .prefix(&format!("hluk-test-{label}-"))
        .tempdir()
        .unwrap()
}

/// Run hluk as a subprocess with piped stdin.
#[allow(dead_code)]
pub fn hluk_with_stdin(
    rootfs: &std::path::Path,
    script: &std::path::Path,
    stdin_data: &[u8],
) -> String {
    hluk_with_stdin_scratch(rootfs, script, stdin_data, 256)
}

#[allow(dead_code)]
pub fn hluk_with_stdin_scratch(
    rootfs: &std::path::Path,
    script: &std::path::Path,
    stdin_data: &[u8],
    scratch_mb: u32,
) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let bin = env!("CARGO_BIN_EXE_hluk");
    let mut child = Command::new(bin)
        .args([
            "run",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--scratch-mb",
            &scratch_mb.to_string(),
        ])
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn hluk");

    // Write data then close stdin (sends EOF to the guest).
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(stdin_data).ok();
    }
    child.stdin.take(); // close → EOF

    let output = child.wait_with_output().expect("hluk didn't finish");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Guest mount point for compiled binaries.
#[allow(dead_code)]
pub const BIN_MOUNT: &str = "/mnt/bin";

/// Prebuilt guest binaries for a compiled runtime.
///
/// Built from `examples/` by `just build-test-bins` on Linux, the same
/// way rootfs images are, so tests need no host toolchain.
#[allow(dead_code)]
pub fn require_bins(runtime: &str) -> PathBuf {
    let dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("build-elfloader/bins/{runtime}"));
    assert!(
        dir.is_dir(),
        "build-elfloader/bins/{runtime} not found — run `just build-test-bins` (on Linux) and copy build-elfloader/bins/ here",
    );
    dir
}

/// [`require_bins`], and the one binary the test is after must be in it:
/// a `bins/` built before that example was added is otherwise reported as
/// a boot failure.
#[allow(dead_code)]
pub fn require_bin(runtime: &str, name: &str) -> PathBuf {
    let dir = require_bins(runtime);
    assert!(
        dir.join(name).is_file(),
        "build-elfloader/bins/{runtime}/{name} not found — rebuild with `just build-test-bins`",
    );
    dir
}

/// The host's non-loopback IP.
#[allow(dead_code)]
pub fn host_ip() -> String {
    let sock = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    sock.connect("8.8.8.8:80").unwrap();
    sock.local_addr().unwrap().ip().to_string()
}

/// A high port that nothing listens on.
#[allow(dead_code)]
pub const UNUSED_PORT: u16 = 19999;

/// Helper: run the policy probe inside the guest and return the output.
#[allow(dead_code)]
pub async fn net_probe(
    policy: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    host: &str,
    port: u16,
) -> String {
    let rootfs = require_rootfs("python");
    let mut builder = SandboxBuilder::from_initrd(rootfs).scratch_mb(256);
    if let Some(policy) = policy {
        builder = builder.network(policy);
    }
    if let Some(listen_ports) = listen_ports {
        builder = builder.listen_ports(listen_ports);
    }
    let mut sandbox = builder.boot().unwrap();
    let code = format!(
        "HOST = {host:?}; PORT = {port}\n{}",
        include_str!("../../examples/python/net_policy_probe.py"),
    );
    let _ = sandbox.run(&*code).await;
    sandbox.drain_output()
}

/// Runs an HTTP GET example, which fetches an outside service, and returns
/// what it printed, for the caller to check the status in.  A 5xx is the
/// service's failure, not the guest's (httpbin.org answers 502 now and
/// then), so the example runs again, `ATTEMPTS` times in all, `PAUSES`
/// apart, each time in a fresh sandbox from `builder` with the network
/// policy set to `AllowAll` (a reused one keeps the failed attempt's
/// sockets); after the last, its output is returned like any other.
/// Anything else ends it, as the guest's: a run that fails panics here
/// (Python raises on a 4xx, and all but Node on a DNS or connection error),
/// and what the examples print instead (Node both, the others another
/// status) is what the caller's check fails on.  Every attempt is in the
/// output, and a retry is a warning naming the example, so a guest that
/// makes the service fail does not pass quietly.
#[allow(dead_code)]
pub async fn run_http_get(builder: impl Fn() -> SandboxBuilder, exec: Exec) -> String {
    use std::fmt::Write as _;
    use std::io::Write as _;

    const PAUSES: [u64; 2] = [2, 5];
    const ATTEMPTS: usize = PAUSES.len() + 1;
    let mut output = String::new();
    for attempt in 1..=ATTEMPTS {
        let mut sandbox = match builder().network(NetworkPolicy::AllowAll).boot() {
            Ok(sandbox) => sandbox,
            Err(e) => panic!("the HTTP GET sandbox did not boot: {e:?}\n{output}"),
        };
        let result = sandbox.run(exec.clone()).await;
        let out = sandbox.drain_output();
        drop(sandbox);
        let _ = write!(output, "--- attempt {attempt} ---\n{out}");
        if !output.ends_with('\n') {
            output.push('\n');
        }
        // The examples print a 5xx and return; Python's urllib raises on
        // one, which fails the call.  A run that fails after printing one
        // failed some other way.
        let retryable = match &result {
            Ok(()) => printed_5xx(&out),
            Err(Error::CallFailed { .. }) => urllib_raised_5xx(&out),
            Err(_) => false,
        };
        if let Err(e) = &result {
            let _ = writeln!(output, "[run failed: {e:?}]");
        }
        if !retryable {
            if let Err(e) = result {
                panic!("the HTTP GET example failed: {e:?}\n{output}");
            }
            break;
        }
        if attempt == ATTEMPTS {
            break;
        }
        let pause = PAUSES[attempt - 1];
        let example = match &exec {
            Exec::File(path) => path.file_name().map_or_else(
                || path.display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            ),
            _ => "the example".to_string(),
        };
        // Straight to stderr, past the test harness's capture, so that it
        // shows on a test that then passes; an annotation on GitHub.
        let prefix = if std::env::var_os("GITHUB_ACTIONS").is_some() {
            "::warning::"
        } else {
            "warning: "
        };
        let _ = writeln!(
            std::io::stderr(),
            "{prefix}{example}: HTTP GET attempt {attempt} of {ATTEMPTS} got a server error, trying again in {pause} s",
        );
        tokio::time::sleep(std::time::Duration::from_secs(pause)).await;
    }
    output
}

fn is_5xx(code: &str) -> bool {
    code.len() == 3 && code.starts_with('5') && code.bytes().all(|b| b.is_ascii_digit())
}

/// Whether an HTTP GET example printed a 5xx: a line `Status: 5NN`
fn printed_5xx(output: &str) -> bool {
    output
        .lines()
        .any(|line| line.trim_end().strip_prefix("Status: ").is_some_and(is_5xx))
}

/// Whether Python's urllib raised for a 5xx:
/// `urllib.error.HTTPError: HTTP Error 5NN: <reason>`
fn urllib_raised_5xx(output: &str) -> bool {
    output.lines().any(|line| {
        line.strip_prefix("urllib.error.HTTPError: HTTP Error ")
            .and_then(|rest| rest.split_once(':'))
            .is_some_and(|(code, _)| is_5xx(code))
    })
}
