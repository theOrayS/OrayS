#define _GNU_SOURCE

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/syscall.h>
#include <unistd.h>

_Static_assert(sizeof(struct msghdr) == 56, "64-bit Linux msghdr size");
_Static_assert(sizeof(struct mmsghdr) == 64, "64-bit Linux mmsghdr size");
_Static_assert(_Alignof(struct mmsghdr) == 8, "64-bit Linux mmsghdr alignment");
_Static_assert(offsetof(struct mmsghdr, msg_len) == 56, "Linux msg_len offset");

static int failures;

#define CHECK(condition, ...)                                                    \
    do {                                                                         \
        if (!(condition)) {                                                      \
            fprintf(stderr, "FAIL %s:%d: ", __func__, __LINE__);               \
            fprintf(stderr, __VA_ARGS__);                                        \
            fputc('\n', stderr);                                                 \
            failures++;                                                          \
        }                                                                        \
    } while (0)

static int sm(int fd, struct mmsghdr *msgvec, unsigned int vlen, int flags)
{
    return (int)syscall(SYS_sendmmsg, fd, msgvec, vlen, flags);
}

static void init_msg(struct mmsghdr *msg, struct iovec *iov, void *data,
                     size_t len, unsigned int sentinel)
{
    memset(msg, 0, sizeof(*msg));
    iov->iov_base = data;
    iov->iov_len = len;
    msg->msg_hdr.msg_iov = iov;
    msg->msg_hdr.msg_iovlen = 1;
    msg->msg_len = sentinel;
}

static int udp_pair(int *sender, int *receiver)
{
    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    socklen_t addrlen = sizeof(addr);

    *sender = -1;
    *receiver = socket(AF_INET, SOCK_DGRAM, 0);
    if (*receiver < 0 || bind(*receiver, (void *)&addr, sizeof(addr)) < 0 ||
        getsockname(*receiver, (void *)&addr, &addrlen) < 0)
        return -1;
    *sender = socket(AF_INET, SOCK_DGRAM, 0);
    return *sender >= 0 && connect(*sender, (void *)&addr, addrlen) == 0 ? 0 : -1;
}

static void close_pair(int first, int second)
{
    if (first >= 0)
        close(first);
    if (second >= 0)
        close(second);
}

static void test_vlen_zero(void)
{
    int tx = -1, rx = -1;
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx >= 0) {
        errno = 0;
        CHECK(sm(tx, (void *)(uintptr_t)1, 0, INT_MAX) == 0,
              "valid socket errno=%d", errno);
    }
    errno = 0;
    CHECK(sm(-1, (void *)(uintptr_t)1, 0, 0) == -1 && errno == EBADF,
          "invalid fd errno=%d", errno);
    close_pair(tx, rx);
}

static void test_udp_success(void)
{
    int tx = -1, rx = -1;
    char data[3] = {'a', 'b', 'c'}, received;
    struct iovec iov[3];
    struct mmsghdr msgs[3];
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx < 0)
        goto out;

    for (int i = 0; i < 3; i++)
        init_msg(&msgs[i], &iov[i], &data[i], 1, 0x5a5a5a5aU);
    CHECK(sm(tx, msgs, 1, 0) == 1, "single UDP: %s", strerror(errno));
    CHECK(msgs[0].msg_len == 1, "single msg_len=%u", msgs[0].msg_len);
    CHECK(recv(rx, &received, 1, 0) == 1 && received == data[0], "single payload");

    for (int i = 0; i < 3; i++)
        init_msg(&msgs[i], &iov[i], &data[i], 1, 0x5a5a5a5aU);
    CHECK(sm(tx, msgs, 3, 0) == 3, "three UDP: %s", strerror(errno));
    for (int i = 0; i < 3; i++) {
        CHECK(msgs[i].msg_len == 1, "msg_len[%d]=%u", i, msgs[i].msg_len);
        CHECK(recv(rx, &received, 1, 0) == 1 && received == data[i],
              "payload[%d]", i);
    }
out:
    close_pair(tx, rx);
}

static void test_unix_success(void)
{
    int sockets[2] = {-1, -1};
    char data[3] = {'x', 'y', 'z'}, received;
    struct iovec iov[3];
    struct mmsghdr msgs[3];
    CHECK(socketpair(AF_UNIX, SOCK_DGRAM, 0, sockets) == 0,
          "socketpair: %s", strerror(errno));
    if (sockets[0] < 0)
        return;
    for (int i = 0; i < 3; i++)
        init_msg(&msgs[i], &iov[i], &data[i], 1, 0x6b6b6b6bU);
    CHECK(sm(sockets[0], msgs, 3, 0) == 3, "send: %s", strerror(errno));
    for (int i = 0; i < 3; i++) {
        CHECK(msgs[i].msg_len == 1, "msg_len[%d]=%u", i, msgs[i].msg_len);
        CHECK(recv(sockets[1], &received, 1, 0) == 1 && received == data[i],
              "payload[%d]", i);
    }
    close_pair(sockets[0], sockets[1]);
}

static void test_invalid_msgvec_and_first_failure(void)
{
    int tx = -1, rx = -1;
    char data;
    struct iovec iov;
    struct mmsghdr msg;
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx < 0)
        goto out;
    errno = 0;
    CHECK(sm(tx, (void *)(uintptr_t)1, 1, 0) == -1 && errno == EFAULT,
          "invalid msgvec errno=%d", errno);
    init_msg(&msg, &iov, (void *)(uintptr_t)1, 1, 0x7c7c7c7cU);
    errno = 0;
    CHECK(sm(tx, &msg, 1, 0) == -1 && errno == EFAULT,
          "first failure errno=%d", errno);
    CHECK(msg.msg_len == 0x7c7c7c7cU, "first failure msg_len=%#x", msg.msg_len);
    (void)data;
out:
    close_pair(tx, rx);
}

static void test_partial_failure(unsigned int failing)
{
    int tx = -1, rx = -1;
    char data[3] = {'p', 'q', 'r'}, received;
    struct iovec iov[3];
    struct mmsghdr msgs[3];
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx < 0)
        goto out;
    for (int i = 0; i < 3; i++)
        init_msg(&msgs[i], &iov[i], &data[i], 1, 0x81818181U + (unsigned)i);
    iov[failing].iov_base = (void *)(uintptr_t)1;
    errno = 0;
    CHECK(sm(tx, msgs, 3, 0) == (int)failing,
          "failure at %u errno=%d", failing, errno);
    for (unsigned int i = 0; i < failing; i++) {
        CHECK(msgs[i].msg_len == 1, "completed msg_len[%u]=%u", i, msgs[i].msg_len);
        CHECK(recv(rx, &received, 1, 0) == 1 && received == data[i],
              "completed payload[%u]", i);
    }
    for (unsigned int i = failing; i < 3; i++)
        CHECK(msgs[i].msg_len == 0x81818181U + i,
              "uncompleted msg_len[%u]=%#x", i, msgs[i].msg_len);
out:
    close_pair(tx, rx);
}

static void test_crossing_invalid_page(void)
{
    int tx = -1, rx = -1;
    long page = sysconf(_SC_PAGESIZE);
    unsigned char *map = MAP_FAILED;
    char data = 'v';
    struct iovec iov;
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx < 0)
        goto out;
    map = mmap(NULL, (size_t)page * 2, PROT_READ | PROT_WRITE,
               MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    CHECK(map != MAP_FAILED, "mmap: %s", strerror(errno));
    if (map == MAP_FAILED)
        goto out;
    struct mmsghdr *msgs = (void *)(map + page - sizeof(*msgs));
    init_msg(&msgs[0], &iov, &data, 1, 0x92929292U);
    CHECK(mprotect(map + page, (size_t)page, PROT_NONE) == 0,
          "mprotect: %s", strerror(errno));
    errno = 0;
    CHECK(sm(tx, msgs, 2, 0) == 1, "partial copy-in errno=%d", errno);
    CHECK(msgs[0].msg_len == 1, "first msg_len=%u", msgs[0].msg_len);
out:
    if (map != MAP_FAILED) {
        mprotect(map + page, (size_t)page, PROT_READ | PROT_WRITE);
        munmap(map, (size_t)page * 2);
    }
    close_pair(tx, rx);
}

static void test_copyout_failure(void)
{
    int tx = -1, rx = -1;
    long page = sysconf(_SC_PAGESIZE);
    unsigned char *map = MAP_FAILED;
    char data = 'w', received;
    struct iovec iov;
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx < 0)
        goto out;
    map = mmap(NULL, (size_t)page, PROT_READ | PROT_WRITE,
               MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    CHECK(map != MAP_FAILED, "mmap: %s", strerror(errno));
    if (map == MAP_FAILED)
        goto out;
    struct mmsghdr *msg = (void *)map;
    init_msg(msg, &iov, &data, 1, 0xa3a3a3a3U);
    CHECK(mprotect(map, (size_t)page, PROT_READ) == 0,
          "mprotect: %s", strerror(errno));
    errno = 0;
    CHECK(sm(tx, msg, 1, 0) == -1 && errno == EFAULT,
          "copyout errno=%d", errno);
    CHECK(msg->msg_len == 0xa3a3a3a3U, "copyout msg_len=%#x", msg->msg_len);
    CHECK(recv(rx, &received, 1, 0) == 1 && received == data,
          "copyout-failed message not delivered");
out:
    if (map != MAP_FAILED) {
        mprotect(map, (size_t)page, PROT_READ | PROT_WRITE);
        munmap(map, (size_t)page);
    }
    close_pair(tx, rx);
}

static void test_address_overflow(void)
{
    int tx = -1, rx = -1;
    uintptr_t ptr = UINTPTR_MAX - sizeof(struct mmsghdr) + 2;
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx >= 0) {
        errno = 0;
        CHECK(sm(tx, (void *)ptr, 2, 0) == -1 && errno == EFAULT,
              "overflow errno=%d", errno);
    }
    close_pair(tx, rx);
}

static void test_vlen_limit(void)
{
    enum { UIO_MAXIOV = 1024, REQUESTED = UIO_MAXIOV + 1 };
    int tx = -1, rx = -1;
    char *data = calloc(REQUESTED, 1);
    struct iovec *iov = calloc(REQUESTED, sizeof(*iov));
    struct mmsghdr *msgs = calloc(REQUESTED, sizeof(*msgs));
    CHECK(data && iov && msgs, "allocation failed");
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (!data || !iov || !msgs || tx < 0)
        goto out;
    for (int i = 0; i < REQUESTED; i++)
        init_msg(&msgs[i], &iov[i], &data[i], 1, 0xb4b4b4b4U);
    errno = 0;
    CHECK(sm(tx, msgs, REQUESTED, 0) == UIO_MAXIOV,
          "over-limit request errno=%d", errno);
    CHECK(msgs[UIO_MAXIOV - 1].msg_len == 1, "last allowed msg_len=%u",
          msgs[UIO_MAXIOV - 1].msg_len);
    CHECK(msgs[UIO_MAXIOV].msg_len == 0xb4b4b4b4U,
          "capped element modified=%#x", msgs[UIO_MAXIOV].msg_len);
out:
    close_pair(tx, rx);
    free(msgs);
    free(iov);
    free(data);
}

static void test_nonblocking_partial(void)
{
    enum { COUNT = 128, SIZE = 1024 };
    int sockets[2] = {-1, -1};
    char *data = calloc(COUNT, SIZE);
    struct iovec *iov = calloc(COUNT, sizeof(*iov));
    struct mmsghdr *msgs = calloc(COUNT, sizeof(*msgs));
    CHECK(data && iov && msgs, "allocation failed");
    CHECK(socketpair(AF_UNIX, SOCK_DGRAM | SOCK_NONBLOCK, 0, sockets) == 0,
          "socketpair: %s", strerror(errno));
    if (!data || !iov || !msgs || sockets[0] < 0)
        goto out;
    for (int i = 0; i < COUNT; i++)
        init_msg(&msgs[i], &iov[i], data + i * SIZE, SIZE, 0xc5c5c5c5U);
    errno = 0;
    int sent = sm(sockets[0], msgs, COUNT, 0);
    CHECK(sent > 0 && sent < COUNT, "partial result=%d errno=%d", sent, errno);
    if (sent > 0 && sent < COUNT) {
        for (int i = 0; i < sent; i++)
            CHECK(msgs[i].msg_len == SIZE, "msg_len[%d]=%u", i, msgs[i].msg_len);
        CHECK(msgs[sent].msg_len == 0xc5c5c5c5U,
              "first unsent msg_len=%#x", msgs[sent].msg_len);
    }
out:
    close_pair(sockets[0], sockets[1]);
    free(msgs);
    free(iov);
    free(data);
}

static void test_fd_and_flags_errors(void)
{
    int tx = -1, rx = -1, file = -1;
    char data = 'e';
    struct iovec iov;
    struct mmsghdr msg;
    CHECK(udp_pair(&tx, &rx) == 0, "udp setup: %s", strerror(errno));
    if (tx < 0)
        goto out;
    init_msg(&msg, &iov, &data, 1, 0xd6d6d6d6U);
    errno = 0;
    CHECK(sm(tx, &msg, 1, INT_MAX) == -1 && errno == EOPNOTSUPP,
          "invalid flags errno=%d", errno);
    CHECK(msg.msg_len == 0xd6d6d6d6U, "invalid flags msg_len=%#x", msg.msg_len);
    file = open("/dev/null", O_WRONLY);
    CHECK(file >= 0, "open /dev/null: %s", strerror(errno));
    init_msg(&msg, &iov, &data, 1, 0xe7e7e7e7U);
    errno = 0;
    CHECK(sm(file, &msg, 1, 0) == -1 && errno == ENOTSOCK,
          "ordinary fd errno=%d", errno);
    CHECK(msg.msg_len == 0xe7e7e7e7U, "ordinary fd msg_len=%#x", msg.msg_len);
out:
    if (file >= 0)
        close(file);
    close_pair(tx, rx);
}

int main(void)
{
    test_vlen_zero();
    test_udp_success();
    test_unix_success();
    test_invalid_msgvec_and_first_failure();
    test_partial_failure(1);
    test_partial_failure(2);
    test_crossing_invalid_page();
    test_copyout_failure();
    test_address_overflow();
    test_vlen_limit();
    test_nonblocking_partial();
    test_fd_and_flags_errors();
    if (failures) {
        fprintf(stderr, "sendmmsg: %d failure(s)\n", failures);
        return 1;
    }
    puts("sendmmsg: PASS");
    return 0;
}
