#include <errno.h>
#include <pwd.h>
#include <stddef.h>
#include <string.h>

static int copy_passwd_string(char **cursor, size_t *remaining, const char *src, char **dst)
{
    size_t len = strlen(src) + 1;
    if (*remaining < len)
        return ERANGE;
    memcpy(*cursor, src, len);
    *dst = *cursor;
    *cursor += len;
    *remaining -= len;
    return 0;
}

static int fill_root_passwd(struct passwd *pw, char *buf, size_t size, struct passwd **res)
{
    if (!pw || !buf || !res)
        return EINVAL;

    char *cursor = buf;
    size_t remaining = size;
    int ret;

    if ((ret = copy_passwd_string(&cursor, &remaining, "root", &pw->pw_name)) != 0 ||
        (ret = copy_passwd_string(&cursor, &remaining, "x", &pw->pw_passwd)) != 0 ||
        (ret = copy_passwd_string(&cursor, &remaining, "root", &pw->pw_gecos)) != 0 ||
        (ret = copy_passwd_string(&cursor, &remaining, "/", &pw->pw_dir)) != 0 ||
        (ret = copy_passwd_string(&cursor, &remaining, "/bin/sh", &pw->pw_shell)) != 0) {
        *res = NULL;
        return ret;
    }

    pw->pw_uid = 0;
    pw->pw_gid = 0;
    *res = pw;
    return 0;
}

int getpwnam_r(const char *name, struct passwd *pw, char *buf, size_t size, struct passwd **res)
{
    if (!res)
        return EINVAL;
    *res = NULL;
    if (!name)
        return EINVAL;
    if (strcmp(name, "root") != 0)
        return 0;
    return fill_root_passwd(pw, buf, size, res);
}

int getpwuid_r(uid_t uid, struct passwd *pw, char *buf, size_t size, struct passwd **res)
{
    if (!res)
        return EINVAL;
    *res = NULL;
    if (uid != 0)
        return 0;
    return fill_root_passwd(pw, buf, size, res);
}
