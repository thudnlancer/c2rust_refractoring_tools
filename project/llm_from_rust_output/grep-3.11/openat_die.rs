use std::process;
use gettextrs::gettext;

fn openat_save_fail(errnum: i32) {
    eprintln!(
        "{}",
        gettext("unable to record current working directory")
    );
    process::exit(errnum);
}

fn openat_restore_fail(errnum: i32) {
    eprintln!(
        "{}",
        gettext("failed to return to initial working directory")
    );
    process::exit(errnum);
}