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
extern "C" {
    fn __fxstatat(
        __ver: i32,
        __fildes: i32,
        __filename: *const i8,
        __stat_buf: *mut stat,
        __flag: i32,
    ) -> i32;
}
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
pub unsafe extern "C" fn lstatat(
    mut fd: i32,
    mut name: *const i8,
    mut st: *mut stat,
) -> i32 {
    return fstatat(fd, name, st, 0x100 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn statat(
    mut fd: i32,
    mut name: *const i8,
    mut st: *mut stat,
) -> i32 {
    return fstatat(fd, name, st, 0 as i32);
}
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: i32,
    mut __filename: *const i8,
    mut __statbuf: *mut stat,
    mut __flag: i32,
) -> i32 {
    return __fxstatat(1 as i32, __fd, __filename, __statbuf, __flag);
}