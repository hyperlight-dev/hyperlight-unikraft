/*
 * hl_py.h — shared CPython driver core for hl_pydriver and hl_pywarmdriver.
 *
 * The agent driver is the python driver plus a boot-time pre-warm of heavy
 * imports; everything else — the env→os.environ bridge, the dispatch body
 * (Exec code or a GuestExec launcher) and the serving loop — is identical,
 * so it lives here and both drivers end their main() with hl_py_serve().
 *
 * Include <Python.h>, hl_fc.h and hl_env.h before this header.
 */

#ifndef HL_PY_H
#define HL_PY_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ── env → os.environ bridge ───────────────────────────────────── */

/*
 * CPython copies the C environ into os.environ at startup and reads that
 * copy from then on, so setenv() alone is invisible to user code.  Each
 * host variable is set on os.environ through the C API: no source text,
 * so nothing to quote and no limit on count or length, and the user's
 * code keeps its own line numbers in tracebacks.
 */
static void hl_py_env_visitor(const char *key, const char *val, void *ctx)
{
	PyObject *environ = ctx;
	PyObject *k = PyUnicode_DecodeFSDefault(key);
	PyObject *v = PyUnicode_DecodeFSDefault(val);

	if (!k || !v || PyObject_SetItem(environ, k, v) < 0)
		PyErr_Print();
	Py_XDECREF(k);
	Py_XDECREF(v);
}

/* Refresh the environment: setenv() for the C side, os.environ for
 * Python's. */
static void hl_py_env_refresh(void)
{
	PyObject *os = PyImport_ImportModule("os");
	PyObject *environ = os ? PyObject_GetAttrString(os, "environ") : NULL;

	if (!environ)
		PyErr_Print();
	hl_env_refresh(environ ? hl_py_env_visitor : NULL, environ);
	Py_XDECREF(environ);
	Py_XDECREF(os);
}

/* ── Dispatch callback ─────────────────────────────────────────── */

/*
 * Refresh env vars, then run the payload: either inline
 * `Exec` code, or a `GuestExec` command (--guest-exec / autonomous) that runs
 * the named guest file with argv — or the conventional /entrypoint.py when the
 * command is empty.  The call ends when that code returns or raises
 * SystemExit, whose code is the call's status as it would be a script's;
 * threads it started, daemon or not, are left behind and make progress
 * on later entries.  (Joining them, as `python file.py` does at exit, is
 * not an option: the interpreter's exit path also runs the callbacks
 * that stop thread pools, which a live interpreter cannot afford, and
 * without them a module-level ThreadPoolExecutor would keep the call
 * open forever.)
 * Runs with the interpreter lock held.
 */
/*
 * The status a SystemExit carries, as the interpreter computes it for a
 * script: None is 0, an int is itself, anything else is printed to
 * stderr and is 1.  Consumes the raised exception.
 */
static int hl_py_exit_status(void)
{
	PyObject *exc = PyErr_GetRaisedException();
	PyObject *code = exc ? PyObject_GetAttrString(exc, "code") : NULL;
	int status;

	if (!code) {
		status = 1;
	} else if (code == Py_None) {
		status = 0;
	} else if (PyLong_Check(code)) {
		/* A process exit status: eight bits, so -1 is 255 as at exit(). */
		status = (int)(PyLong_AsLong(code) & 0xff);
	} else {
		PyObject *err = PySys_GetObject("stderr");	/* borrowed */

		if (err && PyFile_WriteObject(code, err, Py_PRINT_RAW) == 0)
			PyFile_WriteString("\n", err);
		status = 1;
	}
	PyErr_Clear();
	Py_XDECREF(code);
	Py_XDECREF(exc);
	return status;
}

/*
 * Run @src in __main__ as PyRun_SimpleString does, except that a
 * SystemExit ends the call and not the interpreter: PyRun_SimpleString's
 * error printer exits the process, and this process is the guest.  Any
 * other exception is printed and fails the call with status 1.
 */
static int hl_py_exec(const char *src)
{
	PyObject *mainmod = PyImport_AddModule("__main__");	/* borrowed */
	PyObject *globals, *res;

	if (!mainmod) {
		PyErr_Print();
		return -1;
	}
	globals = PyModule_GetDict(mainmod);			/* borrowed */
	res = PyRun_String(src, Py_file_input, globals, globals);
	if (res) {
		Py_DECREF(res);
		return 0;
	}
	if (PyErr_ExceptionMatches(PyExc_SystemExit))
		return hl_py_exit_status();
	/* As `python script.py` exits on an uncaught exception: 1. */
	PyErr_Print();
	return 1;
}

static int hl_py_run(const uint8_t *fc, size_t fc_len)
{
	hl_py_env_refresh();

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
	char stack_buf[4096];
	char *buf;
	if (code_len < sizeof(stack_buf)) {
		buf = stack_buf;
	} else {
		buf = malloc(code_len + 1);
		if (!buf)
			return -1;
	}
	memcpy(buf, code, code_len);
	buf[code_len] = '\0';

	int rc = hl_py_exec(buf);

	if (buf != stack_buf)
		free(buf);

	fflush(stdout);
	fflush(stderr);

	return rc;
}

/* ── Serving loop ──────────────────────────────────────────────── */

/*
 * The main thread's interpreter state while the driver is parked in
 * read().  The lock is released for the wait: a thread a call left
 * behind (a server it started) then runs between calls, and a thread
 * waiting on the lock does not wake every switch interval to ask for a
 * lock the parked thread could never hand over.
 */
static PyThreadState *g_hl_py_tstate;

static int hl_py_dispatch(const uint8_t *fc, size_t fc_len)
{
	int rc;

	PyEval_RestoreThread(g_hl_py_tstate);
	rc = hl_py_run(fc, fc_len);
	g_hl_py_tstate = PyEval_SaveThread();
	return rc;
}

/* Serve calls forever, parked with the interpreter lock released. */
static inline __attribute__((noreturn)) void hl_py_serve(void)
{
	g_hl_py_tstate = PyEval_SaveThread();
	hl_driver_run(hl_py_dispatch);
}

#endif /* HL_PY_H */
