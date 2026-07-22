//! C-compatible Linux time ABI structures owned by the boundary crate.

use core::ffi::c_long;
use core::mem::{align_of, offset_of, size_of};

/// Clock ticks per second reported by `times(2)` (`USER_HZ`).
pub const USER_HZ: c_long = 100;

/// `struct tms` returned by `times(2)`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tms {
    /// User CPU time of the caller, in clock ticks.
    pub tms_utime: c_long,
    /// System CPU time of the caller, in clock ticks.
    pub tms_stime: c_long,
    /// User CPU time of waited-for children, in clock ticks.
    pub tms_cutime: c_long,
    /// System CPU time of waited-for children, in clock ticks.
    pub tms_cstime: c_long,
}

/// `struct rtc_time` filled by the `RTC_RD_TIME` ioctl.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RtcTime {
    /// Seconds (0-59).
    pub tm_sec: i32,
    /// Minutes (0-59).
    pub tm_min: i32,
    /// Hours (0-23).
    pub tm_hour: i32,
    /// Day of the month (1-31).
    pub tm_mday: i32,
    /// Months since January (0-11).
    pub tm_mon: i32,
    /// Years since 1900.
    pub tm_year: i32,
    /// Days since Sunday (0-6).
    pub tm_wday: i32,
    /// Days since January 1 (0-365).
    pub tm_yday: i32,
    /// Daylight saving time flag.
    pub tm_isdst: i32,
}

const _: () = {
    assert!(size_of::<Tms>() == 4 * size_of::<c_long>());
    assert!(align_of::<Tms>() == align_of::<c_long>());
    assert!(offset_of!(Tms, tms_utime) == 0);
    assert!(offset_of!(Tms, tms_stime) == size_of::<c_long>());
    assert!(offset_of!(Tms, tms_cutime) == 2 * size_of::<c_long>());
    assert!(offset_of!(Tms, tms_cstime) == 3 * size_of::<c_long>());

    assert!(size_of::<RtcTime>() == 9 * size_of::<i32>());
    assert!(align_of::<RtcTime>() == align_of::<i32>());
    assert!(offset_of!(RtcTime, tm_sec) == 0);
    assert!(offset_of!(RtcTime, tm_min) == size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_hour) == 2 * size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_mday) == 3 * size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_mon) == 4 * size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_year) == 5 * size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_wday) == 6 * size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_yday) == 7 * size_of::<i32>());
    assert!(offset_of!(RtcTime, tm_isdst) == size_of::<[i32; 8]>());
};
