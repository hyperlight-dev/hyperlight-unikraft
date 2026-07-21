#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/socket.h>
#include <sys/epoll.h>
#include <netinet/in.h>
#include <arpa/inet.h>

/*
 * Cooperative-poll epoll-over-two-sockets app.
 *
 * This is the multi-socket sibling of poll-recv-c. It listens on TWO ports and
 * multiplexes them with a single epoll set, so a single guest thread waits on
 * both at once. Under the cooperative poll model epoll_wait() must yield the
 * vCPU back to the host (parking on the Unikraft posix-poll waitq via
 * uk_file_poll) instead of issuing a blocking host call — otherwise the whole
 * VM would freeze while waiting.
 *
 * Flow:
 *   Phase 1 (accept): both listeners are registered in the epoll set. Each time
 *     a listener becomes readable (an incoming connection), we accept it, drop
 *     the listener from the set, and add the accepted connection instead.
 *   Phase 2 (recv): once both connections are established we epoll_wait for data
 *     on them; each delivers a short ASCII integer which we parse and sum.
 *
 * The host-side test (host/tests/poll_epoll.rs) drives this guest with
 * Sandbox::poll + Sandbox::drive_host_functions while two clients connect and
 * send "40" and "2" on the two ports at staggered times. Because the sends are
 * staggered, the guest's epoll_wait returns for the first socket, parks again,
 * and later returns for the second — proving epoll delivers readiness for two
 * independent sockets across multiple cooperative park/resume cycles. The guest
 * exits with the sum (42), which the test asserts as the exit code.
 */

#define PORT_A 34568
#define PORT_B 34569

static int make_listener(int port) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) {
        printf("poll-epoll: socket(%d) failed\n", port);
        fflush(stdout);
        return -1;
    }

    int one = 1;
    (void)setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        printf("poll-epoll: bind(%d) failed\n", port);
        fflush(stdout);
        return -1;
    }
    if (listen(fd, 1) < 0) {
        printf("poll-epoll: listen(%d) failed\n", port);
        fflush(stdout);
        return -1;
    }
    return fd;
}

static int ep_add(int ep, int fd) {
    struct epoll_event ev;
    memset(&ev, 0, sizeof(ev));
    ev.events = EPOLLIN;
    ev.data.fd = fd;
    return epoll_ctl(ep, EPOLL_CTL_ADD, fd, &ev);
}

static int ep_del(int ep, int fd) {
    return epoll_ctl(ep, EPOLL_CTL_DEL, fd, NULL);
}

/* Report the process exit code to the host via the __hl_exit hcall.
 *
 * A normal main() return only HALTs the VM (the host records exit code 0); to
 * hand a meaningful value back to the driving test we issue an explicit
 * __hl_exit host call over /dev/hcall, mirroring poll-await-c. */
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
    printf("poll-epoll: start\n");
    fflush(stdout);

    int la = make_listener(PORT_A);
    int lb = make_listener(PORT_B);
    if (la < 0 || lb < 0)
        return 1;

    int ep = epoll_create1(0);
    if (ep < 0) {
        printf("poll-epoll: epoll_create1 failed\n");
        fflush(stdout);
        return 1;
    }

    if (ep_add(ep, la) < 0 || ep_add(ep, lb) < 0) {
        printf("poll-epoll: epoll_ctl ADD listener failed\n");
        fflush(stdout);
        return 1;
    }

    printf("poll-epoll: listening on %d and %d\n", PORT_A, PORT_B);
    fflush(stdout);

    /* Phase 1: accept a connection on each listener, driven by epoll. */
    int ca = -1, cb = -1;
    int accepted = 0;
    while (accepted < 2) {
        struct epoll_event evs[4];
        int n = epoll_wait(ep, evs, 4, -1);
        if (n < 0) {
            printf("poll-epoll: epoll_wait (accept) failed\n");
            fflush(stdout);
            return 1;
        }
        for (int i = 0; i < n; i++) {
            int fd = evs[i].data.fd;
            if (fd == la && ca < 0) {
                ca = accept(la, NULL, NULL);
                if (ca < 0) {
                    printf("poll-epoll: accept A failed\n");
                    fflush(stdout);
                    return 1;
                }
                ep_del(ep, la);
                if (ep_add(ep, ca) < 0)
                    return 1;
                accepted++;
                printf("poll-epoll: accepted A\n");
                fflush(stdout);
            } else if (fd == lb && cb < 0) {
                cb = accept(lb, NULL, NULL);
                if (cb < 0) {
                    printf("poll-epoll: accept B failed\n");
                    fflush(stdout);
                    return 1;
                }
                ep_del(ep, lb);
                if (ep_add(ep, cb) < 0)
                    return 1;
                accepted++;
                printf("poll-epoll: accepted B\n");
                fflush(stdout);
            }
        }
    }

    /* Phase 2: recv one ASCII integer from each connection, driven by epoll. */
    int sum = 0;
    int got_a = 0, got_b = 0;
    while (!(got_a && got_b)) {
        struct epoll_event evs[4];
        int n = epoll_wait(ep, evs, 4, -1);
        if (n < 0) {
            printf("poll-epoll: epoll_wait (recv) failed\n");
            fflush(stdout);
            return 1;
        }
        for (int i = 0; i < n; i++) {
            int fd = evs[i].data.fd;
            if ((fd == ca && got_a) || (fd == cb && got_b))
                continue;
            char buf[64];
            ssize_t r = recv(fd, buf, sizeof(buf) - 1, 0);
            if (r <= 0) {
                printf("poll-epoll: recv failed on fd %d\n", fd);
                fflush(stdout);
                return 1;
            }
            buf[r] = '\0';
            int val = atoi(buf);
            sum += val;
            /* Drop the consumed connection from the set: otherwise, once the
             * peer closes, it stays level-triggered readable (EOF) and
             * epoll_wait would return it every iteration, busy-spinning the
             * vCPU instead of parking to wait for the other socket. */
            ep_del(ep, fd);
            if (fd == ca) {
                got_a = 1;
                printf("poll-epoll: A -> %d\n", val);
            } else if (fd == cb) {
                got_b = 1;
                printf("poll-epoll: B -> %d\n", val);
            }
            fflush(stdout);
        }
    }

    close(ca);
    close(cb);
    close(la);
    close(lb);

    printf("poll-epoll: done, sum=%d\n", sum);
    fflush(stdout);

    /* Report the sum to the host so the driving test can assert it end-to-end
     * (a normal exit only yields code 0). */
    report_exit_code(sum);
    return sum;
}
