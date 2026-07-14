//! C-compatible Linux time ABI structures owned by the boundary crate.

use core::ffi::c_long;
use core::mem::{align_of, offset_of, size_of};

pub const USER_HZ: c_long = 100;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tms {
    pub tms_utime: c_long,
    pub tms_stime: c_long,
    pub tms_cutime: c_long,
    pub tms_cstime: c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RtcTime {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
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
