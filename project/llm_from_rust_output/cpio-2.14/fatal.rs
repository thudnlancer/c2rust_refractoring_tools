use std::process;

#[no_mangle]
pub extern "C" fn fatal_exit() {
    process::exit(2);
}