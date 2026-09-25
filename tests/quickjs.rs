// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The quickjs image: scripts and ES modules, state kept across calls,
//! jobs and timers run before a call returns, errors fail the call and
//! leave the engine serving, and a warm snapshot's clones draw their own
//! random numbers.

mod common;

use common::{require_rootfs, temp_dir};
use hyperlight_unikraft::{Error, Exec, Mount, SandboxBuilder};

const SCRATCH_MB: usize = 64;

fn boot() -> hyperlight_unikraft::AppSandbox {
    SandboxBuilder::from_initrd(require_rootfs("quickjs"))
        .scratch_mb(SCRATCH_MB)
        .boot()
        .unwrap()
}

#[test]
fn quickjs_hello() {
    let mut sandbox = boot();
    sandbox
        .run("console.log('Hello from QuickJS', 6 * 7)")
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "Hello from QuickJS 42");
}

/// What a script declares is a global the next call can use: a handler
/// defined once and called many times.
#[test]
fn quickjs_state_is_kept_across_calls() {
    let mut sandbox = boot();
    sandbox
        .run("function handler(event) { return { greeting: 'Hello, ' + event.name }; }; var calls = 0;")
        .unwrap();
    for name in ["a", "b"] {
        sandbox
            .run(format!(
                "calls++; console.log(JSON.stringify(handler({{ name: '{name}' }})), calls)"
            ))
            .unwrap();
    }
    assert_eq!(
        sandbox.drain_output(),
        "{\"greeting\":\"Hello, a\"} 1\r\n{\"greeting\":\"Hello, b\"} 2\r\n"
    );
}

/// Code with `import` runs as a module, with top-level await; the call
/// returns once the timers it started have fired.
#[test]
fn quickjs_modules_timers_and_promises() {
    let mut sandbox = boot();
    sandbox
        .run(
            "import * as os from 'qjs:os';\n\
             await new Promise((r) => os.setTimeout(r, 20));\n\
             os.setTimeout(() => console.log('timer'), 20);\n\
             Promise.resolve().then(() => console.log('job'));\n\
             console.log('module');",
        )
        .unwrap();
    assert_eq!(sandbox.drain_output(), "module\r\njob\r\ntimer\r\n");
}

#[test]
fn quickjs_errors_fail_the_call_and_the_engine_serves_on() {
    let mut sandbox = boot();
    for code in [
        "throw new TypeError('script boom')",
        "import * as std from 'qjs:std'; throw new Error('module boom')",
        "Promise.reject(new Error('rejected'))",
    ] {
        assert!(
            matches!(sandbox.run(code), Err(Error::CallFailed { status: 1 })),
            "{code} did not fail the call"
        );
    }
    let output = sandbox.drain_output();
    for text in ["script boom", "module boom", "rejected"] {
        assert_eq!(output.matches(text).count(), 1, "{text} in: {output}");
    }
    sandbox.run("console.log('still here')").unwrap();
    assert_eq!(sandbox.drain_output().trim(), "still here");
}

#[test]
fn quickjs_file_env_and_mounts() {
    let data = temp_dir("quickjs-data");
    std::fs::write(data.path().join("input.txt"), "one two three\n").unwrap();
    std::fs::write(
        data.path().join("main.mjs"),
        "import * as std from 'qjs:std';\n\
         const text = std.loadFile('/mnt/data/input.txt');\n\
         console.log(scriptArgs.slice(1).join(','), std.getenv('GREETING'), text.split(/\\s+/).filter(Boolean).length);\n",
    )
    .unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("quickjs"))
        .scratch_mb(SCRATCH_MB)
        .mounts(vec![Mount::rw(data.path(), "/mnt/data")])
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[("GREETING", "hi")]);
    sandbox
        .run(Exec::Guest("/mnt/data/main.mjs x y".into()))
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "x,y hi 3");
}

/// Two restores of one snapshot draw different numbers: Math.random is
/// reseeded from the kernel, which reseeds on every restore.
#[test]
fn quickjs_snapshot_clones_draw_their_own_random_numbers() {
    let snap_dir = temp_dir("quickjs-snap");
    let mut sandbox = boot();
    sandbox.run("var seen = 'kept'").unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut draws = Vec::new();
    for _ in 0..2 {
        let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone
            .run("console.log(seen, Math.random(), Math.random())")
            .unwrap();
        let output = clone.drain_output();
        assert!(output.starts_with("kept "), "{output}");
        draws.push(output);
    }
    assert_ne!(draws[0], draws[1], "two clones drew the same numbers");
}

/// A handler defined once and called many times with JSON in and out, as
/// hyperlight-js runs one; a promise it returns is awaited.
#[test]
fn quickjs_calls_a_handler_with_json() {
    let mut sandbox = boot();
    sandbox
        .run(
            "function greet(event) { return { message: 'Hello, ' + event.name, n: ++greet.count } }\n\
             greet.count = 0;\n\
             async function later(event) { return event.x * 2 }",
        )
        .unwrap();
    assert_eq!(
        sandbox.call("greet", r#"{"name":"World"}"#).unwrap(),
        r#"{"message":"Hello, World","n":1}"#
    );
    assert_eq!(
        sandbox.call("greet", r#"{"name":"again"}"#).unwrap(),
        r#"{"message":"Hello, again","n":2}"#
    );
    assert_eq!(sandbox.call("later", r#"{"x":21}"#).unwrap(), "42");
    // Empty input is no argument at all.
    sandbox
        .run("function count(...args) { return args.length }")
        .unwrap();
    assert_eq!(sandbox.call("count", "").unwrap(), "0");
    assert_eq!(sandbox.call("count", "1").unwrap(), "1");
    // A handler that throws fails the call; one that is not there too.
    sandbox
        .run("function bad() { throw new Error('handler boom') }")
        .unwrap();
    assert!(matches!(
        sandbox.call("bad", "{}"),
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(sandbox.call("missing", "{}").is_err());
    assert!(sandbox.drain_output().contains("handler boom"));
}

/// The embedder's functions, as `host.call` and as `host:` module
/// imports; a host error is a JavaScript exception.
#[test]
fn quickjs_calls_host_functions() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("quickjs"))
        .scratch_mb(SCRATCH_MB)
        .host_function("math.add", |args| {
            let v: Vec<f64> = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok(v.iter().sum::<f64>().to_string())
        })
        .host_function("math.fail", |_| Err("no can do".to_string()))
        .host_function("math.verbose", |_| Err("x".repeat(1000)))
        .host_function("greet", |args| {
            let [name]: [String; 1] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok(serde_json::json!({ "hello": name }).to_string())
        })
        .boot()
        .unwrap();
    sandbox
        .run(
            "import { add, fail } from 'host:math';\n\
             console.log(add(2, 3), host.call('greet', 'js').hello);\n\
             try { fail() } catch (e) { console.log('caught', e.message) }",
        )
        .unwrap();
    assert_eq!(sandbox.drain_output(), "5 js\r\ncaught no can do\r\n");
    // A name that is not UTF-8 (a lone surrogate), or the empty one the
    // protocol reserves, throws in JavaScript; the guest serves on.
    sandbox
        .run(
            "for (const n of ['\\uD800', '']) {\n\
               try { host.call(n); console.log('no throw') } catch (e) { console.log('threw') }\n\
             }",
        )
        .unwrap();
    assert_eq!(sandbox.drain_output(), "threw\r\nthrew\r\n");
    // A long host error arrives whole.
    sandbox
        .run("try { host.call('math.verbose') } catch (e) { console.log(e.message.length) }")
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "1000");
    // A handler can call the host too.
    sandbox
        .run("function total(event) { return host.call('math.add', ...event.values) }")
        .unwrap();
    assert_eq!(sandbox.call("total", r#"{"values":[1,2,3]}"#).unwrap(), "6");
    // A module of no host functions is an import error.
    assert!(sandbox.run("import * as x from 'host:nothing'").is_err());
}

/// A rejection nobody handles fails the call even when another promise
/// is handled later; a result larger than a host call carries fails the
/// call rather than come back empty.
#[test]
fn quickjs_rejections_and_oversized_results_fail_the_call() {
    let mut sandbox = boot();
    assert!(matches!(
        sandbox.run(
            "const a = Promise.reject(new Error('lost'));\n\
             const b = Promise.reject(1);\n\
             b.catch(() => {});"
        ),
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(sandbox.drain_output().contains("lost"));

    // A global whose getter throws fails the call with that error, and
    // leaves nothing pending for the next call.
    sandbox
        .run("Object.defineProperty(globalThis, 'g', { get() { throw new Error('getter') } })")
        .unwrap();
    assert!(sandbox.call("g", "{}").is_err());
    assert!(sandbox.drain_output().contains("getter"));
    // `import (` with a space is a dynamic import: the script stays one.
    sandbox
        .run("function dyn() { return 7 }\nif (false) import ('./x.js');")
        .unwrap();
    assert_eq!(sandbox.call("dyn", "").unwrap(), "7");

    sandbox
        .run("function big(n) { return 'x'.repeat(n) }")
        .unwrap();
    assert_eq!(sandbox.call("big", "10").unwrap().len(), 12);
    // Past the call buffer (the driver refuses it), and between the
    // 64 KiB a host call carries and the buffer (the kernel does).
    for n in ["70000", "67000"] {
        assert!(
            sandbox.call("big", n).is_err(),
            "a {n}-byte result did not fail the call"
        );
    }
    sandbox.call("big", "10").unwrap();
}

/// A dotfile opened relative to the working directory, in a directory
/// below the root of a mount: the kernel used to glue the name onto the
/// directory (`/mnt/data/sub.dot`).
#[test]
fn quickjs_opens_a_dotfile_relative_to_the_working_directory() {
    let data = temp_dir("quickjs-dot");
    std::fs::create_dir(data.path().join("sub")).unwrap();
    std::fs::write(data.path().join("sub/.dot"), "hidden").unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("quickjs"))
        .scratch_mb(SCRATCH_MB)
        .mounts(vec![Mount::rw(data.path(), "/mnt/data")])
        .boot()
        .unwrap();
    sandbox
        .run(
            "import * as std from 'qjs:std'; import * as os from 'qjs:os';\n\
             os.chdir('/mnt/data/sub');\n\
             console.log(std.loadFile('.dot'));",
        )
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "hidden");
}

/// A failed `host:` import leaves nothing behind for a later one to find;
/// a failed call has no result; a name the protocol reserves is refused.
#[test]
fn quickjs_failed_imports_and_calls_leave_nothing_behind() {
    let mut sandbox = boot();
    assert!(sandbox.run("import * as db from 'host:db'").is_err());
    assert!(
        sandbox
            .run("import * as db from 'host:db'; console.log(Object.keys(db).length)")
            .is_err(),
        "a second import of a module with no functions succeeded"
    );
    sandbox
        .run("function half(e) { Promise.reject(new Error('later')); return { ok: 1 } }")
        .unwrap();
    assert!(sandbox.call("half", "{}").is_err());
    assert_eq!(sandbox.take_result(), None);

    for name in ["", "a\nb"] {
        let err = SandboxBuilder::from_initrd(require_rootfs("quickjs"))
            .host_function(name, |_| Ok(String::new()))
            .boot()
            .err()
            .unwrap();
        assert!(
            matches!(err, Error::HostFunctionName { .. }),
            "{name:?}: {err}"
        );
    }
}

/// A call that fails before its function runs still finishes what it
/// started, so nothing is left to fail the next call; a name is looked up
/// whole, embedded NUL included; `import"x"` with no space is a module.
#[test]
fn quickjs_calls_are_self_contained() {
    let mut sandbox = boot();
    sandbox
        .run(
            "function greet() { return 'hi' }\n\
             Object.defineProperty(globalThis, 'notfn', { get() { Promise.reject(new Error('stray')); return 5 } })",
        )
        .unwrap();
    assert!(sandbox.call("notfn", "{}").is_err());
    let output = sandbox.drain_output();
    assert!(output.contains("stray"), "{output}");
    sandbox.run("console.log('next call is clean')").unwrap();
    assert_eq!(sandbox.drain_output().trim(), "next call is clean");

    assert!(sandbox.call("greet\0admin", "{}").is_err());
    assert_eq!(sandbox.call("greet", "{}").unwrap(), "\"hi\"");
    sandbox.drain_output();

    // Top-level await only parses in a module.
    sandbox
        .run("import\"qjs:std\";\nawait 0;\nconsole.log('module')")
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "module");
}

/// An awaited promise nothing can settle fails the call rather than spin
/// the guest; an exception a job throws while the call awaits fails it.
#[test]
fn quickjs_awaits_that_cannot_settle_or_throw_fail_the_call() {
    let mut sandbox = boot();
    let started = std::time::Instant::now();
    assert!(
        sandbox
            .run("export {}; await new Promise(() => {})")
            .is_err()
    );
    sandbox
        .run("function never() { return new Promise(() => {}) }\n\
              async function boom() { queueMicrotask(() => { throw new Error('in a job') }); await 0; return 1 }")
        .unwrap();
    assert!(sandbox.call("never", "").is_err());
    assert!(sandbox.call("boom", "").is_err());
    assert!(
        started.elapsed() < std::time::Duration::from_secs(10),
        "an unsettled await took {:?}",
        started.elapsed()
    );
    let output = sandbox.drain_output();
    assert!(output.contains("can never settle"), "{output}");
    assert!(output.contains("in a job"), "{output}");
    // The engine serves on.
    sandbox.run("console.log('ok')").unwrap();
}

/// An uncaught error ends the call and cancels the timers it started, as
/// it would end Node's process: nothing runs in, or fails, the next call,
/// and a timer that keeps throwing cannot hold the call.
#[test]
fn quickjs_an_uncaught_error_ends_what_the_call_started() {
    let mut sandbox = boot();
    assert!(
        sandbox
            .run(
                "import * as os from 'qjs:os';\n\
                 queueMicrotask(() => { throw 1 });\n\
                 os.setTimeout(() => { console.log('late ran') }, 0);"
            )
            .is_err()
    );
    let started = std::time::Instant::now();
    assert!(
        sandbox
            .run("import * as os from 'qjs:os'; os.setInterval(() => { throw new Error('again') }, 1)")
            .is_err()
    );
    assert!(started.elapsed() < std::time::Duration::from_secs(10));
    // A timer a queued job registers while the call is failing is
    // cancelled too: the next call does not wait on it.
    assert!(
        sandbox
            .run(
                "import * as os from 'qjs:os';\n\
                 Promise.resolve().then(() => os.setInterval(() => console.log('leaked'), 1));\n\
                 throw new Error('fail');"
            )
            .is_err()
    );
    sandbox.run("console.log('hi')").unwrap();
    let output = sandbox.drain_output();
    assert!(!output.contains("leaked"), "{output}");
    assert!(!output.contains("late ran"), "{output}");
    assert_eq!(output.matches("again").count(), 1, "{output}");
    assert!(output.trim_end().ends_with("hi"), "{output}");
}

/// A rejection nobody handles fails the call even when an earlier one was
/// handled late; a thenable a handler returns is awaited, as `await` does.
#[test]
fn quickjs_every_rejection_counts_and_thenables_are_awaited() {
    let mut sandbox = boot();
    assert!(matches!(
        sandbox.run(
            "const b = Promise.reject(1);\n\
             const a = Promise.reject(new Error('second one'));\n\
             b.catch(() => {});"
        ),
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(sandbox.drain_output().contains("second one"));
    // A rejection ends the call even while a timer would keep it alive.
    let started = std::time::Instant::now();
    assert!(
        sandbox
            .run(
                "import * as os from 'qjs:os';\n\
                 os.setInterval(() => {}, 5);\n\
                 Promise.reject(new Error('while ticking'));"
            )
            .is_err()
    );
    assert!(started.elapsed() < std::time::Duration::from_secs(10));
    assert!(sandbox.drain_output().contains("while ticking"));
    // The same while a handler awaits something a timer would settle.
    let started = std::time::Instant::now();
    sandbox
        .run(
            "import * as os from 'qjs:os';\n\
             globalThis.slow = () => { Promise.reject(new Error('meanwhile')); \
                return new Promise((r) => os.setTimeout(r, 60000)); };",
        )
        .unwrap();
    assert!(sandbox.call("slow", "").is_err());
    assert!(started.elapsed() < std::time::Duration::from_secs(10));
    assert!(sandbox.drain_output().contains("meanwhile"));
    sandbox
        .run("function thenable() { return { then(resolve) { resolve(42) } } }")
        .unwrap();
    assert_eq!(sandbox.call("thenable", "").unwrap(), "42");
}
