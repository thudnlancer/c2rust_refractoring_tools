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
}
pub type gl_thread_t = i32;
#[no_mangle]
pub unsafe extern "C" fn gl_thread_create(
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> gl_thread_t {
    let mut thread: gl_thread_t = 0;
    let mut ret: i32 = 0;
    ret = 38 as i32;
    if ret != 0 as i32 {
        abort();
    }
    return thread;
}