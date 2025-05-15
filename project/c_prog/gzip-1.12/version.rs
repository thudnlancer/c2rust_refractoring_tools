use ::libc;
#[no_mangle]
pub static mut Version: *const libc::c_char = b"1.12\0" as *const u8
    as *const libc::c_char;
