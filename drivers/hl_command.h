/* SPDX-License-Identifier: Apache-2.0
 * Copyright 2026 The Hyperlight Authors.
 */
#ifndef HL_COMMAND_H
#define HL_COMMAND_H

#include <pthread.h>
#include <signal.h>

#define HLCALL_IOC_EXECUTION _IOW('H', 5, uint64_t)
#define HLCALL_IOC_COMMAND_DRIVER _IOR('H', 6, uint64_t)
struct hl_command_event { int32_t kind, value; };
struct hl_command_io {
	uint64_t stream;
	void *buf;
	uint64_t len, done;
};
#define HLCALL_IOC_COMMAND_EVENT _IOW('H', 7, struct hl_command_event)
#define HLCALL_IOC_COMMAND_IO _IOWR('H', 8, struct hl_command_io)

struct command_pipe { int fd, stream; };
struct command {
	struct command *next;
	pid_t pid;
	uint64_t id;
	char *storage, *cwd;
	char **argv, **env;
	unsigned int stdio;
};
static struct command *commands;

static void command_event(int control, int kind, int value)
{
	struct hl_command_event event = { kind, value };

	if (ioctl(control, HLCALL_IOC_COMMAND_EVENT, &event)) {
		fprintf(stderr, "hl_command: cannot report event: %s\n", strerror(errno));
		_exit(125);
	}
}

static void *command_pipe(void *arg)
{
	struct command_pipe *pipe = arg;
	int control = open(HL_CALL_DEVICE, O_RDWR | O_CLOEXEC);
	if (control < 0) { perror("hl_command: control"); _exit(125); }
	char bytes[4096];
	struct hl_command_io io = {
		.stream = pipe->stream, .buf = bytes, .len = sizeof(bytes)
	};
	int error = 0;
	sigset_t blocked;

	sigemptyset(&blocked);
	sigaddset(&blocked, SIGPIPE);
	error = pthread_sigmask(SIG_BLOCK, &blocked, NULL);
	if (error) goto done;

	for (;;) {
		ssize_t count;

		if (pipe->stream) {
			do { count = read(pipe->fd, bytes, sizeof(bytes)); }
			while (count < 0 && errno == EINTR);
			if (count < 0) { error = errno; break; }
			io.len = count;
			if (ioctl(control, HLCALL_IOC_COMMAND_IO, &io)) {
				if (errno != EPIPE) error = errno;
				break;
			}
			if (!count) break;
		} else {
			if (ioctl(control, HLCALL_IOC_COMMAND_IO, &io)) {
				error = errno; break;
			}
			if (!io.done) break;
			size_t sent = 0;

			while (sent < io.done) {
				count = write(pipe->fd, bytes + sent, io.done - sent);
				if (count < 0 && errno == EINTR) continue;
				if (count <= 0) {
					if (errno != EPIPE) error = errno;
					goto done;
				}
				sent += count;
			}
		}
	}
done:
	close(pipe->fd);
	if (!pipe->stream) command_event(control, 3, 0);
	if (error) command_event(control, 2, error);
	close(control);
	return NULL;
}

static void command_free(struct command *command)
{
	free(command->argv);
	free(command->env);
	free(command->storage);
	free(command);
}

static void *command_run(void *arg)
{
	struct command *command = arg;
	int control = open(HL_CALL_DEVICE, O_RDWR | O_CLOEXEC);
	if (control < 0) { perror("hl_command: control"); _exit(125); }
	int childfd[3] = { -1, -1, -1 };
	struct command_pipe pipes[3] = { { -1, 0 }, { -1, 1 }, { -1, 2 } };
	pthread_t threads[3];
	int started[3] = { 0, 0, 0 };
	volatile int exec_error = 0;
	pid_t pid = -1;
	int error = 0, status;

	if (ioctl(control, HLCALL_IOC_EXECUTION, &command->id)) {
		fprintf(stderr, "hl_command: cannot attach execution: %s\n", strerror(errno));
		_exit(125);
	}
	for (int i = 0; i < 3; i++) {
		if (command->stdio & (1u << i)) {
			int fds[2];

			if (pipe2(fds, O_CLOEXEC)) { error = errno; goto done; }
			childfd[i] = fds[i == 0 ? 0 : 1];
			pipes[i].fd = fds[i == 0 ? 1 : 0];
		} else {
			childfd[i] = open("/dev/null", (i ? O_WRONLY : O_RDONLY) | O_CLOEXEC);
			if (childfd[i] < 0) { error = errno; goto done; }
		}
	}
	pid = vfork();
	if (pid == 0) {
		for (int i = 0; i < 3; i++) {
			if (dup2(childfd[i], i) < 0) { exec_error = errno; _exit(127); }
		}
		if (*command->cwd && chdir(command->cwd)) { exec_error = errno; _exit(127); }
		execve(command->argv[0], command->argv, command->env);
		exec_error = errno;
		_exit(127);
	}
	if (pid < 0) { error = errno; goto done; }
	for (int i = 0; i < 3; i++) {
		close(childfd[i]);
		childfd[i] = -1;
	}
	if (exec_error) {
		error = exec_error;
		while (waitpid(pid, &status, 0) < 0 && errno == EINTR) {}
		goto done;
	}
	command_event(control, 0, pid);
	command->pid = pid;
	for (int i = 0; i < 3; i++) {
		if (pipes[i].fd < 0) continue;
		error = pthread_create(&threads[i], NULL, command_pipe, &pipes[i]);
		if (error) {
			kill(pid, SIGKILL);
			break;
		}
		started[i] = 1;
	}
	while (waitpid(pid, &status, 0) < 0) {
		if (errno == EINTR) continue;
		error = errno;
		goto done;
	}
	command_event(control, 1, status);
	command->pid = 0;
done:
	if (error) command_event(control, 0, -error);
	for (int i = 0; i < 3; i++) {
		if (childfd[i] >= 0) close(childfd[i]);
		if (started[i]) pthread_join(threads[i], NULL);
		else if (pipes[i].fd >= 0) close(pipes[i].fd);
	}
	close(control);
	struct command **link = &commands;
	while (*link && *link != command) link = &(*link)->next;
	if (*link) *link = command->next;
	command_free(command);
	return NULL;
}

static char *command_field(char **cursor, char *end)
{
	char *start = *cursor;
	char *nul = memchr(start, 0, end - start);

	if (!nul) return NULL;
	*cursor = nul + 1;
	return start;
}

static int command_number(char **cursor, char *end, uint64_t *value)
{
	char *field = command_field(cursor, end), *tail;

	if (!field || !*field || *field < '0' || *field > '9') return -1;
	errno = 0;
	*value = strtoull(field, &tail, 10);
	return errno || *tail ? -1 : 0;
}

static int command_dispatch(const uint8_t *fc, size_t fc_len)
{
	size_t len;
	const char *data = fc_arg0_string(fc, fc_len, &len);
	if (fc_name_is(fc, fc_len, "CommandKill")) {
		char *end;
		uint64_t id;

		if (!data || !len || data[0] < '0' || data[0] > '9') return -1;
		errno = 0;
		id = strtoull(data, &end, 10);
		if (errno || (size_t)(end - data) != len) return -1;
		for (struct command *entry = commands; entry; entry = entry->next)
			if (entry->id == id && entry->pid > 0) {
				if (kill(entry->pid, SIGKILL) && errno != ESRCH)
					return -errno;
				break;
			}
		return 0;
	}
	struct command *command = calloc(1, sizeof(*command));
	uint64_t version, argc, envc, stdio;
	pthread_t thread;

	if (!command || !data || !len) { free(command); return -1; }
	command->storage = malloc(len);
	if (!command->storage) { command_free(command); return -1; }
	memcpy(command->storage, data, len);
	char *cursor = command->storage, *end = cursor + len;

	if (command_number(&cursor, end, &version) || version != 1 ||
	    command_number(&cursor, end, &command->id) || !command->id ||
	    command_number(&cursor, end, &argc) || argc > len ||
	    command_number(&cursor, end, &envc) || envc > len ||
	    command_number(&cursor, end, &stdio) || stdio > 7)
		goto invalid;
	command->stdio = stdio;
	command->cwd = command_field(&cursor, end);
	command->argv = calloc(argc + 2, sizeof(char *));
	command->env = calloc(envc + 1, sizeof(char *));
	if (!command->cwd || !command->argv || !command->env) goto invalid;
	for (uint64_t i = 0; i <= argc; i++)
		if (!(command->argv[i] = command_field(&cursor, end))) goto invalid;
	for (uint64_t i = 0; i < envc; i++)
		if (!(command->env[i] = command_field(&cursor, end))) goto invalid;
	if (cursor != end || command->argv[0][0] != '/') goto invalid;
	command->next = commands;
	commands = command;
	if (pthread_create(&thread, NULL, command_run, command)) {
		commands = command->next;
		goto invalid;
	}
	pthread_detach(thread);
	return 0;
invalid:
	command_free(command);
	return -1;
}

static int command_init(void)
{
	uint64_t version;

	if (ioctl(g_hl_call_fd, HLCALL_IOC_COMMAND_DRIVER, &version)) {
		if (errno == ENOTTY || errno == EOPNOTSUPP) return 0;
		fprintf(stderr, "hl_command: negotiation failed: %s\n", strerror(errno));
		return -1;
	}
	if (version != 1) return -1;
	g_hl_command_dispatch = command_dispatch;
	return 0;
}
#endif
