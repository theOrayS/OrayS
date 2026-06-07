#include <errno.h>
#include <pwd.h>
#include <stdio.h>

int getpwnam_r(const char *name, struct passwd *pw, char *buf, size_t size, struct passwd **res)
{
    unimplemented();
    if (res)
        *res = NULL;
    return ENOSYS;
}

int getpwuid_r(uid_t uid, struct passwd *pw, char *buf, size_t size, struct passwd **res)
{
    unimplemented();
    if (res)
        *res = NULL;
    return ENOSYS;
}
