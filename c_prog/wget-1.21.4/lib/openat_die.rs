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
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn openat_save_fail(mut errnum: libc::c_int) {
    error(
        exit_failure,
        errnum,
        dcgettext(
            b"wget-gnulib\0" as *const u8 as *const libc::c_char,
            b"unable to record current working directory\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn openat_restore_fail(mut errnum: libc::c_int) {
    error(
        exit_failure,
        errnum,
        dcgettext(
            b"wget-gnulib\0" as *const u8 as *const libc::c_char,
            b"failed to return to initial working directory\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    abort();
}
