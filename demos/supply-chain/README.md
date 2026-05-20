# Supply Chain Attack Demo — Mini Shai-Hulud

A safe, educational demonstration of a supply chain attack modeled on
[Mini Shai-Hulud](https://thehackernews.com/2026/05/mini-shai-hulud-worm-compromises.html)
(TeamPCP, May 2026), showing how Hyperlight micro-VM isolation contains it
**even when the guest has realistic capabilities**.

## Background

Mini Shai-Hulud compromised 500+ packages across npm, PyPI, and PHP — including
TanStack, Mistral AI, Guardrails AI, and AntV — affecting 518M+ cumulative
downloads. The attack used typosquatted/hijacked packages to:

1. **Steal credentials** — SSH keys, AWS creds, `.env` files, 80+ env vars
2. **Probe cloud metadata** — AWS IMDS, Azure IMDS, GCP metadata (169.254.169.254)
3. **Exfiltrate data** — triple-redundant C2 (custom domain, Session Protocol, GitHub dead drops)
4. **Install persistence** — Claude Code `SessionStart` hooks, VS Code `runOn`, LaunchAgents
5. **Self-propagate** — used stolen npm tokens to poison other packages

## What this demo does

A fake typosquatted package (`reqeusts` instead of `requests`) simulates the
attack payload. A victim application imports it, triggering the malicious code.
The victim app also does legitimate work: reads input from the workspace,
fetches data from `example.com`, and writes output.

**Bare metal** — the attack succeeds: secrets stolen, exfiltrated, persistence
installed. Legitimate work also succeeds.

**Hyperlight sandbox (scoped)** — the guest has real capabilities (`--mount`
for directory access, `--net-allow example.com` for network). Legitimate work
succeeds, but every malicious action is blocked by Hyperlight's security model:

- **Credential theft → BLOCKED** — `~/.ssh/id_rsa`, `~/.aws/credentials`, etc.
  are outside the mounted directory. The guest has filesystem access, but only
  to the scoped directory (no HOME, no `/etc/passwd`).
- **Environment variables → NOT SET** — host environment is never forwarded to
  the guest. `AWS_ACCESS_KEY_ID`, `GITHUB_TOKEN`, etc. are absent.
- **C2 exfiltration → BLOCKED** — loopback addresses (127.0.0.0/8) are
  **always denied** regardless of network policy. The C2 server at
  `127.0.0.1:8080` is unreachable.
- **Cloud metadata → BLOCKED** — link-local addresses (169.254.0.0/16) are
  **always denied**. AWS IMDS, Azure IMDS, and GCP metadata at 169.254.169.254
  are all blocked — a hardcoded safety net, not a user-configurable policy.
- **Persistence → BLOCKED** — host dotfiles (`~/.claude/settings.json`,
  `~/.bashrc`) are not mounted. The guest's ramfs is destroyed on exit.

This is **active defense**, not just absence of resources. An empty Docker
container blocks the same attacks, but only because it has nothing to attack.
Hyperlight blocks them because its network policy engine and filesystem sandbox
enforce scoped access even when capabilities are granted.

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
