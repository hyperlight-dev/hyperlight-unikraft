// Roslyn compilation engine for Hyperlight's .NET JIT driver.
//
// Key design decisions:
//   - ConcurrentBuild = false: Roslyn's default parallel compilation
//     deadlocks the cooperative scheduler.
//   - CreateFromImage (in-memory): avoids memory-mapped file issues.
//   - R2R (ReadyToRun) in the .csproj: pre-compiles Roslyn's methods to
//     native code, reducing JIT overhead from minutes to seconds.
//   - WarmUp() runs during boot so snapshots capture the initialized state.

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.CompilerServices;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;

public static class RoslynCompiler
{
    private static object? _state;
    private static int _compilationId;

    // Common using directives prepended to user code so simple scripts
    // work without explicit imports (same UX as Python/Node drivers).
    private const string Preamble = @"
using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
";

    /// <summary>
    /// Pre-initialize Roslyn during boot so snapshots capture the warm state.
    /// Runs a trivial compilation to exercise the full pipeline (parse → emit).
    /// </summary>
    [MethodImpl(MethodImplOptions.NoInlining)]
    public static void WarmUp()
    {
        _state = Initialize();
        // Run a trivial compilation to warm the Roslyn JIT paths
        CompileAndRun("System.Console.Write(\"\");");
    }

    /// <summary>
    /// Compile and execute C# source code in-process.
    /// Returns (true, null) on success, (false, errorMessage) on failure.
    /// </summary>
    [MethodImpl(MethodImplOptions.NoInlining)]
    public static (bool success, string? error) CompileAndRun(string code)
    {
        try
        {
            if (_state == null)
                _state = Initialize();

            var (references, parseOptions, compileOptions) =
                ((List<MetadataReference>, CSharpParseOptions, CSharpCompilationOptions))_state;

            // Prepend common using directives — C# 9+ top-level statements
            // mean Console.WriteLine("hello") just works.
            string fullCode = Preamble + code;

            var syntaxTree = CSharpSyntaxTree.ParseText(fullCode, parseOptions);
            var compilation = CSharpCompilation.Create(
                $"Script{_compilationId++}",
                syntaxTrees: new[] { syntaxTree },
                references: references,
                options: compileOptions);

            using var ms = new MemoryStream();
            var emitResult = compilation.Emit(ms);

            if (!emitResult.Success)
            {
                var errors = emitResult.Diagnostics
                    .Where(d => d.Severity == DiagnosticSeverity.Error)
                    .Select(d => d.GetMessage());
                return (false, $"compile error: {string.Join("; ", errors)}");
            }

            ms.Seek(0, SeekOrigin.Begin);
            var assembly = Assembly.Load(ms.ToArray());
            var entryPoint = assembly.EntryPoint;

            if (entryPoint == null)
                return (false, "no entry point found");

            // Capture stdout — user code writes to Console.Out
            var sw = new StringWriter();
            var origOut = Console.Out;
            Console.SetOut(sw);

            try
            {
                // Top-level statements compile to Main(string[] args) or Main()
                var parameters = entryPoint.GetParameters();
                if (parameters.Length > 0)
                    entryPoint.Invoke(null, new object[] { Array.Empty<string>() });
                else
                    entryPoint.Invoke(null, null);
            }
            finally
            {
                Console.SetOut(origOut);
            }

            var output = sw.ToString();
            if (output.Length > 0)
                Console.Write(output);
            Console.Out.Flush();

            return (true, null);
        }
        catch (TargetInvocationException ex) when (ex.InnerException != null)
        {
            return (false, $"{ex.InnerException.GetType().Name}: {ex.InnerException.Message}");
        }
        catch (Exception ex)
        {
            return (false, $"{ex.GetType().Name}: {ex.Message}");
        }
    }

    /// <summary>
    /// Build metadata references and compilation options.
    /// Called once on first dispatch.
    /// </summary>
    [MethodImpl(MethodImplOptions.NoInlining)]
    private static object Initialize()
    {
        // Collect metadata references from all runtime assemblies.
        // In a self-contained publish, all BCL assemblies are in /app/.
        // Use CreateFromImage (in-memory) to avoid memory-mapped file issues.
        var runtimeDir = Path.GetDirectoryName(typeof(object).Assembly.Location)!;
        var references = new List<MetadataReference>();
        foreach (var dll in Directory.GetFiles(runtimeDir, "*.dll"))
        {
            try
            {
                references.Add(MetadataReference.CreateFromImage(
                    File.ReadAllBytes(dll)));
            }
            catch
            {
                // Skip files that aren't valid .NET assemblies
            }
        }

        var parseOptions = CSharpParseOptions.Default
            .WithLanguageVersion(LanguageVersion.Latest);

        var compileOptions = new CSharpCompilationOptions(
            OutputKind.ConsoleApplication,
            optimizationLevel: OptimizationLevel.Release,
            allowUnsafe: true,
            // Single-threaded compilation — Roslyn's default parallel
            // compilation deadlocks the cooperative scheduler.
            concurrentBuild: false);

        return (references, parseOptions, compileOptions);
    }
}
