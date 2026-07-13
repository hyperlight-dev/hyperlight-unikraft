#!/usr/bin/env python3
"""A Microsoft Agent Framework agent using a REMOTE hosted model.

The agent is a real `agent_framework` Agent, backed by a custom ChatClient that
calls the GitHub Models inference API (OpenAI-compatible) over the network. It
runs inside a Hyperlight micro-VM (python-agent-driver / pyhl stack) with network
enabled; the API token is passed into the guest environment by `pyhl run --env`
(never baked into the image or written to disk).

Agent Framework provides `OpenAIChatCompletionClient` for normal Python apps,
but that client uses the async OpenAI SDK. This example instead uses a small
synchronous `urllib` client and steps the coroutine directly rather than via
`asyncio.run()`, keeping it free of an event loop (whose self-pipe needs
`socket.socketpair()`, absent from the non-networking base kernels) and portable
across every kernel in this repo.
"""

import json
import logging
import os
import urllib.error
import urllib.request
import warnings

# Keep the demo output clean.
warnings.filterwarnings("ignore")
logging.getLogger("agent_framework").setLevel(logging.ERROR)

from collections.abc import Awaitable, Coroutine, Mapping, Sequence  # noqa: E402
from typing import Any  # noqa: E402

from agent_framework import (  # noqa: E402
    Agent,
    BaseChatClient,
    ChatResponse,
    Message,
)

ENDPOINT = "https://models.github.ai/inference/chat/completions"
MODEL = os.environ.get("GITHUB_MODELS_MODEL", "openai/gpt-4o-mini")
MAX_COMPLETION_TOKENS = int(
    os.environ.get(
        "GITHUB_MODELS_MAX_COMPLETION_TOKENS",
        "1024" if MODEL.startswith("openai/gpt-5") else "128",
    )
)
REASONING_EFFORT = os.environ.get(
    "GITHUB_MODELS_REASONING_EFFORT",
    "minimal" if MODEL.startswith("openai/gpt-5") else "",
).strip()


class GitHubModelsChatClient(BaseChatClient):
    """Chat client that calls the GitHub Models API synchronously via urllib."""

    def __init__(self, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self._token = os.environ.get("GITHUB_TOKEN", "").strip()
        if not self._token:
            raise RuntimeError("GITHUB_TOKEN must be set on the host and passed with pyhl run --env GITHUB_TOKEN")

    def _inner_get_response(
        self,
        *,
        messages: Sequence[Message],
        stream: bool = False,
        options: Mapping[str, Any],
        **kwargs: Any,
    ) -> Awaitable[ChatResponse]:
        payload = {
            "model": MODEL,
            "messages": [{"role": m.role, "content": m.text} for m in messages if m.text],
            "max_completion_tokens": MAX_COMPLETION_TOKENS,
        }
        if REASONING_EFFORT:
            payload["reasoning_effort"] = REASONING_EFFORT
        req = urllib.request.Request(
            ENDPOINT,
            data=json.dumps(payload).encode(),
            headers={
                "Authorization": f"Bearer {self._token}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.load(resp)
        except urllib.error.HTTPError as exc:
            body = exc.read().decode("utf-8", "replace").strip()
            try:
                message = json.loads(body)["error"]["message"]
            except Exception:
                message = body or exc.reason
            raise RuntimeError(f"GitHub Models API returned HTTP {exc.code} for {MODEL}: {message}") from exc

        reply = data["choices"][0]["message"]["content"].strip()
        if not reply:
            raise RuntimeError(
                f"GitHub Models API returned an empty response for {MODEL}; "
                "increase GITHUB_MODELS_MAX_COMPLETION_TOKENS or lower GITHUB_MODELS_REASONING_EFFORT"
            )
        response = ChatResponse(
            messages=[Message(role="assistant", contents=[reply])],
            model=data.get("model", MODEL),
        )

        async def _get() -> ChatResponse:
            return response

        return _get()


def run_sync(coro: Coroutine[Any, Any, Any]) -> Any:
    """Run a coroutine that performs no real async I/O, without an event loop.

    `asyncio.run()` can't be used here: it builds a Unix selector event loop
    whose wake-up self-pipe needs `socket.socketpair()`, which this unikernel
    doesn't implement (ENOSYS). The HTTP call is synchronous and never suspends,
    so we can just step the coroutine to completion.
    """
    try:
        while True:
            pending = coro.send(None)
            if pending is not None:
                raise RuntimeError(f"coroutine requires an event loop (awaited {pending!r})")
    except StopIteration as exc:
        return exc.value


async def main() -> None:
    agent = Agent(
        client=GitHubModelsChatClient(),
        name="RemoteHyperlightAgent",
        instructions="You are a helpful assistant. Answer concisely.",
    )
    query = "In one short sentence, what is the Hyperlight CNCF project?"
    print(f"User:  {query}")
    result = await agent.run(query)
    print(f"Agent: {result.messages[0].text}")


if __name__ == "__main__":
    run_sync(main())
