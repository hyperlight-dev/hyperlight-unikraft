# Guest function calls and host function calls

`run` sends the guest code and waits for it to finish. Two more kinds of call let you use a sandbox the way [hyperlight-js](https://github.com/hyperlight-dev/hyperlight-js) and [hyperlight-wasm](https://github.com/hyperlight-dev/hyperlight-wasm) do:

- a **guest function call** is the host calling into the guest: it runs a function the guest already defined, with an input, and returns its result (`AppSandbox::call`);
- a **host function call** is the guest calling into the host: code in the guest calls a function you registered (`SandboxBuilder::host_function`).

Load the code once, call it as often as you like, and give it only the capabilities you choose.

```rust
use hyperlight_unikraft::SandboxBuilder;

let mut sandbox = SandboxBuilder::from_initrd("build-elfloader/quickjs-rootfs.cpio")
    .host_function("db.lookup", |args| {
        let [id]: [u32; 1] = serde_json::from_str(args).map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "id": id, "name": "Ada" }).to_string())
    })
    .boot()?;

sandbox.run(r#"
    import { lookup } from "host:db";
    globalThis.greet = (event) => ({ message: `Hello, ${lookup(event.id).name}` });
"#)?;

let out = sandbox.call("greet", r#"{"id": 7}"#)?;
assert_eq!(out, r#"{"message":"Hello, Ada"}"#);
```

[`examples/host_functions.rs`](../examples/host_functions.rs) does the same in every runtime that makes host function calls: `cargo run --release --example host_functions`. quickjs, node, python and dotnet-jit each also have a `handler` example to run with `hluk run --call` (`examples/{quickjs,node,python}/handler.*`, `examples/dotnet-jit/Handler.cs`); for wasmtime, [`examples/wasmtime/calculator`](../examples/wasmtime/calculator/) is a library component to call.

## Guest function calls

`AppSandbox::call(function, input)` calls the guest function and waits for its result, which comes back as a string. Anything the function prints goes to `drain_output()`, as it does for `run`.

| Image | `function` is | `input` | Result |
|---|---|---|---|
| quickjs | a global function (`function f() {}`, `var`, or `globalThis.f = ...`) | JSON, passed as the one argument | the return value (awaited if it is a promise) as JSON |
| node | an export of the module `--guest-exec` ran, else a global function | JSON, passed as the one argument | the return value (awaited if it is a promise) as JSON |
| python, python-shell, agent | a function in the file `--guest-exec` ran, else in `__main__` | JSON, passed as the one argument | the return value (awaited if it is a coroutine) as JSON |
| wasmtime | an export of the library loaded last | a JSON array, one element per parameter | the results as JSON |
| dotnet-jit | a `public static` method of a public class in a snippet run earlier (`Greet`, or `Handlers.Greet`) | JSON, deserialized into its one parameter | the return value (awaited if it is a `Task`) as JSON, camelCase |

Empty input calls the function with no argument. A function that returns nothing (`undefined`, `None`, `void`, no results) gives an empty result.

- **State carries over.** Globals, statics, module caches and a WebAssembly instance's memory stay from one call to the next. To have them in a warm snapshot, define the function in `warm_exec`.
- **Errors.** If the function throws or traps, doesn't exist, or can't take the input, the call fails with `Error::CallFailed`. The details are in the output. The guest keeps serving later calls; a WebAssembly component that trapped has to be loaded again.
- **Async work.** QuickJS and Node run the timers and jobs the function started before returning. A promise that nothing can settle fails the call instead of hanging it.
- **Not waiting.** `submit(Exec::Call { .. })` sends the call without waiting, and `take_result()` collects the result once `step` reports `CallDone`.
- **Other images** fail a guest function call. PowerShell and the compiled images (c, go, rust, dotnet-aot) start a new process for every run, so nothing they define is left to call; bash has no way to return a value but stdout.

In the dotnet-jit image a snippet with only definitions (classes, no statements) loads as a library. In the wasmtime image a module with no `_start`, or a component that doesn't export `wasi:cli/run`, is a library: running it (its text, or `Exec::Guest(path)`) loads it and keeps it for guest function calls until another library is loaded. A function of an exported interface is called as `interface#function`, for example `example:app/api#greet`.

### From the CLI

`hluk run --call FUNCTION [--input JSON]` runs the workload, then the guest function call, and prints the result. With no workload, only the call runs.

```sh
hluk run --initrd build-elfloader/quickjs-rootfs.cpio examples/quickjs/handler.js \
  --call handler --input '{"name":"World"}'
# {"greeting":"Hello, World!","calls":1}
```

The CLI registers no host functions, so a host function call fails there. Use the library for that.

## Host function calls

`SandboxBuilder::host_function(name, f)` takes `f: Fn(&str) -> Result<String, String>`. It receives the guest's arguments as a JSON array and returns its result as JSON, or an error message.

| Image | How the guest calls `math.add` | A host error |
|---|---|---|
| quickjs | `host.call("math.add", 2, 3)`, or `import { add } from "host:math"` | throws an `Error` |
| node | `host.call("math.add", 2, 3)` | throws an `Error` |
| python, python-shell, agent | `hyperlight.call("math.add", 2, 3)`, or `hyperlight.host.math.add(2, 3)` | raises `hyperlight.HostError` |
| dotnet-jit | `Host.Call<int>("math.add", 2, 3)`, or `Host.Call(...)` for a `JsonElement?` | throws `Hyperlight.HostException` |
| wasmtime | an import: `add` of module `math` (core module), or `add` of an interface `ns:pkg/math` (component) | traps |

- **Host function calls are synchronous.** The guest waits while your function runs on the thread driving the sandbox. In Python, only the thread running the code or guest function can make a host function call; others get a `RuntimeError`.
- **Snapshots.** Functions belong to the sandbox, not the guest. A restored guest calls the functions of the builder that restored it, so different hosts can serve the same snapshot with different functions. One catch: a quickjs `host:` module is built the first time it is imported, so a snapshot taken after `import ... from "host:db"` keeps that module's exports.
- **Limits.** A host function call's name and arguments together, and its reply, are at most 64 KiB, as is a guest function's result. The empty name is reserved: it lists the registered functions.

### WebAssembly imports

The wasmtime image links every import that WASI doesn't provide to a host function, so a component can be written against a WIT world you implement, with no bindings generated on the host:

```wit
interface math {
    add: func(a: s32, b: s32) -> s32;
}

world calculator {
    import math;        // calls the host function math.add
    export sum-of-squares: func(a: s32, b: s32) -> s32;
}
```

- The interface's last path segment names the host function: `my:app/math` and `other:pkg/math` both call `math.*`. A bare imported function `f` calls `f`.
- The built-in WASI interfaces are `wasi:cli`, `wasi:clocks`, `wasi:filesystem`, `wasi:io`, `wasi:random` and `wasi:sockets`. Any other interface, `wasi:http` included, is yours to provide.
- An interface with resource types can't be provided this way, because a resource can't cross as JSON.

[`examples/wasmtime/calculator`](../examples/wasmtime/calculator/) is a complete component.

## JSON

Guest code never sees JSON: it passes and gets native values (JavaScript values, Python objects, C# types, WIT-typed values), and the driver converts. On the host, you work with the JSON text directly:

- A host function receives its arguments as a JSON array, one element per argument: `host.call("math.add", 2, 3)` arrives as `[2,3]`, and a call with no arguments as `[]`.
- It returns JSON text in `Ok` (`"5"`, `"\"hello\""`, `"{\"id\":7}"`), or an empty string for no value. Text that isn't JSON fails in the guest: the driver can't convert it.
- `Err` is plain text. It isn't parsed; the guest gets it as the error's message.
- A guest function call's input is JSON text, and its result comes back as JSON text.

You don't need serde: `format!` or a string literal is enough, and so is any JSON library. The kernel doesn't look at the bytes at all, and the library only requires UTF-8 (`Error::ResultNotText` otherwise). JSON is what the drivers agree on to turn text into typed values.

For WebAssembly, values convert by their WIT type:

| WIT | JSON |
|---|---|
| integers, floats | numbers, range-checked (`-1` is not a `u32`) |
| `bool` | `true` / `false` |
| `char`, `string` | strings |
| `list<T>`, `tuple<...>` | arrays |
| `record` | an object of its fields |
| `option<T>` | `null`, or the value |
| `result<T, E>` | `{"ok": T}` or `{"err": E}` |
| `enum` | the case's name |
| `variant` | `{"case": payload}`, or the case's name when it has none |
| `flags` | an array of the names set |

A core module's values are numbers. Resources, streams and futures can't be converted. `option<option<T>>` doesn't round-trip: `some(none)` and `none` are both `null`.

## WASI 0.3

The wasmtime image runs WASI 0.3 components as well as 0.1 and 0.2. That covers:

- async exports, including `wasi:cli/run@0.3.0`;
- streams and futures inside the guest;
- concurrent tasks.

A component that uses any 0.3 interface is linked and driven asynchronously. Its 0.2 imports (Rust's standard library still uses them) are linked alongside. A call to an async export returns once the export and the tasks it started have finished.

Wasmtime 49 marks its 0.3 support as experimental, so treat it as such. Build a 0.3 component with the [`wasip3`](https://crates.io/crates/wasip3) crate for `wasm32-wasip2`; see [`examples/wasmtime/hello-p3`](../examples/wasmtime/hello-p3/).

## Compared with hyperlight-js and hyperlight-wasm

| | hyperlight-js / hyperlight-wasm | here |
|---|---|---|
| Handlers, JSON in and out | `add_handler` + `handle_event` | `run` defines them, `call` calls them, in JavaScript, Python and C# |
| Host functions | `host:` modules; `env` or WIT imports | the same, plus `host.call`, Python's `hyperlight.call` and C#'s `Host.Call` |
| Wasm modules and components | `load_module` + `call_guest_function`; WIT world fixed when the host is built | any module or component, typed through JSON at run time |
| WASI | a subset of preview 1 | preview 1, 0.2 and 0.3: files, clocks, random, sockets, environment, arguments |
| Precompiled Wasm | required (`hyperlight-wasm-aot`) | optional: `.wasm` compiles in the guest, `.cwasm` loads as is |
| Snapshots | yes | yes, on disk too, and a warm snapshot on the first `hluk run` |
| Killing a runaway call | `interrupt_handle().kill()` | `interrupt_handle().kill()` |
| Network, filesystem | none | host sockets under a policy, host directories as mounts |

## How it works

A guest function call is a `Call(function, input)` that the kernel queues on `/dev/hlcall`, like `Exec`. The driver writes the result after the status, and the kernel sends it to the host as `CallResult` before `CallDone`. Host function calls all travel as one kernel host function, `HostCall(name, args)`, which the library dispatches by name, so adding one needs no kernel change. [driver.md](driver.md) has the device protocol, and [execution.md](execution.md) has the host side.
