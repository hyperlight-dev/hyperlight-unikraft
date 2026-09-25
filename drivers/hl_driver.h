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
 * call (Exec, GuestExec, and Call if the driver said it serves them with
 * hl_driver_serve_calls()), the callback runs it on this very thread (its
 * own stack, its own TLS), a non-zero status and any result it set with
 * hl_set_result() are reported back with a write(), and the next read()
 * tells the kernel the call completed.  hl_host_call() calls a function
 * the embedder registered with the host.  While
 * the driver is parked in read() the kernel hands the vCPU back to the
 * host, so a call that sleeps or waits on a socket costs no CPU, and the
 * guest can be snapshotted between any two steps -- in the middle of a
 * call included.  See plat/hyperlight/step.c in the kernel.
 *
 * The same device answers three ioctls: how large a call can get, the
 * host's current environment (see hl_env.h for the refresh built on it),
 * and a call to one of the embedder's functions.
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

/* Call a function the embedder registered with the host (the host
 * function HostCall, which dispatches by name): the arguments and the
 * reply pass through the kernel as they are. */
struct hlcall_hostcall {
	const char *name;	/* in: the function's name */
	uint64_t name_len;	/* in */
	const uint8_t *args;	/* in: its arguments */
	uint64_t args_len;	/* in */
	uint8_t *out;		/* in: where to store the reply */
	uint64_t out_cap;	/* in: bytes available at out */
	uint64_t out_len;	/* out: bytes stored */
};

#define HLCALL_IOC_HOSTCALL _IOWR('H', 3, struct hlcall_hostcall)

/* The host library's HostCall reply: a tag byte, then the result or the
 * error message (src/lib.rs, dispatch_host_call). */
#define HL_HOST_CALL_OK 0
#define HL_HOST_CALL_ERR 1

static int g_hl_call_fd = -1;
static uint8_t *g_hl_call_buf;
static size_t g_hl_call_cap;

/* The result of the call being served (hl_set_result), sent with its
 * status when the callback returns; a call-sized buffer, status first.
 * Allocated on first use, like the reply buffer: only drivers that serve
 * calls or call the host need them. */
static uint8_t *g_hl_result_buf;
static size_t g_hl_result_len;
static int g_hl_has_result;

/* Host call replies land here: call-sized, the most the host sends. */
static uint8_t *g_hl_reply_buf;

/* Whether the callback serves `Call` (hl_driver_serve_calls). */
static int g_hl_serves_calls;

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

/*
 * Declare that the callback serves `Call` (a function the guest defined,
 * with an input, for its result).  Without it hl_driver_run fails such a
 * call itself: a callback written for Exec and GuestExec would take the
 * function's name for code, or for a program to run.
 */
static inline void hl_driver_serve_calls(void)
{
	g_hl_serves_calls = 1;
}

/*
 * Set what the call being served returns: sent to the host with the
 * call's status once the callback returns, and read there as the call's
 * result.  -1 if it exceeds the buffer (a call's size, HLCALL_IOC_MAXLEN)
 * or the buffer cannot be allocated;
 * the kernel takes at most what a host call carries (64 KiB by default),
 * and refuses a larger one, which hl_driver_run then reports as a failed
 * call.
 */
static inline int hl_set_result(const void *buf, size_t len)
{
	if (len > g_hl_call_cap)
		return -1;
	if (!g_hl_result_buf) {
		g_hl_result_buf = malloc(sizeof(int32_t) + g_hl_call_cap);
		if (!g_hl_result_buf)
			return -1;
	}
	memcpy(g_hl_result_buf + sizeof(int32_t), buf, len);
	g_hl_result_len = len;
	g_hl_has_result = 1;
	return 0;
}

/*
 * Call the host function the embedder registered as `name` with `args`
 * (the drivers pass JSON), during a call.  Returns 0 with the result, 1
 * with the host's error message, or -1 if the call could not be made
 * (errno set: EPERM outside a call, EIO when the host has no HostCall,
 * ENOMEM when the reply buffer cannot be allocated, EILSEQ from the
 * kernel when `name` is not UTF-8, which the host takes as a string).
 * *out points into a buffer the next host call reuses, NUL-terminated.
 * The empty name lists the registered functions, one per line.  Call it
 * from the thread serving the call: while that thread waits in read()
 * for the next one, the device is locked, and another thread's host call
 * would wait with it, to run under a later call.
 */
static inline int hl_host_call(const char *name, size_t name_len,
			       const void *args, size_t args_len,
			       const char **out, size_t *out_len)
{
	struct hlcall_hostcall hc;

	if (!g_hl_reply_buf) {
		g_hl_reply_buf = malloc(g_hl_call_cap + 1);
		if (!g_hl_reply_buf) {
			errno = ENOMEM;
			return -1;
		}
	}
	hc = (struct hlcall_hostcall){
		.name = name,
		.name_len = name_len,
		.args = args,
		.args_len = args_len,
		.out = g_hl_reply_buf,
		.out_cap = g_hl_call_cap,
	};
	if (ioctl(g_hl_call_fd, HLCALL_IOC_HOSTCALL, &hc) < 0)
		return -1;
	if (hc.out_len < 1) {
		errno = EPROTO;
		return -1;
	}
	g_hl_reply_buf[hc.out_len] = '\0';
	*out = (const char *)g_hl_reply_buf + 1;
	*out_len = (size_t)hc.out_len - 1;
	return g_hl_reply_buf[0] == HL_HOST_CALL_OK ? 0 : 1;
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
		g_hl_has_result = 0;
		int32_t status;
		int ran = 0;

		/* Only the calls the callback serves reach it: Exec and
		 * GuestExec, and Call when it said so.  Anything else fails
		 * here, rather than run as the kind it is not. */
		if (fc_name_is(fc, (size_t)n, "Exec") ||
		    fc_name_is(fc, (size_t)n, "GuestExec") ||
		    (g_hl_serves_calls && fc_name_is(fc, (size_t)n, "Call"))) {
			status = cb(fc, (size_t)n);
			ran = 1;
		} else {
			size_t nl = 0;
			const char *nm = fc_function_name(fc, (size_t)n, &nl);

			if (!nm) {
				nm = "?";
				nl = 1;
			}
			fprintf(stderr, "hl_driver: this runtime does not serve %.*s calls\n",
				(int)nl, nm);
			status = -1;
		}

		if (status != 0 && ran) {
			fprintf(stderr, "hl_driver: dispatch callback failed\n");
			fflush(stderr);
		}
		/* Tell the kernel, and through it the host, how the call went
		 * and what it returned; the next read() marks it done.  A
		 * failed call reports its status alone, whatever the callback
		 * set: its result is not the call's.  A success with no result
		 * needs no write. */
		if (status != 0 &&
		    write(g_hl_call_fd, &status, sizeof(status)) < 0)
			fprintf(stderr, "hl_driver: cannot report the failure: %s\n",
				strerror(errno));
		if (status == 0 && g_hl_has_result) {
			size_t len = sizeof(status) + g_hl_result_len;

			memcpy(g_hl_result_buf, &status, sizeof(status));
			if (write(g_hl_call_fd, g_hl_result_buf, len) < 0) {
				fprintf(stderr, "hl_driver: cannot report the call: %s\n",
					strerror(errno));
				/* A result the kernel refused (too large to send)
				 * must not pass for no result: fail the call. */
				status = -1;
				if (write(g_hl_call_fd, &status, sizeof(status)) < 0)
					fprintf(stderr, "hl_driver: cannot report the failure: %s\n",
						strerror(errno));
			}
		}
	}
}

#endif /* HL_DRIVER_H */
