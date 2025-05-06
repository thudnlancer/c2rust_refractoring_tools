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
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn eaccess(__name: *const i8, __type: i32) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn xalloc_die();
    fn xstrdup(str: *const i8) -> *mut i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xconcatenated_filename(
        directory: *const i8,
        filename: *const i8,
        suffix: *const i8,
    ) -> *mut i8;
}
pub type size_t = u64;
pub type __mode_t = u32;
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
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = i64;
pub type __blkcnt_t = i64;
pub type __blksize_t = i64;
pub type __off_t = i64;
pub type __dev_t = u64;
pub type __gid_t = u32;
pub type __uid_t = u32;
pub type __nlink_t = u64;
pub type __ino_t = u64;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    if (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
        9223372036854775807 as i64 as u64
    } else {
        (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
    })
        .wrapping_div(s) < n
    {
        xalloc_die();
    }
    return xmalloc(n.wrapping_mul(s));
}
#[no_mangle]
pub unsafe extern "C" fn find_in_path(mut progname: *const i8) -> *const i8 {
    let mut path: *mut i8 = 0 as *mut i8;
    let mut path_rest: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    if !(strchr(progname, '/' as i32)).is_null() {
        return progname;
    }
    path = getenv(b"PATH\0" as *const u8 as *const i8);
    if path.is_null() || *path as i32 == '\0' as i32 {
        return progname;
    }
    path = xstrdup(path);
    path_rest = path;
    loop {
        let mut dir: *const i8 = 0 as *const i8;
        let mut last: bool = false;
        let mut progpathname: *mut i8 = 0 as *mut i8;
        dir = path_rest;
        cp = path_rest;
        while *cp as i32 != '\0' as i32 && *cp as i32 != ':' as i32 {
            cp = cp.offset(1);
            cp;
        }
        last = *cp as i32 == '\0' as i32;
        *cp = '\0' as i32 as i8;
        if dir == cp {
            dir = b".\0" as *const u8 as *const i8;
        }
        progpathname = xconcatenated_filename(dir, progname, 0 as *const i8);
        if eaccess(progpathname, 1 as i32) == 0 as i32 {
            let mut statbuf: stat = stat {
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
            if stat(progpathname, &mut statbuf) >= 0 as i32
                && !(statbuf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
            {
                if strcmp(progpathname, progname) == 0 as i32 {
                    free(progpathname as *mut libc::c_void);
                    progpathname = (if ::core::mem::size_of::<i8>() as u64
                        == 1 as i32 as u64
                    {
                        xmalloc(
                            (2 as i32 as u64)
                                .wrapping_add(strlen(progname))
                                .wrapping_add(1 as i32 as u64),
                        )
                    } else {
                        xnmalloc(
                            (2 as i32 as u64)
                                .wrapping_add(strlen(progname))
                                .wrapping_add(1 as i32 as u64),
                            ::core::mem::size_of::<i8>() as u64,
                        )
                    }) as *mut i8;
                    *progpathname.offset(0 as i32 as isize) = '.' as i32 as i8;
                    *progpathname.offset(1 as i32 as isize) = '/' as i32 as i8;
                    memcpy(
                        progpathname.offset(2 as i32 as isize) as *mut libc::c_void,
                        progname as *const libc::c_void,
                        (strlen(progname)).wrapping_add(1 as i32 as u64),
                    );
                }
                free(path as *mut libc::c_void);
                return progpathname;
            }
        }
        free(progpathname as *mut libc::c_void);
        if last {
            break;
        }
        path_rest = cp.offset(1 as i32 as isize);
    }
    free(path as *mut libc::c_void);
    return progname;
}