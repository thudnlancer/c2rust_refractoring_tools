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
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    error(
        exit_failure,
        0 as i32,
        b"%s\0" as *const u8 as *const i8,
        b"memory exhausted\0" as *const u8 as *const i8,
    );
    abort();
}