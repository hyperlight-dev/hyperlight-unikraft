"""HTTP GET using urllib WITHOUT an explicit timeout.

Same as urllib_get.py but omits the timeout= argument to urlopen().
This exercises the code path used by mxc, where the Unikraft guest
kernel relies on the idle thread's halt_irq callback to poll sockets
via __hl_sleep rather than an explicit timeout-driven poll cycle.
"""
import urllib.request
import sys

URL = "http://example.com/"

print(f"Fetching {URL} (no timeout) ...")
try:
    with urllib.request.urlopen(URL) as resp:
        body = resp.read().decode("utf-8", errors="replace")
        print(f"Status: {resp.status}")
        print(f"Body length: {len(body)} bytes")
        if "Example Domain" in body:
            print("SUCCESS: urllib GET (no timeout) worked!")
        else:
            print("WARNING: unexpected body content")
except Exception as e:
    print(f"FAILED: {e}")
    sys.exit(1)
