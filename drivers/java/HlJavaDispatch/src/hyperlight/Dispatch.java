// HlJavaDispatch: the Java runtime's server, started once by
// hl_javadriver and kept for the life of the guest.
//
// Each call comes down a pipe from the driver (drivers/hl_child.h has the
// protocol): 'E' runs code with JShell, 'C' calls a guest function; the
// answer is 'S', the status, and the result.  The pipe fds are the two
// arguments.

package hyperlight;

import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.lang.reflect.Field;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;

public final class Dispatch {
    private Dispatch() {}

    public static void main(String[] args) throws Exception {
        // Unbuffered: BufferedInputStream asks available() between reads, which
        // the JDK answers for this pipe with lseek, and the guest's pipe
        // fails it (ESPIPE).  Each message is read in two reads anyway.
        InputStream in = new FileInputStream(fd(Integer.parseInt(args[0])));
        var out = new FileOutputStream(fd(Integer.parseInt(args[1])));
        Environment.init();

        // JShell and javac warm before the ready byte: the snapshot taken
        // once the driver parks captures them, so a restored guest only
        // compiles its own code.
        var snippets = new Snippets();
        snippets.warmUp();

        out.write(0);
        out.flush();
        Host.attach(in, out);

        // The JVM ends with the loop, whatever ends it: the driver closing
        // the pipe, or something thrown outside a call (an allocation for a
        // message failing).  Left to itself it would wait for the threads
        // code left running, with nobody to answer the driver, which would
        // wait for ever; ended, it is reaped and the next call starts afresh.
        int status = 1;
        try {
            serve(in, snippets);
            status = 0;
        } catch (Throwable t) {
            try {
                System.err.println("hl_java_dispatch: " + t);
            } catch (Throwable ignored) {
                // Out of memory even for that: the status says enough.
            }
        } finally {
            System.out.flush();
            System.err.flush();
            Runtime.getRuntime().halt(status);
        }
    }

    private static void serve(InputStream in, Snippets snippets) throws Exception {
        while (true) {
            byte[] header = Host.readUpTo(in, 9);
            if (header.length < 9)
                return;
            int len = Math.toIntExact(le(header).getLong(1));
            var body = le(Host.readUpTo(in, len));
            if (body.limit() < len)
                return;
            byte[] env = field(body);
            String code = new String(field(body), StandardCharsets.UTF_8);
            boolean isCall = header[0] == 'C';
            String input = isCall ? new String(field(body), StandardCharsets.UTF_8) : "";

            // Every call runs on this thread: an interrupt the last one left
            // would fail the next one's first wait.
            Thread.interrupted();
            boolean ok;
            byte[] result = null;
            try {
                Environment.apply(env);
                Host.begin();
                if (isCall) {
                    var outcome = GuestFunctions.call(snippets, code, input);
                    ok = outcome.ok();
                    result = outcome.result();
                } else {
                    ok = snippets.run(code) == 0;
                }
            } catch (Throwable t) {
                // Whatever the code threw past the dispatcher's own handling
                // (a class's static initializer failing, a StackOverflowError)
                // fails this call, not the runtime and its state.
                Snippets.reportException(t);
                ok = false;
                result = null;
            }
            System.out.flush();
            System.err.flush();

            if (result == null)
                result = new byte[0];
            var reply = ByteBuffer.allocate(2 + 8 + result.length).order(ByteOrder.LITTLE_ENDIAN);
            reply.put((byte) 'S').put((byte) (ok ? 0 : 1)).putLong(result.length).put(result);
            Host.end(reply.array());
        }
    }

    private static ByteBuffer le(byte[] b) {
        return ByteBuffer.wrap(b).order(ByteOrder.LITTLE_ENDIAN);
    }

    private static byte[] field(ByteBuffer b) {
        byte[] f = new byte[Math.toIntExact(b.getLong())];
        b.get(f);
        return f;
    }

    /** A FileDescriptor for an inherited fd, which Java has no public way
     *  to make; the jar manifest's Add-Opens opens java.io to us for it. */
    private static FileDescriptor fd(int n) throws ReflectiveOperationException {
        var d = new FileDescriptor();
        Field f = FileDescriptor.class.getDeclaredField("fd");
        f.setAccessible(true);
        f.setInt(d, n);
        return d;
    }
}
