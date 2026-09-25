// HlDotnetDispatch — persistent C# execution server for Hyperlight.
//
// Receives raw C# source code from the host (via hl_dotnetdriver pipe),
// compiles it in-guest using the Roslyn compilation API, loads the
// resulting assembly, and invokes its entry point.
//
// Roslyn types live in RoslynCompiler.cs — keep them out of this file
// so the JIT doesn't load Roslyn assemblies during boot.
//
// Pipe fd numbers are passed as command-line arguments by hl_dotnetdriver.

using System;
using System.IO;
using System.Text;
using Microsoft.Win32.SafeHandles;

int fdIn = int.Parse(args[0]);
int fdOut = int.Parse(args[1]);

using var pipeIn = new FileStream(
    new SafeFileHandle((nint)fdIn, ownsHandle: false), FileAccess.Read);
using var pipeOut = new FileStream(
    new SafeFileHandle((nint)fdOut, ownsHandle: false), FileAccess.Write);

// Warm up Roslyn before signaling ready — the snapshot captures this state,
// so dispatches after restore skip the warmup cost entirely.
RoslynCompiler.WarmUp();

// Redirect Console.In to a raw FileStream on fd 0.
// The default Console.ReadLine() initialization triggers .NET's signal
// handler setup (sigaction with SA_ONSTACK), which hits a Unikraft kernel
// assertion for threads without an alternate signal stack.  Pre-setting
// Console.In bypasses that initialization path entirely.
Console.SetIn(new StreamReader(
    new FileStream(new SafeFileHandle((nint)0, ownsHandle: false), FileAccess.Read)));

pipeOut.WriteByte(0);
pipeOut.Flush();
Hyperlight.Host.Attach(pipeIn, pipeOut);

// Dispatch loop, one call per message (drivers/hl_child.h has the
// protocol): 'E' runs code, 'C' calls a guest function; the answer is
// 'S', the status, and the result.
var header = new byte[9];
while (true)
{
    if (!ReadExactly(pipeIn, header)) return;
    var body = new byte[BitConverter.ToInt64(header, 1)];
    if (!ReadExactly(pipeIn, body)) return;
    int off = 0;
    byte[] Field()
    {
        long n = BitConverter.ToInt64(body, off);
        var f = body.AsSpan(off + 8, (int)n).ToArray();
        off += 8 + (int)n;
        return f;
    }
    var env = Field();
    var code = Encoding.UTF8.GetString(Field());
    bool isCall = header[0] == (byte)'C';
    var input = isCall ? Encoding.UTF8.GetString(Field()) : "";

    ApplyEnvironment(env);
    lock (Hyperlight.Host.Gate)
        Hyperlight.Host.InCall = true;
    byte status;
    byte[] result = [];
    if (isCall)
    {
        var (ok, value, error) = GuestFunctions.Call(code, input);
        if (!ok)
        {
            Console.Error.WriteLine($"hl_dotnet_dispatch: {code}: {error}");
            Console.Error.Flush();
        }
        status = ok ? (byte)0 : (byte)1;
        result = value ?? [];
    }
    else
    {
        var (success, error) = RoslynCompiler.CompileAndRun(code);
        if (!success && error != null)
        {
            Console.Error.WriteLine($"hl_dotnet_dispatch: {error}");
            Console.Error.Flush();
        }
        status = success ? (byte)0 : (byte)1;
    }
    Console.Out.Flush();

    var reply = new byte[2 + 8 + result.Length];
    reply[0] = (byte)'S';
    reply[1] = status;
    BitConverter.TryWriteBytes(reply.AsSpan(2), (long)result.Length);
    result.CopyTo(reply, 10);
    // Under the host call lock: a host function call still in flight on
    // another thread finishes first, and none starts after the status.
    lock (Hyperlight.Host.Gate)
    {
        Hyperlight.Host.InCall = false;
        pipeOut.Write(reply);
        pipeOut.Flush();
    }
}

static bool ReadExactly(Stream s, byte[] buf)
{
    for (int off = 0; off < buf.Length;)
    {
        int n = s.Read(buf, off, buf.Length - off);
        if (n <= 0) return false;
        off += n;
    }
    return true;
}

// The host's environment, KEY NUL VALUE NUL pairs.
static void ApplyEnvironment(byte[] env)
{
    var parts = Encoding.UTF8.GetString(env).Split('\0');
    for (int i = 0; i + 1 < parts.Length; i += 2)
        Environment.SetEnvironmentVariable(parts[i], parts[i + 1]);
}
