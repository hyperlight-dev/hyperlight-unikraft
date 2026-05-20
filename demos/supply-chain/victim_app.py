#!/usr/bin/env python3
"""
A simple application that fetches data from an API.

The developer installed `reqeusts` instead of `requests` -- a common
typosquatting vector. The malicious payload runs silently on import.
"""

import os
import reqeusts

WORKSPACE = os.environ.get("WORKSPACE", "/host/workspace")


def main():
    print("=== My Legitimate Application ===")

    # Read input from workspace
    input_path = os.path.join(WORKSPACE, "input.txt")
    try:
        with open(input_path) as f:
            data = f.read().strip()
        print(f"  input: {data}")
    except FileNotFoundError:
        data = "(no input file)"
        print(f"  input: {data}")
    except Exception as e:
        data = f"(error: {e})"
        print(f"  input: {data}")

    # Fetch from allowed external API
    try:
        resp = reqeusts.get("http://example.com/")
        print(f"  fetch: HTTP {resp.status_code} ({len(resp.text)} bytes)")
    except Exception as e:
        print(f"  fetch: FAILED ({e})")

    # Write output to workspace
    output_path = os.path.join(WORKSPACE, "output.txt")
    try:
        with open(output_path, "w") as f:
            f.write(f"Processed: {data}\n")
        print(f"  output: {output_path} written")
    except Exception as e:
        print(f"  output: FAILED ({e})")

    print("Application finished.")


if __name__ == "__main__":
    main()
