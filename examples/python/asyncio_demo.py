"""asyncio inside the guest: event loop, timers, and streams over the
host-proxied TCP stack.  The loop's self-pipe is an AF_UNIX socketpair,
served by the kernel's posix-unixsocket."""
import asyncio
import time

PORT = 18095


async def handle(reader, writer):
    data = await reader.read(100)
    writer.write(data.upper())
    await writer.drain()
    writer.close()


async def main():
    t = time.time()
    await asyncio.sleep(0.5)
    elapsed = time.time() - t
    assert elapsed >= 0.4, f"asyncio.sleep returned after {elapsed:.2f}s"
    print(f"asyncio.sleep ok ({elapsed:.2f}s)", flush=True)

    server = await asyncio.start_server(handle, "127.0.0.1", PORT)
    async with server:
        reader, writer = await asyncio.open_connection("127.0.0.1", PORT)
        writer.write(b"ping")
        await writer.drain()
        reply = await reader.read(100)
        writer.close()
    assert reply == b"PING", reply
    print("asyncio streams ok", flush=True)

    results = await asyncio.gather(*(asyncio.sleep(0.05, result=i) for i in range(5)))
    assert results == list(range(5))
    print("asyncio gather ok", flush=True)


print("loop:", type(asyncio.new_event_loop()).__name__, flush=True)
asyncio.run(main())
print("asyncio demo passed", flush=True)
