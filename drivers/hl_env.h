/*
 * hl_env.h — host environment refresh for Hyperlight drivers.
 *
 * The kernel brings the host-provided environment variables into its own
 * environ before every named call, but a driver loaded as an ELF has a
 * separate libc environ that the kernel cannot reach, and it goes stale
 * the moment the host changes a variable -- after a snapshot restore,
 * typically.  So the driver asks the kernel for the host's current
 * environment (the HLCALL_IOC_GETENV ioctl on /dev/hlcall, see
 * hl_driver.h) at the top of each call it serves and setenv()s it.
 *
 * Usage: call hl_env_refresh(cb, ctx) at the top of each dispatch.  The
 * optional callback carries each variable into the runtime: Python sets
 * os.environ through the C API; bash, Node and .NET are fed source text
 * and quote the value with the helpers at the end of this header.
 */

#ifndef HL_ENV_H
#define HL_ENV_H

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>

#include "hl_driver.h" /* g_hl_call_fd, g_hl_call_cap, HLCALL_IOC_GETENV */

/*
 * Invoked for each variable.  key and value are NUL-terminated (the '='
 * is temporarily zeroed).
 */
typedef void (*hl_env_cb_t)(const char *key, const char *val, void *ctx);

/* The entries arrive on the same PEB stack as a call, so a call-sized
 * buffer always holds them; allocated on the first refresh, kept since
 * this runs on every call. */
static char *g_hl_env_buf;

/*
 * Fetch the host's environment and setenv() every variable, overriding
 * what was there; cb, if non-NULL, sees each one too.
 *
 * Returns the number of variables set: 0 if there are none, if the
 * device is not open, or on a kernel without the ioctl (then there is
 * nothing to refresh from).
 */
static inline int hl_env_refresh(hl_env_cb_t cb, void *ctx)
{
	struct hlcall_env env;
	int count = 0;

	if (g_hl_call_fd < 0)
		return 0;
	if (!g_hl_env_buf) {
		g_hl_env_buf = malloc(g_hl_call_cap);
		if (!g_hl_env_buf)
			return 0;
	}

	env.buf = g_hl_env_buf;
	env.cap = g_hl_call_cap;
	env.len = 0;
	if (ioctl(g_hl_call_fd, HLCALL_IOC_GETENV, &env) < 0) {
		if (errno != ENOTTY)
			fprintf(stderr, "hl_env: cannot read the host environment: %s\n",
				strerror(errno));
		return 0;
	}

	for (char *p = env.buf, *end = env.buf + env.len; p < end;
	     p += strlen(p) + 1) {
		char *eq = strchr(p, '=');

		if (!eq || eq == p)
			continue;
		*eq = '\0';
		setenv(p, eq + 1, 1);
		if (cb)
			cb(p, eq + 1, ctx);
		*eq = '=';
		count++;
	}

	return count;
}

/* ── Quoting the environment into runtime source ───────────────── */

/*
 * A driver whose runtime is a separate process (bash, Node, .NET) cannot
 * setenv() into it; it prefixes each call's source with one assignment
 * per variable instead.  The buffer grows with the environment -- the
 * kernel hands over all of it -- and the values are quoted as literals
 * of the target language, so no character in a value can alter the
 * source.  An allocation failure sets `err` and turns later appends
 * into no-ops; the driver then fails the call rather than send a
 * truncated prefix.
 */
struct hl_strbuf {
	char *buf;
	size_t len;
	size_t cap;
	int err;
};

static inline void hl_strbuf_put(struct hl_strbuf *b, const char *s, size_t n)
{
	if (b->err)
		return;
	if (b->len + n + 1 > b->cap) {
		size_t cap = b->cap ? b->cap : 4096;
		char *nb;

		while (cap < b->len + n + 1)
			cap *= 2;
		nb = realloc(b->buf, cap);
		if (!nb) {
			b->err = 1;
			return;
		}
		b->buf = nb;
		b->cap = cap;
	}
	memcpy(b->buf + b->len, s, n);
	b->len += n;
	b->buf[b->len] = '\0';
}

static inline void hl_strbuf_puts(struct hl_strbuf *b, const char *s)
{
	hl_strbuf_put(b, s, strlen(s));
}

static inline void hl_strbuf_free(struct hl_strbuf *b)
{
	free(b->buf);
	b->buf = NULL;
	b->len = b->cap = 0;
}

/* Append @s as a JSON string literal.  JavaScript and C# both read one as
 * a string of the same value: `"` and `\` escaped, control characters as
 * \n \t \r \b \f or \u00XX, every other byte as it is. */
static inline void hl_strbuf_put_json(struct hl_strbuf *b, const char *s)
{
	hl_strbuf_put(b, "\"", 1);
	for (const unsigned char *p = (const unsigned char *)s; *p; p++) {
		char u[8];

		switch (*p) {
		case '"':
			hl_strbuf_puts(b, "\\\"");
			break;
		case '\\':
			hl_strbuf_puts(b, "\\\\");
			break;
		case '\n':
			hl_strbuf_puts(b, "\\n");
			break;
		case '\r':
			hl_strbuf_puts(b, "\\r");
			break;
		case '\t':
			hl_strbuf_puts(b, "\\t");
			break;
		case '\b':
			hl_strbuf_puts(b, "\\b");
			break;
		case '\f':
			hl_strbuf_puts(b, "\\f");
			break;
		default:
			if (*p < 0x20) {
				snprintf(u, sizeof(u), "\\u%04x", *p);
				hl_strbuf_puts(b, u);
			} else {
				hl_strbuf_put(b, (const char *)p, 1);
			}
		}
	}
	hl_strbuf_put(b, "\"", 1);
}

/* Append @s single-quoted for a POSIX shell.  Nothing is special inside
 * single quotes but the quote itself, spelled '\'' -- close, one escaped
 * quote, reopen. */
static inline void hl_strbuf_put_shquoted(struct hl_strbuf *b, const char *s)
{
	hl_strbuf_put(b, "'", 1);
	for (const char *p = s; *p; p++) {
		if (*p == '\'')
			hl_strbuf_puts(b, "'\\''");
		else
			hl_strbuf_put(b, p, 1);
	}
	hl_strbuf_put(b, "'", 1);
}

/* Whether @key is a name a shell accepts in `export KEY=`: ASCII
 * letters, digits and underscore, not starting with a digit.  Spelled
 * out rather than isalpha(), which follows the process locale.  Other
 * keys are left to setenv() alone. */
static inline int hl_env_key_is_identifier(const char *key)
{
	for (const char *p = key; *p; p++) {
		int ok = (*p >= 'a' && *p <= 'z') || (*p >= 'A' && *p <= 'Z') ||
			 *p == '_' || (p != key && *p >= '0' && *p <= '9');

		if (!ok)
			return 0;
	}
	return *key != '\0';
}

#endif /* HL_ENV_H */
