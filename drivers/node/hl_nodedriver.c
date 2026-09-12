/*
 * hl_nodedriver — Node.js runtime driver for Hyperlight.
 *
 * Spawns a persistent Node.js child process during boot via
 * vfork+exec.  The child runs a dispatch loop that reads code
 * from a pipe, evals it, and writes an ack byte back.
 *
 * Flow:
 *   boot (evolve):
 *     main() → create pipes
 *            → write bootstrap JS to /tmp/hl_bootstrap.js
 *            → vfork + exec("node", "/tmp/hl_bootstrap.js")
 *            → read "ready" ack from child (blocks, scheduler
 *              switches to child, V8 starts up, child signals)
 *            → register dispatch callback, halt
 *
 *   host: call("Exec", "console.log(42)")
 *     dispatch → hyperlight_dispatch_function (kernel)
 *              → node_dispatch(fc, fc_len)
 *              → write [len:u64][code] to pipe
 *              → read ack byte (blocks, scheduler switches to
 *                child, child evals code, writes ack)
 *              → halt
 *
 * The child Node process stays alive across dispatches — no V8
 * startup cost per call, and no exit/cleanup hang.
 *
 * TODO: The persistent child is a workaround for a vfork+exec hang
 * where the child Node process can't exit cleanly.  The root cause
 * is likely that hyperlight_dispatch_function doesn't restore the
 * cooperative scheduler's state (current thread, run queues) on
 * re-entry — the same class of bug we fixed for VFS, fd table,
 * and SYSCALL MSRs.  Once scheduler state is properly restored on
 * dispatch re-entry, a simpler per-dispatch vfork+exec+waitpid
 * approach should work.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

#include "../hl_fc.h"
#include "../hl_env.h"
#include "../hl_driver.h"

/* ── State ─────────────────────────────────────────────────────── */

static int g_pipe_to_node;    /* parent writes code here */
static int g_pipe_from_node;  /* parent reads ack here */

/* ── Node env visitor ─────────────────────────────────────────── */

/*
 * Build `process.env["KEY"]="VALUE";\n` lines.  The persistent
 * Node child was spawned at boot and doesn't see setenv changes
 * in the parent.  Prepending process.env assignments to the
 * eval'd code is the only way to propagate host env vars.
 */
struct node_env_buf {
	char *buf;
	size_t pos;
	size_t cap;
};

static void node_env_visitor(const char *key, const char *val, void *ctx)
{
	struct node_env_buf *eb = (struct node_env_buf *)ctx;
	/* process.env["KEY"]="VALUE";\n */
	size_t need = 16 + strlen(key) + strlen(val) + 8;
	if (eb->pos + need >= eb->cap)
		return;
	eb->pos += snprintf(eb->buf + eb->pos, eb->cap - eb->pos,
			    "process.env[\"%s\"]=\"%s\";\n", key, val);
}

/* ── Dispatch callback ─────────────────────────────────────────── */

static int node_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* Refresh env vars — updates glibc environ and builds
	 * process.env assignments for the Node child. */
	char env_prefix[4096];
	struct node_env_buf eb = { env_prefix, 0, sizeof(env_prefix) };
	hl_env_refresh(node_env_visitor, &eb);

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

	/* Send length (8 bytes LE) + env prefix + code to the child */
	uint64_t len64 = (uint64_t)(eb.pos + code_len);
	if (write(g_pipe_to_node, &len64, 8) != 8) {
		fprintf(stderr, "hl_nodedriver: pipe write (len) failed\n");
		fflush(stderr);
		return -1;
	}
	/* Write env prefix first, then user code */
	if (eb.pos > 0) {
		const char *ep = env_prefix;
		size_t erem = eb.pos;
		while (erem > 0) {
			ssize_t n = write(g_pipe_to_node, ep, erem);
			if (n <= 0) {
				fprintf(stderr, "hl_nodedriver: pipe write (env) failed\n");
				fflush(stderr);
				return -1;
			}
			ep += n;
			erem -= n;
		}
	}
	const char *p = code;
	size_t remaining = code_len;
	while (remaining > 0) {
		ssize_t n = write(g_pipe_to_node, p, remaining);
		if (n <= 0) {
			fprintf(stderr, "hl_nodedriver: pipe write (data) failed\n");
			fflush(stderr);
			return -1;
		}
		p += n;
		remaining -= n;
	}

	/* Wait for ack — child sends 0x00 on success, 0x01 on error.
	 * This read blocks and yields to the cooperative scheduler,
	 * which switches to the child Node thread. */
	char ack = 1;
	if (read(g_pipe_from_node, &ack, 1) != 1) {
		fprintf(stderr, "hl_nodedriver: ack read failed\n");
		fflush(stderr);
		return -1;
	}
	return ack != 0 ? -1 : 0;
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
		"// Async-aware dispatch loop.\n"
		"// readSync blocks (yields to cooperative scheduler), eval runs\n"
		"// the code, then we await the event loop draining via 'beforeExit'\n"
		"// before acking — so readline, setTimeout, http.get, and any\n"
		"// other async pattern works out of the box.\n"
		"const hdr = Buffer.alloc(8);\n"
		"(async () => {\n"
		"  while (true) {\n"
		"    let n = fs.readSync(fd_in, hdr, 0, 8);\n"
		"    if (n < 8) break;\n"
		"    let len = Number(hdr.readBigUInt64LE(0));\n"
		"    let buf = Buffer.alloc(len);\n"
		"    let off = 0;\n"
		"    while (off < len) {\n"
		"      n = fs.readSync(fd_in, buf, off, len - off);\n"
		"      if (n <= 0) process.exit(1);\n"
		"      off += n;\n"
		"    }\n"
		"    let code = buf.toString('utf8');\n"
		"    try {\n"
		"      let result = (0, eval)(code);\n"
		"      // If eval returned a Promise, await it first\n"
		"      if (result && typeof result.then === 'function') {\n"
		"        await result;\n"
		"      }\n"
		"      // Wait for the event loop to drain — 'beforeExit' fires\n"
		"      // when Node has no pending I/O, timers, or callbacks.\n"
		"      // For sync code this resolves immediately (one tick).\n"
		"      // For async code (setTimeout, http.get, readline, etc.)\n"
		"      // it waits until all async work completes.\n"
		"      await new Promise(r => process.once('beforeExit', r));\n"
		"      fs.writeSync(fd_out, Buffer.from([0]));  // success ack\n"
		"    } catch (e) {\n"
		"      console.error(e.stack || e);\n"
		"      fs.writeSync(fd_out, Buffer.from([1]));  // error ack\n"
		"    }\n"
		"  }\n"
		"})();\n",
		fd_in, fd_out);

	fclose(f);
	return 0;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv, char **envp)
{
	(void)argc;
	(void)argv;

	/* Parse kernel addresses from env vars */
	if (hl_driver_init(envp, "hl_nodedriver"))
		return 1;

	/* Create pipes: parent→child (code) and child→parent (ack) */
	int pipe_code[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	if (pipe(pipe_code) < 0 || pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_nodedriver: pipe() failed\n");
		return 1;
	}

	/* Write bootstrap JS with the pipe fd numbers baked in */
	if (write_bootstrap(pipe_code[0], pipe_ack[1]) < 0)
		return 1;

	/* Spawn persistent Node child.
	 * vfork: parent blocks until child calls exec.
	 * After exec, parent resumes.  Child inherits all fds. */
	pid_t pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_nodedriver: vfork() failed\n");
		return 1;
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
	g_pipe_to_node = pipe_code[1];
	g_pipe_from_node = pipe_ack[0];

	/* Wait for the child to signal ready.
	 * This blocks, the scheduler switches to the child,
	 * V8 starts up, the bootstrap writes the ready byte. */
	char ready;
	if (read(g_pipe_from_node, &ready, 1) != 1) {
		fprintf(stderr, "hl_nodedriver: child failed to start\n");
		return 1;
	}

	/* Bootstrap is loaded — remove it from the guest filesystem. */
	unlink("/tmp/hl_bootstrap.js");

	/* Register dispatch callback */
	hl_driver_run(node_dispatch);
}
