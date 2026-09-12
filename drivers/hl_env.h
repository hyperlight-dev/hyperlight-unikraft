/*
 * hl_env.h — Shared environment variable refresh for Hyperlight drivers.
 *
 * After snapshot restore, the kernel re-injects host-provided env vars
 * into its own environ (via setenv in dispatch.c), but drivers that
 * were loaded as ELFs via app-elfloader have a SEPARATE glibc environ
 * that stays stale.  This header provides the common logic to query
 * the host and update glibc's environ on each dispatch.
 *
 * Usage:
 *   1. Call hl_env_init(envp) from main() to parse HL_GET_ENV_VARS_FN.
 *   2. Call hl_env_refresh(cb, ctx) at the top of each dispatch to
 *      setenv() all non-HL_ host vars.  The optional callback lets
 *      runtime-specific drivers do extra work per var (e.g. update
 *      Python's os.environ or inject `export` lines for bash).
 *
 * Depends on hl_parse_hex() from hl_fc.h — include hl_fc.h first.
 */

#ifndef HL_ENV_H
#define HL_ENV_H

#include <stdlib.h>
#include <string.h>

/* ── Kernel function pointer for querying host env vars ────────── */

typedef int (*hl_get_env_fn_t)(char *, size_t);

/*
 * Set by hl_env_init(); NULL if the kernel didn't export
 * HL_GET_ENV_VARS_FN (older kernel or evolve without env support).
 */
static hl_get_env_fn_t g_hl_get_env_fn;

/*
 * Parse HL_GET_ENV_VARS_FN from envp.
 * Also parses HL_DISPATCH_CALLBACK_PTR and HL_DISPATCH_ENTRY into
 * the caller-provided pointers (common to every driver's main()).
 */
static inline void hl_env_init(char **envp,
			       hl_dispatch_fn_t **out_cb_slot,
			       uint64_t *out_entry)
{
	for (char **p = envp; p && *p; p++) {
		if (!strncmp(*p, "HL_DISPATCH_CALLBACK_PTR=", 25))
			*out_cb_slot = (hl_dispatch_fn_t *)
				hl_parse_hex(*p + 25);
		else if (!strncmp(*p, "HL_DISPATCH_ENTRY=", 18))
			*out_entry = hl_parse_hex(*p + 18);
		else if (!strncmp(*p, "HL_GET_ENV_VARS_FN=", 19))
			g_hl_get_env_fn = (hl_get_env_fn_t)
				hl_parse_hex(*p + 19);
	}
}

/* ── Remove reserved HL_ vars from glibc environ ─────────────── */

/*
 * After hl_env_init() has parsed the HL_ addresses into globals,
 * remove all HL_ prefixed vars from glibc's environ so guest user
 * code (Python scripts, Node.js code, etc.) cannot read internal
 * addresses like HL_DISPATCH_CALLBACK_PTR via getenv().
 *
 * Must be called AFTER hl_env_init() — the globals are already set,
 * and the env vars are no longer needed.
 */
static inline void hl_env_clean_reserved(void)
{
	extern char **environ;
	/* Collect keys first — unsetenv modifies environ in place. */
	char keys[8][64];
	int n = 0;
	for (char **p = environ; p && *p && n < 8; p++) {
		if ((*p)[0] == 'H' && (*p)[1] == 'L' && (*p)[2] == '_') {
			char *eq = strchr(*p, '=');
			if (eq) {
				size_t klen = (size_t)(eq - *p);
				if (klen < 64) {
					memcpy(keys[n], *p, klen);
					keys[n][klen] = '\0';
					n++;
				}
			}
		}
	}
	for (int i = 0; i < n; i++)
		unsetenv(keys[i]);
}

/* ── Per-dispatch env refresh ──────────────────────────────────── */

/*
 * Optional callback invoked for each non-HL_ env var.
 * key and value are NUL-terminated (the '=' was temporarily zeroed).
 * The callback can use them for runtime-specific updates:
 *   - Python: PyObject_SetItem(os.environ, key, value)
 *   - Bash:   fprintf(f, "export %s='%s'\n", key, value)
 *   - Node:   append "process.env['key']='value';\n" to code
 */
typedef void (*hl_env_cb_t)(const char *key, const char *val, void *ctx);

/*
 * Query the host for env vars and inject them into glibc's environ
 * via setenv().  For each non-HL_ variable, also invokes cb (if
 * non-NULL) for runtime-specific propagation.
 *
 * On a normal boot this is redundant (glibc environ already matches
 * the kernel's) but cheap — one host call returning a small string.
 * After snapshot restore this is the only path that updates glibc's
 * environ with vars the host set after restore.
 *
 * Returns the number of vars set, or 0 if unavailable / no vars.
 */
static inline int hl_env_refresh(hl_env_cb_t cb, void *ctx)
{
	if (!g_hl_get_env_fn)
		return 0;

	char buf[4096];
	int len = g_hl_get_env_fn(buf, sizeof(buf));
	if (len <= 0)
		return 0;

	int count = 0;
	char *p = buf;
	char *end = buf + len;
	while (p < end) {
		char *eq;

		if (*p == '\0') {
			p++;
			continue;
		}
		/* Skip HL_ reserved keys */
		if (p[0] == 'H' && p[1] == 'L' && p[2] == '_')
			goto skip;

		eq = p;
		while (*eq && *eq != '=')
			eq++;
		if (*eq == '=') {
			*eq = '\0';
			setenv(p, eq + 1, 1);
			if (cb)
				cb(p, eq + 1, ctx);
			*eq = '=';
			count++;
		}
skip:
		while (p < end && *p)
			p++;
		p++;
	}

	return count;
}

#endif /* HL_ENV_H */
