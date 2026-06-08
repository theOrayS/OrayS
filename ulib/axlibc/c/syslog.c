#include <stdarg.h>
#include <stdio.h>
#include <string.h>
#include <syslog.h>
#include <unistd.h>

static char syslog_ident[64];
static int syslog_option;
static int syslog_facility = LOG_USER;

void openlog(const char *__ident, int __option, int __facility)
{
    syslog_option = __option;
    syslog_facility = __facility ? __facility : LOG_USER;
    if (__ident && *__ident) {
        strncpy(syslog_ident, __ident, sizeof(syslog_ident) - 1);
        syslog_ident[sizeof(syslog_ident) - 1] = '\0';
    } else {
        syslog_ident[0] = '\0';
    }
}

void syslog(int __pri, const char *__fmt, ...)
{
    int facility = __pri & ~0x07;
    int pri = (__pri & 0x07) | (facility ? facility : syslog_facility);

    if (!(syslog_option & LOG_PERROR))
        return;

    fprintf(stderr, "<%d>", pri);
    if (syslog_ident[0]) {
        fprintf(stderr, "%s", syslog_ident);
        if (syslog_option & LOG_PID)
            fprintf(stderr, "[%d]", (int)getpid());
        fprintf(stderr, ": ");
    }

    va_list ap;
    va_start(ap, __fmt);
    vfprintf(stderr, __fmt ? __fmt : "", ap);
    va_end(ap);
    fprintf(stderr, "\n");
    fflush(stderr);
}
