// Guest function calls: the host calls a public static method of a
// snippet run earlier, by name, with JSON in and out.

#nullable enable

using System;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using System.Threading.Tasks;

public static class GuestFunctions
{
    /// <summary>camelCase out, case-insensitive in: the JSON the other
    /// runtimes' handlers produce and take.</summary>
    internal static readonly JsonSerializerOptions Json = JsonSerializerOptions.Web;

    /// <summary>
    /// Call `name` (`Method`, or `Type.Method` when two types have one)
    /// with `input` deserialized into its one parameter (none for empty
    /// input); its result, awaited if a task, serialized.  `void` and
    /// `Task` give no result.
    /// </summary>
    public static (bool ok, byte[]? result, string? error) Call(string name, string input)
    {
        try
        {
            var method = Find(name);
            if (method == null)
                return (false, null, $"no public static method {name}: define one in a public " +
                    $"class (public static object {name}(JsonElement input) {{ ... }}) before calling it");
            var ps = method.GetParameters();
            object?[] args = ps.Length switch
            {
                0 when input.Length == 0 => [],
                0 => throw new ArgumentException($"{name} takes no input"),
                1 => [input.Length == 0 ? null : JsonSerializer.Deserialize(input, ps[0].ParameterType, Json)],
                _ => throw new ArgumentException($"{name} takes {ps.Length} parameters; a guest function takes one"),
            };
            object? value;
            try
            {
                value = method.Invoke(null, args);
            }
            catch (TargetInvocationException e) when (e.InnerException != null)
            {
                throw e.InnerException;
            }
            var type = method.ReturnType;
            if (value is ValueTask vt)
            {
                vt.AsTask().GetAwaiter().GetResult();
                return (true, null, null);
            }
            if (type.IsGenericType && type.GetGenericTypeDefinition() == typeof(ValueTask<>))
                value = type.GetMethod("AsTask")!.Invoke(value, null);
            if (value is Task task)
            {
                task.GetAwaiter().GetResult();
                var generic = type.IsGenericType && type.GetGenericTypeDefinition() is var d &&
                    (d == typeof(Task<>) || d == typeof(ValueTask<>));
                if (!generic)
                    return (true, null, null);
                value = task.GetType().GetProperty("Result")!.GetValue(task);
            }
            else if (type == typeof(void))
            {
                return (true, null, null);
            }
            return (true, JsonSerializer.SerializeToUtf8Bytes(value, value?.GetType() ?? typeof(object), Json), null);
        }
        catch (Exception e)
        {
            return (false, null, $"{e.GetType().Name}: {e.Message}");
        }
    }

    /// <summary>The method, newest snippet first, so a redefinition wins.</summary>
    private static MethodInfo? Find(string name)
    {
        int dot = name.LastIndexOf('.');
        string? typeName = dot < 0 ? null : name[..dot];
        string methodName = dot < 0 ? name : name[(dot + 1)..];
        for (int i = RoslynCompiler.Loaded.Count - 1; i >= 0; i--)
        {
            var found = RoslynCompiler.Loaded[i].GetExportedTypes()
                .Where(t => typeName == null || t.Name == typeName || t.FullName == typeName)
                .SelectMany(t => t.GetMethods(BindingFlags.Public | BindingFlags.Static))
                .Where(m => m.Name == methodName && !m.IsSpecialName && !m.ContainsGenericParameters)
                .ToList();
            if (found.Count == 1)
                return found[0];
            if (found.Count > 1)
                throw new AmbiguousMatchException(
                    $"{name} names {found.Count} methods; call it as Type.Method, with one overload");
        }
        return null;
    }
}
