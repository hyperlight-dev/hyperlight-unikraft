// The host's environment variables, into this JVM.  System.getenv() is a
// copy taken at startup that Java offers no way to change, and this JVM
// starts once, at boot; so each call writes the host's variables into that
// copy, which System.getenv() and ProcessBuilder.environment() read.  A
// process started without ProcessBuilder.environment() (Runtime.exec, a
// plain ProcessBuilder) gets the JVM's environment from its start instead.

package hyperlight;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.charset.StandardCharsets;
import java.util.Map;

final class Environment {
    private Environment() {}

    private static Map<Object, Object> env;
    private static Method variable, value;

    /** java.lang.ProcessEnvironment's map, opened to us by the jar
     *  manifest's Add-Opens (drivers/java/jvm.args says why not --add-opens). */
    @SuppressWarnings("unchecked")
    static void init() throws ReflectiveOperationException {
        Class<?> pe = Class.forName("java.lang.ProcessEnvironment");
        Field f = pe.getDeclaredField("theEnvironment");
        f.setAccessible(true);
        env = (Map<Object, Object>) f.get(null);
        variable = Class.forName("java.lang.ProcessEnvironment$Variable").getDeclaredMethod("valueOf", String.class);
        value = Class.forName("java.lang.ProcessEnvironment$Value").getDeclaredMethod("valueOf", String.class);
        variable.setAccessible(true);
        value.setAccessible(true);
    }

    /** Set {@code pairs}, KEY NUL VALUE NUL as the driver sends them. */
    static void apply(byte[] pairs) throws ReflectiveOperationException {
        String[] parts = new String(pairs, StandardCharsets.UTF_8).split("\0", -1);
        for (int i = 0; i + 1 < parts.length; i += 2)
            env.put(variable.invoke(null, parts[i]), value.invoke(null, parts[i + 1]));
    }
}
