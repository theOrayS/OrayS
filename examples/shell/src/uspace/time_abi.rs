use core::ffi::c_long;
use core::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use axerrno::LinuxError;
use linux_raw_sys::general;
use std::sync::Arc;

use super::linux_abi::SIGALRM_NUM;
use super::process_lifecycle::terminate_current_thread_for_exit_group;
use super::signal_abi::deliver_user_signal;
use super::task_context::{current_task_ext, current_tid};
use super::task_registry::{user_thread_entry_by_tid, user_thread_entry_for_process};
use super::user_memory::{read_user_value, write_user_value};
use super::{UserProcess, neg_errno};

static REALTIME_OFFSET_NS: AtomicI64 = AtomicI64::new(0);

const NSEC_PER_SEC: i128 = 1_000_000_000;
pub(super) const USER_HZ: c_long = 100;

impl UserProcess {
    pub(super) fn real_timer_armed(&self) -> bool {
        self.real_timer_deadline_us.load(Ordering::Acquire) != 0
    }

    fn take_expired_real_timer(&self, allow_interval: bool) -> Option<u64> {
        let deadline = self.real_timer_deadline_us.load(Ordering::Acquire);
        if deadline == 0 || monotonic_time_micros() < deadline {
            return None;
        }

        let interval = self.real_timer_interval_us.load(Ordering::Acquire);
        if interval != 0 && !allow_interval {
            return None;
        }
        let next_deadline = if interval == 0 {
            0
        } else {
            monotonic_time_micros().saturating_add(interval)
        };
        if self
            .real_timer_deadline_us
            .compare_exchange(deadline, next_deadline, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return None;
        }

        if interval == 0 {
            self.real_timer_generation.fetch_add(1, Ordering::AcqRel);
        }
        Some(interval)
    }

    pub(super) fn consume_expired_real_timer(&self) -> bool {
        if self.take_expired_real_timer(false).is_none() {
            return false;
        }
        if let Some(entry) = user_thread_entry_by_tid(current_tid()) {
            let _ = deliver_user_signal(&entry, SIGALRM_NUM, 0);
        }
        true
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Tms {
    pub(super) tms_utime: c_long,
    pub(super) tms_stime: c_long,
    pub(super) tms_cutime: c_long,
    pub(super) tms_cstime: c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct RtcTime {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct UserTimex {
    pub(super) modes: u32,
    pub(super) offset: c_long,
    pub(super) freq: c_long,
    pub(super) maxerror: c_long,
    pub(super) esterror: c_long,
    pub(super) status: i32,
    pub(super) constant: c_long,
    pub(super) precision: c_long,
    pub(super) tolerance: c_long,
    pub(super) time: general::timeval,
    pub(super) tick: c_long,
    pub(super) ppsfreq: c_long,
    pub(super) jitter: c_long,
    pub(super) shift: i32,
    pub(super) stabil: c_long,
    pub(super) jitcnt: c_long,
    pub(super) calcnt: c_long,
    pub(super) errcnt: c_long,
    pub(super) stbcnt: c_long,
    pub(super) tai: i32,
    pub(super) __padding: [i32; 11],
}

pub(super) fn socket_timeval_to_duration(
    value: general::timeval,
) -> Result<Option<core::time::Duration>, LinuxError> {
    if value.tv_sec < 0 || value.tv_usec < 0 || value.tv_usec >= 1_000_000 {
        return Err(LinuxError::EINVAL);
    }
    if value.tv_sec == 0 && value.tv_usec == 0 {
        Ok(None)
    } else {
        Ok(Some(core::time::Duration::new(
            value.tv_sec as u64,
            value.tv_usec as u32 * 1000,
        )))
    }
}

pub(super) fn socket_duration_to_timeval(
    timeout: Option<core::time::Duration>,
) -> general::timeval {
    match timeout {
        Some(timeout) => general::timeval {
            tv_sec: timeout.as_secs().min(i64::MAX as u64) as _,
            tv_usec: timeout.subsec_micros() as _,
        },
        None => general::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    }
}

fn duration_to_micros(duration: core::time::Duration) -> u64 {
    duration
        .as_secs()
        .saturating_mul(1_000_000)
        .saturating_add(duration.subsec_micros() as u64)
}

pub(super) fn micros_to_duration(micros: u64) -> core::time::Duration {
    core::time::Duration::new(micros / 1_000_000, ((micros % 1_000_000) as u32) * 1000)
}

pub(super) fn timeval_to_micros(value: general::timeval) -> Result<u64, LinuxError> {
    Ok(socket_timeval_to_duration(value)?
        .map(duration_to_micros)
        .unwrap_or(0))
}

pub(super) fn micros_to_timeval(micros: u64) -> general::timeval {
    general::timeval {
        tv_sec: (micros / 1_000_000).min(i64::MAX as u64) as _,
        tv_usec: (micros % 1_000_000) as _,
    }
}

pub(super) fn timespec_from_duration(duration: core::time::Duration) -> general::timespec {
    general::timespec {
        tv_sec: duration.as_secs() as _,
        tv_nsec: duration.subsec_nanos() as _,
    }
}

pub(super) fn clock_gettime_timespec(clockid: u32) -> Result<general::timespec, LinuxError> {
    clock_now_duration(clockid).map(timespec_from_duration)
}

pub(super) fn timeval_from_duration(duration: core::time::Duration) -> general::timeval {
    general::timeval {
        tv_sec: duration.as_secs() as _,
        tv_usec: duration.subsec_micros() as _,
    }
}

pub(super) fn clock_resolution_timespec() -> general::timespec {
    general::timespec {
        tv_sec: 0,
        tv_nsec: 1,
    }
}

pub(super) fn clock_getres_timespec(clockid: u32) -> Result<general::timespec, LinuxError> {
    validate_clock_id(clockid)?;
    Ok(clock_resolution_timespec())
}

pub(super) fn zero_timespec() -> general::timespec {
    general::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }
}

pub(super) fn zero_timezone() -> general::timezone {
    general::timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    }
}

pub(super) fn current_timeval() -> general::timeval {
    timeval_from_duration(adjusted_wall_time())
}

pub(super) fn clock_ticks_now() -> u64 {
    let micros = axhal::time::monotonic_time()
        .as_micros()
        .min(u64::MAX as u128) as u64;
    micros.saturating_mul(USER_HZ as u64) / 1_000_000
}

pub(super) fn process_times(process: &UserProcess) -> Tms {
    let elapsed = clock_ticks_now()
        .saturating_sub(process.start_clock_ticks.load(Ordering::Acquire))
        .min(c_long::MAX as u64) as c_long;
    let user_ticks = elapsed / 2;
    let system_ticks = elapsed.saturating_sub(user_ticks);
    Tms {
        tms_utime: user_ticks,
        tms_stime: system_ticks,
        tms_cutime: process
            .waited_child_user_ticks
            .load(Ordering::Acquire)
            .min(c_long::MAX as u64) as c_long,
        tms_cstime: process
            .waited_child_system_ticks
            .load(Ordering::Acquire)
            .min(c_long::MAX as u64) as c_long,
    }
}

pub(super) fn monotonic_time_micros() -> u64 {
    axhal::time::monotonic_time()
        .as_micros()
        .min(u64::MAX as u128) as u64
}

pub(super) fn times_ticks() -> isize {
    clock_ticks_now().min(isize::MAX as u64) as isize
}

pub(super) fn timespec_to_duration(
    ts: general::timespec,
) -> Result<core::time::Duration, LinuxError> {
    if ts.tv_sec < 0 || ts.tv_nsec < 0 || ts.tv_nsec >= 1_000_000_000 {
        return Err(LinuxError::EINVAL);
    }
    Ok(core::time::Duration::new(
        ts.tv_sec as u64,
        ts.tv_nsec as u32,
    ))
}

pub(super) fn clock_now_duration(clockid: u32) -> Result<core::time::Duration, LinuxError> {
    match clockid {
        general::CLOCK_REALTIME | general::CLOCK_REALTIME_COARSE | general::CLOCK_TAI => {
            Ok(adjusted_wall_time())
        }
        general::CLOCK_MONOTONIC
        | general::CLOCK_MONOTONIC_RAW
        | general::CLOCK_MONOTONIC_COARSE
        | general::CLOCK_BOOTTIME
        | general::CLOCK_PROCESS_CPUTIME_ID
        | general::CLOCK_THREAD_CPUTIME_ID => Ok(axhal::time::monotonic_time()),
        general::CLOCK_REALTIME_ALARM | general::CLOCK_BOOTTIME_ALARM => Err(LinuxError::EINVAL),
        _ => Err(LinuxError::EINVAL),
    }
}

pub(super) fn adjusted_wall_time() -> core::time::Duration {
    let raw_ns = duration_to_ns_i128(axhal::time::wall_time());
    let offset_ns = REALTIME_OFFSET_NS.load(Ordering::Acquire) as i128;
    let adjusted_ns = raw_ns + offset_ns;
    if adjusted_ns <= 0 {
        return core::time::Duration::ZERO;
    }
    let secs = (adjusted_ns / NSEC_PER_SEC).min(u64::MAX as i128) as u64;
    let nanos = (adjusted_ns % NSEC_PER_SEC) as u32;
    core::time::Duration::new(secs, nanos)
}

pub(super) fn set_realtime_offset_from_timespec(ts: general::timespec) {
    let target_ns = ts.tv_sec as i128 * NSEC_PER_SEC + ts.tv_nsec as i128;
    let raw_ns = duration_to_ns_i128(axhal::time::wall_time());
    REALTIME_OFFSET_NS.store(clamp_i128_to_i64(target_ns - raw_ns), Ordering::Release);
}

fn duration_to_ns_i128(duration: core::time::Duration) -> i128 {
    duration.as_secs() as i128 * NSEC_PER_SEC + duration.subsec_nanos() as i128
}

fn clamp_i128_to_i64(value: i128) -> i64 {
    value.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

pub(super) fn rtc_time_from_wall_time() -> RtcTime {
    let now = adjusted_wall_time();
    let total_secs = now.as_secs() as i64;
    let days = total_secs.div_euclid(86_400);
    let secs_of_day = total_secs.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);

    RtcTime {
        tm_sec: (secs_of_day % 60) as i32,
        tm_min: ((secs_of_day / 60) % 60) as i32,
        tm_hour: (secs_of_day / 3600) as i32,
        tm_mday: day,
        tm_mon: month - 1,
        tm_year: year - 1900,
        tm_wday: (days + 4).rem_euclid(7) as i32,
        tm_yday: year_day(year, month, day),
        tm_isdst: 0,
    }
}

fn civil_from_days(days: i64) -> (i32, i32, i32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    (year as i32, month as i32, day as i32)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn year_day(year: i32, month: i32, day: i32) -> i32 {
    const DAYS_BEFORE_MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let mut yday = DAYS_BEFORE_MONTH[(month - 1) as usize] + day - 1;
    if month > 2 && is_leap_year(year) {
        yday += 1;
    }
    yday
}

pub(super) fn validate_clock_id(clockid: u32) -> Result<(), LinuxError> {
    clock_now_duration(clockid).map(|_| ())
}

pub(super) fn adjtimex_input_valid(input: UserTimex) -> bool {
    let modes = input.modes;
    if !adjtimex_modes_valid(modes) {
        return false;
    }
    if modes & ADJ_TICK != 0 {
        return adjtimex_tick_valid(input.tick);
    }
    true
}

pub(super) fn adjtimex_changes_clock(input: UserTimex) -> bool {
    input.modes != 0
}

fn default_timex() -> UserTimex {
    let now = adjusted_wall_time();
    let mut output: UserTimex = unsafe { core::mem::zeroed() };
    output.precision = 1;
    output.time = timeval_from_duration(now);
    output.tick = 10_000;
    output
}

pub(super) fn write_default_timex(process: &UserProcess, tx: usize) -> isize {
    let output = default_timex();
    write_user_value(process, tx, &output)
}

pub(super) fn itimerval_to_micros_pair(
    value: general::itimerval,
) -> Result<(u64, u64), LinuxError> {
    let first_us = timeval_to_micros(value.it_value)?;
    let interval_us = timeval_to_micros(value.it_interval)?;
    Ok((first_us, interval_us))
}

fn itimer_cells(process: &UserProcess, which: i32) -> Result<(&AtomicU64, &AtomicU64), LinuxError> {
    match which {
        value if value == general::ITIMER_REAL as i32 => Ok((
            &process.real_timer_deadline_us,
            &process.real_timer_interval_us,
        )),
        value if value == general::ITIMER_VIRTUAL as i32 => Ok((
            &process.virtual_timer_deadline_us,
            &process.virtual_timer_interval_us,
        )),
        value if value == general::ITIMER_PROF as i32 => Ok((
            &process.prof_timer_deadline_us,
            &process.prof_timer_interval_us,
        )),
        _ => Err(LinuxError::EINVAL),
    }
}

fn current_itimer(process: &UserProcess, which: i32) -> Result<general::itimerval, LinuxError> {
    let (deadline_cell, interval_cell) = itimer_cells(process, which)?;
    let deadline = deadline_cell.load(Ordering::Acquire);
    let remaining = if deadline == 0 {
        0
    } else {
        deadline.saturating_sub(monotonic_time_micros())
    };
    Ok(general::itimerval {
        it_interval: micros_to_timeval(interval_cell.load(Ordering::Acquire)),
        it_value: micros_to_timeval(remaining),
    })
}

fn arm_real_itimer(
    process: Arc<UserProcess>,
    generation: u64,
    first_delay_us: u64,
    interval_us: u64,
) {
    let _ = axtask::spawn(move || {
        let mut delay_us = first_delay_us;
        loop {
            if delay_us == 0 {
                axtask::yield_now();
            } else {
                axtask::sleep(micros_to_duration(delay_us));
            }
            if process.real_timer_generation.load(Ordering::Acquire) != generation
                || process.live_threads.load(Ordering::Acquire) == 0
            {
                break;
            }
            if process.take_expired_real_timer(true).is_some() {
                if let Some(entry) = user_thread_entry_for_process(&process) {
                    let _ = deliver_user_signal(&entry, SIGALRM_NUM, 0);
                }
            }
            if interval_us == 0 {
                break;
            }
            delay_us = interval_us;
        }
    });
}

pub(super) fn sys_setitimer(
    process: &Arc<UserProcess>,
    which: i32,
    new_value: usize,
    old_value: usize,
) -> isize {
    let (deadline_cell, interval_cell) = match itimer_cells(process, which) {
        Ok(cells) => cells,
        Err(err) => return neg_errno(err),
    };
    if old_value != 0 {
        let value = match current_itimer(process, which) {
            Ok(value) => value,
            Err(err) => return neg_errno(err),
        };
        let ret = write_user_value(process, old_value, &value);
        if ret != 0 {
            return ret;
        }
    }

    let new_timer = if new_value == 0 {
        None
    } else {
        match read_user_value::<general::itimerval>(process, new_value) {
            Ok(value) => Some(value),
            Err(_) => return neg_errno(LinuxError::EFAULT),
        }
    };
    let (first_us, interval_us) = match new_timer {
        Some(value) => match itimerval_to_micros_pair(value) {
            Ok(pair) => pair,
            Err(err) => return neg_errno(err),
        },
        None => (0, 0),
    };

    interval_cell.store(interval_us, Ordering::Release);
    if first_us == 0 {
        deadline_cell.store(0, Ordering::Release);
    } else {
        deadline_cell.store(
            monotonic_time_micros().saturating_add(first_us),
            Ordering::Release,
        );
    }
    if which == general::ITIMER_REAL as i32 {
        let generation = process.real_timer_generation.fetch_add(1, Ordering::AcqRel) + 1;
        if first_us != 0 {
            // Only ITIMER_REAL currently delivers SIGALRM.  The virtual/prof
            // slots are still tracked so getitimer/setitimer report real state
            // instead of ENOSYS/EINVAL to user space.
            arm_real_itimer(process.clone(), generation, first_us, interval_us);
        }
    }
    0
}

pub(super) fn sys_getitimer(process: &UserProcess, which: i32, curr_value: usize) -> isize {
    let value = match current_itimer(process, which) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    if curr_value == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    write_user_value(process, curr_value, &value)
}

pub(super) fn read_timespec_duration(
    process: &UserProcess,
    ptr: usize,
) -> Result<core::time::Duration, LinuxError> {
    let ts = read_user_value::<general::timespec>(process, ptr)?;
    timespec_to_duration(ts)
}

pub(super) fn sleep_duration(duration: core::time::Duration) {
    if duration.as_nanos() == 0 {
        return;
    }
    let deadline = axhal::time::wall_time() + duration;
    while axhal::time::wall_time() < deadline {
        if let Some(ext) = current_task_ext()
            && let Some(code) = ext.process.pending_exit_group()
        {
            terminate_current_thread_for_exit_group(ext.process.as_ref(), code);
        }
        axtask::yield_now();
    }
}

pub(super) fn sys_clock_gettime(process: &UserProcess, clk_id: usize, tp: usize) -> isize {
    let ts = match clock_gettime_timespec(clk_id as u32) {
        Ok(ts) => ts,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, tp, &ts)
}

pub(super) fn sys_clock_settime(process: &UserProcess, clk_id: usize, tp: usize) -> isize {
    if clk_id != general::CLOCK_REALTIME as usize {
        return neg_errno(LinuxError::EINVAL);
    }
    let ts = match read_user_value::<general::timespec>(process, tp) {
        Ok(ts) => ts,
        Err(err) => return neg_errno(err),
    };
    if ts.tv_sec < 0 || !(0..1_000_000_000).contains(&ts.tv_nsec) {
        return neg_errno(LinuxError::EINVAL);
    }
    set_realtime_offset_from_timespec(ts);
    0
}

pub(super) fn sys_clock_getres(process: &UserProcess, clk_id: usize, tp: usize) -> isize {
    let ts = match clock_getres_timespec(clk_id as u32) {
        Ok(ts) => ts,
        Err(err) => return neg_errno(err),
    };
    if tp == 0 {
        return 0;
    }
    write_user_value(process, tp, &ts)
}

pub(super) fn sys_gettimeofday(process: &UserProcess, tv: usize, tz: usize) -> isize {
    if tv != 0 {
        let value = current_timeval();
        let ret = write_user_value(process, tv, &value);
        if ret != 0 {
            return ret;
        }
    }
    if tz != 0 {
        let value = zero_timezone();
        let ret = write_user_value(process, tz, &value);
        if ret != 0 {
            return ret;
        }
    }
    0
}

pub(super) fn sys_adjtimex(process: &UserProcess, tx: usize) -> isize {
    const TIME_OK: isize = 0;

    let input = match read_user_value::<UserTimex>(process, tx) {
        Ok(input) => input,
        Err(err) => return neg_errno(err),
    };
    if !adjtimex_input_valid(input) {
        return neg_errno(LinuxError::EINVAL);
    }
    if adjtimex_changes_clock(input) && process.uid() != 0 {
        return neg_errno(LinuxError::EPERM);
    }

    let ret = write_default_timex(process, tx);
    if ret != 0 {
        return ret;
    }
    TIME_OK
}

pub(super) fn sys_clock_adjtime(process: &UserProcess, clk_id: usize, tx: usize) -> isize {
    if clk_id != general::CLOCK_REALTIME as usize {
        return neg_errno(LinuxError::EINVAL);
    }
    sys_adjtimex(process, tx)
}

pub(super) fn sys_times(process: &UserProcess, buf: usize) -> isize {
    let tms = process_times(process);
    let ret = write_user_value(process, buf, &tms);
    if ret != 0 {
        return ret;
    }
    times_ticks()
}

pub(super) fn sys_nanosleep(process: &UserProcess, req: usize, rem: usize) -> isize {
    let duration = match read_timespec_duration(process, req) {
        Ok(duration) => duration,
        Err(err) => return neg_errno(err),
    };
    sleep_duration(duration);
    if rem != 0 {
        let zero = zero_timespec();
        let ret = write_user_value(process, rem, &zero);
        if ret != 0 {
            return ret;
        }
    }
    0
}

pub(super) fn sys_clock_nanosleep(
    process: &UserProcess,
    clockid: usize,
    flags: usize,
    req: usize,
    rem: usize,
) -> isize {
    let duration = match read_timespec_duration(process, req) {
        Ok(duration) => duration,
        Err(err) => return neg_errno(err),
    };
    if flags as u32 & !general::TIMER_ABSTIME != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if flags as u32 & general::TIMER_ABSTIME != 0 {
        let now = match clock_now_duration(clockid as u32) {
            Ok(now) => now,
            Err(err) => return neg_errno(err),
        };
        if let Some(delta) = duration.checked_sub(now) {
            sleep_duration(delta);
        }
        return 0;
    }
    sys_nanosleep(process, req, rem)
}

const ADJ_OFFSET: u32 = 0x0001;
const ADJ_FREQUENCY: u32 = 0x0002;
const ADJ_MAXERROR: u32 = 0x0004;
const ADJ_ESTERROR: u32 = 0x0008;
const ADJ_STATUS: u32 = 0x0010;
const ADJ_TIMECONST: u32 = 0x0020;
const ADJ_TAI: u32 = 0x0080;
const ADJ_SETOFFSET: u32 = 0x0100;
const ADJ_MICRO: u32 = 0x1000;
const ADJ_NANO: u32 = 0x2000;
const ADJ_TICK: u32 = 0x4000;
const ADJ_OFFSET_SINGLESHOT: u32 = 0x8001;
const ADJ_OFFSET_SS_READ: u32 = 0xa001;

const ADJ_REGULAR_MASK: u32 = ADJ_OFFSET
    | ADJ_FREQUENCY
    | ADJ_MAXERROR
    | ADJ_ESTERROR
    | ADJ_STATUS
    | ADJ_TIMECONST
    | ADJ_TAI
    | ADJ_SETOFFSET
    | ADJ_MICRO
    | ADJ_NANO
    | ADJ_TICK;

fn adjtimex_modes_valid(modes: u32) -> bool {
    modes & !ADJ_REGULAR_MASK == 0 || modes == ADJ_OFFSET_SINGLESHOT || modes == ADJ_OFFSET_SS_READ
}

fn adjtimex_tick_valid(tick: c_long) -> bool {
    let min_tick = 900_000 / USER_HZ;
    let max_tick = 1_100_000 / USER_HZ;
    tick >= min_tick && tick <= max_tick
}
