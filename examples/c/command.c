/* SPDX-License-Identifier: Apache-2.0
 * Copyright 2026 The Hyperlight Authors.
 */
#define _GNU_SOURCE
#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
#include <stdint.h>
#include <arpa/inet.h>
#include <sys/socket.h>

static void pause_ms(unsigned long ms)
{
	struct timespec duration = { ms / 1000, (ms % 1000) * 1000000 };
	while (nanosleep(&duration, &duration)) {}
}

int main(int argc, char **argv)
{
	assert(argc >= 2);
	if (!strcmp(argv[1], "net-tree")) {
		int fd = socket(AF_INET, SOCK_STREAM, 0);
		assert(fd >= 0);
		struct sockaddr_in address = {
			.sin_family = AF_INET,
			.sin_port = htons(atoi(argv[2])),
			.sin_addr.s_addr = htonl(INADDR_LOOPBACK)
		};
		assert(connect(fd, (struct sockaddr *)&address, sizeof(address)) == 0);
		char number[32];
		snprintf(number, sizeof(number), "%d", fd);
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "socket-child", number, (char *)NULL);
			_exit(127);
		}
		close(fd);
		return 9;
	}
	if (!strcmp(argv[1], "socket-child")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "socket-leaf", argv[2], (char *)NULL);
			_exit(127);
		}
		close(atoi(argv[2]));
		return 0;
	}
	if (!strcmp(argv[1], "socket-leaf")) {
		int fd = atoi(argv[2]);
		char byte;
		pause_ms(30);
		assert(read(fd, &byte, 1) == 1);
		assert(write(fd, &byte, 1) == 1);
		close(fd);
		printf("socket-descendant\n");
		return 0;
	}
	if (!strcmp(argv[1], "args")) {
		char cwd[1024];
		assert(getcwd(cwd, sizeof(cwd)));
		printf("cwd=%s\nenv=%s\n", cwd, getenv("COMMAND_TEST"));
		for (int i = 2; i < argc; i++) printf("arg=%s\n", argv[i]);
		fprintf(stderr, "stderr\n");
		return 17;
	}
	if (!strcmp(argv[1], "sleep")) {
		assert(argc == 4);
		pause_ms(strtoul(argv[2], NULL, 10));
		printf("%s\n", argv[3]);
		return 0;
	}
	if (!strcmp(argv[1], "echo")) {
		char buffer[4096];
		ssize_t n;
		while ((n = read(0, buffer, sizeof(buffer))) > 0) {
			ssize_t sent = 0;
			while (sent < n) {
				ssize_t count = write(1, buffer + sent, n - sent);
				assert(count > 0);
				sent += count;
			}
		}
		assert(n == 0);
		return 0;
	}
	if (!strcmp(argv[1], "host")) {
		assert(argc == 3);
		int fd = open("/dev/hlcall", O_RDWR | O_CLOEXEC);
		assert(fd >= 0);
		uint8_t reply[4096];
		struct request {
			const char *name;
			uint64_t name_len;
			const char *args;
			uint64_t args_len;
			uint8_t *out;
			uint64_t cap, len;
		} request = { "gate", 4, argv[2], strlen(argv[2]), reply, sizeof(reply), 0 };
		assert(ioctl(fd, _IOWR('H', 3, struct request), &request) == 0);
		assert(request.len && reply[0] == 0);
		assert(write(1, reply + 1, request.len - 1) == (ssize_t)request.len - 1);
		close(fd);
		return 0;
	}
	if (!strcmp(argv[1], "tree")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "sleep", "80", "descendant", (char *)NULL);
			_exit(127);
		}
		return 7;
	}
	if (!strcmp(argv[1], "large")) {
		char bytes[4096];
		memset(bytes, 'x', sizeof(bytes));
		for (int i = 0; i < 80; i++) {
			assert(write(1, bytes, sizeof(bytes)) == sizeof(bytes));
			assert(write(2, bytes, sizeof(bytes)) == sizeof(bytes));
		}
		return 0;
	}
	return 99;
}
