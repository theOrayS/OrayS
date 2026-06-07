#include <errno.h>
#include <stdio.h>
#include <sys/ioctl.h>

// TODO
int ioctl(int __fd, int __request, ...)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
