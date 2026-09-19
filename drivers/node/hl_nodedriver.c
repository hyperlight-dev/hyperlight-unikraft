/*
 * hl_nodedriver — Node.js runtime driver for Hyperlight.
 *
 * Spawns a persistent Node.js child process during boot via
 * vfork+exec.  The child runs a dispatch loop that reads code
 * from a pipe (asynchronously, so its event loop keeps turning
 * between calls), evals it, and writes the call's status back.
 *
 * Flow:
 *   boot (evolve):
 *     main() → hl_driver_init(): open /dev/hlcall
 *            → node_spawn(): pipes, bootstrap JS in /tmp/hl_bootstrap.js,
 *              vfork + exec("node", "/tmp/hl_bootstrap.js"), then the
 *              child's ready byte (blocks, scheduler switches to child,
 *              V8 starts up, child signals)
 *            → hl_driver_run(): block in read() on the call queue
 *
 *   host: call("Exec", "console.log(42)")
 *     read() returns the call → node_dispatch(fc, fc_len)
 *              → write [len:u64][code] to pipe
 *              → read status byte (blocks, scheduler switches to
 *                child, child evals code, writes the status)
 *              → back into read(): the call is done
 *
 * The child Node process stays alive across dispatches: no V8 startup
 * cost per call, and globals and required modules persist from one
 * call to the next, as the interpreter's state does in the Python
 * driver.  An exit inside a call (process.exit(), an uncaught error)
 * ends the call with that status, not the child: see the bootstrap.
 * Should the child end anyway, the next call starts a new one.
 */

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

#include "../hl_fc.h"
#include "../hl_env.h"
#include "../hl_driver.h"

/* ── State ─────────────────────────────────────────────────────── */

static pid_t g_node_pid = -1;
static int g_pipe_to_node = -1;    /* parent writes code here */
static int g_pipe_from_node = -1;  /* parent reads the status here */

/* ── Node env visitor ─────────────────────────────────────────── */

/*
 * The persistent Node child was spawned at boot and does not see the
 * parent's setenv(), so each call's code is prefixed with a
 * `process.env[<key>]=<value>;` line per host variable, key and value
 * as JSON string literals.
 */
static void node_env_visitor(const char *key, const char *val, void *ctx)
{
	struct hl_strbuf *env = ctx;

	hl_strbuf_puts(env, "process.env[");
	hl_strbuf_put_json(env, key);
	hl_strbuf_puts(env, "]=");
	hl_strbuf_put_json(env, val);
	hl_strbuf_puts(env, ";\n");
}

/* ── Bootstrap JS ─────────────────────────────────────────────── */

static int write_bootstrap(int fd_in, int fd_out)
{
	const char *path = "/tmp/hl_bootstrap.js";
	FILE *f = fopen(path, "w");
	if (!f) {
		fprintf(stderr, "hl_nodedriver: cannot create %s\n", path);
		return -1;
	}

	fprintf(f,
		"'use strict';\n"
		"const fs = require('fs');\n"
		"const fd_in = %d;\n"
		"const fd_out = %d;\n"
		"\n"
		"// Expose CJS module globals so eval'd code can require()\n"
		"globalThis.require = require;\n"
		"globalThis.module = module;\n"
		"globalThis.__dirname = __dirname;\n"
		"globalThis.__filename = __filename;\n"
		"\n"
		"// Signal ready to parent\n"
		"fs.writeSync(fd_out, Buffer.from([0]));\n"
		"\n"
		"// Read exactly n bytes from the driver's pipe.  Asynchronous, so\n"
		"// the event loop keeps turning while the child waits for the next\n"
		"// call: a timer or handle a call left behind (unref'd, or the call\n"
		"// would not have ended) keeps making progress, and the pending read\n"
		"// keeps the process alive where an empty loop would have exited it.\n"
		"const { promisify } = require('util');\n"
		"const readAsync = promisify(fs.read);\n"
		"async function readExactly(n) {\n"
		"  const buf = Buffer.alloc(n);\n"
		"  let off = 0;\n"
		"  while (off < n) {\n"
		"    const { bytesRead } = await readAsync(fd_in, buf, off, n - off, null);\n"
		"    if (bytesRead <= 0) return null;\n"
		"    off += bytesRead;\n"
		"  }\n"
		"  return buf;\n"
		"}\n"
		"\n"
		"// An exit ends the call, not the child.  process.exit() throws\n"
		"// this instead; wherever it is thrown (sync code, an awaited\n"
		"// promise, a timer callback) it reaches exited() below, and the\n"
		"// call's status is its code, as `node script.js` would exit with.\n"
		"class HlExit { constructor(code) { this.code = code; } }\n"
		"process.exit = (code) => {\n"
		"  throw new HlExit(code === undefined ? (process.exitCode ?? 0) : code);\n"
		"};\n"
		"\n"
		"// The timers the call in flight has set.  Node keeps no list of\n"
		"// them, so the timer functions are wrapped to keep one: a timer\n"
		"// joins it when set and leaves it when cleared or, for a one-shot,\n"
		"// when it fires.  A new set per call: only this call's timers can\n"
		"// hold the next call, an earlier call's are unref'd or it would\n"
		"// not have ended.\n"
		"const timersMod = require('timers');\n"
		"let liveTimers = new Set();\n"
		"for (const [setName, clearName] of [['setTimeout', 'clearTimeout'],\n"
		"    ['setInterval', 'clearInterval'], ['setImmediate', 'clearImmediate']]) {\n"
		"  const origSet = timersMod[setName], origClear = timersMod[clearName];\n"
		"  const oneShot = setName !== 'setInterval';\n"
		"  const set = function (fn, ...rest) {\n"
		"    const t = origSet.call(this, function (...args) {\n"
		"      if (oneShot) liveTimers.delete(t);\n"
		"      return fn.apply(this, args);\n"
		"    }, ...rest);\n"
		"    liveTimers.add(t);\n"
		"    return t;\n"
		"  };\n"
		"  set[promisify.custom] = origSet[promisify.custom];  // util.promisify(setTimeout)\n"
		"  const clear = function (t) { liveTimers.delete(t); return origClear.call(this, t); };\n"
		"  timersMod[setName] = globalThis[setName] = set;\n"
		"  timersMod[clearName] = globalThis[clearName] = clear;\n"
		"}\n"
		"\n"
		"// What the process's exit would have taken with it, of what the\n"
		"// call in flight started: its timers, and the servers and sockets\n"
		"// opened since the call began.  Cleared and closed, so work the\n"
		"// call left running at its exit does not hold the calls after it;\n"
		"// what earlier calls left running goes on, as in Python.\n"
		"const activeHandles = () => typeof process._getActiveHandles === 'function'\n"
		"  ? process._getActiveHandles() : [];\n"
		"let handlesBefore = new Set();\n"
		"function stopLeftovers() {\n"
		"  for (const t of liveTimers) {\n"
		"    try { (t.constructor.name === 'Immediate' ? clearImmediate : clearTimeout)(t); } catch (e) {}\n"
		"  }\n"
		"  liveTimers.clear();\n"
		"  for (const h of activeHandles()) {\n"
		"    if (handlesBefore.has(h)) continue;\n"
		"    try {\n"
		"      if (typeof h.close === 'function') h.close();\n"
		"      else if (typeof h.unref === 'function') h.unref();\n"
		"    } catch (e) {}\n"
		"  }\n"
		"}\n"
		"\n"
		"// end(status) settles the call in flight; null between calls.\n"
		"let end = null;\n"
		"\n"
		"// An exit, or an error nothing caught: the call in flight ends\n"
		"// with the code (1 for an error, as node exits with), and what was\n"
		"// left running stops, as it would have with the process.  Between\n"
		"// calls there is no call to end; the error is still reported.\n"
		"function exited(e) {\n"
		"  let status = 1;\n"
		"  if (e instanceof HlExit) status = e.code;\n"
		"  else console.error((e && e.stack) || e);\n"
		"  stopLeftovers();\n"
		"  if (end) end(status);\n"
		"}\n"
		"process.on('uncaughtException', exited);\n"
		"process.on('unhandledRejection', exited);\n"
		"\n"
		"(async () => {\n"
		"  while (true) {\n"
		"    const hdr = await readExactly(8);\n"
		"    if (!hdr) break;\n"
		"    const len = Number(hdr.readBigUInt64LE(0));\n"
		"    const buf = len ? await readExactly(len) : Buffer.alloc(0);\n"
		"    if (!buf) break;\n"
		"    const code = buf.toString('utf8');\n"
		"    liveTimers = new Set();\n"
		"    handlesBefore = new Set(activeHandles());\n"
		"    const status = await new Promise((resolve) => {\n"
		"      // The call ends when the event loop drains: 'beforeExit' fires\n"
		"      // once nothing ref'd is pending (setTimeout, http.get, a\n"
		"      // server...).  Nothing of ours is pending here, the next read\n"
		"      // starts only after the status is written, so an unref'd\n"
		"      // leftover ends the call and goes on running while the child\n"
		"      // waits.  process.exitCode is then the status, as at an exit.\n"
		"      const drained = () => end(process.exitCode ?? 0);\n"
		"      end = (s) => { end = null; process.off('beforeExit', drained); resolve(s); };\n"
		"      process.once('beforeExit', drained);\n"
		"      try {\n"
		"        const result = (0, eval)(code);\n"
		"        // A returned promise's rejection is the call's error\n"
		"        if (result && typeof result.then === 'function') result.then(undefined, exited);\n"
		"      } catch (e) {\n"
		"        exited(e);\n"
		"      }\n"
		"    });\n"
		"    process.exitCode = undefined;\n"
		"    fs.writeSync(fd_out, Buffer.from([status & 0xff]));\n"
		"  }\n"
		"})();\n",
		fd_in, fd_out);

	fclose(f);
	return 0;
}

/* ── The child ─────────────────────────────────────────────────── */

/* The child is gone: drop our pipe ends and collect its exit status,
 * which is the status of the call it ended under. */
static int node_reap(void)
{
	int status;

	close(g_pipe_to_node);
	close(g_pipe_from_node);
	g_pipe_to_node = g_pipe_from_node = -1;
	status = hl_wait_status(g_node_pid);
	g_node_pid = -1;
	return status;
}

/*
 * Start the Node child: pipes, the bootstrap with the pipe fds baked
 * in, vfork + exec, then its ready byte.  Called at boot, and again
 * for the call after one that ended the child.
 */
static int node_spawn(void)
{
	int pipe_code[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	pid_t pid;
	char ready;

	if (pipe(pipe_code) < 0) {
		fprintf(stderr, "hl_nodedriver: pipe() failed\n");
		return -1;
	}
	if (pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_nodedriver: pipe() failed\n");
		close(pipe_code[0]);
		close(pipe_code[1]);
		return -1;
	}
	/* The child, and anything it runs, gets only its own two ends. */
	fcntl(pipe_code[1], F_SETFD, FD_CLOEXEC);
	fcntl(pipe_ack[0], F_SETFD, FD_CLOEXEC);
	if (write_bootstrap(pipe_code[0], pipe_ack[1]) < 0) {
		close(pipe_code[0]);
		close(pipe_code[1]);
		close(pipe_ack[0]);
		close(pipe_ack[1]);
		return -1;
	}

	/* vfork: parent blocks until child calls exec.  Child inherits all fds. */
	pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_nodedriver: vfork() failed\n");
		close(pipe_code[0]);
		close(pipe_code[1]);
		close(pipe_ack[0]);
		close(pipe_ack[1]);
		return -1;
	}
	if (pid == 0) {
		/* Child — only exec or _exit allowed after vfork */
		execl("/usr/bin/node", "node", "/tmp/hl_bootstrap.js",
		      (char *)NULL);
		_exit(127);
	}

	/* Parent — close the child's pipe ends */
	close(pipe_code[0]);
	close(pipe_ack[1]);
	g_node_pid = pid;
	g_pipe_to_node = pipe_code[1];
	g_pipe_from_node = pipe_ack[0];

	/* Wait for the child to signal ready.  This blocks, the scheduler
	 * switches to the child, V8 starts up, the bootstrap writes the
	 * ready byte. */
	if (read(g_pipe_from_node, &ready, 1) != 1) {
		fprintf(stderr, "hl_nodedriver: child failed to start\n");
		node_reap();
		return -1;
	}

	/* Bootstrap is loaded — remove it from the guest filesystem. */
	unlink("/tmp/hl_bootstrap.js");
	return 0;
}

/* ── Dispatch callback ─────────────────────────────────────────── */

static int node_dispatch(const uint8_t *fc, size_t fc_len)
{
	struct hl_strbuf env = { 0 };
	unsigned char status;
	uint64_t len64;
	int rc = -1;

	/* The call after one that ended the child starts a new one. */
	if (g_node_pid < 0 && node_spawn() < 0)
		return -1;

	/* Extract the code string from the FunctionCall FlatBuffer */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	/* Guest command (--guest-exec / autonomous): the command is written to a
	 * temp file and a fixed launcher reads it, sets process.argv, and requires
	 * the named guest module; an empty command requires /entrypoint.js. */
	size_t gx_len = code_len;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? code : NULL;
	static const char GX_LAUNCHER[] =
		"const fs=require('fs');\n"
		"let _c='';try{_c=fs.readFileSync('/tmp/hl_gx','utf8').trim();}catch(e){}\n"
		"const _a=_c.length?_c.split(/\\s+/):[];\n"
		"if(_a.length){process.argv=['node',..._a];require(_a[0]);}\n"
		"else if(fs.existsSync('/entrypoint.js')){require('/entrypoint.js');}\n"
		"else{console.log('hl: no /entrypoint.js in rootfs; nothing to run');}\n";
	if (gx) {
		FILE *gf = fopen("/tmp/hl_gx", "w");
		if (!gf) {
			fprintf(stderr, "hl_nodedriver: cannot write guest cmd\n");
			fflush(stderr);
			return -1;
		}
		if (gx_len)
			fwrite(gx, 1, gx_len, gf);
		fclose(gf);
		code = GX_LAUNCHER;
		code_len = sizeof(GX_LAUNCHER) - 1;
	}

	/* Refresh env vars: setenv() here, process.env lines for the child. */
	hl_env_refresh(node_env_visitor, &env);
	if (env.err) {
		fprintf(stderr, "hl_nodedriver: out of memory for the environment\n");
		fflush(stderr);
		goto out;
	}

	/* Send length (8 bytes LE) + env prefix + code, then wait for the
	 * status byte.  The read blocks and yields to the cooperative
	 * scheduler, which switches to the child Node thread. */
	len64 = (uint64_t)(env.len + code_len);
	if (hl_write_all(g_pipe_to_node, &len64, 8) < 0 ||
	    hl_write_all(g_pipe_to_node, env.buf, env.len) < 0 ||
	    hl_write_all(g_pipe_to_node, code, code_len) < 0) {
		/* The child had ended before the call reached it, so the call
		 * did not run and fails; the next one starts a fresh child. */
		fprintf(stderr, "hl_nodedriver: node had exited with status %d\n",
			node_reap());
		fflush(stderr);
		goto out;
	}
	if (read(g_pipe_from_node, &status, 1) != 1) {
		/* The child ended under the call: its exit status is the call's. */
		rc = node_reap();
		goto out;
	}
	rc = status;
out:
	hl_strbuf_free(&env);
	return rc;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	if (hl_driver_init("hl_nodedriver"))
		return 1;

	/* A write to a child that is gone is an error to handle, not a
	 * signal to die of. */
	signal(SIGPIPE, SIG_IGN);

	if (node_spawn() < 0)
		return 1;

	/* Serve named calls from the kernel's queue; never returns */
	hl_driver_run(node_dispatch);
}
