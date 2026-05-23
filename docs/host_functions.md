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
| Custom tools | **Off** | `--enable-tools` + `SandboxBuilder::tool()` |

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
  ToolRegistry::dispatch(payload)  →  JSON in / JSON out
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

**Request** (UTF-8 JSON bytes, max **64 MiB**):

```json
{"name": "<tool_name>", "args": <json_value>}
```

**Success response:**

```json
{"result": <json_value>}
```

**Error response:**

```json
{"error": "<message>"}
```

Unknown tools, malformed JSON, and handler errors become `{"error": "..."}`. The host does not panic on bad guest input.

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
| `--enable-tools` | Enables custom tool registration. Registers a built-in `echo` tool (used by the `python-tools` example). Library users add their own tools via `SandboxBuilder::tool()`. |

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

**CLI:** `--enable-tools` registers a built-in `echo` tool (returns `args` unchanged) used by the [`python-tools` example](../examples/python-tools). The primary purpose of `--enable-tools` is to demonstrate custom host function registration via the API.

**Library:**

```rust
Sandbox::builder("kernel")
    .tool("my_tool", |args| Ok(serde_json::json!({"ok": true})))
    .build()?;
```

Custom handlers run with the same JSON request/response envelope as built-in tools.

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
