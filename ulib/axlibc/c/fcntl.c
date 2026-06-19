#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>

#ifdef AX_CONFIG_FD

// TODO: remove this function in future work
int ax_fcntl(int fd, int cmd, size_t arg);

static int fcntl_cmd_needs_arg(int cmd)
{
    switch (cmd) {
    case F_DUPFD:
    case F_SETFD:
    case F_SETFL:
    case F_GETLK:
    case F_SETLK:
    case F_SETLKW:
    case F_SETOWN:
    case F_SETSIG:
    case F_DUPFD_CLOEXEC:
        return 1;
    default:
        return 0;
    }
}

int fcntl(int fd, int cmd, ... /* arg */)
{
    unsigned long arg = 0;

    if (fcntl_cmd_needs_arg(cmd)) {
        va_list ap;
        va_start(ap, cmd);
        arg = va_arg(ap, unsigned long);
        va_end(ap);
    }

    return ax_fcntl(fd, cmd, arg);
}

#endif // AX_CONFIG_FD

#ifdef AX_CONFIG_FS

// TODO: remove this function in future work
int ax_open(const char *filename, int flags, mode_t mode);

int open(const char *filename, int flags, ...)
{
    mode_t mode = 0;

    if ((flags & O_CREAT) || (flags & O_TMPFILE) == O_TMPFILE) {
        va_list ap;
        va_start(ap, flags);
        mode = va_arg(ap, mode_t);
        va_end(ap);
    }

    return ax_open(filename, flags, mode);
}

// TODO
int posix_fadvise(int __fd, unsigned long __offset, unsigned long __len, int __advise)
{
    unimplemented();
    return ENOSYS;
}

// TODO
int sync_file_range(int fd, off_t pos, off_t len, unsigned flags)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

#endif // AX_CONFIG_FS
