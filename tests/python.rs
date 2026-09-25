// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;

use common::{hluk_with_stdin, require_rootfs, temp_dir};
use hyperlight_unikraft::{Error, Exec, ListenPorts, Mount, NetworkPolicy, SandboxBuilder};

#[test]
fn python_inline_code() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("print('hluk-test-ok')").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("hluk-test-ok"),
        "expected guest to print 'hluk-test-ok', got: {output:?}",
    );
}

#[test]
fn python_exec_file() {
    let rootfs = require_rootfs("python");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/hello.py");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.py to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn python_snapshot_round_trip() {
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
    sandbox.run("print('restored-ok')").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("restored-ok"),
        "expected restored guest to print 'restored-ok', got: {output:?}",
    );
}

#[test]
fn python_multiple_runs() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("x = 1 + 1").unwrap();
    sandbox.run("print(f'x={x}')").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("x=2"),
        "expected 'x=2' after multiple runs, got: {output:?}",
    );
    sandbox.run("import sys; print(sys.version)").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("3.12"),
        "expected sys.version to contain '3.12', got: {output:?}",
    );
}

#[test]
fn python_env_vars() {
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

#[test]
fn python_env_vars_inline() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[("SECRET", "42")]);
    sandbox
        .run("import os; print(os.environ['SECRET'])")
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("42"),
        "expected '42' from SECRET env var, got: {output:?}",
    );
}

#[test]
fn python_env_vars_snapshot_restore() {
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
        .unwrap();
    let output = sandbox.drain_output();
    eprintln!("snapshot env output: {output:?}");
    assert!(
        output.contains("RESTORED_VAR=from_snapshot"),
        "expected env var set after snapshot restore, got: {output:?}",
    );
}

/// Env vars are stateful across dispatches without restore.
#[test]
fn python_env_vars_stateful_across_dispatches() {
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
        .unwrap();
    let out3 = sandbox.drain_output();
    assert!(
        out3.contains("step3: STATEFUL_VAR=persisted"),
        "expected env vars to persist across dispatches without restore, got: {out3:?}",
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

#[test]
fn python_fs_ops() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-ops");

    let mounts = vec![Mount::rw(mount_dir.path(), "/mnt/host")];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/fs_ops.py");
    sandbox.run(Exec::File(script)).unwrap();
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

#[test]
fn python_fs_large_file() {
    let rootfs = require_rootfs("python");
    let mount_dir = temp_dir("fs-large");

    let mounts = vec![Mount::rw(mount_dir.path(), "/mnt/host")];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/fs_large.py");
    sandbox.run(Exec::File(script)).unwrap();
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

#[test]
fn python_guest_fs_ops() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/guest_fs.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Guest filesystem tests passed"),
        "expected guest_fs.py to print 'Guest filesystem tests passed', got: {output:?}",
    );
}

#[test]
fn python_asyncio() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([18095]))
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/asyncio_demo.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("asyncio demo passed"),
        "expected asyncio demo to pass, got: {output:?}",
    );
}

#[test]
fn python_threading() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/threading_demo.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Threading demo passed"),
        "expected threading_demo.py to print 'Threading demo passed', got: {output:?}",
    );
}

#[test]
fn python_subprocess() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/subprocess_demo.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Subprocess demo passed"),
        "expected subprocess_demo.py to print 'Subprocess demo passed', got: {output:?}",
    );
}

#[test]
fn python_tcp_echo() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/tcp_echo.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("TCP echo test passed"),
        "expected tcp_echo.py to print 'TCP echo test passed', got: {output:?}",
    );
}

#[test]
fn python_tcp_bidir() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/tcp_bidir.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Bidirectional TCP test passed"),
        "expected tcp_bidir.py to print 'Bidirectional TCP test passed', got: {output:?}",
    );
}

#[test]
fn python_http_server_client() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/http_server_client.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("HTTP server+client test passed"),
        "expected http_server_client.py to print 'HTTP server+client test passed', got: {output:?}",
    );
}

#[test]
fn python_http_get() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/http_get.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Status: 200"),
        "expected http_get.py to get Status: 200, got: {output:?}",
    );
}

#[test]
fn python_threaded_select() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/threaded_select.py");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Threaded select() test passed"),
        "expected threaded_select.py to pass, got: {output:?}",
    );
}

/// `sys.exit()` ends the call the way it ends a script: its code is the
/// call's status, and the interpreter is still there for the next call.
#[test]
fn python_sys_exit_ends_the_call() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("import sys\nprint('leaving')\nsys.exit(0)\nprint('not reached')")
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\r\n");
    assert!(
        sandbox.run("import sys\nsys.exit(3)").is_err(),
        "a non-zero exit code did not fail the call"
    );
    assert!(
        sandbox.run("import sys\nsys.exit('bye')").is_err(),
        "a message exit did not fail the call"
    );
    assert!(
        sandbox.drain_output().contains("bye"),
        "the exit message was not printed"
    );
    sandbox.run("print('still here')").unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}

/// `sys.exit` ends the call, not the interpreter, so the guest can still be
/// snapshotted afterwards and the restored clone takes calls.
#[test]
fn python_sys_exit_then_snapshot_restore() {
    let rootfs = require_rootfs("python");
    let snap_dir = temp_dir("py-exit-snap");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("kept = 'yes'").unwrap();
    assert!(
        sandbox.run("import sys\nsys.exit(3)").is_err(),
        "a non-zero exit code did not fail the call"
    );
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut restored = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    restored.run("print('kept =', kept)").unwrap();
    assert_eq!(restored.drain_output(), "kept = yes\r\n");
}

/// A mounted directory too large to list in one host call fails that one
/// `listdir` with `EOVERFLOW`, and the sandbox goes on; before, the call
/// failed inside the hypervisor and poisoned the sandbox.  A listing's
/// other errors arrive as themselves, not as `EIO`.
#[test]
fn python_huge_directory_listing_fails_cleanly() {
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
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("listdir errno True"), "{out:?}");
    assert!(out.contains("missing errno True"), "{out:?}");
    sandbox.run("print('still here')").unwrap();
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
#[test]
fn python_calls_a_function_with_json() {
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
        .unwrap();
    assert_eq!(
        sandbox.call("greet", r#"{"name":"World"}"#).unwrap(),
        r#"{"message": "Hello, World", "n": 1}"#
    );
    assert_eq!(
        sandbox.call("greet", r#"{"name":"again"}"#).unwrap(),
        r#"{"message": "Hello, again", "n": 2}"#
    );
    assert_eq!(sandbox.call("later", r#"{"x":21}"#).unwrap(), "42");
    // No input, no argument; None, no result.
    assert_eq!(sandbox.call("nothing", "").unwrap(), "");
    assert!(matches!(
        sandbox.call("bad", "{}"),
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(matches!(
        sandbox.call("leave", "{}"),
        Err(Error::CallFailed { status: 3 })
    ));
    assert!(sandbox.call("missing", "{}").is_err());
    assert!(sandbox.call("greet", "not json").is_err());
    let output = sandbox.drain_output();
    assert!(output.contains("ran"), "{output}");
    assert!(output.contains("handler boom"), "{output}");
    assert!(output.contains("no function 'missing'"), "{output}");
    // The interpreter serves on, its state intact.
    assert_eq!(
        sandbox.call("greet", r#"{"name":"x"}"#).unwrap(),
        r#"{"message": "Hello, x", "n": 3}"#
    );
}

/// The embedder's functions, as `hyperlight.call` and through
/// `hyperlight.host`; a host error is a `hyperlight.HostError`.
#[test]
fn python_calls_host_functions() {
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
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "5 6 None\r\ncaught no can do\r\nthrew ValueError\r\nthrew UnicodeEncodeError\r\n"
    );
    // A handler can call the host too.
    sandbox
        .run("def total(event):\n    return host.math.add(*event['values'])\n")
        .unwrap();
    assert_eq!(sandbox.call("total", r#"{"values":[1,2,3]}"#).unwrap(), "6");
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
        .unwrap();
    assert!(sandbox.drain_output().contains("refused:"));
}

/// A function defined by the file `--guest-exec` ran can be called:
/// runpy runs it in a module of its own, not `__main__`.
#[test]
fn python_calls_a_function_from_a_guest_exec_file() {
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
    sandbox.run(Exec::Guest("/app/handler.py".into())).unwrap();
    assert_eq!(
        sandbox.call("handler", r#"{"name":"py"}"#).unwrap(),
        r#"{"hi": "py"}"#
    );
}

/// `examples/python/handler.py`: loaded once, called twice, its state carried over.
#[test]
fn python_handler_example() {
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/handler.py");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(example)).unwrap();
    assert_eq!(
        sandbox.call("handler", r#"{"name":"World"}"#).unwrap(),
        r#"{"greeting": "Hello, World!", "calls": 1}"#
    );
    assert_eq!(
        sandbox.call("handler", r#"{"name":"again"}"#).unwrap(),
        r#"{"greeting": "Hello, again!", "calls": 2}"#
    );
}
