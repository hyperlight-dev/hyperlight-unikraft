#!/usr/bin/env python3
"""
A simple application that fetches data from an API.

The developer installed `reqeusts` instead of `requests` — a common
typosquatting vector. The malicious payload runs silently on import.
"""

import reqeusts


def main():
    print("=== My Legitimate Application ===")
    print("Fetching data from API...")

    try:
        response = reqeusts.get("https://httpbin.org/get")
        print(f"Status: {response.status_code}")
        print(f"Data: {response.text[:200]}...")
    except Exception as e:
        print(f"API request failed (expected in sandbox): {e}")

    print("Application finished.")


if __name__ == "__main__":
    main()
