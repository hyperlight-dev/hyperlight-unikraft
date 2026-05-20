# Supply Chain Attack Demo — Mini Shai-Hulud

A safe, educational reproduction of the
[Mini Shai-Hulud](https://thehackernews.com/2026/05/mini-shai-hulud-worm-compromises.html)
supply chain attack (TeamPCP, May 2026) running inside a Hyperlight micro-VM.

## Background

Mini Shai-Hulud compromised 500+ packages across npm, PyPI, and PHP — including
TanStack, Mistral AI, Guardrails AI, and AntV — affecting 518M+ cumulative
downloads. The payload:

1. Steals credentials — SSH keys, AWS creds, `.env` files, 80+ env vars
2. Probes cloud metadata — AWS IMDS, Azure IMDS, GCP metadata (169.254.169.254)
3. Exfiltrates to triple-redundant C2 (custom domain, Session Protocol, GitHub dead drops)
4. Installs persistence — Claude Code `SessionStart` hooks, VS Code `runOn`, LaunchAgents
5. Self-propagates using stolen npm/PyPI publish tokens

## The demo

A typosquatted package (`reqeusts` instead of `requests`) carries a simulated
version of this payload. A victim app imports it — triggering the stealer — and
then does legitimate work: reads input from the workspace, fetches data from
`example.com`, writes output.

Two runs, same code:

**Bare metal** — credentials stolen, C2 exfiltration sent, persistence
installed, cloud metadata reached. Legitimate work also succeeds.

**Hyperlight micro-VM** (`--mount ./guest --net-allow example.com`) —
legitimate work succeeds, every attack phase is blocked:

| Phase | What happens | Why |
|---|---|---|
| Credential theft | `~/.ssh/id_rsa`, `~/.aws/credentials` → BLOCKED | `FsSandbox` scopes access to the mounted directory. No HOME, no `/etc/passwd`. |
| Env var harvesting | `AWS_ACCESS_KEY_ID`, `GITHUB_TOKEN` → NOT SET | Host environment is never forwarded into the guest. |
| Cloud metadata | 169.254.169.254 → BLOCKED | `NetworkPolicy` hardcodes a link-local (169.254.0.0/16) deny. |
| C2 exfiltration | 127.0.0.1:8080 → BLOCKED | `NetworkPolicy` hardcodes a loopback (127.0.0.0/8) deny. |
| Persistence | `~/.claude/settings.json`, `~/.bashrc` → BLOCKED | Host dotfiles are outside the mount. Guest ramfs is destroyed on exit. |

## Running the demo

### Prerequisites

Install `pyhl` (the Hyperlight Python runtime):

```bash
cargo install --path host --bin pyhl
```

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

# One-time setup — installs kernel + rootfs, warms Python snapshot
just setup

# Scoped sandbox: mount + network, attack still contained
just run

# Minimal sandbox: mount only, no network
just run-minimal
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
│   └── stealer.py         # Attack payload (7 phases)
├── victim_app.py          # App that imports reqeusts + does legitimate work
├── c2_server.py           # Fake C2 server (receives exfiltrated data)
├── bare-metal/
│   └── run.sh             # Bare-metal demo (plants secrets, runs attack)
└── hl-unikraft/
    └── Justfile           # pyhl-based setup + run commands
```

## Safety

- All "secrets" are planted test data (fake SSH keys, AWS example credentials)
- The C2 server is `localhost` only
- Persistence targets a temporary HOME directory (bare-metal) or doesn't exist (sandbox)
- No real credentials are read, stored, or transmitted
