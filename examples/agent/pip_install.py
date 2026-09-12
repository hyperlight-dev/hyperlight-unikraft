"""Live pip install — requires --net flag for network access.

Usage:
    hluk run agent --net examples/agent/pip_install.py

This demonstrates runtime package installation, enabled by hardware
VM isolation.  The agent rootfs includes SSL support and CA certs so
pip can connect to PyPI over HTTPS.
"""
import subprocess
import sys

# Install a small package at runtime
pkg = "six"
print(f"Installing {pkg}...")
r = subprocess.run(
    [sys.executable, "-m", "pip", "install",
     "--target", "/tmp/pkgs", "--quiet", pkg],
    capture_output=True, text=True, timeout=60,
)

if r.returncode == 0:
    sys.path.insert(0, "/tmp/pkgs")
    import six
    print(f"Success! {pkg} {six.__version__} installed and imported")
else:
    print(f"pip install failed (rc={r.returncode}) — is --net enabled?")
    if r.stderr:
        print(r.stderr[:500])
