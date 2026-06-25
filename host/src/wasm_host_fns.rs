use anyhow::{anyhow, bail, Context, Result};
use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use wasmtime::{Config, Engine, InstancePre, Linker, Module, Store};
use wasmtime_wasi::p2::pipe::{MemoryInputPipe, MemoryOutputPipe};
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::{DirPerms, FilePerms, I32Exit, WasiCtxBuilder};

#[derive(Clone)]
pub struct WasmToolOptions {
    dirs: Vec<WasiDir>,
    env: Vec<(String, String)>,
    fuel: u64,
    output_limit: usize,
}

#[derive(Clone)]
struct WasiDir {
    host: PathBuf,
    guest: String,
    read_only: bool,
}

pub struct WasmTool {
    name: String,
    path: PathBuf,
    engine: Engine,
    pre: InstancePre<WasiP1Ctx>,
    options: WasmToolOptions,
}

impl WasmToolOptions {
    pub fn from_cli(
        rw_dirs: &[String],
        ro_dirs: &[String],
        env: &[String],
        inherit_env: &[String],
        fuel: u64,
        output_limit: usize,
    ) -> Result<Self> {
        if fuel == 0 {
            bail!("--tool-wasi-fuel must be greater than 0");
        }
        if output_limit == 0 {
            bail!("--tool-wasi-output-limit must be greater than 0");
        }

        let mut dirs = Vec::with_capacity(rw_dirs.len() + ro_dirs.len());
        for spec in rw_dirs {
            dirs.push(parse_wasi_dir(spec, false)?);
        }
        for spec in ro_dirs {
            dirs.push(parse_wasi_dir(spec, true)?);
        }
        let mut guest_paths = HashSet::new();
        for dir in &dirs {
            if !guest_paths.insert(dir.guest.clone()) {
                bail!("duplicate Wasm tool WASI guest path: {}", dir.guest);
            }
        }

        let mut merged_env = Vec::with_capacity(env.len() + inherit_env.len());
        for key in inherit_env {
            if key.is_empty() {
                bail!("--tool-wasi-env-inherit key must not be empty");
            }
            let value = std::env::var(key)
                .with_context(|| format!("inherit environment variable {key}"))?;
            set_env_pair(&mut merged_env, key.clone(), value);
        }
        for spec in env {
            let (key, value) = parse_env_pair(spec)?;
            set_env_pair(&mut merged_env, key, value);
        }

        Ok(Self {
            dirs,
            env: merged_env,
            fuel,
            output_limit,
        })
    }

    pub fn has_capabilities(
        rw_dirs: &[String],
        ro_dirs: &[String],
        env: &[String],
        inherit_env: &[String],
    ) -> bool {
        !rw_dirs.is_empty() || !ro_dirs.is_empty() || !env.is_empty() || !inherit_env.is_empty()
    }
}

impl WasmTool {
    pub fn load_all(specs: &[String], options: &WasmToolOptions) -> Result<Vec<Self>> {
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        let mut seen = HashSet::new();
        let mut tools = Vec::with_capacity(specs.len());

        for spec in specs {
            let (name, path) = parse_tool_spec(spec)?;
            if !seen.insert(name.clone()) {
                bail!("duplicate --tool name: {name}");
            }
            let path = std::fs::canonicalize(&path)
                .with_context(|| format!("canonicalize Wasm tool {}", path.display()))?;
            let module = Module::from_file(&engine, &path)
                .with_context(|| format!("compile Wasm tool {}", path.display()))?;
            let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
            preview1::add_to_linker_sync(&mut linker, |ctx| ctx)?;
            let pre = linker
                .instantiate_pre(&module)
                .with_context(|| format!("link Wasm tool {}", path.display()))?;
            tools.push(Self {
                name,
                path,
                engine: engine.clone(),
                pre,
                options: options.clone(),
            });
        }

        Ok(tools)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn invoke(&self, args: Value) -> Result<Value> {
        let request = serde_json::json!({ "name": &self.name, "args": args });
        let stdin = serde_json::to_vec(&request)?;
        let stdout = MemoryOutputPipe::new(self.options.output_limit);
        let stderr = MemoryOutputPipe::new(self.options.output_limit);

        let mut builder = WasiCtxBuilder::new();
        builder
            .allow_blocking_current_thread(true)
            .arg(self.path.to_string_lossy())
            .arg(&self.name)
            .stdin(MemoryInputPipe::new(stdin))
            .stdout(stdout.clone())
            .stderr(stderr.clone());

        for (key, value) in &self.options.env {
            builder.env(key, value);
        }
        for dir in &self.options.dirs {
            let dir_perms = if dir.read_only {
                DirPerms::READ
            } else {
                DirPerms::all()
            };
            let file_perms = if dir.read_only {
                FilePerms::READ
            } else {
                FilePerms::all()
            };
            builder
                .preopened_dir(&dir.host, &dir.guest, dir_perms, file_perms)
                .with_context(|| format!("preopen {} as {}", dir.host.display(), dir.guest))?;
        }

        let wasi = builder.build_p1();
        let mut store = Store::new(&self.engine, wasi);
        store.set_fuel(self.options.fuel)?;

        let instance = self.pre.instantiate(&mut store)?;
        let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        let status = match start.call(&mut store, ()) {
            Ok(()) => 0,
            Err(err) => {
                if let Some(exit) = err.downcast_ref::<I32Exit>() {
                    exit.0
                } else {
                    let stderr_text = pipe_text(&stderr);
                    if stderr_text.trim().is_empty() {
                        return Err(err)
                            .with_context(|| format!("Wasm tool {} trapped", self.name));
                    }
                    return Err(err).with_context(|| {
                        format!(
                            "Wasm tool {} trapped; stderr: {}",
                            self.name,
                            stderr_text.trim()
                        )
                    });
                }
            }
        };

        let stdout_bytes = stdout.contents();
        let stdout_len = stdout_bytes.len();
        let stdout_text = String::from_utf8_lossy(&stdout_bytes).into_owned();
        let stderr_text = pipe_text(&stderr);
        if status != 0 {
            if stderr_text.trim().is_empty() {
                bail!("Wasm tool {} exited with status {status}", self.name);
            }
            bail!(
                "Wasm tool {} exited with status {status}; stderr: {}",
                self.name,
                stderr_text.trim()
            );
        }

        match parse_tool_stdout(&self.name, &stdout_text) {
            Ok(value) => Ok(value),
            Err(err) if stdout_len >= self.options.output_limit => Err(err).with_context(|| {
                format!(
                    "Wasm tool {} stdout may have reached output limit of {} bytes",
                    self.name, self.options.output_limit
                )
            }),
            Err(err) => Err(err),
        }
    }
}

fn parse_tool_spec(spec: &str) -> Result<(String, PathBuf)> {
    let (name, path) = spec
        .split_once('=')
        .ok_or_else(|| anyhow!("--tool must use NAME=WASM syntax: {spec}"))?;
    let name = name.trim();
    let path = path.trim();
    if name.is_empty() {
        bail!("--tool name must not be empty: {spec}");
    }
    if name.starts_with("__") || name.starts_with("fs_") || name.starts_with("net_") {
        bail!("--tool name {name} is reserved");
    }
    if path.is_empty() {
        bail!("--tool path must not be empty: {spec}");
    }
    Ok((name.to_string(), PathBuf::from(path)))
}

fn parse_wasi_dir(spec: &str, read_only: bool) -> Result<WasiDir> {
    let (host, guest) = if let Some(idx) = spec.rfind(':') {
        let (host, guest) = spec.split_at(idx);
        let guest = &guest[1..];
        if is_windows_drive_path(spec, idx) {
            (spec, "/host")
        } else if guest.starts_with('/') || guest == "." || guest.starts_with("./") {
            (host, guest)
        } else {
            bail!(
                "invalid WASI preopen guest path {:?}: expected absolute path, '.', or './path'",
                guest
            );
        }
    } else {
        (spec, "/host")
    };

    if host.is_empty() {
        bail!("WASI preopen host path must not be empty: {spec}");
    }
    if guest.is_empty() {
        bail!("WASI preopen guest path must not be empty: {spec}");
    }

    let host = std::fs::canonicalize(host)
        .with_context(|| format!("canonicalize WASI preopen host path {host}"))?;
    Ok(WasiDir {
        host,
        guest: guest.to_string(),
        read_only,
    })
}

fn parse_env_pair(spec: &str) -> Result<(String, String)> {
    let (key, value) = spec
        .split_once('=')
        .ok_or_else(|| anyhow!("--tool-wasi-env must use KEY=VALUE syntax: {spec}"))?;
    if key.is_empty() {
        bail!("--tool-wasi-env key must not be empty: {spec}");
    }
    Ok((key.to_string(), value.to_string()))
}

fn set_env_pair(env: &mut Vec<(String, String)>, key: String, value: String) {
    if let Some((_, existing)) = env
        .iter_mut()
        .find(|(existing_key, _)| existing_key == &key)
    {
        *existing = value;
    } else {
        env.push((key, value));
    }
}

fn is_windows_drive_path(spec: &str, colon_idx: usize) -> bool {
    colon_idx == 1
        && spec
            .as_bytes()
            .first()
            .map(|b| b.is_ascii_alphabetic())
            .unwrap_or(false)
        && spec
            .as_bytes()
            .get(2)
            .map(|b| *b == b'/' || *b == b'\\')
            .unwrap_or(false)
}

fn pipe_text(pipe: &MemoryOutputPipe) -> String {
    String::from_utf8_lossy(&pipe.contents()).into_owned()
}

fn parse_tool_stdout(name: &str, stdout: &str) -> Result<Value> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return Ok(Value::Null);
    }

    let value: Value = serde_json::from_str(trimmed)
        .with_context(|| format!("Wasm tool {name} wrote non-JSON stdout"))?;
    if let Some(object) = value.as_object() {
        if object.len() == 1 {
            if let Some(result) = object.get("result") {
                return Ok(result.clone());
            }
            if let Some(error) = object.get("error") {
                if let Some(message) = error.as_str() {
                    bail!("Wasm tool {name}: {message}");
                }
                bail!("Wasm tool {name}: {error}");
            }
        }
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    fn tempdir(label: &str) -> TempDir {
        tempfile::Builder::new()
            .prefix(&format!("hl-wasm-tools-{label}-"))
            .tempdir()
            .unwrap()
    }

    fn default_options() -> WasmToolOptions {
        WasmToolOptions::from_cli(&[], &[], &[], &[], 1_000_000, 4096).unwrap()
    }

    fn options_with_limits(fuel: u64, output_limit: usize) -> WasmToolOptions {
        WasmToolOptions::from_cli(&[], &[], &[], &[], fuel, output_limit).unwrap()
    }

    fn load_tool(name: &str, wasm: Vec<u8>, options: WasmToolOptions) -> WasmTool {
        let dir = tempdir(name);
        let path = dir.path().join(format!("{name}.wasm"));
        fs::write(&path, wasm).unwrap();
        let specs = vec![format!("{name}={}", path.display())];
        let mut tools = WasmTool::load_all(&specs, &options).unwrap();
        assert_eq!(tools.len(), 1);
        tools.remove(0)
    }

    fn err_string(err: anyhow::Error) -> String {
        format!("{err:#}")
    }

    fn encode_u32(mut value: u32, out: &mut Vec<u8>) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            out.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    fn encode_i32(mut value: i32, out: &mut Vec<u8>) {
        loop {
            let byte = (value as u8) & 0x7f;
            value >>= 7;
            let done = (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0);
            if done {
                out.push(byte);
                break;
            }
            out.push(byte | 0x80);
        }
    }

    fn encode_i64(mut value: i64, out: &mut Vec<u8>) {
        loop {
            let byte = (value as u8) & 0x7f;
            value >>= 7;
            let done = (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0);
            if done {
                out.push(byte);
                break;
            }
            out.push(byte | 0x80);
        }
    }

    fn push_name(out: &mut Vec<u8>, name: &str) {
        encode_u32(name.len() as u32, out);
        out.extend_from_slice(name.as_bytes());
    }

    fn push_section(module: &mut Vec<u8>, id: u8, payload: Vec<u8>) {
        module.push(id);
        encode_u32(payload.len() as u32, module);
        module.extend(payload);
    }

    fn func_type(params: &[u8], results: &[u8]) -> Vec<u8> {
        let mut out = vec![0x60];
        encode_u32(params.len() as u32, &mut out);
        out.extend_from_slice(params);
        encode_u32(results.len() as u32, &mut out);
        out.extend_from_slice(results);
        out
    }

    fn i32_const(out: &mut Vec<u8>, value: i32) {
        out.push(0x41);
        encode_i32(value, out);
    }

    fn i64_const(out: &mut Vec<u8>, value: i64) {
        out.push(0x42);
        encode_i64(value, out);
    }

    fn i32_store(out: &mut Vec<u8>) {
        out.push(0x36);
        encode_u32(2, out);
        encode_u32(0, out);
    }

    fn i32_load(out: &mut Vec<u8>) {
        out.push(0x28);
        encode_u32(2, out);
        encode_u32(0, out);
    }

    fn call(out: &mut Vec<u8>, index: u32) {
        out.push(0x10);
        encode_u32(index, out);
    }

    fn drop_value(out: &mut Vec<u8>) {
        out.push(0x1a);
    }

    fn end(out: &mut Vec<u8>) {
        out.push(0x0b);
    }

    fn function_body(mut instructions: Vec<u8>) -> Vec<u8> {
        let mut body = vec![0x00];
        if !instructions.ends_with(&[0x0b]) {
            instructions.push(0x0b);
        }
        body.extend(instructions);
        body
    }

    fn module(
        types: Vec<Vec<u8>>,
        imports: Vec<(&str, &str, u32)>,
        functions: Vec<u32>,
        export_start_index: u32,
        memory: bool,
        bodies: Vec<Vec<u8>>,
        data: Vec<(u32, Vec<u8>)>,
    ) -> Vec<u8> {
        let mut module = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];

        let mut type_payload = Vec::new();
        encode_u32(types.len() as u32, &mut type_payload);
        for ty in types {
            type_payload.extend(ty);
        }
        push_section(&mut module, 1, type_payload);

        if !imports.is_empty() {
            let mut import_payload = Vec::new();
            encode_u32(imports.len() as u32, &mut import_payload);
            for (module_name, field_name, type_index) in imports {
                push_name(&mut import_payload, module_name);
                push_name(&mut import_payload, field_name);
                import_payload.push(0x00);
                encode_u32(type_index, &mut import_payload);
            }
            push_section(&mut module, 2, import_payload);
        }

        let mut function_payload = Vec::new();
        encode_u32(functions.len() as u32, &mut function_payload);
        for type_index in functions {
            encode_u32(type_index, &mut function_payload);
        }
        push_section(&mut module, 3, function_payload);

        if memory {
            let memory_payload = vec![0x01, 0x00, 0x01];
            push_section(&mut module, 5, memory_payload);
        }

        let mut export_payload = Vec::new();
        encode_u32(if memory { 2 } else { 1 }, &mut export_payload);
        push_name(&mut export_payload, "_start");
        export_payload.push(0x00);
        encode_u32(export_start_index, &mut export_payload);
        if memory {
            push_name(&mut export_payload, "memory");
            export_payload.push(0x02);
            encode_u32(0, &mut export_payload);
        }
        push_section(&mut module, 7, export_payload);

        let mut code_payload = Vec::new();
        encode_u32(bodies.len() as u32, &mut code_payload);
        for body in bodies {
            encode_u32(body.len() as u32, &mut code_payload);
            code_payload.extend(body);
        }
        push_section(&mut module, 10, code_payload);

        if !data.is_empty() {
            let mut data_payload = Vec::new();
            encode_u32(data.len() as u32, &mut data_payload);
            for (offset, bytes) in data {
                data_payload.push(0x00);
                i32_const(&mut data_payload, offset as i32);
                end(&mut data_payload);
                encode_u32(bytes.len() as u32, &mut data_payload);
                data_payload.extend(bytes);
            }
            push_section(&mut module, 11, data_payload);
        }

        module
    }

    fn stdout_module(bytes: &[u8]) -> Vec<u8> {
        let mut instructions = Vec::new();
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 64);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 12);
        i32_const(&mut instructions, bytes.len() as i32);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 4);
        call(&mut instructions, 0);
        drop_value(&mut instructions);
        end(&mut instructions);

        module(
            vec![
                func_type(&[0x7f, 0x7f, 0x7f, 0x7f], &[0x7f]),
                func_type(&[], &[]),
            ],
            vec![("wasi_snapshot_preview1", "fd_write", 0)],
            vec![1],
            1,
            true,
            vec![function_body(instructions)],
            vec![(64, bytes.to_vec())],
        )
    }

    fn stdin_echo_module() -> Vec<u8> {
        let mut instructions = Vec::new();
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 64);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 12);
        i32_const(&mut instructions, 2048);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 0);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 4);
        call(&mut instructions, 0);
        drop_value(&mut instructions);
        i32_const(&mut instructions, 16);
        i32_const(&mut instructions, 64);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 20);
        i32_const(&mut instructions, 4);
        i32_load(&mut instructions);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 16);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 24);
        call(&mut instructions, 1);
        drop_value(&mut instructions);
        end(&mut instructions);

        module(
            vec![
                func_type(&[0x7f, 0x7f, 0x7f, 0x7f], &[0x7f]),
                func_type(&[], &[]),
            ],
            vec![
                ("wasi_snapshot_preview1", "fd_read", 0),
                ("wasi_snapshot_preview1", "fd_write", 0),
            ],
            vec![1],
            2,
            true,
            vec![function_body(instructions)],
            Vec::new(),
        )
    }

    fn env_value_module(key: &str, value_len: usize) -> Vec<u8> {
        let mut instructions = Vec::new();
        i32_const(&mut instructions, 32);
        i32_const(&mut instructions, 128);
        call(&mut instructions, 0);
        drop_value(&mut instructions);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 128 + key.len() as i32 + 1);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 12);
        i32_const(&mut instructions, value_len as i32);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 4);
        call(&mut instructions, 1);
        drop_value(&mut instructions);
        end(&mut instructions);

        module(
            vec![
                func_type(&[0x7f, 0x7f], &[0x7f]),
                func_type(&[0x7f, 0x7f, 0x7f, 0x7f], &[0x7f]),
                func_type(&[], &[]),
            ],
            vec![
                ("wasi_snapshot_preview1", "environ_get", 0),
                ("wasi_snapshot_preview1", "fd_write", 1),
            ],
            vec![2],
            2,
            true,
            vec![function_body(instructions)],
            Vec::new(),
        )
    }

    fn preopen_read_file_module(path: &[u8]) -> Vec<u8> {
        let mut instructions = Vec::new();
        i32_const(&mut instructions, 3);
        i32_const(&mut instructions, 0);
        i32_const(&mut instructions, 64);
        i32_const(&mut instructions, path.len() as i32);
        i32_const(&mut instructions, 0);
        i64_const(&mut instructions, 2);
        i64_const(&mut instructions, 0);
        i32_const(&mut instructions, 0);
        i32_const(&mut instructions, 4);
        call(&mut instructions, 0);
        drop_value(&mut instructions);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 128);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 12);
        i32_const(&mut instructions, 512);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 4);
        i32_load(&mut instructions);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 20);
        call(&mut instructions, 1);
        drop_value(&mut instructions);
        i32_const(&mut instructions, 24);
        i32_const(&mut instructions, 128);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 28);
        i32_const(&mut instructions, 20);
        i32_load(&mut instructions);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 24);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 32);
        call(&mut instructions, 2);
        drop_value(&mut instructions);
        end(&mut instructions);

        module(
            vec![
                func_type(
                    &[0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7e, 0x7e, 0x7f, 0x7f],
                    &[0x7f],
                ),
                func_type(&[0x7f, 0x7f, 0x7f, 0x7f], &[0x7f]),
                func_type(&[], &[]),
            ],
            vec![
                ("wasi_snapshot_preview1", "path_open", 0),
                ("wasi_snapshot_preview1", "fd_read", 1),
                ("wasi_snapshot_preview1", "fd_write", 1),
            ],
            vec![2],
            3,
            true,
            vec![function_body(instructions)],
            vec![(64, path.to_vec())],
        )
    }

    fn no_output_module() -> Vec<u8> {
        module(
            vec![func_type(&[], &[])],
            Vec::new(),
            vec![0],
            0,
            false,
            vec![function_body(vec![0x0b])],
            Vec::new(),
        )
    }

    fn infinite_loop_module() -> Vec<u8> {
        let instructions = vec![0x03, 0x40, 0x0c, 0x00, 0x0b, 0x0b];
        module(
            vec![func_type(&[], &[])],
            Vec::new(),
            vec![0],
            0,
            false,
            vec![function_body(instructions)],
            Vec::new(),
        )
    }

    fn stderr_exit_module(stderr: &[u8], code: i32) -> Vec<u8> {
        let mut instructions = Vec::new();
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 64);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 12);
        i32_const(&mut instructions, stderr.len() as i32);
        i32_store(&mut instructions);
        i32_const(&mut instructions, 2);
        i32_const(&mut instructions, 8);
        i32_const(&mut instructions, 1);
        i32_const(&mut instructions, 4);
        call(&mut instructions, 0);
        drop_value(&mut instructions);
        i32_const(&mut instructions, code);
        call(&mut instructions, 1);
        end(&mut instructions);

        module(
            vec![
                func_type(&[0x7f, 0x7f, 0x7f, 0x7f], &[0x7f]),
                func_type(&[0x7f], &[]),
                func_type(&[], &[]),
            ],
            vec![
                ("wasi_snapshot_preview1", "fd_write", 0),
                ("wasi_snapshot_preview1", "proc_exit", 1),
            ],
            vec![2],
            2,
            true,
            vec![function_body(instructions)],
            vec![(64, stderr.to_vec())],
        )
    }

    fn unknown_import_module() -> Vec<u8> {
        let mut instructions = Vec::new();
        call(&mut instructions, 0);
        end(&mut instructions);
        module(
            vec![func_type(&[], &[])],
            vec![("host", "missing", 0)],
            vec![0],
            1,
            false,
            vec![function_body(instructions)],
            Vec::new(),
        )
    }

    #[test]
    fn parse_tool_spec_accepts_valid_name_and_path() {
        let (name, path) = parse_tool_spec(" greet = ./handler.wasm ").unwrap();
        assert_eq!(name, "greet");
        assert_eq!(path, PathBuf::from("./handler.wasm"));
    }

    #[test]
    fn parse_tool_spec_rejects_invalid_or_reserved_names() {
        for spec in [
            "missing_equals",
            "=handler.wasm",
            "__hl_exit=handler.wasm",
            "__dispatch=handler.wasm",
            "fs_read=handler.wasm",
            "net_socket=handler.wasm",
            "greet=",
            "greet=   ",
        ] {
            assert!(parse_tool_spec(spec).is_err(), "{spec} should fail");
        }
    }

    #[test]
    fn cli_options_parse_wasi_dirs_env_and_limits() {
        let rw = tempdir("rw");
        let ro = tempdir("ro");
        let opts = WasmToolOptions::from_cli(
            &[format!("{}:/rw", rw.path().display())],
            &[format!("{}:/ro", ro.path().display())],
            &["A=B".to_string(), "EMPTY=".to_string()],
            &[],
            123,
            456,
        )
        .unwrap();

        assert_eq!(opts.fuel, 123);
        assert_eq!(opts.output_limit, 456);
        assert_eq!(
            opts.env,
            vec![("A".into(), "B".into()), ("EMPTY".into(), "".into())]
        );
        assert_eq!(opts.dirs.len(), 2);
        assert_eq!(opts.dirs[0].host, fs::canonicalize(rw.path()).unwrap());
        assert_eq!(opts.dirs[0].guest, "/rw");
        assert!(!opts.dirs[0].read_only);
        assert_eq!(opts.dirs[1].host, fs::canonicalize(ro.path()).unwrap());
        assert_eq!(opts.dirs[1].guest, "/ro");
        assert!(opts.dirs[1].read_only);
    }

    #[test]
    fn cli_options_default_wasi_guest_path_to_host() {
        let dir = tempdir("default-guest");
        let opts =
            WasmToolOptions::from_cli(&[dir.path().display().to_string()], &[], &[], &[], 1, 1)
                .unwrap();
        assert_eq!(opts.dirs[0].guest, "/host");
    }

    #[test]
    fn cli_options_use_last_explicit_env_value_for_duplicate_keys() {
        let opts = WasmToolOptions::from_cli(
            &[],
            &[],
            &[
                "A=first".to_string(),
                "B=only".to_string(),
                "A=second".to_string(),
            ],
            &[],
            1,
            1,
        )
        .unwrap();
        assert_eq!(
            opts.env,
            vec![("A".into(), "second".into()), ("B".into(), "only".into())]
        );
    }

    #[test]
    fn cli_options_reject_invalid_values() {
        let dir = tempdir("invalid-options");
        assert!(WasmToolOptions::from_cli(&[], &[], &[], &[], 0, 1).is_err());
        assert!(WasmToolOptions::from_cli(&[], &[], &[], &[], 1, 0).is_err());
        assert!(
            WasmToolOptions::from_cli(&[], &[], &["NO_EQUALS".to_string()], &[], 1, 1).is_err()
        );
        assert!(WasmToolOptions::from_cli(&[], &[], &["=value".to_string()], &[], 1, 1).is_err());
        assert!(WasmToolOptions::from_cli(&[], &[], &[], &["".to_string()], 1, 1).is_err());
        assert!(WasmToolOptions::from_cli(
            &[format!("{}:relative", dir.path().display())],
            &[],
            &[],
            &[],
            1,
            1,
        )
        .is_err());
        assert!(WasmToolOptions::from_cli(
            &[
                format!("{}:/dup", dir.path().display()),
                format!("{}:/dup", dir.path().display())
            ],
            &[],
            &[],
            &[],
            1,
            1,
        )
        .is_err());
    }

    #[test]
    fn has_capabilities_detects_any_wasi_capability_flag() {
        assert!(!WasmToolOptions::has_capabilities(&[], &[], &[], &[]));
        assert!(WasmToolOptions::has_capabilities(
            &[".".into()],
            &[],
            &[],
            &[]
        ));
        assert!(WasmToolOptions::has_capabilities(
            &[],
            &[".".into()],
            &[],
            &[]
        ));
        assert!(WasmToolOptions::has_capabilities(
            &[],
            &[],
            &["A=B".into()],
            &[]
        ));
        assert!(WasmToolOptions::has_capabilities(
            &[],
            &[],
            &[],
            &["PATH".into()]
        ));
    }

    #[test]
    fn parse_tool_stdout_handles_raw_values_and_envelopes() {
        assert_eq!(parse_tool_stdout("t", "").unwrap(), Value::Null);
        assert_eq!(parse_tool_stdout("t", "  \n ").unwrap(), Value::Null);
        assert_eq!(parse_tool_stdout("t", "42").unwrap(), json!(42));
        assert_eq!(
            parse_tool_stdout("t", "{\"result\":{\"ok\":true}}").unwrap(),
            json!({"ok": true})
        );
        assert_eq!(
            parse_tool_stdout("t", "{\"result\":1,\"extra\":2}").unwrap(),
            json!({"result": 1, "extra": 2})
        );
        assert!(
            err_string(parse_tool_stdout("t", "{\"error\":\"boom\"}").unwrap_err())
                .contains("Wasm tool t: boom")
        );
        assert!(err_string(parse_tool_stdout("t", "not json").unwrap_err())
            .contains("wrote non-JSON stdout"));
    }

    #[test]
    fn load_all_rejects_duplicate_names_invalid_wasm_and_unknown_imports() {
        let dir = tempdir("load-errors");
        let ok = dir.path().join("ok.wasm");
        let bad = dir.path().join("bad.wasm");
        let unknown = dir.path().join("unknown.wasm");
        fs::write(&ok, no_output_module()).unwrap();
        fs::write(&bad, b"not wasm").unwrap();
        fs::write(&unknown, unknown_import_module()).unwrap();
        let options = default_options();

        let duplicate_err = WasmTool::load_all(
            &[format!("a={}", ok.display()), format!("a={}", ok.display())],
            &options,
        )
        .err()
        .expect("duplicate tool name should fail");
        assert!(err_string(duplicate_err).contains("duplicate --tool name"));

        let invalid_err = WasmTool::load_all(&[format!("bad={}", bad.display())], &options)
            .err()
            .expect("invalid wasm should fail");
        assert!(err_string(invalid_err).contains("compile Wasm tool"));

        let link_err = WasmTool::load_all(&[format!("unknown={}", unknown.display())], &options)
            .err()
            .expect("unknown import should fail");
        assert!(err_string(link_err).contains("link Wasm tool"));
    }

    #[test]
    fn invoke_passes_dispatch_request_on_stdin() {
        let tool = load_tool("echo_req", stdin_echo_module(), default_options());
        let result = tool.invoke(json!({"n": 7, "s": "hello"})).unwrap();
        assert_eq!(result["name"], "echo_req");
        assert_eq!(result["args"], json!({"n": 7, "s": "hello"}));
    }

    #[test]
    fn invoke_passes_configured_environment() {
        let key = "HL_WASM_JSON";
        let value = r#"{"result":"env-ok"}"#;
        let options =
            WasmToolOptions::from_cli(&[], &[], &[format!("{key}={value}")], &[], 1_000_000, 4096)
                .unwrap();
        let tool = load_tool("env", env_value_module(key, value.len()), options);
        let result = tool.invoke(json!({})).unwrap();
        assert_eq!(result, json!("env-ok"));
    }

    #[test]
    fn invoke_can_read_explicit_read_only_preopen() {
        let root = tempdir("preopen-read");
        fs::write(root.path().join("answer.json"), br#"{"result":"file-ok"}"#).unwrap();
        let options = WasmToolOptions::from_cli(
            &[],
            &[format!("{}:.", root.path().display())],
            &[],
            &[],
            1_000_000,
            4096,
        )
        .unwrap();
        let tool = load_tool(
            "read_preopen",
            preopen_read_file_module(b"answer.json"),
            options,
        );
        let result = tool.invoke(json!({})).unwrap();
        assert_eq!(result, json!("file-ok"));
    }

    #[test]
    fn invoke_unwraps_result_envelope() {
        let tool = load_tool(
            "answer",
            stdout_module(br#"{"result":{"ok":true,"answer":42}}"#),
            default_options(),
        );
        let result = tool.invoke(json!({"ignored": true})).unwrap();
        assert_eq!(result, json!({"ok": true, "answer": 42}));
    }

    #[test]
    fn invoke_returns_null_for_empty_stdout() {
        let tool = load_tool("empty", no_output_module(), default_options());
        let result = tool.invoke(json!({})).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn invoke_converts_error_envelope_to_handler_error() {
        let tool = load_tool(
            "fail",
            stdout_module(br#"{"error":"boom"}"#),
            default_options(),
        );
        let err = tool.invoke(json!({})).unwrap_err();
        assert!(err_string(err).contains("Wasm tool fail: boom"));
    }

    #[test]
    fn invoke_reports_nonzero_exit_status() {
        let tool = load_tool("exit_only", stderr_exit_module(b"", 9), default_options());
        let err = tool.invoke(json!({})).unwrap_err();
        assert!(err_string(err).contains("exited with status 9"));
    }

    #[test]
    fn invoke_includes_stderr_for_nonzero_exit() {
        let tool = load_tool(
            "stderr_exit",
            stderr_exit_module(b"details from stderr", 7),
            default_options(),
        );
        let err = tool.invoke(json!({})).unwrap_err();
        let msg = err_string(err);
        assert!(msg.contains("exited with status 7"));
        assert!(msg.contains("details from stderr"));
    }

    #[test]
    fn invoke_traps_when_stdout_exceeds_limit() {
        let tool = load_tool(
            "too_much",
            stdout_module(br#"{"result":"this is too long"}"#),
            options_with_limits(1_000_000, 8),
        );
        let err = tool.invoke(json!({})).unwrap_err();
        let msg = err_string(err);
        assert!(msg.contains("stdout may have reached output limit of 8 bytes"));
        assert!(msg.contains("wrote non-JSON stdout"));
    }

    #[test]
    fn invoke_traps_when_fuel_is_exhausted() {
        let tool = load_tool(
            "spin",
            infinite_loop_module(),
            options_with_limits(10, 1024),
        );
        let err = tool.invoke(json!({})).unwrap_err();
        let msg = err_string(err);
        assert!(msg.contains("Wasm tool spin trapped"));
        assert!(msg.contains("fuel"));
    }
}
