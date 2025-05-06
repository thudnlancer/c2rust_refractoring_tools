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
    fn creat(__file: *const i8, __mode: mode_t) -> i32;
    fn fd_safer(_: i32) -> i32;
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
#[no_mangle]
pub unsafe extern "C" fn creat_safer(mut file: *const i8, mut mode: mode_t) -> i32 {
    return fd_safer(creat(file, mode));
}