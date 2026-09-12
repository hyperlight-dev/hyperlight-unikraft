"""Basic agent hello — verifies the agent rootfs works."""
import sys
import json

info = {
    "message": "Hello from the Hyperlight agent!",
    "python": sys.version.split()[0],
}

# Check which data science packages are available
for pkg in ["numpy", "pandas", "scipy", "sklearn", "matplotlib", "seaborn"]:
    try:
        mod = __import__(pkg)
        info[pkg] = getattr(mod, "__version__", "yes")
    except ImportError:
        pass

print(json.dumps(info, indent=2))
