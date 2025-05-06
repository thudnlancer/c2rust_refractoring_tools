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
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
#[no_mangle]
pub static mut fatal_exit_hook: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub unsafe extern "C" fn fatal_exit() -> ! {
    if fatal_exit_hook.is_some() {
        fatal_exit_hook.expect("non-null function pointer")();
    }
    error(
        2 as i32,
        0 as i32,
        dcgettext(
            0 as *const i8,
            b"Error is not recoverable: exiting now\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    error(
        0 as i32,
        0 as i32,
        b"%s\0" as *const u8 as *const i8,
        dcgettext(
            0 as *const i8,
            b"memory exhausted\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fatal_exit();
}