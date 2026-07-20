#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <pthread.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

/*
 * Cooperative-poll multi-threaded socket-recv app.
 *
 * This is the multi-threaded sibling of poll-recv-c. Instead of one thread
 * parking in a single recv(), the guest spawns TWO worker threads, each of
 * which owns its own listening socket on its own port, accepts one connection,
 * and blocks in recv() until its peer sends data. So at the interesting moment
 * BOTH threads are simultaneously parked in a blocking recv() on two different
 * host-proxied sockets.
 *
 * Under the cooperative poll model each blocking accept()/recv() must yield the
 * vCPU back to the host (parking on the Unikraft posix-poll waitq via
 * uk_file_poll) instead of issuing a blocking host call — otherwise the whole
 * VM would freeze. Because two threads park concurrently, this exercises the
 * host driving TWO in-flight async net futures at once on the Tokio reactor
 * (the JoinSet multi-future path in Sandbox::drive_host_functions), and the
 * cooperative scheduler waking two independent guest threads as their sockets
 * become readable at staggered times.
 *
 * The host-side test (host/tests/poll_recv_2thread.rs) drives this guest while
 * two clients connect to the two ports and send "40" and "2" at staggered
 * times. Each worker parses its integer; main joins both threads, sums them,
 * and exits with the sum (42), which the test asserts as the exit code —
 * proving both sockets' payloads reached their respective threads intact.
 */

#define PORT_A 34570
#define PORT_B 34571

/* Distinct negative markers so a functional regression surfaces as a specific
 * exit code rather than a silently-passing run. Kept out of the 0..255 payload
 * range used for the summed result. */
#define ERR_SOCKET (-101)
#define ERR_BIND (-102)
#define ERR_LISTEN (-103)
#define ERR_ACCEPT (-104)
#define ERR_RECV (-105)

struct worker_arg {
    int port;
    int result; /* parsed integer on success, or a negative ERR_* marker */
};

/* One worker owns a single port end to end: socket -> bind -> listen ->
 * accept -> recv. Both workers run this concurrently on separate threads, so
 * their blocking accept()/recv() calls park independently under the poll
 * model. */
static void *worker(void *p) {
    struct worker_arg *a = (struct worker_arg *)p;

    int lfd = socket(AF_INET, SOCK_STREAM, 0);
    if (lfd < 0) {
        a->result = ERR_SOCKET;
        return NULL;
    }

    int one = 1;
    (void)setsockopt(lfd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons((uint16_t)a->port);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (bind(lfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        a->result = ERR_BIND;
        close(lfd);
        return NULL;
    }
    if (listen(lfd, 1) < 0) {
        a->result = ERR_LISTEN;
        close(lfd);
        return NULL;
    }

    printf("poll-recv-2thread: listening on %d\n", a->port);
    fflush(stdout);

    int cfd = accept(lfd, NULL, NULL);
    if (cfd < 0) {
        a->result = ERR_ACCEPT;
        close(lfd);
        return NULL;
    }

    printf("poll-recv-2thread: accepted on %d\n", a->port);
    fflush(stdout);

    char buf[64];
    ssize_t n = recv(cfd, buf, sizeof(buf) - 1, 0);
    if (n <= 0) {
        a->result = ERR_RECV;
        close(cfd);
        close(lfd);
        return NULL;
    }
    buf[n] = '\0';
    a->result = atoi(buf);

    printf("poll-recv-2thread: %d -> %d\n", a->port, a->result);
    fflush(stdout);

    close(cfd);
    close(lfd);
    return NULL;
}

/* Report the process exit code to the host via the __hl_exit hcall.
 *
 * A normal main() return only HALTs the VM (the host records exit code 0); to
 * hand a meaningful value back to the driving test we issue an explicit
 * __hl_exit host call over /dev/hcall, mirroring poll-recv-c / poll-epoll-c. */
static void report_exit_code(int code) {
    char req[80];
    int fd = open("/dev/hcall", O_RDWR);
    if (fd < 0)
        return;
    int len = snprintf(req, sizeof(req),
                       "{\"name\":\"__hl_exit\",\"args\":{\"code\":%d}}", code);
    if (len > 0)
        (void)!write(fd, req, (size_t)len);
    char resp[64];
    (void)!read(fd, resp, sizeof(resp));
    close(fd);
}

int main(void) {
    printf("poll-recv-2thread: start\n");
    fflush(stdout);

    struct worker_arg aa = {PORT_A, ERR_RECV};
    struct worker_arg ab = {PORT_B, ERR_RECV};
    pthread_t ta, tb;

    if (pthread_create(&ta, NULL, worker, &aa) != 0) {
        printf("poll-recv-2thread: pthread_create A failed\n");
        fflush(stdout);
        report_exit_code(106);
        return 1;
    }
    if (pthread_create(&tb, NULL, worker, &ab) != 0) {
        printf("poll-recv-2thread: pthread_create B failed\n");
        fflush(stdout);
        report_exit_code(107);
        return 1;
    }

    pthread_join(ta, NULL);
    pthread_join(tb, NULL);

    /* Surface a per-worker failure as its distinct negative marker (mapped to a
     * positive exit code), so the test can tell socket/bind/accept/recv apart. */
    if (aa.result < 0) {
        printf("poll-recv-2thread: worker A failed (%d)\n", aa.result);
        fflush(stdout);
        report_exit_code(-aa.result);
        return 1;
    }
    if (ab.result < 0) {
        printf("poll-recv-2thread: worker B failed (%d)\n", ab.result);
        fflush(stdout);
        report_exit_code(-ab.result);
        return 1;
    }

    int sum = aa.result + ab.result;
    printf("poll-recv-2thread: done, sum=%d\n", sum);
    fflush(stdout);

    /* Report the sum so the driving test can assert it end-to-end. */
    report_exit_code(sum);
    return 0;
}
