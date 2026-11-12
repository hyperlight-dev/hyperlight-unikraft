// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The php image: code and .php files, state kept across calls, errors
//! and exit() end the call with the status the CLI would exit with and
//! leave the engine serving, the capability surface (stdin, environment,
//! files, network, fibers), guest and host function calls, and a warm
//! snapshot's clones drawing their own random numbers.

mod common;

use std::path::PathBuf;

use common::{hluk_with_stdin_scratch, require_rootfs, run_http_get, temp_dir};
use hyperlight_unikraft::{Error, Exec, Mount, SandboxBuilder};

const SCRATCH_MB: usize = 128;

fn builder() -> SandboxBuilder {
    SandboxBuilder::from_initrd(require_rootfs("php")).scratch_mb(SCRATCH_MB)
}

fn boot() -> hyperlight_unikraft::AppSandbox {
    builder().boot().unwrap()
}

fn example(name: &str) -> Exec {
    Exec::File(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/php")
            .join(name),
    )
}

fn status_of(result: Result<(), Error>) -> Option<i32> {
    match result {
        Err(Error::CallFailed { status }) => Some(status),
        Ok(()) => Some(0),
        Err(_) => None,
    }
}

#[tokio::test]
async fn php_inline_code() {
    let mut sandbox = boot();
    sandbox
        .run(r#"echo "Hello from PHP ", 6 * 7, "\n";"#)
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "Hello from PHP 42\n");
}

#[tokio::test]
async fn php_exec_file() {
    let mut sandbox = boot();
    sandbox.run(example("hello.php")).await.unwrap();
    assert!(
        sandbox.drain_output().starts_with("Hello from PHP 8."),
        "hello.php did not run"
    );
}

/// Functions, classes and globals a call declares are there for the next
/// one, as in `php -a`.
#[tokio::test]
async fn php_state_is_kept_across_calls() {
    let mut sandbox = boot();
    sandbox
        .run("function greet($n) { return \"Hello, $n\"; } class Counter { static $n = 0; } $calls = 0;").await
        .unwrap();
    for name in ["a", "b"] {
        sandbox
            .run(format!(
                "$calls++; Counter::$n += 10; echo greet('{name}'), ' ', $calls, ' ', Counter::$n, \"\\n\";"
            )).await
            .unwrap();
    }
    assert_eq!(sandbox.drain_output(), "Hello, a 1 10\nHello, b 2 20\n");
}

/// Source that starts with markup, an open tag or HTML, after a `#!`
/// line, a byte order mark and blank lines, runs as a file: its text
/// outside the tags is output.  Other source is code.
#[tokio::test]
async fn php_markup_source_runs_as_a_file() {
    let mut sandbox = boot();
    sandbox
        .run("<?php declare(strict_types=1); $x = 6 * 7; ?>\n<p><?= $x ?></p>\n")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "<p>42</p>\n");
    sandbox
        .run("#!/usr/bin/env php\n<?php\necho __LINE__, \"\\n\";\n")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "3\n");
    sandbox
        .run("\u{feff}\n<!DOCTYPE html>\n<h1><?= 'page' ?></h1>")
        .await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "\u{feff}\n<!DOCTYPE html>\n<h1>page</h1>"
    );
    // A byte order mark before a #! line: both are text, as the CLI
    // prints them.
    sandbox
        .run("\u{feff}#!/usr/bin/env php\n<?php echo 1;")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "\u{feff}#!/usr/bin/env php\n1");
    // A # line is a comment in code, and a heredoc can start it.
    sandbox
        .run("#!/usr/bin/env php\necho 'code';")
        .await
        .unwrap();
    sandbox
        .run("<<<EOT\nheredoc\nEOT;\necho ' ok';")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "code ok");
}

/// An uncaught exception, an Error and a parse error end the call with
/// 255, the CLI's status, their message on standard error; a warning is
/// printed there and the call goes on.  The engine serves the next call
/// with what was defined before.
#[tokio::test]
async fn php_errors_fail_the_call_and_the_engine_serves_on() {
    let mut sandbox = boot();
    sandbox
        .run("function kept() { return 'kept'; }")
        .await
        .unwrap();
    for (code, message) in [
        (
            "echo 'out'; throw new RuntimeException('script boom');",
            "Uncaught RuntimeException: script boom",
        ),
        ("echo 1 +;", "PHP Parse error"),
        (
            "undefined_function();",
            "Call to undefined function undefined_function()",
        ),
    ] {
        assert_eq!(status_of(sandbox.run(code).await), Some(255), "{code}");
        let stderr = sandbox.drain_stderr();
        assert!(stderr.contains(message), "{code}: {stderr}");
        sandbox.drain_output();
    }
    sandbox
        .run("echo $undefined; @fopen('/nope', 'r'); fopen('/nope', 'r'); echo kept(), \"\\n\";")
        .await
        .unwrap();
    let stderr = sandbox.drain_stderr();
    assert!(
        stderr.contains("PHP Warning:  Undefined variable $undefined"),
        "{stderr}"
    );
    // A warning names its function, and @ silences it.
    assert_eq!(
        stderr
            .matches("fopen(/nope): Failed to open stream")
            .count(),
        1,
        "{stderr}"
    );
    assert_eq!(sandbox.drain_output().replace(&stderr, ""), "kept\n");
}

/// An exception handler set with set_exception_handler() takes the
/// uncaught exception, and the call succeeds, as a script handled that
/// way exits 0.
#[tokio::test]
async fn php_exception_handler_takes_the_uncaught_exception() {
    let mut sandbox = boot();
    sandbox
        .run(
            "set_exception_handler(function ($e) { echo 'handled: ', $e->getMessage(), \"\\n\"; });\n\
             throw new LogicException('boom');",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "handled: boom\n");
}

/// An exit() in the exception handler ends the call with its status, and
/// the request goes on, the handler and the state with it, whether the
/// handler was set by an earlier call or the same one, and from a file.
/// An exception the handler throws is reported.
#[tokio::test]
async fn php_exit_in_the_exception_handler_keeps_the_request() {
    let data = temp_dir("php-handler");
    std::fs::write(
        data.path().join("throw.php"),
        "<?php throw new Exception('file');\n",
    )
    .unwrap();
    let mut sandbox = builder()
        .mounts(vec![Mount::ro(data.path(), "/mnt/data")])
        .boot()
        .unwrap();
    let handler = "$kept = 1; set_exception_handler(function ($e) { echo 'handled ', $e->getMessage(), \"\\n\"; exit(4); });";
    sandbox.run(handler).await.unwrap();
    for exec in [
        Exec::Code("throw new Exception('a');".into()),
        Exec::Guest("/mnt/data/throw.php".into()),
        Exec::Code(format!("{handler} throw new Exception('b');")),
        Exec::Code("throw new Exception('c');".into()),
    ] {
        assert_eq!(status_of(sandbox.run(exec).await), Some(4));
    }
    sandbox.run("var_dump(isset($kept));").await.unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "handled a\nhandled file\nhandled b\nhandled c\nbool(true)\n"
    );
    sandbox
        .run(
            "set_exception_handler(function ($e) { throw new LogicException('in the handler'); });",
        )
        .await
        .unwrap();
    assert_eq!(
        status_of(sandbox.run("throw new Exception('d');").await),
        Some(255)
    );
    assert!(
        sandbox
            .drain_stderr()
            .contains("Uncaught LogicException: in the handler")
    );
    sandbox.drain_output();
    sandbox.run("var_dump(isset($kept));").await.unwrap();
    assert_eq!(sandbox.drain_output(), "bool(true)\n");
    // A parse error is the script's own: the handler never sees it.
    assert_eq!(status_of(sandbox.run("echo 1 +;").await), Some(255));
    assert!(sandbox.drain_stderr().contains("PHP Parse error"));
    sandbox.drain_output();
    // exit(0) in the handler of a guest function's exception: the call
    // succeeds, with no result.
    sandbox
        .run("function thrower() { throw new Exception('t'); } set_exception_handler(function ($e) { exit(0); });").await
        .unwrap();
    assert_eq!(sandbox.call("thrower", "").await.unwrap(), "");
}

/// An output callback that throws when the call's buffers are flushed
/// is reported, the output passed through, and the request goes on.
#[tokio::test]
async fn php_a_throwing_output_callback_is_reported() {
    let mut sandbox = boot();
    let result = sandbox
        .run("$kept = 1; ob_start(function ($b) { throw new Exception('ob'); }); echo 'hi';")
        .await;
    assert_eq!(status_of(result), Some(255));
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("Uncaught Exception: ob"), "{stderr}");
    assert!(!stderr.contains("ended the PHP request"), "{stderr}");
    assert!(sandbox.drain_output().replace(&stderr, "").contains("hi"));
    sandbox
        .run("echo 'ok ', isset($kept) ? 'kept' : 'new';")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "ok kept");
}

/// An error handler that throws at a warning raised as the source is
/// compiled fails the call with that exception, and the request goes on.
#[tokio::test]
async fn php_an_exception_while_compiling_keeps_the_request() {
    let mut sandbox = boot();
    sandbox
        .run("$kept = 1; set_error_handler(function ($n, $s) { throw new ErrorException($s); });")
        .await
        .unwrap();
    // Implicitly nullable: deprecated as it is compiled (PHP 8.4).
    let result = sandbox.run("function nullable(Foo $x = null) {}").await;
    assert_eq!(status_of(result), Some(255));
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("Uncaught ErrorException"), "{stderr}");
    assert!(!stderr.contains("ended the PHP request"), "{stderr}");
    sandbox.drain_output();
    sandbox
        .run("echo isset($kept) ? 'kept' : 'new';")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "kept");
}

/// An exception whose __toString() throws, from an output callback,
/// leaves nothing pending for the next call.
#[tokio::test]
async fn php_an_exception_thrown_while_reported_is_reported_too() {
    let mut sandbox = boot();
    let result = sandbox.run(
        "ob_start(function ($b) { throw new class('o') extends Exception { function __toString(): string { throw new Exception('inner'); } }; }); echo 'x';",
    ).await;
    assert_eq!(status_of(result), Some(255));
    sandbox.drain_stderr();
    sandbox.drain_output();
    sandbox.run("echo 'ok';").await.unwrap();
    assert_eq!(sandbox.drain_output(), "ok");
}

/// $_SERVER['REQUEST_TIME'] is the call's, not the first one's, nor the
/// snapshot's.
#[tokio::test]
async fn php_request_time_is_the_call_s() {
    let mut sandbox = boot();
    let time = async |sandbox: &mut hyperlight_unikraft::AppSandbox| {
        sandbox
            .run("echo $_SERVER['REQUEST_TIME_FLOAT'];")
            .await
            .unwrap();
        sandbox.drain_output().parse::<f64>().unwrap()
    };
    let first = time(&mut sandbox).await;
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = time(&mut sandbox).await;
    assert!(second - first >= 1.0, "{first} then {second}");
    sandbox
        .run("echo abs($_SERVER['REQUEST_TIME'] - time()) <= 1 ? 'now' : 'stale';")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "now");
}

/// exit()'s status is a process's, its low eight bits, as the CLI exits.
#[tokio::test]
async fn php_exit_status_is_eight_bits() {
    let mut sandbox = boot();
    sandbox.run("exit(256);").await.unwrap();
    assert_eq!(status_of(sandbox.run("exit(-1);").await), Some(255));
    assert_eq!(status_of(sandbox.run("exit(258);").await), Some(2));
}

/// The driver's own JSON work leaves json_last_error() as code left it.
#[tokio::test]
async fn php_json_last_error_is_the_code_s() {
    let mut sandbox = builder()
        .host_function("nothing", |_| Ok(String::new()))
        .boot()
        .unwrap();
    sandbox
        .run("function id($x) { return $x; } function sees() { return json_last_error(); } function bad() { json_decode('['); return 1; }\n\
              json_decode('{bad'); Hyperlight\\call('nothing'); echo json_last_error(), ' ';").await
        .unwrap();
    assert!(sandbox.call("id", "{bad").await.is_err());
    assert_eq!(sandbox.call("id", "[1]").await.unwrap(), "[1]");
    // The function sees code's last error, not the input's decoding.
    assert_eq!(sandbox.call("sees", "[1]").await.unwrap(), "4");
    sandbox
        .run("echo json_last_error(), ' '; json_decode('[]');")
        .await
        .unwrap();
    // What the function itself left stays.
    sandbox.call("bad", "").await.unwrap();
    sandbox.run("echo json_last_error();").await.unwrap();
    let stderr = sandbox.drain_stderr();
    assert_eq!(sandbox.drain_output().replace(&stderr, ""), "4 4 4");
}

/// exit() ends the call with its status, from a nested function too; a
/// string argument is printed and the status is 0.  What the call
/// declared stays.
#[tokio::test]
async fn php_exit_ends_the_call() {
    let mut sandbox = boot();
    let result = sandbox
        .run("$kept = 41; function quit() { exit(3); } echo 'before'; quit(); echo 'after';")
        .await;
    assert_eq!(status_of(result), Some(3));
    sandbox.run("exit('bye');").await.unwrap();
    sandbox.run("echo $kept + 1;").await.unwrap();
    assert_eq!(sandbox.drain_output(), "beforebye42");
    // exit() in a guest function fails the call with its status.
    assert!(matches!(
        sandbox.call("quit", "").await,
        Err(Error::CallFailed { status: 3 })
    ));
}

/// A script past the memory limit fails with PHP's fatal error, not by
/// running the guest out of memory; the limit is two thirds of what
/// the guest had free at boot, and OPcache, whose cache would grow
/// outside it, is off.
#[tokio::test]
async fn php_memory_limit_is_below_the_guest_memory() {
    let mut sandbox = boot();
    sandbox
        .run("echo (int) ini_get('memory_limit') < 128 ? 'under' : 'over', ' ', ini_get('opcache.enable');").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "under 0");
    // ini_set() cannot lift it past the guest either (max_memory_limit).
    sandbox
        .run("@ini_set('memory_limit', '-1'); echo (int) ini_get('memory_limit') > 0 ? 'capped' : 'unlimited';").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "capped");
    // One allocation past memory_limit and under PHP 8.5's hard limit goes
    // through, and the call fails with PHP's error all the same.
    let result = sandbox.run(
        "$n = (int) (((int) ini_get('memory_limit') << 20) * 1.05); $s = str_repeat('x', $n); echo 'after';",
    ).await;
    assert_eq!(status_of(result), Some(255));
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("Allowed memory size"), "{stderr}");
    sandbox.drain_output();
    let result = sandbox
        .run("$a = []; while (true) { $a[] = str_repeat('x', 1 << 20); }")
        .await;
    assert_eq!(status_of(result), Some(255));
    assert!(sandbox.drain_stderr().contains("Allowed memory size"));
    sandbox.drain_output();
    sandbox.run("echo 'still here';").await.unwrap();
    assert_eq!(sandbox.drain_output(), "still here");
}

/// A fatal error (memory exhausted, in a function and under @; a function
/// declared twice) ends the request: its shutdown functions run, and the
/// next call starts a new one, with its memory back, error reporting as
/// configured, and nothing earlier calls declared.  The same from a file in
/// the guest.
#[tokio::test]
async fn php_fatal_error_starts_a_new_request() {
    let data = temp_dir("php-fatal");
    std::fs::write(
        data.path().join("fatal.php"),
        "<?php function grow() { $a = []; while (true) { $a[] = str_repeat('x', 1 << 20); } } @grow();\n",
    )
    .unwrap();
    let mut sandbox = builder()
        .mounts(vec![Mount::ro(data.path(), "/mnt/data")])
        .boot()
        .unwrap();
    for (i, exec) in [
        Exec::Code(
            "function grow() { $a = []; while (true) { $a[] = str_repeat('x', 1 << 20); } }\n\
             register_shutdown_function(function () { echo \"shutdown ran\\n\"; });\n\
             @grow();"
                .into(),
        ),
        Exec::Guest("/mnt/data/fatal.php".into()),
        Exec::Code("function grow() {} function grow() {}".into()),
    ]
    .into_iter()
    .enumerate()
    {
        sandbox.run("$kept = 1;").await.unwrap();
        assert_eq!(
            status_of(sandbox.run(exec.clone()).await),
            Some(255),
            "{exec:?}"
        );
        let stderr = sandbox.drain_stderr();
        assert!(
            stderr.contains("Allowed memory size") || stderr.contains("Cannot redeclare"),
            "{stderr}"
        );
        assert!(stderr.contains("ended the PHP request"), "{stderr}");
        let output = sandbox.drain_output();
        if i == 0 {
            assert!(output.contains("shutdown ran"), "{output}");
        }
        sandbox
            .run("var_dump(isset($kept), function_exists('grow')); echo strlen(str_repeat('x', 40 << 20)), \"\\n\"; echo $undefined;").await
            .unwrap();
        let stderr = sandbox.drain_stderr();
        assert!(stderr.contains("Undefined variable $undefined"), "{stderr}");
        assert_eq!(
            sandbox.drain_output().replace(&stderr, ""),
            "bool(false)\nbool(false)\n41943040\n"
        );
    }
}

/// A snapshot keeps the engine's state, and two restores of one snapshot
/// draw different numbers from mt_rand(), which is reseeded every call;
/// within a call, mt_srand() makes the draws repeatable.
#[tokio::test]
async fn php_snapshot_clones_draw_their_own_random_numbers() {
    let snap_dir = temp_dir("php-snap");
    let mut sandbox = boot();
    sandbox
        .run("$seen = 'kept'; mt_rand(); function seeded() { mt_srand(42); return mt_rand(); }")
        .await
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut draws = Vec::new();
    for _ in 0..2 {
        let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone
            .run("echo $seen, ' ', mt_rand(), ' ', rand(), ' ', seeded();")
            .await
            .unwrap();
        let output = clone.drain_output();
        assert!(output.starts_with("kept "), "{output}");
        draws.push(output);
    }
    assert_ne!(draws[0], draws[1], "two clones drew the same numbers");
    let seeded = |s: &str| s.rsplit(' ').next().unwrap().to_string();
    assert_eq!(seeded(&draws[0]), seeded(&draws[1]));

    // A destructor that runs during the next call's setup (a value left
    // in $_SERVER, freed when the driver rewrites REQUEST_TIME) draws
    // from the reseeded generator, not the snapshot's: the reseed comes
    // before that setup.
    let dtor_dir = temp_dir("php-snap-dtor");
    let mut sandbox = boot();
    sandbox
        .run(
            "class R { function __destruct() { echo 'dtor ', mt_rand(), \"\\n\"; } }\n\
             $_SERVER['REQUEST_TIME'] = new R(); mt_rand();",
        )
        .await
        .unwrap();
    sandbox.snapshot_to(&dtor_dir).unwrap();
    let mut dtor_draws = Vec::new();
    for _ in 0..2 {
        let mut clone = SandboxBuilder::from_snapshot_dir(&dtor_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone.run("echo 'ok\\n';").await.unwrap();
        dtor_draws.push(clone.drain_output());
    }
    assert!(dtor_draws[0].starts_with("dtor "), "{dtor_draws:?}");
    assert_ne!(
        dtor_draws[0], dtor_draws[1],
        "a destructor drew the snapshot's numbers in both clones"
    );

    // OpenSSL's generator, used before the snapshot, too.
    let ssl_dir = temp_dir("php-snap-ssl");
    let mut sandbox = boot();
    sandbox
        .run("openssl_random_pseudo_bytes(16);")
        .await
        .unwrap();
    sandbox.snapshot_to(&ssl_dir).unwrap();
    let mut ssl = Vec::new();
    for _ in 0..2 {
        let mut clone = SandboxBuilder::from_snapshot_dir(&ssl_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone
            .run("echo bin2hex(openssl_random_pseudo_bytes(16));")
            .await
            .unwrap();
        ssl.push(clone.drain_output());
    }
    assert_ne!(ssl[0], ssl[1], "two clones drew the same OpenSSL bytes");
    // And SQLite's.
    let sql_dir = temp_dir("php-snap-sqlite");
    let mut sandbox = boot();
    sandbox
        .run("$db = new PDO('sqlite::memory:'); $db->query('select random()');")
        .await
        .unwrap();
    sandbox.snapshot_to(&sql_dir).unwrap();
    let mut sql = Vec::new();
    for _ in 0..2 {
        let mut clone = SandboxBuilder::from_snapshot_dir(&sql_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone
            .run("echo $db->query('select random()')->fetchColumn();")
            .await
            .unwrap();
        sql.push(clone.drain_output());
    }
    assert_ne!(sql[0], sql[1], "two clones drew the same SQLite numbers");
}

/// A seed code gives mt_srand() stays for the calls after, and for the
/// clones of a snapshot taken after it; mt_srand() without one hands the
/// generator back to the driver.
#[tokio::test]
async fn php_a_seed_code_chose_stays() {
    let snap_dir = temp_dir("php-seed");
    let mut sandbox = boot();
    sandbox
        .run("mt_srand(42); echo mt_rand(), ' ';")
        .await
        .unwrap();
    sandbox.run("echo mt_rand(), ' ';").await.unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();
    sandbox.run("echo mt_rand();").await.unwrap();
    let seeded = sandbox.drain_output();
    let mut expected = boot();
    expected
        .run("mt_srand(42); echo mt_rand(), ' ', mt_rand(), ' ', mt_rand();")
        .await
        .unwrap();
    assert_eq!(seeded, expected.drain_output());
    let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    clone.run("echo mt_rand();").await.unwrap();
    assert_eq!(clone.drain_output(), seeded.rsplit(' ').next().unwrap());

    let mut draws = Vec::new();
    for _ in 0..2 {
        let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone.run("mt_srand();").await.unwrap();
        clone.run("echo mt_rand();").await.unwrap();
        draws.push(clone.drain_output());
    }
    assert_ne!(draws[0], draws[1], "two clones drew the same numbers");
}

#[test]
fn php_stdin_piped() {
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/php/stdin_echo.php");
    let output = hluk_with_stdin_scratch(
        &require_rootfs("php"),
        &script,
        b"hello from host\nline two\n",
        SCRATCH_MB as u32,
    );
    assert!(
        output.contains("lines=2"),
        "expected 2 lines, got: {output:?}"
    );
    assert!(output.contains("echo: hello from host"), "{output:?}");
    assert!(output.contains("stdin-done"), "{output:?}");
}

/// The host's variables reach getenv(), $_ENV and $_SERVER, and a change
/// between calls is seen by the next one.
#[tokio::test]
async fn php_env_vars() {
    let mut sandbox = boot();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox.run(example("env_vars.php")).await.unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "MY_VAR=hello_world\nDEBUG=1\nGREETING=hi there\n"
    );
    sandbox
        .run("echo $_SERVER['DEBUG'], $_ENV['DEBUG'] ?? '-', \"\\n\";")
        .await
        .unwrap();
    sandbox.set_env_vars(&[("DEBUG", "2")]);
    sandbox
        .run("echo getenv('DEBUG'), $_SERVER['DEBUG'], $_ENV['DEBUG'] ?? '-', filter_input(INPUT_ENV, 'DEBUG'), \"\\n\";").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "11\n2222\n");
}

/// putenv() over a host variable, again, and the request it ends with a
/// fatal error putting the old value back: the host's value is there, and
/// whole, for the calls after.
#[tokio::test]
async fn php_putenv_over_a_host_variable() {
    let mut sandbox = boot();
    sandbox.set_env_vars(&[("GREETING", "from the host")]);
    sandbox
        .run("putenv('GREETING=a'); putenv('GREETING=b'); echo getenv('GREETING'), ' ';")
        .await
        .unwrap();
    sandbox.run("echo getenv('GREETING'), ' ';").await.unwrap();
    assert_eq!(
        status_of(sandbox.run("function twice() {} function twice() {}").await),
        Some(255)
    );
    sandbox.drain_stderr();
    sandbox.drain_output();
    sandbox.run("echo getenv('GREETING');").await.unwrap();
    sandbox.set_env_vars(&[("GREETING", "changed")]);
    sandbox.run("echo ' ', getenv('GREETING');").await.unwrap();
    assert_eq!(sandbox.drain_output(), "from the host changed");
}

#[tokio::test]
async fn php_fs() {
    let mount_dir = temp_dir("php-fs");
    let mut sandbox = builder()
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox.run(example("fs_ops.php")).await.unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("read-back: hello from php"), "{output:?}");
    assert!(output.contains("line-count: 3"), "{output:?}");
    assert!(output.contains("listing: x"), "{output:?}");
    assert!(output.contains("fs-ops-done"), "{output:?}");
    // The write landed on the host side of the mount.
    assert_eq!(
        std::fs::read_to_string(mount_dir.path().join("php_fs.txt")).unwrap(),
        "hello from php"
    );
}

/// PHP has no threads: fibers interleave on one, and a generator runs
/// lazily.
#[tokio::test]
async fn php_fibers() {
    let mut sandbox = boot();
    sandbox.run(example("fibers.php")).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("order: a1 b1 c1 a2 b2 c2 a3 b3 c3"),
        "{output:?}"
    );
    assert!(output.contains("sums: 14,14,14"), "{output:?}");
    assert!(output.contains("sum-of-squares: 140"), "{output:?}");
    assert!(output.contains("fibers-done"), "{output:?}");
}

/// A fiber suspended in one call is resumed by the next.
#[tokio::test]
async fn php_fiber_carries_over_between_calls() {
    let mut sandbox = boot();
    sandbox
        .run("$f = new Fiber(function () { $x = Fiber::suspend('first'); echo \"got $x\\n\"; }); echo $f->start(), \"\\n\";").await
        .unwrap();
    sandbox
        .run("$f->resume('second'); var_dump($f->isTerminated());")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "first\ngot second\nbool(true)\n");
}

/// A guest that ends its request again and again keeps serving: each
/// request's standard streams are closed with it.
#[tokio::test]
async fn php_many_request_ends_leak_nothing() {
    let mut sandbox = boot();
    for _ in 0..400 {
        assert_eq!(
            status_of(sandbox.run("function twice() {} function twice() {}").await),
            Some(255)
        );
    }
    sandbox.drain_output();
    sandbox
        .run("fwrite(STDOUT, 'streams '); echo count(get_resources('stream'));")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "streams 3");
}

/// The driver replaces $argv each call: a destructor of the value code
/// left there that throws fails that call before its code runs, as an
/// uncaught exception, and the request goes on.
#[tokio::test]
async fn php_a_replaced_globals_destructor_does_not_end_the_request() {
    let mut sandbox = boot();
    sandbox
        .run("$kept = 1; $argv = new class { function __destruct() { throw new Exception('replaced'); } };").await
        .unwrap();
    assert_eq!(status_of(sandbox.run("echo 'ran';").await), Some(255));
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("Uncaught Exception: replaced"), "{stderr}");
    assert!(!stderr.contains("ended the PHP request"), "{stderr}");
    assert!(!sandbox.drain_output().contains("ran"));
    sandbox
        .run("echo isset($kept) ? 'kept' : 'new';")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "kept");
    // With an exception handler set too: it sees the exception, and the
    // call still does not run.
    sandbox
        .run("set_exception_handler(function ($e) { echo 'handled ', $e->getMessage(); }); $_SERVER['REQUEST_TIME'] = new class { function __destruct() { throw new Exception('setup'); } };").await
        .unwrap();
    assert_eq!(status_of(sandbox.run("echo 'ran';").await), Some(255));
    assert_eq!(sandbox.drain_output(), "handled setup");
}

/// Deep recursion is PHP's catchable Error, within the guest's stack: PHP
/// reads the stack's size from /proc/self/maps.
#[tokio::test]
async fn php_deep_recursion_is_an_error() {
    let mut sandbox = boot();
    sandbox
        .run(
            "function f($n) { return array_map(fn ($x) => f($n + 1), [1]); }\n\
             try { f(0); } catch (Error $e) { echo str_contains($e->getMessage(), 'Maximum call stack size') ? 'stopped' : $e->getMessage(); }",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "stopped");
    sandbox.run("echo 'still here';").await.unwrap();
    assert_eq!(sandbox.drain_output(), "still here");
}

/// A call that sleeps parks the guest; the call still returns.
#[tokio::test]
async fn php_sleep_returns() {
    let mut sandbox = boot();
    let start = std::time::Instant::now();
    sandbox
        .run("$t = microtime(true); usleep(300000); echo microtime(true) - $t >= 0.3 ? 'slept' : 'early';").await
        .unwrap();
    assert!(start.elapsed() >= std::time::Duration::from_millis(300));
    assert_eq!(sandbox.drain_output(), "slept");
}

#[tokio::test]
async fn php_http_get() {
    let output = run_http_get(builder, example("http_get.php")).await;
    assert!(output.contains("Status: 200"), "{output}");
}

/// The extensions the image is built with load, and work: SQLite through
/// PDO and SQLite3, mbstring, DOM, sodium, OpenSSL, zlib and bcmath.
#[tokio::test]
async fn php_extensions() {
    let mut sandbox = boot();
    sandbox
        .run(
            "$db = new PDO('sqlite::memory:');\n\
             $db->exec('create table t (x)'); $db->exec('insert into t values (1), (2), (3)');\n\
             echo $db->query('select sum(x) from t')->fetchColumn(), ' ';\n\
             echo (new SQLite3(':memory:'))->querySingle('select 6 * 7'), ' ';\n\
             echo mb_strtoupper('héllo'), ' ';\n\
             $doc = new DOMDocument(); $doc->loadXML('<a><b>dom</b></a>'); echo $doc->getElementsByTagName('b')->item(0)->textContent, ' ';\n\
             echo strlen(sodium_crypto_secretbox_keygen()), ' ';\n\
             echo strlen(openssl_random_pseudo_bytes(16)), ' ';\n\
             echo gzuncompress(gzcompress('zlib')), ' ';\n\
             echo bcadd('0.1', '0.2', 1), ' ';\n\
             var_dump(extension_loaded('hyperlight'));",
        ).await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "6 42 HÉLLO dom 32 16 zlib 0.3 bool(true)\n"
    );
}

/// A file in the guest: `$argv` and `$argc` are its command line, as the
/// CLI sets them; a file that is not there fails with the CLI's message
/// and status.
#[tokio::test]
async fn php_guest_exec_file_with_args() {
    let data = temp_dir("php-data");
    std::fs::write(data.path().join("input.txt"), "one two three\n").unwrap();
    std::fs::write(
        data.path().join("main.php"),
        "<?php\n$words = str_word_count(file_get_contents('/mnt/data/input.txt'));\n\
         echo implode(',', array_slice($argv, 1)), ' ', $argc, ' ', $_SERVER['argv'][0], ' ', __FILE__, ' ', getenv('GREETING'), ' ', $words, \"\\n\";\n",
    )
    .unwrap();
    let mut sandbox = builder()
        .mounts(vec![Mount::rw(data.path(), "/mnt/data")])
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[("GREETING", "hi")]);
    sandbox
        .run(Exec::Guest("/mnt/data/main.php x y".into()))
        .await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "x,y 3 /mnt/data/main.php /mnt/data/main.php hi 3\n"
    );
    // The image's entry point, /entrypoint.php, is the default, for white
    // space too; this image has none.
    // A missing file after one with arguments, then a request ended by a
    // fatal error: the new request builds $argv from the last command line.
    assert_eq!(
        status_of(
            sandbox
                .run(Exec::Guest("/mnt/data/missing.php".into()))
                .await
        ),
        Some(1)
    );
    assert_eq!(
        status_of(sandbox.run("function twice() {} function twice() {}").await),
        Some(255)
    );
    sandbox.run("echo 'alive';").await.unwrap();
    sandbox.drain_stderr();
    assert!(sandbox.drain_output().ends_with("alive"));
    for command in ["", "  "] {
        assert_eq!(
            status_of(sandbox.run(Exec::Guest(command.into())).await),
            Some(0)
        );
        assert!(
            sandbox
                .drain_stderr()
                .contains("hl: no /entrypoint.php in rootfs; nothing to run")
        );
    }
    // A command line with arguments, then an empty one (nothing to run),
    // then a guest function's fatal error: the new request starts.
    sandbox
        .run("function grow() { $a = []; while (true) { $a[] = str_repeat('x', 1 << 20); } }")
        .await
        .unwrap();
    sandbox
        .run(Exec::Guest("/mnt/data/main.php x y".into()))
        .await
        .unwrap();
    sandbox.run(Exec::Guest(String::new())).await.unwrap();
    assert!(matches!(
        sandbox.call("grow", "").await,
        Err(Error::CallFailed { status: 255 })
    ));
    sandbox.run("echo 'restarted';").await.unwrap();
    sandbox.drain_stderr();
    assert!(sandbox.drain_output().ends_with("restarted"));
    // A script that failed to compile is not taken as included: fixed, it
    // can be required.
    std::fs::write(
        data.path().join("lib.php"),
        "<?php function lib_fn() { return 1 +; }\n",
    )
    .unwrap();
    assert_eq!(
        status_of(sandbox.run(Exec::Guest("/mnt/data/lib.php".into())).await),
        Some(255)
    );
    std::fs::write(
        data.path().join("lib.php"),
        "<?php function lib_fn() { return 'fixed'; }\n",
    )
    .unwrap();
    sandbox.drain_output();
    sandbox
        .run("require_once '/mnt/data/lib.php'; echo lib_fn();")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "fixed");
}

/// A guest function, named as a PHP callable: a function, a namespaced
/// one or a static method, with JSON in and out.
#[tokio::test]
async fn php_calls_a_guest_function() {
    let mut sandbox = boot();
    sandbox
        .run(
            "function greet(array $event): array { static $n = 0; return ['message' => 'Hello, ' . $event['name'], 'n' => ++$n]; }\n\
             class Handlers { public static function double(int $x): int { return $x * 2; } }\n\
             function count_args(...$args) { return count($args); }\n\
             function nothing(): void {}\n\
             function boom() { throw new DomainException('handler boom'); }\n\
             function not_json() { return NAN; }\n\
             function big(int $n) { return str_repeat('x', $n); }",
        ).await
        .unwrap();
    sandbox
        .run("namespace App; function shout(string $s): string { return mb_strtoupper($s); }")
        .await
        .unwrap();
    assert_eq!(
        sandbox.call("greet", r#"{"name":"World"}"#).await.unwrap(),
        r#"{"message":"Hello, World","n":1}"#
    );
    assert_eq!(
        sandbox.call("greet", r#"{"name":"again"}"#).await.unwrap(),
        r#"{"message":"Hello, again","n":2}"#
    );
    assert_eq!(sandbox.call("Handlers::double", "21").await.unwrap(), "42");
    // A __callStatic() handler, many times over: its trampolines are freed.
    sandbox
        .run("class Magic { static function __callStatic($n, $a) { return $n; } }")
        .await
        .unwrap();
    sandbox.run("$before = memory_get_usage();").await.unwrap();
    for _ in 0..500 {
        assert_eq!(
            sandbox.call("Magic::anything", "1").await.unwrap(),
            r#""anything""#
        );
    }
    sandbox
        .run("echo memory_get_usage() - $before < 16384 ? 'flat' : memory_get_usage() - $before;")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "flat");
    assert_eq!(
        sandbox.call(r"App\shout", r#""hé/llo""#).await.unwrap(),
        r#""HÉ/LLO""#
    );
    // Empty input is no argument; null and void are no result.
    assert_eq!(sandbox.call("count_args", "").await.unwrap(), "0");
    assert_eq!(sandbox.call("count_args", "null").await.unwrap(), "1");
    assert_eq!(sandbox.call("nothing", "").await.unwrap(), "");
    // A list stays an array, an object an object, 1.0 a float.
    sandbox.run("function id($x) { return $x; }").await.unwrap();
    assert_eq!(
        sandbox
            .call("id", r#"{"a":[1,2.0,"x"],"b":{"c":null}}"#)
            .await
            .unwrap(),
        r#"{"a":[1,2.0,"x"],"b":{"c":null}}"#
    );

    // A handler that throws fails the call with the CLI's status; one that
    // is not there, input that is not JSON, a result that cannot be JSON
    // or that is too large fail it too.  The guest serves on.
    assert!(matches!(
        sandbox.call("boom", "").await,
        Err(Error::CallFailed { status: 255 })
    ));
    assert!(
        sandbox
            .drain_stderr()
            .contains("Uncaught DomainException: handler boom")
    );
    assert!(matches!(
        sandbox.call("missing", "{}").await,
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(sandbox.drain_stderr().contains("no function missing"));
    // An autoloader that throws as a class name is looked up fails the
    // call alone.
    sandbox
        .run("spl_autoload_register(function ($c) { throw new RuntimeException(\"no $c\"); });")
        .await
        .unwrap();
    assert!(sandbox.call("Missing::run", "").await.is_err());
    assert!(sandbox.drain_stderr().contains("no Missing"));
    assert!(matches!(
        sandbox.call("greet", "{not json").await,
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(
        sandbox
            .drain_stderr()
            .contains("the input to greet is not JSON: Syntax error")
    );
    assert!(matches!(
        sandbox.call("not_json", "").await,
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(
        sandbox
            .drain_stderr()
            .contains("Inf and NaN cannot be JSON encoded")
    );
    assert_eq!(sandbox.call("big", "10").await.unwrap().len(), 12);
    for n in ["1100000", "71000"] {
        assert!(
            sandbox.call("big", n).await.is_err(),
            "a {n}-byte result did not fail the call"
        );
    }
    // A type error is an uncaught TypeError.
    assert!(matches!(
        sandbox.call("Handlers::double", r#""x""#).await,
        Err(Error::CallFailed { status: 255 })
    ));
    assert_eq!(
        sandbox.call("greet", r#"{"name":"last"}"#).await.unwrap(),
        r#"{"message":"Hello, last","n":3}"#
    );
}

/// An exception that escapes a guest function fails the call: uncaught,
/// with 255; with an exception handler set, which sees it, as the
/// function never returned.  One that jsonSerialize() throws for its
/// result, or that its result's destructor throws after it, fails it
/// too, and the guest serves on.
#[tokio::test]
async fn php_guest_function_exceptions_fail_the_call() {
    let mut sandbox = boot();
    sandbox
        .run(
            "function thrower() { throw new Exception('escaped'); }\n\
             class Bad implements JsonSerializable { function jsonSerialize(): mixed { throw new Exception('serialize'); } }\n\
             function bad_result() { return new Bad; }\n\
             function own_handler() { set_exception_handler(function ($e) { echo 'own: ', $e->getMessage(), \"\\n\"; }); throw new Exception('mine'); }",
        ).await
        .unwrap();
    for (function, message) in [("thrower", "escaped"), ("bad_result", "serialize")] {
        assert!(
            matches!(
                sandbox.call(function, "").await,
                Err(Error::CallFailed { status: 255 })
            ),
            "{function}"
        );
        let stderr = sandbox.drain_stderr();
        assert!(stderr.contains(message), "{stderr}");
        sandbox.drain_output();
    }
    // A handler the function sets itself sees its exception; the call
    // fails all the same.
    assert!(sandbox.call("own_handler", "").await.is_err());
    assert!(sandbox.drain_output().contains("own: mine"));
    sandbox
        .run("set_exception_handler(function ($e) { echo 'handled: ', $e->getMessage(), \"\\n\"; });").await
        .unwrap();
    for (function, message) in [("thrower", "escaped"), ("bad_result", "serialize")] {
        assert!(sandbox.call(function, "").await.is_err(), "{function}");
        let stderr = sandbox.drain_stderr();
        let output = sandbox.drain_output().replace(&stderr, "");
        assert_eq!(output, format!("handled: {message}\n"), "{function}");
    }
    sandbox
        .run("throw new Exception('in code');")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "handled: in code\n");
    // A handler that sets itself again leaves the handler stack as it was.
    sandbox
        .run("set_exception_handler($h = function ($e) use (&$h) { set_exception_handler($h); });")
        .await
        .unwrap();
    for _ in 0..3 {
        sandbox.run("throw new Exception('again');").await.unwrap();
    }
    sandbox
        .run("restore_exception_handler(); throw new Exception('restored');")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "handled: restored\n");
    // One that clears itself stays cleared.
    sandbox
        .run("set_exception_handler(function ($e) { echo 'once'; set_exception_handler(null); });")
        .await
        .unwrap();
    sandbox.run("throw new Exception('first');").await.unwrap();
    assert!(sandbox.call("thrower", "").await.is_err());
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("Uncaught Exception: escaped"), "{stderr}");
    assert_eq!(sandbox.drain_output().replace(&stderr, ""), "once");
    // The exceptions that escaped those calls left the cycle collector on.
    sandbox
        .run(
            "class Node { public $other; }\n\
             for ($i = 0; $i < 100; $i++) { $a = new Node; $b = new Node; $a->other = $b; $b->other = $a; }\n\
             unset($a, $b);\n\
             echo gc_status()['protected'] ? 'protected ' : '', gc_collect_cycles();",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "200");
    // A destructor of the result that throws, after the call and with no
    // handler to take it, fails the call; the request goes on.
    let mut sandbox = boot();
    sandbox
        .run(
            "function thrower() {}\n\
             class Dying { public $v = 1; function __destruct() { throw new Exception('destructor'); } }\n\
             function dying_result() { return new Dying; }",
        ).await
        .unwrap();
    assert!(matches!(
        sandbox.call("dying_result", "").await,
        Err(Error::CallFailed { status: 255 })
    ));
    let stderr = sandbox.drain_stderr();
    assert!(stderr.contains("destructor"), "{stderr}");
    sandbox.drain_output();
    sandbox
        .run("echo function_exists('thrower') ? 'kept' : 'new';")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "kept");
}

/// The embedder's functions through Hyperlight\call; a host error is a
/// Hyperlight\HostException with the whole message.
#[tokio::test]
async fn php_calls_host_functions() {
    let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let seen_by_host = seen.clone();
    let mut sandbox = builder()
        .host_function("math.add", |args| {
            let v: Vec<f64> = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok(v.iter().sum::<f64>().to_string())
        })
        .host_function("math.fail", |_| Err("no can do".to_string()))
        .host_function("math.verbose", |_| Err("x".repeat(1000)))
        .host_function("echo", move |args| {
            seen_by_host.lock().unwrap().push(args.to_string());
            Ok(args.to_string())
        })
        .host_function("nothing", |_| Ok(String::new()))
        .host_function("not_json", |_| Ok("{".to_string()))
        .boot()
        .unwrap();
    sandbox
        .run(
            "use function Hyperlight\\call;\n\
             echo call('math.add', 2, 3), ' ', var_export(call('echo', 'é/x', 1.0, ['k' => [true, null]]), true), \"\\n\";\n\
             var_dump(call('nothing'));\n\
             try { call('math.fail'); } catch (Hyperlight\\HostException $e) { echo 'caught ', $e->getMessage(), \"\\n\"; }\n\
             try { call('math.verbose'); } catch (RuntimeException $e) { echo strlen($e->getMessage()), \"\\n\"; }\n\
             try { call('not_json'); } catch (JsonException $e) { echo 'reply ', $e->getMessage(), \"\\n\"; }\n\
             try { call('echo', NAN); } catch (JsonException $e) { echo 'args ', $e->getMessage(), \"\\n\"; }\n\
             class Bad implements JsonSerializable { function jsonSerialize(): mixed { throw new LogicException('unserializable'); } }\n\
             try { call('echo', new Bad); } catch (LogicException $e) { echo $e->getMessage(), \"\\n\"; }\n\
             try { call(''); } catch (ValueError $e) { echo 'empty name', \"\\n\"; }\n\
             try { call('missing'); } catch (Hyperlight\\HostException $e) { echo 'missing', \"\\n\"; }\n\
             try { call('echo', a: 1); } catch (Error $e) { echo 'named', \"\\n\"; }",
        ).await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "5 array (\n  0 => 'é/x',\n  1 => 1.0,\n  2 => \n  array (\n    'k' => \n    array (\n      0 => true,\n      1 => NULL,\n    ),\n  ),\n)\nNULL\ncaught no can do\n1000\nreply Syntax error\n\
         args echo's arguments cannot be JSON: Inf and NaN cannot be JSON encoded\nunserializable\nempty name\nmissing\nnamed\n"
    );
    // Text as it is, and 1.0 a float; NAN never reached the host.
    assert_eq!(*seen.lock().unwrap(), [r#"["é/x",1.0,{"k":[true,null]}]"#]);
    // A guest function can call the host too.
    sandbox
        .run("function total(array $event) { return Hyperlight\\call('math.add', ...$event['values']); }").await
        .unwrap();
    assert_eq!(
        sandbox
            .call("total", r#"{"values":[1,2,3]}"#)
            .await
            .unwrap(),
        "6"
    );
}

/// `examples/php/handler.php`: loaded once, called twice, its state
/// carried over.
#[tokio::test]
async fn php_handler_example() {
    let mut sandbox = boot();
    sandbox.run(example("handler.php")).await.unwrap();
    assert_eq!(
        sandbox
            .call("handler", r#"{"name":"World"}"#)
            .await
            .unwrap(),
        r#"{"greeting":"Hello, World!","calls":1}"#
    );
    assert_eq!(
        sandbox
            .call("handler", r#"{"name":"again"}"#)
            .await
            .unwrap(),
        r#"{"greeting":"Hello, again!","calls":2}"#
    );
}
