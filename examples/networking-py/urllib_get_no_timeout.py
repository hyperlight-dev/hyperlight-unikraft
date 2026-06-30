"""HTTP GET using urllib WITHOUT an explicit timeout.

Same as urllib_get.py but omits the timeout= argument to urlopen().
This exercises the code path used by mxc, where the Unikraft guest
kernel busy-polls net_poll with timeout_ms=0 and relies on the host
to detect read-readiness via select().
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
