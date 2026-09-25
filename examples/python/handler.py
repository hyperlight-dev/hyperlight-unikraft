# A guest function: load it once, then call it as often as you like.
#
#     hluk run --initrd build-elfloader/python-rootfs.cpio examples/python/handler.py \
#         --call handler --input '{"name":"World"}'
#
# or from Rust: sandbox.run(Exec::File("examples/python/handler.py".into()))?, then
# sandbox.call("handler", r#"{"name":"World"}"#)?  // {"greeting": "Hello, World!", "calls": 1}
#
# The input arrives parsed (a dict here), and the return value goes back as JSON.
calls = 0


def handler(event):
    global calls
    calls += 1
    return {"greeting": f"Hello, {event['name']}!", "calls": calls}
