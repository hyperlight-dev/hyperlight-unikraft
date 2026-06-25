# echo-wasm-host-fxn

A tiny WASIp1 command module used by the Python host-tools example as a custom host function.

The Hyperlight host runs the module through `--tool echo=...`. The module receives the `__dispatch` request JSON on stdin:

```json
{"name":"echo","args":{"message":"hello"}}
```

It writes `{"result": args}` to stdout, so the outer host dispatch returns the original `args` value as the tool result.

```bash
just build
hyperlight-unikraft kernel --initrd app.cpio \
    --tool echo=target/wasm32-wasip1/release/echo-wasm-host-fxn.wasm \
    -- /test_tools.py
```
