# Supply Chain Attack Demo — Mini Shai-Hulud

A safe, educational reproduction of the [Mini Shai-Hulud](https://thehackernews.com/2026/05/mini-shai-hulud-worm-compromises.html) supply chain attack (TeamPCP, May 2026), run inside a Hyperlight micro-VM with `hluk`.

## Background

Mini Shai-Hulud compromised 500+ packages across npm, PyPI, and PHP — including TanStack, Mistral AI, Guardrails AI, and AntV — affecting 518M+ cumulative downloads. The payload:

1. Steals credentials — SSH keys, AWS creds, `.env` files, 80+ env vars
2. Probes cloud metadata — AWS IMDS, Azure IMDS, GCP metadata (169.254.169.254)
3. Exfiltrates to triple-redundant C2 (custom domain, Session Protocol, GitHub dead drops)
4. Installs persistence — Claude Code `SessionStart` hooks, VS Code `runOn`, LaunchAgents
5. Self-propagates using stolen npm/PyPI publish tokens

## The demo

A typosquatted package (`reqeusts` instead of `requests`) carries a simulated version of this payload. A victim app imports it — triggering the stealer — and then does legitimate work: reads input from the workspace, fetches from `example.com`, writes output.

Two runs, same code:

**Bare metal** — credentials stolen, C2 exfiltration sent, persistence installed, cloud metadata probed. Legitimate work also succeeds.

**Hyperlight micro-VM** (`--mount ./guest:/host --net-allow example.com`) — legitimate work succeeds, every attack phase is blocked:

| Phase | What happens | Why |
|---|---|---|
| Credential theft | `~/.ssh/id_rsa`, `~/.aws/credentials` → BLOCKED | `hluk`'s cap-std mount scopes filesystem access to the mounted directory. No HOME, no `/etc/passwd`. |
| Env var harvesting | `AWS_ACCESS_KEY_ID`, `GITHUB_TOKEN` → NOT SET | The host environment is never forwarded into the guest. |
| Cloud metadata | 169.254.169.254 → BLOCKED | Every network policy blocks link-local (169.254.0.0/16, fe80::/10) unconditionally. |
| C2 exfiltration | 127.0.0.1:8080 → BLOCKED (`Errno 13`) | `--net-allow`/`--net-block` also block loopback (127.0.0.0/8, ::1). |
| Persistence | `~/.claude/settings.json`, `~/.bashrc` → BLOCKED | Host dotfiles are outside the mount; the guest ramfs is destroyed on exit. |

The isolation is hardware-enforced: the guest runs in its own VM address space, not a shared-kernel sandbox.

## Running the demo

Everything is driven by the demo's `Justfile`. Build the python rootfs once from the repo root (`just build-rootfs python`); the demo builds `hluk` for you.

Linux only (KVM for the VM, Docker to build the rootfs).

### Bare metal (attack succeeds)

Creates a temporary HOME with planted fake secrets, starts a local C2 listener, runs the victim app, then cleans everything up.

```bash
cd demos/supply-chain
just bare-metal
```

### Hyperlight sandbox (attack contained)

```bash
cd demos/supply-chain

# Scoped sandbox: mount + allowed egress to example.com. Attack still contained.
just run

# Minimal sandbox: mount only, no network.
just run-minimal
```

Scoped-sandbox output (abridged):

```
-- Phase 4: Exfiltration ------------------------------------------
  status: BLOCKED -- <urlopen error [Errno 13] Permission denied>
...
  RESULT: Attack CONTAINED by Hyperlight sandbox
=== My Legitimate Application ===
  input: Hello from the host filesystem
  fetch: HTTP 200 (559 bytes)
  output: /host/workspace/output.txt written
```

### C2 server (standalone)

To watch exfiltrated data arrive in real time (useful for split-terminal demos):

```bash
just c2          # or: python3 c2_server.py
```

## File structure

```
supply-chain/
├── reqeusts/          # Typosquatted package
│   ├── __init__.py    # Triggers the payload on import
│   ├── api.py         # Fake requests-like API surface
│   └── stealer.py     # Attack payload (6 phases)
├── victim_app.py      # App that imports reqeusts + does legitimate work
├── c2_server.py       # Fake C2 server (receives exfiltrated data)
├── bare-metal/run.sh  # Bare-metal demo (plants secrets, runs the attack)
└── Justfile           # hluk-based sandbox run commands
```

`guest/` is assembled at run time from the sources above and is git-ignored.

## Safety

- All "secrets" are planted test data (fake SSH keys, AWS example credentials).
- The C2 server binds `127.0.0.1` only.
- Persistence targets a temporary HOME (bare-metal) or is unreachable (sandbox).
- No real credentials are read, stored, or transmitted.
