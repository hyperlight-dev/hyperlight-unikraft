// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Two guests restored from one snapshot draw different random numbers,
//! in every runtime: the kernel reseeds its CSPRNG on restore, and each
//! driver reseeds the generators its runtime seeded before the snapshot
//! (docs/random.md).  Each test uses the generators before the snapshot,
//! so they are seeded and hold state, then draws from them in two
//! restores of it.

mod common;

use common::{BIN_MOUNT, require_bin, require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, Mount, SandboxBuilder};

/// [`two_clones_with`], with no mounts.
async fn two_clones(builder: SandboxBuilder, warm: Exec, draw: Exec) -> [String; 2] {
    two_clones_with(builder, Vec::new(), warm, draw).await
}

/// Boot `builder`, run `warm`, snapshot, and run `draw` in two restores
/// of the snapshot, with `mounts`: what each printed.
async fn two_clones_with(
    builder: SandboxBuilder,
    mounts: Vec<Mount>,
    warm: Exec,
    draw: Exec,
) -> [String; 2] {
    let snap_dir = temp_dir("snapshot-random");
    let mut sandbox = builder.mounts(mounts.clone()).boot().unwrap();
    sandbox.run(warm).await.unwrap();
    sandbox.drain_output();
    sandbox.snapshot_to(&snap_dir).unwrap();
    drop(sandbox);
    let restore_and_draw = async || {
        let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
            .unwrap()
            .mounts(mounts.clone())
            .boot()
            .unwrap();
        clone.run(draw.clone()).await.unwrap();
        clone.drain_output()
    };
    [restore_and_draw().await, restore_and_draw().await]
}

/// Each line of the two clones' output differs: one generator a line.
fn assert_each_line_differs(outputs: &[String; 2], generators: &[&str]) {
    let a: Vec<&str> = outputs[0].lines().collect();
    let b: Vec<&str> = outputs[1].lines().collect();
    assert_eq!(a.len(), generators.len(), "{outputs:?}");
    assert_eq!(b.len(), generators.len(), "{outputs:?}");
    for ((x, y), name) in a.iter().zip(&b).zip(generators) {
        assert_ne!(x, y, "two clones drew the same from {name}: {outputs:?}");
    }
}

/// Python's random module, OpenSSL (ssl.RAND_bytes), SQLite's random()
/// and tempfile's names, each used before the snapshot.
#[tokio::test]
async fn python_shell_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("python-shell")).scratch_mb(256);
    let outputs = two_clones(
        builder,
        Exec::Code(
            "import random, ssl, sqlite3, tempfile\n\
             random.random(); ssl.RAND_bytes(16)\n\
             db = sqlite3.connect(':memory:'); db.execute('select random()').fetchone()\n\
             next(tempfile._get_candidate_names())\n"
                .into(),
        ),
        Exec::Code(
            "print(random.random())\n\
             print(ssl.RAND_bytes(16).hex())\n\
             print(db.execute('select random()').fetchone()[0])\n\
             print(next(tempfile._get_candidate_names()))\n"
                .into(),
        ),
    )
    .await;
    assert_each_line_differs(&outputs, &["random", "OpenSSL", "SQLite", "tempfile"]);
}

/// A seed code gave random before the snapshot is not kept across it: a
/// restore draws anew, as from a seed of the kernel's.
#[tokio::test]
async fn python_reseeds_once_per_restore_not_per_call() {
    let snap_dir = temp_dir("snapshot-random-once");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    // Within one guest a seed set in one call holds for the next.
    sandbox.run("import random; random.seed(42)").await.unwrap();
    sandbox.run("print(random.random())").await.unwrap();
    let mut expected = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    expected
        .run("import random; random.seed(42); print(random.random())")
        .await
        .unwrap();
    let seeded = expected.drain_output();
    assert_eq!(sandbox.drain_output(), seeded);
    sandbox.snapshot_to(&snap_dir).unwrap();
    drop(sandbox);
    let restore_and_draw = async || {
        let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
            .unwrap()
            .boot()
            .unwrap();
        clone.run("print(random.random())").await.unwrap();
        let draw = clone.drain_output();
        // Reseeded once, by the first call: a seed set after it holds.
        clone.run("random.seed(42)").await.unwrap();
        clone.run("print(random.random())").await.unwrap();
        assert_eq!(clone.drain_output(), seeded);
        draw
    };
    let draws = [restore_and_draw().await, restore_and_draw().await];
    assert_ne!(draws[0], draws[1], "two clones drew the same: {draws:?}");
}

/// NumPy's legacy global generator (np.random.rand and the like) and
/// cryptography's own OpenSSL, both in the agent image.
#[tokio::test]
async fn agent_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("agent")).scratch_mb(1536);
    let outputs = two_clones(
        builder,
        Exec::Code(
            "import numpy as np\n\
             from cryptography.hazmat.primitives import serialization\n\
             from cryptography.hazmat.primitives.asymmetric import ed25519\n\
             np.random.rand(); ed25519.Ed25519PrivateKey.generate()\n"
                .into(),
        ),
        Exec::Code(
            "print(np.random.rand())\n\
             key = ed25519.Ed25519PrivateKey.generate().public_key()\n\
             print(key.public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw).hex())\n"
                .into(),
        ),
    ).await;
    assert_each_line_differs(&outputs, &["numpy", "cryptography"]);
}

/// Math.random, OpenSSL (crypto.randomBytes) and the bytes randomUUID()
/// and randomInt() keep ahead, each used before the snapshot.
#[tokio::test]
async fn node_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("node")).scratch_mb(512);
    let outputs = two_clones(
        builder,
        Exec::Code(
            "const c = require('crypto');\n\
             Math.random(); c.randomBytes(16); c.randomUUID(); globalThis.crypto.randomUUID(); c.randomInt(10);"
                .into(),
        ),
        Exec::Code(
            "const c = require('crypto');\n\
             console.log(Math.random());\n\
             console.log(c.randomBytes(16).toString('hex'));\n\
             console.log(c.randomUUID());\n\
             console.log(globalThis.crypto.randomUUID());\n\
             console.log(c.randomInt(2 ** 40));"
                .into(),
        ),
    ).await;
    assert_each_line_differs(
        &outputs,
        &[
            "Math.random",
            "OpenSSL",
            "crypto.randomUUID",
            "webcrypto randomUUID",
            "crypto.randomInt",
        ],
    );
}

/// Math.random keeps what Math.random promises: a double in [0, 1).
#[tokio::test]
async fn node_math_random_stays_in_range() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("node"))
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox
        .run(
            "let ok = true, sum = 0;\n\
             for (let i = 0; i < 100000; i++) { const x = Math.random(); ok = ok && x >= 0 && x < 1; sum += x; }\n\
             console.log(ok, Math.abs(sum / 100000 - 0.5) < 0.01, Math.random.name, Math.random.length);",
        ).await
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "true true random 0");
}

/// A program the exec driver starts runs after the restore, so what its
/// runtime seeds (Go's math/rand from AT_RANDOM, Rust's HashMap keys from
/// getrandom) seeds from the kernel, reseeded: nothing for a driver to
/// do, but checked.
#[tokio::test]
async fn go_and_rust_clones_draw_their_own() {
    for (runtime, bin, scratch) in [("go", "random", 128), ("rust", "random", 64)] {
        let bins = require_bin(runtime, bin);
        let builder = SandboxBuilder::from_initrd(require_rootfs(runtime)).scratch_mb(scratch);
        let run = Exec::Guest(format!("{BIN_MOUNT}/{bin}"));
        let outputs =
            two_clones_with(builder, vec![Mount::ro(&bins, BIN_MOUNT)], run.clone(), run).await;
        assert_each_line_differs(&outputs, &[runtime]);
    }
}

/// The shell's $RANDOM, used before the snapshot, and a RANDOM the code
/// made a variable of its own, which the reseed leaves alone.  The
/// reseed keeps the code's line numbers.
#[tokio::test]
async fn bash_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("bash")).scratch_mb(256);
    let outputs = two_clones(
        builder,
        Exec::Code("echo $RANDOM >/dev/null".into()),
        Exec::Code("echo $RANDOM$RANDOM".into()),
    )
    .await;
    assert_each_line_differs(&outputs, &["$RANDOM"]);
    let builder = SandboxBuilder::from_initrd(require_rootfs("bash")).scratch_mb(256);
    let outputs = two_clones(
        builder,
        Exec::Code("export RANDOM=7".into()),
        Exec::Code("echo $RANDOM $LINENO".into()),
    )
    .await;
    assert_eq!(outputs, ["7 1\n", "7 1\n"].map(String::from));

    // A first call that does not parse still finds $RANDOM reseeded.
    let snap_dir = temp_dir("snapshot-random-bash");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("bash"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("echo $RANDOM >/dev/null").await.unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();
    drop(sandbox);
    let restore_and_draw = async || {
        let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
            .unwrap()
            .boot()
            .unwrap();
        let _ = clone.run("echo \"unterminated").await;
        clone.drain_output();
        clone.run("echo $RANDOM$RANDOM").await.unwrap();
        clone.drain_output()
    };
    let draws = [restore_and_draw().await, restore_and_draw().await];
    assert_ne!(draws[0], draws[1], "two clones drew the same: {draws:?}");
}

/// Math.random, new Random(), ThreadLocalRandom, SplittableRandom,
/// RandomGenerator.getDefault(), Collections.shuffle, UUID.randomUUID and
/// the seeders every DRBG and SHA1PRNG draw from, each used before the
/// snapshot.
#[tokio::test]
async fn java_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("java")).scratch_mb(512);
    let outputs = two_clones(
        builder,
        Exec::Code(
            "import java.util.*; import java.util.concurrent.*; import java.util.random.*;\n\
             Math.random(); new Random().nextLong(); ThreadLocalRandom.current().nextLong();\n\
             new SplittableRandom().nextLong(); RandomGenerator.getDefault().nextLong();\n\
             Collections.shuffle(new ArrayList<>(List.of(1, 2))); UUID.randomUUID();\n\
             java.security.SecureRandom.getInstance(\"DRBG\").nextLong();\n\
             java.security.SecureRandom.getInstance(\"SHA1PRNG\").nextLong();"
                .into(),
        ),
        Exec::Code(
            "System.out.println(Math.random());\n\
             System.out.println(new Random().nextLong());\n\
             System.out.println(ThreadLocalRandom.current().nextLong());\n\
             System.out.println(new SplittableRandom().nextLong());\n\
             System.out.println(RandomGenerator.getDefault().nextLong());\n\
             var l = new ArrayList<Integer>(); for (int i = 0; i < 20; i++) l.add(i);\n\
             Collections.shuffle(l); System.out.println(l);\n\
             System.out.println(UUID.randomUUID());\n\
             System.out.println(java.security.SecureRandom.getInstance(\"DRBG\").nextLong());\n\
             System.out.println(java.security.SecureRandom.getInstance(\"SHA1PRNG\").nextLong());"
                .into(),
        ),
    )
    .await;
    assert_each_line_differs(
        &outputs,
        &[
            "Math.random",
            "new Random()",
            "ThreadLocalRandom",
            "SplittableRandom",
            "RandomGenerator.getDefault()",
            "Collections.shuffle",
            "UUID.randomUUID",
            "a DRBG made after the restore",
            "a SHA1PRNG made after the restore",
        ],
    );
}

/// PHP's mt_rand and random_bytes, used before the snapshot (a seed code
/// gave mt_srand() stays, tests/php.rs).
#[tokio::test]
async fn php_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("php")).scratch_mb(128);
    let outputs = two_clones(
        builder,
        Exec::Code("mt_rand(); random_bytes(4);".into()),
        Exec::Code("echo mt_rand(), \"\\n\", bin2hex(random_bytes(8)), \"\\n\";".into()),
    )
    .await;
    assert_each_line_differs(&outputs, &["mt_rand", "random_bytes"]);
}

/// QuickJS's Math.random, used before the snapshot.
#[tokio::test]
async fn quickjs_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("quickjs")).scratch_mb(64);
    let outputs = two_clones(
        builder,
        Exec::Code("Math.random();".into()),
        Exec::Code("console.log(Math.random());".into()),
    )
    .await;
    assert_each_line_differs(&outputs, &["Math.random"]);
}

/// Random.Shared, new Random(), RandomNumberGenerator (OpenSSL's),
/// Guid.NewGuid and RandomNumberGenerator.GetInt32, each used before the
/// snapshot.
#[tokio::test]
async fn dotnet_clones_draw_their_own() {
    let builder = SandboxBuilder::from_initrd(require_rootfs("dotnet-jit")).scratch_mb(768);
    let outputs = two_clones(
        builder,
        Exec::Code(
            "using System.Security.Cryptography;\n\
             Random.Shared.NextInt64(); new Random().NextInt64(); RandomNumberGenerator.GetBytes(16);\n\
             using (var rng = RandomNumberGenerator.Create()) rng.GetBytes(new byte[16]);\n\
             Guid.NewGuid(); RandomNumberGenerator.GetInt32(10);"
                .into(),
        ),
        Exec::Code(
            "using System.Security.Cryptography;\n\
             Console.WriteLine(Random.Shared.NextInt64());\n\
             Console.WriteLine(new Random().NextInt64());\n\
             Console.WriteLine(Convert.ToHexString(RandomNumberGenerator.GetBytes(16)));\n\
             var b = new byte[16]; using (var rng = RandomNumberGenerator.Create()) rng.GetBytes(b);\n\
             Console.WriteLine(Convert.ToHexString(b));\n\
             Console.WriteLine(Guid.NewGuid());\n\
             Console.WriteLine(RandomNumberGenerator.GetInt32(int.MaxValue));"
                .into(),
        ),
    ).await;
    assert_each_line_differs(
        &outputs,
        &[
            "Random.Shared",
            "new Random()",
            "RandomNumberGenerator.GetBytes",
            "RandomNumberGenerator.Create",
            "Guid.NewGuid",
            "RandomNumberGenerator.GetInt32",
        ],
    );
}

/// A guest function call runs on the dispatcher's own thread, which the
/// snapshot has, not on a thread of its own as run code does: Java's
/// ThreadLocalRandom and .NET's Random.Shared there, each used before the
/// snapshot, through a guest function call.
#[tokio::test]
async fn guest_function_calls_draw_their_own() {
    let cases = [
        (
            "java",
            512,
            "import java.util.concurrent.*;\n\
             long draw() { return ThreadLocalRandom.current().nextLong(); }",
            "draw",
        ),
        (
            "dotnet-jit",
            768,
            "public static class Handlers { public static long Draw() => Random.Shared.NextInt64(); }",
            "Draw",
        ),
    ];
    for (runtime, scratch, define, function) in cases {
        let snap_dir = temp_dir("snapshot-random-call");
        let mut sandbox = SandboxBuilder::from_initrd(require_rootfs(runtime))
            .scratch_mb(scratch)
            .boot()
            .unwrap();
        sandbox.run(define).await.unwrap();
        sandbox.call(function, "").await.unwrap();
        sandbox.snapshot_to(&snap_dir).unwrap();
        drop(sandbox);
        let restore_and_draw = async || {
            let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
                .unwrap()
                .boot()
                .unwrap();
            clone.call(function, "").await.unwrap()
        };
        let draws = [restore_and_draw().await, restore_and_draw().await];
        assert_ne!(
            draws[0], draws[1],
            "two {runtime} clones drew the same: {draws:?}"
        );
    }
}
