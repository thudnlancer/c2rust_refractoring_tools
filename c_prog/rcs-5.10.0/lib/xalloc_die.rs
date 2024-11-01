#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    error(
        exit_failure,
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"memory exhausted\0" as *const u8 as *const libc::c_char,
    );
    abort();
}
