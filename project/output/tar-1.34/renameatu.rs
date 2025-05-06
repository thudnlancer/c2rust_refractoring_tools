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
    fn renameat(__oldfd: i32, __old: *const i8, __newfd: i32, __new: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn __fxstatat(
        __ver: i32,
        __fildes: i32,
        __filename: *const i8,
        __stat_buf: *mut stat,
        __flag: i32,
    ) -> i32;
    fn syscall(__sysno: i64, _: ...) -> i64;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
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
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: i32,
    mut __filename: *const i8,
    mut __statbuf: *mut stat,
    mut __flag: i32,
) -> i32 {
    return __fxstatat(1 as i32, __fd, __filename, __statbuf, __flag);
}
unsafe extern "C" fn errno_fail(mut e: i32) -> i32 {
    *__errno_location() = e;
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn lstatat(
    mut fd: i32,
    mut name: *const i8,
    mut st: *mut stat,
) -> i32 {
    return fstatat(fd, name, st, 0x100 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn renameatu(
    mut fd1: i32,
    mut src: *const i8,
    mut fd2: i32,
    mut dst: *const i8,
    mut flags: u32,
) -> i32 {
    let mut ret_val: i32 = -(1 as i32);
    let mut err: i32 = 22 as i32;
    ret_val = syscall(316 as i32 as i64, fd1, src, fd2, dst, flags) as i32;
    err = *__errno_location();
    if !(ret_val < 0 as i32
        && (err == 22 as i32 || err == 38 as i32 || err == 95 as i32))
    {
        return ret_val;
    }
    let mut src_len: size_t = 0;
    let mut dst_len: size_t = 0;
    let mut src_temp: *mut i8 = src as *mut i8;
    let mut dst_temp: *mut i8 = dst as *mut i8;
    let mut src_slash: bool = false;
    let mut dst_slash: bool = false;
    let mut rename_errno: i32 = 20 as i32;
    let mut src_st: stat = stat {
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
    let mut dst_st: stat = stat {
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
    let mut dst_found_nonexistent: bool = 0 as i32 != 0;
    if flags & !((1 as i32) << 0 as i32) as u32 != 0 {
        return errno_fail(95 as i32);
    }
    if flags & ((1 as i32) << 0 as i32) as u32 != 0 as i32 as u32 {
        if lstatat(fd2, dst, &mut dst_st) == 0 as i32 || *__errno_location() == 75 as i32
        {
            return errno_fail(17 as i32);
        }
        if *__errno_location() != 2 as i32 {
            return -(1 as i32);
        }
        dst_found_nonexistent = 1 as i32 != 0;
    }
    src_len = strlen(src);
    dst_len = strlen(dst);
    if src_len == 0 || dst_len == 0 {
        return renameat(fd1, src, fd2, dst);
    }
    src_slash = *src.offset(src_len.wrapping_sub(1 as i32 as u64) as isize) as i32
        == '/' as i32;
    dst_slash = *dst.offset(dst_len.wrapping_sub(1 as i32 as u64) as isize) as i32
        == '/' as i32;
    if !src_slash && !dst_slash {
        return renameat(fd1, src, fd2, dst);
    }
    if lstatat(fd1, src, &mut src_st) != 0 {
        return -(1 as i32);
    }
    if dst_found_nonexistent {
        if !(src_st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) {
            return errno_fail(2 as i32);
        }
    } else if lstatat(fd2, dst, &mut dst_st) != 0 {
        if *__errno_location() != 2 as i32
            || !(src_st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
        {
            return -(1 as i32);
        }
    } else if !(dst_st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) {
        return errno_fail(20 as i32)
    } else if !(src_st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) {
        return errno_fail(21 as i32)
    }
    ret_val = renameat(fd1, src_temp, fd2, dst_temp);
    rename_errno = *__errno_location();
    if src_temp != src as *mut i8 {
        rpl_free(src_temp as *mut libc::c_void);
    }
    if dst_temp != dst as *mut i8 {
        rpl_free(dst_temp as *mut libc::c_void);
    }
    *__errno_location() = rename_errno;
    return ret_val;
}