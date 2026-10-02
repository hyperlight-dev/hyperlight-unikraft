/*
 * FUTEX_REQUEUE and FUTEX_CMP_REQUEUE as Linux defines them: a requeue
 * wakes only as many waiters as asked (none for 0) and moves the next ones
 * to another futex, a requeue onto the same futex returns, and a requeued
 * waiter that times out says so and leaves nothing behind.  Every wait on
 * a waiter has a deadline: a kernel that loses one fails the check rather
 * than hanging it.
 */
#include <errno.h>
#include <limits.h>
#include <linux/futex.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

static unsigned int x, y;
static int finished; /* waiters that returned from their wait */
static int failures;
static int cmp; /* FUTEX_CMP_REQUEUE, with the current value, else FUTEX_REQUEUE */

static long futex(unsigned int *uaddr, int op, unsigned int val,
		  const struct timespec *timeout, unsigned int *uaddr2,
		  unsigned int val3)
{
	return syscall(SYS_futex, uaddr, op, val, timeout, uaddr2, val3);
}

static long requeue(unsigned int *from, unsigned int nwake, long nmove,
		    unsigned int *to)
{
	return futex(from, cmp ? FUTEX_CMP_REQUEUE : FUTEX_REQUEUE, nwake,
		     (const struct timespec *)nmove, to, *from);
}

static void check(int ok, const char *what)
{
	printf("%s: %s: %s\n", ok ? "ok" : "FAIL",
	       cmp ? "FUTEX_CMP_REQUEUE" : "FUTEX_REQUEUE", what);
	if (!ok)
		failures++;
}

/* Whether n waiters have returned within ms milliseconds. */
static int finished_within(int n, int ms)
{
	for (int i = 0; i < ms / 10; i++) {
		if (__atomic_load_n(&finished, __ATOMIC_SEQ_CST) >= n)
			return 1;
		usleep(10000);
	}
	return 0;
}

static void *wait_untimed(void *arg)
{
	futex(arg, FUTEX_WAIT, 0, NULL, NULL, 0);
	__atomic_add_fetch(&finished, 1, __ATOMIC_SEQ_CST);
	return NULL;
}

/* Two seconds: long enough that a host pausing the vCPU before the
 * requeue does not time the waiter out on the source futex first. */
static void *wait_timed(void *arg)
{
	struct timespec t = { 2, 0 };
	long r = futex(&x, FUTEX_WAIT, 0, &t, NULL, 0);

	*(long *)arg = r < 0 ? -errno : r;
	__atomic_add_fetch(&finished, 1, __ATOMIC_SEQ_CST);
	return NULL;
}

static void start(pthread_t *t, void *(*fn)(void *), void *arg)
{
	if (pthread_create(t, NULL, fn, arg)) {
		printf("FAIL: pthread_create\n");
		exit(2);
	}
}

/* Join n waiters, or give up on them: a waiter the kernel lost is still
 * asleep, and the exit takes it down. */
static void join(pthread_t *t, int n, int total, int ms, const char *what)
{
	if (!finished_within(total, ms)) {
		check(0, what);
		printf("futex requeue: %d failed (a waiter never woke)\n", failures);
		exit(1);
	}
	for (int i = 0; i < n; i++)
		pthread_join(t[i], NULL);
}

static void checks(void)
{
	pthread_t t[2];
	long r, timed;

	x = y = 0;
	__atomic_store_n(&finished, 0, __ATOMIC_SEQ_CST);

	/* Requeue waking none: the waiter moves, still asleep, and a wake
	 * on the target reaches it. */
	start(&t[0], wait_untimed, &x);
	usleep(50000);
	r = requeue(&x, 0, 1, &y);
	check(r == 1, "wake 0, move 1 counts one waiter");
	usleep(50000);
	check(__atomic_load_n(&finished, __ATOMIC_SEQ_CST) == 0,
	      "the requeued waiter is still asleep");
	r = futex(&x, FUTEX_WAKE, 1, NULL, NULL, 0);
	check(r == 0, "no waiter is left on the source futex");
	r = futex(&y, FUTEX_WAKE, 1, NULL, NULL, 0);
	check(r == 1, "a wake on the target futex finds it");
	join(t, 1, 1, 1000, "a wake on the target futex wakes it");

	/* Requeue onto the same futex: returns, the waiters stay. */
	start(&t[0], wait_untimed, &x);
	start(&t[1], wait_untimed, &x);
	usleep(50000);
	r = requeue(&x, 0, INT_MAX, &x);
	check(r == 2, "a requeue onto the same futex returns, counting both");
	r = futex(&x, FUTEX_WAKE, 2, NULL, NULL, 0);
	check(r == 2, "both are still waiting on it");
	join(t, 2, 3, 1000, "both wake");

	/* A requeued waiter that times out reports the timeout, and leaves
	 * no entry for a later wake to find. */
	timed = 1;
	start(&t[0], wait_timed, &timed);
	usleep(50000);
	r = requeue(&x, 0, 1, &y);
	check(r == 1, "the timed waiter is requeued");
	join(t, 1, 4, 4000, "the timed waiter returns");
	check(timed == -ETIMEDOUT, "a requeued waiter that times out reports ETIMEDOUT");
	r = futex(&y, FUTEX_WAKE, INT_MAX, NULL, NULL, 0);
	check(r == 0, "and leaves nothing on the target futex");
}

int main(void)
{
	long r;

	checks();
	cmp = 1;
	checks();

	x = 1;
	r = futex(&x, FUTEX_CMP_REQUEUE, 0, (const struct timespec *)1L, &y, 0);
	check(r == -1 && errno == EAGAIN, "the wrong value is EAGAIN");

	printf(failures ? "futex requeue: %d failed\n" : "futex requeue: all checks pass\n",
	       failures);
	return failures != 0;
}
