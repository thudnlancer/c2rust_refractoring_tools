#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
#[no_mangle]
pub static mut fatal_exit_hook: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub unsafe extern "C" fn fatal_exit() -> ! {
    if fatal_exit_hook.is_some() {
        fatal_exit_hook.expect("non-null function pointer")();
    }
    error(
        2 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"Error is not recoverable: exiting now\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"memory exhausted\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fatal_exit();
}
