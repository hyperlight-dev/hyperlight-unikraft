// A guest function: load it once, then call it as often as you like.
//
//     hluk run --initrd build-elfloader/java-rootfs.cpio examples/java/Handler.java \
//         --call handler --input '{"name":"World"}'
//
// or from Rust: sandbox.run(Exec::File("examples/java/Handler.java".into()))?, then
// sandbox.call("handler", r#"{"name":"World"}"#)?  // {"greeting":"Hello, World!","calls":1}
//
// A guest function is a static method, here a top-level one. The input is
// converted to its parameter, a record, and the record it returns is sent
// back as JSON. With only declarations and no main, the file loads as a
// library.

record Event(String name) {}

record Greeting(String greeting, int calls) {}

class Counter {
    static int calls;
}

Greeting handler(Event e) {
    return new Greeting("Hello, " + e.name() + "!", ++Counter.calls);
}
