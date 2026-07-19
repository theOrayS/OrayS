pub const RESTART_SUPPORTED: bool = false;
pub const CPU_USAGE_SUPPORTED: bool = false;
pub const MEMORY_USAGE_SUPPORTED: bool = false;
pub const WALL_CLOCK_SUPPORTED: bool = false;

#[cfg(feature = "orays")]
pub fn shutdown() -> ! {
    axstd::process::exit(0)
}
