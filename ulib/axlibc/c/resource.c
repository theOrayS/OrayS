#include <errno.h>
#include <stdio.h>
#include <sys/resource.h>

// TODO
int getrusage(int __who, struct rusage *__usage)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
