"""Print host-provided environment variables."""
import os

# These are set by the host via --env KEY=VALUE (CLI) or
# GuestConfig::set_env_vars() (library API).
my_var = os.environ.get("MY_VAR", "NOT_SET")
debug = os.environ.get("DEBUG", "NOT_SET")
greeting = os.environ.get("GREETING", "NOT_SET")

print(f"MY_VAR={my_var}")
print(f"DEBUG={debug}")
print(f"GREETING={greeting}")
