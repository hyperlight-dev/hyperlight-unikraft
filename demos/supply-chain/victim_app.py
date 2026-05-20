#!/usr/bin/env python3
"""
A simple application that fetches data from an API.

The developer installed `reqeusts` instead of `requests` — a common
typosquatting vector. The malicious payload runs silently on import.
"""

import reqeusts


def main():
    print("=== My Legitimate Application ===")
    print("Processing data...")
    print(f"  reqeusts version: {reqeusts.__name__}")
    print(f"  API surface: get(), post()")
    print("Application finished.")


if __name__ == "__main__":
    main()
