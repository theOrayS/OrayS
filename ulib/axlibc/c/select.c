#ifdef AX_CONFIG_SELECT

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/select.h>
#include <sys/time.h>

int pselect(int n, fd_set *restrict rfds, fd_set *restrict wfds, fd_set *restrict efds,
            const struct timespec *restrict ts, const sigset_t *restrict mask)
{
    if (mask) {
        errno = ENOSYS;
        return -1;
    }
    if (!ts)
        return select(n, rfds, wfds, efds, NULL);
    struct timeval tv = {ts->tv_sec, ts->tv_nsec / 1000};
    return select(n, rfds, wfds, efds, &tv);
}

#endif // AX_CONFIG_SELECT
