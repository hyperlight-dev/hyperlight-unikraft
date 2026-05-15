/*
 * hl_pydriver — Python-runtime driver for Hyperlight.
 *
 * First call enters via main() (standard deferred-dispatch path that
 * app-elfloader sets up). main() does the one-time Py_Initialize and
 * warm-up imports, then installs an FC-aware dispatch callback via
 * hyperlight_dispatch_register_v2(). It also handles the first call's
 * user code so the caller gets their result.
 *
 * Every subsequent call goes through the v2 callback directly —
 * dispatch_inner sees g_dispatch_callback != NULL and invokes it
 * without touching the legacy g_run_callback that would otherwise
 * re-enter main(). Python interpreter state (sys.modules, the GIL,
 * heap allocations for numpy/pandas types) persists across calls.
 *
 * Flow:
 *   host: call("run", <code string>)           ┐
 *   guest dispatch → deferred_run → main()     │
 *   main reads HL_FC_{BYTES,LEN}_PTR from env  │
 *   Py_Initialize() + warm-up imports          │ ~2 s, once
 *   hyperlight_dispatch_register_v2(run_code)  │
 *   run_code(fc_bytes, fc_len)                 ┘
 *   main returns, VM halts, host sees result
 *
 *   host: call("run", <code string>)           ┐
 *   guest dispatch → run_code(fc_bytes, fc_len)│ ~50 ms, subsequent
 *   PyRun_SimpleString(user code)              │
 *   VM halts, host sees result                 ┘
 */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/syscall.h>

/* -- Minimal FlatBuffers reader for the incoming FunctionCall ---------
 * hl_pydriver only cares about the first string parameter (the user's
 * Python source to exec). Hand-rolled so we don't depend on flatcc
 * or the kernel's fb.h. Format: size-prefixed FunctionCall table
 * with a parameters vector of Parameter(value_type=hlstring).
 */

static inline uint32_t fb_u32(const uint8_t *b, size_t o)
{
	return b[o] | ((uint32_t)b[o+1] << 8) |
	       ((uint32_t)b[o+2] << 16) | ((uint32_t)b[o+3] << 24);
}
static inline uint16_t fb_u16(const uint8_t *b, size_t o)
{
	return b[o] | ((uint16_t)b[o+1] << 8);
}
static inline size_t fb_vtable(const uint8_t *b, size_t tbl)
{
	return tbl - (int32_t)fb_u32(b, tbl);
}
static inline uint16_t fb_field(const uint8_t *b, size_t tbl, uint16_t vt)
{
	size_t v = fb_vtable(b, tbl);
	uint16_t vs = fb_u16(b, v);
	return vt >= vs ? 0 : fb_u16(b, v + vt);
}
static inline size_t fb_follow(const uint8_t *b, size_t tbl, uint16_t vt)
{
	uint16_t f = fb_field(b, tbl, vt);
	if (!f) return 0;
	size_t p = tbl + f;
	return p + fb_u32(b, p);
}

/* Extract the first parameter as an hlstring (ParameterValue union
 * discriminant = 7). Returns pointer into `fc` + length, or NULL on
 * error or when arg 0 isn't a string.
 */
static const char *fc_arg0_string(const uint8_t *fc, size_t fc_len,
				  size_t *out_len)
{
	if (fc_len < 8) return NULL;
	size_t root = 4 + fb_u32(fc, 4);
	size_t params = fb_follow(fc, root, 6);
	if (!params) return NULL;
	if (fb_u32(fc, params) == 0) return NULL;
	size_t p0_pos = params + 4;
	size_t p0 = p0_pos + fb_u32(fc, p0_pos);
	uint16_t tf = fb_field(fc, p0, 4);
	if (!tf) return NULL;
	if (fc[p0 + tf] != 7 /* hlstring */) return NULL;
	size_t hs = fb_follow(fc, p0, 6);
	if (!hs) return NULL;
	size_t s = fb_follow(fc, hs, 4);
	if (!s || s + 4 > fc_len) return NULL;
	uint32_t slen = fb_u32(fc, s);
	if (s + 4 + slen > fc_len) return NULL;
	*out_len = slen;
	return (const char *)(fc + s + 4);
}

/* -- Kernel plumbing ------------------------------------------------
 * Slot addresses injected by app-elfloader into our environ at boot.
 * All kernel-side state, read/written by dereferencing.
 *
 *   HL_FC_BYTES_PTR:     kernel global (const uint8_t *) — bytes of
 *                        the current incoming FunctionCall flatbuffer
 *   HL_FC_LEN_PTR:       kernel global (size_t) — length of above
 *   HL_V2_CALLBACK_PTR:  kernel global (hl_dispatch_fn_t) — the
 *                        FC-aware callback the kernel dispatches to
 *                        on every call after it's set
 */
typedef void (*hl_dispatch_fn_t)(const uint8_t *fc, size_t fc_len);

static const uint8_t   **g_fc_bytes_slot;
static size_t           *g_fc_len_slot;
static hl_dispatch_fn_t *g_v2_callback_slot;

/* Saved FS_BASE value captured right after Py_Initialize / warm-up
 * finishes. Restored at the head of every v2-callback invocation so
 * Python's TLS pointer stays valid even if something in the dispatch
 * preamble (dispatch_prepare's MSR restore, Hyperlight's own
 * save/restore of segment state) leaves FS_BASE pointing elsewhere.
 */
static uint64_t g_py_fsbase;

static inline uint64_t rdmsr_fsbase(void)
{
	uint32_t lo, hi;
	__asm__ volatile("rdmsr" : "=a"(lo), "=d"(hi) : "c"(0xC0000100));
	return ((uint64_t)hi << 32) | lo;
}
static inline void wrmsr_fsbase(uint64_t v)
{
	uint32_t lo = (uint32_t)v, hi = (uint32_t)(v >> 32);
	__asm__ volatile("wrmsr" : : "c"(0xC0000100), "a"(lo), "d"(hi));
}

/* -- Core work ------------------------------------------------------- */

static void report_exit_code(int code)
{
	char req[128];
	int n = snprintf(req, sizeof(req),
		"{\"name\":\"__hl_exit\",\"args\":{\"code\":%d}}", code);
	int fd = open("/dev/hcall", O_RDWR);
	if (fd < 0)
		return;
	write(fd, req, n);
	char resp[128];
	read(fd, resp, sizeof(resp));
	close(fd);
}

static int run_code_with_exceptions(const char *code)
{
	PyObject *m = PyImport_AddModule("__main__");
	if (!m) return 1;
	PyObject *d = PyModule_GetDict(m);
	if (!d) return 1;

	PyObject *result = PyRun_String(code, Py_file_input, d, d);
	if (result) {
		Py_DECREF(result);
		return 0;
	}

	if (PyErr_ExceptionMatches(PyExc_SystemExit)) {
		PyObject *type, *value, *tb;
		PyErr_Fetch(&type, &value, &tb);
		PyErr_NormalizeException(&type, &value, &tb);
		int exit_code = 1;
		if (value) {
			PyObject *ca = PyObject_GetAttrString(value, "code");
			if (ca) {
				if (PyLong_Check(ca)) {
					exit_code = (int)PyLong_AsLong(ca);
				} else if (ca == Py_None) {
					exit_code = 0;
				} else {
					PyObject *s = PyObject_Str(ca);
					if (s) {
						const char *msg = PyUnicode_AsUTF8(s);
						if (msg)
							fprintf(stderr, "%s\n", msg);
						Py_DECREF(s);
					}
				}
				Py_DECREF(ca);
			}
		}
		Py_XDECREF(type);
		Py_XDECREF(value);
		Py_XDECREF(tb);
		return exit_code;
	}

	PyErr_Print();
	return 1;
}

static void py_run_user_code(const uint8_t *fc, size_t fc_len)
{
	if (g_py_fsbase)
		wrmsr_fsbase(g_py_fsbase);

	size_t code_len = 0;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return;

	char stack_buf[4096];
	char *buf;
	if (code_len < sizeof(stack_buf)) {
		memcpy(stack_buf, code, code_len);
		stack_buf[code_len] = '\0';
		buf = stack_buf;
	} else {
		buf = malloc(code_len + 1);
		if (!buf)
			return;
		memcpy(buf, code, code_len);
		buf[code_len] = '\0';
	}

	int exit_code = run_code_with_exceptions(buf);

	if (buf != stack_buf)
		free(buf);

	if (exit_code != 0)
		report_exit_code(exit_code);
}

static void py_initialize_once(void)
{
	Py_Initialize();

	/* sys.argv so scripts that look at it don't crash. Also force
	 * stdout/stderr to UTF-8 — the guest has no locale configured,
	 * so Python defaults to ASCII and any non-ASCII char (em-dash,
	 * smart quotes, …) in a script's print() raises UnicodeEncodeError. */
	PyRun_SimpleString(
		"import sys\n"
		"sys.argv = ['hl_pydriver']\n"
		"sys.stdout.reconfigure(encoding='utf-8')\n"
		"sys.stderr.reconfigure(encoding='utf-8')\n");

	/* Pre-import the python-agent stack so every subsequent call
	 * through the v2 callback sees a warm sys.modules and pays only
	 * the user's own code cost. Best-effort: an import that fails
	 * just warns — the user's PyRun_SimpleString will raise its own
	 * traceback if they actually need that module. */
	PyRun_SimpleString(
		"import sys, importlib\n"
		"for _mod in ("
		"    'numpy', 'pandas', 'pydantic', 'yaml', 'jinja2',"
		"    'bs4', 'tabulate', 'click', 'tenacity', 'tqdm',"
		"    'openpyxl', 'pypdf', 'markdown_it', 'PIL', 'lxml',"
		"    'cryptography', 'dateutil', 'dotenv'):\n"
		"  try:\n"
		"    importlib.import_module(_mod)\n"
		"  except Exception as _e:\n"
		"    sys.stderr.write(f'warn: preload {_mod} failed: {_e}\\n')\n");

	/* Monkey-patch time.sleep to call the host via /dev/hcall.
	 * Unikraft's cooperative scheduler on Hyperlight has no timer
	 * interrupt, so the kernel-level nanosleep is a no-op. This
	 * routes the sleep to the host thread which actually blocks. */
	PyRun_SimpleString(
		"import time as _hl_time\n"
		"def _hl_sleep(secs):\n"
		"    if secs <= 0:\n"
		"        return\n"
		"    import json\n"
		"    fd = open('/dev/hcall', 'r+b', buffering=0)\n"
		"    fd.write(json.dumps("
		"{'name':'__hl_sleep','args':{'ns':int(secs*1e9)}}).encode())\n"
		"    fd.read()\n"
		"    fd.close()\n"
		"_hl_time.sleep = _hl_sleep\n"
		"del _hl_time, _hl_sleep\n");
}

/* -- Entry points --------------------------------------------------- */

int main(int argc, char **argv, char **envp)
{
	static int py_initialized;

	(void)argc; (void)argv;

	/* Resolve the slot addresses once. Injected by app-elfloader as
	 * env vars — we just parse the hex addresses. */
	if (!g_fc_bytes_slot) {
		for (char **p = envp; p && *p; p++) {
			if (!strncmp(*p, "HL_FC_BYTES_PTR=", 16))
				g_fc_bytes_slot = (const uint8_t **)(uintptr_t)
					strtoul(*p + 16, NULL, 16);
			else if (!strncmp(*p, "HL_FC_LEN_PTR=", 14))
				g_fc_len_slot = (size_t *)(uintptr_t)
					strtoul(*p + 14, NULL, 16);
			else if (!strncmp(*p, "HL_V2_CALLBACK_PTR=", 19))
				g_v2_callback_slot = (hl_dispatch_fn_t *)(uintptr_t)
					strtoul(*p + 19, NULL, 16);
		}
		if (!g_fc_bytes_slot || !g_fc_len_slot
		    || !g_v2_callback_slot) {
			fprintf(stderr,
				"hl_pydriver: HL_* env vars missing\n");
			return 1;
		}
	}

	if (!py_initialized) {
		py_initialize_once();
		/* Capture FS_BASE now — this is the TLS pointer Python's
		 * internals have wired themselves up against. Future v2
		 * callback entries will restore it before touching any
		 * Python state. */
		g_py_fsbase = rdmsr_fsbase();
		py_initialized = 1;
		/* Install ourselves as the FC-aware dispatch callback. */
		*g_v2_callback_slot = py_run_user_code;
	}

	const uint8_t *fc = *g_fc_bytes_slot;
	size_t fc_len = *g_fc_len_slot;
	if (fc && fc_len)
		py_run_user_code(fc, fc_len);

	fflush(stdout);
	fflush(stderr);

	/* Hand-rolled exit_group via inline syscall: skips glibc's
	 * exit() atexit chain AND any TLS state glibc's syscall()
	 * wrapper might touch (seen the latter corrupt Python's TLS
	 * between first-call halt and second-call re-entry in
	 * testing). The kernel's exit_group handler on
	 * Unikraft-Hyperlight halts the VM cleanly — same end effect
	 * as a normal return, just without the destructive cleanup.
	 */
	register long rax __asm__("rax") = 231; /* SYS_exit_group */
	register long rdi __asm__("rdi") = 0;
	__asm__ volatile("syscall" : : "r"(rax), "r"(rdi)
			 : "rcx", "r11", "memory");
	/* not reached */
	return 0;
}
