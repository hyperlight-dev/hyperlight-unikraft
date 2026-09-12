/*
 * hl_pydriver — Python runtime driver for Hyperlight.
 *
 * Loaded by app-elfloader during boot (evolve).  Initializes CPython,
 * registers a dispatch callback, then halts the VM directly
 *
 * Flow:
 *   boot (evolve):
 *     main() → parse env vars for kernel addresses
 *            → Py_Initialize()
 *            → *callback_slot = hl_py_dispatch
 *            → outl port 108 (halt VM, RAX = dispatch entry)
 *            → host: evolve() returns
 *
 *   host: call("Exec", "print(42)")
 *     dispatch → hyperlight_dispatch_function (kernel)
 *              → hl_py_dispatch(fc, fc_len)   [shared, in hl_py.h]
 *              → restore FS_BASE
 *              → PyRun_SimpleString("print(42)")
 *              → print() → write(1,...) → works (fds still open)
 *              → halt
 *
 * The FS_BASE save/restore, env bridge and dispatch body are shared with
 * hl_pywarmdriver via hl_py.h.
 *
 * TODO: HL_DISPATCH_CALLBACK_PTR / HL_DISPATCH_ENTRY are raw kernel
 * addresses injected as env vars.  Replace with a cleaner interface
 * (vDSO export, syscall, or device ioctl).
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

int main(int argc, char **argv, char **envp)
{
	(void)argc;
	(void)argv;

	/* Parse kernel addresses from env vars injected by
	 * dispatch.c's uk_late_initcall. */
	if (hl_driver_init(envp, "hl_pydriver"))
		return 1;

	/* Initialize Python while VFS is fully alive — open(),
	 * read(), etc. all work for loading /usr/lib/python3.12/ */
	Py_UTF8Mode = 1;
	Py_Initialize();

	PyRun_SimpleString(
		"import sys\n"
		"sys.argv = ['hl_pydriver']\n");

	/* Save FS_BASE after Python init — the host may clobber it
	 * on dispatch (different thread context or snapshot restore). */
	g_py_fsbase = rd_fsbase();

	/* Register dispatch callback */
	hl_driver_run(hl_py_dispatch);
}
