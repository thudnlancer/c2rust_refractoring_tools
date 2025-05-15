use std::process;

static mut EXIT_STATUS: i32 = 0;

pub fn pax_exit() {
    let status = unsafe { EXIT_STATUS };
    process::exit(status);
}