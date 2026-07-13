#!/usr/bin/env python3
"""A Microsoft Agent Framework agent doing REAL, fully offline inference.

The agent is a real `agent_framework` Agent, backed by a custom ChatClient that
runs a small GGUF model with llama-cpp-python. The model is baked into the
image, so inference happens entirely inside the Hyperlight micro-VM — no network
access and no API keys.
"""

import logging
import os
import sys
import types
import warnings

# Keep the demo output clean.
warnings.filterwarnings("ignore")
logging.getLogger("agent_framework").setLevel(logging.ERROR)

# The trimmed python-base omits a few stdlib modules that llama-cpp-python imports
# transitively (multiprocessing via llama.py, sqlite3 via diskcache). This example
# never uses either — inference is single-process and we don't use the disk cache —
# so we register minimal stubs so the imports succeed.
_mp = types.ModuleType("multiprocessing")
_mp.cpu_count = lambda: os.cpu_count() or 1
sys.modules.setdefault("multiprocessing", _mp)

_sq = types.ModuleType("sqlite3")
_sq.Binary = memoryview


class _SqliteError(Exception):
    pass


_sq.Error = _sq.OperationalError = _sq.DatabaseError = _SqliteError
_sq.IntegrityError = _sq.ProgrammingError = _SqliteError
_sq.connect = lambda *a, **k: None
_sq.register_adapter = _sq.register_converter = lambda *a, **k: None
_sq.PARSE_DECLTYPES = 1
_sq.PARSE_COLNAMES = 2
_sq.Row = object
_sq.sqlite_version = _sq.version = "0"
sys.modules.setdefault("sqlite3", _sq)

from collections.abc import Awaitable, Coroutine, Mapping, Sequence  # noqa: E402
from typing import Any  # noqa: E402

from agent_framework import (  # noqa: E402
    Agent,
    BaseChatClient,
    ChatResponse,
    Message,
)
from llama_cpp import Llama  # noqa: E402

MODEL_PATH = "/model.gguf"


class LlamaCppChatClient(BaseChatClient):
    """Chat client backed by a local llama.cpp GGUF model."""

    def __init__(self, model_path: str = MODEL_PATH, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self._llm = Llama(
            model_path=model_path,
            n_ctx=512,
            n_batch=64,  # small compute buffers to fit the identity-mapped guest
            n_threads=max(1, os.cpu_count() or 1),
            use_mmap=False,  # the guest ramfs doesn't support file-backed mmap
            verbose=False,
        )

    def _inner_get_response(
        self,
        *,
        messages: Sequence[Message],
        stream: bool = False,
        options: Mapping[str, Any],
        **kwargs: Any,
    ) -> Awaitable[ChatResponse]:
        chat = [
            {"role": m.role, "content": m.text}
            for m in messages
            if m.text
        ]
        completion = self._llm.create_chat_completion(
            messages=chat,
            max_tokens=64,
            temperature=0.7,
        )
        reply = completion["choices"][0]["message"]["content"].strip()
        response = ChatResponse(
            messages=[Message(role="assistant", contents=[reply])],
            model=os.path.basename(MODEL_PATH),
        )

        async def _get() -> ChatResponse:
            return response

        return _get()


def run_sync(coro: Coroutine[Any, Any, Any]) -> Any:
    """Run a coroutine that performs no real async I/O, without an event loop.

    `asyncio.run()` can't be used here: it builds a Unix selector event loop
    whose wake-up self-pipe needs `socket.socketpair()`, which this unikernel
    doesn't implement (ENOSYS). The client's inference call is synchronous and
    never suspends, so we can just step the coroutine to completion.
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
        client=LlamaCppChatClient(),
        name="LocalHyperlightAgent",
        instructions="You are a concise assistant. Answer in one short sentence.",
    )
    query = "In one sentence, what is a vm?"
    print(f"User:  {query}")
    result = await agent.run(query)
    print(f"Agent: {result.messages[0].text}")


if __name__ == "__main__":
    run_sync(main())
