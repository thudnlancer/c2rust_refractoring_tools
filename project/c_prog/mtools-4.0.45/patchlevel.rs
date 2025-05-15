use ::libc;
#[no_mangle]
pub static mut mversion: *const libc::c_char = b"4.0.45\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut mdate: *const libc::c_char = b"September 28th, 2024\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut mformat_banner: *const libc::c_char = b"MTOO4045\0" as *const u8
    as *const libc::c_char;
