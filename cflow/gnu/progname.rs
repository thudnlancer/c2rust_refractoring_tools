#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    static mut program_invocation_name: *mut libc::c_char;
    static mut program_invocation_short_name: *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
#[no_mangle]
pub static mut program_name: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn set_program_name(mut argv0: *const libc::c_char) {
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    slash = strrchr(argv0, '/' as i32);
    base = if !slash.is_null() {
        slash.offset(1 as libc::c_int as isize)
    } else {
        argv0
    };
    if base.offset_from(argv0) as libc::c_long >= 7 as libc::c_int as libc::c_long
        && strncmp(
            base.offset(-(7 as libc::c_int as isize)),
            b"/.libs/\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        argv0 = base;
        if strncmp(
            base,
            b"lt-\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            argv0 = base.offset(3 as libc::c_int as isize);
            program_invocation_short_name = argv0 as *mut libc::c_char;
        }
    }
    program_name = argv0;
    program_invocation_name = argv0 as *mut libc::c_char;
}
