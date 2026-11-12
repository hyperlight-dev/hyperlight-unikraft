// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The java image: Java source run by JShell in the guest's JVM, state
//! kept across calls, a declared main run, errors fail the call and leave
//! the runtime serving, guest and host function calls with JSON, and
//! `System.exit()` ending the JVM for a fresh one.

mod common;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use common::{hluk_with_stdin_scratch, require_rootfs, run_http_get, temp_dir};
use hyperlight_unikraft::{Error, Exec, Mount, SandboxBuilder};

const SCRATCH_MB: usize = 512;

fn builder() -> SandboxBuilder {
    SandboxBuilder::from_initrd(require_rootfs("java")).scratch_mb(SCRATCH_MB)
}

fn boot() -> hyperlight_unikraft::AppSandbox {
    builder().boot().unwrap()
}

fn example(name: &str) -> Exec {
    Exec::File(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/java")
            .join(name),
    )
}

#[tokio::test]
async fn java_inline_code() {
    let mut sandbox = boot();
    sandbox
        .run("System.out.println(\"hluk-java-ok \" + 6 * 7);")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "hluk-java-ok 42\n");
}

/// A compact source file: the guest runs its main, as `java Hello.java`
/// would.
#[tokio::test]
async fn java_exec_file() {
    let mut sandbox = boot();
    for _ in 0..2 {
        sandbox.run(example("Hello.java")).await.unwrap();
        assert_eq!(
            sandbox.drain_output(),
            "Hello from Java 25 on Hyperlight!\n"
        );
    }
}

/// A class with `public static void main(String[])` runs too, and a call
/// that runs statements of its own does not also run a main it declares.
#[tokio::test]
async fn java_runs_a_declared_main() {
    let mut sandbox = boot();
    sandbox
        .run(
            "public class App {\n\
                 public static void main(String[] args) { System.out.println(\"main ran, args=\" + args.length); }\n\
             }",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "main ran, args=0\n");
    // A file runs its own main, not another overload an earlier call left.
    sandbox
        .run("void main() { System.out.println(\"A\"); }")
        .await
        .unwrap();
    sandbox
        .run("void main(String[] args) { System.out.println(\"B\"); }")
        .await
        .unwrap();
    sandbox
        .run("void main() { System.out.println(\"A\"); }")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "A\nB\nA\n");
    // With both in one file, main(String[]) is the one Java runs.
    sandbox
        .run("void main(String[] a) { System.out.println(\"args\"); }\nvoid main() { System.out.println(\"noargs\"); }").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "args\n");
    // Java 25 takes main(String[]) over main(), static or not.
    sandbox
        .run("class Rank { static void main() { System.out.println(\"static\"); } void main(String[] a) { System.out.println(\"instance args\"); } }").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "instance args\n");
    // The same file run again runs its main again, unchanged as it is.
    sandbox
        .run(
            "public class App {\n\
                 public static void main(String[] args) { System.out.println(\"main ran, args=\" + args.length); }\n\
             }",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "main ran, args=0\n");
    sandbox
        .run(
            "void main() { System.out.println(\"not run\"); }\nSystem.out.println(\"statements\");",
        )
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "statements\n");
    // An instance main, made with the class's no-argument constructor.
    sandbox
        .run("class Inst { String who = \"instance\"; void main() { System.out.println(who + \" main\"); } }").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "instance main\n");
    // Not with a private constructor, which Java 25's launcher refuses.
    assert!(
        sandbox
            .run("class Priv { private Priv() {} void main() { System.out.println(\"ran\"); } }")
            .await
            .is_err()
    );
    let out = sandbox.drain_output();
    assert!(
        out.contains("constructor is private") && !out.contains("ran"),
        "{out}"
    );
}

/// A compact source file's fields are declarations, not statements, and
/// its main may use a method declared after it, as `java Main.java` allows.
/// A later call that only redefines what main uses does not run it again.
#[tokio::test]
async fn java_compact_source_file_shapes() {
    let mut sandbox = boot();
    sandbox
        .run(
            "int count = 41;\n\
             void main() { System.out.println(greet() + \" \" + (count + 1)); }\n\
             String greet() { return \"hi\"; }",
        )
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "hi 42\n");
    sandbox
        .run("String greet() { return \"bye\"; }")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "");

    // A file with a package declaration runs, as `java App.java` runs it.
    sandbox
        .run("// App.java\n/* copied from a project */\npackage com.example.app;\n\npublic class Pkg { public static void main(String[] a) { System.out.println(\"packaged\"); } }").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "packaged\n");

    // Not a package name Java takes: rejected, as `java` would.
    assert!(sandbox
        .run("package 123;\nclass Num { public static void main(String[] a) { System.out.println(\"ran\"); } }").await
        .is_err());
    assert!(sandbox
        .run("package a.class;\nclass Kw { public static void main(String[] a) { System.out.println(\"ran\"); } }").await
        .is_err());
    assert!(!sandbox.drain_output().contains("ran"));
    sandbox
        .run("package \u{3c0}.\u{3bb};\nclass Uni { public static void main(String[] a) { System.out.println(\"unicode\"); } }").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "unicode\n");

    // `void main(` in a comment or a string is not a main: a library
    // waiting on a declaration loads, as it would without them.
    sandbox
        .run("class Lib extends NotYet { /* void main( */ String s = \"void main(\"; }")
        .await
        .unwrap();
    assert!(!sandbox.drain_output().contains("cannot run"));

    // A long comment header, with a package line after it or not.
    let header = "// a licence line\n".repeat(3000);
    sandbox
        .run(format!("{header}package a.b;\nclass Hdr {{ public static void main(String[] a) {{ System.out.println(\"header\"); }} }}")).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "header\n");
    sandbox
        .run(format!("{header}System.out.println(\"no package\");"))
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "no package\n");

    // A main that cannot run fails the call, as `java Main.java` would not
    // compile: a class waiting on a missing type, and a main calling a
    // missing method.
    assert!(sandbox
        .run("class App implements NoSuchType { public static void main(String[] a) { System.out.println(\"ran\"); } }").await
        .is_err());
    assert!(sandbox.drain_output().contains("App's main cannot run"));
    assert!(
        sandbox
            .run("class App2 { public static void main(String[] a) { helpr(); } }")
            .await
            .is_err()
    );
    assert!(sandbox.drain_output().contains("App2's main cannot run"));
}

/// What the code throws past the dispatcher fails the call, never the JVM:
/// a static initializer that throws, and an input too large for its
/// parameter.  State from before is still there.
#[tokio::test]
async fn java_failures_keep_the_runtime() {
    let mut sandbox = boot();
    sandbox
        .run("int kept = 42;\nclass H { static int v = 1 / 0; static int f() { return v; } }\nlong id(long x) { return x; }").await
        .unwrap();
    assert!(matches!(
        sandbox.call("H.f", "").await,
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(matches!(
        sandbox.call("id", "99999999999999999999").await,
        Err(Error::CallFailed { status: 1 })
    ));
    let out = sandbox.drain_output();
    assert!(out.contains("ExceptionInInitializerError"), "{out}");
    assert!(out.contains("/ by zero"), "{out}");
    assert!(out.contains("does not fit a Long"), "{out}");
    // A guest function waiting on a declaration says so; a null where the
    // parameter's collection cannot hold one fails plainly.
    sandbox
        .run("int greet(int x) { return x + 1; }")
        .await
        .unwrap();
    sandbox
        .run("int greet(int x) { return undefinedVar; }")
        .await
        .unwrap();
    assert!(sandbox.call("greet", "1").await.is_err());
    sandbox
        .run("int first(Deque<Integer> d) { return d.peekFirst(); }")
        .await
        .unwrap();
    assert!(sandbox.call("first", "[null]").await.is_err());
    let out = sandbox.drain_output();
    assert!(
        out.contains("error: greet (variable undefinedVar) uses something not declared yet"),
        "{out}"
    );
    assert!(out.contains("a Deque cannot hold null"), "{out}");
    sandbox.run("System.out.println(kept);").await.unwrap();
    assert_eq!(sandbox.drain_output(), "42\n");
}

/// Variables, methods and classes a call declares are there for the next.
#[tokio::test]
async fn java_state_is_kept_across_calls() {
    let mut sandbox = boot();
    sandbox
        .run("int calls = 0;\nString greet(String name) { return \"Hello, \" + name + \" #\" + (++calls); }").await
        .unwrap();
    sandbox
        .run("System.out.println(greet(\"a\"));")
        .await
        .unwrap();
    sandbox
        .run("System.out.println(greet(\"b\"));")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "Hello, a #1\nHello, b #2\n");
}

/// A compile error and an uncaught exception each fail the call with
/// status 1 and say why; the runtime serves the next call.
#[tokio::test]
async fn java_errors_fail_the_call() {
    let mut sandbox = boot();
    assert!(matches!(
        sandbox.run("System.out.println(undefinedThing);").await,
        Err(Error::CallFailed { status: 1 })
    ));
    let out = sandbox.drain_output();
    assert!(out.contains("cannot find symbol"), "{out:?}");
    assert!(out.contains("undefinedThing"), "{out:?}");

    assert!(matches!(
        sandbox
            .run("System.out.println(\"before\");\nthrow new IllegalStateException(\"boom\");")
            .await,
        Err(Error::CallFailed { status: 1 })
    ));
    let out = sandbox.drain_output();
    assert!(out.contains("before"), "{out:?}");
    assert!(
        out.contains("Exception in thread \"main\" java.lang.IllegalStateException: boom"),
        "{out:?}"
    );

    sandbox
        .run("System.out.println(\"still here\");")
        .await
        .unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}

#[tokio::test]
async fn java_snapshot_round_trip() {
    let snap_dir = temp_dir("java-snap");
    let mut sandbox = boot();
    sandbox.run("int kept = 41;").await.unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox
        .run("System.out.println(\"restored \" + (kept + 1));")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "restored 42\n");
}

#[test]
fn java_stdin_piped() {
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/java/StdinEcho.java");
    let output = hluk_with_stdin_scratch(
        &require_rootfs("java"),
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

/// The host's variables reach `System.getenv()`, and a change between
/// calls is seen by the next one.
#[tokio::test]
async fn java_env_vars() {
    let mut sandbox = boot();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox.run(example("EnvVars.java")).await.unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "MY_VAR=hello_world\nDEBUG=1\nGREETING=hi there\n"
    );
    sandbox.set_env_vars(&[("DEBUG", "2")]);
    sandbox
        .run("System.out.println(System.getenv(\"DEBUG\"));")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "2\n");
}

#[tokio::test]
async fn java_fs() {
    let mount_dir = temp_dir("java-fs");
    let mut sandbox = builder()
        .mounts(vec![Mount::rw(mount_dir.path(), "/mnt/host")])
        .boot()
        .unwrap();
    sandbox.run(example("FsOps.java")).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("read-back: hello from java"),
        "expected FsOps.java to read back its write, got: {output:?}",
    );
    assert!(
        output.contains("line-count: 3") && output.contains("fs-ops-done"),
        "expected FsOps.java to finish, got: {output:?}",
    );
    // The write landed on the host side of the mount.
    assert!(mount_dir.path().join("java_fs.txt").exists());
}

#[tokio::test]
async fn java_threading() {
    let mut sandbox = boot();
    sandbox.run(example("Threads.java")).await.unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("sum-of-squares: 140"), "{output:?}");
    assert!(output.contains("thread-counter: 4"), "{output:?}");
    assert!(
        output.contains("virtual thread ran") && output.contains("threads-done"),
        "{output:?}"
    );
}

#[tokio::test]
async fn java_http_get() {
    let output = run_http_get(builder, example("HttpGet.java")).await;
    assert!(output.contains("Status: 200"), "{output}");
}

/// A call ends when its code returns; a thread it started is left behind
/// and makes progress on the steps that follow, and the next call finds
/// the runtime intact.
#[tokio::test]
async fn java_thread_runs_between_calls() {
    let mut sandbox = boot();
    sandbox
        .run(concat!(
            "Thread.ofPlatform().start(() -> { for (int i = 1; i <= 5; i++) { try { Thread.sleep(300); } catch (InterruptedException e) {} System.out.println(\"tick \" + i); } });\n",
            "System.out.println(\"started\");\n",
        )).await
        .unwrap();
    // Judged by output, not the clock: a slow host may take longer to
    // compile the call than the thread takes to finish.
    let out = sandbox.drain_output();
    assert!(out.contains("started"), "{out:?}");
    assert!(
        !out.contains("tick 5"),
        "the call waited for the thread: {out:?}"
    );
    let mut out = String::new();
    // 120 s, not 30: the nightly's nested-virt arm64 runner needs many steps to reach tick 5.
    let deadline = Instant::now() + Duration::from_secs(120);
    while !out.contains("tick 5") && Instant::now() < deadline {
        sandbox.step().await.unwrap();
        out.push_str(&sandbox.drain_output());
    }
    assert!(
        out.contains("tick 5"),
        "thread did not run while parked: {out:?}"
    );
    sandbox.run("System.out.println(\"again\");").await.unwrap();
    assert!(sandbox.drain_output().contains("again"));
}

/// Every call runs on the dispatcher's thread, so an interrupt one call
/// leaves is cleared before the next.
#[tokio::test]
async fn java_an_interrupt_does_not_carry_over() {
    let mut sandbox = boot();
    sandbox
        .run("Thread.currentThread().interrupt();")
        .await
        .unwrap();
    sandbox
        .run("Thread.sleep(10); System.out.println(\"slept\");")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "slept\n");
}

/// A JVM ended by `System.exit()` gives its memory back: the next one
/// starts in it, with the heap the first had, as often as the code exits.
#[tokio::test]
async fn java_restarts_get_the_memory_back() {
    let mut sandbox = boot();
    let max = async |s: &mut hyperlight_unikraft::AppSandbox| -> u64 {
        s.run("System.out.println(Runtime.getRuntime().maxMemory() >> 20);")
            .await
            .unwrap();
        s.drain_output().trim().parse().unwrap()
    };
    let first = max(&mut sandbox).await;
    for restart in 1..=6 {
        sandbox.run("System.exit(0);").await.unwrap();
        let heap = max(&mut sandbox).await;
        assert!(
            heap * 10 >= first * 9,
            "restart {restart}: heap {heap} MiB, first {first} MiB"
        );
    }
}

/// A JVM started after `System.exit()` sizes its heap from the memory free
/// then, so running out is still an OutOfMemoryError the code can catch,
/// after each restart.
#[tokio::test]
async fn java_a_restarted_jvm_still_throws_out_of_memory() {
    let mut sandbox = boot();
    let fill = concat!(
        "var keep = new ArrayList<byte[]>();\n",
        "try { while (true) keep.add(new byte[1 << 20]); }\n",
        "catch (OutOfMemoryError e) { int n = keep.size(); keep = null; System.out.println(\"oom after \" + n); }\n",
    );
    sandbox.run(fill).await.unwrap();
    assert!(sandbox.drain_output().contains("oom after"));
    for restart in 1..=3 {
        sandbox.run("System.exit(0);").await.unwrap();
        sandbox.run(fill).await.unwrap();
        assert!(
            sandbox.drain_output().contains("oom after"),
            "restart {restart}"
        );
    }
}

/// The value an expression leaves ($1) is freed, and a variable declared
/// again with the same source keeps one value: forty megabytes ten times
/// over fits a heap of under 200, in seconds.
#[tokio::test]
async fn java_old_values_are_freed() {
    let mut sandbox = boot();
    for _ in 0..10 {
        sandbox
            .run("Arrays.copyOf(new byte[0], 40 << 20);")
            .await
            .unwrap();
        sandbox
            .run("byte[] data = new byte[40 << 20];")
            .await
            .unwrap();
    }
    sandbox
        .run("System.out.println(data.length >> 20);")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "40\n");
    // Nor is a value rendered as text, which would run its toString().
    sandbox
        .run("record Loud(int x) { public String toString() { System.out.println(\"toString ran\"); return \"L\"; } }\nvar l = new Loud(1);\nnew Loud(2);").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "");
}

/// Input is held to JSON's grammar, which Java's own number and escape
/// parsing is looser than.
#[tokio::test]
async fn java_input_must_be_json() {
    let mut sandbox = boot();
    sandbox
        .run("double id(double x) { return x; }\nString s(String x) { return x; }")
        .await
        .unwrap();
    for bad in ["01", "1.", "1.e5", "+1", "-", "1e"] {
        assert!(sandbox.call("id", bad).await.is_err(), "{bad} was taken");
    }
    for bad in [r#""\u+041""#, r#""\u-041""#, r#""\u004""#] {
        assert!(sandbox.call("s", bad).await.is_err(), "{bad} was taken");
    }
    assert_eq!(sandbox.call("id", "-0.5e1").await.unwrap(), "-5");
    // Out of range is refused, not infinity; a float's range too.
    assert!(sandbox.call("id", "1e400").await.is_err());
    sandbox.run("float f(float x) { return x; }").await.unwrap();
    assert!(sandbox.call("f", "1e39").await.is_err());
    assert_eq!(sandbox.call("f", "1.5").await.unwrap(), "1.5");
    // Past a long, an untyped value is a BigInteger.
    sandbox
        .run("String kind(Object o) { return o.getClass().getSimpleName(); }")
        .await
        .unwrap();
    assert_eq!(
        sandbox.call("kind", "18446744073709551615").await.unwrap(),
        r#""BigInteger""#
    );
    assert_eq!(
        sandbox.call("s", r#""\u0041\n""#).await.unwrap(),
        r#""A\n""#
    );
}

/// A variable declared again keeps its old value for code still running
/// that reads it, as in JShell.
#[tokio::test]
async fn java_redeclaring_a_variable_leaves_running_code_its_value() {
    let mut sandbox = boot();
    sandbox
        .run("var hits = new java.util.concurrent.atomic.AtomicInteger();")
        .await
        .unwrap();
    sandbox
        .run("var go = new java.util.concurrent.CountDownLatch(1);\nvar done = new java.util.concurrent.CountDownLatch(1);\nThread.ofPlatform().start(() -> { try { go.await(); } catch (InterruptedException e) {} System.out.println(\"old hits \" + hits.incrementAndGet()); done.countDown(); });").await
        .unwrap();
    sandbox
        .run("var hits = new java.util.concurrent.atomic.AtomicInteger(100);\ngo.countDown(); done.await(); System.out.println(\"new hits \" + hits.get());").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "old hits 1\nnew hits 100\n");
}

/// What a thread the code started throws is printed, as Java does.
#[tokio::test]
async fn java_a_thread_s_uncaught_exception_is_printed() {
    let mut sandbox = boot();
    sandbox
        .run("var t = new Thread(() -> { throw new RuntimeException(\"boom\"); }); t.start(); t.join(); System.out.println(\"after\");").await
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("java.lang.RuntimeException: boom"), "{out}");
    assert!(out.contains("after"), "{out}");
}

/// A thread that never ends, left behind by one call, does not hold up the
/// next: JShell does not wait for the threads a snippet started.
#[tokio::test]
async fn java_a_thread_left_running_does_not_block_the_next_call() {
    let mut sandbox = boot();
    sandbox
        .run(
            "Thread.ofPlatform().start(() -> { while (true) { try { Thread.sleep(1000); } catch (InterruptedException e) {} } });\n\
             System.out.println(\"left running\");",
        ).await
        .unwrap();
    sandbox.run("System.out.println(\"next\");").await.unwrap();
    assert_eq!(sandbox.drain_output(), "left running\nnext\n");
}

/// `System.exit()` ends the JVM, which is the runtime: the call ends with
/// that status, and the next call starts a fresh one, without the state
/// the first had.
#[tokio::test]
async fn java_system_exit_ends_the_call() {
    let mut sandbox = boot();
    sandbox
        .run("int kept = 1;\nSystem.out.println(\"leaving\"); System.exit(0); System.out.println(\"not reached\");").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\n");
    sandbox
        .run("System.out.println(\"still here\");")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "still here\n");
    assert!(
        sandbox.run("System.out.println(kept);").await.is_err(),
        "the fresh JVM had the old one's state"
    );

    // A non-zero code fails the call with it.
    assert!(matches!(
        sandbox.run("System.exit(3);").await,
        Err(Error::CallFailed { status: 3 })
    ));
}

/// The heap is a share of the memory the kernel reports, so an allocation
/// that cannot be served is an OutOfMemoryError the code can catch, not a
/// page fault the kernel cannot serve.
#[tokio::test]
async fn java_out_of_memory_is_an_error() {
    let mut sandbox = boot();
    sandbox
        .run(concat!(
            "System.out.println(\"max=\" + (Runtime.getRuntime().maxMemory() >> 20));\n",
            "var keep = new ArrayList<byte[]>();\n",
            "try { while (true) keep.add(new byte[1 << 20]); }\n",
            "catch (OutOfMemoryError e) { int n = keep.size(); keep = null; System.out.println(\"oom after \" + n); }\n",
        )).await
        .unwrap();
    let out = sandbox.drain_output();
    let max_mb: u64 = out
        .lines()
        .find_map(|l| l.trim().strip_prefix("max="))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no max line in {out:?}"));
    assert!(
        (100..SCRATCH_MB as u64 / 2).contains(&max_mb),
        "the heap should be a share of the guest's memory, got {max_mb} MiB: {out:?}"
    );
    assert!(out.contains("oom after"), "no OutOfMemoryError: {out:?}");
    sandbox
        .run("System.out.println(\"still here\");")
        .await
        .unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}

/// A static method of a snippet run earlier, called with JSON in and out:
/// top-level or in a class, the input converted to its parameter, a
/// future awaited, statics kept, and a newer definition wins.
#[tokio::test]
async fn java_calls_a_guest_function() {
    let mut sandbox = boot();
    sandbox
        .run(
            "record Event(String name) {}\n\
             record Greeting(String greeting, int calls) {}\n\
             class Handlers {\n\
                 static int calls;\n\
                 static Greeting greet(Event e) { return new Greeting(\"Hello, \" + e.name() + \"!\", ++calls); }\n\
                 static CompletableFuture<Integer> later(Map<String, Object> e) {\n\
                     return CompletableFuture.supplyAsync(() -> ((Number) e.get(\"x\")).intValue() * 2);\n\
                 }\n\
                 static void nothing() { System.out.println(\"ran\"); }\n\
                 static int bad(Object e) { throw new IllegalStateException(\"handler boom\"); }\n\
             }\n\
             double total(List<Double> v) { return v.stream().mapToDouble(Double::doubleValue).sum(); }",
        ).await
        .unwrap();
    assert_eq!(
        sandbox.call("greet", r#"{"name":"World"}"#).await.unwrap(),
        r#"{"greeting":"Hello, World!","calls":1}"#
    );
    assert_eq!(
        sandbox
            .call("Handlers.greet", r#"{"name":"again"}"#)
            .await
            .unwrap(),
        r#"{"greeting":"Hello, again!","calls":2}"#
    );
    assert_eq!(sandbox.call("later", r#"{"x":21}"#).await.unwrap(), "42");
    assert_eq!(sandbox.call("total", "[1, 2.5, 3]").await.unwrap(), "6.5");
    assert_eq!(sandbox.call("nothing", "").await.unwrap(), "");
    assert!(matches!(
        sandbox.call("bad", "{}").await,
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(sandbox.call("missing", "{}").await.is_err());
    assert!(sandbox.call("greet", "not json").await.is_err());
    assert!(sandbox.call("greet", "[1]").await.is_err());
    let output = sandbox.drain_output();
    assert!(output.contains("ran"), "{output}");
    assert!(output.contains("handler boom"), "{output}");
    assert!(output.contains("no static method missing"), "{output}");
    // A snippet with statements still runs, and its definitions are found:
    // a redefinition replaces the old one.
    sandbox
        .run("System.out.println(\"statements\");\nclass Handlers { static String greet(Object e) { return \"newer\"; } }").await
        .unwrap();
    assert_eq!(sandbox.call("greet", "{}").await.unwrap(), r#""newer""#);
    // A name in two places, two classes or two overloads, is ambiguous.
    sandbox
        .run("class A { static int g() { return 1; } }\nclass B { static int g() { return 2; } }\nint f(int x) { return x; }\nString f(String s) { return s; }").await
        .unwrap();
    assert!(sandbox.call("g", "").await.is_err());
    assert_eq!(sandbox.call("B.g", "").await.unwrap(), "2");
    assert!(sandbox.call("f", "7").await.is_err());
    let output = sandbox.drain_output();
    assert!(output.contains("g names 2 methods (B.g, A.g)"), "{output}");
    // A top-level method wins a bare name over a class's.
    sandbox.run("int g() { return 0; }").await.unwrap();
    assert_eq!(sandbox.call("g", "").await.unwrap(), "0");
    // Empty input is an empty Optional, not null.
    sandbox
        .run("String opt(Optional<String> o) { return o.orElse(\"none\"); }")
        .await
        .unwrap();
    assert_eq!(sandbox.call("opt", "").await.unwrap(), r#""none""#);
}

/// Only what JShell still has is called: a class redefined without a
/// method no longer offers it.  A nested class's method is named by its
/// path.  Collections and map keys take the parameter's types, and a float
/// comes back as written.
#[tokio::test]
async fn java_guest_function_lookup_and_types() {
    let mut sandbox = boot();
    sandbox
        .run("class H { static int f() { return 1; } }")
        .await
        .unwrap();
    assert_eq!(sandbox.call("H.f", "").await.unwrap(), "1");
    sandbox
        .run("class H { static int g() { return 2; } }")
        .await
        .unwrap();
    assert!(sandbox.call("H.f", "").await.is_err());
    assert!(sandbox.call("f", "").await.is_err());
    assert_eq!(sandbox.call("H.g", "").await.unwrap(), "2");
    // A nested class the new version dropped is gone with it.
    sandbox
        .run("class K { static class Inner { static int m() { return 1; } } }")
        .await
        .unwrap();
    assert_eq!(sandbox.call("K.Inner.m", "").await.unwrap(), "1");
    sandbox.run("class K { }").await.unwrap();
    assert!(sandbox.call("K.Inner.m", "").await.is_err());
    assert!(sandbox.call("m", "").await.is_err());
    // A version waiting on a declaration replaces the old one, which is no
    // longer called, and is called once it can be.
    sandbox
        .run("class W { static int f() { return 1; } }")
        .await
        .unwrap();
    assert_eq!(sandbox.call("W.f", "").await.unwrap(), "1");
    sandbox
        .run("class W extends Base { static int f() { return 2; } }")
        .await
        .unwrap();
    assert!(sandbox.call("W.f", "").await.is_err());
    sandbox.run("class Base { }").await.unwrap();
    assert_eq!(sandbox.call("W.f", "").await.unwrap(), "2");

    sandbox
        .run(
            "class Outer { static class Inner { static String m() { return \"inner\"; } } }\n\
             int first(Deque<Integer> d) { return d.peekFirst(); }\n\
             String keys(TreeMap<Integer, String> m) { return m.firstKey() + 1 + m.firstEntry().getValue(); }\n\
             float tenth() { return 0.1f; }\n\
             enum Color { RED; public String toString() { return \"red\"; } }\n\
             Map<Color, Integer> colors(Map<Color, Integer> m) { return m; }\n\
             java.nio.file.Path where() { return Path.of(\"a\"); }\n\
             int local() { class Local { static int onlyLocal() { return 1; } } return Local.onlyLocal(); }",
        ).await
        .unwrap();
    assert_eq!(
        sandbox.call("Outer.Inner.m", "").await.unwrap(),
        r#""inner""#
    );
    assert_eq!(sandbox.call("first", "[7, 8]").await.unwrap(), "7");
    assert_eq!(
        sandbox
            .call("keys", r#"{"2": "b", "1": "a"}"#)
            .await
            .unwrap(),
        r#""2a""#
    );
    assert_eq!(sandbox.call("tenth", "").await.unwrap(), "0.1");
    assert_eq!(
        sandbox.call("colors", r#"{"RED": 1}"#).await.unwrap(),
        r#"{"RED":1}"#
    );
    assert_eq!(sandbox.call("where", "").await.unwrap(), r#""a""#);
    // A local class's method is not a guest function.
    assert!(sandbox.call("onlyLocal", "").await.is_err());
    assert_eq!(sandbox.call("local", "").await.unwrap(), "1");
}

/// A JVM started again after `System.exit()` starts with the environment
/// it booted with: a host variable that steers the launcher does not reach
/// it, and the code still sees the host's variables.
#[tokio::test]
async fn java_restart_ignores_host_launcher_variables() {
    let mut sandbox = boot();
    sandbox.set_env_vars(&[("LD_LIBRARY_PATH", "/nowhere"), ("MY_VAR", "seen")]);
    sandbox.run("System.exit(0);").await.unwrap();
    sandbox
        .run("System.out.println(System.getenv(\"MY_VAR\") + \" \" + System.getenv(\"LD_LIBRARY_PATH\"));").await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "seen /nowhere\n");
}

/// The embedder's functions through `Host.call`; a host error is a
/// `HostException`.
#[tokio::test]
async fn java_calls_host_functions() {
    let mut sandbox = builder()
        .host_function("math.add", |args| {
            let v: Vec<f64> = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok(v.iter().sum::<f64>().to_string())
        })
        .host_function("math.fail", |_| Err("no can do".to_string()))
        .host_function("db.lookup", |args| {
            let id = args.trim_matches(|c| c == '[' || c == ']');
            Ok(format!(r#"{{"id": {id}, "name": "Ada"}}"#))
        })
        .host_function("big", |_| Ok(format!("\"{}\"", "x".repeat(40_000))))
        .boot()
        .unwrap();
    // A reply larger than a pipe buffer.
    sandbox
        .run("System.out.println(Host.call(String.class, \"big\").length());")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "40000\n");
    sandbox
        .run(
            "System.out.println(Host.call(int.class, \"math.add\", 2, 3));\n\
             record User(int id, String name) {}\n\
             System.out.println(Host.call(User.class, \"db.lookup\", 7).name());\n\
             System.out.println(((Map<?, ?>) Host.call(\"db.lookup\", 8)).get(\"id\"));\n\
             try { Host.call(\"math.fail\"); } catch (HostException e) { System.out.println(\"caught \" + e.getMessage()); }",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "5\nAda\n8\ncaught no can do\n");
    sandbox
        .run("double sum(double[] v) { return Host.call(double.class, \"math.add\", Arrays.stream(v).boxed().toArray()); }").await
        .unwrap();
    assert_eq!(sandbox.call("sum", "[1,2,3]").await.unwrap(), "6");
}

/// A host function call from a thread the guest function left behind,
/// after the call ended, is refused rather than interleaved with the next
/// call on the pipe, and calls go on working.  The thread runs while the
/// host steps the parked guest, so no call is in flight when it tries.
#[tokio::test]
async fn java_host_call_after_the_call_is_refused() {
    let mut sandbox = builder()
        .host_function("log", |_| Ok(String::new()))
        .boot()
        .unwrap();
    sandbox
        .run(
            "class Bg {\n\
                 static void start() {\n\
                     Thread.ofPlatform().daemon().start(() -> {\n\
                         try { Thread.sleep(50); } catch (InterruptedException e) {}\n\
                         try { Host.call(\"log\"); System.out.println(\"bg: called\"); }\n\
                         catch (IllegalStateException e) { System.out.println(\"bg: refused\"); }\n\
                     });\n\
                 }\n\
                 static int echo(int x) { return x; }\n\
             }",
        )
        .await
        .unwrap();
    sandbox.call("start", "").await.unwrap();
    let mut out = String::new();
    // 120 s, not 30: the nightly's nested-virt arm64 runner needs many steps to reach the bg output.
    let deadline = Instant::now() + Duration::from_secs(120);
    while !out.contains("bg:") && Instant::now() < deadline {
        sandbox.step().await.unwrap();
        out.push_str(&sandbox.drain_output());
    }
    assert!(out.contains("bg: refused"), "{out}");
    for i in 0..3 {
        assert_eq!(
            sandbox.call("echo", &i.to_string()).await.unwrap(),
            i.to_string()
        );
    }
}

/// `examples/java/Handler.java`: loaded once, called twice, its state
/// carried over.
#[tokio::test]
async fn java_handler_example() {
    let mut sandbox = boot();
    sandbox.run(example("Handler.java")).await.unwrap();
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
