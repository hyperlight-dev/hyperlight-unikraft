#include <fcntl.h>
#include <string.h>
#include <unistd.h>

/*
 * Cooperative-poll async-await smoke app.
 *
 * Exercises the hcall yield/await request-ID mechanism (see
 * plat/hyperlight/hcall.c and plat/hyperlight/poll.c). The guest issues a
 * single host function call to a custom `async_add` tool that does NOT answer
 * immediately: the host returns the yield sentinel
 * {"result":{"__hl_yield__":<u64>}}. The kernel's hyperlight_hcall() then
 * transparently parks this thread; on each subsequent host `poll` the host
 * passes a batch of completed tasks as the poll argument, and the kernel
 * delivers the matching request ID's result to this parked call (see
 * hyperlight_hcall_deliver_batch). From this app's point of view the
 * write()/read() on /dev/hcall is an ordinary blocking call that eventually
 * yields the sum.
 *
 * The app then reports the received sum back to the host as its exit code via
 * __hl_exit, so the driving test can assert the value made it through the
 * yield/await round-trips intact (a=40, b=2 => exit code 42). A parse/error
 * failure reports a distinct non-42 code so the test fails loudly rather than
 * hanging.
 */

/* Issue one /dev/hcall request; copy the response into resp (NUL-terminated).
 * Returns the response length, or -1 on error.
 */
static long hcall(const char *req, char *resp, unsigned long resp_cap)
{
	int fd = open("/dev/hcall", O_RDWR);
	long n;

	if (fd < 0)
		return -1;

	if (write(fd, req, strlen(req)) < 0) {
		close(fd);
		return -1;
	}

	n = read(fd, resp, resp_cap - 1);
	close(fd);
	if (n < 0)
		return -1;

	resp[n] = '\0';
	return n;
}

/* Parse the integer following `"result":` in a JSON response. Returns 0 on
 * success (value in *out), -1 if the key is absent or unparseable.
 */
static int parse_result_int(const char *json, long *out)
{
	const char *p = strstr(json, "\"result\":");
	int neg = 0;
	long v = 0;
	int digits = 0;

	if (!p)
		return -1;
	p += 9; /* strlen("\"result\":") */

	while (*p == ' ' || *p == '\t')
		p++;
	if (*p == '-') {
		neg = 1;
		p++;
	}
	while (*p >= '0' && *p <= '9') {
		v = v * 10 + (*p - '0');
		p++;
		digits++;
	}
	if (!digits)
		return -1;

	*out = neg ? -v : v;
	return 0;
}

static void report_exit_code(int code)
{
	char req[64];
	char resp[128];
	char num[12];
	int ni = 0, i, j = 0;
	int c = code < 0 ? -code : code;

	/* Build {"name":"__hl_exit","args":{"code":<code>}} without snprintf. */
	if (c == 0) {
		num[ni++] = '0';
	} else {
		char tmp[12];
		int ti = 0;
		while (c > 0) {
			tmp[ti++] = '0' + (c % 10);
			c /= 10;
		}
		if (code < 0)
			num[ni++] = '-';
		while (ti > 0)
			num[ni++] = tmp[--ti];
	}
	num[ni] = '\0';

	const char *pre = "{\"name\":\"__hl_exit\",\"args\":{\"code\":";
	const char *suf = "}}";
	for (i = 0; pre[i]; i++)
		req[j++] = pre[i];
	for (i = 0; num[i]; i++)
		req[j++] = num[i];
	for (i = 0; suf[i]; i++)
		req[j++] = suf[i];
	req[j] = '\0';

	(void)hcall(req, resp, sizeof(resp));
}

int main(void)
{
	char resp[256];
	long sum = 0;

	/* Single call to a host tool that answers asynchronously. The kernel
	 * hides the yield/await round-trips: this returns only the final result.
	 */
	if (hcall("{\"name\":\"async_add\",\"args\":{\"a\":40,\"b\":2}}",
		  resp, sizeof(resp)) < 0) {
		report_exit_code(101); /* transport failure */
		return 101;
	}

	if (strstr(resp, "\"error\"")) {
		report_exit_code(102); /* host reported an error */
		return 102;
	}

	if (parse_result_int(resp, &sum) < 0) {
		report_exit_code(103); /* unparseable result */
		return 103;
	}

	/* Report the received sum as the exit code (expected 42). */
	report_exit_code((int)sum);
	return (int)sum;
}
