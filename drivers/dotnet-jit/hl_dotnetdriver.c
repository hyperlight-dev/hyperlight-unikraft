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
 *     main() → set .NET env vars (W^X, GC limits, etc.)
 *            → create pipes
 *            → vfork + exec("/app/HlDotnetDispatch", fd_in, fd_out)
 *            → read "ready" ack from child (blocks, scheduler
 *              switches to child, CoreCLR + Roslyn warm up,
 *              child signals)
 *            → register dispatch callback, halt
 *
 *   host: call("Exec", <C# source code>)
 *     dispatch → hyperlight_dispatch_function (kernel)
 *              → dotnet_dispatch(fc, fc_len)
 *              → write [len:u64][payload] to pipe
 *              → read ack byte (blocks, scheduler switches to
 *                child, Roslyn compile + execute, writes ack)
 *              → halt
 *
 * Roslyn is warmed up during boot so snapshots capture the
 * initialized state — dispatches after restore are fast.
 *
 * The .NET process stays alive across dispatches.  Each dispatch
 * compiles and runs independently (no shared state between calls).
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

static int g_pipe_to_dotnet;    /* parent writes payload here */
static int g_pipe_from_dotnet;  /* parent reads ack here */

/* ── .NET env visitor ──────────────────────────────────────────── */

/*
 * Append an `Environment.SetEnvironmentVariable("KEY", "VALUE");`
 * line to a buffer.  The persistent .NET child was spawned at boot
 * and doesn't see setenv changes in the parent — prepending env
 * assignments to the compiled C# code is the only way to propagate
 * host env vars to the Roslyn-executed scripts.
 *
 * RoslynCompiler.cs already prepends `using System;`, so
 * `Environment.SetEnvironmentVariable` resolves without a full
 * namespace qualifier.
 */
struct dotnet_env_buf {
	char *buf;
	size_t pos;
	size_t cap;
};

static void dotnet_env_visitor(const char *key, const char *val, void *ctx)
{
	struct dotnet_env_buf *db = (struct dotnet_env_buf *)ctx;
	/* Worst case: Environment.SetEnvironmentVariable("...","...");\n */
	size_t need = 48 + strlen(key) + strlen(val) * 2 + 8;
	if (db->pos + need >= db->cap)
		return; /* buffer full — skip */

	db->pos += snprintf(db->buf + db->pos, db->cap - db->pos,
			    "Environment.SetEnvironmentVariable(\"");
	/* Key — escape quotes and backslashes */
	for (const char *p = key; *p && db->pos < db->cap - 4; p++) {
		if (*p == '"' || *p == '\\')
			db->buf[db->pos++] = '\\';
		db->buf[db->pos++] = *p;
	}
	if (db->pos + 4 < db->cap) {
		db->buf[db->pos++] = '"';
		db->buf[db->pos++] = ',';
		db->buf[db->pos++] = ' ';
		db->buf[db->pos++] = '"';
	}
	/* Value — escape quotes and backslashes */
	for (const char *p = val; *p && db->pos < db->cap - 4; p++) {
		if (*p == '"' || *p == '\\')
			db->buf[db->pos++] = '\\';
		db->buf[db->pos++] = *p;
	}
	if (db->pos + 4 < db->cap) {
		db->buf[db->pos++] = '"';
		db->buf[db->pos++] = ')';
		db->buf[db->pos++] = ';';
		db->buf[db->pos++] = '\n';
	}
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

	/* Build env var prefix for the compiled C# code.
	 * Also refreshes glibc environ (for the parent process). */
	char env_prefix[4096];
	struct dotnet_env_buf db = { env_prefix, 0, sizeof(env_prefix) };
	hl_env_refresh(dotnet_env_visitor, &db);
	env_prefix[db.pos] = '\0';

	int rc = -1;

	/* Send length (8 bytes LE) + env prefix + code to the child */
	uint64_t len64 = (uint64_t)(db.pos + code_len);
	if (write(g_pipe_to_dotnet, &len64, 8) != 8)
		goto out;
	/* Write env prefix first, then the user code */
	if (db.pos > 0) {
		const char *ep = env_prefix;
		size_t erem = db.pos;
		while (erem > 0) {
			ssize_t n = write(g_pipe_to_dotnet, ep, erem);
			if (n <= 0)
				goto out;
			ep += n;
			erem -= n;
		}
	}
	const char *p = code;
	size_t remaining = code_len;
	while (remaining > 0) {
		ssize_t n = write(g_pipe_to_dotnet, p, remaining);
		if (n <= 0)
			goto out;
		p += n;
		remaining -= n;
	}

	/* Wait for ack — child sends 0x00 on success, 0x01 on error.
	 * This read blocks and yields to the cooperative scheduler,
	 * which switches to the child .NET thread. */
	char ack = 1;
	if (read(g_pipe_from_dotnet, &ack, 1) != 1)
		goto out;
	rc = ack != 0 ? -1 : 0;

out:
	free(auton_buf);
	return rc;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv, char **envp)
{
	(void)argc;
	(void)argv;

	/* Parse kernel addresses from env vars */
	if (hl_driver_init(envp, "hl_dotnetdriver"))
		return 1;

	/*
	 * Set .NET runtime environment variables before spawning the
	 * child.  These are critical for CoreCLR in a unikernel:
	 *
	 *   W^X:         JIT needs write+execute on the same pages.
	 *   Diagnostics: Named pipes / IPC that doesn't work here.
	 *   GC heap:     768 MB — Roslyn compilation needs significant
	 *                heap for metadata references and compilation
	 *                data structures.
	 *   Server GC:   Disable (single-CPU environment).
	 *   Globalization: Skip ICU (not in rootfs).
	 *   Stack size:  256 KB per thread — Roslyn's deep call stacks
	 *                need more than the default 64 KB.
	 *   ThreadPool:  Limit thread count for cooperative scheduler.
	 */
	setenv("DOTNET_EnableWriteXorExecute", "0", 1);
	setenv("COMPlus_EnableDiagnostics", "0", 1);
	setenv("DOTNET_GCHeapHardLimit", "0x30000000", 1);
	setenv("DOTNET_gcServer", "0", 1);
	setenv("DOTNET_SYSTEM_GLOBALIZATION_INVARIANT", "1", 1);
	setenv("DOTNET_DefaultStackSize", "0x40000", 1);
	setenv("DOTNET_ThreadPool_ForceMinWorkerThreads", "1", 1);
	setenv("DOTNET_ThreadPool_ForceMaxWorkerThreads", "4", 1);

	/* Create pipes: parent→child (payload) and child→parent (ack) */
	int pipe_code[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	if (pipe(pipe_code) < 0 || pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_dotnetdriver: pipe() failed\n");
		return 1;
	}

	/* Spawn persistent .NET dispatch server.
	 * Pass pipe fd numbers as command-line arguments. */
	char fd_in_str[16], fd_out_str[16];
	snprintf(fd_in_str, sizeof(fd_in_str), "%d", pipe_code[0]);
	snprintf(fd_out_str, sizeof(fd_out_str), "%d", pipe_ack[1]);

	pid_t pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_dotnetdriver: vfork() failed\n");
		return 1;
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
	g_pipe_to_dotnet = pipe_code[1];
	g_pipe_from_dotnet = pipe_ack[0];

	/* Wait for the child to signal ready.
	 * This blocks, the scheduler switches to the child,
	 * CoreCLR starts up, the dispatch server writes the ready byte. */
	char ready;
	if (read(g_pipe_from_dotnet, &ready, 1) != 1) {
		fprintf(stderr,
			"hl_dotnetdriver: .NET dispatch server "
			"failed to start\n");
		return 1;
	}

	/* Register dispatch callback */
	hl_driver_run(dotnet_dispatch);
}
