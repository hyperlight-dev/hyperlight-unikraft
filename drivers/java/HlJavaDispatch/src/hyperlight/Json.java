// JSON in and out of the guest: a guest function's input and result, and a
// host function's arguments and reply.  The JDK has no JSON of its own, so
// this is the small subset the calls need: objects are Maps, arrays Lists,
// numbers Long (BigInteger past it) or Double, and a value converts to a parameter's type
// (records, collections, arrays, boxes, enums) by reflection.

package hyperlight;

import java.lang.reflect.Array;
import java.lang.reflect.Constructor;
import java.lang.reflect.GenericArrayType;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.RecordComponent;
import java.lang.reflect.Type;
import java.lang.reflect.WildcardType;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.nio.file.Path;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collection;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

public final class Json {
    private Json() {}

    /** The value of {@code text}: a Map, a List, a String, a Long (a
     *  BigInteger past a long's range) or a Double, a Boolean, or null.
     *  IllegalArgumentException if it is not JSON, or a number no double
     *  holds. */
    public static Object parse(String text) {
        var p = new Parser(text);
        p.ws();
        Object v = p.value();
        p.ws();
        if (p.i != text.length())
            throw p.error("trailing characters");
        return v;
    }

    /** {@code text} parsed and converted to {@code type}. */
    public static <T> T parse(String text, Class<T> type) {
        @SuppressWarnings("unchecked")
        T t = (T) convert(parse(text), type);
        return t;
    }

    /** {@code value} as JSON: maps and records as objects, iterables and
     *  arrays as arrays, and anything else that is not a string, number,
     *  boolean, enum or null as its toString(). */
    public static String stringify(Object value) {
        var sb = new StringBuilder();
        write(sb, value);
        return sb.toString();
    }

    // ── Conversion ─────────────────────────────────────────────

    /** A parsed value as {@code type}, a parameter's generic type. */
    public static Object convert(Object v, Type type) {
        Class<?> raw = raw(type);
        if (raw == Object.class)
            return v;
        if (v == null) {
            if (raw.isPrimitive())
                throw new IllegalArgumentException("null is not a " + raw.getName());
            return raw == Optional.class ? Optional.empty() : null;
        }
        if (raw == Optional.class)
            return Optional.ofNullable(convert(v, arg(type, 0)));
        if (raw == String.class)
            return expect(v, String.class, raw);
        if (raw == boolean.class || raw == Boolean.class)
            return expect(v, Boolean.class, raw);
        if (raw == char.class || raw == Character.class) {
            String s = expect(v, String.class, raw);
            if (s.length() != 1)
                throw new IllegalArgumentException("\"" + s + "\" is not one character");
            return s.charAt(0);
        }
        if (Number.class.isAssignableFrom(box(raw)))
            return number(expect(v, Number.class, raw), box(raw));
        if (raw.isEnum())
            return enumValue(raw, expect(v, String.class, raw));
        if (raw.isArray()) {
            List<?> list = expect(v, List.class, raw);
            Type component = type instanceof GenericArrayType g ? g.getGenericComponentType() : raw.getComponentType();
            Object array = Array.newInstance(raw.getComponentType(), list.size());
            for (int i = 0; i < list.size(); i++)
                Array.set(array, i, convert(list.get(i), component));
            return array;
        }
        if (Collection.class.isAssignableFrom(raw)) {
            List<?> list = expect(v, List.class, raw);
            Collection<Object> out = collection(raw);
            for (Object e : list) {
                Object item = convert(e, arg(type, 0));
                if (item == null && (out instanceof ArrayDeque || out instanceof TreeSet))
                    throw new IllegalArgumentException("a " + raw.getSimpleName() + " cannot hold null");
                out.add(item);
            }
            return out;
        }
        if (Map.class.isAssignableFrom(raw)) {
            Map<?, ?> map = expect(v, Map.class, raw);
            Map<Object, Object> out = map(raw);
            for (var e : map.entrySet())
                out.put(key((String) e.getKey(), arg(type, 0)), convert(e.getValue(), arg(type, 1)));
            return out;
        }
        if (raw.isRecord())
            return record(raw, expect(v, Map.class, raw));
        if (raw.isInstance(v))
            return v;
        throw new IllegalArgumentException("cannot convert JSON to " + type.getTypeName()
                + ": take a record, a Map, a List, an array or a simple value");
    }

    /** A collection of the parameter's type: the usual implementation of an
     *  interface, or the class itself when it can be made. */
    @SuppressWarnings("unchecked")
    private static Collection<Object> collection(Class<?> raw) {
        if (raw.isAssignableFrom(ArrayList.class))
            return new ArrayList<>();
        if (raw.isAssignableFrom(LinkedHashSet.class))
            return new LinkedHashSet<>();
        if (raw.isAssignableFrom(TreeSet.class))
            return new TreeSet<>();
        if (raw.isAssignableFrom(ArrayDeque.class))
            return new ArrayDeque<>();
        return (Collection<Object>) make(raw);
    }

    @SuppressWarnings("unchecked")
    private static Map<Object, Object> map(Class<?> raw) {
        if (raw.isAssignableFrom(LinkedHashMap.class))
            return new LinkedHashMap<>();
        if (raw.isAssignableFrom(TreeMap.class))
            return new TreeMap<>();
        return (Map<Object, Object>) make(raw);
    }

    private static Object make(Class<?> raw) {
        try {
            return raw.getDeclaredConstructor().newInstance();
        } catch (ReflectiveOperationException e) {
            throw new IllegalArgumentException("cannot make a " + raw.getName() + " for JSON", e);
        }
    }

    /** An object's key, which JSON writes as a string, as the map's key type:
     *  a number, a boolean, an enum or a character from its text. */
    private static Object key(String k, Type type) {
        Class<?> raw = raw(type);
        if (raw == Object.class || raw == String.class || raw == CharSequence.class)
            return k;
        if (raw.isEnum() || raw == Character.class)
            return convert(k, type);
        try {
            return convert(parse(k), type);
        } catch (IllegalArgumentException e) {
            throw new IllegalArgumentException("key \"" + k + "\" is not a " + raw.getSimpleName(), e);
        }
    }

    private static Object record(Class<?> raw, Map<?, ?> map) {
        RecordComponent[] cs = raw.getRecordComponents();
        Class<?>[] types = new Class<?>[cs.length];
        Object[] args = new Object[cs.length];
        for (int i = 0; i < cs.length; i++) {
            types[i] = cs[i].getType();
            args[i] = convert(field(map, cs[i].getName()), cs[i].getGenericType());
        }
        try {
            Constructor<?> c = raw.getDeclaredConstructor(types);
            c.setAccessible(true);
            return c.newInstance(args);
        } catch (ReflectiveOperationException e) {
            Throwable cause = e.getCause() != null ? e.getCause() : e;
            throw new IllegalArgumentException("cannot make a " + raw.getSimpleName() + ": " + cause, cause);
        }
    }

    /** A field by name, case-insensitively when the exact name is absent:
     *  the camelCase the other runtimes write matches a Java name, and a
     *  PascalCase one does too. */
    private static Object field(Map<?, ?> map, String name) {
        if (map.containsKey(name))
            return map.get(name);
        for (var e : map.entrySet())
            if (e.getKey() instanceof String k && k.equalsIgnoreCase(name))
                return e.getValue();
        return null;
    }

    private static Object number(Number n, Class<?> boxed) {
        if (boxed == Double.class)
            return n.doubleValue();
        if (boxed == Float.class) {
            float f = n.floatValue();
            if (Float.isInfinite(f))
                throw new IllegalArgumentException(n + " does not fit a Float");
            return f;
        }
        if (boxed == BigDecimal.class)
            return new BigDecimal(n.toString());
        if (boxed == Number.class)
            return n;
        if (n instanceof Double d && (d != Math.rint(d) || d.isInfinite()))
            throw new IllegalArgumentException(n + " is not a whole number");
        BigInteger i = n instanceof BigInteger b ? b : new BigDecimal(n.toString()).toBigIntegerExact();
        if (boxed == BigInteger.class)
            return i;
        if (i.bitLength() >= 64)
            throw new IllegalArgumentException(n + " does not fit a " + boxed.getSimpleName());
        long l = i.longValue();
        if (boxed == Long.class)
            return l;
        if (boxed == Integer.class && l == (int) l)
            return (int) l;
        if (boxed == Short.class && l == (short) l)
            return (short) l;
        if (boxed == Byte.class && l == (byte) l)
            return (byte) l;
        throw new IllegalArgumentException(n + " does not fit a " + boxed.getSimpleName());
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    private static Object enumValue(Class<?> raw, String name) {
        return Enum.valueOf((Class) raw, name);
    }

    private static <T> T expect(Object v, Class<T> json, Class<?> target) {
        if (!json.isInstance(v))
            throw new IllegalArgumentException("expected " + kind(json) + " for " + target.getSimpleName()
                    + ", got " + stringify(v));
        return json.cast(v);
    }

    private static String kind(Class<?> json) {
        if (json == Map.class) return "an object";
        if (json == List.class) return "an array";
        if (json == Number.class) return "a number";
        if (json == Boolean.class) return "a boolean";
        return "a string";
    }

    private static Class<?> raw(Type t) {
        if (t instanceof Class<?> c) return c;
        if (t instanceof ParameterizedType p) return raw(p.getRawType());
        if (t instanceof GenericArrayType g) return Array.newInstance(raw(g.getGenericComponentType()), 0).getClass();
        if (t instanceof WildcardType w) return raw(w.getUpperBounds()[0]);
        return Object.class;
    }

    private static Type arg(Type t, int i) {
        if (t instanceof ParameterizedType p && p.getActualTypeArguments().length > i)
            return p.getActualTypeArguments()[i];
        return Object.class;
    }

    private static Class<?> box(Class<?> c) {
        if (!c.isPrimitive()) return c;
        if (c == int.class) return Integer.class;
        if (c == long.class) return Long.class;
        if (c == double.class) return Double.class;
        if (c == float.class) return Float.class;
        if (c == short.class) return Short.class;
        if (c == byte.class) return Byte.class;
        if (c == boolean.class) return Boolean.class;
        if (c == char.class) return Character.class;
        return Void.class;
    }

    // ── Writing ────────────────────────────────────────────────

    private static void write(StringBuilder sb, Object v) {
        if (v == null) {
            sb.append("null");
        } else if (v instanceof String || v instanceof Character || v instanceof Enum<?>) {
            string(sb, v instanceof Enum<?> e ? e.name() : v.toString());
        } else if (v instanceof Boolean) {
            sb.append(v);
        } else if (v instanceof Double || v instanceof Float) {
            double d = ((Number) v).doubleValue();
            if (Double.isNaN(d) || Double.isInfinite(d))
                sb.append("null");
            else if (d == Math.rint(d) && Math.abs(d) < 1e15)
                sb.append((long) d);
            else
                // A float's own shortest form: widened, 0.1f is 0.10000000149011612.
                sb.append(v instanceof Float f ? f.toString() : Double.toString(d));
        } else if (v instanceof Number) {
            sb.append(v);
        } else if (v instanceof Optional<?> o) {
            write(sb, o.orElse(null));
        } else if (v instanceof Map<?, ?> m) {
            sb.append('{');
            boolean first = true;
            for (var e : m.entrySet()) {
                if (!first) sb.append(',');
                first = false;
                string(sb, e.getKey() instanceof Enum<?> k ? k.name() : String.valueOf(e.getKey()));
                sb.append(':');
                write(sb, e.getValue());
            }
            sb.append('}');
        } else if (v instanceof Record r) {
            sb.append('{');
            boolean first = true;
            for (RecordComponent c : r.getClass().getRecordComponents()) {
                if (!first) sb.append(',');
                first = false;
                string(sb, c.getName());
                sb.append(':');
                try {
                    var accessor = c.getAccessor();
                    accessor.setAccessible(true);
                    write(sb, accessor.invoke(r));
                } catch (ReflectiveOperationException e) {
                    throw new IllegalStateException("cannot read " + c.getName() + ": " + e, e);
                }
            }
            sb.append('}');
        } else if (v instanceof Path p) {
            // A Path iterates over its names, and a one-name path over itself.
            string(sb, p.toString());
        } else if (v instanceof Iterable<?> it) {
            sb.append('[');
            boolean first = true;
            for (Object e : it) {
                if (!first) sb.append(',');
                first = false;
                write(sb, e);
            }
            sb.append(']');
        } else if (v.getClass().isArray()) {
            sb.append('[');
            for (int i = 0, n = Array.getLength(v); i < n; i++) {
                if (i > 0) sb.append(',');
                write(sb, Array.get(v, i));
            }
            sb.append(']');
        } else {
            string(sb, v.toString());
        }
    }

    private static void string(StringBuilder sb, String s) {
        sb.append('"');
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            switch (c) {
                case '"' -> sb.append("\\\"");
                case '\\' -> sb.append("\\\\");
                case '\n' -> sb.append("\\n");
                case '\r' -> sb.append("\\r");
                case '\t' -> sb.append("\\t");
                case '\b' -> sb.append("\\b");
                case '\f' -> sb.append("\\f");
                default -> {
                    if (c < 0x20)
                        sb.append(String.format("\\u%04x", (int) c));
                    else
                        sb.append(c);
                }
            }
        }
        sb.append('"');
    }

    // ── Parsing ────────────────────────────────────────────────

    private static final class Parser {
        final String s;
        int i;

        Parser(String s) {
            this.s = s;
        }

        IllegalArgumentException error(String what) {
            return new IllegalArgumentException("invalid JSON at " + i + ": " + what);
        }

        void ws() {
            while (i < s.length() && " \t\r\n".indexOf(s.charAt(i)) >= 0)
                i++;
        }

        char peek() {
            if (i >= s.length())
                throw error("unexpected end");
            return s.charAt(i);
        }

        void expect(char c) {
            if (peek() != c)
                throw error("expected '" + c + "'");
            i++;
        }

        Object value() {
            char c = peek();
            switch (c) {
                case '{': return object();
                case '[': return array();
                case '"': return string();
                case 't': return literal("true", Boolean.TRUE);
                case 'f': return literal("false", Boolean.FALSE);
                case 'n': return literal("null", null);
                default:
                    if (c == '-' || (c >= '0' && c <= '9'))
                        return number();
                    throw error("unexpected '" + c + "'");
            }
        }

        Object literal(String word, Object v) {
            if (!s.startsWith(word, i))
                throw error("expected " + word);
            i += word.length();
            return v;
        }

        Map<String, Object> object() {
            var m = new LinkedHashMap<String, Object>();
            expect('{');
            ws();
            if (peek() == '}') {
                i++;
                return m;
            }
            while (true) {
                ws();
                String k = string();
                ws();
                expect(':');
                ws();
                m.put(k, value());
                ws();
                if (peek() == ',') {
                    i++;
                    continue;
                }
                expect('}');
                return m;
            }
        }

        List<Object> array() {
            var l = new ArrayList<Object>();
            expect('[');
            ws();
            if (peek() == ']') {
                i++;
                return l;
            }
            while (true) {
                ws();
                l.add(value());
                ws();
                if (peek() == ',') {
                    i++;
                    continue;
                }
                expect(']');
                return l;
            }
        }

        String string() {
            expect('"');
            var sb = new StringBuilder();
            while (true) {
                char c = peek();
                i++;
                if (c == '"')
                    return sb.toString();
                if (c < 0x20)
                    throw error("control character in a string");
                if (c != '\\') {
                    sb.append(c);
                    continue;
                }
                char e = peek();
                i++;
                switch (e) {
                    case '"', '\\', '/' -> sb.append(e);
                    case 'n' -> sb.append('\n');
                    case 'r' -> sb.append('\r');
                    case 't' -> sb.append('\t');
                    case 'b' -> sb.append('\b');
                    case 'f' -> sb.append('\f');
                    case 'u' -> {
                        // Four ASCII hex digits: parseInt would take a sign,
                        // Character.digit other scripts' digits.
                        int code = 0;
                        for (int k = 0; k < 4; k++) {
                            char h = i < s.length() ? s.charAt(i) : 'x';
                            int d = h >= '0' && h <= '9' ? h - '0'
                                    : h >= 'a' && h <= 'f' ? h - 'a' + 10
                                    : h >= 'A' && h <= 'F' ? h - 'A' + 10 : -1;
                            if (d < 0)
                                throw error("bad \\u escape");
                            code = code * 16 + d;
                            i++;
                        }
                        sb.append((char) code);
                    }
                    default -> throw error("bad escape \\" + e);
                }
            }
        }

        /** JSON's grammar, which Java's parsers are looser than (01, 1., +1):
         *  -?(0|[1-9][0-9]*)(.[0-9]+)?([eE][+-]?[0-9]+)? */
        Number number() {
            int start = i;
            if (at('-'))
                i++;
            if (at('0'))
                i++;
            else if (digits() == 0)
                throw error("bad number");
            boolean integral = true;
            if (at('.')) {
                i++;
                integral = false;
                if (digits() == 0)
                    throw error("bad number: no digits after '.'");
            }
            if (at('e') || at('E')) {
                i++;
                integral = false;
                if (at('+') || at('-'))
                    i++;
                if (digits() == 0)
                    throw error("bad number: no digits in the exponent");
            }
            String t = s.substring(start, i);
            if (!integral) {
                double d = Double.parseDouble(t);
                if (Double.isInfinite(d)) {
                    i = start;
                    throw error("number out of range: " + t);
                }
                return d;
            }
            BigInteger b = new BigInteger(t);
            return b.bitLength() < 64 ? (Number) b.longValue() : b;
        }

        private boolean at(char c) {
            return i < s.length() && s.charAt(i) == c;
        }

        private int digits() {
            int n = 0;
            while (i < s.length() && s.charAt(i) >= '0' && s.charAt(i) <= '9') {
                i++;
                n++;
            }
            return n;
        }
    }
}
