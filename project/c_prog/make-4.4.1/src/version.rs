use ::libc;
#[no_mangle]
pub static mut version_string: *const libc::c_char = b"4.4.1\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut make_host: *const libc::c_char = b"x86_64-pc-linux-gnu\0" as *const u8
    as *const libc::c_char;
