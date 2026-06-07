#include <errno.h>
#include <stdio.h>
#include <sys/resource.h>
#include <sys/wait.h>

// TODO
pid_t waitpid(pid_t pid, int *status, int options)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO
pid_t wait3(int *status, int _options, struct rusage *usage)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
