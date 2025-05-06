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
    fn abort() -> !;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    static mut exit_failure: i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn openat_save_fail(mut errnum: i32) {
    error(
        exit_failure,
        errnum,
        dcgettext(
            b"wget-gnulib\0" as *const u8 as *const i8,
            b"unable to record current working directory\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn openat_restore_fail(mut errnum: i32) {
    error(
        exit_failure,
        errnum,
        dcgettext(
            b"wget-gnulib\0" as *const u8 as *const i8,
            b"failed to return to initial working directory\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    abort();
}