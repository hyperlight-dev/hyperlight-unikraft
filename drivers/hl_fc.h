/*
 * hl_fc.h — Minimal FunctionCall FlatBuffer reader for Hyperlight drivers.
 *
 * Extracts the first string parameter from a Hyperlight FunctionCall
 * FlatBuffer.  Shared across all runtime drivers (Python, Node, .NET).
 *
 * Also provides the env-var parsing for the dispatch addresses
 * injected by the kernel's dispatch.c via uk_late_initcall.
 */

#ifndef HL_FC_H
#define HL_FC_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>

/* ── FlatBuffer primitives ─────────────────────────────────────── */

static inline uint32_t fb_u32(const uint8_t *b, size_t o)
{
	return b[o] | ((uint32_t)b[o+1] << 8) |
	       ((uint32_t)b[o+2] << 16) | ((uint32_t)b[o+3] << 24);
}

static inline uint16_t fb_u16(const uint8_t *b, size_t o)
{
	return b[o] | ((uint16_t)b[o+1] << 8);
}

static inline size_t fb_vtable(const uint8_t *b, size_t tbl)
{
	return tbl - (int32_t)fb_u32(b, tbl);
}

static inline uint16_t fb_field(const uint8_t *b, size_t tbl, uint16_t vt)
{
	size_t v = fb_vtable(b, tbl);
	return vt >= fb_u16(b, v) ? 0 : fb_u16(b, v + vt);
}

static inline size_t fb_follow(const uint8_t *b, size_t tbl, uint16_t vt)
{
	uint16_t f = fb_field(b, tbl, vt);
	if (!f)
		return 0;
	size_t p = tbl + f;
	return p + fb_u32(b, p);
}

/* ── FunctionCall string extraction ────────────────────────────── */

/*
 * Extract the first parameter as a string from a FunctionCall FlatBuffer.
 *
 * Returns a pointer into `fc` and sets *out_len to the string length,
 * or returns NULL if the first parameter isn't a string.
 *
 * The returned pointer is NOT NUL-terminated — the caller must copy
 * and terminate before passing to string APIs.
 */
static inline const char *fc_arg0_string(const uint8_t *fc, size_t fc_len,
					 size_t *out_len)
{
	if (fc_len < 8)
		return NULL;

	size_t root = 4 + fb_u32(fc, 4);

	/* FunctionCall.parameters (vtable offset 6) → vector of Parameter */
	size_t params = fb_follow(fc, root, 6);
	if (!params || fb_u32(fc, params) == 0)
		return NULL;

	/* First parameter */
	size_t p0_pos = params + 4;
	size_t p0 = p0_pos + fb_u32(fc, p0_pos);

	/* Parameter.value_type (vtable offset 4) must be 7 = hlstring */
	uint16_t tf = fb_field(fc, p0, 4);
	if (!tf || fc[p0 + tf] != 7)
		return NULL;

	/* Parameter.value (vtable offset 6) → hlstring table */
	size_t hs = fb_follow(fc, p0, 6);
	if (!hs)
		return NULL;

	/* hlstring.value (vtable offset 4) → string */
	size_t s = fb_follow(fc, hs, 4);
	if (!s || s + 4 > fc_len)
		return NULL;

	uint32_t slen = fb_u32(fc, s);
	if (s + 4 + slen > fc_len)
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
	if (fc_len < 8)
		return NULL;

	size_t root = 4 + fb_u32(fc, 4);

	/* FunctionCall.function_name (vtable offset 4) → string */
	size_t s = fb_follow(fc, root, 4);
	if (!s || s + 4 > fc_len)
		return NULL;

	uint32_t slen = fb_u32(fc, s);
	if (s + 4 + slen > fc_len)
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

/* ── Hex address parsing ───────────────────────────────────────── */

static inline uintptr_t hl_parse_hex(const char *s)
{
	uintptr_t v = 0;

	if (s[0] == '0' && (s[1] == 'x' || s[1] == 'X'))
		s += 2;
	for (; *s; s++) {
		unsigned d;
		if (*s >= '0' && *s <= '9')
			d = *s - '0';
		else if (*s >= 'a' && *s <= 'f')
			d = *s - 'a' + 10;
		else if (*s >= 'A' && *s <= 'F')
			d = *s - 'A' + 10;
		else
			break;
		v = (v << 4) | d;
	}
	return v;
}

/* ── Callback type ─────────────────────────────────────────────── */

typedef int (*hl_dispatch_fn_t)(const uint8_t *fc, size_t fc_len);

#endif /* HL_FC_H */
