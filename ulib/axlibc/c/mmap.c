#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <sys/mman.h>

// TODO:
void *mmap(void *addr, size_t len, int prot, int flags, int fildes, off_t off)
{
    unimplemented();
    errno = ENOSYS;
    return MAP_FAILED;
}

// TODO:
int munmap(void *addr, size_t length)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO:
void *mremap(void *old_address, size_t old_size, size_t new_size, int flags,
             ... /* void *new_address */)
{
    unimplemented();
    errno = ENOSYS;
    return MAP_FAILED;
}

// TODO
int mprotect(void *addr, size_t len, int prot)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO
int madvise(void *addr, size_t len, int advice)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}
