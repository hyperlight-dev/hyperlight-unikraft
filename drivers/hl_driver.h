/*
 * hl_driver.h — shared boilerplate for every Hyperlight runtime driver.
 *
 * A driver's job is the same regardless of runtime: set up its runtime,
 * then serve named guest calls (`Exec`, `GuestExec`) with a dispatch
 * callback for as long as the guest lives.  Only the callback and any
 * runtime setup differ, so a driver's main() reduces to:
 *
 *     int main(void) {
 *         if (hl_driver_init("hl_foodriver"))
 *             return 1;
 *         ... runtime-specific init ...
 *         hl_driver_run(foo_dispatch);   // serves calls; never returns
 *     }
 *
 * How calls arrive: the kernel queues each named FunctionCall on
 * /dev/hlcall.  The driver blocks in read() on it; each read returns one
 * call, the callback runs it on this very thread (its own stack, its own
 * TLS), a non-zero callback result is reported back with a write() of the
 * status, and the next read() tells the kernel the call completed.  While
 * the driver is parked in read() the kernel hands the vCPU back to the
 * host, so a call that sleeps or waits on a socket costs no CPU, and the
 * guest can be snapshotted between any two steps -- in the middle of a
 * call included.  See plat/hyperlight/step.c in the kernel.
 *
 * The same device answers two ioctls: how large a call can get, and the
 * host's current environment (see hl_env.h for the refresh built on it).
 * The numbers and the struct are the kernel's ABI, defined in
 * plat/hyperlight/include/hyperlight-x86/step.h.
 *
 * Include hl_fc.h (for hl_dispatch_fn_t) transitively via this header.
 */

#ifndef HL_DRIVER_H
#define HL_DRIVER_H

#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include <unistd.h>

#include "hl_fc.h"  /* hl_dispatch_fn_t */

/* The kernel's named-call queue. */
#define HL_CALL_DEVICE "/dev/hlcall"

/* Its ioctls, _IOR/_IOWR-encoded like any device's: type letter 'H', a
 * sequence number, and the argument's direction and size in the number
 * itself, so a layout mismatch with the kernel reads as ENOTTY. */

/* The largest call a read() on the queue can return (a uint64_t).  The
 * kernel takes it from the host's PEB, so the driver never hard-codes a
 * call size. */
#define HLCALL_IOC_MAXLEN _IOR('H', 1, uint64_t)

/* The host's environment as it is now: KEY=VALUE entries separated by NUL
 * and terminated by one, `len` bytes without that terminator.  ENOBUFS if
 * it does not fit `cap`; HLCALL_IOC_MAXLEN bytes always do. */
struct hlcall_env {
	char *buf;	/* in: where to store the entries */
	uint64_t cap;	/* in: bytes available at buf */
	uint64_t len;	/* out: bytes stored, not counting the final NUL */
};

#define HLCALL_IOC_GETENV _IOWR('H', 2, struct hlcall_env)

static int g_hl_call_fd = -1;
static uint8_t *g_hl_call_buf;
static size_t g_hl_call_cap;

/*
 * Open the call queue and size the read buffer from it.  Returns 0 on
 * success, 1 on error (already reported on stderr).
 */
static inline int hl_driver_init(const char *name)
{
	uint64_t cap = 0;

	/* Read to receive calls, write to report how one went.  Close-on-exec:
	 * a program the driver runs must not hold the call queue. */
	g_hl_call_fd = open(HL_CALL_DEVICE, O_RDWR | O_CLOEXEC);
	if (g_hl_call_fd < 0) {
		fprintf(stderr, "%s: cannot open %s: %s (kernel without the step model?)\n",
			name, HL_CALL_DEVICE, strerror(errno));
		return 1;
	}
	if (ioctl(g_hl_call_fd, HLCALL_IOC_MAXLEN, &cap) < 0) {
		fprintf(stderr, "%s: cannot size the call buffer: %s\n",
			name, strerror(errno));
		return 1;
	}
	if (cap == 0) {
		fprintf(stderr, "%s: kernel reports no call queue\n", name);
		return 1;
	}
	/* Heap, not stack: a callback may recurse deeply (an interpreter). */
	g_hl_call_buf = malloc(cap);
	if (!g_hl_call_buf) {
		fprintf(stderr, "%s: cannot allocate a %llu-byte call buffer\n",
			name, (unsigned long long)cap);
		return 1;
	}
	g_hl_call_cap = (size_t)cap;
	return 0;
}

/* Write all of @buf to @fd, across short writes.  -1 on error. */
static inline int hl_write_all(int fd, const void *buf, size_t len)
{
	const char *p = buf;

	while (len > 0) {
		ssize_t n = write(fd, p, len);

		if (n < 0 && errno == EINTR)
			continue;
		if (n <= 0)
			return -1;
		p += n;
		len -= (size_t)n;
	}
	return 0;
}

/*
 * The call status for a child process that has ended: its exit code,
 * or 128 + the signal that killed it, the shell convention.  Reaps it.
 */
static inline int hl_wait_status(pid_t pid)
{
	int status;

	if (waitpid(pid, &status, 0) != pid) {
		fprintf(stderr, "hl_driver: waitpid(%d): %s\n",
			(int)pid, strerror(errno));
		return -1;
	}
	if (WIFEXITED(status))
		return WEXITSTATUS(status);
	if (WIFSIGNALED(status))
		return 128 + WTERMSIG(status);
	return -1;
}

/*
 * Serve named guest calls forever -- never returns.
 *
 * Why never exit?  exit_group would tear down the VFS fd table (closing
 * stdout/stderr) and run atexit handlers; the driver's heap and TLS must
 * stay intact for the calls to come.  Parking in read() keeps everything
 * alive while costing nothing.
 */
static inline __attribute__((noreturn)) void hl_driver_run(hl_dispatch_fn_t cb)
{
	uint8_t *fc = g_hl_call_buf;

	for (;;) {
		ssize_t n = read(g_hl_call_fd, fc, g_hl_call_cap);

		if (n < 0) {
			if (errno == EINTR)
				continue;
			fprintf(stderr, "hl_driver: %s: %s\n",
				HL_CALL_DEVICE, strerror(errno));
			fflush(stderr);
			_exit(1);
		}
		if (n == 0)
			continue;
		int32_t status = cb(fc, (size_t)n);

		if (status != 0) {
			/* Tell the kernel, and through it the host, that the
			 * call failed; the next read() still marks it done. */
			fprintf(stderr, "hl_driver: dispatch callback failed\n");
			fflush(stderr);
			if (write(g_hl_call_fd, &status, sizeof(status)) < 0)
				fprintf(stderr, "hl_driver: cannot report the failure: %s\n",
					strerror(errno));
		}
	}
}

#endif /* HL_DRIVER_H */
