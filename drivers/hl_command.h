/* SPDX-License-Identifier: Apache-2.0
 * Copyright 2026 The Hyperlight Authors.
 */
#ifndef HL_COMMAND_H
#define HL_COMMAND_H

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
#define HLCALL_IOC_EXECUTION_KILL _IOW('H', 9, uint64_t)
struct hl_command_stdio { uint64_t stream, fd; };
#define HLCALL_IOC_COMMAND_STDIO _IOWR('H', 10, struct hl_command_stdio)

static int command_null[2];
struct command {
	uint64_t id;
	char *storage, *cwd;
	char **argv, **env;
	unsigned int stdio;
};

static void command_event(int control, int kind, int value)
{
	struct hl_command_event event = { kind, value };

	if (ioctl(control, HLCALL_IOC_COMMAND_EVENT, &event)) {
		fprintf(stderr, "hl_command: cannot report event: %s\n", strerror(errno));
		_exit(125);
	}
}

static void command_free(struct command *command)
{
	free(command->argv);
	free(command->env);
	free(command->storage);
	free(command);
}

static void command_run(struct command *command)
{
	int control = g_hl_call_fd;
	int childfd[3] = { -1, -1, -1 };
	volatile int exec_error = 0;
	pid_t pid = -1;
	int error = 0;

	if (ioctl(control, HLCALL_IOC_EXECUTION, &command->id)) {
		fprintf(stderr, "hl_command: cannot attach execution: %s\n", strerror(errno));
		_exit(125);
	}
	for (int i = 0; i < 3; i++) {
		if (command->stdio & (1u << i)) {
			struct hl_command_stdio io = { .stream = i };

			if (ioctl(control, HLCALL_IOC_COMMAND_STDIO, &io)) {
				error = errno;
				goto done;
			}
			childfd[i] = io.fd;
		} else {
			childfd[i] = command_null[i != 0];
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
		if (command->stdio & (1u << i)) close(childfd[i]);
		childfd[i] = -1;
	}
	error = exec_error;
done:
	for (int i = 0; i < 3; i++) {
		if (childfd[i] >= 0 && (command->stdio & (1u << i)))
			close(childfd[i]);
	}
	command_event(control, 0, error ? -error : pid);
	command_free(command);
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
		if (errno || !id || (size_t)(end - data) != len) return -1;
		return ioctl(g_hl_call_fd, HLCALL_IOC_EXECUTION_KILL, &id) ?
			-errno : 0;
	}
	struct command *command = calloc(1, sizeof(*command));
	uint64_t version, argc, envc, stdio;

	if (!command || !data || !len) { free(command); return -1; }
	command->storage = malloc(len);
	if (!command->storage) { command_free(command); return -1; }
	memcpy(command->storage, data, len);
	char *cursor = command->storage, *end = cursor + len;

	if (command_number(&cursor, end, &version) || version != 7 ||
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
	command_run(command);
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
	if (version != 7) return -1;
	command_null[0] = open("/dev/null", O_RDONLY | O_CLOEXEC);
	command_null[1] = open("/dev/null", O_WRONLY | O_CLOEXEC);
	if (command_null[0] < 0 || command_null[1] < 0) return -1;
	g_hl_command_dispatch = command_dispatch;
	return 0;
}
#endif
