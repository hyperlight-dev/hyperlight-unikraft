/*
 * hl_pydriver — Python runtime driver for Hyperlight.
 *
 * Loaded by app-elfloader during boot (evolve).  Initializes CPython,
 * then serves named calls from the kernel's call queue (/dev/hlcall).
 *
 * Flow:
 *   boot (evolve):
 *     main() → hl_driver_init(): open /dev/hlcall
 *            → Py_Initialize()
 *            → hl_driver_run(): block in read() on the call queue
 *            → kernel: scheduler idle → yield to host; evolve() returns
 *
 *   host: call("Exec", "print(42)")
 *     kernel pump queues the call, wakes this thread
 *              → hl_py_dispatch(fc, fc_len)   [shared, in hl_py.h]
 *              → PyRun_SimpleString("print(42)")
 *              → print() → write(1,...) → works (fds still open)
 *              → back into read(): call reported complete on the next
 *                yield; the host's run() returns
 *
 * The env bridge and dispatch body are shared with
 * hl_pywarmdriver via hl_py.h.
 */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../hl_fc.h"
#include "../hl_env.h"
#include "../hl_py.h"
#include "../hl_driver.h"

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	if (hl_driver_init("hl_pydriver"))
		return 1;

	/* The elfloader seeds PATH=/bin (CONFIG_LIBPOSIX_ENVIRON_ENVP0 in
	 * defconfig-elfloader), where no interpreter lives, so a bare
	 * subprocess.run(["python3", ...]) fails.  Replace it before
	 * Py_Initialize copies environ into os.environ; a host --env PATH is
	 * applied per call after this and still wins. */
	setenv("PATH", "/usr/local/bin:/usr/bin:/bin", 1);

	/* Initialize Python while VFS is fully alive — open(),
	 * read(), etc. all work for loading /usr/lib/python3.12/ */
	Py_UTF8Mode = 1;
	Py_Initialize();

	PyRun_SimpleString(
		"import sys\n"
		"sys.argv = ['hl_pydriver']\n");

	/* Serve named calls from the kernel's queue; never returns */
	hl_py_serve();
}
