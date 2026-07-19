#![no_std]

extern crate alloc;

#[cfg(any(test, feature = "host-tools"))]
extern crate std;

pub mod app;
pub mod apps;
pub mod desktop;
pub mod graphics;
pub mod platform;
pub mod widgets;
