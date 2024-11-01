#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
    fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> libc::c_int;
    fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> libc::c_int;
    fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: libc::c_int,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_once_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_RWLOCK_DEFAULT_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2RustUnnamed_0 = 0;
pub type gl_recursive_lock_t = pthread_mutex_t;
#[no_mangle]
pub unsafe extern "C" fn glthread_rwlock_init_for_glibc(
    mut lock: *mut pthread_rwlock_t,
) -> libc::c_int {
    let mut attributes: pthread_rwlockattr_t = pthread_rwlockattr_t {
        __size: [0; 8],
    };
    let mut err: libc::c_int = 0;
    err = pthread_rwlockattr_init(&mut attributes);
    if err != 0 as libc::c_int {
        return err;
    }
    err = pthread_rwlockattr_setkind_np(
        &mut attributes,
        PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP as libc::c_int,
    );
    if err == 0 as libc::c_int {
        err = pthread_rwlock_init(lock, &mut attributes);
    }
    pthread_rwlockattr_destroy(&mut attributes);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn glthread_recursive_lock_init_multithreaded(
    mut lock: *mut gl_recursive_lock_t,
) -> libc::c_int {
    let mut attributes: pthread_mutexattr_t = pthread_mutexattr_t {
        __size: [0; 4],
    };
    let mut err: libc::c_int = 0;
    err = pthread_mutexattr_init(&mut attributes);
    if err != 0 as libc::c_int {
        return err;
    }
    err = pthread_mutexattr_settype(
        &mut attributes,
        PTHREAD_MUTEX_RECURSIVE as libc::c_int,
    );
    if err != 0 as libc::c_int {
        pthread_mutexattr_destroy(&mut attributes);
        return err;
    }
    err = pthread_mutex_init(lock, &mut attributes);
    if err != 0 as libc::c_int {
        pthread_mutexattr_destroy(&mut attributes);
        return err;
    }
    err = pthread_mutexattr_destroy(&mut attributes);
    if err != 0 as libc::c_int {
        return err;
    }
    return 0 as libc::c_int;
}
static mut fresh_once: pthread_once_t = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn glthread_once_singlethreaded(
    mut once_control: *mut pthread_once_t,
) -> libc::c_int {
    let mut firstbyte: *mut libc::c_char = once_control as *mut libc::c_char;
    if *firstbyte as libc::c_int
        == *(&fresh_once as *const pthread_once_t as *const libc::c_char) as libc::c_int
    {
        *firstbyte = !(*(&fresh_once as *const pthread_once_t as *const libc::c_char)
            as libc::c_int) as libc::c_char;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
