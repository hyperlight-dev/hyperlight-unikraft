/*
 * hl_py.h — shared CPython driver core for hl_pydriver and hl_pywarmdriver.
 *
 * The agent driver is the python driver plus a boot-time pre-warm of heavy
 * imports; everything else — FS_BASE save/restore, the env→os.environ bridge,
 * and the dispatch body (Exec code or a GuestExec launcher) — is identical, so
 * it lives here and both drivers register hl_py_dispatch().
 *
 * Include <Python.h>, hl_fc.h and hl_env.h before this header.
 */

#ifndef HL_PY_H
#define HL_PY_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ── FS_BASE save/restore ──────────────────────────────────────── */

/*
 * CPython's thread-local storage is keyed off FS_BASE.  The host resets
 * general-purpose registers and FPU between dispatch calls but preserves
 * segment registers and FS_BASE in both normal and snapshot/restore paths.
 * We save/restore here as a defensive measure — kernel exception handlers
 * during CoW prefault could theoretically modify FS_BASE.
 *
 * We use the RDFSBASE/WRFSBASE instructions rather than
 * rdmsr/wrmsr(IA32_FS_BASE): the hyperlight platform enables CR4.FSGSBASE,
 * and from hyperlight 0.17.0 the guest runs behind a default-deny KVM MSR
 * filter that faults (#GP) on a bare FS_BASE MSR access.  The instructions
 * read/write the same base directly.
 */
static uint64_t g_py_fsbase;

static inline uint64_t rd_fsbase(void)
{
	uint64_t v;
	__asm__ volatile("rdfsbase %0" : "=r"(v));
	return v;
}

static inline void wr_fsbase(uint64_t v)
{
	__asm__ volatile("wrfsbase %0" : : "r"(v));
}

/* ── env → os.environ bridge ───────────────────────────────────── */

/*
 * Build `os.environ["KEY"]="VALUE"\n` lines for the interpreter.  Python's
 * os.environ is the authoritative source for env vars in user scripts —
 * glibc's environ alone isn't enough because CPython caches os.environ at
 * startup.  We use assignment rather than `export` because this runs through
 * PyRun_SimpleString, not a shell.
 */
struct hl_py_env_buf {
	char *buf;
	size_t pos;
	size_t cap;
};

static void hl_py_env_visitor(const char *key, const char *val, void *ctx)
{
	struct hl_py_env_buf *eb = (struct hl_py_env_buf *)ctx;
	size_t need = 16 + strlen(key) + strlen(val) + 8;
	if (eb->pos + need >= eb->cap)
		return;
	eb->pos += snprintf(eb->buf + eb->pos, eb->cap - eb->pos,
			    "os.environ[\"%s\"]=\"%s\"\n", key, val);
}

/* ── Dispatch callback ─────────────────────────────────────────── */

/*
 * Restore FS_BASE, refresh env vars, then run the payload: either inline
 * `Exec` code, or a `GuestExec` command (--guest-exec / autonomous) that runs
 * the named guest file with argv — or the conventional /entrypoint.py when the
 * command is empty.  Registered as the dispatch callback by both drivers.
 */
static int hl_py_dispatch(const uint8_t *fc, size_t fc_len)
{
	if (g_py_fsbase)
		wr_fsbase(g_py_fsbase);

	/* Refresh env vars — updates glibc environ and builds a Python
	 * prefix that sets os.environ for each host var. */
	char env_prefix[4096];
	struct hl_py_env_buf eb = { env_prefix, 0, sizeof(env_prefix) };
	hl_env_refresh(hl_py_env_visitor, &eb);

	/* Prepend "import os\n" if we have any env assignments */
	char full_prefix[4096 + 16];
	size_t prefix_len = 0;
	if (eb.pos > 0) {
		prefix_len = snprintf(full_prefix, sizeof(full_prefix),
				      "import os\n%.*s", (int)eb.pos, env_prefix);
	}

	/* Extract the code string from the FunctionCall FlatBuffer */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	/* Guest command ("GuestExec" — --guest-exec / autonomous): run the named
	 * guest file with argv, or the conventional /entrypoint.py when empty.
	 * The command (arg0) is handed to Python as a variable (no source
	 * escaping) and shlex splits it into argv. */
	size_t gx_len = code_len;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? code : NULL;
	if (gx) {
		PyObject *mainmod = PyImport_AddModule("__main__");
		if (mainmod) {
			PyObject *cmdobj =
				PyUnicode_FromStringAndSize(gx, (Py_ssize_t)gx_len);
			if (cmdobj) {
				PyDict_SetItemString(PyModule_GetDict(mainmod),
						     "_HL_CMD", cmdobj);
				Py_DECREF(cmdobj);
			}
		}
		static const char LAUNCHER[] =
			"import os, sys, shlex, runpy\n"
			"_a = shlex.split(_HL_CMD)\n"
			"if _a:\n"
			"    sys.argv = _a\n"
			"    runpy.run_path(_a[0], run_name='__main__')\n"
			"elif os.path.exists('/entrypoint.py'):\n"
			"    sys.argv = ['/entrypoint.py']\n"
			"    runpy.run_path('/entrypoint.py', run_name='__main__')\n"
			"else:\n"
			"    print('hl: no /entrypoint.py in rootfs; nothing to run')\n";
		code = LAUNCHER;
		code_len = sizeof(LAUNCHER) - 1;
	}

	/* NUL-terminate — fc_arg0_string returns a non-terminated slice */
	size_t total = prefix_len + code_len;
	char stack_buf[4096];
	char *buf;
	if (total < sizeof(stack_buf)) {
		buf = stack_buf;
	} else {
		buf = malloc(total + 1);
		if (!buf)
			return -1;
	}
	if (prefix_len > 0)
		memcpy(buf, full_prefix, prefix_len);
	memcpy(buf + prefix_len, code, code_len);
	buf[total] = '\0';

	int rc = PyRun_SimpleString(buf);

	if (buf != stack_buf)
		free(buf);

	fflush(stdout);
	fflush(stderr);

	return rc;
}

#endif /* HL_PY_H */
