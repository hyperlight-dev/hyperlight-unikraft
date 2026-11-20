/* SPDX-License-Identifier: Apache-2.0 */
/* Copyright 2026 The Hyperlight Authors. */

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <hyperlight/postcard.h>
#include "../../drivers/hl_fc.h"

static __u8 buf[262144];

int main(int argc, char **argv)
{
	__u64 control = 0, len;

	assert(argc == 2);
	if (!strcmp(argv[1], "encode")) {
		const struct hl_param params[] = {
			{ .type = HL_PV_HLINT, .i32_val = (-2147483647 - 1) },
			{ .type = HL_PV_HLUINT, .u32_val = 0xffffffffU },
			{ .type = HL_PV_HLLONG, .i64_val = (-9223372036854775807LL - 1) },
			{ .type = HL_PV_HLULONG, .u64_val = ~0ULL },
			{ .type = HL_PV_HLSTRING, .str = { "hello", 5 } },
			{ .type = HL_PV_HLVECBYTES, .vec = { (const __u8 *)"first", 5 } },
			{ .type = HL_PV_HLVECBYTES, .vec = { NULL, 0 } },
			{ .type = HL_PV_HLVECBYTES, .vec = { (const __u8 *)"last", 4 } },
		};
		len = hl_pc_encode_call(buf, sizeof(buf), "HostCall", HL_RT_VECBYTES,
					params, 8, &control);
		assert(len);
		printf("%llu\n", (unsigned long long)control);
		assert(fwrite(buf, 1, len, stdout) == len);
		assert(!hl_pc_encode_call(buf, len - 1, "HostCall", HL_RT_VECBYTES,
					  params, 8, &control));
		assert(!hl_pc_encode_call(buf, sizeof(buf), "x", 0, params, -1, &control));
		return 0;
	}
	if (!strcmp(argv[1], "log")) {
		len = hl_pc_encode_log(buf, sizeof(buf), HL_LOG_WARNING,
				      "kernel", "test.c", 300, "bad\xff", 4);
		assert(len && fwrite(buf, 1, len, stdout) == len);
		return 0;
	}
	/* The native test harness prefixes its input with a control length,
	 * independent of the production transport's MsgHeader.
	 */
	{
		unsigned char prefix[4];

		assert(fread(prefix, 1, 4, stdin) == 4);
		control = prefix[0] | ((__u64)prefix[1] << 8) |
			  ((__u64)prefix[2] << 16) | ((__u64)prefix[3] << 24);
	}
	len = fread(buf, 1, sizeof(buf), stdin);
	assert(!ferror(stdin) && feof(stdin));
	if (!strcmp(argv[1], "result")) {
		struct hl_pc_result result;

		if (hl_pc_decode_result(buf, len, control, &result))
			return 1;
		printf("%d %u %llu %llu\n", result.error, result.tag,
		       (unsigned long long)result.value,
		       (unsigned long long)result.len);
		if (result.data)
			assert(fwrite(result.data, 1, result.len, stdout) == result.len);
		return 0;
	}
	if (!strcmp(argv[1], "call")) {
		struct hl_pc_call call;
		const char *name, *arg;
		size_t name_len, arg_len;

		if (hl_pc_decode_call(buf, len, control, &call))
			return 1;
		name = fc_function_name(buf, len, &name_len);
		arg = fc_arg0_string(buf, len, &arg_len);
		if (!name || !arg)
			return 1;
		assert(name_len == call.name_len && !memcmp(name, call.name, name_len));
		assert(hl_pc_name_is(&call, "Call"));
		assert(hl_pc_env_version(&call) == ~0ULL);
		assert(fwrite(arg, 1, arg_len, stdout) == arg_len);
		return 0;
	}
	if (!strcmp(argv[1], "driver")) {
		size_t name_len;

		return fc_function_name(buf, len, &name_len) ? 0 : 1;
	}
	return 2;
}
