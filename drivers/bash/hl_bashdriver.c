/*
 * hl_bashdriver — BusyBox hush runtime driver for Hyperlight.
 *
 * Spawns a persistent BusyBox hush child process during boot via
 * vfork+exec.  The child runs a dispatch loop that reads a signal
 * from a pipe, sources /tmp/hl_dispatch.sh, and writes an ack
 * byte back.
 *
 * BusyBox is built in NOMMU mode so hush uses vfork() instead of
 * fork().  All applets (cat, grep, sed, awk, sort, ls, find, etc.)
 * run as separate processes via vfork+execve.  NOFORK applets
 * (echo, mkdir, touch, etc.) still run in-process for speed.
 *
 * Limitation: pipes (|) and output redirects from external commands
 * (cmd > file) crash the kernel because vfork shares the parent's
 * stack with the child.  A kernel fix (separate child stack) is needed.
 * Use temp files from builtins or awk single-pass for chaining.
 *
 * Flow:
 *   boot (evolve):
 *     main() → create pipes
 *            → write bootstrap script to /tmp/hl_bootstrap.sh
 *            → vfork + exec("/bin/sh", "/tmp/hl_bootstrap.sh")
 *            → read "ready" ack from child
 *            → register dispatch callback, halt
 *
 *   host: call("Exec", "echo hello")
 *     dispatch → hyperlight_dispatch_function (kernel)
 *              → bash_dispatch(fc, fc_len)
 *              → write code to /tmp/hl_dispatch.sh
 *              → write signal to pipe
 *              → read ack byte
 *              → halt
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

static int g_pipe_to_sh;    /* parent writes signal here */
static int g_pipe_from_sh;  /* parent reads ack here */

/* ── Bash env visitor ─────────────────────────────────────────── */

/*
 * Build `export KEY="VALUE"\n` lines.  The persistent hush child
 * was spawned at boot and doesn't see setenv changes in the parent.
 * Prepending export lines to the dispatch script is the only way
 * to propagate host env vars to shell code.
 */
struct bash_env_buf {
	char *buf;
	size_t pos;
	size_t cap;
};

static void bash_env_visitor(const char *key, const char *val, void *ctx)
{
	struct bash_env_buf *eb = (struct bash_env_buf *)ctx;
	size_t need = 10 + strlen(key) + strlen(val) + 4;
	if (eb->pos + need >= eb->cap)
		return;
	eb->pos += snprintf(eb->buf + eb->pos, eb->cap - eb->pos,
			    "export %s=\"%s\"\n", key, val);
}

/* ── Dispatch callback ─────────────────────────────────────────── */

static int bash_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* Refresh env vars — updates glibc environ and builds
	 * export lines for the shell. */
	char env_prefix[4096];
	struct bash_env_buf eb = { env_prefix, 0, sizeof(env_prefix) };
	hl_env_refresh(bash_env_visitor, &eb);

	/* Extract the code string from the FunctionCall FlatBuffer */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	/* Guest command (--guest-exec / autonomous): the command is shell, so
	 * run it as-is ("path args" executes that script with its args); an empty
	 * command sources the conventional /entrypoint.sh. */
	size_t gx_len = code_len;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? code : NULL;
	static const char GX_DEFAULT[] =
		"if [ -f /entrypoint.sh ]; then . /entrypoint.sh; "
		"else echo 'hl: no /entrypoint.sh in rootfs; nothing to run'; fi\n";
	if (gx) {
		if (gx_len == 0) {
			code = GX_DEFAULT;
			code_len = sizeof(GX_DEFAULT) - 1;
		} else {
			code = gx;
			code_len = gx_len;
		}
	}

	/* Write code to temp file — the shell sources this */
	FILE *f = fopen("/tmp/hl_dispatch.sh", "w");
	if (!f) {
		fprintf(stderr, "hl_bashdriver: cannot write dispatch file\n");
		fflush(stderr);
		return -1;
	}
	/* Prepend export lines if any */
	if (eb.pos > 0)
		fwrite(env_prefix, 1, eb.pos, f);
	fwrite(code, 1, code_len, f);
	fputc('\n', f);
	fclose(f);

	/* Signal the shell to source the file */
	if (write(g_pipe_to_sh, "g\n", 2) != 2) {
		fprintf(stderr, "hl_bashdriver: pipe write (signal) failed\n");
		fflush(stderr);
		return -1;
	}

	/* Wait for ack — '0' on success, '1' on error.
	 * This read blocks and yields to the cooperative scheduler. */
	char ack = '1';
	if (read(g_pipe_from_sh, &ack, 1) != 1) {
		fprintf(stderr, "hl_bashdriver: ack read failed\n");
		fflush(stderr);
		return -1;
	}

	/* Dispatch file consumed — remove from guest filesystem. */
	unlink("/tmp/hl_dispatch.sh");

	return ack != '0' ? -1 : 0;
}

/* ── Bootstrap script ─────────────────────────────────────────── */

static int write_bootstrap(int fd_in, int fd_out)
{
	const char *path = "/tmp/hl_bootstrap.sh";
	FILE *f = fopen(path, "w");
	if (!f) {
		fprintf(stderr, "hl_bashdriver: cannot create %s\n", path);
		return -1;
	}

	fprintf(f,
		"#!/bin/sh\n"
		"# Hyperlight bash dispatch loop.\n"
		"# Reads signals from FD %d, sources /tmp/hl_dispatch.sh,\n"
		"# writes ack byte ('0'=success, '1'=error) to FD %d.\n"
		"\n"
		"# Signal ready\n"
		"printf '0' >&%d\n"
		"\n"
		"# Dispatch loop\n"
		"while read -r _signal <&%d; do\n"
		"    if . /tmp/hl_dispatch.sh; then\n"
		"        printf '0' >&%d\n"
		"    else\n"
		"        printf '1' >&%d\n"
		"    fi\n"
		"done\n",
		fd_in, fd_out,
		fd_out,
		fd_in,
		fd_out,
		fd_out);

	fclose(f);
	return 0;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv, char **envp)
{
	(void)argc;
	(void)argv;

	/* Parse kernel addresses from env vars */
	if (hl_driver_init(envp, "hl_bashdriver"))
		return 1;

	/* Create pipes: parent→child (signal) and child→parent (ack) */
	int pipe_sig[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	if (pipe(pipe_sig) < 0 || pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_bashdriver: pipe() failed\n");
		return 1;
	}

	/* Write bootstrap script with pipe fd numbers baked in */
	if (write_bootstrap(pipe_sig[0], pipe_ack[1]) < 0)
		return 1;

	/* Spawn persistent shell child.
	 * vfork: parent blocks until child calls exec. */
	pid_t pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_bashdriver: vfork() failed\n");
		return 1;
	}
	if (pid == 0) {
		/* Child — only exec or _exit allowed after vfork */
		execl("/bin/sh", "sh", "/tmp/hl_bootstrap.sh",
		      (char *)NULL);
		_exit(127);
	}

	/* Parent — close the child's pipe ends */
	close(pipe_sig[0]);
	close(pipe_ack[1]);
	g_pipe_to_sh = pipe_sig[1];
	g_pipe_from_sh = pipe_ack[0];

	/* Wait for the child to signal ready */
	char ready;
	if (read(g_pipe_from_sh, &ready, 1) != 1) {
		fprintf(stderr, "hl_bashdriver: shell failed to start\n");
		return 1;
	}

	/* Bootstrap is loaded — remove it from the guest filesystem. */
	unlink("/tmp/hl_bootstrap.sh");

	/* Register dispatch callback */
	hl_driver_run(bash_dispatch);
}
