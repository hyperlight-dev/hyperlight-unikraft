// A guest function: load it once, then call it as often as you like.
//
//     hluk run --initrd build-elfloader/quickjs-rootfs.cpio examples/quickjs/handler.js \
//         --call handler --input '{"name":"World"}'
//
// or from Rust: sandbox.run(Exec::File("examples/quickjs/handler.js".into()))?, then
// sandbox.call("handler", r#"{"name":"World"}"#)?  // {"greeting":"Hello, World!","calls":1}
//
// Declared in a script, `handler` and `calls` are globals the next call sees.
var calls = 0;

function handler(event) {
  calls++;
  return { greeting: `Hello, ${event.name}!`, calls };
}
