#ifdef AX_CONFIG_NET

#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

int accept4(int fd, struct sockaddr *restrict addr, socklen_t *restrict len, int flg)
{
    if (!flg)
        return accept(fd, addr, len);
    if (flg & ~(SOCK_CLOEXEC | SOCK_NONBLOCK)) {
        errno = EINVAL;
        return -1;
    }
    int ret = accept(fd, addr, len);
    if (ret < 0)
        return ret;
    if ((flg & SOCK_CLOEXEC) && fcntl(ret, F_SETFD, FD_CLOEXEC) < 0) {
        int saved_errno = errno;
        close(ret);
        errno = saved_errno;
        return -1;
    }
    if (flg & SOCK_NONBLOCK) {
        int current_flags = fcntl(ret, F_GETFL);
        if (current_flags < 0 || fcntl(ret, F_SETFL, current_flags | O_NONBLOCK) < 0) {
            int saved_errno = errno;
            close(ret);
            errno = saved_errno;
            return -1;
        }
    }
    return ret;
}

int getsockopt(int fd, int level, int optname, void *restrict optval, socklen_t *restrict optlen)
{
    unimplemented();
    errno = ENOPROTOOPT;
    return -1;
}

int setsockopt(int fd, int level, int optname, const void *optval, socklen_t optlen)
{
    unimplemented("fd: %d, level: %d, optname: %d, optlen: %d", fd, level, optname, optlen);
    errno = ENOPROTOOPT;
    return -1;
}

// TODO
ssize_t sendmsg(int fd, const struct msghdr *msg, int flags)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

#endif // AX_CONFIG_NET
