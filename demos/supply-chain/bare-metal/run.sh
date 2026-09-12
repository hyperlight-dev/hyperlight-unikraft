#!/bin/bash
# Run the supply chain attack demo on bare metal.
#
# This script:
# 1. Creates a temporary HOME with planted fake secrets
# 2. Creates a workspace with input data (legitimate workload)
# 3. Starts the C2 server in the background
# 4. Runs the victim app (stealer triggers on import)
# 5. Shows the modified persistence files and workspace output
# 6. Cleans everything up
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEMO_DIR="$(dirname "$SCRIPT_DIR")"

# ── Create temp home with fake secrets ───────────────────────────────
DEMO_HOME=$(mktemp -d /tmp/supply-chain-demo.XXXXXX)
trap 'kill $C2_PID 2>/dev/null; rm -rf "$DEMO_HOME"' EXIT

mkdir -p "$DEMO_HOME/.ssh"
cat > "$DEMO_HOME/.ssh/id_rsa" << 'EOF'
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA3Tz2mr7SZiAMfQyuvBjM9O+FAKE+KEY+DATA+DO+NOT+USE
Lp6j+TSmHLzT6Yb/n/DEMO/ONLY/NOT/REAL/3Tz2mr7SZiAMfQyuvBjM9Ooer
QyuvBjM9O+er3Tz2mr7SZiAMfQyuvBjM9O+er3Tz2mr7SZiAMfQyuvBjM9O+e
-----END RSA PRIVATE KEY-----
EOF

mkdir -p "$DEMO_HOME/.aws"
cat > "$DEMO_HOME/.aws/credentials" << 'EOF'
[default]
aws_access_key_id = AKIAIOSFODNN7EXAMPLE
aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
region = us-east-1
EOF

cat > "$DEMO_HOME/.env" << 'EOF'
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
NPM_TOKEN=npm_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
DATABASE_URL=postgres://admin:s3cretPassw0rd@prod-db.internal:5432/main
STRIPE_SECRET_KEY=sk_FAKE_xxxxxxxxxxxxxxxxxxxxxxxxxxxx
EOF

mkdir -p "$DEMO_HOME/.claude"
cat > "$DEMO_HOME/.claude/settings.json" << 'EOF'
{
  "permissions": {
    "allow": ["Bash(git *)"],
    "deny": []
  }
}
EOF

touch "$DEMO_HOME/.bashrc"

# ── Create workspace with input data ─────────────────────────────────
WORKSPACE="$DEMO_HOME/workspace"
mkdir -p "$WORKSPACE"
echo "Hello from the host filesystem" > "$WORKSPACE/input.txt"

# ── Start C2 server ─────────────────────────────────────────────────
python3 "$DEMO_DIR/c2_server.py" &
C2_PID=$!
sleep 0.5

# ── Run the victim app ──────────────────────────────────────────────
echo ""
echo "========================================"
echo "  BARE-METAL RUN (no sandbox)"
echo "========================================"
echo ""

HOME="$DEMO_HOME" \
WORKSPACE="$WORKSPACE" \
SUPPLY_CHAIN_DEMO=1 \
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE \
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY \
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx \
C2_URL=http://127.0.0.1:8080/exfil \
PYTHONPATH="$DEMO_DIR" \
python3 "$DEMO_DIR/victim_app.py"

# ── Show persistence artifacts ───────────────────────────────────────
echo ""
echo "── Post-attack: ~/.claude/settings.json ─────────────────────"
cat "$DEMO_HOME/.claude/settings.json"
echo ""
echo ""
echo "── Post-attack: ~/.bashrc (last 3 lines) ────────────────────"
tail -3 "$DEMO_HOME/.bashrc"
echo ""

# ── Show workspace output ────────────────────────────────────────────
echo ""
echo "── Workspace output ─────────────────────────────────────────"
cat "$WORKSPACE/output.txt" 2>/dev/null || echo "(no output written)"
echo ""
