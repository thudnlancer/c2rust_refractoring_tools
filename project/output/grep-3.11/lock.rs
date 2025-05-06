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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> i32;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> i32;
    fn pthread_mutexattr_settype(__attr: *mut pthread_mutexattr_t, __kind: i32) -> i32;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> i32;
    fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> i32;
    fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> i32;
    fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: i32,
    ) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: libc::c_schar,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
    pub __flags: u32,
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
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
pub type pthread_once_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [i8; 56],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [i8; 8],
    pub __align: i64,
}
pub type C2RustUnnamed = u32;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = u32;
pub const PTHREAD_RWLOCK_DEFAULT_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2RustUnnamed_0 = 0;
pub type gl_recursive_lock_t = pthread_mutex_t;
#[no_mangle]
pub unsafe extern "C" fn glthread_rwlock_init_for_glibc(
    mut lock: *mut pthread_rwlock_t,
) -> i32 {
    let mut attributes: pthread_rwlockattr_t = pthread_rwlockattr_t {
        __size: [0; 8],
    };
    let mut err: i32 = 0;
    err = pthread_rwlockattr_init(&mut attributes);
    if err != 0 as i32 {
        return err;
    }
    err = pthread_rwlockattr_setkind_np(
        &mut attributes,
        PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP as i32,
    );
    if err == 0 as i32 {
        err = pthread_rwlock_init(lock, &mut attributes);
    }
    pthread_rwlockattr_destroy(&mut attributes);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn glthread_recursive_lock_init_multithreaded(
    mut lock: *mut gl_recursive_lock_t,
) -> i32 {
    let mut attributes: pthread_mutexattr_t = pthread_mutexattr_t {
        __size: [0; 4],
    };
    let mut err: i32 = 0;
    err = pthread_mutexattr_init(&mut attributes);
    if err != 0 as i32 {
        return err;
    }
    err = pthread_mutexattr_settype(&mut attributes, PTHREAD_MUTEX_RECURSIVE as i32);
    if err != 0 as i32 {
        pthread_mutexattr_destroy(&mut attributes);
        return err;
    }
    err = pthread_mutex_init(lock, &mut attributes);
    if err != 0 as i32 {
        pthread_mutexattr_destroy(&mut attributes);
        return err;
    }
    err = pthread_mutexattr_destroy(&mut attributes);
    if err != 0 as i32 {
        return err;
    }
    return 0 as i32;
}
static mut fresh_once: pthread_once_t = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn glthread_once_singlethreaded(
    mut once_control: *mut pthread_once_t,
) -> i32 {
    let mut firstbyte: *mut i8 = once_control as *mut i8;
    if *firstbyte as i32 == *(&fresh_once as *const pthread_once_t as *const i8) as i32 {
        *firstbyte = !(*(&fresh_once as *const pthread_once_t as *const i8) as i32)
            as i8;
        return 1 as i32;
    } else {
        return 0 as i32
    };
}