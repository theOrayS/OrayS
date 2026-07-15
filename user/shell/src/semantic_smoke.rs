use std::fs::File;
use std::io::Write;

const USER_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pr3-semantic-smoke"));
const USER_PATH: &str = "/tmp/pr3-semantic-smoke";

#[cfg(target_arch = "riscv64")]
const ARCH: &str = "riscv64";
#[cfg(target_arch = "loongarch64")]
const ARCH: &str = "loongarch64";

fn harness_fail(reason: &str) -> ! {
    println!("PR3_SMOKE_V1 HARNESS_FAIL arch={ARCH} reason={reason}");
    println!("PR3_SMOKE_V1 SHUTDOWN arch={ARCH}");
    std::process::exit(1)
}

pub fn run() -> ! {
    println!("PR3_SMOKE_V1 HARNESS_START arch={ARCH}");

    let mut file = match File::create(USER_PATH) {
        Ok(file) => file,
        Err(_) => harness_fail("create_user_elf"),
    };
    if file.write_all(USER_IMAGE).is_err() {
        harness_fail("write_user_elf");
    }
    drop(file);

    let exit_code = match crate::uspace::run_user_program_in_timeout("/", &[USER_PATH], 10) {
        Ok(code) => code,
        Err(_) => harness_fail("load_or_run_user_elf"),
    };
    if exit_code == 137 {
        harness_fail("guest_timeout");
    }
    if exit_code != 0 {
        harness_fail("guest_nonzero_exit");
    }
    if crate::uspace::live_user_task_count_for_diagnostics() != 0 {
        crate::uspace::cleanup_user_processes();
        harness_fail("live_user_tasks_after_exit");
    }

    println!("PR3_SMOKE_V1 HARNESS_PASS arch={ARCH} status=0");
    println!("PR3_SMOKE_V1 SHUTDOWN arch={ARCH}");
    std::process::exit(0)
}
