use std::process;

static EXIT_FAILURE: i32 = 1;

pub fn openat_save_fail(errnum: i32) -> ! {
    eprintln!("unable to record current working directory");
    process::exit(EXIT_FAILURE);
}

pub fn openat_restore_fail(errnum: i32) -> ! {
    eprintln!("failed to return to initial working directory");
    process::exit(EXIT_FAILURE);
}