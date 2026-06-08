#include <errno.h>
#include <signal.h>
#include <stddef.h>
#include <stdio.h>
#include <unistd.h>

static struct sigaction signal_actions[_NSIG];
static sigset_t signal_mask;
static sigset_t pending_signals;

static int valid_signal(int signum)
{
    return signum > 0 && signum < _NSIG;
}

static unsigned signal_index(int signum)
{
    return (unsigned)signum - 1;
}

static unsigned signal_word(int signum)
{
    return signal_index(signum) / (8 * sizeof signal_mask.__bits[0]);
}

static unsigned long signal_bit(int signum)
{
    return 1UL << (signal_index(signum) & (8 * sizeof signal_mask.__bits[0] - 1));
}

static int signal_set_contains(const sigset_t *set, int signum)
{
    return set->__bits[signal_word(signum)] & signal_bit(signum);
}

static void signal_set_add(sigset_t *set, int signum)
{
    set->__bits[signal_word(signum)] |= signal_bit(signum);
}

static void signal_set_del(sigset_t *set, int signum)
{
    set->__bits[signal_word(signum)] &= ~signal_bit(signum);
}

static void remove_unmaskable_signals(sigset_t *set)
{
    signal_set_del(set, SIGKILL);
    signal_set_del(set, SIGSTOP);
}

static int default_action(int signum)
{
    switch (signum) {
    case SIGCHLD:
    case SIGURG:
    case SIGWINCH:
    case SIGCONT:
        return 0;
    case SIGSTOP:
    case SIGTSTP:
    case SIGTTIN:
    case SIGTTOU:
        errno = ENOSYS;
        return -1;
    default:
        _exit(128 + signum);
    }
}

static int deliver_signal_now(int signum)
{
    struct sigaction action = signal_actions[signum];
    void (*handler)(int) = action.sa_handler;

    if (handler == SIG_IGN)
        return 0;
    if (handler == SIG_DFL)
        return default_action(signum);

    if (action.sa_flags & SA_RESETHAND)
        signal_actions[signum] = (struct sigaction){0};

    sigset_t saved_mask = signal_mask;
    sigset_t next_mask = saved_mask;
    for (int blocked = 1; blocked < _NSIG; blocked++) {
        if (signal_set_contains(&action.sa_mask, blocked))
            signal_set_add(&next_mask, blocked);
    }
    remove_unmaskable_signals(&next_mask);
    if (!(action.sa_flags & SA_NODEFER))
        signal_set_add(&next_mask, signum);
    signal_mask = next_mask;

    if (action.sa_flags & SA_SIGINFO) {
        siginfo_t info = {
            .si_signo = signum,
            .si_errno = 0,
            .si_code = SI_USER,
        };
        action.sa_sigaction(signum, &info, NULL);
    } else {
        handler(signum);
    }

    signal_mask = saved_mask;
    return 0;
}

static int deliver_unblocked_pending(void)
{
    for (int signum = 1; signum < _NSIG; signum++) {
        if (signal_set_contains(&pending_signals, signum) &&
            !signal_set_contains(&signal_mask, signum)) {
            signal_set_del(&pending_signals, signum);
            if (deliver_signal_now(signum) < 0)
                return -1;
        }
    }
    return 0;
}

int sigaction_helper(int signum, const struct sigaction *act, struct sigaction *oldact,
                     size_t sigsetsize)
{
    if (sigsetsize != sizeof(sigset_t) || !valid_signal(signum) || signum == SIGKILL ||
        signum == SIGSTOP) {
        errno = EINVAL;
        return -1;
    }

    if (oldact)
        *oldact = signal_actions[signum];

    if (act) {
        struct sigaction next = *act;
        remove_unmaskable_signals(&next.sa_mask);
        signal_actions[signum] = next;
    }

    return 0;
}

void (*signal(int signum, void (*handler)(int)))(int)
{
    struct sigaction old;
    struct sigaction act = {
        .sa_handler = handler, .sa_flags = SA_RESTART, /* BSD signal semantics */
    };

    if (sigaction_helper(signum, &act, &old, sizeof(sigset_t)) < 0)
        return SIG_ERR;

    return (old.sa_flags & SA_SIGINFO) ? NULL : old.sa_handler;
}

int sigaction(int sig, const struct sigaction *restrict act, struct sigaction *restrict oact)
{
    return sigaction_helper(sig, act, oact, sizeof(sigset_t));
}

int kill(pid_t __pid, int __sig)
{
    if (__sig && !valid_signal(__sig)) {
        errno = EINVAL;
        return -1;
    }
    if (__pid == getpid()) {
        if (__sig == 0)
            return 0;
        return raise(__sig);
    }
    if (__pid <= 0) {
        errno = ENOSYS;
        return -1;
    }
    errno = ESRCH;
    return -1;
}

int sigemptyset(sigset_t *set)
{
    set->__bits[0] = 0;
    if (sizeof(long) == 4 || _NSIG > 65)
        set->__bits[1] = 0;
    if (sizeof(long) == 4 && _NSIG > 65) {
        set->__bits[2] = 0;
        set->__bits[3] = 0;
    }
    return 0;
}

int raise(int __sig)
{
    if (!valid_signal(__sig)) {
        errno = EINVAL;
        return -1;
    }
    if (signal_set_contains(&signal_mask, __sig)) {
        signal_set_add(&pending_signals, __sig);
        return 0;
    }
    return deliver_signal_now(__sig);
}

int sigaddset(sigset_t *set, int sig)
{
    unsigned s = sig - 1;
    if (s >= _NSIG - 1 || sig - 32U < 3) {
        errno = EINVAL;
        return -1;
    }
    set->__bits[s / 8 / sizeof *set->__bits] |= 1UL << (s & (8 * sizeof *set->__bits - 1));
    return 0;
}

int pthread_sigmask(int __how, const sigset_t *restrict __newmask, sigset_t *restrict __oldmask)
{
    if (__oldmask)
        *__oldmask = signal_mask;
    if (!__newmask)
        return 0;

    sigset_t next = *__newmask;
    remove_unmaskable_signals(&next);
    switch (__how) {
    case SIG_BLOCK:
        for (int signum = 1; signum < _NSIG; signum++) {
            if (signal_set_contains(&next, signum))
                signal_set_add(&signal_mask, signum);
        }
        break;
    case SIG_UNBLOCK:
        for (int signum = 1; signum < _NSIG; signum++) {
            if (signal_set_contains(&next, signum))
                signal_set_del(&signal_mask, signum);
        }
        break;
    case SIG_SETMASK:
        signal_mask = next;
        break;
    default:
        return EINVAL;
    }
    return deliver_unblocked_pending() < 0 ? errno : 0;
}

#ifdef AX_CONFIG_MULTITASK
int pthread_kill(pthread_t t, int sig)
{
    if (sig && !valid_signal(sig))
        return EINVAL;
    if (t != pthread_self())
        return ENOSYS;
    if (sig == 0)
        return 0;
    return raise(sig) < 0 ? errno : 0;
}
#endif
