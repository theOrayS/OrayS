#include <errno.h>
#include <poll.h>
#include <stdio.h>

// TODO
int poll(struct pollfd *__fds, nfds_t __nfds, int __timeout)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
