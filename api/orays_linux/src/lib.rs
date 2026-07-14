//! Implementation-independent Linux compatibility boundary types.
//!
//! Kernel policy, process ownership and user-memory access implementations
//! belong to backend crates such as `arceos-shell`.

#![no_std]
#![forbid(unsafe_code)]

pub use orays_linux_abi as abi;

pub mod backend;
pub mod user;
