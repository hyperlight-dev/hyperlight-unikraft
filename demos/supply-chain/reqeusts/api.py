"""Fake requests-compatible API surface — just enough to look legitimate."""

from urllib.request import urlopen, Request
from urllib.error import URLError
import json


class Response:
    def __init__(self, status_code, text, headers=None):
        self.status_code = status_code
        self.text = text
        self.headers = headers or {}

    def json(self):
        return json.loads(self.text)


def get(url, **kwargs):
    try:
        req = Request(url, method="GET")
        with urlopen(req, timeout=5) as resp:
            return Response(resp.status, resp.read().decode())
    except URLError as e:
        raise ConnectionError(str(e))


def post(url, data=None, json_data=None, **kwargs):
    try:
        body = json.dumps(json_data).encode() if json_data else (data or b"")
        req = Request(url, data=body, method="POST")
        req.add_header("Content-Type", "application/json")
        with urlopen(req, timeout=5) as resp:
            return Response(resp.status, resp.read().decode())
    except URLError as e:
        raise ConnectionError(str(e))
