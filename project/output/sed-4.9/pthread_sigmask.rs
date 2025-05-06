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
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn __errno_location() -> *mut i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[no_mangle]
pub unsafe extern "C" fn pthread_sigmask(
    mut how: i32,
    mut new_mask: *const sigset_t,
    mut old_mask: *mut sigset_t,
) -> i32 {
    let mut ret: i32 = sigprocmask(how, new_mask, old_mask);
    return if ret < 0 as i32 { *__errno_location() } else { 0 as i32 };
}