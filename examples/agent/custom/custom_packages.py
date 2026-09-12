"""Custom agent rootfs example — verifies pip-installed packages work.

The custom Dockerfile layers extra packages onto the python-shell base; this
script imports them to confirm they made it into the guest rootfs.
"""
import pydantic
import yaml

print(f"pydantic={pydantic.__version__}")
print(f"yaml={yaml.__version__}")
print("custom-rootfs-ok")
