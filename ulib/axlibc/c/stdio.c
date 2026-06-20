#include "printf.h"
#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// LOCK used by `puts()`
#ifdef AX_CONFIG_MULTITASK
#include <pthread.h>
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
#endif

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

FILE __stdin_FILE = {.fd = 0, .buffer_len = 0, .flags = 0};

FILE __stdout_FILE = {.fd = 1, .buffer_len = 0, .flags = 0};

FILE __stderr_FILE = {.fd = 2, .buffer_len = 0, .flags = 0};

FILE *const stdin = &__stdin_FILE;
FILE *const stdout = &__stdout_FILE;
FILE *const stderr = &__stderr_FILE;

static int __is_standard_stream(FILE *f)
{
    return f == stdin || f == stdout || f == stderr;
}

static void __mark_file_error(FILE *f)
{
    if (f)
        f->flags |= F_ERR;
}

static void __mark_file_eof(FILE *f)
{
    if (f)
        f->flags |= F_EOF;
}

// Returns: number of chars written if the full buffer was flushed, negative for
// failure.  On failure any unwritten tail remains in f->buf so a short write
// cannot be reported as success while silently dropping buffered data.
static int __write_buffer(FILE *f)
{
    if (f->buffer_len == 0)
        return 0;
    size_t written = 0;
    while (written < f->buffer_len) {
        ssize_t r = write(f->fd, f->buf + written, f->buffer_len - written);
        if (r > 0) {
            written += r;
            continue;
        }
        if (r == 0)
            errno = EIO;
        __mark_file_error(f);
        if (written > 0) {
            size_t remaining = f->buffer_len - written;
            memmove(f->buf, f->buf + written, remaining);
            f->buffer_len = remaining;
        }
        return -1;
    }
    f->buffer_len = 0;
    return written;
}

static int __fflush(FILE *f)
{
    int r = __write_buffer(f);
    return r >= 0 ? 0 : r;
}

static int out(FILE *f, const char *s, size_t l)
{
    int ret = 0;
    for (size_t i = 0; i < l; i++) {
        char c = s[i];
        f->buf[f->buffer_len++] = c;
        if (f->buffer_len == FILE_BUF_SIZE || c == '\n') {
            int r = __write_buffer(f);
            if (r < 0)
                return r;
            ret += r;
        }
    }
    return ret;
}

int getchar(void)
{
    return getc(stdin);
}

int getc(FILE *f)
{
    if (!f) {
        errno = EINVAL;
        return EOF;
    }
    unsigned char c;
    ssize_t len = read(f->fd, &c, 1);
    if (len == 1)
        return c;
    if (len == 0)
        __mark_file_eof(f);
    else
        __mark_file_error(f);
    return EOF;
}

int fflush(FILE *f)
{
    if (!f) {
        int ret = 0;
        if (__fflush(stdout) < 0)
            ret = EOF;
        if (__fflush(stderr) < 0)
            ret = EOF;
        return ret;
    }
    return __fflush(f);
}

static inline int do_putc(int c, FILE *f)
{
    char byte = c;
    return out(f, &byte, 1);
}

int fputc(int c, FILE *f)
{
    return do_putc(c, f);
}

int putc(int c, FILE *f)
{
    return do_putc(c, f);
}

int putchar(int c)
{
    return do_putc(c, stdout);
}

int puts(const char *s)
{
    if (!s) {
        errno = EINVAL;
        return EOF;
    }
#ifdef AX_CONFIG_MULTITASK
    pthread_mutex_lock(&lock);
#endif

    size_t len = strlen(s);
    int r = out(stdout, s, len);
    int ret = 0;
    if (r < 0) {
        ret = EOF;
        goto out;
    }
    r = out(stdout, "\n", 1);
    if (r < 0) {
        ret = EOF;
        goto out;
    }
    ret = (int)len + 1;

out:
#ifdef AX_CONFIG_MULTITASK
    pthread_mutex_unlock(&lock);
#endif

    return ret;
}

void perror(const char *msg)
{
    FILE *f = stderr;
    char *errstr = strerror(errno);

    if (msg && *msg) {
        out(f, msg, strlen(msg));
        out(f, ": ", 2);
    }
    out(f, errstr, strlen(errstr));
    out(f, "\n", 1);
}

static void __out_wrapper(char c, void *arg)
{
    out(arg, &c, 1);
}

int printf(const char *restrict fmt, ...)
{
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = vfprintf(stdout, fmt, ap);
    va_end(ap);
    return ret;
}

int fprintf(FILE *restrict f, const char *restrict fmt, ...)
{
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = vfprintf(f, fmt, ap);
    va_end(ap);
    return ret;
}

int vfprintf(FILE *restrict f, const char *restrict fmt, va_list ap)
{
    return vfctprintf(__out_wrapper, f, fmt, ap);
}

// TODO
int sscanf(const char *restrict __s, const char *restrict __format, ...)
{
    unimplemented();
    errno = ENOSYS;
    return EOF;
}

#ifdef AX_CONFIG_FS

static FILE *__new_file_for_fd(int fd)
{
    FILE *f = (FILE *)malloc(sizeof(FILE));
    if (!f) {
        errno = ENOMEM;
        return NULL;
    }
    f->fd = fd;
    f->buffer_len = 0;
    f->flags = 0;
    return f;
}

int __fmodeflags(const char *mode)
{
    int flags;
    if (strchr(mode, '+'))
        flags = O_RDWR;
    else if (*mode == 'r')
        flags = O_RDONLY;
    else
        flags = O_WRONLY;
    if (strchr(mode, 'x'))
        flags |= O_EXCL;
    if (strchr(mode, 'e'))
        flags |= O_CLOEXEC;
    if (*mode != 'r')
        flags |= O_CREAT;
    if (*mode == 'w')
        flags |= O_TRUNC;
    if (*mode == 'a')
        flags |= O_APPEND;
    return flags;
}

static int __fd_mode_compatible(int fd_flags, int mode_flags)
{
    int fd_access = fd_flags & O_ACCMODE;
    int mode_access = mode_flags & O_ACCMODE;
    if (mode_access == O_RDWR)
        return fd_access == O_RDWR;
    if (mode_access == O_WRONLY)
        return fd_access == O_WRONLY || fd_access == O_RDWR;
    if (mode_access == O_RDONLY)
        return fd_access == O_RDONLY || fd_access == O_RDWR;
    return 0;
}

FILE *fopen(const char *filename, const char *mode)
{
    FILE *f;
    int flags;

    if (!mode || !strchr("rwa", *mode)) {
        errno = EINVAL;
        return 0;
    }

    flags = __fmodeflags(mode);
    int fd = open(filename, flags, 0666);
    if (fd < 0)
        return NULL;

    f = __new_file_for_fd(fd);
    if (!f)
        close(fd);
    return f;
}

char *fgets(char *restrict s, int n, FILE *restrict f)
{
    if (n <= 0)
        return NULL;
    if (n == 1) {
        *s = '\0';
        return s;
    }

    int cnt = 0;
    while (cnt < n - 1) {
        char c;
        ssize_t len = read(f->fd, (void *)&c, 1);
        if (len > 0) {
            s[cnt++] = c;
            if (c == '\n')
                break;
        } else if (len == 0) {
            __mark_file_eof(f);
            break;
        } else {
            __mark_file_error(f);
            if (cnt == 0)
                return NULL;
            break;
        }
    }
    if (cnt == 0 && (f->flags & F_EOF))
        return NULL;
    s[cnt] = '\0';
    return s;
}

size_t fread(void *restrict destv, size_t size, size_t nmemb, FILE *restrict f)
{
    if (size == 0 || nmemb == 0)
        return 0;

    size_t total = size * nmemb;
    size_t read_len = 0;
    char *dest = (char *)destv;
    while (read_len < total) {
        ssize_t len = read(f->fd, dest + read_len, total - read_len);
        if (len > 0) {
            read_len += len;
        } else if (len == 0) {
            __mark_file_eof(f);
            break;
        } else {
            __mark_file_error(f);
            break;
        }
    }
    return read_len / size;
}

size_t fwrite(const void *restrict src, size_t size, size_t nmemb, FILE *restrict f)
{
    if (size == 0 || nmemb == 0)
        return 0;

    size_t total = size * nmemb;
    size_t write_len = 0;
    const char *bytes = (const char *)src;
    while (write_len < total) {
        ssize_t len = write(f->fd, bytes + write_len, total - write_len);
        if (len > 0) {
            write_len += len;
        } else if (len == 0) {
            break;
        } else {
            __mark_file_error(f);
            break;
        }
    }
    return write_len / size;
}

int fputs(const char *restrict s, FILE *restrict f)
{
    size_t l = strlen(s);
    return (fwrite(s, 1, l, f) == l) - 1;
}

int fclose(FILE *f)
{
    if (!f) {
        errno = EINVAL;
        return EOF;
    }
    int flush_ret = fflush(f);
    int close_ret = close(f->fd);
    int saved_errno = errno;
    if (!__is_standard_stream(f))
        free(f);
    if (flush_ret < 0 || close_ret < 0) {
        errno = saved_errno;
        return EOF;
    }
    return 0;
}

int fileno(FILE *f)
{
    return f->fd;
}

int feof(FILE *f)
{
    return f && (f->flags & F_EOF) ? 1 : 0;
}

// TODO
int fseek(FILE *__stream, long __off, int __whence)
{
    if (!__stream) {
        errno = EINVAL;
        return -1;
    }
    if (fflush(__stream) < 0)
        return -1;
    if (lseek(__stream->fd, __off, __whence) < 0) {
        __mark_file_error(__stream);
        return -1;
    }
    __stream->flags &= ~F_EOF;
    return 0;
}

off_t ftello(FILE *__stream)
{
    if (!__stream) {
        errno = EINVAL;
        return -1;
    }
    off_t pos = lseek(__stream->fd, 0, SEEK_CUR);
    if (pos < 0)
        __mark_file_error(__stream);
    return pos;
}

// TODO
char *tmpnam(char *buf)
{
    unimplemented();
    errno = ENOSYS;
    return NULL;
}

void clearerr(FILE *f)
{
    if (f)
        f->flags &= ~(F_EOF | F_ERR);
}

int ferror(FILE *f)
{
    return f && (f->flags & F_ERR) ? 1 : 0;
}

// TODO
FILE *freopen(const char *restrict filename, const char *restrict mode, FILE *restrict f)
{
    unimplemented();
    errno = ENOSYS;
    return NULL;
}

// TODO
int fscanf(FILE *restrict f, const char *restrict fmt, ...)
{
    unimplemented();
    errno = ENOSYS;
    return EOF;
}

long ftell(FILE *f)
{
    return ftello(f);
}

// TODO
int remove(const char *path)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO
int setvbuf(FILE *restrict f, char *restrict buf, int type, size_t size)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

// TODO
FILE *tmpfile(void)
{
    unimplemented();
    errno = ENOSYS;
    return NULL;
}

int ungetc(int c, FILE *f)
{
    unimplemented();
    errno = ENOSYS;
    return EOF;
}

ssize_t getdelim(char **restrict s, size_t *restrict n, int delim, FILE *restrict f)
{
    unimplemented();
    errno = ENOSYS;
    return -1;
}

ssize_t getline(char **restrict s, size_t *restrict n, FILE *restrict f)
{
    return getdelim(s, n, '\n', f);
}

int __uflow(FILE *f)
{
    return getc(f);
}

int getc_unlocked(FILE *f)
{
    return getc(f);
}

FILE *fdopen(int fd, const char *mode)
{
    if (!mode || !strchr("rwa", *mode)) {
        errno = EINVAL;
        return NULL;
    }
    int fd_flags = fcntl(fd, F_GETFL);
    if (fd_flags < 0)
        return NULL;
    int mode_flags = __fmodeflags(mode);
    if (!__fd_mode_compatible(fd_flags, mode_flags)) {
        errno = EBADF;
        return NULL;
    }
    if ((mode_flags & O_APPEND) && !(fd_flags & O_APPEND)) {
        if (fcntl(fd, F_SETFL, fd_flags | O_APPEND) < 0)
            return NULL;
        fd_flags = fcntl(fd, F_GETFL);
        if (fd_flags < 0)
            return NULL;
        if (!(fd_flags & O_APPEND)) {
            errno = EOPNOTSUPP;
            return NULL;
        }
    }
    return __new_file_for_fd(fd);
}

#endif // AX_CONFIG_FS
