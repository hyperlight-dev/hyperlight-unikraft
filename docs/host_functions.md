# Host functions and attack surface

This document describes how the **hyperlight-unikraft** host exposes capabilities to a Unikraft guest on the Hyperlight platform, what is enabled by default, and what security boundaries apply.

Implementation lives in [`host/src/lib.rs`](../host/src/lib.rs). Guest-side callers include [`lib/hostfs`](https://github.com/unikraft/unikraft/tree/plat-hyperlight/lib/hostfs) (filesystem) and [`lib/hostsock`](https://github.com/unikraft/unikraft/tree/plat-hyperlight/lib/hostsock) (networking), both on the Unikraft `plat-hyperlight` branch.

---

## Summary

| Surface | Default | Enable with |
|---------|---------|-------------|
| `__dispatch` host function | Always registered at VM boot | (automatic) |
| `__hl_exit`, `__hl_sleep` | Always available via `__dispatch` | (automatic) |
| `fs_*` tools | **Off** | `--mount HOST[:GUEST]` (repeatable) |
| `net_*` tools | **Off** | `--net`, `--net-allow`, or `--net-block` |
| Inbound listen | **Off** | `--port PORT` (requires network enabled) |
| Custom tools | **Off** | `--tool NAME=WASM` with `wasm-host-fns` or `SandboxBuilder::tool()` |

With **no flags**, the guest cannot reach the host filesystem or network through dispatch. Only internal plumbing (`__hl_exit`, `__hl_sleep`) is wired.

---

## Architecture

```
Guest (Unikraft)
  ├─ lib/hostfs  (when --mount) ──► hyperlight_hcall() ──► __dispatch ──► fs_* handlers ──► FsSandbox ──► host files
  ├─ lib/hostsock (when --net)  ──► hyperlight_hcall() ──► __dispatch ──► net_* handlers ──► host sockets
  └─ /dev/hcall or direct hcall ──► hyperlight_hcall() ──► __dispatch ──► named tool in ToolRegistry
        │
        ▼
Hyperlight host (hyperlight-unikraft)
  ToolRegistry::dispatch(payload)  →  JSON tool payloads in binary async frames
```

There is **one** guest-to-host RPC channel for tools: the Hyperlight host function **`__dispatch`**, registered when the sandbox is created. All tool names are looked up in a host-side `ToolRegistry`.

### End-to-end example: `time.sleep(2)` in a Python guest

1. The Python app calls `time.sleep(2)`.
2. The Hyperlight Python driver patches the call to open `/dev/hcall` and write JSON: `{"name": "__hl_sleep", "args": {"ns": 2000000000}}`.
3. Unikraft's Hyperlight platform routes writes to `/dev/hcall` to execute the `__dispatch` host function call.
4. The host parses the call to `__dispatch`, looks up `__hl_sleep` in the `ToolRegistry`, and calls the handler.
5. `__hl_sleep` executes, and the response flows back via the normal hyperlight-core host function mechanism (writing the response into shared memory via the input shared buffer).

---

## `__dispatch` wire format

Two request formats are accepted:

### Cooperative (async) request

Used when calling an **async tool** (e.g. `net_connect`, `net_recv`,
`__hl_sleep`) from a Unikraft guest, which always runs under the
cooperative poll pump. `hyperlight_hcall()` places the ordinary JSON tool
request in a versioned binary frame:

| Offset | Size | Field |
|--------|------|-------|
| 0 | 4 | Magic `HLAF` |
| 4 | 1 | Version (`1`) |
| 5 | 1 | Kind: request `1`, result `2`, pending `3`, batch `4` |
| 6 | 2 | Reserved (`0`) |
| 8 | 8 | Little-endian request ID, or entry count for a batch |
| 16 | 4 | Little-endian payload length |
| 20 | variable | JSON payload or binary batch entries |

The request ID is a guest-assigned nonzero `u64`. An immediate result frame
contains the normal `{"result":...}` or `{"error":...}` JSON payload. A pending
frame has no payload; the guest parks on its ID.

The next `poll` byte-vector argument carries a batch frame. Its payload is a
sequence of `{request_id: u64, payload_length: u32, JSON payload}` entries.
Framing makes IDs and result boundaries unambiguous without parsing JSON
control fields. `/dev/hcall` still accepts and returns JSON; the kernel hides
these internal frames.

A duplicate or zero request ID is rejected atomically without overwriting an
existing task or queuing new work. Each sandbox accepts at most 1024 concurrent
async tasks.

### Legacy (sync) request

Used for synchronous tools (e.g. `fs_read`, `net_socket`, `__hl_exit`):

```json
{"name": "<tool_name>", "args": <json_value>}
```

Calling an **async** tool in legacy format is supported: a caller that did not
frame its request has no parked operation for a later `poll` batch to resume,
so the host resolves the tool inline and returns the result in the same
blocking call.

**Success response:**

```json
{"result": <json_value>}
```

**Error response:**

```json
{"error": "<message>"}
```

Unknown tools, malformed JSON or frames, invalid IDs, duplicate IDs, and
pending-limit violations become framed `{"error":"..."}` results. The host
does not panic on bad guest input.

**Debug:** set `HL_DISPATCH_DEBUG=1` in the environment to log each request/response on stderr.

---

## CLI: enabling host capabilities

```bash
hyperlight-unikraft KERNEL [--initrd CPIO] [options] [-- APP_ARGS...]
```

| Flag | Effect |
|------|--------|
| `--mount HOST[:GUEST]` | Preopen `HOST` at guest path `GUEST` (default `/host`). Registers all `fs_*` tools. Repeat for multiple mounts. |
| `--net` | Outbound networking: register `net_*` tools with **allow-all** policy (still blocks loopback and link-local). |
| `--net-allow HOST_OR_IP` | Allow-list outbound destinations (implies `--net`). Repeatable. |
| `--net-block HOST_OR_IP` | Block-list; all other destinations allowed (implies `--net`). Mutually exclusive with `--net-allow`. |
| `--port PORT` | Allow `net_bind` / listen on `PORT` (implies `--net`). Without `--port`, outbound-only: bind is rejected. |
| `--tool NAME=WASM` | With the Cargo feature `wasm-host-fns`, registers `WASM` as a host-side WASIp1 custom tool named `NAME`. Repeatable. |
| `--tool-wasi-dir HOST[:GUEST]` | Preopens a read-write host directory for every CLI Wasm tool. Default guest path is `/host`. Repeatable. |
| `--tool-wasi-dir-ro HOST[:GUEST]` | Preopens a read-only host directory for every CLI Wasm tool. Default guest path is `/host`. Repeatable. |
| `--tool-wasi-env KEY=VALUE` | Sets an environment variable for every CLI Wasm tool. Repeatable. |
| `--tool-wasi-env-inherit KEY` | Copies one host environment variable into every CLI Wasm tool. Repeatable. |
| `--tool-wasi-fuel FUEL` | Sets the instruction-fuel budget for each call to every CLI Wasm tool. Default `100000000`. |
| `--tool-wasi-output-limit SIZE` | Caps captured stdout and stderr for each call to every CLI Wasm tool. Default `1Mi`. |

`--tool-wasi-*` flags configure the Wasmtime/WASI sandbox for Wasm custom tools only. They do not expose the guest `--mount` filesystem, and they do not change the `fs_*` handlers used by `lib/hostfs`.

The CLI currently applies the same Wasm filesystem, environment, fuel, and output settings to every `--tool` registered in one invocation. If tools need different permissions or limits, do not grant the union to all handlers; that requires a narrower per-tool configuration surface or a separate host integration.

**Mount rules (host-enforced before boot):**

- `GUEST` must be absolute (e.g. `/data`, `/host`).
- Cannot use reserved guest paths: `/`, `/bin`, `/dev`, `/proc`, `/sys`, `/usr`.
- Duplicate `GUEST` paths are rejected.

**Initrd metadata** (not dispatch, but host-to-guest config): cmdline (`HLCMDLN`), mount table (`HLHSMNT`), optional wall-clock seed (`HLWALL0`). See `prepend_cmdline_to_initrd()` in `host/src/lib.rs`.

---

## Always-registered internal tools

Registered for every sandbox, regardless of `--mount` / `--net`:

### `__hl_exit`

Guest driver exit hook.

| Arg | Type | Description |
|-----|------|-------------|
| `code` | number (optional) | Exit code; default `1` |

**Result:** `{}`  
Host stores the code in an atomic read after the VM run.

### `__hl_sleep`

Sleep on the host thread (used by guest drivers).

| Arg | Type | Description |
|-----|------|-------------|
| `ns` | number (optional) | Nanoseconds; capped at **60 s** |

**Result:** `{}`  
Can be cancelled via `SleepCancel` when tearing down the sandbox.

**Async:** `__hl_sleep` is an async tool. In poll builds `hyperlight_hcall()`
supplies its binary request frame; in non-poll builds it is called in legacy
format and the host resolves it inline (blocking the vCPU thread).

---

## Filesystem tools (`fs_*`)

Registered when at least one `--mount` / `Preopen` is configured. Paths in `args.path` are **guest paths** (e.g. `/host/project/file.txt`). The host routes to the longest matching preopen prefix, then resolves under that host directory via [`FsSandbox`](../host/src/lib.rs).

**Sandbox guarantees:**

- Host directory is canonicalized at mount setup.
- `..` and symlink chains cannot escape the mount root (hop limit 40).
- Mount root itself cannot be deleted (`fs_unlink`).

| Tool | Args | Result (success) |
|------|------|------------------|
| `fs_read` | `path` | `{"text": "<utf-8 string>"}` — whole file, max **16 MiB** |
| `fs_write` | `path`, `text`, `append?` | `{"bytes_written": N}` — max **16 MiB** text |
| `fs_read_bytes` | `path`, `offset?`, `len?` | `{"data": "<base64>", "eof": bool, "bytes_read": N}` — default `len` 65536, max **16 MiB** |
| `fs_write_bytes` | `path`, `data` (base64), `offset?`, `append?` | `{"bytes_written": N}` — max **16 MiB** decoded |
| `fs_list` | `path` (required; must match a preopen prefix, e.g. `/host`) | `{"entries": [{"name", "is_dir", "is_file", "is_symlink"}, ...]}` — max **100 000** entries |
| `fs_stat` | `path` | `{"size", "is_dir", "is_file", "mtime_ns", "atime_ns"}` |
| `fs_truncate` | `path`, `length` | `{}` — max length **1 GiB** |
| `fs_mkdir` | `path`, `parents?` | `{}` |
| `fs_unlink` | `path` | `{}` — file or empty dir; not mount root |

Errors are normalized to Linux-style `std::io::Error` wording where possible so the guest `lib/hostfs` can map them to POSIX errno (see `normalize_fs_error()`).

**Guest integration:** With `CONFIG_LIBHOSTFS`, unmodified POSIX under the mount point uses these same tools via `hostfs_rpc_*` → `hyperlight_hcall()`. See [`lib/hostfs`](https://github.com/unikraft/unikraft/tree/plat-hyperlight/lib/hostfs).

---

## Network tools (`net_*`)

Registered only when a [`NetworkPolicy`](../host/src/lib.rs) is set (`--net`, `--net-allow`, or `--net-block`).

Sockets are host-side (`socket2`); the guest sees opaque numeric **`fd`** handles (per-sandbox table, max **1024** sockets, **30 s** read/write/connect timeout).

**Common arg:** `addr` + `port` for sockaddr (IPv4/IPv6 string + port).

| Tool | Purpose |
|------|---------|
| `net_socket` | `family`, `type`, `protocol` → `{"fd"}` |
| `net_connect` | Outbound connect (policy-checked) |
| `net_bind` | Bind; requires `--port` allowlist entry |
| `net_listen` | Listen after bind |
| `net_accept` | Accept; returns new `fd` + peer |
| `net_send` / `net_recv` | Stream/datagram I/O — payload max **1 MiB decoded bytes** (base64-encoded on wire) |
| `net_sendto` / `net_recvfrom` | Datagram with address (policy on destination) |
| `net_close` | Close host socket |
| `net_shutdown` | Shutdown (best-effort) |
| `net_setsockopt` / `net_getsockopt` | Limited socket options |
| `net_getpeername` / `net_getsockname` | Peer/local address |

### Network policy

| Policy | Behavior |
|--------|----------|
| **AllowAll** (`--net`) | Any outbound IP except **loopback** and **link-local** (blocks cloud metadata-style addresses). |
| **AllowList** (`--net-allow`) | Only listed IPs/hostnames; hostnames re-resolved at check time; DNS to resolver IPs on port **53** allowed for listed resolvers + common public DNS. |
| **BlockList** (`--net-block`) | Block listed targets; others allowed (same loopback/link-local deny for all policies). |

**Inbound:** `--port` adds a listen-port allowlist. Without it, `net_bind` fails with "no --port specified" (outbound-only mode).

---

## Custom tools

**CLI Wasm tools:** build with the optional feature and pass one or more `--tool` flags. The `examples/echo-wasm-host-fxn` module is a minimal echo handler used by `examples/python-tools`:

```bash
cargo build --manifest-path host/Cargo.toml --features wasm-host-fns --bin hyperlight-unikraft
rustup target add wasm32-wasip1
cargo build --manifest-path examples/echo-wasm-host-fxn/Cargo.toml --release --target wasm32-wasip1
hyperlight-unikraft kernel --initrd app.cpio \
    --tool echo=examples/echo-wasm-host-fxn/target/wasm32-wasip1/release/echo-wasm-host-fxn.wasm
```

Each `--tool NAME=WASM` module is compiled and linked before VM boot, then invoked as a fresh WASIp1 command for every matching guest `__dispatch` call. The handler receives the existing dispatch request on stdin:

```json
{"name":"echo","args":<json_value>}
```

The handler writes JSON to stdout. It may write either a raw JSON result value or the normal dispatch envelope:

```json
{"result":<json_value>}
```

```json
{"error":"message"}
```

A raw value is treated as the tool result. A single-key `result` envelope is unwrapped. A single-key `error` envelope becomes the outer `__dispatch` error response. Empty stdout returns JSON null.

Wasm tools are separate from the built-in `fs_*` and `net_*` dispatch handlers. `--mount` controls what the guest can access through `lib/hostfs`; `--tool-wasi-dir*` controls what the host-side Wasm handler can access through its own WASI filesystem view.

WASI capabilities are denied by default except stdio used for the protocol, clocks, and random. Use `--tool-wasi-dir`, `--tool-wasi-dir-ro`, `--tool-wasi-env`, and `--tool-wasi-env-inherit` to grant explicit filesystem and environment access to handlers. These grants and the `--tool-wasi-fuel` / `--tool-wasi-output-limit` settings apply to every CLI Wasm tool registered by the process. Tool names beginning with `__`, `fs_`, or `net_` are reserved.

**Why WASIp1 command modules today?** The current CLI maps one `--tool NAME=WASM` flag to one tool name and one fresh handler invocation. WASIp1 keeps that ABI small: JSON request on stdin, JSON response on stdout, no long-lived reactor state, and broad language/toolchain support. Component-model or reactor-style handlers could support a future `--tools component.wasm` shape with multiple exported tools and auto-registration, but that would need a separate registration and lifecycle model; it is not the current ABI.

**Library:**

```rust
Sandbox::builder("kernel")
    .tool("my_tool", |args| Ok(serde_json::json!({"ok": true})))
    .build()?;
```

`SandboxBuilder::tool()` handlers receive the inner `args` JSON value from the dispatch request; the registry has already matched the outer `name`. Handler return values become the `result` field in the outer `__dispatch` response, and handler errors become `{"error": "..."}`.

---

## Resource limits (host-enforced)

| Limit | Value |
|-------|-------|
| Dispatch payload | 64 MiB |
| `fs_read` / `fs_read_bytes` | 16 MiB per call |
| `fs_write` / `fs_write_bytes` | 16 MiB per call |
| `fs_truncate` length | 1 GiB |
| `fs_list` entries | 100 000 |
| `net_send` / `net_sendto` | 1 MiB decoded bytes |
| `__hl_sleep` | 60 s |
| Wasm tool fuel | 100 000 000 instructions per call by default; configurable with `--tool-wasi-fuel`; same value applies to every CLI Wasm tool |
| Wasm tool stdout / stderr | 1 MiB each per call by default; configurable with `--tool-wasi-output-limit`; same value applies to every CLI Wasm tool |
| Open host sockets | 1024 per sandbox |
| AllowList learned DNS IPs | 256 |

---

## Security and attack surface

**Default posture:** The guest is a micro-VM with no host FS and no host network unless the operator opts in. That matches Hyperlight's embed-in-application threat model: the host application chooses what to expose per sandbox.

**When `--mount` is used:**

- The guest can read/write/delete files under the preopened host trees only.
- Path traversal and symlink escape are rejected host-side; operators should still mount **non-sensitive** directories and treat the guest as **untrusted**.
- Large reads/writes are capped to limit guest-driven host memory use.

**When `--net` is used:**

- The guest uses the **host network stack**; policy filters destinations but does not isolate traffic from other host processes.
- Loopback and link-local connects are denied to reduce access to host services and instance metadata.
- Allow-list mode still permits DNS to configured resolvers on port 53 so resolvers can be used without listing every CDN IP.
- Inbound listen requires explicit `--port`; otherwise bind is denied.

**`__dispatch` itself:**

- Always registered: internal tools cannot be disabled without code changes.
- A compromised guest can invoke any **registered** tool name; do not register powerful custom tools unless needed.
- Payload size is capped; malformed JSON fails closed with an error response.

**When `--tool` is used with `wasm-host-fns`:**

- Handler code runs on the host inside Wasmtime, not inside the Unikraft VM.
- WASI filesystem and environment access are capability-based and off unless explicitly granted with `--tool-wasi-*` flags.
- CLI Wasm capability and limit flags apply to every registered Wasm tool; avoid combining handlers with different privilege needs in one invocation.
- Fuel limits bound Wasm instruction execution, but do not turn filesystem operations into a full wall-clock timeout.
- Handlers are untrusted code from the host operator's filesystem; only load modules you intend to grant these capabilities to.

**Not exposed via dispatch:** Host shell, arbitrary process spawn, unrestricted host `exec`, or kernel modules — only the tools listed above.

**Operators should:** Use minimal flags, allow-lists over `--net` where possible, mount least-privilege directories, and run guests with the smallest initrd/runtime required.

---

## Programmatic API

Same behavior as the CLI via [`Sandbox`](../host/src/lib.rs) / [`SandboxBuilder`](../host/src/lib.rs):

```rust
use hyperlight_unikraft::{AllowList, NetworkPolicy, Preopen, Sandbox};

let mut sbox = Sandbox::builder("./kernel")
    .initrd_file("./app.cpio")
    .preopen(Preopen::new("./workspace", "/host")?)
    .network(NetworkPolicy::AllowList(AllowList::from_hosts(&["api.example.com"])?))
    .listen_ports(hyperlight_unikraft::ListenPorts::from_ports([8080]))
    .build()?;
```

See crate docs in `host/src/lib.rs` for snapshot/restore and `call_run()`.
