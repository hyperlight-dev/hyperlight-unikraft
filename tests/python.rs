// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;

use common::{hluk_with_stdin, require_rootfs, run_http_get, temp_dir};
use hyperlight_unikraft::{Error, Exec, ListenPorts, Mount, NetworkPolicy, SandboxBuilder};

#[tokio::test]
async fn python_inline_code() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("print('hluk-test-ok')").await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("hluk-test-ok"),
        "expected guest to print 'hluk-test-ok', got: {output:?}",
    );
}

/// The image has its users and groups, so the user, its home and `~`
/// resolve as on Linux (getpass.getuser() used to raise KeyError).
#[tokio::test]
async fn python_knows_its_user() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("import getpass, os, pwd; print(getpass.getuser(), pwd.getpwuid(os.getuid()).pw_dir, os.path.expanduser('~'))").await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("root /root /root"), "got: {output:?}");
}

/// link() that fails, across file systems or of a directory, fails with
/// Linux's errno, and the guest goes on (the kernel used to crash on
/// those error paths).
#[tokio::test]
async fn python_failed_links_return_their_errno() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno, os\n",
            "open('/tmp/x', 'w').close()\n",
            "os.mkdir('/tmp/d')\n",
            "for src, dst in (('/tmp/x', '/dev/y'), ('/tmp/d', '/tmp/d2'), ('/tmp/x', '/tmp/no/y')):\n",
            "    try:\n",
            "        os.link(src, dst)\n",
            "    except OSError as e:\n",
            "        print(errno.errorcode[e.errno])\n",
            "print('alive')\n",
        )).await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("EXDEV\nEPERM\nENOENT\nalive"),
        "got: {output:?}"
    );
}

/// A line longer than one host call carries reaches the host whole, in
/// several HostWrite calls the kernel cuts between characters (it used
/// to drop all but the first 4 KiB).  How the host joins a character
/// split between writes is covered by `python_non_utf8_output_...` and
/// the `GuestOutput` unit test.
#[tokio::test]
async fn python_long_line_is_printed_whole() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("import sys; sys.stdout.write('x' + 'é' * 100000 + 'end\\n'); sys.stdout.flush()")
        .await
        .unwrap();
    // The leading 'x' puts the 69,620-byte marks inside an 'é', so the
    // kernel has to back the cuts off to where a character starts.
    let output = sandbox.drain_output();
    let line = output
        .lines()
        .find(|l| l.ends_with("end"))
        .unwrap_or_default();
    assert_eq!(line, format!("x{}end", "é".repeat(100000)));
}

/// Output that is not UTF-8, or that ends a write inside a character,
/// is still output: the call goes on, and the text reads as it would on
/// a terminal (it used to fail the whole call).
#[tokio::test]
async fn python_non_utf8_output_does_not_fail_the_call() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            "import sys\n\
             out = sys.stdout.buffer\n\
             for chunk in (b'ol\\xc3', b'\\xa9 \\xff', b'!\\n'):\n\
             \x20   out.write(chunk); out.flush()",
        )
        .await
        .unwrap();
    assert!(sandbox.drain_output().contains("olé \u{fffd}!"));
}

#[tokio::test]
async fn python_exec_file() {
    let rootfs = require_rootfs("python");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/hello.py");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.py to produce output containing 'Hello', got: {output:?}",
    );
}

#[tokio::test]
async fn python_snapshot_round_trip() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-snap");

    // Save
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    // Restore + run
    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.run("print('restored-ok')").await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("restored-ok"),
        "expected restored guest to print 'restored-ok', got: {output:?}",
    );
}

#[tokio::test]
async fn python_multiple_runs() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("x = 1 + 1").await.unwrap();
    sandbox.run("print(f'x={x}')").await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("x=2"),
        "expected 'x=2' after multiple runs, got: {output:?}",
    );
    sandbox.run("import sys; print(sys.version)").await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("3.12"),
        "expected sys.version to contain '3.12', got: {output:?}",
    );
}

#[tokio::test]
async fn python_env_vars() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox
        .run(Exec::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/env_vars.py"),
        ))
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("MY_VAR=hello_world"),
        "expected MY_VAR=hello_world, got: {output:?}"
    );
    assert!(
        output.contains("DEBUG=1"),
        "expected DEBUG=1, got: {output:?}"
    );
    assert!(
        output.contains("GREETING=hi there"),
        "expected GREETING=hi there, got: {output:?}"
    );
}

#[tokio::test]
async fn python_env_vars_inline() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[("SECRET", "42")]);
    sandbox
        .run("import os; print(os.environ['SECRET'])")
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("42"),
        "expected '42' from SECRET env var, got: {output:?}",
    );
}

#[tokio::test]
async fn python_env_vars_snapshot_restore() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-env-snap");

    // Save snapshot (no env vars set at save time)
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    // Restore + set env vars AFTER restore
    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[("RESTORED_VAR", "from_snapshot")]);
    sandbox
        .run(
            r#"
import os
v = os.environ.get('RESTORED_VAR', 'NOT_FOUND')
print(f'RESTORED_VAR={v}')
"#,
        )
        .await
        .unwrap();
    let output = sandbox.drain_output();
    eprintln!("snapshot env output: {output:?}");
    assert!(
        output.contains("RESTORED_VAR=from_snapshot"),
        "expected env var set after snapshot restore, got: {output:?}",
    );
}

/// Env vars are stateful across dispatches without restore.
#[tokio::test]
async fn python_env_vars_stateful_across_dispatches() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    // 1. Run with no env vars set — STATEFUL_VAR should not exist.
    sandbox
        .run(
            r#"
import os
v = os.environ.get('STATEFUL_VAR', 'NOT_FOUND')
print(f'step1: STATEFUL_VAR={v}')
"#,
        )
        .await
        .unwrap();
    let out1 = sandbox.drain_output();
    assert!(
        out1.contains("step1: STATEFUL_VAR=NOT_FOUND"),
        "expected no STATEFUL_VAR before setting, got: {out1:?}",
    );

    // 2. Set env vars, then run — should see them.
    sandbox.set_env_vars(&[("STATEFUL_VAR", "persisted")]);
    sandbox
        .run(
            r#"
import os
v = os.environ.get('STATEFUL_VAR', 'NOT_FOUND')
print(f'step2: STATEFUL_VAR={v}')
"#,
        )
        .await
        .unwrap();
    let out2 = sandbox.drain_output();
    assert!(
        out2.contains("step2: STATEFUL_VAR=persisted"),
        "expected STATEFUL_VAR=persisted after setting, got: {out2:?}",
    );

    // 3. Run AGAIN without setting env vars or restoring.
    sandbox
        .run(
            r#"
import os
v = os.environ.get('STATEFUL_VAR', 'NOT_FOUND')
print(f'step3: STATEFUL_VAR={v}')
"#,
        )
        .await
        .unwrap();
    let out3 = sandbox.drain_output();
    assert!(
        out3.contains("step3: STATEFUL_VAR=persisted"),
        "expected env vars to persist across dispatches without restore, got: {out3:?}",
    );
}

/// Stdin reaches the guest as the bytes piped in: a character the host's
/// 4 KiB reads cut in two, and bytes that are not UTF-8 (they used to
/// arrive as U+FFFD).  (Ones the terminal layer acts on, a chunk ending
/// in Ctrl-D or CR, are left out: that is the terminal, not the host.)
#[test]
fn python_stdin_is_bytes() {
    let rootfs = require_rootfs("python");
    let dir = temp_dir("py-stdin-bytes");
    let script = dir.path().join("hash.py");
    std::fs::write(
        &script,
        "import sys\n\
         print('stdin', sys.stdin.buffer.read().hex())\n",
    )
    .unwrap();
    // 'x' then 'é's: the 4 KiB reads end inside one; then bytes no UTF-8
    // decoder takes.
    let mut data = b"x".to_vec();
    data.extend("é".repeat(5000).as_bytes());
    data.extend([0xff, 0x00, 0xc3]);
    let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
    // Piped in, it is not echoed; the line may still carry a prompt.
    let output = hluk_with_stdin(&rootfs, &script, &data);
    let want = format!("stdin {hex}");
    assert!(
        output
            .lines()
            .any(|l| l.trim_end_matches('\r').ends_with(&want)),
        "expected the {} bytes piped in, got: {output:?}",
        data.len()
    );
}

#[test]
fn python_stdin_piped() {
    let rootfs = require_rootfs("python");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/stdin_echo.py");
    let output = hluk_with_stdin(&rootfs, &script, b"hello from host\nline two\n");
    assert!(
        output.contains("lines=2"),
        "expected 2 lines, got: {output:?}"
    );
    assert!(
        output.contains("echo: hello from host"),
        "expected first line, got: {output:?}"
    );
    assert!(
        output.contains("echo: line two"),
        "expected second line, got: {output:?}"
    );
    assert!(
        output.contains("stdin-done"),
        "expected stdin-done marker, got: {output:?}"
    );
    // Not echoed back, as a terminal would: a pipe fed it
    assert!(
        !output.lines().any(|l| l.trim_end() == "hello from host"),
        "the piped input was echoed: {output:?}"
    );
}

#[test]
fn python_stdin_inline_piped() {
    let rootfs = require_rootfs("python");
    use std::io::Write;
    use std::process::{Command, Stdio};

    let bin = env!("CARGO_BIN_EXE_hluk");
    let mut child = Command::new(bin)
        .args([
            "run",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--exec",
            "import sys; data = sys.stdin.read(); print(f'got: {data}')",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn hluk");

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(b"secret data").ok();
    }
    child.stdin.take();

    let output = child.wait_with_output().expect("hluk didn't finish");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("got: secret data"),
        "expected stdin data, got: {stdout:?}",
    );
}

#[test]
fn python_stdin_empty_piped() {
    let rootfs = require_rootfs("python");
    use std::process::{Command, Stdio};

    let bin = env!("CARGO_BIN_EXE_hluk");
    let mut child = Command::new(bin)
        .args([
            "run",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--exec",
            "import sys; data = sys.stdin.read(); print(f'len={len(data)}')",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn hluk");

    child.stdin.take();

    let output = child.wait_with_output().expect("hluk didn't finish");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("len=0"),
        "expected empty stdin (len=0), got: {stdout:?}",
    );
}

/// `os.chmod` on a mount reaches the host file, its permission bits only.
#[cfg(unix)]
#[tokio::test]
async fn python_chmod_reaches_the_host() {
    use std::os::unix::fs::PermissionsExt;

    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-chmod");
    std::fs::write(mount_dir.path().join("f"), "f").unwrap();
    std::fs::write(mount_dir.path().join("g"), "g").unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox
        .run(
            "import os\nos.chmod('/mnt/host/f', 0o600)\nos.chmod('/mnt/host/g', 0o6755)\nprint('changed')",
        ).await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("changed"), "got: {output:?}");
    let mode = |name: &str| {
        std::fs::metadata(mount_dir.path().join(name))
            .unwrap()
            .permissions()
            .mode()
            & 0o7777
    };
    assert_eq!(mode("f"), 0o600);
    // Setuid and setgid are not the guest's to give.
    assert_eq!(mode("g"), 0o755);
}

/// A path that is not UTF-8 fails the syscall, not the guest function
/// call: the host cannot take it as a string, and the kernel refuses it
/// before sending (it used to fail the whole call).
#[tokio::test]
async fn python_non_utf8_path_fails_the_syscall_only() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-non-utf8");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox
        .run(
            "import errno, os\n\
             try:\n\
             \x20   os.stat(b'/mnt/host/\\xff')\n\
             except OSError as e:\n\
             \x20   print('refused', errno.errorcode.get(e.errno))\n\
             print('still running')",
        )
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("refused EILSEQ"), "got: {output:?}");
    assert!(output.contains("still running"), "got: {output:?}");
}

/// A path too long for the kernel's hostfs buffers fails with
/// ENAMETOOLONG; it used to be cut short, and the syscall act on another
/// host directory.  Linux only: the host directories it makes, about
/// 1,100 bytes deep in the temporary directory, are past the path limits
/// of macOS (1,024 bytes) and Windows.
#[cfg(target_os = "linux")]
#[tokio::test]
async fn python_long_host_path_is_refused() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-long-path");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox
        .run(
            "import errno, os\n\
             path = '/mnt/host'\n\
             try:\n\
             \x20   for i in range(6):\n\
             \x20       path += '/' + str(i) * 200\n\
             \x20       os.mkdir(path)\n\
             except OSError as e:\n\
             \x20   print('refused at', i, errno.errorcode.get(e.errno))",
        )
        .await
        .unwrap();
    // 200-byte names: five make 1,004 bytes below the mount (1,005 with
    // the NUL), which fits; six make 1,205, which does not.
    assert!(sandbox.drain_output().contains("refused at 5 ENAMETOOLONG"),);
    // Nothing was made under a name cut short.
    let mut deepest = mount_dir.path().to_path_buf();
    for i in 0..5 {
        deepest.push(i.to_string().repeat(200));
    }
    assert_eq!(std::fs::read_dir(&deepest).unwrap().count(), 0);
}

#[tokio::test]
async fn python_fs_ops() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-ops");

    let mounts = vec![Mount::rw(mount_dir.path(), "/mnt/host")];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/fs_ops.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Cleanup done"),
        "expected fs_ops.py to print 'Cleanup done', got: {output:?}",
    );

    let sentinel = mount_dir.path().join("sentinel.txt");
    let content = std::fs::read_to_string(&sentinel)
        .expect("sentinel.txt should exist on the host after guest write");
    assert_eq!(content, "guest-was-here\n", "sentinel content mismatch");

    let entries: Vec<_> = std::fs::read_dir(&mount_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(
        entries,
        vec!["sentinel.txt"],
        "expected only sentinel.txt, found: {:?}",
        entries
    );
}

#[tokio::test]
async fn python_fs_large_file() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-large");

    let mounts = vec![Mount::rw(mount_dir.path(), "/mnt/host")];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/fs_large.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("large file round-trip OK"),
        "expected fs_large.py to print 'large file round-trip OK', got: {output:?}",
    );

    let entries: Vec<_> = std::fs::read_dir(&mount_dir).unwrap().collect();
    assert!(
        entries.is_empty(),
        "fs_large.py should clean up, but {} entries remain",
        entries.len()
    );
}

#[tokio::test]
async fn python_guest_fs_ops() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/guest_fs.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Guest filesystem tests passed"),
        "expected guest_fs.py to print 'Guest filesystem tests passed', got: {output:?}",
    );
}

#[tokio::test]
async fn python_asyncio() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([18095]))
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/asyncio_demo.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("asyncio demo passed"),
        "expected asyncio demo to pass, got: {output:?}",
    );
}

#[tokio::test]
async fn python_threading() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/threading_demo.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Threading demo passed"),
        "expected threading_demo.py to print 'Threading demo passed', got: {output:?}",
    );
}

#[tokio::test]
async fn python_subprocess() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/subprocess_demo.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Subprocess demo passed"),
        "expected subprocess_demo.py to print 'Subprocess demo passed', got: {output:?}",
    );
}

#[tokio::test]
async fn python_tcp_echo() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/tcp_echo.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("TCP echo test passed"),
        "expected tcp_echo.py to print 'TCP echo test passed', got: {output:?}",
    );
}

#[tokio::test]
async fn python_tcp_bidir() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([9998]))
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/tcp_bidir.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Bidirectional TCP test passed"),
        "expected tcp_bidir.py to print 'Bidirectional TCP test passed', got: {output:?}",
    );
}

#[tokio::test]
async fn python_http_server_client() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([9997]))
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/http_server_client.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("HTTP server+client test passed"),
        "expected http_server_client.py to print 'HTTP server+client test passed', got: {output:?}",
    );
}

#[tokio::test]
async fn python_http_get() {
    let rootfs = require_rootfs("python");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/http_get.py");
    let output = run_http_get(
        || SandboxBuilder::from_initrd(rootfs.clone()).scratch_mb(256),
        Exec::File(script),
    )
    .await;
    assert!(
        output.contains("Status: 200"),
        "expected http_get.py to get Status: 200, got:\n{output}",
    );
}

#[tokio::test]
async fn python_threaded_select() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([9123]))
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/threaded_select.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Threaded select() test passed"),
        "expected threaded_select.py to pass, got: {output:?}",
    );
}

/// `sys.exit()` ends the call the way it ends a script: its code is the
/// call's status, and the interpreter is still there for the next call.
#[tokio::test]
async fn python_sys_exit_ends_the_call() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("import sys\nprint('leaving')\nsys.exit(0)\nprint('not reached')")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\n");
    assert!(
        sandbox.run("import sys\nsys.exit(3)").await.is_err(),
        "a non-zero exit code did not fail the call"
    );
    assert!(
        sandbox.run("import sys\nsys.exit('bye')").await.is_err(),
        "a message exit did not fail the call"
    );
    assert!(
        sandbox.drain_output().contains("bye"),
        "the exit message was not printed"
    );
    sandbox.run("print('still here')").await.unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}

/// `sys.exit` ends the call, not the interpreter, so the guest can still be
/// snapshotted afterwards and the restored clone takes calls.
#[tokio::test]
async fn python_sys_exit_then_snapshot_restore() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-exit-snap");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("kept = 'yes'").await.unwrap();
    assert!(
        sandbox.run("import sys\nsys.exit(3)").await.is_err(),
        "a non-zero exit code did not fail the call"
    );
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut restored = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    restored.run("print('kept =', kept)").await.unwrap();
    assert_eq!(restored.drain_output(), "kept = yes\n");
}

/// A mounted directory too large to list in one host call fails that one
/// `listdir` with `EOVERFLOW`, and the sandbox goes on; before, the call
/// failed inside the hypervisor and poisoned the sandbox.  A listing's
/// other errors arrive as themselves, not as `EIO`.
#[tokio::test]
async fn python_huge_directory_listing_fails_cleanly() {
    let rootfs = require_rootfs("python");
    let dir = temp_dir("huge-dir");
    for i in 0..5000 {
        std::fs::write(dir.path().join(format!("file_{i}.txt")), b"").unwrap();
    }
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(vec![Mount::ro(dir.path(), "/mnt/huge")])
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno, os\n",
            "try:\n",
            "    os.listdir('/mnt/huge')\n",
            "    print('listed')\n",
            "except OSError as e:\n",
            "    print('listdir errno', e.errno == errno.EOVERFLOW)\n",
            "try:\n",
            "    os.listdir('/mnt/huge/nope')\n",
            "except OSError as e:\n",
            "    print('missing errno', e.errno == errno.ENOENT)\n",
        ))
        .await
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("listdir errno True"), "{out:?}");
    assert!(out.contains("missing errno True"), "{out:?}");
    sandbox.run("print('still here')").await.unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}

/// At the end of stdin, input() raises EOFError every time, read()
/// returns "", and select() reports stdin readable (the end is there to
/// read): before, stdin polled as not readable and anything that waited
/// on it deadlocked the guest.
#[test]
fn python_stdin_past_the_end() {
    let rootfs = require_rootfs("python");
    let dir = temp_dir("python-eof");
    let script = dir.path().join("eof.py");
    std::fs::write(
        &script,
        "import select, sys\n\
         print('line', repr(sys.stdin.readline()))\n\
         for i in range(2):\n    \
             try:\n        input()\n    \
             except EOFError:\n        print('EOFError', i)\n\
         print('read', repr(sys.stdin.read()))\n\
         r, _, _ = select.select([sys.stdin], [], [], None)\n\
         print('select', bool(r))\n",
    )
    .unwrap();
    let output = hluk_with_stdin(&rootfs, &script, b"hello\n");
    for line in [
        "line 'hello\\n'",
        "EOFError 0",
        "EOFError 1",
        "read ''",
        "select True",
    ] {
        assert!(output.contains(line), "{line} missing from: {output:?}");
    }
}

/// A function defined in `__main__`, called with JSON in and out; a
/// coroutine is awaited, an exception fails the call.
#[tokio::test]
async fn python_calls_a_function_with_json() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            "calls = 0\n\
             def greet(event):\n    \
                 global calls\n    \
                 calls += 1\n    \
                 return {'message': 'Hello, ' + event['name'], 'n': calls}\n\
             async def later(event):\n    \
                 return event['x'] * 2\n\
             def nothing():\n    \
                 print('ran')\n\
             def bad(event):\n    \
                 raise ValueError('handler boom')\n\
             def leave(event):\n    \
                 raise SystemExit(3)\n",
        )
        .await
        .unwrap();
    assert_eq!(
        sandbox.call("greet", r#"{"name":"World"}"#).await.unwrap(),
        r#"{"message": "Hello, World", "n": 1}"#
    );
    assert_eq!(
        sandbox.call("greet", r#"{"name":"again"}"#).await.unwrap(),
        r#"{"message": "Hello, again", "n": 2}"#
    );
    assert_eq!(sandbox.call("later", r#"{"x":21}"#).await.unwrap(), "42");
    // No input, no argument; None, no result.
    assert_eq!(sandbox.call("nothing", "").await.unwrap(), "");
    assert!(matches!(
        sandbox.call("bad", "{}").await,
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(matches!(
        sandbox.call("leave", "{}").await,
        Err(Error::CallFailed { status: 3 })
    ));
    assert!(sandbox.call("missing", "{}").await.is_err());
    assert!(sandbox.call("greet", "not json").await.is_err());
    let output = sandbox.drain_output();
    assert!(output.contains("ran"), "{output}");
    assert!(output.contains("handler boom"), "{output}");
    assert!(output.contains("no function 'missing'"), "{output}");
    // The interpreter serves on, its state intact.
    assert_eq!(
        sandbox.call("greet", r#"{"name":"x"}"#).await.unwrap(),
        r#"{"message": "Hello, x", "n": 3}"#
    );
}

/// The embedder's functions, as `hyperlight.call` and through
/// `hyperlight.host`; a host error is a `hyperlight.HostError`.
#[tokio::test]
async fn python_calls_host_functions() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .host_function("math.add", |args| {
            let v: Vec<f64> = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok(v.iter().sum::<f64>().to_string())
        })
        .host_function("math.fail", |_| Err("no can do".to_string()))
        .host_function("nothing", |_| Ok(String::new()))
        .boot()
        .unwrap();
    sandbox
        .run(
            "import hyperlight\n\
             from hyperlight import host\n\
             print(hyperlight.call('math.add', 2, 3), host.math.add(1, 2, 3), host.nothing())\n\
             try:\n    \
                 host.math.fail()\n\
             except hyperlight.HostError as e:\n    \
                 print('caught', e)\n\
             for n in ['', '\\ud800']:\n    \
                 try:\n        \
                     hyperlight.call(n)\n    \
                 except Exception as e:\n        \
                     print('threw', type(e).__name__)\n",
        )
        .await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "5 6 None\ncaught no can do\nthrew ValueError\nthrew UnicodeEncodeError\n"
    );
    // A handler can call the host too.
    sandbox
        .run("def total(event):\n    return host.math.add(*event['values'])\n")
        .await
        .unwrap();
    assert_eq!(
        sandbox
            .call("total", r#"{"values":[1,2,3]}"#)
            .await
            .unwrap(),
        "6"
    );
    // Another thread cannot: only the one running the call can reach the
    // host (see hl_host_call).
    sandbox
        .run(
            "import threading\n\
             def other():\n    \
                 try:\n        \
                     host.math.add(1)\n    \
                 except RuntimeError as e:\n        \
                     print('refused:', e)\n\
             t = threading.Thread(target=other)\n\
             t.start()\n\
             t.join()\n",
        )
        .await
        .unwrap();
    assert!(sandbox.drain_output().contains("refused:"));
}

/// A function defined by the file `--guest-exec` ran can be called:
/// runpy runs it in a module of its own, not `__main__`.
#[tokio::test]
async fn python_calls_a_function_from_a_guest_exec_file() {
    let dir = temp_dir("py-guest-exec-call");
    std::fs::write(
        dir.path().join("handler.py"),
        "def handler(event):\n    return {'hi': event['name']}\n",
    )
    .unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .mount(Mount::ro(dir.path(), "/app"))
        .boot()
        .unwrap();
    sandbox
        .run(Exec::Guest("/app/handler.py".into()))
        .await
        .unwrap();
    assert_eq!(
        sandbox.call("handler", r#"{"name":"py"}"#).await.unwrap(),
        r#"{"hi": "py"}"#
    );
}

/// `examples/python/handler.py`: loaded once, called twice, its state carried over.
#[tokio::test]
async fn python_handler_example() {
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/handler.py");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(example)).await.unwrap();
    assert_eq!(
        sandbox
            .call("handler", r#"{"name":"World"}"#)
            .await
            .unwrap(),
        r#"{"greeting": "Hello, World!", "calls": 1}"#
    );
    assert_eq!(
        sandbox
            .call("handler", r#"{"name":"again"}"#)
            .await
            .unwrap(),
        r#"{"greeting": "Hello, again!", "calls": 2}"#
    );
}

/// The kernel keeps the environment it last fetched and refetches only
/// when the host changed it: every change reaches the next call, whether
/// or not a snapshot restore came in between.
#[tokio::test]
async fn python_environment_changes_reach_every_call() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let read = async |sandbox: &mut hyperlight_unikraft::AppSandbox| {
        sandbox
            .run("import os; print(os.environ.get('VERSIONED', '-'))")
            .await
            .unwrap();
        sandbox.drain_output().trim().to_string()
    };
    sandbox.set_env_vars(&[("VERSIONED", "one")]);
    assert_eq!(read(&mut sandbox).await, "one");
    assert_eq!(read(&mut sandbox).await, "one");
    sandbox.set_env_vars(&[("VERSIONED", "two")]);
    assert_eq!(read(&mut sandbox).await, "two");
    let snapshot = sandbox.snapshot().unwrap();
    sandbox.set_env_vars(&[("VERSIONED", "three")]);
    assert_eq!(read(&mut sandbox).await, "three");
    // Restored to a snapshot whose kernel copy says "two": the host's
    // "three" still wins.
    sandbox.restore(snapshot.clone()).unwrap();
    assert_eq!(read(&mut sandbox).await, "three");
    sandbox.restore(snapshot).unwrap();
    sandbox.set_env_vars(&[("VERSIONED", "four")]);
    assert_eq!(read(&mut sandbox).await, "four");
}

/// Files extracted from the initrd reference it rather than a copy, so the
/// initrd has to survive a snapshot whole: modules no one imported before
/// the snapshot load after a restore, in place and in a new sandbox, and a
/// rootfs file changed after the snapshot is back to its contents on the
/// next restore.
#[tokio::test]
async fn python_rootfs_reads_after_restore() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let snapshot = sandbox.snapshot().unwrap();
    let code = "import decimal, email.parser, http.client, json\n\
                p = json.__file__\n\
                before = open(p).read()\n\
                with open(p, 'a') as f: f.write('# changed')\n\
                print('rootfs-ok', '# changed' not in before,\n\
                      open(p).read() == before + '# changed')";
    let run = async |sandbox: &mut hyperlight_unikraft::AppSandbox| {
        sandbox.run(code).await.unwrap();
        sandbox.drain_output().trim().to_string()
    };
    sandbox.restore(snapshot.clone()).unwrap();
    assert_eq!(run(&mut sandbox).await, "rootfs-ok True True");
    sandbox.restore(snapshot.clone()).unwrap();
    assert_eq!(run(&mut sandbox).await, "rootfs-ok True True");
    let mut fresh = SandboxBuilder::from_snapshot(snapshot).boot().unwrap();
    assert_eq!(run(&mut fresh).await, "rootfs-ok True True");
}

/// One write larger than a host call carries arrives whole: the kernel
/// sends it in pieces (it used to keep the first 4 KiB and report the
/// rest written), cut between characters, never inside one.
#[tokio::test]
async fn python_large_write_arrives_whole() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("import sys\nsys.stdout.write('x' * 100000)\nsys.stdout.flush()")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "x".repeat(100_000));

    // Three bytes a character: 4096 falls inside one.
    sandbox
        .run("import sys\nsys.stdout.write('\u{20ac}' * 3000)\nsys.stdout.flush()")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "\u{20ac}".repeat(3000));
}

/// Standard error is a stream of its own: `drain_stderr` has it alone,
/// `drain_output` both, in the order written, and newlines arrive as the
/// program wrote them.
#[tokio::test]
async fn python_stderr_apart_from_stdout() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .console_terminal(true)
        .boot()
        .unwrap();
    sandbox
        .run("import sys\nprint('out')\nprint('err', file=sys.stderr)\nprint('out again')")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_stderr(), "err\n");
    assert_eq!(sandbox.drain_output(), "out\nerr\nout again\n");

    assert!(sandbox.run("1 / 0").await.is_err());
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("ZeroDivisionError"), "{stderr:?}");

    // Multi-byte characters, well past a page: they arrive whole.
    sandbox
        .run("import sys\nsys.stderr.write('\u{20ac}' * 3000)\nsys.stderr.flush()")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_stderr(), "\u{20ac}".repeat(3000));

    // fd 2 is a file of its own, a terminal where this process's stderr
    // is one; /dev/stdout keeps newlines as written too.
    sandbox.drain_output();
    sandbox
        .run("import os\nprint(os.fstat(1).st_ino != os.fstat(2).st_ino, os.isatty(2))\nf = open('/dev/stdout', 'w')\nf.write('a\\nb\\n')\nf.close()").await
        .unwrap();
    use std::io::IsTerminal;
    let terminal = if std::io::stderr().is_terminal() {
        "True"
    } else {
        "False"
    };
    assert_eq!(sandbox.drain_output(), format!("True {terminal}\na\nb\n"));
}

/// A host without `HostWriteErr` gets standard error through `HostWrite`,
/// all of it, as before the stream had its own.
#[tokio::test]
async fn python_stderr_reaches_a_host_without_host_write_err() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .without_host_write_err()
        .boot()
        .unwrap();
    sandbox
        .run("import sys\nprint('out')\nprint('err', file=sys.stderr)\nsys.stderr.write('e' * 10000 + '\\n')\nsys.stderr.flush()\nprint('out again')").await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        format!("out\nerr\n{}\nout again\n", "e".repeat(10000))
    );
}

/// What a call leaves in Python's buffers reaches the host when the call
/// ends: output with no newline, bytes through `sys.stdout.buffer`, the
/// original `sys.__stdout__` once `sys.stdout` is replaced, and what was
/// written before an exception (ahead of its traceback) or a
/// `SystemExit`, for code, a guest file and a guest function call alike.
#[tokio::test]
async fn python_unflushed_output_reaches_the_host() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("print('a', end='')").await.unwrap();
    assert_eq!(sandbox.drain_output(), "a");
    sandbox
        .run("import sys\nsys.stdout.buffer.write(b'bytes')")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "bytes");
    sandbox
        .run("import io, sys\nsys.__stdout__.write('original')\nsys.stdout = io.StringIO()")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "original");
    sandbox
        .run("import sys\nsys.stdout = sys.__stdout__")
        .await
        .unwrap();

    assert!(
        sandbox
            .run("import sys\nsys.stdout.write('before')\n1 / 0")
            .await
            .is_err()
    );
    let output = sandbox.drain_output();
    assert!(output.starts_with("beforeTraceback"), "{output:?}");
    assert!(
        sandbox
            .run("import sys\nsys.stdout.write('out')\nsys.stderr.write('e')\nsys.exit(3)")
            .await
            .is_err()
    );
    let stderr = sandbox.drain_stderr();
    assert!(stderr.starts_with('e'), "{stderr:?}");
    let output = sandbox.drain_output();
    assert!(output.starts_with("out"), "{output:?}");

    sandbox
        .run("open('/tmp/unflushed.py', 'w').write(\"print('guest file', end='')\")")
        .await
        .unwrap();
    sandbox.drain_output();
    sandbox
        .run(Exec::Guest("/tmp/unflushed.py".into()))
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "guest file");

    sandbox
        .run("def quiet():\n    print('q', end='')\n")
        .await
        .unwrap();
    sandbox.drain_output();
    assert_eq!(sandbox.call("quiet", "").await.unwrap(), "");
    assert_eq!(sandbox.drain_output(), "q");

    sandbox
        .run("def fails():\n    print('x', end='')\n    1 / 0\n")
        .await
        .unwrap();
    sandbox.drain_output();
    assert!(sandbox.call("fails", "").await.is_err());
    let output = sandbox.drain_output();
    assert!(output.starts_with("xTraceback"), "{output:?}");
}

/// The issue's program: 4 MiB written 1 KiB at a time keeps the first
/// 64 KiB on the host and counts the rest, however the guest writes it,
/// reopening `/dev/stdout` included.  The next call starts a new budget.
#[tokio::test]
async fn python_output_limit() {
    const LIMIT: usize = 64 * 1024;
    const WRITTEN: u64 = 4096 * 1024;
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .output_limit(LIMIT)
        .echo_output(false)
        .boot()
        .unwrap();
    sandbox.drain_output();

    for code in [
        "import os\nfor _ in range(4096): os.write(1, b'X' * 1024)",
        "f = open('/dev/stdout', 'wb', buffering=0)\nfor _ in range(4096): f.write(b'X' * 1024)",
    ] {
        sandbox.run(code).await.unwrap();
        let output = sandbox.take_output();
        assert_eq!(output.text.len(), LIMIT, "{code}");
        assert!(output.text.bytes().all(|b| b == b'X'), "{code}");
        assert_eq!(output.omitted, WRITTEN - LIMIT as u64, "{code}");
    }

    sandbox.run("print('next')").await.unwrap();
    let output = sandbox.take_output();
    assert_eq!(output.text, "next\n");
    assert!(!output.truncated());
}

/// Standard error has a budget of its own, and `take_output` clears it.
#[tokio::test]
async fn python_output_limit_stderr() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .output_limit(100)
        .echo_output(false)
        .boot()
        .unwrap();
    sandbox.drain_output();

    sandbox
        .run("import os\nos.write(2, b'E' * 300)\nos.write(1, b'O' * 10)")
        .await
        .unwrap();
    let stderr = sandbox.take_stderr();
    assert_eq!(stderr.text, "E".repeat(100));
    assert_eq!(stderr.omitted, 200);
    let output = sandbox.take_output();
    assert_eq!(output.text, "E".repeat(100));
    assert_eq!(output.omitted, 210);

    sandbox
        .run("import os\nos.write(2, b'E' * 300)")
        .await
        .unwrap();
    sandbox.drain_output();
    assert_eq!(sandbox.take_stderr(), Default::default());
}

/// A restored guest gets the limit of the builder that restores it.
#[tokio::test]
async fn python_output_limit_after_restore() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-output-limit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .output_limit(10)
        .echo_output(false)
        .boot()
        .unwrap();
    sandbox.drain_output();
    sandbox.run("print('0123456789abcdef')").await.unwrap();
    let output = sandbox.take_output();
    assert_eq!(output.text, "0123456789");
    assert_eq!(output.omitted, 7);
}

/// Off, `echo_output` keeps the guest's output off this process's stdout
/// and stderr; on, the default, each stream reaches its own.  The test
/// harness captures what a test prints, so each case runs in a child, this
/// test again, uncaptured.
#[tokio::test]
async fn python_echo_output() {
    if let Ok(echo) = std::env::var("HLUK_TEST_ECHO") {
        let mut builder = SandboxBuilder::from_initrd(require_rootfs("python")).scratch_mb(256);
        // "on" is the default, left as it is.
        if echo == "off" {
            builder = builder.echo_output(false);
        }
        let mut sandbox = builder.boot().unwrap();
        sandbox
            .run("import os\nos.write(1, b'marker-out')\nos.write(2, b'marker-err')")
            .await
            .unwrap();
        // Captured either way.
        assert_eq!(sandbox.drain_stderr(), "marker-err");
        return;
    }
    for (echo, shown) in [("off", false), ("on", true)] {
        let child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["python_echo_output", "--exact", "--nocapture"])
            .env("HLUK_TEST_ECHO", echo)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&child.stdout);
        let stderr = String::from_utf8_lossy(&child.stderr);
        assert!(child.status.success(), "echo {echo}: {stdout}{stderr}");
        assert_eq!(
            stdout.contains("marker-out"),
            shown,
            "echo {echo}: {stdout}"
        );
        assert_eq!(
            stderr.contains("marker-err"),
            shown,
            "echo {echo}: {stderr}"
        );
        assert!(!stdout.contains("marker-err"), "echo {echo}: {stdout}");
        assert!(!stderr.contains("marker-out"), "echo {echo}: {stderr}");
    }
}

/// Children after and from threads, waits, kills, timeouts, sessions and
/// process groups: what an agent's command runner does with processes.
/// Over thirty Pythons, one after another, in the image's 256 MiB: each
/// gives its memory back when it ends.
#[tokio::test]
async fn python_processes() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/process_demo.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Process demo passed"),
        "expected process_demo.py to pass, got: {output:?}",
    );
}

/// select() on host sockets reports only what is there, so a server's
/// shutdown() returns.
#[tokio::test]
async fn python_server_shutdown() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/server_shutdown.py");
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Server shutdown demo passed"),
        "expected server_shutdown.py to pass, got: {output:?}",
    );
}

/// PYTHONPATH given after the interpreter started (a restored guest) is on
/// sys.path for the call, in place of the one before it, and gone with it.
#[tokio::test]
async fn python_pythonpath_follows_the_environment() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-pythonpath");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    let probe = "import sys; print('path', [p for p in sys.path if p.startswith('/lib-')])";
    sandbox.set_env_vars(&[("PYTHONPATH", "/lib-a:/lib-b")]);
    sandbox.run(probe).await.unwrap();
    sandbox.set_env_vars(&[("PYTHONPATH", "/lib-c")]);
    sandbox.run(probe).await.unwrap();
    sandbox.set_env_vars(&[]);
    sandbox.run(probe).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("path ['/lib-a', '/lib-b']")
            && output.contains("path ['/lib-c']")
            && output.contains("path []"),
        "expected sys.path to follow PYTHONPATH, got: {output:?}",
    );
}

/// A guest out of memory kills the program that asked, as Linux's OOM
/// killer would, rather than taking the kernel down with a fault.
#[tokio::test]
async fn python_out_of_memory_is_a_kill() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    match sandbox.run("b = b'x' * (400 << 20)").await {
        Err(Error::GuestExited { status: 137 }) => {}
        other => panic!("expected the guest to exit with 137, got {other:?}"),
    }
}

/// The memory of a large file goes back to programs once the file is
/// deleted: after three 40 MiB files came and went, a child can allocate
/// 120 MiB (the kernel kept about three times a file's size before).
#[tokio::test]
async fn python_deleted_file_memory_is_reused() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            "import os, subprocess, sys\n\
             for _ in range(3):\n\
             \x20   with open('/tmp/big', 'wb') as f:\n\
             \x20       for _ in range(40):\n\
             \x20           f.write(b'x' * (1 << 20))\n\
             \x20   os.unlink('/tmp/big')\n\
             r = subprocess.run([sys.executable, '-c', 'b = bytearray(120 << 20)'])\n\
             print('child', r.returncode)",
        )
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "child 0\n");
}

/// With a child holding all but a few MiB, the parent's file write fails
/// with ENOSPC, as on a full tmpfs, instead of the guest's kernel faulting
/// with no memory left; the parent can still kill the child, and the write
/// works once it is gone. The buffer written is made before the child fills
/// memory, so that the write, not the parent's own page fault, runs out.
#[tokio::test]
async fn python_full_guest_fails_the_write_not_the_guest() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            "import os, subprocess, sys\n\
             chunk = b'x' * (1 << 20)\n\
             hog = subprocess.Popen([sys.executable, '-c',\n\
             \x20   'import os, sys\\n'\n\
             \x20   'l = []\\n'\n\
             \x20   'page = os.sysconf(\"SC_PAGE_SIZE\")\\n'\n\
             \x20   'while os.sysconf(\"SC_AVPHYS_PAGES\") * page > (8 << 20): l.append(bytearray(1 << 20))\\n'\n\
             \x20   'print(\"holding\", flush=True)\\n'\n\
             \x20   'sys.stdin.read()\\n'],\n\
             \x20   stdin=subprocess.PIPE, stdout=subprocess.PIPE)\n\
             print('hog:', hog.stdout.readline().decode().strip())\n\
             def write():\n\
             \x20   try:\n\
             \x20       with open('/tmp/big', 'wb') as f:\n\
             \x20           for _ in range(32):\n\
             \x20               f.write(chunk)\n\
             \x20       return 'ok'\n\
             \x20   except OSError as e:\n\
             \x20       return os.strerror(e.errno)\n\
             print('full:', write())\n\
             hog.kill()\n\
             print('killed:', hog.wait())\n\
             print('after:', write())",
        ).await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "hog: holding\nfull: No space left on device\nkilled: -9\nafter: ok\n"
    );
}

/// An epoll instance cannot watch itself (EINVAL, as on Linux); closing it
/// after the attempt used to crash the guest.
#[tokio::test]
async fn python_epoll_refuses_itself() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs).boot().unwrap();
    sandbox
        .run(
            "import os, select\n\
             ep = select.epoll()\n\
             try:\n\
             \x20   ep.register(ep.fileno(), select.EPOLLIN)\n\
             \x20   print('registered')\n\
             except OSError as e:\n\
             \x20   print(os.strerror(e.errno))\n\
             ep.close()\n\
             print('closed')",
        )
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "Invalid argument\nclosed\n");
}

/// A variable the guest had and the host set over it for a while has the
/// guest's own value again once the host stops sending it.
#[tokio::test]
async fn python_env_var_taken_back_restores_the_guests_own() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-env-own");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    let probe = "import os; print('PATH=' + os.environ.get('PATH', '<unset>'))";
    sandbox.run(probe).await.unwrap();
    let own = sandbox.drain_output();
    sandbox.set_env_vars(&[("PATH", "/opt/bin")]);
    sandbox.run(probe).await.unwrap();
    sandbox.set_env_vars(&[]);
    sandbox.run(probe).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        own.contains("PATH=/usr/local/bin")
            && output.starts_with("PATH=/opt/bin")
            && output.ends_with(&own),
        "expected PATH to come back to {own:?}, got: {output:?}",
    );
}

/// A variable the host gave at boot and then stops sending is unset: it
/// was the host's, not the guest's own.
#[tokio::test]
async fn python_env_var_given_at_boot_can_be_taken_away() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .env("PYTHONPATH", "/boot-lib")
        .boot()
        .unwrap();
    let probe = "import os, sys; print(os.environ.get('PYTHONPATH'), '/boot-lib' in sys.path)";
    // Changed before the first call, then taken away: still the host's.
    sandbox.set_env_vars(&[("PYTHONPATH", "/other")]);
    sandbox.run(probe).await.unwrap();
    sandbox.set_env_vars(&[]);
    sandbox.run(probe).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.starts_with("/other False") && output.contains("None False"),
        "expected PYTHONPATH to be gone, got: {output:?}",
    );
}

/// The driver's own PATH stays the guest's when the host gives the same
/// value at boot: taking the host's away leaves PATH set, not unset.
#[tokio::test]
async fn python_path_given_at_boot_as_the_drivers_is_kept() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .boot()
        .unwrap();
    let probe = "import os; print('PATH=' + os.environ.get('PATH', '<unset>'))";
    sandbox.set_env_vars(&[]);
    sandbox.run(probe).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("PATH=/usr/local/bin:/usr/bin:/bin"),
        "expected the driver's PATH, got: {output:?}",
    );
}

/// An empty entry of a nonempty PYTHONPATH is the working directory, as at
/// startup; and a standard directory a PYTHONPATH moved to the front goes
/// back where it was once that PYTHONPATH is gone, also when the next one
/// moves another.
#[tokio::test]
async fn python_pythonpath_keeps_empty_entries_and_the_standard_order() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-pythonpath-order");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox
        .run("import sys, sysconfig; print('site', sysconfig.get_paths()['purelib']); print('start', sys.path)").await
        .unwrap();
    let start = sandbox.drain_output();
    let site = start
        .lines()
        .find_map(|l| l.strip_prefix("site "))
        .expect("purelib")
        .to_owned();
    let start = start
        .lines()
        .find_map(|l| l.strip_prefix("start "))
        .expect("sys.path")
        .to_owned();

    sandbox.set_env_vars(&[("PYTHONPATH", "/lib-a:")]);
    sandbox
        .run("import os, sys; print('cwd', os.path.abspath('') in sys.path)")
        .await
        .unwrap();
    let dynload = start
        .split('\'')
        .find(|p| p.ends_with("/lib-dynload"))
        .expect("lib-dynload")
        .to_owned();
    sandbox.set_env_vars(&[("PYTHONPATH", dynload.as_str())]);
    sandbox.run("pass").await.unwrap();
    sandbox.set_env_vars(&[("PYTHONPATH", site.as_str())]);
    sandbox
        .run("import sys; print('front', sys.path[0])")
        .await
        .unwrap();
    sandbox.set_env_vars(&[]);
    sandbox
        .run("import sys; print('end', sys.path)")
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("cwd True")
            && output.contains(&format!("front {site}"))
            && output.contains(&format!("end {start}")),
        "expected {start} back after {site} was moved, got: {output:?}",
    );
}

/// A mounted file's mode and times are the host's, and chmod and utime
/// change them there: the mode used to be 755 for every file, the times
/// the epoch and utime did nothing.
#[tokio::test]
async fn python_mount_modes_and_times() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-modes-times");
    let file = mount_dir.path().join("f.txt");
    std::fs::write(&file, "x").unwrap();
    let mtime = std::time::UNIX_EPOCH + std::time::Duration::new(1_500_000_000, 250_000_000);
    std::fs::File::options()
        .write(true)
        .open(&file)
        .unwrap()
        .set_modified(mtime)
        .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o640)).unwrap();
    }
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox
        .run(
            "import os\n\
             st = os.stat('/mnt/host/f.txt')\n\
             print('mode', oct(st.st_mode & 0o777), 'mtime', st.st_mtime_ns)\n\
             os.chmod('/mnt/host/f.txt', 0o4755)\n\
             print('set', oct(os.stat('/mnt/host/f.txt').st_mode & 0o7777))\n\
             os.chmod('/mnt/host/f.txt', 0o600)\n\
             os.utime('/mnt/host/f.txt', ns=(1_000_000_000_000_000_000, 1_600_000_000_500_000_000))\n\
             try:\n\
             \x20   os.utime('/mnt/host/f.txt', (-1, -1))\n\
             except OSError as e:\n\
             \x20   print('before 1970', e.errno)\n\
             print('changed')",
        ).await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("mtime 1500000000250000000"), "{output}");
    // EINVAL, and the host still serving: the crates that set times panic
    // on one before the epoch
    assert!(output.contains("before 1970 22"), "{output}");
    assert!(output.contains("changed"), "{output}");
    let meta = std::fs::metadata(&file).unwrap();
    assert_eq!(
        meta.modified().unwrap(),
        std::time::UNIX_EPOCH + std::time::Duration::new(1_600_000_000, 500_000_000)
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert!(output.contains("mode 0o640"), "{output}");
        // The host file never takes set-user-ID from the guest
        assert!(output.contains("set 0o755"), "{output}");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    }
}

/// mkdir and rmdir take a directory with trailing slashes, as busybox's
/// `mkdir -p` passes it, on the host's directories and the guest's own;
/// the working directory at a mount's root has no trailing slash.
#[tokio::test]
async fn python_trailing_slashes() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-trailing-slash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox
        .run(
            "import os\n\
             for top in ('/mnt/host', '/tmp'):\n\
             \x20   os.mkdir(top + '/a/')\n\
             \x20   os.mkdir(top + '/a/b//')\n\
             \x20   print(top, sorted(os.listdir(top + '/a')))\n\
             \x20   os.rmdir(top + '/a/b/')\n\
             \x20   os.rmdir(top + '/a/')\n\
             os.chdir('/mnt/host')\n\
             print('cwd', repr(os.getcwd()))",
        )
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("/mnt/host ['b']"), "{output}");
    assert!(output.contains("/tmp ['b']"), "{output}");
    assert!(output.contains("cwd '/mnt/host'"), "{output}");
    assert_eq!(std::fs::read_dir(mount_dir.path()).unwrap().count(), 0);
}

/// A guest whose output is captured, not printed to a terminal, has no
/// terminal for its console: programs must not color it or prompt.
#[tokio::test]
async fn python_captured_console_is_not_a_terminal() {
    let rootfs = require_rootfs("python");
    // Allowed a terminal, but with nothing echoed there is none to see
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .echo_output(false)
        .console_terminal(true)
        .boot()
        .unwrap();
    sandbox
        .run(
            "import sys, termios\n\
             print([s.isatty() for s in (sys.stdin, sys.stdout, sys.stderr)])\n\
             try:\n\
             \x20   termios.tcgetattr(1)\n\
             except termios.error as e:\n\
             \x20   print('tcgetattr', e.args[0])",
        )
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("[False, False, False]"), "{output}");
    // ENOTTY
    assert!(output.contains("tcgetattr 25"), "{output}");
}

/// A script past what a host call carries runs; one past what a guest
/// function call carries is refused before it is sent, and the guest
/// serves on.
#[tokio::test]
async fn python_large_script() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .echo_output(false)
        .boot()
        .unwrap();
    let padding = format!("# {}\n", "x".repeat(97)).repeat(5_000);
    sandbox
        .run(format!("{padding}print('large ok')"))
        .await
        .unwrap();
    assert!(sandbox.drain_output().contains("large ok"));
    let too_large = "#".repeat(hyperlight_unikraft::CALL_PAYLOAD_MAX + 1);
    assert!(matches!(
        sandbox.run(too_large).await,
        Err(Error::CallTooLarge { bytes }) if bytes == hyperlight_unikraft::CALL_PAYLOAD_MAX + 1
    ));
    sandbox.run("print('still serving')").await.unwrap();
    assert!(sandbox.drain_output().contains("still serving"));
}

/// What a failed program says is its own: no kernel messages for an exec
/// that failed as a PATH search does, no driver message after a call that
/// exited with a status.
#[tokio::test]
async fn python_failures_print_only_their_own_words() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .echo_output(false)
        .boot()
        .unwrap();
    sandbox
        .run(
            "import subprocess\n\
             try:\n\
             \x20   subprocess.run(['/nonexistent'])\n\
             except FileNotFoundError:\n\
             \x20   print('not found')",
        )
        .await
        .unwrap();
    // Standard error first: draining the output clears it too
    assert_eq!(sandbox.drain_stderr(), "");
    assert!(sandbox.drain_output().contains("not found"));
    assert!(matches!(
        sandbox.run("import sys; sys.exit(3)").await,
        Err(Error::CallFailed { status: 3 })
    ));
    assert_eq!(sandbox.drain_stderr(), "");
}

/// A write far past the end of a file fails, as on Linux, not a crash: cut
/// short where an offset ends, it is ENOSPC for a file no memory holds, and
/// EFBIG at the end itself. The file is left as it was.
#[tokio::test]
async fn python_write_far_past_the_end_fails_cleanly() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"import os
fd = os.open("/tmp/f", os.O_RDWR | os.O_CREAT)
os.write(fd, b"x")
# The write is cut short where an offset ends, and the file cannot grow to there.
try:
    os.pwrite(fd, b"y" * 8000, 2**63 - 4000)
    print("wrote")
except OSError as e:
    print("near the end: errno", e.errno)
# At the end itself nothing can be written.
try:
    os.pwrite(fd, b"y", 2**63 - 1)
    print("wrote")
except OSError as e:
    print("at the end: errno", e.errno)
print("size", os.fstat(fd).st_size)
"#,
        )
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("near the end: errno 28")
            && output.contains("at the end: errno 27")
            && output.contains("size 1"),
        "expected ENOSPC near the end, EFBIG at it, and the file untouched, got: {output:?}"
    );
}
