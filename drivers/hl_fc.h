/*
 * hl_fc.h — Minimal FunctionCall FlatBuffer reader for Hyperlight drivers.
 *
 * Extracts the function name and the first string parameter from a
 * Hyperlight FunctionCall FlatBuffer, as the kernel hands it to the
 * driver through /dev/hlcall.  Shared across all runtime drivers.
 *
 * The vtable offsets and the union discriminant below follow the field
 * order of hyperlight's schema (src/schema/function_call.fbs and
 * function_types.fbs in the hyperlight repository): FunctionCall is
 * function_name, parameters; Parameter is the value union, whose
 * seventh member is hlstring; hlstring is value.  The buffer is
 * size-prefixed, hence the root offset read at byte 4.
 *
 * Every offset is checked against the buffer's length as it is
 * followed: a malformed call makes the extractors return NULL, which
 * fails the call, rather than read outside the buffer.
 */

#ifndef HL_FC_H
#define HL_FC_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>

/* ── FlatBuffer primitives ─────────────────────────────────────── */

/* An offset that would leave the buffer; every later step passes it
 * through, so a check at the start of a chain covers the whole chain. */
#define FB_BAD ((size_t)-1)

/* Whether [o, o + n) lies within a buffer of `len` bytes. */
static inline int fb_in(size_t len, size_t o, size_t n)
{
	return o != FB_BAD && o <= len && n <= len - o;
}

static inline int fb_get_u32(const uint8_t *b, size_t len, size_t o, uint32_t *out)
{
	if (!fb_in(len, o, 4))
		return 0;
	*out = b[o] | ((uint32_t)b[o+1] << 8) |
	       ((uint32_t)b[o+2] << 16) | ((uint32_t)b[o+3] << 24);
	return 1;
}

static inline int fb_get_u16(const uint8_t *b, size_t len, size_t o, uint16_t *out)
{
	if (!fb_in(len, o, 2))
		return 0;
	*out = b[o] | ((uint16_t)b[o+1] << 8);
	return 1;
}

/* The root table of a size-prefixed buffer: the uoffset at byte 4. */
static inline size_t fb_root(const uint8_t *b, size_t len)
{
	uint32_t off;

	if (!fb_get_u32(b, len, 4, &off) || !fb_in(len, 4 + (size_t)off, 4))
		return FB_BAD;
	return 4 + (size_t)off;
}

/* A table's vtable: the table starts with a signed offset back to it. */
static inline size_t fb_vtable(const uint8_t *b, size_t len, size_t tbl)
{
	uint32_t raw;
	int64_t v;

	if (!fb_get_u32(b, len, tbl, &raw))
		return FB_BAD;
	v = (int64_t)tbl - (int32_t)raw;
	if (v < 0 || !fb_in(len, (size_t)v, 4))
		return FB_BAD;
	return (size_t)v;
}

/* The offset of field `vt` within its table: 0 if the field is absent
 * (a vtable shorter than `vt`, or a zero entry), FB_BAD if malformed. */
static inline size_t fb_field(const uint8_t *b, size_t len, size_t tbl, uint16_t vt)
{
	size_t v = fb_vtable(b, len, tbl);
	uint16_t vsize, f;

	if (v == FB_BAD || !fb_get_u16(b, len, v, &vsize))
		return FB_BAD;
	if (vt >= vsize)
		return 0;
	if (!fb_get_u16(b, len, v + vt, &f))
		return FB_BAD;
	return f;
}

/* Follow an offset field to the table, vector or string it points at:
 * 0 if the field is absent, FB_BAD if malformed. */
static inline size_t fb_follow(const uint8_t *b, size_t len, size_t tbl, uint16_t vt)
{
	size_t f = fb_field(b, len, tbl, vt);
	size_t p, target;
	uint32_t off;

	if (f == 0 || f == FB_BAD)
		return f;
	p = tbl + f;
	if (!fb_get_u32(b, len, p, &off))
		return FB_BAD;
	target = p + (size_t)off;
	return fb_in(len, target, 4) ? target : FB_BAD;
}

/* ── FunctionCall string extraction ────────────────────────────── */

/*
 * Extract the first parameter as a string from a FunctionCall FlatBuffer.
 *
 * Returns a pointer into `fc` and sets *out_len to the string length,
 * or returns NULL if the first parameter isn't a string, or the buffer
 * is malformed.
 *
 * The returned pointer is NOT NUL-terminated — the caller must copy
 * and terminate before passing to string APIs.
 */
static inline const char *fc_arg0_string(const uint8_t *fc, size_t fc_len,
					 size_t *out_len)
{
	size_t root, params, p0_pos, p0, tf, hs, s;
	uint32_t count, off, slen;

	root = fb_root(fc, fc_len);
	if (root == FB_BAD)
		return NULL;

	/* FunctionCall.parameters (vtable offset 6) → vector of Parameter */
	params = fb_follow(fc, fc_len, root, 6);
	if (!params || params == FB_BAD ||
	    !fb_get_u32(fc, fc_len, params, &count) || count == 0)
		return NULL;

	/* First parameter: the vector's first uoffset */
	p0_pos = params + 4;
	if (!fb_get_u32(fc, fc_len, p0_pos, &off))
		return NULL;
	p0 = p0_pos + (size_t)off;

	/* Parameter.value_type (vtable offset 4) must be 7 = hlstring */
	tf = fb_field(fc, fc_len, p0, 4);
	if (!tf || tf == FB_BAD || !fb_in(fc_len, p0 + tf, 1) || fc[p0 + tf] != 7)
		return NULL;

	/* Parameter.value (vtable offset 6) → hlstring table */
	hs = fb_follow(fc, fc_len, p0, 6);
	if (!hs || hs == FB_BAD)
		return NULL;

	/* hlstring.value (vtable offset 4) → string: a length, then the bytes */
	s = fb_follow(fc, fc_len, hs, 4);
	if (!s || s == FB_BAD || !fb_get_u32(fc, fc_len, s, &slen) ||
	    !fb_in(fc_len, s + 4, slen))
		return NULL;

	*out_len = slen;
	return (const char *)(fc + s + 4);
}

/* ── FunctionCall name ─────────────────────────────────────────── */

/*
 * Extract the FunctionCall's function name.  Returns a pointer into
 * `fc` (NOT NUL-terminated) and sets *out_len, or NULL on parse error.
 * Used to distinguish a guest command ("GuestExec") from inline code
 * ("Exec") without an in-band payload marker.
 */
static inline const char *fc_function_name(const uint8_t *fc, size_t fc_len,
					   size_t *out_len)
{
	size_t root, s;
	uint32_t slen;

	root = fb_root(fc, fc_len);
	if (root == FB_BAD)
		return NULL;

	/* FunctionCall.function_name (vtable offset 4) → string */
	s = fb_follow(fc, fc_len, root, 4);
	if (!s || s == FB_BAD || !fb_get_u32(fc, fc_len, s, &slen) ||
	    !fb_in(fc_len, s + 4, slen))
		return NULL;

	*out_len = slen;
	return (const char *)(fc + s + 4);
}

/* True if the FunctionCall's name equals the NUL-terminated `name`. */
static inline int fc_name_is(const uint8_t *fc, size_t fc_len, const char *name)
{
	size_t n;
	const char *fn = fc_function_name(fc, fc_len, &n);
	return fn && n == strlen(name) && memcmp(fn, name, n) == 0;
}

/* ── Guest-exec argv split ─────────────────────────────────────── */

/*
 * Split a NUL-terminated string in place on whitespace into `argv`
 * (NULL-terminated, capacity `max`).  Returns argc.  Naive whitespace
 * split — matches urunc's strings.Fields on the cmdline.
 */
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

/* ── Callback type ─────────────────────────────────────────────── */

typedef int (*hl_dispatch_fn_t)(const uint8_t *fc, size_t fc_len);

#endif /* HL_FC_H */
