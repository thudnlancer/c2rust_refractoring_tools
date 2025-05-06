#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stat_time_normalize(mut result: i32, mut st: *mut stat) -> i32 {
    return result;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_birthtime(mut st: *const stat) -> timespec {
    let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    t.tv_sec = -(1 as i32) as __time_t;
    t.tv_nsec = -(1 as i32) as __syscall_slong_t;
    return t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_ctime(mut st: *const stat) -> timespec {
    return (*st).st_ctim;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_atime_ns(mut st: *const stat) -> i64 {
    return (*st).st_atim.tv_nsec;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_ctime_ns(mut st: *const stat) -> i64 {
    return (*st).st_ctim.tv_nsec;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_mtime_ns(mut st: *const stat) -> i64 {
    return (*st).st_mtim.tv_nsec;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_birthtime_ns(mut st: *const stat) -> i64 {
    return 0 as i32 as i64;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}