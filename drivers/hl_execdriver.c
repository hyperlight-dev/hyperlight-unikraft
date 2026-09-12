/*
 * hl_execdriver — Exec-based runtime driver for compiled languages.
 *
 * On dispatch, receives a binary path (e.g. "/mnt/bin/hello") and runs
 * it via vfork+execve.  Exit detection uses a pipe: the child inherits
 * the write end, and when it exits the write end closes, producing
 * EOF on the parent's read.  Used by C, Rust, Go, and .NET Native AOT
 * runtimes where the user mounts a host directory containing the
 * compiled binary into the guest.
 *
 * Flow:
 *   boot (evolve):
 *     main() → register dispatch callback → halt
 *
 *   host: call("Exec", "/bin/hello")
 *     dispatch → exec_dispatch(fc, fc_len)
 *              → pipe() + vfork + execl(path)
 *              → read(pipe) blocks until child exits (EOF)
 *              → return → halt
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

#include "hl_fc.h"
#include "hl_env.h"
#include "hl_driver.h"

/* ── Dispatch callback ─────────────────────────────────────────── */

static int exec_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* Refresh glibc environ so the exec'd child inherits host vars. */
	hl_env_refresh(NULL, NULL);

	/* Extract the command from the FunctionCall FlatBuffer */
	size_t cmd_len;
	const char *cmd = fc_arg0_string(fc, fc_len, &cmd_len);

	char cmdbuf[4096];
	char *argv[64];
	size_t gx_len = cmd ? cmd_len : 0;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? cmd : NULL;
	if (gx) {
		/* Guest command: "path [args]" (empty → /entrypoint), i.e. a
		 * --guest-exec or autonomous urunc-style run. */
		const char *src = gx_len ? gx : "/entrypoint";
		size_t n = gx_len ? gx_len : strlen("/entrypoint");
		if (n >= sizeof(cmdbuf))
			return -1;
		memcpy(cmdbuf, src, n);
		cmdbuf[n] = '\0';
		hl_split_ws(cmdbuf, argv, 64);
	} else {
		/* Bare binary path (--exec /mnt/bin/foo). */
		if (!cmd || cmd_len == 0 || cmd_len >= sizeof(cmdbuf))
			return -1;
		memcpy(cmdbuf, cmd, cmd_len);
		cmdbuf[cmd_len] = '\0';
		argv[0] = cmdbuf;
		argv[1] = NULL;
	}
	if (!argv[0])
		return 0;
	if (access(argv[0], X_OK) != 0) {
		fprintf(stderr, "hl: no executable %s in rootfs; nothing to run\n",
			argv[0]);
		fflush(stderr);
		return 0;
	}

	/* Create a pipe for exit detection.  The child inherits the
	 * write end via exec (no CLOEXEC).  When the child exits,
	 * the kernel closes its fds, dropping the last writer and
	 * producing EOF on the parent's read end. */
	int fds[2];
	if (pipe(fds) < 0) {
		fprintf(stderr, "hl_execdriver: pipe() failed\n");
		fflush(stderr);
		return -1;
	}

	pid_t pid = vfork();
	if (pid < 0) {
		close(fds[0]);
		close(fds[1]);
		fprintf(stderr, "hl_execdriver: vfork() failed\n");
		fflush(stderr);
		return -1;
	}
	if (pid == 0) {
		/* Child — only exec or _exit allowed after vfork.
		 * Both pipe ends are inherited; exec keeps them. */
		execv(argv[0], argv);
		_exit(127);
	}

	/* Parent — close write end so the only writer is the child */
	close(fds[1]);

	/* Block until the child exits (EOF on the read end) */
	char buf;
	while (read(fds[0], &buf, 1) > 0)
		;
	close(fds[0]);

	return 0;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv, char **envp)
{
	(void)argc;
	(void)argv;

	/* Parse kernel addresses from env vars */
	if (hl_driver_init(envp, "hl_execdriver"))
		return 1;

	/* Register dispatch callback */
	hl_driver_run(exec_dispatch);
}
