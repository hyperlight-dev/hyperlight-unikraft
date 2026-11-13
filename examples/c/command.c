/* SPDX-License-Identifier: Apache-2.0
 * Copyright 2026 The Hyperlight Authors.
 */
#define _GNU_SOURCE
#include <assert.h>
#include <errno.h>
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

static void host_call(const char *name, const char *args)
{
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
	} request = { name, strlen(name), args, strlen(args), reply, sizeof(reply), 0 };
	int rc;
	for (;;) {
		rc = ioctl(fd, _IOWR('H', 3, struct request), &request);
		if (!rc || errno != EINTR) break;
		assert(write(1, "interrupted\n", 12) == 12);
	}
	assert(rc == 0);
	assert(request.len && reply[0] == 0);
	assert(write(1, reply + 1, request.len - 1) == (ssize_t)request.len - 1);
	close(fd);
}

int main(int argc, char **argv)
{
	assert(argc >= 2);
	if (!strcmp(argv[1], "park-socket") || !strcmp(argv[1], "park-listener")) {
		int fd = socket(AF_INET, SOCK_STREAM, 0);
		assert(fd >= 0);
		struct sockaddr_in address = {
			.sin_family = AF_INET,
			.sin_port = htons(atoi(argv[2])),
			.sin_addr.s_addr = htonl(INADDR_LOOPBACK)
		};
		int listening = !strcmp(argv[1], "park-listener");
		if (listening) {
			assert(bind(fd, (struct sockaddr *)&address, sizeof(address)) == 0);
			assert(listen(fd, 4) == 0);
		} else {
			assert(connect(fd, (struct sockaddr *)&address, sizeof(address)) == 0);
		}
		assert(write(1, "ready\n", 6) == 6);
		if (listening) {
			host_call("gate", "[]");
			int peer = accept(fd, NULL, NULL);
			assert(peer >= 0);
			close(peer);
		} else {
			char byte;
			assert(read(fd, &byte, 1) == 0);
		}
		close(fd);
		assert(write(1, "closed\n", 7) == 7);
		return 0;
	}
	if (!strcmp(argv[1], "net-tree")) {
		int fd = socket(AF_INET, SOCK_STREAM, 0);
		assert(fd >= 0);
		struct sockaddr_in address = {
			.sin_family = AF_INET,
			.sin_port = htons(atoi(argv[2])),
			.sin_addr.s_addr = htonl(INADDR_LOOPBACK)
		};
		assert(connect(fd, (struct sockaddr *)&address, sizeof(address)) == 0);
		int done[2];
		assert(pipe(done) == 0);
		char number[32], completion[32];
		snprintf(number, sizeof(number), "%d", fd);
		snprintf(completion, sizeof(completion), "%d", done[1]);
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			close(done[0]);
			execl(argv[0], argv[0], "socket-child", number, completion, (char *)NULL);
			_exit(127);
		}
		close(fd);
		close(done[1]);
		char byte;
		assert(read(done[0], &byte, 1) == 1);
		close(done[0]);
		return 9;
	}
	if (!strcmp(argv[1], "socket-child")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "socket-leaf", argv[2], argv[3], (char *)NULL);
			_exit(127);
		}
		close(atoi(argv[2]));
		close(atoi(argv[3]));
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
		assert(fflush(stdout) == 0);
		assert(write(atoi(argv[3]), "!", 1) == 1);
		close(atoi(argv[3]));
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
	if (!strcmp(argv[1], "sleep-tree")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "sleep", argv[2], "descendant", (char *)NULL);
			_exit(127);
		}
		int status;
		assert(waitpid(child, &status, 0) == child);
		assert(WIFEXITED(status) && WEXITSTATUS(status) == 0);
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
	if (!strcmp(argv[1], "kill-tree")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "kill-middle", (char *)NULL);
			_exit(127);
		}
		if (!strcmp(argv[2], "live")) pause_ms(60000);
		else host_call("root-gate", "[]");
		return 7;
	}
	if (!strcmp(argv[1], "kill-middle")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "detached-host", "[]", (char *)NULL);
			_exit(127);
		}
		return 0;
	}
	if (!strcmp(argv[1], "host") || !strcmp(argv[1], "detached-host")) {
		assert(argc == 3);
		if (!strcmp(argv[1], "detached-host")) {
			assert(setsid() >= 0);
			assert(write(1, "before kill\n", 12) == 12);
		}
		host_call("gate", argv[2]);
		return 0;
	}
	if (!strcmp(argv[1], "tree-host")) {
		pid_t child = vfork();
		assert(child >= 0);
		if (!child) {
			execl(argv[0], argv[0], "host", "[]", (char *)NULL);
			_exit(127);
		}
		return 7;
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
