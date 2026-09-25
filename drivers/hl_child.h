/*
 * hl_child.h — the pipe protocol between a driver and a runtime it runs
 * as a child process (node, dotnet-jit).
 *
 * The child owns the runtime; the driver owns /dev/hlcall.  Each call
 * the driver serves goes down the code pipe as one message:
 *
 *   'E' (code) or 'C' (a guest function call), a u64 length, then
 *   fields, each a u64 length and its bytes: the environment (in
 *   whatever form the child applies it), the code or the function's
 *   name, and for 'C' the input.
 *
 * and the child answers up the status pipe with, in any number:
 *
 *   'H', the name and arguments of a host function call, each a field:
 *   the driver makes it and writes the reply down the code pipe, a tag
 *   (0 the result, 1 the host's error, 2 the call could not be made)
 *   and a field;
 *
 * then once:
 *
 *   'S', a status byte, and the result as a field (empty for none).
 *
 * Lengths are little-endian.  Include hl_driver.h first.
 */

#ifndef HL_CHILD_H
#define HL_CHILD_H

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* Read exactly @len bytes; -1 if the child is gone. */
static int hl_child_read_all(int fd, void *buf, size_t len)
{
	char *p = buf;

	while (len > 0) {
		ssize_t n = read(fd, p, len);

		if (n < 0 && errno == EINTR)
			continue;
		if (n <= 0)
			return -1;
		p += n;
		len -= (size_t)n;
	}
	return 0;
}

/* Read and drop @len bytes; -1 if the child is gone. */
static int hl_child_drain(int fd, uint64_t len)
{
	char sink[4096];

	while (len > 0) {
		size_t n = len < sizeof(sink) ? (size_t)len : sizeof(sink);

		if (hl_child_read_all(fd, sink, n) < 0)
			return -1;
		len -= n;
	}
	return 0;
}

/* A field into *buf, a new NUL-terminated buffer; *buf stays NULL, the
 * bytes read and dropped, if they exceed a call.  -1 if the child is gone
 * or the buffer cannot be allocated. */
static int hl_child_read_field(int fd, uint64_t *len, char **buf)
{
	*buf = NULL;
	if (hl_child_read_all(fd, len, sizeof(*len)) < 0)
		return -1;
	if (*len > g_hl_call_cap)
		return hl_child_drain(fd, *len);
	*buf = malloc(*len + 1);
	if (!*buf || hl_child_read_all(fd, *buf, *len) < 0) {
		free(*buf);
		*buf = NULL;
		return -1;
	}
	(*buf)[*len] = '\0';
	return 0;
}

static int hl_child_write_field(int fd, const void *buf, uint64_t len)
{
	if (hl_write_all(fd, &len, sizeof(len)) < 0)
		return -1;
	return hl_write_all(fd, buf, len);
}

/* Send one call: 'E' with the code, or 'C' with the function's name and
 * the input.  -1 if the child is gone. */
static int hl_child_send(int fd, int is_call, const void *env, size_t env_len,
			 const void *code, size_t code_len, const void *input,
			 size_t input_len)
{
	unsigned char kind = is_call ? 'C' : 'E';
	uint64_t len = 8 + env_len + 8 + code_len + (is_call ? 8 + input_len : 0);

	if (hl_write_all(fd, &kind, 1) < 0 || hl_write_all(fd, &len, sizeof(len)) < 0 ||
	    hl_child_write_field(fd, env, env_len) < 0 ||
	    hl_child_write_field(fd, code, code_len) < 0)
		return -1;
	return is_call ? hl_child_write_field(fd, input, input_len) : 0;
}

/* Make the child's host function call and send the reply.  -1 if the
 * child is gone. */
static int hl_child_host_call(int to, int from)
{
	uint64_t name_len, args_len;
	char *name = NULL, *args = NULL;
	const char *out;
	size_t out_len;
	unsigned char tag;
	int r, rc = -1;

	if (hl_child_read_field(from, &name_len, &name) < 0 ||
	    hl_child_read_field(from, &args_len, &args) < 0)
		goto out;
	if (name && args) {
		r = hl_host_call(name, name_len, args, args_len, &out, &out_len);
	} else {
		/* More than a call carries, so more than a host call can. */
		errno = E2BIG;
		r = -1;
	}
	switch (r) {
	case 0:
		tag = 0;
		break;
	case 1:
		tag = 1;
		break;
	default:
		tag = 2;
		out = strerror(errno);
		out_len = strlen(out);
		break;
	}
	if (hl_write_all(to, &tag, 1) == 0 && hl_child_write_field(to, out, out_len) == 0)
		rc = 0;
out:
	free(name);
	free(args);
	return rc;
}

/* Wait for the call's end, making the host function calls it asks for on
 * the way: its status, with its result set (hl_set_result); -1 if the
 * child is gone.  @who names the driver in messages. */
static int hl_child_await(int to, int from, const char *who)
{
	for (;;) {
		unsigned char kind, status;
		uint64_t len;
		char *result;

		if (hl_child_read_all(from, &kind, 1) < 0)
			return -1;
		if (kind == 'H') {
			if (hl_child_host_call(to, from) < 0)
				return -1;
			continue;
		}
		if (kind != 'S' || hl_child_read_all(from, &status, 1) < 0 ||
		    hl_child_read_field(from, &len, &result) < 0)
			return -1;
		if (!result) {
			fprintf(stderr, "%s: the result does not fit a call\n", who);
			return 1;
		}
		if (len && hl_set_result(result, len) < 0) {
			fprintf(stderr, "%s: the result does not fit a call\n", who);
			status = 1;
		}
		free(result);
		return status;
	}
}

#endif /* HL_CHILD_H */
