//! hyperlight-unikraft: run Unikraft unikernels on the Hyperlight VMM
//!
//! ## Usage
//!
//! ```bash
//! hyperlight-unikraft <kernel> [--initrd <cpio>] [--memory <size>] [-- <app-args>]
//! ```

use anyhow::Result;
use clap::Parser;
use hyperlight_unikraft::{
    parse_memory, AllowList, BlockList, ListenPorts, NetworkPolicy, Preopen, Sandbox,
};
use std::path::PathBuf;

#[cfg(feature = "wasm-host-fns")]
mod wasm_host_fns;

#[derive(Parser, Debug)]
#[command(
    name = "hyperlight-unikraft",
    version,
    about = "Run Unikraft unikernels on Hyperlight"
)]
struct Args {
    /// Path to the Unikraft kernel binary
    kernel: PathBuf,

    /// Path to initrd/rootfs CPIO archive
    #[arg(long)]
    initrd: Option<PathBuf>,

    /// Memory allocation (e.g., 32Mi, 256Mi, 1Gi)
    #[arg(long, short = 'm', default_value = "32Mi")]
    memory: String,

    /// Stack size (e.g., 8Mi)
    #[arg(long, default_value = "8Mi")]
    stack: String,

    /// Quiet mode — suppress host-side status messages
    #[arg(long, short = 'q')]
    quiet: bool,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool",
        value_name = "NAME=WASM",
        help = "Register a WASIp1 module as a host tool"
    )]
    tool: Vec<String>,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool-wasi-dir",
        value_name = "HOST[:GUEST]",
        help = "Preopen a read-write host directory for Wasm tools"
    )]
    tool_wasi_dir: Vec<String>,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool-wasi-dir-ro",
        value_name = "HOST[:GUEST]",
        help = "Preopen a read-only host directory for Wasm tools"
    )]
    tool_wasi_dir_ro: Vec<String>,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool-wasi-env",
        value_name = "KEY=VALUE",
        help = "Set an environment variable for Wasm tools"
    )]
    tool_wasi_env: Vec<String>,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool-wasi-env-inherit",
        value_name = "KEY",
        help = "Inherit one host environment variable into Wasm tools"
    )]
    tool_wasi_env_inherit: Vec<String>,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool-wasi-fuel",
        default_value_t = 100_000_000,
        value_name = "FUEL",
        help = "Fuel units available to each Wasm tool call"
    )]
    tool_wasi_fuel: u64,

    #[cfg(feature = "wasm-host-fns")]
    #[arg(
        long = "tool-wasi-output-limit",
        default_value = "1Mi",
        value_name = "SIZE",
        help = "Maximum stdout or stderr captured from one Wasm tool call"
    )]
    tool_wasi_output_limit: String,

    /// Preopen a host directory for the guest's sandboxed filesystem.
    ///
    /// Syntax: `HOST_DIR[:GUEST_PATH]`. When `GUEST_PATH` is omitted the
    /// default is `/host`. Repeatable — pass `--mount` multiple times to
    /// expose several host directories at distinct guest mount points.
    ///
    /// lib/hostfs in the guest auto-mounts `HOST_DIR` at `GUEST_PATH`;
    /// unmodified POSIX calls (open/read/write/stat/…) route through
    /// the FsSandbox tool handlers. Guest-supplied paths are resolved
    /// relative to the matching `HOST_DIR` and any escape (via `..` or
    /// symlinks) is rejected host-side. `GUEST_PATH` must be absolute
    /// and cannot shadow the kernel's own reserved directories
    /// (`/bin`, `/dev`, `/proc`, `/sys`, `/usr`, `/`).
    #[arg(long, value_name = "HOST[:GUEST]")]
    mount: Vec<String>,

    /// Enable guest networking. Without this flag, the guest has no
    /// network access.
    #[arg(long)]
    net: bool,

    /// Restrict guest networking to the listed hosts/IPs.
    /// Implies --net. Hostnames are resolved at sandbox creation time.
    /// Repeatable: `--net-allow api.github.com --net-allow 10.0.0.1`.
    #[arg(
        long = "net-allow",
        value_name = "HOST_OR_IP",
        conflicts_with = "net_block"
    )]
    net_allow: Vec<String>,

    /// Block the listed hosts/IPs; all other destinations are allowed.
    /// Implies --net. Hostnames are resolved at sandbox creation time.
    /// Repeatable: `--net-block evil.com --net-block 10.0.0.1`.
    #[arg(
        long = "net-block",
        value_name = "HOST_OR_IP",
        conflicts_with = "net_allow"
    )]
    net_block: Vec<String>,

    /// Allow the guest to bind (listen) on the given port. Implies --net.
    /// Without this flag, `net_bind` is rejected (outbound-only).
    /// Repeatable: `--port 8080 --port 3000`.
    #[arg(long, value_name = "PORT")]
    port: Vec<u16>,

    /// Run the application N additional times via snapshot/restore + call.
    /// The first run always happens. --repeat=2 means 3 total runs.
    #[arg(long, default_value = "0")]
    repeat: u32,

    /// Inline code snippet. The guest interpreter is invoked with
    /// `["-c", <code>]` — works for Python, `sh`, `node -e` style
    /// interpreters that treat `-c` as "run the next arg as code".
    /// The host handles all argparse-escape quoting internally, so your
    /// code can contain arbitrary spaces, quotes, newlines, etc.
    ///
    /// Conflicts with positional `-- <args>`.
    #[arg(long, short = 'e', conflicts_with = "app_args", value_name = "CODE")]
    exec: Option<String>,

    /// Application arguments (passed after --)
    #[arg(last = true)]
    app_args: Vec<String>,
}

/// Escape a string so that the guest-side `uk_argparse` tokenizer preserves
/// it as a single argv entry, regardless of embedded whitespace or quotes.
///
/// Wraps the string in `"..."` and backslash-escapes internal `\` / `"`.
/// The argparse rules then:
///   - open-quote on the leading `"` (stripped),
///   - `\"` → literal `"` (preserved inside the in-quote region),
///   - `\\` → literal `\`,
///   - whitespace inside the quote is preserved,
///   - close-quote on the final `"` (stripped).
fn argparse_escape(code: &str) -> String {
    let mut out = String::with_capacity(code.len() + 4);
    out.push('"');
    for ch in code.chars() {
        if ch == '\\' || ch == '"' {
            out.push('\\');
        }
        out.push(ch);
    }
    out.push('"');
    out
}

fn main() -> Result<()> {
    let t0 = std::time::Instant::now();
    let args = Args::parse();

    let heap_size = parse_memory(&args.memory)?;
    let stack_size = parse_memory(&args.stack)?;

    if !args.quiet {
        eprintln!("hyperlight-unikraft v{}", env!("CARGO_PKG_VERSION"));
        eprintln!("Kernel: {:?}", args.kernel);
        if let Some(ref p) = args.initrd {
            eprintln!("Initrd: {:?}", p);
        }
        eprintln!("Memory: {heap_size} B, Stack: {stack_size} B");
    }

    let preopens: Vec<Preopen> = args
        .mount
        .iter()
        .map(|spec| Preopen::parse_cli(spec))
        .collect::<Result<_>>()?;

    // Reject duplicate guest paths before the VM boots — two mounts
    // on the same guest path would silently shadow each other.
    for i in 0..preopens.len() {
        for j in (i + 1)..preopens.len() {
            if preopens[i].guest_path == preopens[j].guest_path {
                return Err(anyhow::anyhow!(
                    "duplicate --mount guest path: {:?}",
                    preopens[i].guest_path
                ));
            }
        }
    }

    if !args.quiet {
        for p in &preopens {
            eprintln!("Preopened: {:?} -> {} (guest)", p.host_dir, p.guest_path);
        }
    }

    // Phase 1: evolve — boots kernel, loads ELF, signals ready.
    // Zero-copy initrd via map_file_cow. If --mount is set, the directory is
    // preopened: the FsSandbox handlers get wired in and lib/hostfs in the
    // guest mounts it at the configured guest path.
    // --exec CODE is sugar for `-- -c <CODE>`, but with the argparse
    // escaping applied so the user doesn't have to think about it.
    let app_args: Vec<String> = match args.exec {
        Some(ref code) => vec!["-c".into(), argparse_escape(code)],
        None => args.app_args.clone(),
    };

    let has_ports = !args.port.is_empty();
    let network = if !args.net_allow.is_empty() {
        Some(NetworkPolicy::AllowList(AllowList::from_hosts(
            &args.net_allow,
        )?))
    } else if !args.net_block.is_empty() {
        Some(NetworkPolicy::BlockList(BlockList::from_hosts(
            &args.net_block,
        )?))
    } else if args.net || has_ports {
        Some(NetworkPolicy::AllowAll)
    } else {
        None
    };

    let listen_ports = if has_ports {
        Some(ListenPorts::from_ports(args.port.iter().copied()))
    } else {
        None
    };

    #[cfg(feature = "wasm-host-fns")]
    let wasm_tools = {
        if args.tool.is_empty()
            && wasm_host_fns::WasmToolOptions::has_capabilities(
                &args.tool_wasi_dir,
                &args.tool_wasi_dir_ro,
                &args.tool_wasi_env,
                &args.tool_wasi_env_inherit,
            )
        {
            return Err(anyhow::anyhow!(
                "--tool-wasi-* flags require at least one --tool"
            ));
        }
        if args.tool.is_empty() {
            Vec::new()
        } else {
            let output_limit = parse_memory(&args.tool_wasi_output_limit)?;
            let output_limit = usize::try_from(output_limit).map_err(|_| {
                anyhow::anyhow!(
                    "--tool-wasi-output-limit too large: {}",
                    args.tool_wasi_output_limit
                )
            })?;
            let options = wasm_host_fns::WasmToolOptions::from_cli(
                &args.tool_wasi_dir,
                &args.tool_wasi_dir_ro,
                &args.tool_wasi_env,
                &args.tool_wasi_env_inherit,
                args.tool_wasi_fuel,
                output_limit,
            )?;
            let tools = wasm_host_fns::WasmTool::load_all(&args.tool, &options)?;
            if !args.quiet {
                for tool in &tools {
                    eprintln!("Tool: {} -> {}", tool.name(), tool.path().display());
                }
            }
            tools
        }
    };

    let mut builder = Sandbox::builder(&args.kernel)
        .args(app_args)
        .heap_size(heap_size)
        .stack_size(stack_size);
    if let Some(ref p) = args.initrd {
        builder = builder.initrd_file(p);
    }
    for p in preopens {
        builder = builder.preopen(p);
    }
    if let Some(policy) = network {
        builder = builder.network(policy);
    }
    if let Some(ports) = listen_ports {
        builder = builder.listen_ports(ports);
    }
    #[cfg(feature = "wasm-host-fns")]
    for tool in wasm_tools {
        let name = tool.name().to_string();
        let tool = std::sync::Arc::new(tool);
        builder = builder.tool(&name, move |args| tool.invoke(args));
    }
    let mut sandbox = builder.build()?;
    let evolve_time = t0.elapsed();

    // Phase 2: restore + call — runs the application
    let total_runs = 1 + args.repeat;
    for i in 0..total_runs {
        let t_restore = std::time::Instant::now();
        sandbox.restore()?;
        let restore_time = t_restore.elapsed();

        let t_call = std::time::Instant::now();
        sandbox.call_run()?;
        let call_time = t_call.elapsed();

        if !args.quiet || args.repeat > 0 {
            eprintln!(
                "[run {}/{}] restore={:.1}ms call={:.1}ms",
                i + 1,
                total_runs,
                restore_time.as_secs_f64() * 1000.0,
                call_time.as_secs_f64() * 1000.0,
            );
        }
    }

    eprintln!(
        "[timing] evolve={:.1}ms total={:.1}ms",
        evolve_time.as_secs_f64() * 1000.0,
        t0.elapsed().as_secs_f64() * 1000.0,
    );
    Ok(())
}
