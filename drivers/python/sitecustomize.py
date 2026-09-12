"""Fix sys.executable.

The elfloader sets argv[0] to the driver name ('hl_pydriver'), which
isn't a filesystem path.  Python uses argv[0] to find its executable,
so sys.executable ends up empty.
"""
import sys

if not sys.executable:
    sys.executable = "/usr/local/bin/python3"
