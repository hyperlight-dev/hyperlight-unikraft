// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! hl_wasmtimedriver: the wasmtime image's driver.  It brings a Wasmtime
//! engine up once, then runs WebAssembly for each call the host makes:
//!
//! - `Exec` carries WebAssembly text (WAT), a module or a component, run
//!   as a WASI command.
//! - `GuestExec` carries `path [args]`: a `.wasm` (module or component),
//!   `.wat`, or `.cwasm` precompiled by `wasmtime compile` of the same
//!   version, in the guest filesystem or a mount; empty runs
//!   `/entrypoint.wasm`.
//!
//! - `Call` carries (function, input): call the export `function` of the
//!   program loaded last, with `input` a JSON array of its arguments; its
//!   results, as JSON, are the call's result (see json.rs for the
//!   mapping).  A component's interface export is `interface#function`.
//!
//! A command (a module with `_start`, a component exporting
//! `wasi:cli/run`) runs to completion under WASI preview 1, 0.2 or 0.3
//! (a component using any 0.3 interface is linked and run asynchronously,
//! its 0.2 imports with it), with the
//! call's arguments, the host's environment, stdio, and the guest
//! filesystem preopened at `/`.  Its exit code is the call's status.  A
//! module or component without one is a library: it is instantiated
//! (running `_initialize` if it has one) and kept, its memory and state
//! with it, for `Call`s until another program is loaded; the environment
//! it sees is the one it was loaded with.
//!
//! An import WASI does not provide is a function of the embedder's
//! (`SandboxBuilder::host_function`): a module's import `m.f`, and a
//! component's function `f` of an imported interface `ns:pkg/m@v` or a
//! bare `f`, call the host function `m.f` (or `f`) with the arguments as
//! a JSON array, and take its result as JSON.  A host error traps.
//!
//! The engine and every compiled program are kept across calls, so a
//! program compiled once (or during a warm snapshot) is not compiled again.

mod hlcall;
mod json;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde_json::Value as Json;
use wasmtime::component::types::{ComponentFunc, ComponentItem};
use wasmtime::component::{Component, Linker as ComponentLinker, ResourceTable};
use wasmtime::error::Context;
use wasmtime::{
    Config, Engine, ExternType, FuncType, Linker, Module, Result, Store, bail, format_err,
};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::bindings::Command as P2AsyncCommand;
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::p3::bindings::Command as P3Command;
use wasmtime_wasi::runtime::in_tokio;
use wasmtime_wasi::{FsPerms, I32Exit, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use hlcall::{Call, Device};

const NAME: &str = "hl_wasmtimedriver";
const ENTRYPOINT: &str = "/entrypoint.wasm";

/// Compiled code, a core module or a component.
#[derive(Clone)]
enum Program {
    Module(Module),
    Component(Component),
}

/// A file's version for the compile cache: a rewritten file is compiled
/// again, and replaces the old version's code.
type FileVersion = (Option<SystemTime>, u64);

struct Driver {
    engine: Engine,
    module_linker: Linker<WasiP1Ctx>,
    component_linker: ComponentLinker<P2State>,
    /// For a component using WASI 0.3: its functions are async, so the
    /// store is driven asynchronously, and 0.2 is linked async to match.
    p3_linker: ComponentLinker<P2State>,
    compiled: HashMap<PathBuf, (FileVersion, Program)>,
    /// The library loaded last, which `Call` calls into.
    loaded: Option<Loaded>,
}

/// A library instance kept for calls, with the store that owns its state.
enum Loaded {
    Module {
        store: Store<WasiP1Ctx>,
        instance: wasmtime::Instance,
    },
    Component {
        store: Store<P2State>,
        instance: wasmtime::component::Instance,
        /// Linked with WASI 0.3: called asynchronously.
        p3: bool,
    },
}

/// What a component's store holds: the WASI 0.2 context and its resources.
struct P2State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for P2State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

fn engine() -> Result<Engine> {
    let mut config = Config::new();
    // The guest kernel does not deliver a fault in JIT code to a signal
    // handler, so out-of-bounds accesses are caught with explicit bounds
    // checks instead of guard pages; and without guard pages there is no
    // point reserving 4 GiB of address space per memory, which a guest a
    // few hundred MiB in size cannot back anyway.
    config.signals_based_traps(false);
    config.memory_reservation(0);
    config.memory_guard_size(0);
    config.memory_reservation_for_growth(1 << 20);
    config.memory_init_cow(false);
    config.wasm_component_model(true);
    // WASI 0.3: async functions, streams and futures.
    config.wasm_component_model_async(true);
    Engine::new(&config)
}

impl Driver {
    fn new() -> Result<Driver> {
        let engine = engine()?;
        let mut module_linker = Linker::new(&engine);
        p1::add_to_linker_sync(&mut module_linker, |cx| cx)?;
        let mut component_linker = ComponentLinker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut component_linker)?;
        let mut p3_linker = ComponentLinker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut p3_linker)?;
        wasmtime_wasi::p3::add_to_linker(&mut p3_linker)?;
        Ok(Driver {
            engine,
            module_linker,
            component_linker,
            p3_linker,
            compiled: HashMap::new(),
            loaded: None,
        })
    }

    /// Serve one call; its status and result.
    fn dispatch(&mut self, call: &Call, env: &[(String, String)]) -> (i32, Option<Vec<u8>>) {
        if call.name() == Some("Call") {
            let (Some(function), Some(input)) = (call.string_arg(0), call.string_arg(1)) else {
                eprintln!("{NAME}: Call without a function and an input");
                return (-1, None);
            };
            return match self.call(function, input) {
                Ok(result) => (0, result),
                Err(e) => {
                    eprintln!("{NAME}: {function}: {e:?}");
                    (1, None)
                }
            };
        }
        let result = match call.name() {
            Some("Exec") => match call.string_arg(0) {
                Some(wat) => self
                    .compile_text(wat)
                    .and_then(|p| self.run(&p, &["main.wasm".to_string()], env)),
                None => Err(format_err!("Exec without a string parameter")),
            },
            Some("GuestExec") => {
                let mut argv = hlcall::split_ws(call.string_arg(0).unwrap_or(""));
                if argv.is_empty() {
                    argv.push(ENTRYPOINT.to_string());
                }
                self.load(Path::new(&argv[0]))
                    .and_then(|p| self.run(&p, &argv, env))
            }
            other => Err(format_err!("no such call {other:?}")),
        };
        match result {
            Ok(status) => (status, None),
            Err(e) => {
                eprintln!("{NAME}: {e:?}");
                (1, None)
            }
        }
    }

    fn compile_text(&self, text: &str) -> Result<Program> {
        let bytes = wat::parse_str(text).context("the call is not WebAssembly text")?;
        self.compile(&bytes)
    }

    fn compile(&self, bytes: &[u8]) -> Result<Program> {
        Ok(if wasmparser::Parser::is_component(bytes) {
            Program::Component(Component::new(&self.engine, bytes)?)
        } else {
            Program::Module(Module::new(&self.engine, bytes)?)
        })
    }

    /// A program from the guest filesystem, compiled once per version of
    /// the file.
    fn load(&mut self, path: &Path) -> Result<Program> {
        let meta = std::fs::metadata(path).with_context(|| format!("{}", path.display()))?;
        let version = (meta.modified().ok(), meta.len());
        if let Some((v, p)) = self.compiled.get(path)
            && *v == version
        {
            return Ok(p.clone());
        }
        let bytes = std::fs::read(path).with_context(|| format!("{}", path.display()))?;
        let program = if path.extension().is_some_and(|e| e == "cwasm") {
            // SAFETY: a .cwasm is trusted to be `wasmtime compile` output
            // for this engine's version and settings; Wasmtime checks both
            // and refuses one that does not match.
            match Engine::detect_precompiled(&bytes) {
                Some(wasmtime::Precompiled::Component) => {
                    Program::Component(unsafe { Component::deserialize(&self.engine, &bytes)? })
                }
                Some(wasmtime::Precompiled::Module) => {
                    Program::Module(unsafe { Module::deserialize(&self.engine, &bytes)? })
                }
                None => bail!("{}: not precompiled WebAssembly", path.display()),
            }
        } else {
            let bytes = wat::parse_bytes(&bytes).with_context(|| format!("{}", path.display()))?;
            self.compile(&bytes)?
        };
        self.compiled
            .insert(path.to_path_buf(), (version, program.clone()));
        Ok(program)
    }

    fn wasi(argv: &[String], env: &[(String, String)]) -> Result<WasiCtxBuilder> {
        let mut b = WasiCtxBuilder::new();
        // WASI's generators would otherwise be seeded once, in this
        // process, and captured by a warm snapshot with the library a
        // context belongs to: every clone would draw the same "random"
        // bytes.  Drawn from the kernel instead, which reseeds on every
        // restore.
        b.secure_random(KernelRng)
            .insecure_random(KernelRng)
            .insecure_random_seed(u128::from_ne_bytes(kernel_random()));
        b.inherit_stdio()
            .args(argv)
            .envs(env)
            .inherit_network()
            .allow_ip_name_lookup(true)
            .preopened_dir("/", "/", FsPerms::ReadWrite)?;
        Ok(b)
    }

    /// Run a command, or load a library; the command's exit code, 0 for a
    /// library.
    fn run(&mut self, program: &Program, argv: &[String], env: &[(String, String)]) -> Result<i32> {
        let outcome = match program {
            Program::Module(module) => {
                let mut linker = self.module_linker.clone();
                link_module_host_functions(&mut linker, module)?;
                let mut store = Store::new(&self.engine, Self::wasi(argv, env)?.build_p1());
                let instance = linker.instantiate(&mut store, module)?;
                match instance.get_typed_func::<(), ()>(&mut store, "_start") {
                    Ok(start) => start.call(&mut store, ()),
                    Err(_) => {
                        if let Ok(init) =
                            instance.get_typed_func::<(), ()>(&mut store, "_initialize")
                        {
                            init.call(&mut store, ())?;
                        }
                        self.loaded = Some(Loaded::Module { store, instance });
                        return Ok(0);
                    }
                }
            }
            Program::Component(component) => {
                let p3 = uses_p3(component, &self.engine);
                let mut linker = if p3 {
                    self.p3_linker.clone()
                } else {
                    self.component_linker.clone()
                };
                link_component_host_functions(&mut linker, component, &self.engine)?;
                let state = P2State {
                    ctx: Self::wasi(argv, env)?.build(),
                    table: ResourceTable::new(),
                };
                let mut store = Store::new(&self.engine, state);
                let run = component
                    .component_type()
                    .exports(&self.engine)
                    .find_map(|(name, _)| name.strip_prefix("wasi:cli/run@"))
                    .map(str::to_owned);
                let Some(run) = run else {
                    let instance = if p3 {
                        in_tokio(linker.instantiate_async(&mut store, component))?
                    } else {
                        linker.instantiate(&mut store, component)?
                    };
                    self.loaded = Some(Loaded::Component {
                        store,
                        instance,
                        p3,
                    });
                    return Ok(0);
                };
                // `wasi:cli/run` returns an error with no code (`exit(1)`
                // in the guest's libc); an `exit(n)` unwinds as I32Exit.
                let ran = if !p3 {
                    Command::instantiate(&mut store, component, &linker)
                        .and_then(|command| command.wasi_cli_run().call_run(&mut store))
                } else if run.starts_with("0.3") {
                    in_tokio(async {
                        let command =
                            P3Command::instantiate_async(&mut store, component, &linker).await?;
                        store
                            .run_concurrent(async move |store| {
                                command.wasi_cli_run().call_run(store).await
                            })
                            .await?
                    })
                } else {
                    in_tokio(async {
                        let command =
                            P2AsyncCommand::instantiate_async(&mut store, component, &linker)
                                .await?;
                        command.wasi_cli_run().call_run(&mut store).await
                    })
                };
                match ran {
                    Ok(Ok(())) => Ok(()),
                    Ok(Err(())) => return Ok(1),
                    Err(e) => Err(e),
                }
            }
        };
        match outcome {
            Ok(()) => Ok(0),
            Err(e) => match e.downcast_ref::<I32Exit>() {
                Some(exit) => Ok(exit.0),
                None => Err(e),
            },
        }
    }

    /// Call an export of the loaded library; its results as JSON.
    fn call(&mut self, function: &str, input: &str) -> Result<Option<Vec<u8>>> {
        let mut trapped = None;
        let results = match self.loaded.as_mut() {
            None => bail!(
                "nothing is loaded to call: run a module without _start, or a component \
                 without wasi:cli/run, first"
            ),
            Some(Loaded::Module { store, instance }) => {
                let func = instance
                    .get_func(&mut *store, function)
                    .ok_or_else(|| format_err!("the module exports no function {function:?}"))?;
                let ty = func.ty(&*store);
                let params: Vec<_> = ty.params().collect();
                let args = json::arguments(input, params.len())?
                    .iter()
                    .zip(&params)
                    .map(|(j, t)| json::core_from_json(t, j))
                    .collect::<Result<Vec<_>>>()?;
                let mut results = vec![wasmtime::Val::I32(0); ty.results().len()];
                func.call(&mut *store, &args, &mut results)?;
                results
                    .iter()
                    .map(json::core_to_json)
                    .collect::<Result<Vec<_>>>()?
            }
            Some(Loaded::Component {
                store,
                instance,
                p3,
            }) => {
                // `interface#function` for a function of an exported
                // interface, the bare name for one the world exports.
                let func = match function.split_once('#') {
                    Some((iface, name)) => instance
                        .get_export_index(&mut *store, None, iface)
                        .and_then(|i| instance.get_export_index(&mut *store, Some(&i), name))
                        .and_then(|i| instance.get_func(&mut *store, i)),
                    None => instance.get_func(&mut *store, function),
                }
                .ok_or_else(|| format_err!("the component exports no function {function:?}"))?;
                let ty = func.ty(&*store);
                let params: Vec<_> = ty.params().map(|(_, t)| t).collect();
                let args = json::arguments(input, params.len())?
                    .iter()
                    .zip(&params)
                    .map(|(j, t)| json::from_json(t, j))
                    .collect::<Result<Vec<_>>>()?;
                let mut results = vec![wasmtime::component::Val::Bool(false); ty.results().len()];
                // An async export (WASI 0.3's kind) runs to completion,
                // with the tasks it starts, before its results are in.
                let called = if *p3 {
                    in_tokio(func.call_async(&mut *store, &args, &mut results))
                } else {
                    func.call(&mut *store, &args, &mut results)
                };
                match called {
                    Ok(()) => results
                        .iter()
                        .map(json::to_json)
                        .collect::<Result<Vec<_>>>()?,
                    Err(e) => {
                        trapped = Some(e);
                        Vec::new()
                    }
                }
            }
        };
        // Wasmtime will not enter an instance a trap left in an unknown
        // state again: drop it, so the next call says what to do rather
        // than fail the same way.
        if let Some(e) = trapped {
            self.loaded = None;
            return Err(
                e.context("the component trapped and was unloaded; load it again to call it")
            );
        }
        Ok(json::results(results).map(|j| j.to_string().into_bytes()))
    }
}

/// Randomness straight from the kernel's getrandom().
struct KernelRng;

fn fill_from_kernel(buf: &mut [u8]) {
    let mut done = 0;
    while done < buf.len() {
        // SAFETY: writes at most the remaining bytes of `buf`.
        let n = unsafe { libc::getrandom(buf[done..].as_mut_ptr().cast(), buf.len() - done, 0) };
        if n > 0 {
            done += n as usize;
        } else if std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {
            // The kernel's generator cannot fail once booted; a WASI
            // program must not get zeros for random bytes regardless.
            panic!("getrandom: {}", std::io::Error::last_os_error());
        }
    }
}

fn kernel_random<const N: usize>() -> [u8; N] {
    let mut b = [0u8; N];
    fill_from_kernel(&mut b);
    b
}

impl rand_core::TryRng for KernelRng {
    type Error = std::convert::Infallible;
    fn try_next_u32(&mut self) -> std::result::Result<u32, Self::Error> {
        Ok(u32::from_ne_bytes(kernel_random()))
    }
    fn try_next_u64(&mut self) -> std::result::Result<u64, Self::Error> {
        Ok(u64::from_ne_bytes(kernel_random()))
    }
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> std::result::Result<(), Self::Error> {
        fill_from_kernel(dst);
        Ok(())
    }
}

/// Call the embedder's host function `name` with `args`, as the kernel
/// forwards it, and read its reply as JSON (`null` for none).
fn host_function(name: &str, args: Vec<Json>) -> Result<Json> {
    let reply = hlcall::host_call(name, &Json::Array(args).to_string())
        .map_err(|e| format_err!("host function {name}: {e}"))?;
    if reply.is_empty() {
        return Ok(Json::Null);
    }
    serde_json::from_str(&reply)
        .map_err(|e| format_err!("host function {name}'s result is not JSON: {e}"))
}

/// The value(s) a host function returned, one per result.
fn reply_values(reply: Json, count: usize) -> Result<Vec<Json>> {
    Ok(match count {
        0 => Vec::new(),
        1 => vec![reply],
        n => match reply {
            Json::Array(items) if items.len() == n => items,
            other => bail!("expected {n} results as an array, got {other}"),
        },
    })
}

/// Satisfy every import of `module` that WASI does not with the host
/// function `module.name`.
fn link_module_host_functions(linker: &mut Linker<WasiP1Ctx>, module: &Module) -> Result<()> {
    // A module may import the same function more than once.
    linker.allow_shadowing(true);
    for import in module.imports() {
        let (m, n) = (import.module(), import.name());
        if matches!(m, "wasi_snapshot_preview1" | "wasi_unstable") {
            continue;
        }
        let ExternType::Func(ty) = import.ty() else {
            bail!("import {m}.{n} is not a function: only functions come from the host");
        };
        let name = format!("{m}.{n}");
        let results: Vec<_> = FuncType::results(&ty).collect();
        linker.func_new(m, n, ty.clone(), move |_caller, params, out| {
            let args = params
                .iter()
                .map(json::core_to_json)
                .collect::<Result<_>>()?;
            let reply = host_function(&name, args)?;
            for ((slot, ty), j) in out
                .iter_mut()
                .zip(&results)
                .zip(reply_values(reply, results.len())?)
            {
                *slot = json::core_from_json(ty, &j)?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// Whether `component` needs the async linker: it imports or exports a
/// WASI 0.3 interface, or exports an async function (which needs no WASI
/// import, and which a sync call cannot enter).
fn uses_p3(component: &Component, engine: &Engine) -> bool {
    let ty = component.component_type();
    let p3 = |name: &str| name.starts_with("wasi:") && name.contains("@0.3");
    let is_async = |item: &ComponentItem| match item {
        ComponentItem::ComponentFunc(f) => f.async_(),
        ComponentItem::ComponentInstance(i) => i
            .exports(engine)
            .any(|(_, e)| matches!(&e.ty, ComponentItem::ComponentFunc(f) if f.async_())),
        _ => false,
    };
    ty.imports(engine).any(|(n, _)| p3(n))
        || ty.exports(engine).any(|(n, e)| p3(n) || is_async(&e.ty))
}

/// Whether the WASI 0.2 linker provides the interface `name`: the ones in
/// wasmtime-wasi's p2 world.  Another `wasi:` interface (`wasi:http`,
/// `wasi:keyvalue`) is the embedder's to provide, like any other import.
fn provided_by_wasi(name: &str) -> bool {
    const PROVIDED: &[&str] = &[
        "wasi:cli/",
        "wasi:clocks/",
        "wasi:filesystem/",
        "wasi:io/",
        "wasi:random/",
        "wasi:sockets/",
    ];
    PROVIDED.iter().any(|p| name.starts_with(p))
}

/// Satisfy every function import of `component` outside WASI with the
/// host function named after it: `m.f` for the function `f` of an
/// imported interface `ns:pkg/m@version`, `f` for a bare function.
fn link_component_host_functions(
    linker: &mut ComponentLinker<P2State>,
    component: &Component,
    engine: &Engine,
) -> Result<()> {
    for (name, item) in component.component_type().imports(engine) {
        match item.ty {
            ComponentItem::ComponentInstance(iface) if !provided_by_wasi(name) => {
                let short = name
                    .rsplit_once('/')
                    .map_or(name, |(_, rest)| rest)
                    .split('@')
                    .next()
                    .unwrap_or(name);
                let mut instance = linker.instance(name)?;
                for (fname, fitem) in iface.exports(engine) {
                    if let ComponentItem::ComponentFunc(_) = fitem.ty {
                        let full = format!("{short}.{fname}");
                        instance.func_new(fname, move |_store, ty, params, out| {
                            call_component_host(&full, &ty, params, out)
                        })?;
                    }
                }
            }
            ComponentItem::ComponentFunc(_) => {
                let full = name.to_string();
                linker
                    .root()
                    .func_new(name, move |_store, ty, params, out| {
                        call_component_host(&full, &ty, params, out)
                    })?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn call_component_host(
    name: &str,
    ty: &ComponentFunc,
    params: &[wasmtime::component::Val],
    out: &mut [wasmtime::component::Val],
) -> Result<()> {
    let args = params.iter().map(json::to_json).collect::<Result<_>>()?;
    let reply = host_function(name, args)?;
    let types: Vec<_> = ty.results().collect();
    for ((slot, ty), j) in out
        .iter_mut()
        .zip(&types)
        .zip(reply_values(reply, types.len())?)
    {
        *slot = json::from_json(ty, &j)?;
    }
    Ok(())
}

fn main() {
    let mut device = match Device::open() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{NAME}: cannot open /dev/hlcall: {e} (kernel without the step model?)");
            std::process::exit(1);
        }
    };
    let mut driver = match Driver::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{NAME}: cannot start Wasmtime: {e:?}");
            std::process::exit(1);
        }
    };
    // Serve calls for as long as the guest lives; returning would end the
    // process and with it the engine and the compiled code.
    loop {
        let (status, result) = match device.next_call() {
            Ok((buf, env)) => match Call::parse(buf) {
                Some(call) => driver.dispatch(&call, &env),
                None => {
                    eprintln!("{NAME}: a malformed call");
                    (-1, None)
                }
            },
            Err(e) => {
                eprintln!("{NAME}: /dev/hlcall: {e}");
                std::process::exit(1);
            }
        };
        device.finish(status, result.as_deref());
    }
}
