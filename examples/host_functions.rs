// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Host function calls from every runtime that makes them.  The embedder
//! registers `math.add` once, and each guest defines a guest function,
//! sum of squares, that calls it:
//!
//! - JavaScript: `import { add } from 'host:math'` (quickjs) or
//!   `host.call('math.add', ...)` (node);
//! - Python: `hyperlight.host.math.add(...)`;
//! - C#: `Host.Call<long>("math.add", ...)` (dotnet-jit);
//! - Java: `Host.call(long.class, "math.add", ...)`;
//! - PHP: `Hyperlight\call('math.add', ...)`;
//! - WebAssembly: the WIT interface `my:app/math` the component imports,
//!   from a plain export and from an async one (WASI 0.3).
//!
//! ```sh
//! for r in quickjs node python dotnet-jit java php wasmtime; do just build-rootfs $r; done
//! (cd examples/wasmtime/calculator && cargo build --release --target wasm32-wasip2)
//! cargo run --release --example host_functions
//! ```

use std::path::Path;

use hyperlight_unikraft::{AppSandbox, Exec, Mount, SandboxBuilder};

fn sandbox(runtime: &str) -> hyperlight_unikraft::Result<SandboxBuilder> {
    let rootfs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("build-elfloader/{runtime}-rootfs.cpio"));
    // Arguments arrive as a JSON array, the reply goes back as JSON text;
    // an Err is an exception in the guest (a trap in WebAssembly).
    Ok(
        SandboxBuilder::from_initrd(rootfs).host_function("math.add", |args| {
            let [a, b]: [i64; 2] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            println!("  host: math.add({a}, {b})");
            Ok((a + b).to_string())
        }),
    )
}

async fn show(
    sandbox: &mut AppSandbox,
    function: &str,
    input: &str,
) -> hyperlight_unikraft::Result<()> {
    let result = sandbox.call(function, input).await?;
    println!("  {function}({input}) = {result}");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyperlight_unikraft::Result<()> {
    println!("quickjs:");
    let mut js = sandbox("quickjs")?.boot()?;
    js.run(
        "import { add } from 'host:math';\n\
         globalThis.sumOfSquares = ({ a, b }) => add(a * a, b * b);",
    )
    .await?;
    show(&mut js, "sumOfSquares", r#"{"a": 3, "b": 4}"#).await?;

    println!("node:");
    let mut node = sandbox("node")?.boot()?;
    node.run("function sumOfSquares({ a, b }) { return host.call('math.add', a * a, b * b) }")
        .await?;
    show(&mut node, "sumOfSquares", r#"{"a": 3, "b": 4}"#).await?;

    println!("python:");
    let mut py = sandbox("python")?.boot()?;
    py.run(
        "from hyperlight import host\n\
         def sum_of_squares(e):\n    \
             return host.math.add(e['a'] ** 2, e['b'] ** 2)\n",
    )
    .await?;
    show(&mut py, "sum_of_squares", r#"{"a": 3, "b": 4}"#).await?;

    println!("dotnet-jit:");
    let mut cs = sandbox("dotnet-jit")?.boot()?;
    cs.run(
        "public record Pair(long A, long B);\n\
         public static class Squares {\n\
             public static long SumOfSquares(Pair p) => Host.Call<long>(\"math.add\", p.A * p.A, p.B * p.B);\n\
         }",
    ).await?;
    show(&mut cs, "SumOfSquares", r#"{"a": 3, "b": 4}"#).await?;

    println!("java:");
    let mut java = sandbox("java")?.boot()?;
    java.run(
        "record Pair(long a, long b) {}\n\
         long sumOfSquares(Pair p) { return Host.call(long.class, \"math.add\", p.a() * p.a(), p.b() * p.b()); }",
    ).await?;
    show(&mut java, "sumOfSquares", r#"{"a": 3, "b": 4}"#).await?;

    println!("php:");
    let mut php = sandbox("php")?.boot()?;
    php.run(
        "function sum_of_squares(array $p): int { return Hyperlight\\call('math.add', $p['a'] ** 2, $p['b'] ** 2); }",
    ).await?;
    show(&mut php, "sum_of_squares", r#"{"a": 3, "b": 4}"#).await?;

    println!("wasmtime:");
    let component = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/wasmtime/calculator/target/wasm32-wasip2/release");
    let mut wasm = sandbox("wasmtime")?
        .mount(Mount::ro(component, "/app"))
        .boot()?;
    // A component that exports no `wasi:cli/run` is a library: loading it
    // keeps it for the calls that follow.
    wasm.run(Exec::Guest("/app/calculator.wasm".into())).await?;
    show(&mut wasm, "sum-of-squares", "[3, 4]").await?;
    // An async export (WASI 0.3) is awaited, timer and all.
    show(&mut wasm, "slow-sum-of-squares", "[5, 12, 50]").await?;
    Ok(())
}
