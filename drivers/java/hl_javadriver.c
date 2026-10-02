/*
 * hl_javadriver — Java runtime driver for Hyperlight.
 *
 * Spawns a persistent JVM during boot via vfork+exec, running the
 * dispatch server (hyperlight.Dispatch), which hands each call's source
 * to JShell in the same JVM: statements, expressions and declarations,
 * compiled by javac in the guest and kept across calls.
 *
 * Flow:
 *   boot (evolve):
 *     main() → hl_driver_init(): open /dev/hlcall
 *            → java_spawn(): pipes, vfork + exec("/opt/java/bin/java",
 *              ..., fd_in, fd_out), then the child's ready byte (blocks,
 *              scheduler switches to the JVM, JShell and javac warm up,
 *              child signals)
 *            → hl_driver_run(): block in read() on the call queue
 *
 *   host: call("Exec", <Java source>)
 *     read() returns the call → java_dispatch(fc, fc_len)
 *              → send it down the pipe (hl_child.h)
 *              → read the status and result (blocks, scheduler switches
 *                to the JVM, JShell compiles and runs it, writes them),
 *                making any host function call it asks for on the way
 *              → back into read(): the call is done
 *
 *   A guest function call ("Call") names a static method of a snippet
 *   run earlier; the child calls it by reflection.
 *
 * JShell is warmed up during boot so snapshots capture the initialized
 * state — dispatches after restore only compile their own code.
 *
 * The JVM stays alive across dispatches.  System.exit() ends it, which
 * is the runtime: the call ends with that exit status, and the next call
 * starts a fresh one, warm-up included, while the memory allows: until
 * the kernel returns an exited process's memory, each one needs the room
 * for another runtime.
 */

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>
#include <sys/sysinfo.h>

#include "../hl_fc.h"
#include "../hl_env.h"
#include "../hl_driver.h"
#include "../hl_child.h"

/* ── State ─────────────────────────────────────────────────────── */

static pid_t g_java_pid = -1;
static int g_pipe_to_java = -1;    /* parent writes payload here */
static int g_pipe_from_java = -1;  /* parent reads the status here */

/* The JVM's environment: the driver's at boot.  hl_env_refresh() setenv()s
 * the host's variables into the driver for each call; a JVM started again
 * after System.exit() must not take them as its own (LD_LIBRARY_PATH steers
 * the launcher, JAVA_TOOL_OPTIONS the JVM).  The dispatcher applies them per
 * call, as it does for the first JVM. */
static char **g_java_envp;

/* What the boot JVM took, warm-up included, in MiB: the kernel does not
 * return an exited process's memory, so each JVM after System.exit() takes
 * that much more, and one started without the room crashes the guest. */
static unsigned long g_java_mb;
extern char **environ;

/* ── Environment visitor ───────────────────────────────────────── */

/*
 * The persistent JVM was spawned at boot and does not see the parent's
 * setenv(), so each call carries the host's variables as KEY NUL VALUE
 * NUL pairs, which the child sets before the call runs.
 */
static void java_env_visitor(const char *key, const char *val, void *ctx)
{
	struct hl_strbuf *env = ctx;

	hl_strbuf_put(env, key, strlen(key) + 1);
	hl_strbuf_put(env, val, strlen(val) + 1);
}

/* ── The child ─────────────────────────────────────────────────── */

static unsigned long java_free_mb(void)
{
	struct sysinfo si;

	if (sysinfo(&si) < 0)
		return 0;
	return (unsigned long)((uint64_t)si.freeram * si.mem_unit >> 20);
}

/* Copy the environment as it is now into g_java_envp, strings and all:
 * setenv() may free a string it replaces later. */
static int java_env_snapshot(void)
{
	size_t n = 0;

	while (environ[n])
		n++;
	g_java_envp = calloc(n + 1, sizeof(*g_java_envp));
	if (!g_java_envp)
		return -1;
	for (size_t i = 0; i < n; i++)
		if (!(g_java_envp[i] = strdup(environ[i])))
			return -1;
	return 0;
}

/* The child is gone: drop our pipe ends and collect its exit status,
 * which is the status of the call it ended under. */
static int java_reap(void)
{
	int status;

	close(g_pipe_to_java);
	close(g_pipe_from_java);
	g_pipe_to_java = g_pipe_from_java = -1;
	status = hl_wait_status(g_java_pid);
	g_java_pid = -1;
	return status;
}

/* The JVM's command line: its options are in /app/jvm.args, where the
 * image build reads them too (drivers/java/jvm.args); the archive is the
 * build's, of the classes the dispatcher loads while it warms up. */
static char *const java_argv[] = {
	"/opt/java/bin/java",
	"@/app/jvm.args",
	NULL, /* -XX:MaxRAM: the memory free now (java_spawn) */
	"-XX:SharedArchiveFile=/app/hl-java.jsa",
	"-jar", "/app/hl-java.jar",
	NULL, NULL, /* the pipe fds */
	NULL,
};
#define JAVA_ARGC (sizeof(java_argv) / sizeof(java_argv[0]) - 3)

/*
 * Start the JVM: pipes, vfork + exec with the pipe fds as its last
 * arguments, then its ready byte, which comes once JShell is warm.
 * Called at boot, and again for the call after one that ended the child.
 */
static int java_spawn(void)
{
	int pipe_code[2];  /* [0]=read, [1]=write */
	int pipe_ack[2];
	char fd_in_str[16], fd_out_str[16], max_ram[32];
	char *argv[sizeof(java_argv) / sizeof(java_argv[0])];
	pid_t pid;
	char ready;

	if (pipe(pipe_code) < 0) {
		fprintf(stderr, "hl_javadriver: pipe() failed\n");
		return -1;
	}
	if (pipe(pipe_ack) < 0) {
		fprintf(stderr, "hl_javadriver: pipe() failed\n");
		close(pipe_code[0]);
		close(pipe_code[1]);
		return -1;
	}
	/* The child, and anything it runs, gets only its own two ends. */
	fcntl(pipe_code[1], F_SETFD, FD_CLOEXEC);
	fcntl(pipe_ack[0], F_SETFD, FD_CLOEXEC);
	snprintf(fd_in_str, sizeof(fd_in_str), "%d", pipe_code[0]);
	snprintf(fd_out_str, sizeof(fd_out_str), "%d", pipe_ack[1]);
	/* The heap is a share (jvm.args) of what the JVM takes for the guest's
	 * memory: what is free, not the total, which a JVM started after
	 * System.exit() shares with the ones before it, whose memory the
	 * kernel keeps.  Sized from the total, its heap would outgrow the
	 * memory left, and an allocation fault the guest instead of throwing
	 * OutOfMemoryError. */
	unsigned long free_mb = java_free_mb();

	/* A JVM after the first also needs room for itself, what the first
	 * took: its heap's share is of the rest. */
	snprintf(max_ram, sizeof(max_ram), "-XX:MaxRAM=%luM",
		 g_java_mb && free_mb > g_java_mb ? free_mb - g_java_mb : free_mb);
	memcpy(argv, java_argv, sizeof(argv));
	argv[2] = max_ram;
	argv[JAVA_ARGC] = fd_in_str;
	argv[JAVA_ARGC + 1] = fd_out_str;

	pid = vfork();
	if (pid < 0) {
		fprintf(stderr, "hl_javadriver: vfork() failed\n");
		close(pipe_code[0]);
		close(pipe_code[1]);
		close(pipe_ack[0]);
		close(pipe_ack[1]);
		return -1;
	}
	if (pid == 0) {
		/* Child — only exec or _exit allowed after vfork */
		execve("/opt/java/bin/java", argv, g_java_envp);
		_exit(127);
	}

	/* Parent — close the child's pipe ends */
	close(pipe_code[0]);
	close(pipe_ack[1]);
	g_java_pid = pid;
	g_pipe_to_java = pipe_code[1];
	g_pipe_from_java = pipe_ack[0];

	/* Wait for the child to signal ready.  This blocks, the scheduler
	 * switches to the child, the JVM starts up, the dispatch server
	 * writes the ready byte. */
	if (read(g_pipe_from_java, &ready, 1) != 1) {
		fprintf(stderr, "hl_javadriver: the Java dispatch server failed to start\n");
		java_reap();
		return -1;
	}
	return 0;
}

/* ── Dispatch callback ─────────────────────────────────────────── */

static int java_dispatch(const uint8_t *fc, size_t fc_len)
{
	/* Extract the Java source from the FunctionCall FlatBuffer. */
	size_t code_len;
	const char *code = fc_arg0_string(fc, fc_len, &code_len);
	if (!code)
		return -1;

	/* Guest command (--guest-exec / autonomous): run the named .java file
	 * from the initrd (the command's first token; args are not forwarded
	 * to Java yet), or the conventional /entrypoint.java when empty. */
	char *auton_buf = NULL;
	size_t gx_len = code_len;
	const char *gx = fc_name_is(fc, fc_len, "GuestExec") ? code : NULL;
	if (gx) {
		char pathbuf[4096];
		char *av[64];
		const char *src = gx_len ? gx : "/entrypoint.java";
		size_t n = gx_len ? gx_len : strlen("/entrypoint.java");
		if (n >= sizeof(pathbuf))
			return -1;
		memcpy(pathbuf, src, n);
		pathbuf[n] = '\0';
		hl_split_ws(pathbuf, av, 64);
		const char *path = av[0] ? av[0] : "/entrypoint.java";
		FILE *ef = fopen(path, "rb");
		if (!ef) {
			fprintf(stderr, "hl: no %s in rootfs; nothing to run\n",
				path);
			fflush(stderr);
			return 0;
		}
		long sz = -1;
		if (fseek(ef, 0, SEEK_END) == 0)
			sz = ftell(ef);
		if (sz < 0 || fseek(ef, 0, SEEK_SET) != 0) {
			fclose(ef);
			return -1;
		}
		auton_buf = malloc((size_t)sz + 1);
		if (!auton_buf) {
			fclose(ef);
			return -1;
		}
		size_t rd = fread(auton_buf, 1, (size_t)sz, ef);
		fclose(ef);
		auton_buf[rd] = '\0';
		code = auton_buf;
		code_len = rd;
	}

	struct hl_strbuf env = { 0 };
	const char *input = NULL;
	size_t input_len = 0;
	int is_call = fc_name_is(fc, fc_len, "Call");
	int rc = -1;

	if (is_call) {
		input = fc_arg_string(fc, fc_len, 1, &input_len);
		if (!code_len || !input)
			goto out;
	}

	/* The call after one that ended the child starts a new one. */
	if (g_java_pid < 0) {
		/* Twice what the first took: room for the JVM, and a heap of
		 * half that (java_spawn). */
		unsigned long free_mb = java_free_mb();

		if (free_mb < g_java_mb * 2) {
			fprintf(stderr, "hl_javadriver: %lu MiB left, too little for a fresh JVM "
				"(the first took %lu): an exited JVM's memory is not given "
				"back, so restore the sandbox from a snapshot or give it "
				"more memory\n", free_mb, g_java_mb);
			fflush(stderr);
			goto out;
		}
		if (java_spawn() < 0)
			goto out;
	}

	/* Refresh env vars: setenv() here, KEY/VALUE pairs for the child. */
	hl_env_refresh(java_env_visitor, &env);
	if (env.err) {
		fprintf(stderr, "hl_javadriver: out of memory for the environment\n");
		fflush(stderr);
		goto out;
	}

	/* Send the call (hl_child.h), then wait for its status.  The read
	 * blocks and yields to the cooperative scheduler, which switches to
	 * the JVM. */
	if (hl_child_send(g_pipe_to_java, is_call, env.buf, env.len, code, code_len,
			  input, input_len) < 0) {
		/* The child had ended before the call reached it, so the call
		 * did not run and fails; the next one starts a fresh child. */
		fprintf(stderr, "hl_javadriver: the JVM had exited with status %d\n",
			java_reap());
		fflush(stderr);
		goto out;
	}
	rc = hl_child_await(g_pipe_to_java, g_pipe_from_java, "hl_javadriver");
	if (rc < 0) {
		/* The child ended under the call (System.exit, a crash):
		 * its exit status is the call's. */
		rc = java_reap();
		goto out;
	}

out:
	hl_strbuf_free(&env);
	free(auton_buf);
	return rc;
}

/* ── Entry point ───────────────────────────────────────────────── */

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	if (hl_driver_init("hl_javadriver"))
		return 1;

	/* On musl the java launcher re-executes itself unless this starts
	 * with the directories it would set it to, finding itself for that
	 * through /proc/self/exe, which the guest lacks, or argv[0].  Set,
	 * it also finds libjli, which it otherwise does through $ORIGIN,
	 * which musl resolves through /proc too. */
	setenv("LD_LIBRARY_PATH", "/opt/java/lib/server:/opt/java/lib:/opt/java/../lib", 1);
	if (java_env_snapshot() < 0) {
		fprintf(stderr, "hl_javadriver: out of memory for the environment\n");
		return 1;
	}

	/* A write to a child that is gone is an error to handle, not a
	 * signal to die of. */
	signal(SIGPIPE, SIG_IGN);

	unsigned long before = java_free_mb();

	if (java_spawn() < 0)
		return 1;
	unsigned long after = java_free_mb();

	g_java_mb = before > after ? before - after : 0;
	hl_driver_serve_calls();

	/* Serve named calls from the kernel's queue; never returns */
	hl_driver_run(java_dispatch);
}
