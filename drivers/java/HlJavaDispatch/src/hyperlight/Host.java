// Host function calls: Java in the guest calls a function the embedder
// registered (SandboxBuilder::host_function), with JSON in and out.
// The driver owns /dev/hlcall, so the request goes up the status pipe
// and the reply comes down the code pipe (drivers/hl_child.h).

package hyperlight;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;

public final class Host {
    private Host() {}

    private static InputStream in;
    private static OutputStream out;
    private static boolean inCall;

    /** Held for a whole host function call, and by the dispatch loop to
     *  end a call, so the two never share the pipes. */
    private static final Object GATE = new Object();

    /**
     * Call the host function {@code name} with {@code args}, sent as a JSON
     * array; its result parsed ({@link Json#parse(String)}), or null when it
     * returned none.  Throws HostException with the host's message when it
     * fails.
     */
    public static Object call(String name, Object... args) {
        if (name == null || name.isEmpty())
            throw new IllegalArgumentException("a function name is required");
        byte[] n = name.getBytes(StandardCharsets.UTF_8);
        byte[] a = Json.stringify(args).getBytes(StandardCharsets.UTF_8);
        byte tag;
        String body;
        synchronized (GATE) {
            // Checked under the lock the dispatch loop ends a call with: a
            // thread the call left behind cannot slip a request in after the
            // call's status.
            if (!inCall || in == null || out == null)
                throw new IllegalStateException("hyperlight.Host.call: only while the host is running code or a guest function");
            var msg = ByteBuffer.allocate(1 + 8 + n.length + 8 + a.length).order(ByteOrder.LITTLE_ENDIAN);
            msg.put((byte) 'H').putLong(n.length).put(n).putLong(a.length).put(a);
            try {
                out.write(msg.array());
                out.flush();
                var hdr = ByteBuffer.wrap(readUpTo(in, 9)).order(ByteOrder.LITTLE_ENDIAN);
                if (hdr.limit() < 9)
                    throw new IllegalStateException("hl_javadriver went away");
                tag = hdr.get(0);
                int len = Math.toIntExact(hdr.getLong(1));
                byte[] reply = readUpTo(in, len);
                if (reply.length < len)
                    throw new IllegalStateException("hl_javadriver went away");
                body = new String(reply, StandardCharsets.UTF_8);
            } catch (IOException e) {
                throw new IllegalStateException("hl_javadriver went away", e);
            }
        }
        return switch (tag) {
            case 0 -> body.isEmpty() ? null : Json.parse(body);
            case 1 -> throw new HostException(body);
            default -> throw new IllegalStateException("host function " + name + ": " + body);
        };
    }

    /** {@link #call(String, Object...)}, its result converted to {@code type}. */
    public static <T> T call(Class<T> type, String name, Object... args) {
        @SuppressWarnings("unchecked")
        T t = (T) Json.convert(call(name, args), type);
        return t;
    }

    /** Up to {@code len} bytes, fewer only at the end of the stream, by plain
     *  reads: the JDK's helpers ask the pipe available(), which the guest's
     *  pipe fails (Dispatch.main). */
    static byte[] readUpTo(InputStream in, int len) throws IOException {
        byte[] b = new byte[len];
        int off = 0;
        while (off < len) {
            int n = in.read(b, off, len - off);
            if (n < 0)
                return java.util.Arrays.copyOf(b, off);
            off += n;
        }
        return b;
    }

    // ── For the dispatch loop ──────────────────────────────────

    static void attach(InputStream pipeIn, OutputStream pipeOut) {
        in = pipeIn;
        out = pipeOut;
    }

    static void begin() {
        synchronized (GATE) {
            inCall = true;
        }
    }

    /** End the call: under the lock, a host function call still in flight
     *  on another thread finishes first, and none starts after the status. */
    static void end(byte[] reply) throws IOException {
        synchronized (GATE) {
            inCall = false;
            out.write(reply);
            out.flush();
        }
    }
}
