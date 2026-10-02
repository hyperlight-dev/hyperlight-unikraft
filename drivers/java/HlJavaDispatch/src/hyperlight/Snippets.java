// The code the host sends, run by JShell in this JVM: statements,
// expressions and declarations in any mix, each call building on what the
// previous ones defined.  A call that only declares a main runs it, as
// `java Hello.java` would.

package hyperlight;

import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Set;
import java.util.regex.Pattern;
import javax.lang.model.SourceVersion;

import jdk.jshell.DeclarationSnippet;
import jdk.jshell.Diag;
import jdk.jshell.EvalException;
import jdk.jshell.JShell;
import jdk.jshell.MethodSnippet;
import jdk.jshell.Snippet;
import jdk.jshell.SnippetEvent;
import jdk.jshell.SourceCodeAnalysis.Completeness;
import jdk.jshell.TypeDeclSnippet;
import jdk.jshell.UnresolvedReferenceException;
import jdk.jshell.execution.DirectExecutionControl;
import jdk.jshell.spi.SPIResolutionException;
import jdk.jshell.spi.ExecutionControl;
import jdk.jshell.spi.ExecutionControlProvider;
import jdk.jshell.spi.ExecutionEnv;

final class Snippets {
    // What a compact source file (Java 25) sees without imports, and this
    // package for Host and Json.
    private static final String PREAMBLE = "import module java.base;\nimport hyperlight.*;\n";

    final SnippetLoader loader = new SnippetLoader(Snippets.class.getClassLoader());
    private final JShell js;
    private int runs;

    // The one instance, for reportException's callers outside it.
    private static Snippets instance;

    /** The declaration whose stub threw, from the class of the frame that
     *  threw it, and what it waits on. */
    private String waiting(SPIResolutionException r) {
        if (r.getStackTrace().length == 0)
            return "it";
        String thrower = r.getStackTrace()[0].getClassName();
        return js.snippets()
                .filter(s -> s instanceof DeclarationSnippet && js.status(s).isActive())
                .filter(s -> thrower.equals(wrapper(s)) || thrower.startsWith(wrapper(s) + "$"))
                .map(s -> {
                    var d = (DeclarationSnippet) s;
                    var missing = js.unresolvedDependencies(d).toList();
                    return missing.isEmpty() ? d.name() : d.name() + " (" + String.join(", ", missing) + ")";
                })
                .findFirst()
                .orElse("it");
    }

    Snippets() throws Exception {
        instance = this;
        js = JShell.builder()
                .executionEngine(new Engine(loader), Map.of())
                .build();
        // The compiler needs the hyperlight classes on its class path; the
        // loader finds them through its parent.
        js.addToClasspath(Path.of(Snippets.class.getProtectionDomain().getCodeSource().getLocation().toURI()).toString());
        if (run(PREAMBLE) != 0)
            throw new IllegalStateException("the preamble did not compile");
    }

    /** Compile and load once, so a snapshot taken after boot has javac warm. */
    void warmUp() {
        run("void hl$warm() { System.out.print(\"\"); }\nhl$warm();\nrecord Hl$Warm(int x) {}");
        js.snippets().filter(s -> s.source().contains("hl$warm") || s.source().contains("Hl$Warm")).toList()
                .forEach(js::drop);
    }

    /** Run {@code code}: 0 when it ran to the end, 1 when it did not compile
     *  or threw, with the reason on stderr. */
    int run(String code) {
        runs++;
        var ran = new ArrayList<Snippet>();
        try {
            return run(code, ran);
        } finally {
            // JShell keeps every snippet, about 50 KiB each: a statement or an
            // expression has nothing to keep once it has run, so a guest that
            // runs code call after call stays the size it was.  Declarations
            // stay, for the calls that use them.
            for (Snippet s : ran)
                js.drop(s);
            // An expression's value is a static field of its wrapper ($1),
            // which no code can name once it has run, but the class stays
            // loaded, so the value goes unless cleared.  A variable declared
            // again keeps its old value: code still running (a thread, a
            // server's handler) reads the old field.
            clearValues(ran);
        }
    }

    /** Null the values of the dropped expressions among {@code ran}. */
    private void clearValues(List<Snippet> ran) {
        Map<String, Class<?>> byName = null;
        for (Snippet s : ran) {
            if (s.subKind() != Snippet.SubKind.TEMP_VAR_EXPRESSION_SUBKIND)
                continue;
            if (byName == null) {
                byName = new HashMap<>();
                for (Class<?> c : loader.loaded())
                    byName.put(c.getName(), c);
            }
            Class<?> c = byName.get(wrapper(s));
            if (c == null)
                continue;
            for (Field f : c.getDeclaredFields()) {
                int mod = f.getModifiers();
                if (!Modifier.isStatic(mod) || Modifier.isFinal(mod) || f.getType().isPrimitive())
                    continue;
                try {
                    f.setAccessible(true);
                    f.set(null, null);
                } catch (ReflectiveOperationException | RuntimeException e) {
                    // A field that cannot be cleared keeps its value, as before.
                }
            }
        }
    }

    // A file's package declaration, after any comments: JShell has no
    // packages, and `java App.java` runs a file with one.
    private static final Pattern PACKAGE = Pattern.compile("package\\s+([^;]*);");

    /** {@code code} without its package declaration.  The comments before
     *  it are skipped by hand: a regex repeating over them recurses once a
     *  repetition, and a long header overflows the stack. */
    static String stripPackage(String code) {
        int i = 0, n = code.length();
        while (i < n) {
            if (Character.isWhitespace(code.charAt(i))) {
                i++;
            } else if (code.startsWith("//", i)) {
                int eol = code.indexOf('\n', i);
                i = eol < 0 ? n : eol + 1;
            } else if (code.startsWith("/*", i)) {
                int end = code.indexOf("*/", i + 2);
                if (end < 0)
                    return code;
                i = end + 2;
            } else {
                break;
            }
        }
        // Only a name Java takes (identifiers, no keywords): anything else
        // stays, for JShell to reject as `java` would.
        var m = PACKAGE.matcher(code).region(i, n);
        if (!m.lookingAt() || !SourceVersion.isName(m.group(1).replaceAll("\\s+", "")))
            return code;
        return code.substring(0, i) + code.substring(m.end());
    }

    private int run(String code, List<Snippet> ran) {
        code = stripPackage(code);
        boolean statements = false;
        var topLevelMains = new ArrayList<MethodSnippet>();
        var classes = new ArrayList<TypeDeclSnippet>();
        var recoverable = new ArrayList<DeclarationSnippet>();
        String rest = code;
        while (!rest.isBlank()) {
            var info = js.sourceCodeAnalysis().analyzeCompletion(rest);
            if (info.completeness() == Completeness.EMPTY)
                break;
            // An incomplete snippet goes in as it is, for JShell's error.
            String source = info.completeness().isComplete() ? info.source() : rest;
            rest = info.completeness().isComplete() ? info.remaining() : "";
            for (SnippetEvent e : js.eval(source)) {
                // Dependents JShell recompiled are reported, but are not this code.
                if (e.causeSnippet() != null)
                    continue;
                Snippet s = e.snippet();
                if (ranCode(s) || e.status() == Snippet.Status.REJECTED)
                    ran.add(s);
                if (e.status() == Snippet.Status.REJECTED) {
                    reportRejected(s);
                    return 1;
                }
                if (e.exception() != null) {
                    reportException(e.exception());
                    return 1;
                }
                // A field (VAR) is a declaration: a compact source file has them.
                statements |= ranCode(s);
                switch (s.kind()) {
                    case METHOD -> {
                        if (((MethodSnippet) s).name().equals("main"))
                            topLevelMains.add((MethodSnippet) s);
                    }
                    case TYPE_DECL -> classes.add((TypeDeclSnippet) s);
                    default -> {}
                }
                if (s instanceof DeclarationSnippet d && unresolved(d))
                    recoverable.add(d);
            }
        }
        // Said once the whole code is in: a later snippet may declare what an
        // earlier one used.
        String blocked = null;
        for (DeclarationSnippet d : recoverable) {
            if (!unresolved(d))
                continue;
            if (blocked == null && holdsMain(d))
                blocked = d.name();
            System.err.println("hl_java_dispatch: warning: " + d.name() + " cannot be used until "
                    + String.join(", ", js.unresolvedDependencies(d).toList()) + " is declared");
        }
        if (statements)
            return 0;
        // As `java Main.java` would not compile it.
        if (blocked != null) {
            System.err.println("error: " + (blocked.equals("main") ? "main" : blocked + "'s main") + " cannot run");
            return 1;
        }
        return runMain(topLevelMains, classes);
    }

    /** The wrapper classes of the snippets JShell has now and has loaded:
     *  a class of a replaced snippet, or of one waiting on a declaration, is
     *  not current even when its name is.  JShell keeps the snippet-to-class
     *  map to itself (Snippet.classFullName), opened to us by the jar
     *  manifest. */
    private Set<String> currentWrappers() {
        var names = new HashSet<String>();
        js.snippets().forEach(s -> {
            var status = js.status(s);
            if (status == Snippet.Status.VALID || status == Snippet.Status.RECOVERABLE_DEFINED)
                names.add(wrapper(s));
        });
        return names;
    }

    private static final Method CLASS_FULL_NAME;
    static {
        try {
            CLASS_FULL_NAME = Snippet.class.getDeclaredMethod("classFullName");
            CLASS_FULL_NAME.setAccessible(true);
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    private static String wrapper(Snippet s) {
        try {
            return (String) CLASS_FULL_NAME.invoke(s);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cannot read a snippet's class: " + e, e);
        }
    }

    /** How many times code has run: what a guest function was looked up
     *  against changes only with it. */
    int runs() {
        return runs;
    }

    /** Code that ran rather than declared: a statement, or an expression,
     *  which JShell files as a temporary variable ($1) when it has a value. */
    private static boolean ranCode(Snippet s) {
        return s.kind() == Snippet.Kind.STATEMENT || s.kind() == Snippet.Kind.EXPRESSION
                || s.subKind() == Snippet.SubKind.TEMP_VAR_EXPRESSION_SUBKIND;
    }

    private static final Pattern MAIN = Pattern.compile("\\bvoid\\s+main\\s*\\(");

    /** A top-level main, or a class declaring one: JShell does not load a
     *  class that waits on a declaration, so its source is all there is. */
    private static boolean holdsMain(DeclarationSnippet d) {
        if (d instanceof MethodSnippet m)
            return m.name().equals("main");
        return d instanceof TypeDeclSnippet && MAIN.matcher(codeOnly(d.source())).find();
    }

    /** {@code src} with its comments and string, text-block and character
     *  literals blanked, so only code is matched. */
    static String codeOnly(String src) {
        var out = new StringBuilder(src.length());
        int i = 0, n = src.length();
        while (i < n) {
            int end;
            if (src.startsWith("//", i)) {
                end = src.indexOf('\n', i);
                end = end < 0 ? n : end;
            } else if (src.startsWith("/*", i)) {
                end = src.indexOf("*/", i + 2);
                end = end < 0 ? n : end + 2;
            } else if (src.startsWith("\"\"\"", i)) {
                end = closing(src, i + 3, "\"\"\"");
            } else if (src.charAt(i) == '"' || src.charAt(i) == '\'') {
                end = closing(src, i + 1, String.valueOf(src.charAt(i)));
            } else {
                out.append(src.charAt(i++));
                continue;
            }
            out.append(" ".repeat(end - i));
            i = end;
        }
        return out.toString();
    }

    /** Past the {@code quote} that closes a literal begun before {@code i},
     *  skipping escapes. */
    private static int closing(String src, int i, String quote) {
        while (i < src.length()) {
            if (src.charAt(i) == '\\')
                i += 2;
            else if (src.startsWith(quote, i))
                return i + quote.length();
            else
                i++;
        }
        return src.length();
    }

    private boolean unresolved(DeclarationSnippet d) {
        var status = js.status(d);
        return status == Snippet.Status.RECOVERABLE_DEFINED || status == Snippet.Status.RECOVERABLE_NOT_DEFINED;
    }

    /** Declarations only: run a main this code declared, if any, from the
     *  class JShell compiled for its snippet (JShell does not compile a
     *  declaration again whose source is unchanged, so it may be an earlier
     *  call's, and an older {@code main} overload may still be there).  A
     *  top-level {@code void main()} first (a compact source file), then the
     *  first class declared with one; {@code String[]} before none, then
     *  static before instance, as Java 25 picks. */
    private int runMain(List<MethodSnippet> topLevelMains, List<TypeDeclSnippet> classes) {
        Map<String, Class<?>> byName = new HashMap<>();
        for (Class<?> c : loader.loaded())
            byName.put(c.getName(), c);
        // Each top-level overload has a wrapper of its own: the best of them.
        Method main = null;
        for (MethodSnippet m : topLevelMains)
            if (byName.get(wrapper(m)) instanceof Class<?> c && mainOf(c) instanceof Method found
                    && (main == null || rank(found) < rank(main)))
                main = found;
        for (int i = 0; main == null && i < classes.size(); i++) {
            TypeDeclSnippet t = classes.get(i);
            // A class is nested in its snippet's wrapper: REPL.$JShell$7$App.
            if (byName.get(wrapper(t) + "$" + t.name()) instanceof Class<?> c)
                main = mainOf(c);
        }
        if (main == null)
            return 0;
        try {
            main.setAccessible(true);
            Object target = null;
            if (!Modifier.isStatic(main.getModifiers())) {
                var ctor = main.getDeclaringClass().getDeclaredConstructor();
                // As Java 25's launcher: an instance main needs a no-argument
                // constructor that is not private.
                if (Modifier.isPrivate(ctor.getModifiers())) {
                    System.err.println("error: " + main.getDeclaringClass().getSimpleName()
                            + "'s no-argument constructor is private, so its main cannot run");
                    return 1;
                }
                ctor.setAccessible(true);
                target = ctor.newInstance();
            }
            if (main.getParameterCount() == 1)
                main.invoke(target, (Object) new String[0]);
            else
                main.invoke(target);
            return 0;
        } catch (InvocationTargetException e) {
            reportException(e.getCause());
            return 1;
        } catch (ReflectiveOperationException e) {
            System.err.println("hl_java_dispatch: cannot run " + main + ": " + e);
            return 1;
        }
    }

    /** The classes of the snippets JShell has now, newest first. */
    List<Class<?>> currentClasses() {
        Set<String> wrappers = currentWrappers();
        List<Class<?>> loaded = loader.loaded();
        var current = new ArrayList<Class<?>>();
        for (int i = loaded.size() - 1; i >= 0; i--) {
            Class<?> wrapper = loaded.get(i);
            while (wrapper.getEnclosingClass() != null)
                wrapper = wrapper.getEnclosingClass();
            if (wrappers.contains(wrapper.getName()))
                current.add(loaded.get(i));
        }
        return current;
    }

    private static Method mainOf(Class<?> c) {
        Method found = null;
        int rank = Integer.MAX_VALUE;
        for (Method m : c.getDeclaredMethods()) {
            if (!m.getName().equals("main") || m.getReturnType() != void.class || Modifier.isPrivate(m.getModifiers()))
                continue;
            if (!(m.getParameterCount() == 0 || takesArgs(m)))
                continue;
            int r = rank(m);
            if (r < rank) {
                rank = r;
                found = m;
            }
        }
        return found;
    }

    /** Java's order for a main (JEP 512): String[] before none, then static. */
    private static int rank(Method m) {
        return (takesArgs(m) ? 0 : 2) + (Modifier.isStatic(m.getModifiers()) ? 0 : 1);
    }

    private static boolean takesArgs(Method m) {
        return m.getParameterCount() == 1 && m.getParameterTypes()[0] == String[].class;
    }

    private static String name(Snippet s) {
        return s instanceof DeclarationSnippet d ? d.name() : s.source().strip();
    }

    /** javac's message, the line it is about, and a caret under the spot. */
    private void reportRejected(Snippet s) {
        String src = s.source();
        boolean any = false;
        for (Diag d : js.diagnostics(s).toList()) {
            if (!d.isError())
                continue;
            any = true;
            System.err.println("error: " + d.getMessage(Locale.ROOT));
            long pos = d.getStartPosition();
            if (pos < 0 || pos > src.length())
                continue;
            int start = src.lastIndexOf('\n', (int) pos - 1) + 1;
            int end = src.indexOf('\n', (int) pos);
            String line = src.substring(start, end < 0 ? src.length() : end);
            System.err.println("    " + line);
            System.err.println("    " + " ".repeat((int) pos - start) + "^");
        }
        if (!any)
            System.err.println("error: rejected: " + src.strip());
    }

    /** What the java launcher prints for an uncaught exception, down to
     *  the dispatcher's own frames. */
    static void reportException(Throwable t) {
        if (t instanceof UnresolvedReferenceException u) {
            System.err.println("error: " + name(u.getSnippet()) + " uses something not declared yet");
            return;
        }
        // The same, raised in a method a guest function call reached directly:
        // the stub JShell compiled for a declaration still waiting on another.
        if (t instanceof SPIResolutionException r && instance != null) {
            System.err.println("error: " + instance.waiting(r) + " uses something not declared yet");
            return;
        }
        System.err.print("Exception in thread \"main\" ");
        print(t, "");
    }

    private static void print(Throwable t, String prefix) {
        String type = t instanceof EvalException e ? e.getExceptionClassName() : t.getClass().getName();
        String msg = t.getMessage();
        System.err.println(prefix + type + (msg != null ? ": " + msg : ""));
        StackTraceElement[] frames = t.getStackTrace();
        int end = 0;
        while (end < frames.length && !internal(frames[end].getClassName()))
            end++;
        // The reflection that called into the code, under the dispatcher.
        while (end > 0 && reflective(frames[end - 1].getClassName()))
            end--;
        for (int i = 0; i < end; i++) {
            StackTraceElement f = frames[i];
            // A snippet's own frames have no class: "at f(#6:1)" is line 1
            // of snippet 6, as the jshell tool shows them.
            System.err.println("\tat " + (f.getClassName().isEmpty()
                    ? f.getMethodName() + "(" + f.getFileName() + ":" + f.getLineNumber() + ")" : f));
        }
        if (t.getCause() != null && t.getCause() != t)
            print(t.getCause(), "Caused by: ");
    }

    /** The dispatcher's own frames and JShell's, where the code's stack ends. */
    private static boolean internal(String c) {
        return c.startsWith("hyperlight.Snippets") || c.startsWith("hyperlight.GuestFunctions")
                || c.startsWith("hyperlight.Dispatch") || c.startsWith("jdk.jshell.");
    }

    private static boolean reflective(String c) {
        return c.startsWith("jdk.internal.") || c.startsWith("java.lang.reflect.");
    }

    /** JShell's direct engine, with our loader so we see what it loads, each
     *  snippet run in a thread of the dispatcher's group that it waits for.
     *  JShell's local engine runs each in a thread group of its own, which
     *  drops what a thread the snippet started throws, where Java prints it;
     *  run on the dispatcher's own thread, a snippet keeps JShell's work in
     *  other threads waiting on the one vCPU, and the guest boots 0.8 s
     *  slower. */
    private record Engine(SnippetLoader loader) implements ExecutionControlProvider {
        @Override
        public String name() {
            return "hyperlight";
        }

        @Override
        public ExecutionControl generate(ExecutionEnv env, Map<String, String> parameters) {
            return new DirectExecutionControl(loader) {
                // With no value: JShell renders every expression's and
                // variable's value as text for the jshell tool to show,
                // Arrays.toString of a 40 MiB array or the value's own
                // toString() run uninvited, and nothing here shows them.
                @Override
                protected String invoke(Method doitMethod) throws Exception {
                    Throwable[] thrown = new Throwable[1];
                    Thread t = new Thread(() -> {
                        try {
                            doitMethod.invoke(null);
                        } catch (Throwable e) {
                            thrown[0] = e;
                        }
                    }, "main");
                    t.start();
                    t.join();
                    if (thrown[0] instanceof Exception e)
                        throw e;
                    if (thrown[0] instanceof Error e)
                        throw e;
                    return "";
                }

                @Override
                public String varValue(String className, String varName) {
                    return "";
                }
            };
        }
    }
}
