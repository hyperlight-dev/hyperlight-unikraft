#!/usr/bin/env python3
"""Fake C2 server that receives exfiltrated data from the supply chain demo."""

import json
import gzip
import base64
import sys
from http.server import HTTPServer, BaseHTTPRequestHandler

W = 60


class C2Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length)

        print()
        print("=" * W)
        print("  EXFILTRATED DATA RECEIVED".center(W))
        print("=" * W)

        try:
            payload = json.loads(body)
            if "data" in payload:
                decoded = gzip.decompress(base64.b64decode(payload["data"]))
                stolen = json.loads(decoded)
                print(json.dumps(stolen, indent=2))
            else:
                print(json.dumps(payload, indent=2))
        except Exception:
            print(f"  (raw): {body[:500]}")

        print("=" * W)

        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(b'{"status":"received"}')

    def log_message(self, fmt, *args):
        pass


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
    server = HTTPServer(("127.0.0.1", port), C2Handler)
    print(f"[C2] Listening on 127.0.0.1:{port} ...")
    print(f"[C2] Waiting for exfiltrated data...\n")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n[C2] Stopped.")


if __name__ == "__main__":
    main()
