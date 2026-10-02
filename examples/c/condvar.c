/*
 * Threads waiting on one condition variable, released together by a
 * broadcast, fifty times over: every one of them must wake.  Built with
 * musl, whose condition variables hand each woken waiter's successor to
 * the mutex with FUTEX_REQUEUE.
 */
#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

enum { WAITERS = 4, ROUNDS = 50 };

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static int go, woke;

static void *waiter(void *arg)
{
	(void)arg;
	pthread_mutex_lock(&lock);
	while (!go)
		pthread_cond_wait(&cond, &lock);
	woke++;
	pthread_mutex_unlock(&lock);
	return NULL;
}

int main(void)
{
	for (int round = 0; round < ROUNDS; round++) {
		pthread_t threads[WAITERS];

		go = 0;
		woke = 0;
		for (int i = 0; i < WAITERS; i++) {
			if (pthread_create(&threads[i], NULL, waiter, NULL)) {
				printf("round %d: pthread_create failed\n", round);
				return 2;
			}
		}
		usleep(2000); /* let them all wait */
		pthread_mutex_lock(&lock);
		go = 1;
		pthread_cond_broadcast(&cond);
		pthread_mutex_unlock(&lock);
		/* A waiter left asleep never joins: give up after a second. */
		for (int i = 0; i < 100 && woke < WAITERS; i++)
			usleep(10000);
		if (woke < WAITERS) {
			printf("round %d: %d of %d waiters woke\n", round, woke, WAITERS);
			return 1;
		}
		for (int i = 0; i < WAITERS; i++)
			pthread_join(threads[i], NULL);
	}
	printf("every waiter woke, %d rounds\n", ROUNDS);
	return 0;
}
