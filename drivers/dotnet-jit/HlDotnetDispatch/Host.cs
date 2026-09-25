// Host function calls: C# in the guest calls a function the embedder
// registered (SandboxBuilder::host_function), with JSON in and out.
// The driver owns /dev/hlcall, so the request goes up the status pipe
// and the reply comes down the code pipe (drivers/hl_child.h).

#nullable enable

using System;
using System.IO;
using System.Text;
using System.Text.Json;

namespace Hyperlight
{
    /// <summary>A host function returned an error.</summary>
    public class HostException(string message) : Exception(message);

    public static class Host
    {
        private static Stream? _in, _out;
        /// <summary>Held for a whole host function call, and by the
        /// dispatch loop to end a call, so the two never share the pipes.</summary>
        internal static readonly object Gate = new();
        private static readonly UTF8Encoding StrictUtf8 = new(false, true);

        internal static bool InCall;

        internal static void Attach(Stream pipeIn, Stream pipeOut)
        {
            _in = pipeIn;
            _out = pipeOut;
        }

        /// <summary>
        /// Call the host function `name` with `args`, sent as a JSON array;
        /// its result, or null when it returned none.  Throws
        /// HostException with the host's message when it fails.
        /// </summary>
        public static JsonElement? Call(string name, params object?[] args)
        {
            if (string.IsNullOrEmpty(name))
                throw new ArgumentException("a function name is required", nameof(name));
            byte[] n = StrictUtf8.GetBytes(name);
            byte[] a = JsonSerializer.SerializeToUtf8Bytes(args, GuestFunctions.Json);
            byte tag;
            string body;
            lock (Gate)
            {
                // Checked under the lock the dispatch loop ends a call
                // with: a thread the call left behind cannot slip a request
                // in after the call's status.
                if (!InCall || _in == null || _out == null)
                    throw new InvalidOperationException("Hyperlight.Host.Call: only while the host is running code or a guest function");
                var msg = new byte[1 + 8 + n.Length + 8 + a.Length];
                msg[0] = (byte)'H';
                BitConverter.TryWriteBytes(msg.AsSpan(1), (long)n.Length);
                n.CopyTo(msg, 9);
                BitConverter.TryWriteBytes(msg.AsSpan(9 + n.Length), (long)a.Length);
                a.CopyTo(msg, 17 + n.Length);
                _out.Write(msg);
                _out.Flush();
                var hdr = new byte[9];
                ReadExactly(hdr);
                tag = hdr[0];
                var reply = new byte[BitConverter.ToInt64(hdr, 1)];
                ReadExactly(reply);
                body = Encoding.UTF8.GetString(reply);
            }
            return tag switch
            {
                0 when body.Length == 0 => null,
                0 => JsonDocument.Parse(body).RootElement.Clone(),
                1 => throw new HostException(body),
                _ => throw new InvalidOperationException($"host function {name}: {body}"),
            };
        }

        /// <summary>Call, with the result deserialized into T.</summary>
        public static T? Call<T>(string name, params object?[] args)
        {
            var r = Call(name, args);
            return r is null ? default : r.Value.Deserialize<T>(GuestFunctions.Json);
        }

        private static void ReadExactly(byte[] buf)
        {
            for (int off = 0; off < buf.Length;)
            {
                int got = _in!.Read(buf, off, buf.Length - off);
                if (got <= 0)
                    throw new IOException("hl_dotnetdriver went away");
                off += got;
            }
        }
    }
}
