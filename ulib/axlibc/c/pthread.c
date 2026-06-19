#ifdef AX_CONFIG_MULTITASK

#include <errno.h>
#include <limits.h>
#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

extern int ax_pthread_mutex_trylock(pthread_mutex_t *m);

int pthread_setcancelstate(int new, int *old)
{
    unimplemented();
    return ENOSYS;
}

int pthread_setcanceltype(int new, int *old)
{
    unimplemented();
    return ENOSYS;
}

void pthread_testcancel(void)
{
    return;
}

// TODO
int pthread_cancel(pthread_t t)
{
    unimplemented();
    return ENOSYS;
}

int pthread_mutex_trylock(pthread_mutex_t *m)
{
    return ax_pthread_mutex_trylock(m);
}

int pthread_mutexattr_init(pthread_mutexattr_t *a)
{
    if (!a)
        return EINVAL;
    a->__attr = PTHREAD_MUTEX_DEFAULT;
    return 0;
}

int pthread_mutexattr_destroy(pthread_mutexattr_t *a)
{
    if (!a)
        return EINVAL;
    return 0;
}

int pthread_mutexattr_gettype(const pthread_mutexattr_t *restrict a, int *restrict type)
{
    if (!a || !type)
        return EINVAL;
    *type = a->__attr & 0x3;
    return 0;
}

int pthread_mutexattr_settype(pthread_mutexattr_t *a, int type)
{
    if (!a)
        return EINVAL;
    switch (type) {
    case PTHREAD_MUTEX_NORMAL:
    case PTHREAD_MUTEX_RECURSIVE:
    case PTHREAD_MUTEX_ERRORCHECK:
        a->__attr = (a->__attr & ~0x3u) | (unsigned)type;
        return 0;
    default:
        return EINVAL;
    }
}

// TODO
int pthread_setname_np(pthread_t thread, const char *name)
{
    unimplemented();
    return ENOSYS;
}

int pthread_cond_init(pthread_cond_t *restrict c, const pthread_condattr_t *restrict a)
{
    *c = (pthread_cond_t){0};
    if (a) {
        c->_c_clock = a->__attr & 0x7fffffff;
        if (a->__attr >> 31)
            c->_c_shared = (void *)-1;
    }
    return 0;
}

// TODO
int pthread_cond_signal(pthread_cond_t *__cond)
{
    unimplemented();
    return ENOSYS;
}

// TODO
int pthread_cond_wait(pthread_cond_t *__restrict__ __cond, pthread_mutex_t *__restrict__ __mutex)
{
    unimplemented();
    return ENOSYS;
}

// TODO
int pthread_cond_broadcast(pthread_cond_t *c)
{
    unimplemented();
    return ENOSYS;
}

#define DEFAULT_STACK_SIZE 131072
#define DEFAULT_GUARD_SIZE 8192

// TODO
int pthread_attr_init(pthread_attr_t *a)
{
    *a = (pthread_attr_t){0};
    // __acquire_ptc();
    a->_a_stacksize = DEFAULT_STACK_SIZE;
    a->_a_guardsize = DEFAULT_GUARD_SIZE;
    // __release_ptc();
    return 0;
}

int pthread_attr_getstacksize(const pthread_attr_t *restrict a, size_t *restrict size)
{
    *size = a->_a_stacksize;
    return 0;
}

int pthread_attr_setstacksize(pthread_attr_t *a, size_t size)
{
    if (size - PTHREAD_STACK_MIN > SIZE_MAX / 4)
        return EINVAL;
    a->_a_stackaddr = 0;
    a->_a_stacksize = size;
    return 0;
}

#endif // AX_CONFIG_MULTITASK
