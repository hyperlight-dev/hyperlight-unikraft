# Supply Chain Attack Demo — Mini Shai-Hulud

A safe, educational demonstration of a supply chain attack modeled on
[Mini Shai-Hulud](https://thehackernews.com/2026/05/mini-shai-hulud-worm-compromises.html)
(TeamPCP, May 2026), showing how Hyperlight micro-VM isolation contains it.

## Background

Mini Shai-Hulud compromised 500+ packages across npm, PyPI, and PHP — including
TanStack, Mistral AI, Guardrails AI, and AntV — affecting 518M+ cumulative
downloads. The attack used typosquatted/hijacked packages to:

1. **Steal credentials** — SSH keys, AWS creds, `.env` files, 80+ env vars
2. **Exfiltrate data** — triple-redundant C2 (custom domain, Session Protocol, GitHub dead drops)
3. **Install persistence** — Claude Code `SessionStart` hooks, VS Code `runOn`, LaunchAgents
4. **Self-propagate** — used stolen npm tokens to poison other packages

## What this demo does

A fake typosquatted package (`reqeusts` instead of `requests`) simulates the
attack payload. A victim application imports it, triggering the malicious code.

**Bare metal** — the attack succeeds: secrets are stolen, exfiltrated, persistence installed.

**Hyperlight sandbox** — every malicious action is blocked by the micro-VM's
default-deny security model:
- **Filesystem is isolated** — the guest runs on its own ramfs (from the CPIO
  initrd). Host directories are only visible if explicitly mounted with
  `--mount`, and even then access is scoped to that directory with path-escape
  prevention. The attacker's `~/.ssh/id_rsa`, `~/.aws/credentials`, etc. simply
  don't exist inside the VM.
- **Environment variables are compile-time only** — the guest kernel only has
  `PATH` and `LD_LIBRARY_PATH` (set in `kraft.yaml`). There is no `--env` flag;
  the host's environment is never forwarded. `AWS_ACCESS_KEY_ID`, `GITHUB_TOKEN`,
  etc. are all absent.
- **Networking is opt-in** — without `--net`, the guest has zero network access
  (`socket()` returns "Function not implemented"). Even with `--net`, outbound
  connections can be restricted to specific hosts via `--net-allow`.
- **Persistence is impossible** — the guest's ramfs is destroyed when the VM
  exits. There is no way to write to the host's `~/.claude/settings.json` or
  `~/.bashrc` unless the host explicitly mounts those paths.

## Running the demo

### Bare metal (attack succeeds)

The script creates a temporary HOME with planted fake secrets, starts a C2
listener, and runs the victim app. Everything is cleaned up on exit.

```bash
cd demos/supply-chain
./bare-metal/run.sh
```

### Hyperlight sandbox (attack contained)

```bash
cd demos/supply-chain/hl-unikraft

# One-time setup
just build     # Build or pull Unikraft kernel
just rootfs    # Build rootfs with Python + malicious package

# Run
just run       # Execute inside the sandbox
```

### C2 server (standalone)

To watch exfiltrated data arrive in real time (useful for split-terminal demos):

```bash
python3 c2_server.py
```

## File structure

```
supply-chain/
├── reqeusts/              # Typosquatted package
│   ├── __init__.py        # Triggers payload on import
│   ├── api.py             # Fake requests-like API surface
│   └── stealer.py         # Attack payload (6 phases)
├── victim_app.py          # Legitimate-looking app that imports reqeusts
├── c2_server.py           # Fake C2 server (receives exfiltrated data)
├── bare-metal/
│   └── run.sh             # Bare-metal demo (plants secrets, runs attack)
└── hl-unikraft/
    ├── Dockerfile          # Rootfs with Python + malicious package
    ├── kraft.yaml          # Unikraft kernel config
    └── Justfile            # Build + run commands
```

## Safety

- All "secrets" are planted test data (fake SSH keys, AWS example credentials)
- The C2 server is `localhost` only
- Persistence targets a temporary HOME directory (bare-metal) or doesn't exist (sandbox)
- No real credentials are read, stored, or transmitted
