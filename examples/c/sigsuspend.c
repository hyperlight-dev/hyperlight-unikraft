/*
 * sigsuspend() as Linux defines it, used the usual way: block a signal,
 * then `while (!got) sigsuspend(&old)`.  The handler runs before the old
 * mask, which blocks the signal, comes back, so one call does it, whether
 * the signal was queued before the call or arrives during it; and the old
 * mask is back after it, though the handler made a system call of its
 * own.  The loops are bounded and an alarm ends a sigsuspend() nothing
 * wakes: a kernel that gets it wrong fails the check rather than spinning
 * or sleeping for good.
 */
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static volatile sig_atomic_t got, timed_out;
static int failures;

static void handler(int signum)
{
	/* A system call from the handler: its own way out of the kernel
	 * must not put the old mask back from under this one.
	 */
	(void)getppid();
	got = signum;
}

static void on_alarm(int signum)
{
	(void)signum;
	timed_out = 1;
}

static void *send_later(void *arg)
{
	(void)arg;
	usleep(100 * 1000);
	kill(getpid(), SIGUSR1);
	return NULL;
}

static void check(const char *what, int queued_before)
{
	sigset_t block, old;
	pthread_t sender;
	int calls = 0;

	got = 0;
	timed_out = 0;
	sigemptyset(&block);
	sigaddset(&block, SIGUSR1);
	sigprocmask(SIG_BLOCK, &block, &old);
	if (queued_before) {
		kill(getpid(), SIGUSR1);
	} else if (pthread_create(&sender, NULL, send_later, NULL)) {
		printf("FAIL %s: pthread_create\n", what);
		failures++;
		sigprocmask(SIG_SETMASK, &old, NULL);
		return;
	}

	alarm(5);
	while (!got && !timed_out && calls < 1000) {
		sigsuspend(&old);
		calls++;
	}
	alarm(0);
	if (!queued_before)
		pthread_join(sender, NULL);

	sigset_t now;
	sigprocmask(SIG_SETMASK, NULL, &now);
	if (got != SIGUSR1 || calls != 1 || !sigismember(&now, SIGUSR1)) {
		printf("FAIL %s: handler %s, %d calls, mask %s\n", what,
		       got ? "ran" : timed_out ? "never ran (timed out)" :
				 "never ran", calls,
		       sigismember(&now, SIGUSR1) ? "restored" : "lost");
		failures++;
	}
	sigprocmask(SIG_SETMASK, &old, NULL);
}

int main(void)
{
	struct sigaction sa;

	memset(&sa, 0, sizeof(sa));
	sa.sa_handler = handler;
	sigaction(SIGUSR1, &sa, NULL);
	sa.sa_handler = on_alarm;
	sigaction(SIGALRM, &sa, NULL);

	check("signal queued before the call", 1);
	check("signal arriving during the call", 0);

	if (failures)
		return 1;
	printf("sigsuspend: all checks pass\n");
	return 0;
}
