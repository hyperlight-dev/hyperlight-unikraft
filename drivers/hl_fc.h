/*
 * hl_fc.h -- Postcard FunctionCall reader shared by runtime drivers.
 *
 * /dev/hlcall delivers the same control data and external byte stream as
 * Hyperlight's transport, without its header. The kernel validates UTF-8
 * and the control length before queueing a call. Fields follow
 * hyperlight-common's CallHead and WireValue serde definitions.
 */

#ifndef HL_FC_H
#define HL_FC_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>

struct fc_reader {
	const uint8_t *buf;
	size_t len, pos;
	int failed;
};

static inline const uint8_t *fc_take(struct fc_reader *r, size_t n)
{
	const uint8_t *p;

	if (r->failed || n > r->len - r->pos) {
		r->failed = 1;
		return NULL;
	}
	p = r->buf + r->pos;
	r->pos += n;
	return p;
}

static inline uint64_t fc_uint(struct fc_reader *r, unsigned int bits)
{
	uint64_t value = 0;
	unsigned int shift;
	const uint8_t *p;

	for (shift = 0; shift < bits; shift += 7) {
		p = fc_take(r, 1);
		if (!p)
			return 0;
		if (bits - shift < 7 && *p >= (1U << (bits - shift)))
			break;
		value |= (uint64_t)(*p & 0x7f) << shift;
		if (!(*p & 0x80))
			return value;
	}
	r->failed = 1;
	return 0;
}

static inline const char *fc_string(struct fc_reader *r, size_t *len)
{
	uint64_t n = fc_uint(r, 64);

	if (n > SIZE_MAX) {
		r->failed = 1;
		return NULL;
	}
	*len = (size_t)n;
	return (const char *)fc_take(r, *len);
}

/* Find a string parameter, or the name when index == SIZE_MAX. Validate
 * the whole call so truncated parameters and surplus bytes are rejected.
 */
static inline const char *fc_extract(const uint8_t *fc, size_t len,
				    size_t index, size_t *out_len)
{
	struct fc_reader r = { .buf = fc, .len = len, .pos = 0, .failed = 0 };
	size_t n = 0, found_len = 0, external = 0;
	const char *s, *found = NULL;
	const uint8_t *p;
	uint64_t count, i, tag, bytes;

	s = fc_string(&r, &n);
	if (index == SIZE_MAX) {
		found = s;
		found_len = n;
	}
	p = fc_take(&r, 1);
	if (!p || *p != 0 || fc_uint(&r, 32) > 10)
		return NULL;
	count = fc_uint(&r, 64);
	if (r.failed || count > len - r.pos)
		return NULL;
	for (i = 0; i < count; i++) {
		tag = fc_uint(&r, 32);
		switch (tag) {
		case 0: case 1:
			(void)fc_uint(&r, 32);
			break;
		case 2: case 3:
			(void)fc_uint(&r, 64);
			break;
		case 4: case 5:
			(void)fc_take(&r, tag == 4 ? 4 : 8);
			break;
		case 6:
			s = fc_string(&r, &n);
			if (i == index) {
				found = s;
				found_len = n;
			}
			break;
		case 7:
			p = fc_take(&r, 1);
			if (!p || *p > 1)
				return NULL;
			break;
		case 9:
			bytes = fc_uint(&r, 64);
			p = fc_take(&r, 1);
			if (!p || *p > 1 || bytes > len - external)
				return NULL;
			external += (size_t)bytes;
			break;
		default:
			return NULL;
		}
		if (r.failed)
			return NULL;
	}
	if (r.failed || external != len - r.pos)
		return NULL;
	*out_len = found_len;
	return found;
}

static inline const char *fc_arg_string(const uint8_t *fc, size_t len,
					size_t index, size_t *out_len)
{
	if (index == SIZE_MAX)
		return NULL;
	return fc_extract(fc, len, index, out_len);
}

static inline const char *fc_arg0_string(const uint8_t *fc, size_t len,
					 size_t *out_len)
{
	return fc_arg_string(fc, len, 0, out_len);
}

static inline const char *fc_function_name(const uint8_t *fc, size_t len,
					   size_t *out_len)
{
	return fc_extract(fc, len, SIZE_MAX, out_len);
}

static inline int fc_name_is(const uint8_t *fc, size_t len, const char *name)
{
	size_t n;
	const char *fn = fc_function_name(fc, len, &n);

	return fn && n == strlen(name) && memcmp(fn, name, n) == 0;
}

static inline int hl_split_ws(char *buf, char **argv, int max)
{
	int argc = 0;
	char *p = buf;

	while (*p && argc < max - 1) {
		while (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r')
			p++;
		if (!*p)
			break;
		argv[argc++] = p;
		while (*p && *p != ' ' && *p != '\t' && *p != '\n' && *p != '\r')
			p++;
		if (*p)
			*p++ = '\0';
	}
	argv[argc] = NULL;
	return argc;
}

typedef int (*hl_dispatch_fn_t)(const uint8_t *fc, size_t fc_len);

#endif /* HL_FC_H */
