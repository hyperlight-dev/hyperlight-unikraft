/*
 * hl_pwshdriver — PowerShell runtime driver for Hyperlight.
 *
 * On dispatch, receives PowerShell code, writes it to a temp file,
 * and runs it via vfork+execve of pwsh.  Each dispatch is a fresh
 * pwsh invocation (no persistent subprocess).
 *
 * Exit detection uses a pipe (same as the exec driver): the child
 * inherits the write end, and EOF signals the parent when it exits.
 *
 * Flow:
 *   boot (evolve):
 *     main() → register dispatch callback → halt
 *
 *   host: call("Exec", "Write-Host 'hello'")
 *     dispatch → pwsh_dispatch(fc, fc_len)
 *              → write code to /tmp/hl_dispatch.ps1
 *              → pipe() + vfork + execl("pwsh", "-File", ...)
 *              → read(pipe) blocks until child exits
 *              → return → halt
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

#include "../hl_fc.h"
#include "../hl_env.h"
#include "../hl_driver.h"

/* ── Dispatch callback ─────────────────────────────────────────── */

static int pwsh_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* Refresh glibc environ so the exec'd pwsh child inherits host vars. */
	hl_env_refresh(NULL, NULL);

	/* Extract the PS1 code from the FunctionCall FlatBuffer */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	/* Guest command (--guest-exec / autonomous): the command is written to a
	 * temp file and a fixed launcher reads it, splits it, and invokes the
	 * named guest script with its args; an empty command runs the conventional
	 * /entrypoint.ps1. Uses .NET methods rather than cmdlets (Test-Path/…)
	 * because module loading is limited in the rootfs. */
	size_t gx_len = code_len;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? code : NULL;
	static const char GX_LAUNCHER[] =
		"$c = [IO.File]::ReadAllText('/tmp/hl_gx').Trim()\n"
		"if ($c) { $a = $c -split '\\s+'; if ($a.Length -gt 1) { & $a[0] @($a[1..($a.Length-1)]) } else { & $a[0] } }\n"
		"elseif ([IO.File]::Exists('/entrypoint.ps1')) { & '/entrypoint.ps1' }\n"
		"else { [Console]::WriteLine('hl: no /entrypoint.ps1 in rootfs; nothing to run') }\n";
	if (gx) {
		FILE *gf = fopen("/tmp/hl_gx", "w");
		if (!gf) {
			fprintf(stderr, "hl_pwshdriver: cannot write guest cmd\n");
			fflush(stderr);
			return -1;
		}
		if (gx_len)
			fwrite(gx, 1, gx_len, gf);
		fclose(gf);
		code = GX_LAUNCHER;
		code_len = sizeof(GX_LAUNCHER) - 1;
	}

	/* Write code to temp file */
	FILE *f = fopen("/tmp/hl_dispatch.ps1", "w");
	if (!f) {
		fprintf(stderr, "hl_pwshdriver: cannot write dispatch file\n");
		fflush(stderr);
		return -1;
	}
	fwrite(code, 1, code_len, f);
	fputc('\n', f);
	fclose(f);

	/* Pipe for exit detection */
	int fds[2];
	if (pipe(fds) < 0) {
		fprintf(stderr, "hl_pwshdriver: pipe() failed\n");
		fflush(stderr);
		return -1;
	}

	pid_t pid = vfork();
	if (pid < 0) {
		close(fds[0]);
		close(fds[1]);
		fprintf(stderr, "hl_pwshdriver: vfork() failed\n");
		fflush(stderr);
		return -1;
	}
	if (pid == 0) {
		/* Child — only exec or _exit allowed after vfork */
		execl("/opt/microsoft/powershell/7/pwsh", "pwsh",
		      "-NoProfile", "-NonInteractive",
		      "-File", "/tmp/hl_dispatch.ps1",
		      (char *)NULL);
		_exit(127);
	}

	/* Parent — close write end, read until EOF */
	close(fds[1]);
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
	if (hl_driver_init(envp, "hl_pwshdriver"))
		return 1;

	/* .NET requires ICU for globalization; skip it in the unikernel */
	putenv("DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=true");

	/* Register dispatch callback */
	hl_driver_run(pwsh_dispatch);
}
