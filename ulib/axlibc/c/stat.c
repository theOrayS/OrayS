#include <errno.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>

// TODO:
int fchmod(int fd, mode_t mode)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO:
int mkdir(const char *path, mode_t mode)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO
int chmod(const char *path, mode_t mode)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO
mode_t umask(mode_t mask)
{
    unimplemented("mask: %d", mask);
    errno = ENOSYS;
    return (mode_t)-1;
}

// TODO
int fstatat(int fd, const char *restrict path, struct stat *restrict st, int flag)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
