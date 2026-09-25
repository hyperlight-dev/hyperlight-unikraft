// A guest function: load it once, then call it as often as you like.
//
//     hluk run --initrd build-elfloader/dotnet-jit-rootfs.cpio examples/dotnet-jit/Handler.cs \
//         --call Handler --input '{"name":"World"}'
//
// or from Rust: sandbox.run(Exec::File("examples/dotnet-jit/Handler.cs".into()))?, then
// sandbox.call("Handler", r#"{"name":"World"}"#)?  // {"greeting":"Hello, World!","calls":1}
//
// A guest function is a public static method of a public class. The input
// is deserialized into its parameter, and the return value serialized
// back, camelCase. With only definitions, the snippet loads as a library.

public record Event(string Name);

public static class Handlers
{
    static int calls;

    public static object Handler(Event e) =>
        new { Greeting = $"Hello, {e.Name}!", Calls = ++calls };
}
