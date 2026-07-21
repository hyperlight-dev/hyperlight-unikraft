#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

/*
 * Cooperative-poll socket-recv app.
 *
 * Listens on 127.0.0.1:PORT, accepts one connection, and blocks in recv()
 * until the peer sends data. Under the cooperative poll model both accept()
 * and recv() must yield the vCPU back to the host (via the Unikraft
 * poll/epoll layer) instead of issuing a blocking host call — otherwise the
 * whole VM would freeze for the duration of the wait. The host-side test
 * (host/tests/poll_recv.rs) drives this guest with Sandbox::poll +
 * Sandbox::drive_host_functions while a client thread connects and sends a payload;
 * the guest should reach the recv, print the bytes, and exit (Done).
 */

#define PORT 34567

/* Report the process exit code to the host via the __hl_exit hcall.
 *
 * A normal main() return only HALTs the VM (the host records exit code 0); to
 * hand a meaningful value back to the driving test we issue an explicit
 * __hl_exit host call over /dev/hcall, mirroring poll-await-c / poll-epoll-c.
 * The host test asserts this value, so a functional regression (e.g. the guest
 * failing to accept or recv the payload) surfaces as a distinct exit code
 * rather than a silently-passing run. */
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
    printf("poll-recv: start\n");
    fflush(stdout);

    int lfd = socket(AF_INET, SOCK_STREAM, 0);
    if (lfd < 0) {
        printf("poll-recv: socket failed\n");
        fflush(stdout);
        report_exit_code(101);
        return 1;
    }

    int one = 1;
    (void)setsockopt(lfd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(PORT);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (bind(lfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        printf("poll-recv: bind failed\n");
        fflush(stdout);
        report_exit_code(102);
        return 1;
    }

    if (listen(lfd, 1) < 0) {
        printf("poll-recv: listen failed\n");
        fflush(stdout);
        report_exit_code(103);
        return 1;
    }

    printf("poll-recv: listening\n");
    fflush(stdout);

    int cfd = accept(lfd, NULL, NULL);
    if (cfd < 0) {
        printf("poll-recv: accept failed\n");
        fflush(stdout);
        report_exit_code(104);
        return 1;
    }

    printf("poll-recv: accepted\n");
    fflush(stdout);

    char buf[64];
    ssize_t n = recv(cfd, buf, sizeof(buf) - 1, 0);
    if (n < 0) {
        printf("poll-recv: recv failed\n");
        fflush(stdout);
        report_exit_code(105);
        return 1;
    }
    buf[n] = '\0';
    printf("poll-recv: got %zd bytes: %s\n", n, buf);
    fflush(stdout);

    close(cfd);
    close(lfd);

    printf("poll-recv: done\n");
    fflush(stdout);
    report_exit_code((int)n);
    return 0;
}
