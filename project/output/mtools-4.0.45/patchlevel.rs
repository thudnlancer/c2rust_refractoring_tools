#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub static mut mversion: *const i8 = b"4.0.45\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut mdate: *const i8 = b"September 28th, 2024\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut mformat_banner: *const i8 = b"MTOO4045\0" as *const u8 as *const i8;