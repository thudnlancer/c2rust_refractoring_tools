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
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut i32,
    ) -> i32;
}
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
pub type gl_thread_t = pthread_t;
#[no_mangle]
pub unsafe extern "C" fn gl_thread_create(
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> gl_thread_t {
    let mut thread: gl_thread_t = 0;
    let mut ret: i32 = 0;
    ret = if (Some(
        pthread_mutexattr_gettype
            as unsafe extern "C" fn(*const pthread_mutexattr_t, *mut i32) -> i32,
    ))
        .is_some() || 0 as i32 != 0
    {
        pthread_create(&mut thread, 0 as *const pthread_attr_t, func, arg)
    } else {
        38 as i32
    };
    if ret != 0 as i32 {
        abort();
    }
    return thread;
}