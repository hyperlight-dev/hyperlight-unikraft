/*
 * hl_dotnetdriver — .NET JIT runtime driver for Hyperlight.
 *
 * Spawns a persistent .NET dispatch server during boot via
 * vfork+exec.  The server uses the Roslyn compilation API to compile
 * and execute raw C# source code in-guest — same dispatch pattern
 * as the Python and Node drivers.
 *
 * Flow:
 *   boot (evolve):
 *     main() → hl_driver_init(): open /dev/hlcall
 *            → set .NET env vars (W^X, GC limit, etc.)
 *            → dotnet_spawn(): pipes, vfork + exec("/app/HlDotnetDispatch",
 *              fd_in, fd_out), then the child's ready byte (blocks,
 *              scheduler switches to child, CoreCLR + Roslyn warm up,
 *              child signals)
 *            → hl_driver_run(): block in read() on the call queue
 *
 *   host: call("Exec", <C# source code>)
 *     read() returns the call → dotnet_dispatch(fc, fc_len)
 *              → write [len:u64][payload] to pipe
 *              → read status byte (blocks, scheduler switches to
 *                child, Roslyn compile + execute, writes the status)
 *              → back into read(): the call is done
 *
 * Roslyn is warmed up during boot so snapshots capture the
 * initialized state — dispatches after restore are fast.
 *
 * The .NET process stays alive across dispatches.  Each dispatch
 * compiles and runs independently (no shared state between calls).
 * Environment.Exit() ends the process, which is the runtime: the call
 * ends with that exit status, and the next call starts a fresh one,
 * warm-up included.  Until the kernel returns an exited process's
 * memory, that fresh one needs the room for a second runtime.
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

static pid_t g_dotnet_pid = -1;
static int g_pipe_to_dotnet = -1;    /* parent writes payload here */
static int g_pipe_from_dotnet = -1;  /* parent reads the status here */

/* ── .NET env visitor ──────────────────────────────────────────── */

/*
 * The persistent .NET child was spawned at boot and does not see the
 * parent's setenv(), so each call's source is prefixed with an
 * `Environment.SetEnvironmentVariable(<key>, <value>);` line per host
 * variable, key and value as JSON string literals, which C# reads the
 * same way.  RoslynCompiler.cs prepends `using System;`, so the call
 * resolves without a namespace qualifier.
 */
static void dotnet_env_visitor(const char *key, const char *val, void *ctx)
{
	struct hl_strbuf *env = ctx;

	hl_strbuf_puts(env, "Environment.SetEnvironmentVariable(");
	hl_strbuf_put_json(env, key);
	hl_strbuf_puts(env, ", ");
	hl_strbuf_put_json(env, val);
	hl_strbuf_puts(env, ");\n");
}

/* ── The child ─────────────────────────────────────────────────── */

/* The child is gone: drop our pipe ends and collect its exit status,
 * which is the status of the call it ended under. */
static int dotnet_reap(void)
{
	int status;

	close(g_pipe_to_dotnet);
	close(g_pipe_from_dotnet);
	g_pipe_to_dotnet = g_pipe_from_dotnet = -1;
	status = hl_wait_status(g_dotnet_pid);
	g_dotnet_pid = -1;
	return status;
}

/*
 * Start the .NET dispatch server: pipes, vfork + exec with the pipe
 * fds as its arguments, then its ready byte, which comes once CoreCLR
 * and Roslyn are warm.  Called at boot, and again for the call after
 * one that ended the child.
 */
static int dotnet_spawn(void)
{
	int pipe_code[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	char fd_in_str[16], fd_out_str[16];
	pid_t pid;
	char ready;

	if (pipe(pipe_code) < 0) {
		fprintf(stderr, "hl_dotnetdriver: pipe() failed\n");
		return -1;
	}
	if (pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_dotnetdriver: pipe() failed\n");
		close(pipe_code[0]);
		close(pipe_code[1]);
		return -1;
	}
	/* The child, and anything it runs, gets only its own two ends. */
	fcntl(pipe_code[1], F_SETFD, FD_CLOEXEC);
	fcntl(pipe_ack[0], F_SETFD, FD_CLOEXEC);
	snprintf(fd_in_str, sizeof(fd_in_str), "%d", pipe_code[0]);
	snprintf(fd_out_str, sizeof(fd_out_str), "%d", pipe_ack[1]);

	pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_dotnetdriver: vfork() failed\n");
		close(pipe_code[0]);
		close(pipe_code[1]);
		close(pipe_ack[0]);
		close(pipe_ack[1]);
		return -1;
	}
	if (pid == 0) {
		/* Child — only exec or _exit allowed after vfork */
		execl("/app/HlDotnetDispatch", "HlDotnetDispatch",
		      fd_in_str, fd_out_str, (char *)NULL);
		_exit(127);
	}

	/* Parent — close the child's pipe ends */
	close(pipe_code[0]);
	close(pipe_ack[1]);
	g_dotnet_pid = pid;
	g_pipe_to_dotnet = pipe_code[1];
	g_pipe_from_dotnet = pipe_ack[0];

	/* Wait for the child to signal ready.  This blocks, the scheduler
	 * switches to the child, CoreCLR starts up, the dispatch server
	 * writes the ready byte. */
	if (read(g_pipe_from_dotnet, &ready, 1) != 1) {
		fprintf(stderr, "hl_dotnetdriver: .NET dispatch server failed to start\n");
		dotnet_reap();
		return -1;
	}
	return 0;
}

/* ── Dispatch callback ─────────────────────────────────────────── */

static int dotnet_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* Extract the C# source code from the FunctionCall FlatBuffer. */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	/* Guest command (--guest-exec / autonomous): compile+run the named .cs
	 * file from the initrd (the command's first token; args are not forwarded
	 * to C# yet), or the conventional /entrypoint.cs when empty. */
	char *auton_buf = NULL;
	size_t gx_len = code_len;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? code : NULL;
	if (gx) {
		char pathbuf[4096];
		char *av[64];
		const char *src = gx_len ? gx : "/entrypoint.cs";
		size_t n = gx_len ? gx_len : strlen("/entrypoint.cs");
		if (n >= sizeof(pathbuf))
			return -1;
		memcpy(pathbuf, src, n);
		pathbuf[n] = '\0';
		hl_split_ws(pathbuf, av, 64);
		const char *cspath = av[0] ? av[0] : "/entrypoint.cs";
		FILE *ef = fopen(cspath, "rb");
		if (!ef) {
			fprintf(stderr, "hl: no %s in rootfs; nothing to run\n",
				cspath);
			fflush(stderr);
			return 0;
		}
		long sz = -1;
		if (fseek(ef, 0, SEEK_END) == 0)
			sz = ftell(ef);
		if (sz < 0 || fseek(ef, 0, SEEK_SET) != 0) {
			fclose(ef);
			return -1;
		}
		auton_buf = malloc((size_t)sz + 1);
		if (!auton_buf) {
			fclose(ef);
			return -1;
		}
		size_t rd = fread(auton_buf, 1, (size_t)sz, ef);
		fclose(ef);
		auton_buf[rd] = '\0';
		code = auton_buf;
		code_len = rd;
	}

	struct hl_strbuf env = { 0 };
	unsigned char status;
	uint64_t len64;
	int rc = -1;

	/* The call after one that ended the child starts a new one. */
	if (g_dotnet_pid < 0 && dotnet_spawn() < 0)
		goto out;

	/* Refresh env vars: setenv() here, SetEnvironmentVariable lines for
	 * the child. */
	hl_env_refresh(dotnet_env_visitor, &env);
	if (env.err) {
		fprintf(stderr, "hl_dotnetdriver: out of memory for the environment\n");
		fflush(stderr);
		goto out;
	}

	/* Send length (8 bytes LE) + env prefix + code, then wait for the
	 * status byte: 0 on success, 1 on error.  The read blocks and
	 * yields to the cooperative scheduler, which switches to the child
	 * .NET thread. */
	len64 = (uint64_t)(env.len + code_len);
	if (hl_write_all(g_pipe_to_dotnet, &len64, 8) < 0 ||
	    hl_write_all(g_pipe_to_dotnet, env.buf, env.len) < 0 ||
	    hl_write_all(g_pipe_to_dotnet, code, code_len) < 0) {
		/* The child had ended before the call reached it, so the call
		 * did not run and fails; the next one starts a fresh child. */
		fprintf(stderr, "hl_dotnetdriver: .NET had exited with status %d\n",
			dotnet_reap());
		fflush(stderr);
		goto out;
	}
	if (read(g_pipe_from_dotnet, &status, 1) != 1) {
		/* The child ended under the call (Environment.Exit, a crash):
		 * its exit status is the call's. */
		rc = dotnet_reap();
		goto out;
	}
	rc = status;

out:
	hl_strbuf_free(&env);
	free(auton_buf);
	return rc;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	if (hl_driver_init("hl_dotnetdriver"))
		return 1;

	/*
	 * Set .NET runtime environment variables before spawning the
	 * child.  These are critical for CoreCLR in a unikernel:
	 *
	 *   W^X:         JIT needs write+execute on the same pages.
	 *   Diagnostics: Named pipes / IPC that doesn't work here.
	 *   GC heap:     a hard limit at a share of the memory the kernel
	 *                reports (sysconf), so that it tracks the scratch
	 *                size and the GC throws OutOfMemoryException before
	 *                the kernel runs out of pages and faults the
	 *                process.  The rest of the memory holds the rootfs,
	 *                CoreCLR's own allocations and the kernel.  GC
	 *                settings are hexadecimal: 0x32 is 50 %.
	 *   Server GC:   Disable (single-CPU environment).
	 *   Globalization: Skip ICU (not in rootfs).
	 *   Stack size:  256 KB per thread — Roslyn's deep call stacks
	 *                need more than the default 64 KB.
	 *   ThreadPool:  Limit thread count for cooperative scheduler.
	 */
	setenv("DOTNET_EnableWriteXorExecute", "0", 1);
	setenv("COMPlus_EnableDiagnostics", "0", 1);
	setenv("DOTNET_GCHeapHardLimitPercent", "0x32", 1);
	setenv("DOTNET_gcServer", "0", 1);
	setenv("DOTNET_SYSTEM_GLOBALIZATION_INVARIANT", "1", 1);
	setenv("DOTNET_DefaultStackSize", "0x40000", 1);
	setenv("DOTNET_ThreadPool_ForceMinWorkerThreads", "1", 1);
	setenv("DOTNET_ThreadPool_ForceMaxWorkerThreads", "4", 1);

	/* A write to a child that is gone is an error to handle, not a
	 * signal to die of. */
	signal(SIGPIPE, SIG_IGN);

	if (dotnet_spawn() < 0)
		return 1;

	/* Serve named calls from the kernel's queue; never returns */
	hl_driver_run(dotnet_dispatch);
}
