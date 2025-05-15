use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn set_binary_mode(fd: c_int, mode: c_int) -> c_int {
    __gl_setmode(fd, mode)
}

#[no_mangle]
pub extern "C" fn __gl_setmode(_fd: c_int, _mode: c_int) -> c_int {
    0
}