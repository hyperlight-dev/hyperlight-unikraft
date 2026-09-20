/*
 * hl_pywarmdriver — pre-warming CPython driver for Hyperlight.
 *
 * Extended version of hl_pydriver that pre-warms heavy Python imports
 * (numpy, pandas, scipy, sklearn, matplotlib) at boot time, before
 * the VM halts.  Snapshot save captures this warmed state so that
 * snapshot restore skips all import overhead.
 *
 * This same binary is compiled into both the full agent rootfs (which
 * has all 26 pip packages) and the python-shell rootfs (which has none).
 * Imports are wrapped in try/except so the driver boots cleanly in
 * either rootfs — missing packages are silently skipped.
 *
 * The env bridge and dispatch body are shared with
 * hl_pydriver via hl_py.h; only the boot-time pre-warm below is specific
 * to this driver.
 *
 * Flow:
 *   boot (evolve):
 *     main() → hl_driver_init(): open /dev/hlcall
 *            → Py_Initialize()
 *            → pre-warm stdlib + data-science imports
 *            → hl_driver_run(): block in read() on the call queue
 *            → kernel: scheduler idle → yield to host; evolve() returns
 */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "hl_fc.h"
#include "hl_env.h"
#include "hl_py.h"
#include "hl_driver.h"

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	if (hl_driver_init("hl_pywarmdriver"))
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
		"sys.argv = ['hl_pywarmdriver']\n");

	/*
	 * Pre-warm imports.
	 *
	 * Import heavy packages here so they're in memory when the
	 * VM halts.  Snapshot save captures this warmed state, making
	 * snapshot restore skip all import overhead.
	 *
	 * Imports are wrapped in try/except so the driver works even
	 * if some packages aren't installed (e.g., python-shell rootfs).
	 *
	 * MPL_IGNORE_SYSTEM_FONTS: the unikernel has no system fonts
	 * (no /usr/share/fonts, no fc-list).  Without this flag,
	 * matplotlib.font_manager runs subprocess fc-list during
	 * import, which crashes the unikernel.  Matplotlib's bundled
	 * fonts (DejaVu, etc.) still work.
	 */
	PyRun_SimpleString(
		"import warnings\n"
		"warnings.filterwarnings('ignore')\n"
		"\n"
		"# Standard library\n"
		"import os, sys, json, csv, io, pathlib, math, random\n"
		"import datetime, re, hashlib, base64, struct\n"
		"import collections, itertools, functools, contextlib\n"
		"import typing, dataclasses, logging, subprocess\n"
		"import urllib.parse, concurrent.futures\n"
		"\n"
		"os.environ['MPL_IGNORE_SYSTEM_FONTS'] = '1'\n"
		"\n"
		"# Data science (heavy — biggest win from pre-warming)\n"
		"try:\n"
		"    import numpy\n"
		"except ImportError:\n"
		"    pass\n"
		"try:\n"
		"    import pandas\n"
		"except ImportError:\n"
		"    pass\n"
		"try:\n"
		"    import scipy\n"
		"except ImportError:\n"
		"    pass\n"
		"try:\n"
		"    import sklearn\n"
		"except ImportError:\n"
		"    pass\n"
		"try:\n"
		"    import matplotlib\n"
		"    matplotlib.use('Agg')\n"
		"except ImportError:\n"
		"    pass\n");

	PyRun_SimpleString("warnings.resetwarnings()\n");

	/* Serve named calls from the kernel's queue; never returns */
	hl_py_serve();
}
