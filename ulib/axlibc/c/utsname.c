#include <errno.h>
#include <stdio.h>
#include <sys/utsname.h>

// TODO
int uname(struct utsname *a)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
