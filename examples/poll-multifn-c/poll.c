#include <fcntl.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

/*
 * Cooperative-poll multi-function app.
 *
 * Models the persistent-runtime shape that examples/python-agent-driver uses,
 * but in a few hundred bytes instead of a Python interpreter: main() runs once
 * to do the expensive setup, installs an FC-aware dispatch callback, serves the
 * call that started it, and halts. Every later host call reaches that callback
 * directly, so the setup is paid for once.
 *
 * The point of the example is what happens under CONFIG_HYPERLIGHT_POLL. The
 * named call does not run on the thread that returns control to the host — the
 * kernel routes it to a scheduler thread (plat/hyperlight/poll.c) — so the
 * handler is free to block. It proves that by sleeping in the middle of the
 * call: the guest parks, the vCPU goes back to the host, and the call resumes
 * on a later poll before returning its result.
 *
 * Protocol with the driving test (host/tests/poll_multifn.rs):
 *   - each call takes one decimal integer and adds it to a running total that
 *     lives in guest memory, so snapshot/restore behaviour is observable;
 *   - the new total is reported with a `report` host function call;
 *   - a malformed call reports a distinct `error` code instead, so a
 *     regression fails loudly rather than looking like a smaller sum.
 */

#define ERR_NO_ARG	101
#define ERR_ARG_TOO_LONG 102

/* Nanoseconds the handler parks for mid-call. Long enough that the host
 * observes at least one cooperative yield, short enough to keep the test fast.
 */
#define HANDLER_PARK_US 20000

/* -- FlatBuffers reader for the incoming FunctionCall ------------------
 * Only the first string parameter is needed. Hand-rolled so the example
 * stays a single self-contained file (mirrors examples/multifn-c).
 */
static uint32_t fb_u32(const uint8_t *b, size_t o)
{
	return b[o] | ((uint32_t)b[o + 1] << 8) | ((uint32_t)b[o + 2] << 16) |
	       ((uint32_t)b[o + 3] << 24);
}

static uint16_t fb_u16(const uint8_t *b, size_t o)
{
	return b[o] | ((uint16_t)b[o + 1] << 8);
}

static uint16_t fb_field(const uint8_t *b, size_t tbl, uint16_t vt_off)
{
	size_t vt = tbl - (int32_t)fb_u32(b, tbl);

	if (vt_off >= fb_u16(b, vt))
		return 0;
	return fb_u16(b, vt + vt_off);
}

static size_t fb_follow(const uint8_t *b, size_t tbl, uint16_t vt_off)
{
	uint16_t f = fb_field(b, tbl, vt_off);
	size_t p;

	if (!f)
		return 0;
	p = tbl + f;
	return p + fb_u32(b, p);
}

/* First parameter of a size-prefixed FunctionCall, as an hlstring
 * (ParameterValue union discriminant 7). Returns a pointer into `b`, which is
 * NOT NUL-terminated, or NULL if the argument is absent or another type.
 */
static const char *fc_arg0_string(const uint8_t *b, size_t len, size_t *out_len)
{
	size_t fc, params, p0, val, str;
	uint16_t tf;
	uint32_t slen;

	if (!b || len < 8)
		return NULL;

	fc = 4 + fb_u32(b, 4);
	params = fb_follow(b, fc, 6);
	if (!params || fb_u32(b, params) == 0)
		return NULL;

	p0 = params + 4 + fb_u32(b, params + 4);
	tf = fb_field(b, p0, 4);
	if (!tf || b[p0 + tf] != 7)
		return NULL;

	val = fb_follow(b, p0, 6);
	if (!val)
		return NULL;
	str = fb_follow(b, val, 4);
	if (!str || str + 4 > len)
		return NULL;

	slen = fb_u32(b, str);
	if (str + 4 + slen > len)
		return NULL;

	*out_len = slen;
	return (const char *)(b + str + 4);
}

/* Issue one /dev/hcall request and discard the response. */
static void hcall(const char *req)
{
	char resp[128];
	int fd = open("/dev/hcall", O_RDWR);

	if (fd < 0)
		return;
	if (write(fd, req, strlen(req)) >= 0)
		(void)!read(fd, resp, sizeof(resp));
	close(fd);
}

static void report(const char *key, long value)
{
	char req[128];
	int n = snprintf(req, sizeof(req),
			 "{\"name\":\"report\",\"args\":{\"%s\":%ld}}", key,
			 value);

	if (n > 0 && n < (int)sizeof(req))
		hcall(req);
}

/* Running total, deliberately in guest memory: the host observes it through
 * `report`, so a snapshot taken after one call and restored before the next
 * shows the total continuing from where the snapshot was taken.
 */
static long g_total;

static void handle_call(const uint8_t *fc, size_t fc_len)
{
	char buf[32];
	size_t arg_len = 0;
	const char *arg;
	long v = 0;
	int neg = 0;
	size_t i = 0;

	/* Copy the argument out before anything else: it points into the PEB
	 * input stack, which the host reuses for host-call responses.
	 */
	arg = fc_arg0_string(fc, fc_len, &arg_len);
	if (!arg) {
		report("error", ERR_NO_ARG);
		return;
	}
	if (arg_len >= sizeof(buf)) {
		report("error", ERR_ARG_TOO_LONG);
		return;
	}
	memcpy(buf, arg, arg_len);
	buf[arg_len] = '\0';

	if (buf[i] == '-') {
		neg = 1;
		i++;
	}
	for (; buf[i] >= '0' && buf[i] <= '9'; i++)
		v = v * 10 + (buf[i] - '0');
	if (neg)
		v = -v;

	/* Block in the middle of the call. Under the poll model this parks the
	 * handler's thread and returns the vCPU to the host; the call resumes
	 * on a later poll. On the blocking model it would strand the vCPU.
	 */
	usleep(HANDLER_PARK_US);

	g_total += v;
	report("total", g_total);
}

typedef void (*hl_dispatch_fn_t)(const uint8_t *fc, size_t fc_len);

static const uint8_t **g_fc_bytes_slot;
static size_t *g_fc_len_slot;
static hl_dispatch_fn_t *g_v2_callback_slot;

static unsigned long parse_hex(const char *s)
{
	unsigned long v = 0;

	if (s[0] == '0' && (s[1] == 'x' || s[1] == 'X'))
		s += 2;
	for (; *s; s++) {
		if (*s >= '0' && *s <= '9')
			v = (v << 4) | (unsigned long)(*s - '0');
		else if (*s >= 'a' && *s <= 'f')
			v = (v << 4) | (unsigned long)(*s - 'a' + 10);
		else if (*s >= 'A' && *s <= 'F')
			v = (v << 4) | (unsigned long)(*s - 'A' + 10);
		else
			break;
	}
	return v;
}

int main(int argc, char **argv, char **envp)
{
	const uint8_t *fc;
	size_t fc_len;

	(void)argc;
	(void)argv;

	/* app-elfloader publishes the addresses of the kernel's dispatch slots
	 * in the environment so a loaded ELF can reach them without linking
	 * against kernel symbols.
	 */
	for (char **p = envp; p && *p; p++) {
		if (!strncmp(*p, "HL_FC_BYTES_PTR=", 16))
			g_fc_bytes_slot = (const uint8_t **)parse_hex(*p + 16);
		else if (!strncmp(*p, "HL_FC_LEN_PTR=", 14))
			g_fc_len_slot = (size_t *)parse_hex(*p + 14);
		else if (!strncmp(*p, "HL_V2_CALLBACK_PTR=", 19))
			g_v2_callback_slot =
				(hl_dispatch_fn_t *)parse_hex(*p + 19);
	}
	if (!g_fc_bytes_slot || !g_fc_len_slot || !g_v2_callback_slot) {
		report("error", 100);
		return 1;
	}

	/* Serve every later call from here instead of re-running main(). */
	*g_v2_callback_slot = handle_call;

	fc = *g_fc_bytes_slot;
	fc_len = *g_fc_len_slot;
	if (fc && fc_len)
		handle_call(fc, fc_len);

	/* Halt the VM, keeping the registered callback and g_total in guest
	 * memory for the snapshot the host takes next.
	 */
	return 0;
}
