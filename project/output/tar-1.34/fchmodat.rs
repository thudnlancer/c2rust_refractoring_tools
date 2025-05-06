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
    fn __fxstatat(
        __ver: i32,
        __fildes: i32,
        __filename: *const i8,
        __stat_buf: *mut stat,
        __flag: i32,
    ) -> i32;
    fn chmod(__file: *const i8, __mode: __mode_t) -> i32;
    fn fchmodat(__fd: i32, __file: *const i8, __mode: __mode_t, __flag: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn openat(__fd: i32, __file: *const i8, __oflag: i32, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn close(__fd: i32) -> i32;
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
pub type mode_t = __mode_t;
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
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: i32,
    mut __filename: *const i8,
    mut __statbuf: *mut stat,
    mut __flag: i32,
) -> i32 {
    return __fxstatat(1 as i32, __fd, __filename, __statbuf, __flag);
}
unsafe extern "C" fn orig_fchmodat(
    mut dir: i32,
    mut file: *const i8,
    mut mode: mode_t,
    mut flags: i32,
) -> i32 {
    return fchmodat(dir, file, mode, flags);
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fchmodat(
    mut dir: i32,
    mut file: *const i8,
    mut mode: mode_t,
    mut flags: i32,
) -> i32 {
    if flags == 0x100 as i32 {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut fd: i32 = openat(
            dir,
            file,
            0o10000000 as i32 | 0o400000 as i32 | 0o2000000 as i32,
        );
        if fd < 0 as i32 {
            return fd;
        }
        if fstatat(fd, b"\0" as *const u8 as *const i8, &mut st, 0x1000 as i32)
            != 0 as i32
        {
            let mut stat_errno: i32 = *__errno_location();
            close(fd);
            *__errno_location() = stat_errno;
            return -(1 as i32);
        }
        if st.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
            close(fd);
            *__errno_location() = 95 as i32;
            return -(1 as i32);
        }
        static mut fmt: [i8; 17] = unsafe {
            *::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"/proc/self/fd/%d\0")
        };
        let mut buf: [i8; 26] = [0; 26];
        sprintf(buf.as_mut_ptr(), fmt.as_ptr(), fd);
        let mut chmod_result: i32 = chmod(buf.as_mut_ptr(), mode);
        let mut chmod_errno: i32 = *__errno_location();
        close(fd);
        if chmod_result == 0 as i32 {
            return chmod_result;
        }
        if chmod_errno != 2 as i32 {
            *__errno_location() = chmod_errno;
            return chmod_result;
        }
        flags = 0 as i32;
    }
    return orig_fchmodat(dir, file, mode, flags);
}