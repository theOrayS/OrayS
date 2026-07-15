//! Linux ABI definitions shared by the OrayS compatibility boundary.
//!
//! This crate intentionally contains no kernel implementation types.  Policy,
//! user-memory access and syscall handlers belong to higher layers.

#![no_std]
#![forbid(unsafe_code)]

pub mod constants;
pub mod syscall;
pub mod time;
