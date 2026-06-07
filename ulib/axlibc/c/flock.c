#include <errno.h>
#include <stdio.h>
#include <sys/file.h>

// TODO
int flock(int __fd, int __operation)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
