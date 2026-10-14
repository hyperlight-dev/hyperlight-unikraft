/*
 * A vfork() parent resumes when its own child execs or exits, not when
 * another thread it made exits while the child runs: here a thread that
 * ends 100 ms into a vfork child that runs for 300 ms.  (A child of
 * vfork() runs on the parent's stack, and an early resume crashed the
 * guest; this one has a stack of its own, so an early resume shows only
 * as a child not yet done.)
 */
#define _GNU_SOURCE
#include <pthread.h>
#include <sched.h>
#include <signal.h>
#include <stdio.h>
#include <sys/syscall.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static char child_stack[64 * 1024];
static volatile int child_done;

static int child(void *arg)
{
	struct timespec ts = { 0, 300 * 1000 * 1000 };

	(void)arg;
	syscall(SYS_nanosleep, &ts, NULL);
	child_done = 1;
	syscall(SYS_exit, 0);
	return 0;
}

static void *short_lived(void *arg)
{
	(void)arg;
	usleep(100 * 1000);
	return NULL;
}

int main(void)
{
	pthread_t thread;
	int resumed_after, status;
	pid_t pid;

	if (pthread_create(&thread, NULL, short_lived, NULL)) {
		printf("FAIL: pthread_create\n");
		return 1;
	}
	pid = clone(child, child_stack + sizeof(child_stack),
		    SIGCHLD | CLONE_VM | CLONE_VFORK, NULL);
	resumed_after = child_done;
	if (pid < 0 || waitpid(pid, &status, 0) != pid) {
		printf("FAIL: clone or waitpid\n");
		return 1;
	}
	pthread_join(thread, NULL);
	if (!resumed_after) {
		printf("FAIL: the vfork parent resumed before its child was done\n");
		return 1;
	}
	printf("vfork wake: all checks pass\n");
	return 0;
}
