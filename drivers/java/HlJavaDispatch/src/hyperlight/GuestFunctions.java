// Guest function calls: the host calls a static method of a snippet run
// earlier, by name, with JSON in and out.

package hyperlight;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.Future;

final class GuestFunctions {
    private GuestFunctions() {}

    /** A call's outcome: its result (null for none), or why it failed. */
    record Outcome(byte[] result, boolean ok) {}

    /**
     * Call {@code name} ({@code method}, or {@code Type.method} when two
     * classes have one, {@code Outer.Inner.method} for a nested class) with {@code input} converted to its one parameter
     * (null, or Optional.empty(), for empty input); its result, awaited if a future, as JSON.
     * {@code void} and a future of {@code Void} give no result.
     */
    static Outcome call(Snippets snippets, String name, String input) {
        Method m;
        try {
            m = find(snippets, name);
        } catch (IllegalArgumentException e) {
            return fail(name, e.getMessage());
        }
        if (m == null)
            return fail(name, "no static method " + name + ": define one (static Object " + name
                    + "(Map<String, Object> input) { ... }) before calling it");
        Object[] args;
        try {
            args = switch (m.getParameterCount()) {
                case 0 -> {
                    if (!input.isEmpty())
                        throw new IllegalArgumentException(name + " takes no input");
                    yield new Object[0];
                }
                case 1 -> new Object[] { Json.convert(input.isEmpty() ? null : Json.parse(input), m.getGenericParameterTypes()[0]) };
                default -> throw new IllegalArgumentException(name + " takes " + m.getParameterCount()
                        + " parameters; a guest function takes one");
            };
        } catch (IllegalArgumentException e) {
            return fail(name, e.getMessage());
        }
        Object value;
        try {
            m.setAccessible(true);
            value = m.invoke(null, args);
            Type type = m.getGenericReturnType();
            if (value instanceof CompletionStage<?> cs)
                value = cs.toCompletableFuture().get();
            else if (value instanceof Future<?> f)
                value = f.get();
            if (type == void.class || type == Void.class || futureOfVoid(type))
                return new Outcome(null, true);
            return new Outcome(Json.stringify(value).getBytes(StandardCharsets.UTF_8), true);
        } catch (InvocationTargetException | ExecutionException e) {
            Snippets.reportException(e.getCause());
            return new Outcome(null, false);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return fail(name, "interrupted");
        } catch (RuntimeException | IllegalAccessException e) {
            return fail(name, e.toString());
        }
    }

    private static Outcome fail(String name, String why) {
        System.err.println("hl_java_dispatch: " + name + ": " + why);
        return new Outcome(null, false);
    }

    private static boolean futureOfVoid(Type t) {
        return t instanceof ParameterizedType p && p.getRawType() instanceof Class<?> raw
                && (Future.class.isAssignableFrom(raw) || CompletionStage.class.isAssignableFrom(raw))
                && p.getActualTypeArguments()[0] == Void.class;
    }

    // The lookups since code last ran: a handler run once and called many
    // times is found once.
    private static final Map<String, Method> CACHE = new HashMap<>();
    private static int cachedAt = -1;

    private static Method find(Snippets snippets, String name) {
        if (snippets.runs() != cachedAt) {
            CACHE.clear();
            cachedAt = snippets.runs();
        }
        Method m = CACHE.get(name);
        if (m == null) {
            m = search(snippets, name);
            if (m != null)
                CACHE.put(name, m);
        }
        return m;
    }

    /** The method among the classes of the snippets JShell has now: a
     *  top-level method lives in a JShell wrapper class of its own, a class's
     *  in the class, named by its path from the top-level class
     *  ({@code Outer.Inner.m}).  An unqualified name is the top-level method
     *  when there is one, else a class's; two overloads, or two classes with
     *  none at the top level, are ambiguous. */
    private static Method search(Snippets snippets, String name) {
        int dot = name.lastIndexOf('.');
        String typePath = dot < 0 ? null : name.substring(0, dot);
        String methodName = dot < 0 ? name : name.substring(dot + 1);
        var topLevel = new ArrayList<Method>();
        var inClasses = new ArrayList<Method>();
        var classPaths = new ArrayList<String>();
        for (Class<?> c : snippets.currentClasses()) {
            String path = path(c);
            // Unqualified: a wrapper or a named class, not an anonymous or local one.
            if (typePath == null ? "".equals(path) : !typePath.equals(path))
                continue;
            for (Method m : c.getDeclaredMethods())
                if (m.getName().equals(methodName) && Modifier.isStatic(m.getModifiers())
                        && !m.isSynthetic() && !m.isBridge()) {
                    if (path == null) {
                        topLevel.add(m);
                    } else {
                        inClasses.add(m);
                        classPaths.add(path + "." + methodName);
                    }
                }
        }
        if (topLevel.size() > 1)
            throw new IllegalArgumentException(name + " names " + topLevel.size()
                    + " top-level overloads; a guest function has one");
        if (topLevel.size() == 1)
            return topLevel.get(0);
        if (inClasses.size() > 1)
            throw new IllegalArgumentException(name + " names " + inClasses.size() + " methods ("
                    + String.join(", ", classPaths) + "): call it as Type.method, with one overload");
        return inClasses.isEmpty() ? null : inClasses.get(0);
    }

    /** A declared class's path below its JShell wrapper ({@code Outer.Inner});
     *  null for a wrapper, "" for an anonymous or local class. */
    private static String path(Class<?> c) {
        if (c.getEnclosingClass() == null)
            return null;
        var parts = new ArrayList<String>();
        for (Class<?> k = c; k.getEnclosingClass() != null; k = k.getEnclosingClass()) {
            if (k.isAnonymousClass() || k.isLocalClass())
                return "";
            parts.add(0, k.getSimpleName());
        }
        return String.join(".", parts);
    }
}
