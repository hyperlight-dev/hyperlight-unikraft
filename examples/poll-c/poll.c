#include <stdio.h>
#include <time.h>

/*
 * Cooperative-poll smoke app.
 *
 * Prints a line, sleeps a few short intervals, then exits. Under the
 * cooperative poll model each nanosleep drives the unikernel scheduler to
 * idle with a pending timer, so the host observes a sequence of
 * PollStatus::Wait steps followed by PollStatus::Done once main returns
 * (a normal exit_group -> SYSHALT). This exercises the full
 * yield / re-enter / terminate cycle at runtime.
 */
int main(void) {
    printf("poll app: start\n");
    fflush(stdout);

    for (int i = 0; i < 3; i++) {
        struct timespec ts = { .tv_sec = 0, .tv_nsec = 10 * 1000 * 1000 }; /* 10 ms */
        nanosleep(&ts, NULL);
        printf("poll app: woke %d\n", i);
        fflush(stdout);
    }

    printf("poll app: done\n");
    fflush(stdout);
    return 0;
}
