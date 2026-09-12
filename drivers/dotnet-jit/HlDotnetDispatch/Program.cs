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

// Dispatch loop — one execution per iteration
var header = new byte[8];
while (true)
{
    // Read 8-byte length (little-endian uint64)
    int bytesRead = 0;
    while (bytesRead < 8)
    {
        int n = pipeIn.Read(header, bytesRead, 8 - bytesRead);
        if (n <= 0) return;
        bytesRead += n;
    }

    long len = BitConverter.ToInt64(header, 0);
    var buf = new byte[len];
    int off = 0;
    while (off < (int)len)
    {
        int n = pipeIn.Read(buf, off, (int)len - off);
        if (n <= 0) return;
        off += n;
    }

    string code = Encoding.UTF8.GetString(buf);

    // Compile + execute via RoslynCompiler (loaded on first call)
    var (success, error) = RoslynCompiler.CompileAndRun(code);

    if (!success && error != null)
    {
        Console.Error.WriteLine($"hl_dotnet_dispatch: {error}");
        Console.Error.Flush();
    }

    pipeOut.WriteByte(success ? (byte)0 : (byte)1);
    pipeOut.Flush();
}
