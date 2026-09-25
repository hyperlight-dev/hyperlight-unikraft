/*
 * hl_py.h — shared CPython driver core for hl_pydriver and hl_pywarmdriver.
 *
 * The agent driver is the python driver plus a boot-time pre-warm of heavy
 * imports; everything else — the env→os.environ bridge, the dispatch body
 * (Exec code, a GuestExec launcher, or a Call), the hyperlight module
 * (host functions) and the serving loop — is identical,
 * so it lives here and both drivers end their main() with hl_py_serve().
 *
 * Include <Python.h>, hl_fc.h, hl_env.h and hl_driver.h before this header,
 * and call hl_py_preinit() before Py_Initialize().
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

/* ── Calls and host functions ──────────────────────────────────── */

/* The thread serving a call, 0 between calls: hyperlight.call works
 * only there (see hl_host_call). */
static unsigned long g_hl_py_serving;

static PyObject *g_hl_py_dumps, *g_hl_py_loads, *g_hl_py_host_error;
static PyObject *g_hl_py_call_fn;	/* hyperlight._call */

/* hyperlight.call(name, *args): the host function `name`, its arguments
 * and result as JSON. */
static PyObject *hl_py_host_call(PyObject *self, PyObject *const *args,
				 Py_ssize_t nargs)
{
	PyObject *list, *json, *ret;
	const char *name, *a, *out;
	Py_ssize_t name_len, a_len;
	size_t out_len;
	int rc;

	(void)self;
	if (nargs < 1 || !PyUnicode_Check(args[0])) {
		PyErr_SetString(PyExc_TypeError, "hyperlight.call(name, *args)");
		return NULL;
	}
	name = PyUnicode_AsUTF8AndSize(args[0], &name_len);
	if (!name)
		return NULL;
	/* The empty name asks the host for its list of functions. */
	if (!name_len) {
		PyErr_SetString(PyExc_ValueError, "hyperlight.call: a function name is required");
		return NULL;
	}
	if (!g_hl_py_serving || PyThread_get_thread_ident() != g_hl_py_serving) {
		PyErr_SetString(PyExc_RuntimeError,
				"hyperlight.call: only from the thread running a call");
		return NULL;
	}
	list = PyList_New(nargs - 1);
	if (!list)
		return NULL;
	for (Py_ssize_t i = 1; i < nargs; i++)
		PyList_SET_ITEM(list, i - 1, Py_NewRef(args[i]));
	json = PyObject_CallOneArg(g_hl_py_dumps, list);
	Py_DECREF(list);
	if (!json)
		return NULL;
	a = PyUnicode_AsUTF8AndSize(json, &a_len);
	if (!a) {
		Py_DECREF(json);
		return NULL;
	}
	rc = hl_host_call(name, (size_t)name_len, a, (size_t)a_len, &out, &out_len);
	int saved_errno = errno;

	Py_DECREF(json);
	if (rc < 0) {
		PyErr_Format(PyExc_RuntimeError, "host function %s: %s", name,
			     strerror(saved_errno));
		return NULL;
	}
	ret = PyUnicode_DecodeUTF8(out, (Py_ssize_t)out_len, rc ? "replace" : NULL);
	if (!ret)
		return NULL;
	if (rc > 0) {
		PyErr_SetObject(g_hl_py_host_error, ret);
		Py_DECREF(ret);
		return NULL;
	}
	if (!out_len) {
		Py_DECREF(ret);
		Py_RETURN_NONE;
	}
	Py_SETREF(ret, PyObject_CallOneArg(g_hl_py_loads, ret));
	return ret;
}

static PyMethodDef hl_py_methods[] = {
	{ "call", (PyCFunction)(void (*)(void))hl_py_host_call, METH_FASTCALL,
	  "call(name, *args): the host function `name`, arguments and result as JSON" },
	{ NULL, NULL, 0, NULL },
};

static struct PyModuleDef hl_py_module = {
	PyModuleDef_HEAD_INIT, "hyperlight",
	"The embedder's host functions, and the driver's side of calls.", -1,
	hl_py_methods, NULL, NULL, NULL, NULL,
};

/* In Python: `host.math.add(2, 3)` is `call("math.add", 2, 3)`, as
 * `import { add } from "host:math"` is in the quickjs image; and the
 * body of a call, run by hl_py_call. */
static const char HL_PY_MODULE_SRC[] =
	"import json as _json, sys as _sys\n"
	"class _Host:\n"
	"    def __init__(self, prefix=''):\n"
	"        self._prefix = prefix\n"
	"    def __getattr__(self, name):\n"
	"        if name.startswith('__'):\n"
	"            raise AttributeError(name)\n"
	"        return _Host(self._prefix + name + '.')\n"
	"    def __call__(self, *args):\n"
	"        return call(self._prefix[:-1], *args)\n"
	"host = _Host()\n"
	"_main = {}\n"
	"def _call(name, text):\n"
	"    # The file --guest-exec ran last (runpy runs it in a module of its\n"
	"    # own and hands back its globals), else __main__.\n"
	"    fn = _main.get(name)\n"
	"    if not callable(fn):\n"
	"        fn = vars(_sys.modules['__main__']).get(name)\n"
	"    if not callable(fn):\n"
	"        raise NameError(f'no function {name!r}: define it (def {name}(input): ...) before calling it')\n"
	"    r = fn(_json.loads(text)) if text else fn()\n"
	"    if hasattr(r, '__await__'):\n"
	"        import asyncio\n"
	"        async def wait():\n"
	"            return await r\n"
	"        r = asyncio.run(wait())\n"
	"    return None if r is None else _json.dumps(r)\n";

static PyObject *hl_py_module_init(void)
{
	PyObject *m = PyModule_Create(&hl_py_module), *json;

	if (!m)
		return NULL;
	g_hl_py_host_error = PyErr_NewExceptionWithDoc(
		"hyperlight.HostError", "A host function returned an error.", NULL, NULL);
	json = PyImport_ImportModule("json");
	if (!g_hl_py_host_error || !json ||
	    PyModule_AddObjectRef(m, "HostError", g_hl_py_host_error) < 0)
		goto fail;
	g_hl_py_dumps = PyObject_GetAttrString(json, "dumps");
	g_hl_py_loads = PyObject_GetAttrString(json, "loads");
	Py_CLEAR(json);
	if (!g_hl_py_dumps || !g_hl_py_loads)
		goto fail;
	PyObject *r = PyRun_String(HL_PY_MODULE_SRC, Py_file_input,
				   PyModule_GetDict(m), PyModule_GetDict(m));
	if (!r)
		goto fail;
	Py_DECREF(r);
	g_hl_py_call_fn = PyObject_GetAttrString(m, "_call");
	if (!g_hl_py_call_fn)
		goto fail;
	return m;
fail:
	Py_XDECREF(json);
	Py_DECREF(m);
	return NULL;
}

/* Register the hyperlight module; before Py_Initialize. */
static inline void hl_py_preinit(void)
{
	PyImport_AppendInittab("hyperlight", hl_py_module_init);
}

/*
 * Call the function `name` of the file a GuestExec ran last, else of
 * __main__, with `input` parsed as JSON (no
 * argument for empty input); what it returns, awaited if it is
 * awaitable, is the call's result as JSON, None none.  A SystemExit
 * ends the call with its status, any other exception with 1.
 */
static int hl_py_call(const char *name, size_t name_len, const char *input,
		      size_t input_len)
{
	PyObject *n, *in, *res = NULL;
	int status = 0;

	if (!g_hl_py_call_fn) {
		PyObject *m = PyImport_ImportModule("hyperlight");

		Py_XDECREF(m);
		if (!g_hl_py_call_fn) {
			PyErr_Print();
			return 1;
		}
	}
	n = PyUnicode_DecodeUTF8(name, (Py_ssize_t)name_len, NULL);
	in = n ? PyUnicode_DecodeUTF8(input, (Py_ssize_t)input_len, NULL) : NULL;
	if (in)
		res = PyObject_CallFunctionObjArgs(g_hl_py_call_fn, n, in, NULL);
	Py_XDECREF(n);
	Py_XDECREF(in);
	if (!res) {
		if (PyErr_ExceptionMatches(PyExc_SystemExit))
			return hl_py_exit_status();
		PyErr_Print();
		return 1;
	}
	if (res != Py_None) {
		Py_ssize_t len;
		const char *text = PyUnicode_AsUTF8AndSize(res, &len);

		if (!text) {
			PyErr_Print();
			status = 1;
		} else if (hl_set_result(text, (size_t)len) < 0) {
			fprintf(stderr, "hl_py: %.*s's result does not fit a call\n",
				(int)name_len, name);
			status = 1;
		}
	}
	Py_DECREF(res);
	return status;
}

static int hl_py_run(const uint8_t *fc, size_t fc_len)
{
	hl_py_env_refresh();

	/* Extract the code string from the FunctionCall FlatBuffer */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	if (fc_name_is(fc, fc_len, "Call")) {
		size_t in_len;
		const char *in = fc_arg_string(fc, fc_len, 1, &in_len);
		int rc;

		if (!code_len || !in)
			return -1;
		rc = hl_py_call(code, code_len, in, in_len);
		fflush(stdout);
		fflush(stderr);
		return rc;
	}

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
			"import os, sys, shlex, runpy, hyperlight\n"
			"_a = shlex.split(_HL_CMD)\n"
			"if _a:\n"
			"    sys.argv = _a\n"
			"    hyperlight._main = runpy.run_path(_a[0], run_name='__main__')\n"
			"elif os.path.exists('/entrypoint.py'):\n"
			"    sys.argv = ['/entrypoint.py']\n"
			"    hyperlight._main = runpy.run_path('/entrypoint.py', run_name='__main__')\n"
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
	g_hl_py_serving = PyThread_get_thread_ident();
	rc = hl_py_run(fc, fc_len);
	g_hl_py_serving = 0;
	g_hl_py_tstate = PyEval_SaveThread();
	return rc;
}

/* Serve calls forever, parked with the interpreter lock released. */
static inline __attribute__((noreturn)) void hl_py_serve(void)
{
	/* Imported at boot, so it is in a warm snapshot and a call's
	 * first use costs nothing. */
	PyObject *m = PyImport_ImportModule("hyperlight");

	if (!m)
		PyErr_Print();
	Py_XDECREF(m);
	hl_driver_serve_calls();
	g_hl_py_tstate = PyEval_SaveThread();
	hl_driver_run(hl_py_dispatch);
}

#endif /* HL_PY_H */
