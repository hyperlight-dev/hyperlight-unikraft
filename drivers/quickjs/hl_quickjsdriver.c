/*
 * hl_quickjsdriver — QuickJS runtime driver for Hyperlight.
 *
 * Embeds QuickJS (quickjs-ng) and its std/os library: one runtime and one
 * context, brought up once and kept for the guest's life, so what a call
 * defines (a global, an imported module) is there for the next, and a
 * warm snapshot has the engine ready.
 *
 *   Exec       the call's source, run as an ES module when a line starts
 *              with import or export, and as a script otherwise, so a
 *              script's globals are there for the next call.
 *   GuestExec  "path [args]": a .js/.mjs file in the guest filesystem or a
 *              mount, run as a module, with scriptArgs set to the argv;
 *              empty runs /entrypoint.js.
 *   Call       (function, input): call the global function `function`
 *              with `input` parsed as JSON, await what it returns, and
 *              send that back as JSON, the call's result.  A handler is
 *              defined once, by an Exec or in a warm snapshot, and called
 *              many times.
 *
 * The embedder's host functions (SandboxBuilder::host_function) reach
 * JavaScript two ways: host.call(name, ...args), and `a.b` as the export
 * `b` of the module "host:a".  Arguments go to the host as a JSON array,
 * the result comes back parsed, and a host error is thrown as an Error.
 *
 * A call returns when its code and every job and timer it started have
 * run, as Node's does when its event loop drains.  A promise it awaits
 * that nothing left can settle (no job, timer or I/O handler) fails the
 * call, as Node exits on an unsettled top-level await, rather than wait
 * forever.  An uncaught error or an unhandled rejection ends the call: the
 * jobs already queued still run, then the timers and I/O handlers it
 * started are cancelled.  os.Worker is not supported: quickjs-libc builds
 * it, but a worker's thread and messages are outside this model of one
 * call at a time.  An uncaught exception
 * or an unhandled promise rejection fails the call with status 1; the
 * engine is left as it was and serves the next call.  std.exit() ends the
 * guest: it is libc's exit(), and nothing can unwind QuickJS from inside
 * a call safely.
 *
 * Globals: print and console (log/info to stdout, warn/error to stderr),
 * scriptArgs; modules qjs:std, qjs:os (files, timers) and qjs:bjson.
 * Math.random draws from a generator seeded from getrandom() at the top of
 * every call, so every clone of a warm snapshot draws its own sequence:
 * QuickJS's own is seeded once, at context creation, and would be
 * captured in the snapshot with the rest of the heap.
 */

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/random.h>

#include "quickjs.h"
#include "quickjs-libc.h"

#include "hl_fc.h"
#include "hl_env.h"
#include "hl_driver.h"

/* From hl_qjs_await.c, which builds quickjs-libc. */
int hl_os_poll(JSContext *ctx);
int hl_os_idle(JSContext *ctx);
void hl_os_cancel(JSContext *ctx);

#define NAME "hl_quickjsdriver"
#define ENTRYPOINT "/entrypoint.js"
#define MAX_ARGS 64

static JSRuntime *g_rt;
static JSContext *g_ctx;

/* ── Math.random ───────────────────────────────────────────────── */

static uint64_t g_rng;

static void reseed(void)
{
	if (getrandom(&g_rng, sizeof(g_rng), 0) != sizeof(g_rng) || !g_rng)
		g_rng ^= 0x9e3779b97f4a7c15ULL;
}

/* xorshift64*, turned into a double in [0, 1) the way QuickJS's own
 * Math.random does it: 52 random mantissa bits under a fixed exponent. */
static JSValue hl_math_random(JSContext *ctx, JSValueConst this_val,
			      int argc, JSValueConst *argv)
{
	union { double d; uint64_t u; } v;

	(void)this_val;
	(void)argc;
	(void)argv;
	g_rng ^= g_rng >> 12;
	g_rng ^= g_rng << 25;
	g_rng ^= g_rng >> 27;
	v.u = ((g_rng * 0x2545F4914F6CDD1DULL) >> 12) | 0x3ff0000000000000ULL;
	return JS_NewFloat64(ctx, v.d - 1.0);
}

/* ── console.warn / console.error ──────────────────────────────── */

static JSValue hl_print_stderr(JSContext *ctx, JSValueConst this_val,
			       int argc, JSValueConst *argv)
{
	(void)this_val;
	for (int i = 0; i < argc; i++) {
		size_t len;
		const char *s = JS_ToCStringLen(ctx, &len, argv[i]);

		if (!s)
			return JS_EXCEPTION;
		if (i)
			fputc(' ', stderr);
		fwrite(s, 1, len, stderr);
		JS_FreeCString(ctx, s);
	}
	fputc('\n', stderr);
	fflush(stderr);
	return JS_UNDEFINED;
}

/* ── Unhandled rejections ──────────────────────────────────────── */

static void dump_value(JSContext *ctx, JSValueConst v);

/* A rejection with no handler when the call's jobs have all run fails the
 * call.  quickjs-libc's tracker would exit() the process instead.  Every
 * one is kept, with its promise: a handler that comes later removes that
 * promise's, and only that one. */
struct rejection {
	JSValue promise;
	JSValue reason;
};
static struct rejection *g_rejections;
static size_t g_nrejections, g_rejections_cap;

static void drop_rejection(JSContext *ctx, size_t i)
{
	JS_FreeValue(ctx, g_rejections[i].promise);
	JS_FreeValue(ctx, g_rejections[i].reason);
	g_rejections[i] = g_rejections[--g_nrejections];
}

static void track_rejection(JSContext *ctx, JSValueConst promise,
			    JSValueConst reason, bool is_handled, void *opaque)
{
	(void)opaque;
	if (is_handled) {
		for (size_t i = 0; i < g_nrejections; i++) {
			if (JS_IsSameValue(ctx, promise, g_rejections[i].promise)) {
				drop_rejection(ctx, i);
				return;
			}
		}
		return;
	}
	if (g_nrejections == g_rejections_cap) {
		size_t cap = g_rejections_cap ? 2 * g_rejections_cap : 8;
		struct rejection *r = realloc(g_rejections, cap * sizeof(*r));

		if (!r)
			return;
		g_rejections = r;
		g_rejections_cap = cap;
	}
	g_rejections[g_nrejections].promise = JS_DupValue(ctx, promise);
	g_rejections[g_nrejections].reason = JS_DupValue(ctx, reason);
	g_nrejections++;
}

/* Report the rejections left unhandled and forget them all; how many. */
static size_t report_rejections(JSContext *ctx)
{
	size_t n = g_nrejections;

	while (g_nrejections) {
		fprintf(stderr, "Unhandled promise rejection: ");
		dump_value(ctx, g_rejections[0].reason);
		drop_rejection(ctx, 0);
	}
	return n;
}

/* Print an exception or rejection reason, with its stack when it has one. */
static void dump_value(JSContext *ctx, JSValueConst v)
{
	const char *s = JS_ToCString(ctx, v);

	fprintf(stderr, "%s\n", s ? s : "[exception]");
	JS_FreeCString(ctx, s);
	if (JS_IsError(v)) {
		JSValue stack = JS_GetPropertyStr(ctx, v, "stack");

		if (!JS_IsUndefined(stack)) {
			const char *st = JS_ToCString(ctx, stack);

			if (st)
				fputs(st, stderr);
			JS_FreeCString(ctx, st);
		}
		JS_FreeValue(ctx, stack);
	}
}

/* Print the pending exception.  A module whose evaluation throws also
 * rejects its promise before anything handles it, so the same error has
 * reached the rejection tracker: it is reported once, here. */
static void dump_exception(JSContext *ctx)
{
	JSValue e = JS_GetException(ctx);

	for (size_t i = 0; i < g_nrejections; i++) {
		if (JS_IsStrictEqual(ctx, e, g_rejections[i].reason)) {
			drop_rejection(ctx, i);
			break;
		}
	}
	dump_value(ctx, e);
	JS_FreeValue(ctx, e);
}

/* Whether source is an ES module: a line that starts with an import or
 * export statement.  JS_DetectModule would take any code that parses as a
 * module, which is nearly all of it, and a module's declarations stay in
 * the module: a script's `var` and `function` are globals the next call
 * can use, as a handler defined once and called many times needs. */
static int looks_like_module(const char *src, size_t len)
{
	static const char *const keywords[] = { "import", "export" };
	const char *p = src, *end = src + len;

	/* A UTF-8 byte order mark before the first line is not code. */
	if (len >= 3 && !memcmp(p, "\xef\xbb\xbf", 3))
		p += 3;
	while (p < end) {
		while (p < end && (*p == ' ' || *p == '\t'))
			p++;
		for (size_t k = 0; k < sizeof(keywords) / sizeof(keywords[0]); k++) {
			size_t n = strlen(keywords[k]);

			const char *q;

			if ((size_t)(end - p) <= n || memcmp(p, keywords[k], n) ||
			    !p[n] || !strchr(" \t{*\"'", p[n]))
				continue;
			/* `import(`, `import (`: a dynamic import, fine in a
			 * script. */
			for (q = p + n; q < end && (*q == ' ' || *q == '\t'); q++)
				;
			if (k == 0 && q < end && *q == '(')
				continue;
			return 1;
		}
		while (p < end && *p != '\n')
			p++;
		p++;
	}
	return 0;
}

/* ── Host functions ────────────────────────────────────────────── */

/* Call the host function `name` with argv as a JSON array. */
static JSValue call_host(JSContext *ctx, const char *name, size_t name_len,
			 int argc, JSValueConst *argv)
{
	JSValue arr = JS_NewArray(ctx), json;
	const char *args, *out;
	size_t args_len, out_len;
	int rc;

	for (int i = 0; i < argc; i++)
		JS_SetPropertyUint32(ctx, arr, i, JS_DupValue(ctx, argv[i]));
	json = JS_JSONStringify(ctx, arr, JS_UNDEFINED, JS_UNDEFINED);
	JS_FreeValue(ctx, arr);
	if (JS_IsException(json))
		return json;
	args = JS_ToCStringLen(ctx, &args_len, json);
	JS_FreeValue(ctx, json);
	if (!args)
		return JS_EXCEPTION;
	rc = hl_host_call(name, name_len, args, args_len, &out, &out_len);
	int saved_errno = errno; /* before the free below can change it */

	JS_FreeCString(ctx, args);
	if (rc < 0)
		return JS_ThrowInternalError(ctx, "host function %.*s: %s",
					     (int)name_len, name, strerror(saved_errno));
	if (rc > 0) {
		/* The whole message: JS_ThrowPlainError formats into a
		 * 256-byte buffer and would cut a longer one short. */
		JSValue err = JS_NewError(ctx);

		JS_DefinePropertyValueStr(ctx, err, "message",
					  JS_NewStringLen(ctx, out, out_len),
					  JS_PROP_WRITABLE | JS_PROP_CONFIGURABLE);
		return JS_Throw(ctx, err);
	}
	if (!out_len)
		return JS_UNDEFINED;
	return JS_ParseJSON(ctx, out, out_len, "<host>");
}

/* host.call(name, ...args) */
static JSValue js_host_call(JSContext *ctx, JSValueConst this_val,
			    int argc, JSValueConst *argv)
{
	size_t len;
	const char *name;
	JSValue ret;

	if (argc < 1)
		return JS_ThrowTypeError(ctx, "host.call(name, ...args)");
	name = JS_ToCStringLen(ctx, &len, argv[0]);
	if (!name)
		return JS_EXCEPTION;
	/* The empty name asks the host for its list of functions. */
	if (!len) {
		JS_FreeCString(ctx, name);
		return JS_ThrowTypeError(ctx, "host.call: a function name is required");
	}
	ret = call_host(ctx, name, len, argc - 1, argv + 1);
	JS_FreeCString(ctx, name);
	return ret;
}

/* An export of a host: module: data[0] is the function's full name. */
static JSValue js_host_export(JSContext *ctx, JSValueConst this_val, int argc,
			      JSValueConst *argv, int magic, JSValueConst *data)
{
	size_t len;
	const char *name = JS_ToCStringLen(ctx, &len, data[0]);
	JSValue ret;

	if (!name)
		return JS_EXCEPTION;
	ret = call_host(ctx, name, len, argc, argv);
	JS_FreeCString(ctx, name);
	return ret;
}

/* The host's function names, one per line, copied out of the reply
 * buffer (the next host call reuses it); NULL with an exception set. */
static char *host_function_names(JSContext *ctx)
{
	const char *out;
	size_t len;
	char *names;

	switch (hl_host_call("", 0, NULL, 0, &out, &len)) {
	case 0:
		break;
	case 1:
		JS_ThrowInternalError(ctx, "cannot list the host's functions: %.*s",
				      (int)len, out);
		return NULL;
	default:
		JS_ThrowInternalError(ctx, "cannot list the host's functions: %s",
				      strerror(errno));
		return NULL;
	}
	names = js_malloc(ctx, len + 1);
	if (!names)
		return NULL;
	memcpy(names, out, len);
	names[len] = '\0';
	return names;
}

/* For each host function `module.fn`, call `each(fn, full name)`. */
static int for_each_host_export(JSContext *ctx, const char *module,
				int (*each)(JSContext *, JSModuleDef *,
					    const char *, const char *),
				JSModuleDef *m)
{
	char *names = host_function_names(ctx), *line, *save;
	size_t mlen = strlen(module);
	int count = 0;

	if (!names)
		return -1;
	for (line = strtok_r(names, "\n", &save); line;
	     line = strtok_r(NULL, "\n", &save)) {
		if (strncmp(line, module, mlen) || line[mlen] != '.' || !line[mlen + 1])
			continue;
		if (each(ctx, m, line + mlen + 1, line) < 0) {
			js_free(ctx, names);
			return -1;
		}
		count++;
	}
	js_free(ctx, names);
	return count;
}

static int count_export(JSContext *ctx, JSModuleDef *m, const char *fn,
			const char *full)
{
	return 0;
}

static int add_export(JSContext *ctx, JSModuleDef *m, const char *fn,
		      const char *full)
{
	return JS_AddModuleExport(ctx, m, fn);
}

static int set_export(JSContext *ctx, JSModuleDef *m, const char *fn,
		      const char *full)
{
	JSValue name = JS_NewString(ctx, full);
	JSValue f = JS_NewCFunctionData(ctx, js_host_export, 0, 0, 1, &name);

	JS_FreeValue(ctx, name);
	return JS_SetModuleExport(ctx, m, fn, f);
}

/* The module's name without "host:". */
static const char *host_module_of(JSContext *ctx, JSModuleDef *m)
{
	/* JS_GetModuleName hands back a reference to the atom. */
	JSAtom atom = JS_GetModuleName(ctx, m);
	const char *name = JS_AtomToCString(ctx, atom);

	JS_FreeAtom(ctx, atom);
	return name;
}

static int init_host_module(JSContext *ctx, JSModuleDef *m)
{
	const char *name = host_module_of(ctx, m);
	int rc;

	if (!name)
		return -1;
	rc = for_each_host_export(ctx, name + strlen("host:"), set_export, m);
	JS_FreeCString(ctx, name);
	return rc < 0 ? -1 : 0;
}

/* "host:a" is a module of the host's functions `a.*`; everything else is
 * a file, as qjs loads it. */
static JSModuleDef *load_module(JSContext *ctx, const char *module_name,
				void *opaque, JSValueConst attributes)
{
	JSModuleDef *m;
	int count;

	if (strncmp(module_name, "host:", 5))
		return js_module_loader(ctx, module_name, opaque, attributes);
	/* Counted before the module is created: QuickJS keeps a module from
	 * the moment it is, so a failed import would leave one behind that
	 * later imports find, empty. */
	count = for_each_host_export(ctx, module_name + 5, count_export, NULL);
	if (count < 0)
		return NULL;
	if (count == 0) {
		JS_ThrowReferenceError(ctx, "no host functions %s.*: the embedder registers "
				       "them with SandboxBuilder::host_function",
				       module_name + 5);
		return NULL;
	}
	m = JS_NewCModule(ctx, module_name, init_host_module);
	if (!m)
		return NULL;
	if (for_each_host_export(ctx, module_name + 5, add_export, m) < 0)
		return NULL;
	return m;
}

/* ── Running code ──────────────────────────────────────────────── */

/* scriptArgs for the call: argv, or just the file name for Exec. */
static int set_script_args(char **argv, int argc)
{
	JSValue global = JS_GetGlobalObject(g_ctx);
	JSValue args = JS_NewArray(g_ctx);

	for (int i = 0; i < argc; i++)
		JS_SetPropertyUint32(g_ctx, args, i, JS_NewString(g_ctx, argv[i]));
	JS_SetPropertyStr(g_ctx, global, "scriptArgs", args);
	JS_FreeValue(g_ctx, global);
	return 0;
}

/*
 * Wait for a promise, running jobs, timers and I/O until it settles: its
 * value, or JS_EXCEPTION with the rejection thrown.  Not js_std_await,
 * which spins when nothing is left to wait for, a vCPU at 100% inside one
 * VM entry that no step timeout or deadlock check can see, and prints and
 * clears an exception a job throws, so the error does not fail the call.
 * Here a job's or a timer's exception is the call's, and a promise that
 * nothing left can settle fails it, as Node exits on an unsettled
 * top-level await: "nothing left" is js_os_poll's own test, no job, timer,
 * I/O handler or worker port (a signal handler alone does not count).  Not a promise: the
 * value as it is.
 */
static JSValue await_value(JSValue obj)
{
	JSRuntime *rt = JS_GetRuntime(g_ctx);

	for (;;) {
		JSContext *ctx1;
		JSValue ret;
		int err;

		switch (JS_PromiseState(g_ctx, obj)) {
		case JS_PROMISE_FULFILLED:
			ret = JS_PromiseResult(g_ctx, obj);
			JS_FreeValue(g_ctx, obj);
			return ret;
		case JS_PROMISE_REJECTED:
			ret = JS_Throw(g_ctx, JS_PromiseResult(g_ctx, obj));
			JS_FreeValue(g_ctx, obj);
			return ret;
		case JS_PROMISE_PENDING:
			break;
		default:
			return obj;
		}

		err = JS_ExecutePendingJob(rt, &ctx1);
		if (err < 0) {
			JS_FreeValue(g_ctx, obj);
			return JS_EXCEPTION;
		}
		if (err > 0)
			continue;
		/* The jobs have run: a promise left rejected with no handler
		 * ends the call (finish_call reports it), whatever this one
		 * is still waiting for -- once this one's own state is seen,
		 * at the top, on the next turn. */
		if (g_nrejections && JS_PromiseState(g_ctx, obj) == JS_PROMISE_PENDING) {
			JS_FreeValue(g_ctx, obj);
			return JS_ThrowInternalError(g_ctx, "a promise was rejected with no handler");
		}
		if (hl_os_idle(g_ctx)) {
			JS_FreeValue(g_ctx, obj);
			return JS_ThrowInternalError(g_ctx, "the awaited promise can never settle: "
						     "no job, timer or I/O handler is left to settle it");
		}
		errno = 0;
		if (hl_os_poll(g_ctx) == 0)
			continue;
		/* -1: a timer or handler threw, or the poll itself failed. */
		if (JS_HasException(g_ctx)) {
			JS_FreeValue(g_ctx, obj);
			return JS_EXCEPTION;
		}
		if (errno == EINTR)
			continue;
		JS_FreeValue(g_ctx, obj);
		return JS_ThrowInternalError(g_ctx, "waiting for timers or I/O failed: %s",
					     strerror(errno));
	}
}

/* Promise.resolve(v): v if it is a promise, what it resolves to if it is
 * a thenable, a promise of v otherwise -- what `await v` waits for.
 * Takes v. */
static JSValue promise_resolve(JSValue v)
{
	JSValue global = JS_GetGlobalObject(g_ctx);
	JSValue ctor = JS_GetPropertyStr(g_ctx, global, "Promise");
	JSValue resolve = JS_GetPropertyStr(g_ctx, ctor, "resolve");
	JSValue ret = JS_Call(g_ctx, resolve, ctor, 1, (JSValueConst *)&v);

	JS_FreeValue(g_ctx, resolve);
	JS_FreeValue(g_ctx, ctor);
	JS_FreeValue(g_ctx, global);
	JS_FreeValue(g_ctx, v);
	return ret;
}

/* Run jobs, timers and I/O until nothing is left: 0, 1 when one threw
 * (the exception pending), -1 when waiting for them failed or a promise
 * was left rejected with no handler.  Rejections are checked each time
 * the jobs have run, as Node checks once its microtasks have: a timer
 * that would keep the call alive does not keep a rejection from ending
 * it.  Not
 * js_std_loop, which takes a failed poll -- an interrupted one included --
 * for the end, leaving timers to run in the next call. */
static int run_loop(void)
{
	JSRuntime *rt = JS_GetRuntime(g_ctx);

	for (;;) {
		JSContext *ctx1;
		int err = JS_ExecutePendingJob(rt, &ctx1);

		if (err < 0)
			return 1;
		if (err > 0)
			continue;
		if (g_nrejections)
			return -1;
		if (hl_os_idle(g_ctx))
			return 0;
		errno = 0;
		if (hl_os_poll(g_ctx) == 0)
			continue;
		if (JS_HasException(g_ctx))
			return 1;
		if (errno == EINTR)
			continue;
		fprintf(stderr, "%s: waiting for timers or I/O failed: %s\n", NAME,
			strerror(errno));
		return -1;
	}
}

/* The end of every call, however it went: run the jobs and timers it
 * left, and fail it on an error they raise or a rejection nobody
 * handled.  `status`, or 1 then. */
static int finish_call(int status)
{
	JSContext *ctx1;

	if (status == 0) {
		int r = run_loop();

		if (r > 0)
			dump_exception(g_ctx);
		if (r != 0)
			status = 1;
	}
	/* An uncaught error or an unhandled rejection ends the call and
	 * what it started: the jobs already queued still run (QuickJS has
	 * no way to drop them), their errors reported, then every timer,
	 * I/O handler and worker port is cancelled -- the ones those jobs
	 * registered too -- so none runs in the next call, and one that
	 * keeps throwing cannot hold this one. */
	if (status != 0) {
		for (int err; (err = JS_ExecutePendingJob(JS_GetRuntime(g_ctx), &ctx1)) != 0;)
			if (err < 0)
				dump_exception(g_ctx);
		hl_os_cancel(g_ctx);
	}
	if (report_rejections(g_ctx))
		status = status ? status : 1;
	fflush(stdout);
	fflush(stderr);
	return status;
}

/* Evaluate `len` bytes of source, then run every job and timer it left.
 * 0, or 1 for an uncaught exception or an unhandled rejection. */
static int run_source(const char *src, size_t len, const char *filename,
		      int as_module)
{
	JSValue val;
	int status = 0;

	if (as_module) {
		/* Compiled first so import.meta can be set, as qjs does. */
		val = JS_Eval(g_ctx, src, len, filename,
			      JS_EVAL_TYPE_MODULE | JS_EVAL_FLAG_COMPILE_ONLY);
		if (!JS_IsException(val)) {
			if (js_module_set_import_meta(g_ctx, val, filename[0] == '/',
						      true) < 0) {
				JS_FreeValue(g_ctx, val);
				val = JS_EXCEPTION;
			} else {
				val = JS_EvalFunction(g_ctx, val);
			}
		}
		if (!JS_IsException(val))
			val = await_value(val);
	} else {
		val = JS_Eval(g_ctx, src, len, filename, JS_EVAL_TYPE_GLOBAL);
	}
	if (JS_IsException(val)) {
		dump_exception(g_ctx);
		status = 1;
	}
	JS_FreeValue(g_ctx, val);

	/* The call is done when its jobs and timers are. */
	return finish_call(status);
}

static int has_suffix(const char *s, const char *suffix)
{
	size_t n = strlen(s), m = strlen(suffix);

	return n >= m && !strcmp(s + n - m, suffix);
}

static int run_file(char **argv, int argc)
{
	size_t len;
	uint8_t *buf;
	int as_module, status;

	buf = js_load_file(g_ctx, &len, argv[0]);
	if (!buf) {
		fprintf(stderr, "%s: cannot read %s\n", NAME, argv[0]);
		return 1;
	}
	set_script_args(argv, argc);
	/* .mjs is a module; .js one when it has import/export lines. */
	as_module = has_suffix(argv[0], ".mjs") ||
		    looks_like_module((const char *)buf, len);
	status = run_source((const char *)buf, len, argv[0], as_module);
	js_free(g_ctx, buf);
	return status;
}

/* Call the global function `name` with `input` parsed as JSON; its
 * result, awaited, becomes the call's, as JSON.  0, or 1 as run_source. */
static int call_function(const char *name, size_t name_len, const char *input,
			 size_t input_len)
{
	JSValue global, fn, arg, val = JS_UNDEFINED, json;
	JSAtom atom;
	char *in;
	int status = 0;

	/* By length: a name with a NUL in it names no other function. */
	atom = JS_NewAtomLen(g_ctx, name, name_len);
	if (atom == JS_ATOM_NULL) {
		dump_exception(g_ctx);
		return finish_call(1);
	}
	global = JS_GetGlobalObject(g_ctx);
	fn = JS_GetProperty(g_ctx, global, atom);
	JS_FreeValue(g_ctx, global);
	JS_FreeAtom(g_ctx, atom);
	if (JS_IsException(fn)) {
		/* Looking it up threw (a getter): that is the call's error. */
		dump_exception(g_ctx);
		return finish_call(1);
	}
	if (!JS_IsFunction(g_ctx, fn)) {
		fprintf(stderr, "%s: no global function %.*s: define it (function %.*s(input) "
			"{...}) before calling it\n", NAME, (int)name_len, name,
			(int)name_len, name);
		JS_FreeValue(g_ctx, fn);
		return finish_call(1);
	}

	/* JS_ParseJSON wants a NUL after the input, which hl_fc.h does not
	 * promise. */
	in = js_malloc(g_ctx, input_len + 1);
	if (!in) {
		JS_FreeValue(g_ctx, fn);
		return finish_call(-1);
	}
	memcpy(in, input, input_len);
	in[input_len] = '\0';
	arg = input_len ? JS_ParseJSON(g_ctx, in, input_len, "<input>") : JS_UNDEFINED;
	js_free(g_ctx, in);
	if (!JS_IsException(arg)) {
		/* Empty input is no argument, not one undefined. */
		val = JS_Call(g_ctx, fn, JS_UNDEFINED, input_len ? 1 : 0, (JSValueConst *)&arg);
		JS_FreeValue(g_ctx, arg);
		if (!JS_IsException(val))
			val = await_value(promise_resolve(val));
	} else {
		val = JS_EXCEPTION;
	}
	JS_FreeValue(g_ctx, fn);

	if (JS_IsException(val)) {
		dump_exception(g_ctx);
		status = 1;
	} else if (!JS_IsUndefined(val)) {
		json = JS_JSONStringify(g_ctx, val, JS_UNDEFINED, JS_UNDEFINED);
		if (JS_IsException(json)) {
			dump_exception(g_ctx);
			status = 1;
		} else if (!JS_IsUndefined(json)) {
			size_t len;
			const char *text = JS_ToCStringLen(g_ctx, &len, json);

			if (!text) {
				dump_exception(g_ctx);
				status = 1;
			} else if (hl_set_result(text, len) < 0) {
				fprintf(stderr, "%s: %.*s's result does not fit a call\n",
					NAME, (int)name_len, name);
				status = 1;
			}
			JS_FreeCString(g_ctx, text);
		}
		JS_FreeValue(g_ctx, json);
	}
	JS_FreeValue(g_ctx, val);
	return finish_call(status);
}

static int quickjs_dispatch(const uint8_t *fc, size_t fc_len)
{
	size_t len;
	const char *arg = fc_arg0_string(fc, fc_len, &len);

	hl_env_refresh(NULL, NULL);
	reseed();

	if (fc_name_is(fc, fc_len, "Call")) {
		size_t in_len;
		const char *in = fc_arg_string(fc, fc_len, 1, &in_len);

		if (!arg || !len || !in)
			return -1;
		return call_function(arg, len, in, in_len);
	}

	if (fc_name_is(fc, fc_len, "GuestExec")) {
		char line[4096];
		char *argv[MAX_ARGS];
		int argc;
		const char *src = (arg && len) ? arg : ENTRYPOINT;
		size_t n = (arg && len) ? len : strlen(ENTRYPOINT);

		if (n >= sizeof(line))
			return -1;
		memcpy(line, src, n);
		line[n] = '\0';
		argc = hl_split_ws(line, argv, MAX_ARGS);
		if (!argc)
			return 0;
		return run_file(argv, argc);
	}

	if (!arg)
		return -1;
	char *exec_argv[] = { "<exec>", NULL };

	set_script_args(exec_argv, 1);
	/* JS_Eval wants a NUL at src[len], which hl_fc.h does not promise,
	 * so the source is copied. */
	char *src = malloc(len + 1);
	int status;

	if (!src)
		return -1;
	memcpy(src, arg, len);
	src[len] = '\0';
	status = run_source(src, len, "<exec>", looks_like_module(src, len));
	free(src);
	return status;
}

/* ── Entry point ───────────────────────────────────────────────── */

static int start_engine(void)
{
	JSValue global, console, math, host;

	g_rt = JS_NewRuntime();
	if (!g_rt)
		return -1;
	js_std_init_handlers(g_rt);
	g_ctx = JS_NewContext(g_rt);
	if (!g_ctx)
		return -1;
	js_init_module_std(g_ctx, "qjs:std");
	js_init_module_os(g_ctx, "qjs:os");
	js_init_module_bjson(g_ctx, "qjs:bjson");
	JS_SetModuleLoaderFunc2(g_rt, NULL, load_module,
				js_module_check_attributes, NULL);
	JS_SetHostPromiseRejectionTracker(g_rt, track_rejection, NULL);
	js_std_add_helpers(g_ctx, 0, NULL);

	global = JS_GetGlobalObject(g_ctx);
	console = JS_GetPropertyStr(g_ctx, global, "console");
	JS_SetPropertyStr(g_ctx, console, "info",
			  JS_GetPropertyStr(g_ctx, console, "log"));
	JS_SetPropertyStr(g_ctx, console, "warn",
			  JS_NewCFunction(g_ctx, hl_print_stderr, "warn", 1));
	JS_SetPropertyStr(g_ctx, console, "error",
			  JS_NewCFunction(g_ctx, hl_print_stderr, "error", 1));
	host = JS_NewObject(g_ctx);
	JS_SetPropertyStr(g_ctx, host, "call",
			  JS_NewCFunction(g_ctx, js_host_call, "call", 1));
	JS_SetPropertyStr(g_ctx, global, "host", host);
	math = JS_GetPropertyStr(g_ctx, global, "Math");
	JS_SetPropertyStr(g_ctx, math, "random",
			  JS_NewCFunction(g_ctx, hl_math_random, "random", 0));
	JS_FreeValue(g_ctx, math);
	JS_FreeValue(g_ctx, console);
	JS_FreeValue(g_ctx, global);
	return 0;
}

int main(void)
{
	if (hl_driver_init(NAME))
		return 1;
	if (start_engine()) {
		fprintf(stderr, "%s: cannot start QuickJS\n", NAME);
		return 1;
	}
	reseed();
	hl_driver_serve_calls();
	/* Serve named calls from the kernel's queue; never returns. */
	hl_driver_run(quickjs_dispatch);
}
