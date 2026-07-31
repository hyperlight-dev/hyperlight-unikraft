/*
 * multifn-c — a minimal C guest that exercises Hyperlight's
 * multi-function dispatch from the Unikraft-hosted side.
 *
 * The host is expected to call:
 *     call("init", ())            -> we print "INIT" and mark initialized
 *     call("run",  <string arg>)  -> we print "RUN: <arg>" (1 string arg)
 *
 * main() runs once, installs a dispatch callback and serves the call that
 * started it; every later call reaches that callback directly. The kernel's
 * .data persists between calls via snapshot/restore, so a `static` flag is
 * enough to remember that init already ran. The kernel pops the input stack
 * before the callback runs but leaves the bytes in shared memory — our
 * current-FC pointer slots live in kernel .data and are updated per call.
 *
 * Inputs main() reads at startup:
 *   envp HL_FC_BYTES_PTR=0x...      address of a (const uint8_t *) slot
 *   envp HL_FC_LEN_PTR=0x...        address of a (size_t) slot
 *   envp HL_V2_CALLBACK_PTR=0x...   address of the dispatch callback slot
 * Both addresses are stable across the whole VM lifetime; the values
 * at those addresses change per call.
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

/* Minimal FlatBuffer reader for the Hyperlight FunctionCall shape.
 * Copy of the helpers in plat/hyperlight/include/hyperlight-x86/fb.h
 * duplicated here so we don't need to share headers with the kernel.
 */
static inline uint32_t fb_u32(const uint8_t *b, size_t o)
{
	return b[o] | ((uint32_t)b[o+1] << 8) |
	       ((uint32_t)b[o+2] << 16) | ((uint32_t)b[o+3] << 24);
}
static inline uint16_t fb_u16(const uint8_t *b, size_t o)
{
	return b[o] | ((uint16_t)b[o+1] << 8);
}
static inline int32_t fb_i32(const uint8_t *b, size_t o)
{
	return (int32_t)fb_u32(b, o);
}
static inline size_t fb_vtable(const uint8_t *b, size_t tbl)
{
	return tbl - fb_i32(b, tbl);
}
static inline uint16_t fb_field(const uint8_t *b, size_t tbl, uint16_t vt)
{
	size_t v = fb_vtable(b, tbl);
	uint16_t vs = fb_u16(b, v);
	return vt >= vs ? 0 : fb_u16(b, v + vt);
}
static inline size_t fb_follow(const uint8_t *b, size_t tbl, uint16_t vt)
{
	uint16_t f = fb_field(b, tbl, vt);
	if (!f) return 0;
	size_t p = tbl + f;
	return p + fb_u32(b, p);
}

/* Pull the `function_name` field out of a size-prefixed FunctionCall. */
static int parse_fc_name(const uint8_t *b, size_t len,
			 const char **out, size_t *out_len)
{
	if (len < 8) return -1;
	size_t fc = 4 + fb_u32(b, 4);
	if (fc >= len) return -1;
	size_t p = fb_follow(b, fc, 4);
	if (!p || p + 4 > len) return -1;
	uint32_t nlen = fb_u32(b, p);
	if (p + 4 + nlen > len) return -1;
	*out = (const char *)(b + p + 4);
	*out_len = nlen;
	return 0;
}

/* Pull the first parameter out of a FunctionCall as a hlstring. Returns
 * a pointer into `b` (NOT NUL-terminated) or NULL if the arg is absent
 * or isn't an hlstring. value_type=7 is hlstring in the ParameterValue
 * union (see src/schema/function_types.fbs).
 */
static const char *parse_fc_arg0_string(const uint8_t *b, size_t len,
					size_t *out_len)
{
	if (len < 8) return NULL;
	size_t fc = 4 + fb_u32(b, 4);
	/* parameters vector is at VT[6] on FunctionCall. */
	size_t params = fb_follow(b, fc, 6);
	if (!params) return NULL;
	uint32_t plen = fb_u32(b, params);
	if (plen == 0) return NULL;
	/* First element offset, 4 bytes after the vector length. */
	size_t p0_pos = params + 4;
	size_t p0 = p0_pos + fb_u32(b, p0_pos);
	/* Parameter.value_type is a u8 inline field at VT[4]. */
	uint16_t tf = fb_field(b, p0, 4);
	if (!tf) return NULL;
	uint8_t vt = b[p0 + tf];
	if (vt != 7) return NULL; /* not hlstring */
	/* Parameter.value is at VT[6]: follow to the hlstring table. */
	size_t hs = fb_follow(b, p0, 6);
	if (!hs) return NULL;
	/* hlstring.value at VT[4]: follow to the string data. */
	size_t s = fb_follow(b, hs, 4);
	if (!s || s + 4 > len) return NULL;
	uint32_t slen = fb_u32(b, s);
	if (s + 4 + slen > len) return NULL;
	*out_len = slen;
	return (const char *)(b + s + 4);
}

static int already_initialized = 0;

static void handle_call(const uint8_t *fc, size_t fc_len)
{
	const char *name = NULL;
	size_t name_len = 0;

	if (parse_fc_name(fc, fc_len, &name, &name_len) < 0) {
		fprintf(stderr, "multifn-c: FC parse failed\n");
		return;
	}

	if (name_len == 4 && !memcmp(name, "init", 4)) {
		already_initialized = 1;
		printf("INIT\n");
	} else if (name_len == 3 && !memcmp(name, "run", 3)) {
		if (!already_initialized)
			printf("RUN (uninitialized!)\n");
		size_t arg_len = 0;
		const char *arg = parse_fc_arg0_string(fc, fc_len, &arg_len);
		if (arg)
			printf("RUN: %.*s\n", (int)arg_len, arg);
		else
			printf("RUN: <no arg>\n");
	} else {
		printf("UNKNOWN: %.*s\n", (int)name_len, name);
	}

	fflush(stdout);
}

typedef void (*hl_dispatch_fn_t)(const uint8_t *fc, size_t fc_len);

int main(int argc, char **argv, char **envp)
{
	const uint8_t **fc_bytes_slot = NULL;
	size_t *fc_len_slot = NULL;
	hl_dispatch_fn_t *v2_callback_slot = NULL;

	(void)argc;
	(void)argv;

	for (char **p = envp; p && *p; p++) {
		if (!strncmp(*p, "HL_FC_BYTES_PTR=", 16)) {
			unsigned long v = strtoul(*p + 16, NULL, 16);
			fc_bytes_slot = (const uint8_t **)(uintptr_t)v;
		} else if (!strncmp(*p, "HL_FC_LEN_PTR=", 14)) {
			unsigned long v = strtoul(*p + 14, NULL, 16);
			fc_len_slot = (size_t *)(uintptr_t)v;
		} else if (!strncmp(*p, "HL_V2_CALLBACK_PTR=", 19)) {
			unsigned long v = strtoul(*p + 19, NULL, 16);
			v2_callback_slot = (hl_dispatch_fn_t *)(uintptr_t)v;
		}
	}
	if (!fc_bytes_slot || !fc_len_slot || !v2_callback_slot) {
		fprintf(stderr, "multifn-c: env vars missing\n");
		return 1;
	}

	/* Serve every later call from here instead of re-running main(). */
	*v2_callback_slot = handle_call;

	const uint8_t *fc = *fc_bytes_slot;
	size_t fc_len = *fc_len_slot;
	if (!fc || fc_len == 0) {
		fprintf(stderr, "multifn-c: no current FC bytes\n");
		return 1;
	}
	handle_call(fc, fc_len);

	/* Halt, leaving the callback and already_initialized in guest memory
	 * for whatever snapshot the host takes next.
	 */
	return 0;
}
