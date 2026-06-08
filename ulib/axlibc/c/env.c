#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static char *initial_environ[] = {NULL};
char **environ = initial_environ;
static char **owned_environ;
static size_t environ_count;
static size_t environ_capacity;

static int env_name_len(const char *name, size_t *len)
{
    if (!name || !*name || strchr(name, '=')) {
        errno = EINVAL;
        return -1;
    }
    *len = strlen(name);
    return 0;
}

static ssize_t env_find(const char *name, size_t name_len)
{
    if (!environ)
        return -1;
    for (size_t i = 0; environ[i]; i++) {
        if (!strncmp(name, environ[i], name_len) && environ[i][name_len] == '=')
            return (ssize_t)i;
    }
    return -1;
}

static size_t env_count(char **env)
{
    size_t count = 0;
    if (env) {
        while (env[count])
            count++;
    }
    return count;
}

static char *env_strdup(const char *src)
{
    size_t len = strlen(src) + 1;
    char *dst = malloc(len);
    if (!dst) {
        errno = ENOMEM;
        return NULL;
    }
    memcpy(dst, src, len);
    return dst;
}

static void env_free_owned(char **env, size_t count)
{
    if (!env)
        return;
    for (size_t i = 0; i < count; i++)
        free(env[i]);
    free(env);
}

static int env_ensure_owned(void)
{
    if (environ == owned_environ) {
        environ_count = env_count(environ);
        return 0;
    }

    size_t count = env_count(environ);
    size_t capacity = count + 1;
    if (capacity < 4)
        capacity = 4;
    char **new_env = malloc(capacity * sizeof(char *));
    if (!new_env) {
        errno = ENOMEM;
        return -1;
    }

    size_t copied = 0;
    for (; copied < count; copied++) {
        new_env[copied] = env_strdup(environ[copied]);
        if (!new_env[copied]) {
            env_free_owned(new_env, copied);
            return -1;
        }
    }
    new_env[count] = NULL;

    size_t old_count = owned_environ ? env_count(owned_environ) : 0;
    env_free_owned(owned_environ, old_count);
    owned_environ = new_env;
    environ = owned_environ;
    environ_count = count;
    environ_capacity = capacity;
    return 0;
}

static int env_reserve(size_t needed)
{
    if (env_ensure_owned() != 0)
        return -1;
    if (environ_capacity >= needed)
        return 0;
    size_t new_capacity = environ_capacity ? environ_capacity * 2 : 4;
    while (new_capacity < needed)
        new_capacity *= 2;
    char **new_env = realloc(owned_environ, new_capacity * sizeof(char *));
    if (!new_env) {
        errno = ENOMEM;
        return -1;
    }
    owned_environ = new_env;
    environ = new_env;
    environ_capacity = new_capacity;
    return 0;
}

char *getenv(const char *name)
{
    size_t l;
    if (env_name_len(name, &l) != 0)
        return 0;
    ssize_t idx = env_find(name, l);
    return idx >= 0 ? environ[idx] + l + 1 : 0;
}

int setenv(const char *__name, const char *__value, int __replace)
{
    size_t name_len;
    if (env_name_len(__name, &name_len) != 0)
        return -1;
    if (!__value)
        __value = "";
    if (env_ensure_owned() != 0)
        return -1;

    ssize_t idx = env_find(__name, name_len);
    if (idx >= 0 && !__replace)
        return 0;

    size_t value_len = strlen(__value);
    char *entry = malloc(name_len + 1 + value_len + 1);
    if (!entry) {
        errno = ENOMEM;
        return -1;
    }
    memcpy(entry, __name, name_len);
    entry[name_len] = '=';
    memcpy(entry + name_len + 1, __value, value_len + 1);

    if (idx >= 0) {
        free(environ[idx]);
        environ[idx] = entry;
        return 0;
    }

    if (env_reserve(environ_count + 2) != 0) {
        free(entry);
        return -1;
    }
    environ[environ_count++] = entry;
    environ[environ_count] = NULL;
    return 0;
}

int unsetenv(const char *__name)
{
    size_t name_len;
    if (env_name_len(__name, &name_len) != 0)
        return -1;
    if (env_ensure_owned() != 0)
        return -1;
    ssize_t idx = env_find(__name, name_len);
    if (idx < 0)
        return 0;

    free(environ[idx]);
    for (size_t i = (size_t)idx; i < environ_count; i++)
        environ[i] = environ[i + 1];
    environ_count--;
    return 0;
}
