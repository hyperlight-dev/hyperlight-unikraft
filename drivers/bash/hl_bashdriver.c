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
 *     main() → hl_driver_init(): open /dev/hlcall
 *            → bash_spawn(): pipes, bootstrap script in
 *              /tmp/hl_bootstrap.sh, vfork + exec("/bin/sh", it),
 *              then the child's ready ack
 *            → hl_driver_run(): block in read() on the call queue
 *
 *   host: call("Exec", "echo hello")
 *     read() returns the call → bash_dispatch(fc, fc_len)
 *              → write code to /tmp/hl_dispatch.sh
 *              → write signal to pipe
 *              → read ack byte
 *              → back into read(): the call is done
 *
 * The shell stays alive across dispatches, so variables and the
 * working directory persist from one call to the next.  `exit` ends
 * it, which is the runtime: the call ends with that exit status, and
 * the next call starts a fresh shell.
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

static pid_t g_sh_pid = -1;
static int g_pipe_to_sh = -1;    /* parent writes signal here */
static int g_pipe_from_sh = -1;  /* parent reads ack here */

/* ── Bash env visitor ─────────────────────────────────────────── */

/*
 * The persistent hush child was spawned at boot and does not see the
 * parent's setenv(), so the dispatch script starts with an
 * `export KEY='value'` line per host variable, the value single-quoted
 * so the shell takes it as it is.  A key the shell would not accept as
 * a name is left to setenv() alone.
 */
static void bash_env_visitor(const char *key, const char *val, void *ctx)
{
	struct hl_strbuf *env = ctx;

	if (!hl_env_key_is_identifier(key))
		return;
	hl_strbuf_puts(env, "export ");
	hl_strbuf_puts(env, key);
	hl_strbuf_puts(env, "=");
	hl_strbuf_put_shquoted(env, val);
	hl_strbuf_puts(env, "\n");
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

/* ── The child ─────────────────────────────────────────────────── */

/* The shell is gone: drop our pipe ends and collect its exit status,
 * which is the status of the call it ended under. */
static int bash_reap(void)
{
	int status;

	close(g_pipe_to_sh);
	close(g_pipe_from_sh);
	g_pipe_to_sh = g_pipe_from_sh = -1;
	status = hl_wait_status(g_sh_pid);
	g_sh_pid = -1;
	return status;
}

/*
 * Start the shell: pipes, the bootstrap with the pipe fds baked in,
 * vfork + exec, then its ready ack.  Called at boot, and again for the
 * call after one that ended the shell.
 */
static int bash_spawn(void)
{
	int pipe_sig[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	pid_t pid;
	char ready;

	if (pipe(pipe_sig) < 0) {
		fprintf(stderr, "hl_bashdriver: pipe() failed\n");
		return -1;
	}
	if (pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_bashdriver: pipe() failed\n");
		close(pipe_sig[0]);
		close(pipe_sig[1]);
		return -1;
	}
	/* The child, and anything it runs, gets only its own two ends. */
	fcntl(pipe_sig[1], F_SETFD, FD_CLOEXEC);
	fcntl(pipe_ack[0], F_SETFD, FD_CLOEXEC);
	if (write_bootstrap(pipe_sig[0], pipe_ack[1]) < 0) {
		close(pipe_sig[0]);
		close(pipe_sig[1]);
		close(pipe_ack[0]);
		close(pipe_ack[1]);
		return -1;
	}

	/* vfork: parent blocks until child calls exec. */
	pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_bashdriver: vfork() failed\n");
		close(pipe_sig[0]);
		close(pipe_sig[1]);
		close(pipe_ack[0]);
		close(pipe_ack[1]);
		return -1;
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
	g_sh_pid = pid;
	g_pipe_to_sh = pipe_sig[1];
	g_pipe_from_sh = pipe_ack[0];

	/* Wait for the child to signal ready */
	if (read(g_pipe_from_sh, &ready, 1) != 1) {
		fprintf(stderr, "hl_bashdriver: shell failed to start\n");
		bash_reap();
		return -1;
	}

	/* Bootstrap is loaded — remove it from the guest filesystem. */
	unlink("/tmp/hl_bootstrap.sh");
	return 0;
}

/* ── Dispatch callback ─────────────────────────────────────────── */

static int bash_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* The call after one that ended the shell starts a new one. */
	if (g_sh_pid < 0 && bash_spawn() < 0)
		return -1;

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

	/* Refresh env vars: setenv() here, export lines for the shell. */
	struct hl_strbuf env = { 0 };
	hl_env_refresh(bash_env_visitor, &env);
	if (env.err) {
		fprintf(stderr, "hl_bashdriver: out of memory for the environment\n");
		fflush(stderr);
		hl_strbuf_free(&env);
		return -1;
	}

	/* Write code to temp file — the shell sources this */
	FILE *f = fopen("/tmp/hl_dispatch.sh", "w");
	if (!f) {
		fprintf(stderr, "hl_bashdriver: cannot write dispatch file\n");
		fflush(stderr);
		hl_strbuf_free(&env);
		return -1;
	}
	/* Prepend export lines if any */
	if (env.len > 0)
		fwrite(env.buf, 1, env.len, f);
	hl_strbuf_free(&env);
	fwrite(code, 1, code_len, f);
	fputc('\n', f);
	fclose(f);

	/* Signal the shell to source the file, then wait for its ack: '0'
	 * on success, '1' on error.  The read blocks and yields to the
	 * cooperative scheduler. */
	char ack = '1';
	int sent = write(g_pipe_to_sh, "g\n", 2) == 2;
	int acked = sent && read(g_pipe_from_sh, &ack, 1) == 1;

	/* Dispatch file consumed — remove from guest filesystem. */
	unlink("/tmp/hl_dispatch.sh");

	if (!sent) {
		/* The shell had ended before the call reached it, so the call
		 * did not run and fails; the next one starts a fresh shell. */
		fprintf(stderr, "hl_bashdriver: the shell had exited with status %d\n",
			bash_reap());
		fflush(stderr);
		return -1;
	}
	if (!acked) {
		/* The shell ended under the call (`exit`): its exit status is
		 * the call's. */
		return bash_reap();
	}
	/* The script's failure is status 1, as `sh script.sh` would be. */
	return ack != '0' ? 1 : 0;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	if (hl_driver_init("hl_bashdriver"))
		return 1;

	/* A write to a shell that is gone is an error to handle, not a
	 * signal to die of. */
	signal(SIGPIPE, SIG_IGN);

	if (bash_spawn() < 0)
		return 1;

	/* Serve named calls from the kernel's queue; never returns */
	hl_driver_run(bash_dispatch);
}
