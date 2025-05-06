#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut program_invocation_name: *mut i8;
    static mut program_invocation_short_name: *mut i8;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
}
#[no_mangle]
pub static mut program_name: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn set_program_name(mut argv0: *const i8) {
    let mut slash: *const i8 = 0 as *const i8;
    let mut base: *const i8 = 0 as *const i8;
    slash = strrchr(argv0, '/' as i32);
    base = if !slash.is_null() { slash.offset(1 as i32 as isize) } else { argv0 };
    if base.offset_from(argv0) as i64 >= 7 as i32 as i64
        && strncmp(
            base.offset(-(7 as i32 as isize)),
            b"/.libs/\0" as *const u8 as *const i8,
            7 as i32 as u64,
        ) == 0 as i32
    {
        argv0 = base;
        if strncmp(base, b"lt-\0" as *const u8 as *const i8, 3 as i32 as u64) == 0 as i32
        {
            argv0 = base.offset(3 as i32 as isize);
            program_invocation_short_name = argv0 as *mut i8;
        }
    }
    program_name = argv0;
    program_invocation_name = argv0 as *mut i8;
}