// Where JShell's compiled snippets are loaded: its own LoaderDelegate, so
// the dispatcher knows every class a snippet defined, newest last, which is
// where a guest function call looks for the method it names.

package hyperlight;

import java.io.File;
import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import jdk.jshell.execution.LoaderDelegate;
import jdk.jshell.spi.ExecutionControl.ClassBytecodes;
import jdk.jshell.spi.ExecutionControl.ClassInstallException;
import jdk.jshell.spi.ExecutionControl.InternalException;

final class SnippetLoader implements LoaderDelegate {
    private final Map<String, byte[]> bytecodes = new ConcurrentHashMap<>();
    private final List<Class<?>> loaded = new ArrayList<>();
    private final Loader loader;

    SnippetLoader(ClassLoader parent) {
        loader = new Loader(parent);
    }

    /** Every class a snippet defined, oldest first. */
    synchronized List<Class<?>> loaded() {
        return List.copyOf(loaded);
    }

    @Override
    public synchronized void load(ClassBytecodes[] cbcs) throws ClassInstallException {
        boolean[] installed = new boolean[cbcs.length];
        for (ClassBytecodes cbc : cbcs)
            bytecodes.put(cbc.name(), cbc.bytecodes());
        for (int i = 0; i < cbcs.length; i++) {
            try {
                loaded.add(Class.forName(cbcs[i].name(), false, loader));
                installed[i] = true;
            } catch (ClassNotFoundException | LinkageError e) {
                throw new ClassInstallException("cannot load " + cbcs[i].name() + ": " + e, installed);
            }
        }
    }

    @Override
    public void classesRedefined(ClassBytecodes[] cbcs) {
        for (ClassBytecodes cbc : cbcs)
            bytecodes.put(cbc.name(), cbc.bytecodes());
    }

    @Override
    public void addToClasspath(String path) throws InternalException {
        try {
            for (String p : path.split(File.pathSeparator))
                loader.add(Path.of(p).toUri().toURL());
        } catch (MalformedURLException e) {
            throw new InternalException(e.toString());
        }
    }

    @Override
    public Class<?> findClass(String name) throws ClassNotFoundException {
        return Class.forName(name, false, loader);
    }

    private final class Loader extends URLClassLoader {
        Loader(ClassLoader parent) {
            // Parent first: a snippet sees the dispatcher's own hyperlight
            // classes, the very ones the dispatcher talks to the driver with.
            super(new URL[0], parent);
        }

        void add(URL url) {
            addURL(url);
        }

        @Override
        protected Class<?> findClass(String name) throws ClassNotFoundException {
            // Once defined, the class loader has it; the bytes are not needed.
            byte[] b = bytecodes.remove(name);
            if (b == null)
                return super.findClass(name);
            return defineClass(name, b, 0, b.length);
        }
    }
}
