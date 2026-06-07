#include <errno.h>
#include <dlfcn.h>
#include <pthread.h>
#include <stdio.h>

// TODO
int dladdr(const void *__address, Dl_info *__info)
{
    errno = ENOSYS;
    return 0;
}

// TODO
void *dlopen(const char *__file, int __mode)
{
    unimplemented();
    errno = ENOSYS;
    return NULL;
}

// TODO
char *dlerror()
{
    return "dynamic loading is not implemented";
}

// TODO
void *dlsym(void *__restrict__ __handle, const char *__restrict__ __name)
{

    unimplemented();
    errno = ENOSYS;
    return NULL;
}

// TODO
int dlclose(void *p)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
