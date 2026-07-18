/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * hostsock: host-proxied AF_INET socket driver for Hyperlight
 *
 * Every POSIX socket operation is forwarded to the Hyperlight host via
 * the __dispatch RPC interface. The host performs real networking on
 * behalf of the guest. Binary payloads are base64-encoded in JSON.
 *
 * Copyright (c) 2025, Microsoft Corporation. All rights reserved.
 */

#include <errno.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <poll.h>

#include <sys/socket.h>
#include <netinet/in.h>

#include <uk/alloc.h>
#include <uk/assert.h>
#include <uk/errptr.h>
#include <uk/print.h>
#include <uk/socket_driver.h>
#include <uk/posix-fd.h>

#include <hyperlight-x86/hcall.h>
#include <uk/plat/time.h>

extern void time_block_until(__snsec until);

/* Max single RPC payload (matches HCALL_MAX_PAYLOAD). */
#define HOSTSOCK_RPC_MAX 65536

/* Per-socket driver data. */
struct hostsock_data {
	uint32_t host_fd;
	int nonblock;
};

/* Static per-call buffers. Not thread-safe; callers serialise via
 * posix-socket and vfscore locking. */
static char rpc_req[HOSTSOCK_RPC_MAX];
static char rpc_resp[HOSTSOCK_RPC_MAX];

/* Static scatter/gather buffer for sendmsg/recvmsg multi-iovec
 * flattening.  Stack allocation (32 KB) is unsafe on 64 KB stacks. */
static char iov_flat_buf[HOSTSOCK_RPC_MAX / 2];

/* -------- base64 codec (same as hostfs) -------- */

static const char b64_enc[] =
	"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

static size_t b64_encoded_len(size_t n)
{
	return ((n + 2) / 3) * 4;
}

static size_t b64_encode(const void *src_, size_t n, char *dst)
{
	const unsigned char *src = src_;
	size_t i, o = 0;

	for (i = 0; i + 3 <= n; i += 3) {
		unsigned v = ((unsigned)src[i] << 16) |
			     ((unsigned)src[i + 1] << 8) |
			     (unsigned)src[i + 2];
		dst[o++] = b64_enc[(v >> 18) & 0x3F];
		dst[o++] = b64_enc[(v >> 12) & 0x3F];
		dst[o++] = b64_enc[(v >> 6) & 0x3F];
		dst[o++] = b64_enc[v & 0x3F];
	}
	if (i < n) {
		unsigned v = (unsigned)src[i] << 16;
		if (i + 1 < n)
			v |= (unsigned)src[i + 1] << 8;
		dst[o++] = b64_enc[(v >> 18) & 0x3F];
		dst[o++] = b64_enc[(v >> 12) & 0x3F];
		dst[o++] = (i + 1 < n) ? b64_enc[(v >> 6) & 0x3F] : '=';
		dst[o++] = '=';
	}
	return o;
}

static int b64_val(char c)
{
	if (c >= 'A' && c <= 'Z') return c - 'A';
	if (c >= 'a' && c <= 'z') return c - 'a' + 26;
	if (c >= '0' && c <= '9') return c - '0' + 52;
	if (c == '+') return 62;
	if (c == '/') return 63;
	return -1;
}

static long b64_decode(const char *src, size_t n, void *dst_, size_t cap)
{
	unsigned char *dst = dst_;
	size_t i = 0, o = 0;
	int v[4], k;

	while (n > 0 && (src[n-1] == '\n' || src[n-1] == '\r'
			 || src[n-1] == ' ' || src[n-1] == '\t'))
		n--;

	while (i + 4 <= n) {
		for (k = 0; k < 4; k++) {
			char c = src[i + k];
			if (c == '=')
				v[k] = -2;
			else {
				v[k] = b64_val(c);
				if (v[k] < 0)
					return -1;
			}
		}
		i += 4;
		unsigned bits = ((unsigned)(v[0] < 0 ? 0 : v[0]) << 18) |
				((unsigned)(v[1] < 0 ? 0 : v[1]) << 12) |
				((unsigned)(v[2] < 0 ? 0 : v[2]) << 6) |
				(unsigned)(v[3] < 0 ? 0 : v[3]);
		if (o >= cap) return -1;
		dst[o++] = (bits >> 16) & 0xFF;
		if (v[2] != -2) {
			if (o >= cap) return -1;
			dst[o++] = (bits >> 8) & 0xFF;
		}
		if (v[3] != -2) {
			if (o >= cap) return -1;
			dst[o++] = bits & 0xFF;
		}
	}
	if (i != n) return -1;
	return (long)o;
}

/* -------- IPv4 address helpers (no arpa/inet.h in nolibc) -------- */

static void ipv4_ntop(const struct in_addr *src, char *dst, size_t sz)
{
	const unsigned char *b = (const unsigned char *)&src->s_addr;
	snprintf(dst, sz, "%u.%u.%u.%u", b[0], b[1], b[2], b[3]);
}

static int ipv4_pton(const char *src, struct in_addr *dst)
{
	unsigned a, b, c, d;
	if (sscanf(src, "%u.%u.%u.%u", &a, &b, &c, &d) != 4)
		return 0;
	unsigned char *p = (unsigned char *)&dst->s_addr;
	p[0] = a; p[1] = b; p[2] = c; p[3] = d;
	return 1;
}

/* -------- tiny JSON helpers (same pattern as hostfs) -------- */

static const char *json_scan_key(const char *json, const char *key)
{
	char needle[64];
	int n = snprintf(needle, sizeof(needle), "\"%s\"", key);
	if (n < 0 || (size_t)n >= sizeof(needle))
		return NULL;
	const char *p = strstr(json, needle);
	if (!p) return NULL;
	p += n;
	while (*p == ' ' || *p == '\t' || *p == ':')
		p++;
	return p;
}

static int json_has_error(const char *json)
{
	return strstr(json, "\"error\"") != NULL;
}

static int json_errno(const char *json)
{
	const char *p = json_scan_key(json, "error");
	if (!p || *p != '"')
		return -EIO;
	p++;
	if (strstr(p, "AddrInUse") || strstr(p, "address already in use"))
		return -EADDRINUSE;
	if (strstr(p, "AddrNotAvail"))
		return -EADDRNOTAVAIL;
	if (strstr(p, "ConnectionRefused") || strstr(p, "refused"))
		return -ECONNREFUSED;
	if (strstr(p, "ConnectionReset") || strstr(p, "reset"))
		return -ECONNRESET;
	if (strstr(p, "ConnectionAborted"))
		return -ECONNABORTED;
	if (strstr(p, "NotConnected"))
		return -ENOTCONN;
	if (strstr(p, "TimedOut") || strstr(p, "timed out"))
		return -ETIMEDOUT;
	if (strstr(p, "WouldBlock"))
		return -EWOULDBLOCK;
	if (strstr(p, "permission") || strstr(p, "Permission"))
		return -EACCES;
	if (strstr(p, "denies") || strstr(p, "policy"))
		return -ECONNREFUSED;
	if (strstr(p, "InvalidInput") || strstr(p, "Invalid argument"))
		return -EINVAL;
	if (strstr(p, "bad_fd"))
		return -EBADF;
	return -EIO;
}

static int json_get_int(const char *json, const char *key, long long *out)
{
	const char *p = json_scan_key(json, key);
	if (!p) return -1;
	char *end;
	long long v = strtoll(p, &end, 10);
	if (end == p) return -1;
	*out = v;
	return 0;
}

static int json_get_string(const char *json, const char *key,
			   char *out, size_t cap)
{
	const char *p = json_scan_key(json, key);
	if (!p || *p != '"') return -1;
	p++;
	size_t o = 0;
	while (*p && *p != '"') {
		if (o + 1 >= cap) return -1;
		if (*p == '\\' && p[1]) {
			out[o++] = p[1];
			p += 2;
		} else {
			out[o++] = *p++;
		}
	}
	out[o] = '\0';
	return (int)o;
}

/* -------- RPC round-trip -------- */

static int rpc_exchange(size_t req_len, size_t *resp_len)
{
	__sz rlen = 0;
	int rc = hyperlight_hcall((const __u8 *)rpc_req, req_len,
				  (__u8 *)rpc_resp,
				  sizeof(rpc_resp) - 1, &rlen);
	if (rc < 0) {
		uk_pr_err("hostsock: hcall failed: %d\n", rc);
		return -EIO;
	}
	rpc_resp[rlen] = '\0';
	if (resp_len) *resp_len = rlen;
	if (json_has_error(rpc_resp)) {
		int e = json_errno(rpc_resp);
		uk_pr_crit("hostsock: RPC error (%d): %s\n", e, rpc_resp);
		return e;
	}
	return 0;
}

static int build_req(const char *fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	int n = vsnprintf(rpc_req, sizeof(rpc_req), fmt, ap);
	va_end(ap);
	if (n < 0 || (size_t)n >= sizeof(rpc_req))
		return -ENOMEM;
	return n;
}

/* -------- sockaddr helpers -------- */

static void ipv6_ntop(const struct in6_addr *src, char *dst, size_t sz)
{
	const unsigned char *b = src->s6_addr;
	/* Check for IPv4-mapped */
	int v4mapped = 1;
	for (int i = 0; i < 10; i++)
		if (b[i]) { v4mapped = 0; break; }
	if (v4mapped && b[10] == 0xff && b[11] == 0xff) {
		snprintf(dst, sz, "::ffff:%u.%u.%u.%u",
			 b[12], b[13], b[14], b[15]);
		return;
	}
	/* General case: find longest run of zeros for :: */
	unsigned short w[8];
	for (int i = 0; i < 8; i++)
		w[i] = (b[i*2] << 8) | b[i*2+1];
	int best_start = -1, best_len = 0, cur_start = -1, cur_len = 0;
	for (int i = 0; i < 8; i++) {
		if (w[i] == 0) {
			if (cur_start < 0) cur_start = i;
			cur_len++;
		} else {
			if (cur_len > best_len) {
				best_start = cur_start;
				best_len = cur_len;
			}
			cur_start = -1;
			cur_len = 0;
		}
	}
	if (cur_len > best_len) {
		best_start = cur_start;
		best_len = cur_len;
	}
	char *p = dst;
	char *end = dst + sz;
	for (int i = 0; i < 8; i++) {
		if (best_len >= 2 && i == best_start) {
			p += snprintf(p, end - p, "::");
			i += best_len - 1;
			continue;
		}
		if (i > 0 && !(best_len >= 2 && i == best_start + best_len))
			p += snprintf(p, end - p, ":");
		p += snprintf(p, end - p, "%x", w[i]);
	}
}

static void sockaddr_to_json(const struct sockaddr *addr, socklen_t len,
			     char *buf, size_t cap)
{
	if (addr->sa_family == AF_INET && len >= sizeof(struct sockaddr_in)) {
		const struct sockaddr_in *in = (const struct sockaddr_in *)addr;
		char ip[INET_ADDRSTRLEN];
		ipv4_ntop(&in->sin_addr, ip, sizeof(ip));
		snprintf(buf, cap,
			 "\"family\":2,\"addr\":\"%s\",\"port\":%u",
			 ip, ntohs(in->sin_port));
	} else if (addr->sa_family == AF_INET6 &&
		   len >= sizeof(struct sockaddr_in6)) {
		const struct sockaddr_in6 *in6 =
			(const struct sockaddr_in6 *)addr;
		char ip[64];
		ipv6_ntop(&in6->sin6_addr, ip, sizeof(ip));
		snprintf(buf, cap,
			 "\"family\":10,\"addr\":\"%s\",\"port\":%u",
			 ip, ntohs(in6->sin6_port));
	} else {
		snprintf(buf, cap,
			 "\"family\":2,\"addr\":\"0.0.0.0\",\"port\":0");
	}
}

static int ipv6_pton(const char *src, struct in6_addr *dst)
{
	memset(dst, 0, sizeof(*dst));
	/* Handle IPv4-mapped ::ffff:a.b.c.d */
	if (strncmp(src, "::ffff:", 7) == 0) {
		dst->s6_addr[10] = 0xff;
		dst->s6_addr[11] = 0xff;
		unsigned a, b, c, d;
		if (sscanf(src + 7, "%u.%u.%u.%u", &a, &b, &c, &d) != 4)
			return 0;
		dst->s6_addr[12] = a;
		dst->s6_addr[13] = b;
		dst->s6_addr[14] = c;
		dst->s6_addr[15] = d;
		return 1;
	}
	/* Handle ::1 (loopback) */
	if (strcmp(src, "::1") == 0) {
		dst->s6_addr[15] = 1;
		return 1;
	}
	/* Handle :: (unspecified) */
	if (strcmp(src, "::") == 0)
		return 1;
	/* General IPv6 parsing: simplified two-part with :: */
	unsigned short words[8];
	int nwords = 0, gap = -1;
	const char *p = src;
	while (*p && nwords < 8) {
		if (p[0] == ':' && p[1] == ':') {
			gap = nwords;
			p += 2;
			continue;
		}
		if (*p == ':')
			p++;
		unsigned val = 0;
		int digits = 0;
		while (*p && *p != ':' && digits < 4) {
			unsigned c = *p++;
			if (c >= '0' && c <= '9') val = val * 16 + c - '0';
			else if (c >= 'a' && c <= 'f') val = val * 16 + c - 'a' + 10;
			else if (c >= 'A' && c <= 'F') val = val * 16 + c - 'A' + 10;
			else return 0;
			digits++;
		}
		words[nwords++] = (unsigned short)val;
	}
	if (gap >= 0) {
		int shift = 8 - nwords;
		int i;
		for (i = nwords - 1; i >= gap; i--)
			words[i + shift] = words[i];
		for (i = gap; i < gap + shift; i++)
			words[i] = 0;
	}
	for (int i = 0; i < 8; i++) {
		dst->s6_addr[i * 2] = words[i] >> 8;
		dst->s6_addr[i * 2 + 1] = words[i] & 0xff;
	}
	return 1;
}

static void json_to_sockaddr(const char *json, struct sockaddr *addr,
			     socklen_t *addr_len)
{
	char ip[64];
	long long port = 0;
	long long family = AF_INET;
	if (json_get_string(json, "addr", ip, sizeof(ip)) < 0)
		return;
	json_get_int(json, "port", &port);
	json_get_int(json, "family", &family);

	if (family == AF_INET6) {
		struct sockaddr_in6 *in6 = (struct sockaddr_in6 *)addr;
		memset(in6, 0, sizeof(*in6));
		in6->sin6_family = AF_INET6;
		in6->sin6_port = htons((uint16_t)port);
		ipv6_pton(ip, &in6->sin6_addr);
		if (addr_len)
			*addr_len = sizeof(struct sockaddr_in6);
	} else {
		struct sockaddr_in *in = (struct sockaddr_in *)addr;
		memset(in, 0, sizeof(*in));
		in->sin_family = AF_INET;
		in->sin_port = htons((uint16_t)port);
		ipv4_pton(ip, &in->sin_addr);
		if (addr_len)
			*addr_len = sizeof(struct sockaddr_in);
	}
}

/* -------- tracked sockets for event rescan -------- */

#define HOSTSOCK_MAX_TRACKED 64
static posix_sock *tracked_socks[HOSTSOCK_MAX_TRACKED];
static int tracked_count;

static const void *_hostsock_vol_marker;
static struct { uint32_t host_fd; unsigned events; }
	_hostsock_ev_cache[HOSTSOCK_MAX_TRACKED];
static int _hostsock_ev_cache_count;

static void hostsock_track(posix_sock *sock)
{
	if (tracked_count < HOSTSOCK_MAX_TRACKED)
		tracked_socks[tracked_count++] = sock;
}

static void hostsock_untrack(posix_sock *sock)
{
	for (int i = 0; i < tracked_count; i++) {
		if (tracked_socks[i] == sock) {
			tracked_socks[i] = tracked_socks[--tracked_count];
			return;
		}
	}
}

int hostsock_rescan_events(void);

struct uk_thread *_hostsock_listener_tid;
int _hostsock_listener_pid;

const void *hostsock_get_vol_marker(void)
{
	return _hostsock_vol_marker;
}

/* -------- socket operations -------- */

static uint32_t get_host_fd(posix_sock *sock)
{
	struct hostsock_data *d = posix_sock_get_data(sock);
	return d->host_fd;
}

static void *hostsock_create(struct posix_socket_driver *d,
			     int family, int type, int protocol)
{
	int nonblock = (type & SOCK_NONBLOCK) ? 1 : 0;
	int sock_type = type & ~SOCK_FLAGS;
	int n = build_req(
		"{\"name\":\"net_socket\",\"args\":"
		"{\"family\":%d,\"type\":%d,\"protocol\":%d}}",
		family, sock_type, protocol);
	if (n < 0)
		return ERR2PTR(-ENOMEM);

	int rc = rpc_exchange(n, NULL);
	if (rc < 0)
		return ERR2PTR(rc);

	long long fd = -1;
	if (json_get_int(rpc_resp, "fd", &fd) < 0)
		return ERR2PTR(-EIO);

	struct hostsock_data *sd = uk_malloc(d->allocator, sizeof(*sd));
	if (!sd)
		return ERR2PTR(-ENOMEM);
	sd->host_fd = (uint32_t)fd;
	sd->nonblock = nonblock;

	uk_pr_debug("hostsock: create fd=%u (family=%d type=%d nb=%d)\n",
		    sd->host_fd, family, sock_type, nonblock);
	uk_pr_crit("hostsock: create fd=%u type=%d nb=%d\n",
		   sd->host_fd, sock_type, nonblock);
	return sd;
}

static int hostsock_bind(posix_sock *sock,
			 const struct sockaddr *addr, socklen_t addr_len)
{
	char abuf[128];
	sockaddr_to_json(addr, addr_len, abuf, sizeof(abuf));
	int n = build_req(
		"{\"name\":\"net_bind\",\"args\":{\"fd\":%u,%s}}",
		get_host_fd(sock), abuf);
	if (n < 0) return -ENOMEM;
	return rpc_exchange(n, NULL);
}

static int hostsock_listen(posix_sock *sock, int backlog)
{
	int n = build_req(
		"{\"name\":\"net_listen\",\"args\":{\"fd\":%u,\"backlog\":%d}}",
		get_host_fd(sock), backlog);
	if (n < 0) return -ENOMEM;
	return rpc_exchange(n, NULL);
}

/* Check if a host socket has a pending event using net_poll(timeout=0). */
static int hostsock_check_ready(uint32_t host_fd, int events)
{
	int n = build_req(
		"{\"name\":\"net_poll\",\"args\":"
		"{\"fds\":[{\"fd\":%u,\"events\":%d}],\"timeout_ms\":0}}",
		host_fd, events);
	if (n < 0)
		return 0;
	size_t resp_len;
	if (rpc_exchange(n, &resp_len) < 0)
		return 0;
	long long revents = 0;
	json_get_int(rpc_resp, "revents", &revents);
	return (int)revents;
}

static void *hostsock_accept4(posix_sock *sock,
			      struct sockaddr *restrict addr,
			      socklen_t *restrict addr_len,
			      int flags)
{
	struct hostsock_data *listen_data = posix_sock_get_data(sock);

	/*
	 * Always check readiness before calling the host's blocking accept.
	 * A blocking hcall freezes the entire VM (single vCPU), so we must
	 * never let accept block on the host side.  Return EAGAIN and let
	 * the Unikraft poll/epoll layer handle the wait.
	 *
	 * This also covers runtimes that set non-blocking mode via
	 * fcntl(F_SETFL, O_NONBLOCK) rather than ioctl(FIONBIO) — fcntl
	 * updates the uk_file flags but not our sd->nonblock field.
	 */
	{
		int ready = hostsock_check_ready(listen_data->host_fd, 1);
		if (!(ready & 1)) /* POLLIN */
			return ERR2PTR(-EAGAIN);
	}

	int n = build_req(
		"{\"name\":\"net_accept\",\"args\":{\"fd\":%u}}",
		get_host_fd(sock));
	if (n < 0) return ERR2PTR(-ENOMEM);

	int rc = rpc_exchange(n, NULL);
	if (rc < 0) return ERR2PTR(rc);

	long long new_fd = -1;
	if (json_get_int(rpc_resp, "fd", &new_fd) < 0)
		return ERR2PTR(-EIO);

	struct posix_socket_driver *drv = posix_sock_get_driver(sock);
	struct hostsock_data *sd = uk_malloc(drv->allocator, sizeof(*sd));
	if (!sd)
		return ERR2PTR(-ENOMEM);
	sd->host_fd = (uint32_t)new_fd;
	sd->nonblock = (flags & SOCK_NONBLOCK) ? 1 : 0;

	if (addr && addr_len)
		json_to_sockaddr(rpc_resp, addr, addr_len);

	uk_pr_debug("hostsock: accept -> fd=%u (nb=%d)\n",
		    sd->host_fd, sd->nonblock);
	return sd;
}

static int is_loopback(const struct sockaddr *addr)
{
	if (addr->sa_family == AF_INET) {
		const unsigned char *b =
			(const unsigned char *)&((const struct sockaddr_in *)addr)
			->sin_addr.s_addr;
		return b[0] == 127;
	}
	if (addr->sa_family == AF_INET6) {
		const struct in6_addr *a =
			&((const struct sockaddr_in6 *)addr)->sin6_addr;
		static const unsigned char lo[16] =
			{0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1};
		return memcmp(a->s6_addr, lo, 16) == 0;
	}
	return 0;
}

static int hostsock_connect(posix_sock *sock,
			    const struct sockaddr *addr, socklen_t addr_len)
{
	/* Loopback is always denied by host NetworkPolicy.  Silently
	 * succeed so libuv probes (recvmmsg → 127.0.0.1:65535) don't
	 * poison errno for other threads under cooperative scheduling. */
	if (is_loopback(addr))
		return 0;

	char abuf[128];
	sockaddr_to_json(addr, addr_len, abuf, sizeof(abuf));
	int n = build_req(
		"{\"name\":\"net_connect\",\"args\":{\"fd\":%u,%s}}",
		get_host_fd(sock), abuf);
	if (n < 0) return -ENOMEM;
	int rc = rpc_exchange(n, NULL);
	if (addr->sa_family == AF_INET) {
		struct sockaddr_in *in = (struct sockaddr_in *)addr;
		char ip[20]; ipv4_ntop(&in->sin_addr, ip, sizeof(ip));
		uk_pr_crit("hostsock: connect fd=%u -> %s:%u rc=%d\n",
			   get_host_fd(sock), ip,
			   (unsigned)__builtin_bswap16(in->sin_port), rc);
	}
	return rc;
}

static ssize_t hostsock_sendto(posix_sock *sock,
			       const void *buf, size_t len, int flags,
			       const struct sockaddr *dest_addr,
			       socklen_t addrlen)
{
	size_t enc_len = b64_encoded_len(len);
	char abuf[128] = {0};
	if (dest_addr && addrlen > 0)
		sockaddr_to_json(dest_addr, addrlen, abuf, sizeof(abuf));

	/* Build header */
	int n;
	if (abuf[0]) {
		n = snprintf(rpc_req, sizeof(rpc_req),
			"{\"name\":\"net_sendto\",\"args\":"
			"{\"fd\":%u,\"flags\":%d,%s,\"data\":\"",
			get_host_fd(sock), flags, abuf);
	} else {
		n = snprintf(rpc_req, sizeof(rpc_req),
			"{\"name\":\"net_send\",\"args\":"
			"{\"fd\":%u,\"flags\":%d,\"data\":\"",
			get_host_fd(sock), flags);
	}
	if (n < 0 || (size_t)n + enc_len + 4 >= sizeof(rpc_req))
		return -EMSGSIZE;

	enc_len = b64_encode(buf, len, rpc_req + n);
	n += (int)enc_len;
	memcpy(rpc_req + n, "\"}}", 3);
	n += 3;

	int rc = rpc_exchange((size_t)n, NULL);
	if (rc < 0) return rc;

	long long sent = 0;
	if (json_get_int(rpc_resp, "sent", &sent) < 0)
		return -EIO;
	if (dest_addr && dest_addr->sa_family == AF_INET) {
		struct sockaddr_in *in = (struct sockaddr_in *)dest_addr;
		if (__builtin_bswap16(in->sin_port) == 53)
			uk_pr_crit("hostsock: sendto fd=%u port=53 len=%zu sent=%lld\n",
				   get_host_fd(sock), len, sent);
	}
	return (ssize_t)sent;
}

static ssize_t hostsock_recvfrom(posix_sock *sock,
				 void *restrict buf, size_t len, int flags,
				 struct sockaddr *from,
				 socklen_t *restrict fromlen)
{
	struct hostsock_data *sd = posix_sock_get_data(sock);

	/*
	 * Poll readiness with timed retries.  A blocking recv hcall would
	 * freeze the entire VM, so we must poll the host ourselves.
	 * We retry for up to ~30 s (300 × 100 ms) to cover slow network
	 * round-trips.  Each sleep call lets the host poll sockets too.
	 */
	for (int attempt = 0; attempt < 300; attempt++) {
		int ready = hostsock_check_ready(sd->host_fd, 1);
		if (ready & 1)
			goto do_recv;
		if (sd->nonblock || (flags & MSG_DONTWAIT))
			return -EAGAIN;
		time_block_until((__snsec)ukplat_monotonic_clock()
				 + 100000000LL);
	}
	return -EAGAIN;

do_recv:

	int n = build_req(
		"{\"name\":\"net_recvfrom\",\"args\":"
		"{\"fd\":%u,\"len\":%zu,\"flags\":%d}}",
		get_host_fd(sock), len, flags);
	if (n < 0) return -ENOMEM;

	int rc = rpc_exchange(n, NULL);
	if (rc < 0) return rc;

	/* Extract base64 data */
	const char *p = strstr(rpc_resp, "\"data\":\"");
	if (!p) return -EIO;
	p += 8;
	const char *end = strchr(p, '"');
	if (!end) return -EIO;

	long decoded = b64_decode(p, (size_t)(end - p), buf, len);
	if (decoded < 0) return -EIO;

	if (from && fromlen)
		json_to_sockaddr(rpc_resp, from, fromlen);

	return (ssize_t)decoded;
}

static ssize_t hostsock_sendmsg(posix_sock *sock,
				const struct msghdr *msg, int flags)
{
	size_t total = 0;
	for (size_t i = 0; i < (size_t)msg->msg_iovlen; i++)
		total += msg->msg_iov[i].iov_len;

	if (msg->msg_iovlen == 1) {
		return hostsock_sendto(sock, msg->msg_iov[0].iov_base,
				      msg->msg_iov[0].iov_len, flags,
				      msg->msg_name, msg->msg_namelen);
	}

	if (total > sizeof(iov_flat_buf))
		return -EMSGSIZE;
	size_t off = 0;
	for (size_t i = 0; i < (size_t)msg->msg_iovlen; i++) {
		memcpy(iov_flat_buf + off, msg->msg_iov[i].iov_base,
		       msg->msg_iov[i].iov_len);
		off += msg->msg_iov[i].iov_len;
	}
	return hostsock_sendto(sock, iov_flat_buf, total, flags,
			       msg->msg_name, msg->msg_namelen);
}

static ssize_t hostsock_recvmsg(posix_sock *sock,
				struct msghdr *msg, int flags)
{
	size_t total = 0;
	for (size_t i = 0; i < (size_t)msg->msg_iovlen; i++)
		total += msg->msg_iov[i].iov_len;

	if (msg->msg_iovlen == 1) {
		return hostsock_recvfrom(sock, msg->msg_iov[0].iov_base,
					msg->msg_iov[0].iov_len, flags,
					msg->msg_name, &msg->msg_namelen);
	}

	if (total > sizeof(iov_flat_buf))
		return -EMSGSIZE;

	ssize_t got = hostsock_recvfrom(sock, iov_flat_buf, total, flags,
					msg->msg_name, &msg->msg_namelen);
	if (got <= 0)
		return got;

	/* Scatter into iovecs */
	size_t off = 0;
	for (size_t i = 0; i < (size_t)msg->msg_iovlen && off < (size_t)got;
	     i++) {
		size_t chunk = msg->msg_iov[i].iov_len;
		if (chunk > (size_t)got - off)
			chunk = (size_t)got - off;
		memcpy(msg->msg_iov[i].iov_base, iov_flat_buf + off, chunk);
		off += chunk;
	}
	return got;
}

static int hostsock_shutdown(posix_sock *sock, int how)
{
	int n = build_req(
		"{\"name\":\"net_shutdown\",\"args\":{\"fd\":%u,\"how\":%d}}",
		get_host_fd(sock), how);
	if (n < 0) return -ENOMEM;
	return rpc_exchange(n, NULL);
}

static int hostsock_getpeername(posix_sock *sock,
				struct sockaddr *restrict addr,
				socklen_t *restrict addr_len)
{
	int n = build_req(
		"{\"name\":\"net_getpeername\",\"args\":{\"fd\":%u}}",
		get_host_fd(sock));
	if (n < 0) return -ENOMEM;
	int rc = rpc_exchange(n, NULL);
	if (rc < 0) return rc;
	json_to_sockaddr(rpc_resp, addr, addr_len);
	return 0;
}

static int hostsock_getsockname(posix_sock *sock,
				struct sockaddr *restrict addr,
				socklen_t *restrict addr_len)
{
	int n = build_req(
		"{\"name\":\"net_getsockname\",\"args\":{\"fd\":%u}}",
		get_host_fd(sock));
	if (n < 0) return -ENOMEM;
	int rc = rpc_exchange(n, NULL);
	if (rc < 0) return rc;
	json_to_sockaddr(rpc_resp, addr, addr_len);
	return 0;
}

static int hostsock_getsockopt(posix_sock *sock,
			       int level, int optname,
			       void *restrict optval,
			       socklen_t *restrict optlen)
{
	int n = build_req(
		"{\"name\":\"net_getsockopt\",\"args\":"
		"{\"fd\":%u,\"level\":%d,\"optname\":%d}}",
		get_host_fd(sock), level, optname);
	if (n < 0) return -ENOMEM;
	int rc = rpc_exchange(n, NULL);
	if (rc < 0) return rc;

	long long val = 0;
	if (json_get_int(rpc_resp, "value", &val) == 0) {
		if (optlen && *optlen >= sizeof(int)) {
			*(int *)optval = (int)val;
			*optlen = sizeof(int);
		}
	}
	return 0;
}

static int hostsock_setsockopt(posix_sock *sock,
			       int level, int optname,
			       const void *optval, socklen_t optlen)
{
	int val = 0;
	if (optval && optlen >= sizeof(int))
		val = *(const int *)optval;

	int n = build_req(
		"{\"name\":\"net_setsockopt\",\"args\":"
		"{\"fd\":%u,\"level\":%d,\"optname\":%d,\"value\":%d}}",
		get_host_fd(sock), level, optname, val);
	if (n < 0) return -ENOMEM;
	return rpc_exchange(n, NULL);
}

static ssize_t hostsock_read(posix_sock *sock,
			     const struct iovec *iov, size_t iovcnt)
{
	if (iovcnt == 0)
		return 0;
	/* Read into first iovec only for simplicity */
	return hostsock_recvfrom(sock, iov[0].iov_base, iov[0].iov_len,
				 0, NULL, NULL);
}

static ssize_t hostsock_write(posix_sock *sock,
			      const struct iovec *iov, size_t iovcnt)
{
	/* 48000 raw bytes → ~64000 base64 bytes, fits in 65536-byte rpc_req
	 * with room for the JSON header/trailer. */
	static const size_t MAX_CHUNK = 48000;

	if (iovcnt == 0)
		return 0;

	ssize_t total = 0;
	for (size_t i = 0; i < iovcnt; i++) {
		const char *base = iov[i].iov_base;
		size_t remaining = iov[i].iov_len;

		while (remaining > 0) {
			size_t chunk = remaining > MAX_CHUNK ? MAX_CHUNK : remaining;
			ssize_t sent = hostsock_sendto(sock, base, chunk,
						       0, NULL, 0);
			if (sent < 0)
				return total > 0 ? total : sent;
			total += sent;
			base += sent;
			remaining -= (size_t)sent;
			if ((size_t)sent < chunk)
				return total;
		}
	}
	return total;
}

static int hostsock_close(posix_sock *sock)
{
	struct hostsock_data *sd = posix_sock_get_data(sock);
	uint32_t fd = sd->host_fd;

	uk_pr_debug("hostsock: close fd=%u\n", fd);

	int n = build_req(
		"{\"name\":\"net_close\",\"args\":{\"fd\":%u}}", fd);
	if (n >= 0)
		rpc_exchange(n, NULL);

	hostsock_untrack(sock);
	struct posix_socket_driver *drv = posix_sock_get_driver(sock);
	uk_free(drv->allocator, sd);
	return 0;
}

static int hostsock_ioctl(posix_sock *sock, int request, void *argp)
{
	/* FIONBIO = 0x5421 on Linux */
	if (request == 0x5421 && argp) {
		struct hostsock_data *sd = posix_sock_get_data(sock);
		sd->nonblock = (*(int *)argp) ? 1 : 0;
		return 0;
	}
	return -ENOSYS;
}

static int hostsock_socketpair(struct posix_socket_driver *d __attribute__((unused)),
			       int family __attribute__((unused)),
			       int type __attribute__((unused)),
			       int protocol __attribute__((unused)),
			       void *sockvec[2] __attribute__((unused)))
{
	return -EOPNOTSUPP;
}

static void hostsock_poll_setup(posix_sock *sock)
{
	if (!_hostsock_vol_marker)
		_hostsock_vol_marker = sock->vol;
	uint32_t fd = get_host_fd(sock);
	/* POLLIN=1, POLLOUT=4 — check real readiness via host poll(). */
	int revents = hostsock_check_ready(fd, 1 | 4);
	unsigned events = 0;
	if (revents & 1)
		events |= UKFD_POLLIN;
	if (revents & 4)
		events |= UKFD_POLLOUT;
	posix_sock_event_set(sock, events);
	hostsock_track(sock);
}

int hostsock_rescan_events(void)
{
	int woke = 0;

	_hostsock_ev_cache_count = 0;
	for (int i = 0; i < tracked_count; i++) {
		posix_sock *sock = tracked_socks[i];
		uint32_t fd = get_host_fd(sock);
		int revents = hostsock_check_ready(fd, 1 | 4);
		unsigned events = 0;
		if (revents & 1)
			events |= UKFD_POLLIN;
		if (revents & 4)
			events |= UKFD_POLLOUT;
		_hostsock_ev_cache[_hostsock_ev_cache_count].host_fd = fd;
		_hostsock_ev_cache[_hostsock_ev_cache_count].events = events;
		_hostsock_ev_cache_count++;
		posix_sock_event_assign(sock, events);
		if (events)
			woke = 1;
	}
	return woke;
}

unsigned int hostsock_lookup_file_events(const struct uk_file *f)
{
	struct posix_socket_node *node;
	struct hostsock_data *sd;
	uint32_t host_fd;

	if (!_hostsock_vol_marker || f->vol != _hostsock_vol_marker)
		return 0;
	node = (struct posix_socket_node *)f->node;
	if (!node || !node->sock_data)
		return 0;
	sd = (struct hostsock_data *)node->sock_data;
	host_fd = sd->host_fd;
	for (int i = 0; i < _hostsock_ev_cache_count; i++) {
		if (_hostsock_ev_cache[i].host_fd == host_fd)
			return _hostsock_ev_cache[i].events;
	}
	return 0;
}

static struct posix_socket_ops hostsock_ops = {
	.create      = hostsock_create,
	.accept4     = hostsock_accept4,
	.bind        = hostsock_bind,
	.shutdown    = hostsock_shutdown,
	.getpeername = hostsock_getpeername,
	.getsockname = hostsock_getsockname,
	.getsockopt  = hostsock_getsockopt,
	.setsockopt  = hostsock_setsockopt,
	.connect     = hostsock_connect,
	.listen      = hostsock_listen,
	.recvfrom    = hostsock_recvfrom,
	.recvmsg     = hostsock_recvmsg,
	.sendmsg     = hostsock_sendmsg,
	.sendto      = hostsock_sendto,
	.socketpair  = hostsock_socketpair,
	.read        = hostsock_read,
	.write       = hostsock_write,
	.close       = hostsock_close,
	.ioctl       = hostsock_ioctl,
	.poll_setup  = hostsock_poll_setup,
};

POSIX_SOCKET_FAMILY_REGISTER(AF_INET, &hostsock_ops);
POSIX_SOCKET_FAMILY_REGISTER(AF_INET6, &hostsock_ops);

/* -------- host-side getaddrinfo -------- */

#ifndef EAI_NONAME
#define EAI_NONAME -2
#define EAI_MEMORY -10
#define EAI_SYSTEM -11
#endif

struct addrinfo {
	int ai_flags;
	int ai_family;
	int ai_socktype;
	int ai_protocol;
	socklen_t ai_addrlen;
	struct sockaddr *ai_addr;
	char *ai_canonname;
	struct addrinfo *ai_next;
};

int getaddrinfo(const char *node, const char *service,
		const struct addrinfo *hints, struct addrinfo **res)
{
	uk_pr_crit("hostsock: getaddrinfo node=%s svc=%s\n",
		   node ? node : "(null)", service ? service : "(null)");
	if (!node)
		return EAI_NONAME;

	unsigned port = 0;
	if (service)
		port = (unsigned)strtoul(service, NULL, 10);

	int n = build_req(
		"{\"name\":\"net_getaddrinfo\",\"args\":"
		"{\"host\":\"%s\",\"port\":%u}}",
		node, port);
	if (n < 0)
		return EAI_MEMORY;

	size_t resp_len;
	if (rpc_exchange(n, &resp_len) < 0)
		return EAI_SYSTEM;

	const char *arr = json_scan_key(rpc_resp, "addrs");
	if (!arr || *arr != '[')
		return EAI_NONAME;
	arr++;

	struct addrinfo *head = NULL, **tail = &head;
	while (*arr) {
		while (*arr == ' ' || *arr == ',')
			arr++;
		if (*arr == ']')
			break;
		if (*arr != '{')
			break;
		arr++;

		long long family = 2;
		{
			const char *fp = strstr(arr, "\"family\"");
			if (fp) {
				fp += 8;
				while (*fp == ':' || *fp == ' ')
					fp++;
				family = strtoll(fp, NULL, 10);
			}
		}

		char addr_str[64] = {0};
		{
			const char *ap = strstr(arr, "\"addr\":\"");
			if (ap) {
				ap += 8;
				size_t i = 0;
				while (*ap && *ap != '"' && i + 1 < sizeof(addr_str))
					addr_str[i++] = *ap++;
				addr_str[i] = '\0';
			}
		}

		long long rport = port;
		{
			const char *pp = strstr(arr, "\"port\"");
			if (pp) {
				pp += 6;
				while (*pp == ':' || *pp == ' ')
					pp++;
				rport = strtoll(pp, NULL, 10);
			}
		}

		while (*arr && *arr != '}')
			arr++;
		if (*arr == '}')
			arr++;

		if (!addr_str[0])
			continue;

		struct addrinfo *ai = uk_calloc(uk_alloc_get_default(),
						1, sizeof(*ai));
		if (!ai)
			break;

		if (family == 2) {
			struct sockaddr_in *sin = uk_calloc(
				uk_alloc_get_default(), 1, sizeof(*sin));
			if (!sin) { uk_free(uk_alloc_get_default(), ai); break; }
			sin->sin_family = AF_INET;
			sin->sin_port = __builtin_bswap16((uint16_t)rport);
			ipv4_pton(addr_str, &sin->sin_addr);
			ai->ai_family = AF_INET;
			ai->ai_socktype = SOCK_STREAM;
			ai->ai_addrlen = sizeof(*sin);
			ai->ai_addr = (struct sockaddr *)sin;
		} else {
			uk_free(uk_alloc_get_default(), ai);
			continue;
		}

		*tail = ai;
		tail = &ai->ai_next;
	}

	if (!head)
		return EAI_NONAME;

	*res = head;
	return 0;
}

void freeaddrinfo(struct addrinfo *res)
{
	while (res) {
		struct addrinfo *next = res->ai_next;
		if (res->ai_addr)
			uk_free(uk_alloc_get_default(), res->ai_addr);
		uk_free(uk_alloc_get_default(), res);
		res = next;
	}
}
