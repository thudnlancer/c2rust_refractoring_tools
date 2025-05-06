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
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn eaccess(__name: *const i8, __type: i32) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn concatenated_filename(
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
static mut suffixes: [*const i8; 1] = [b"\0" as *const u8 as *const i8];
#[no_mangle]
pub unsafe extern "C" fn find_in_given_path(
    mut progname: *const i8,
    mut path: *const i8,
    mut directory: *const i8,
    mut optimize_for_exec: bool,
) -> *const i8 {
    let mut has_slash: bool = 0 as i32 != 0;
    let mut p: *const i8 = 0 as *const i8;
    p = progname;
    while *p as i32 != '\0' as i32 {
        if *p as i32 == '/' as i32 {
            has_slash = 1 as i32 != 0;
            break;
        } else {
            p = p.offset(1);
            p;
        }
    }
    if has_slash {
        if optimize_for_exec {
            return progname
        } else {
            let mut failure_errno: i32 = 0;
            let mut i: size_t = 0;
            let mut directory_as_prefix: *const i8 = if !directory.is_null()
                && !(*progname.offset(0 as i32 as isize) as i32 == '/' as i32)
            {
                directory
            } else {
                b"\0" as *const u8 as *const i8
            };
            failure_errno = 2 as i32;
            i = 0 as i32 as size_t;
            while i
                < (::core::mem::size_of::<[*const i8; 1]>() as u64)
                    .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
            {
                let mut suffix: *const i8 = suffixes[i as usize];
                let mut progpathname: *mut i8 = concatenated_filename(
                    directory_as_prefix,
                    progname,
                    suffix,
                );
                if progpathname.is_null() {
                    return 0 as *const i8;
                }
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
                    if stat(progpathname, &mut statbuf) >= 0 as i32 {
                        if !(statbuf.st_mode & 0o170000 as i32 as u32
                            == 0o40000 as i32 as u32)
                        {
                            if strcmp(progpathname, progname) == 0 as i32 {
                                free(progpathname as *mut libc::c_void);
                                return progname;
                            } else {
                                return progpathname
                            }
                        }
                        *__errno_location() = 13 as i32;
                    }
                }
                if *__errno_location() != 2 as i32 {
                    failure_errno = *__errno_location();
                }
                free(progpathname as *mut libc::c_void);
                i = i.wrapping_add(1);
                i;
            }
            *__errno_location() = failure_errno;
            return 0 as *const i8;
        }
    }
    if path.is_null() {
        path = b"\0" as *const u8 as *const i8;
    }
    let mut path_copy: *mut i8 = strdup(path);
    if path_copy.is_null() {
        return 0 as *const i8;
    }
    let mut failure_errno_0: i32 = 0;
    let mut path_rest: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    failure_errno_0 = 2 as i32;
    path_rest = path_copy;
    's_188: loop {
        let mut dir: *const i8 = 0 as *const i8;
        let mut last: bool = false;
        let mut dir_as_prefix_to_free: *mut i8 = 0 as *mut i8;
        let mut dir_as_prefix: *const i8 = 0 as *const i8;
        let mut i_0: size_t = 0;
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
        if !directory.is_null() && !(*dir.offset(0 as i32 as isize) as i32 == '/' as i32)
        {
            dir_as_prefix_to_free = concatenated_filename(
                directory,
                dir,
                0 as *const i8,
            );
            if dir_as_prefix_to_free.is_null() {
                failure_errno_0 = *__errno_location();
                break;
            } else {
                dir_as_prefix = dir_as_prefix_to_free;
            }
        } else {
            dir_as_prefix_to_free = 0 as *mut i8;
            dir_as_prefix = dir;
        }
        i_0 = 0 as i32 as size_t;
        while i_0
            < (::core::mem::size_of::<[*const i8; 1]>() as u64)
                .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
        {
            let mut suffix_0: *const i8 = suffixes[i_0 as usize];
            let mut progpathname_0: *mut i8 = concatenated_filename(
                dir_as_prefix,
                progname,
                suffix_0,
            );
            if progpathname_0.is_null() {
                failure_errno_0 = *__errno_location();
                free(dir_as_prefix_to_free as *mut libc::c_void);
                break 's_188;
            } else {
                if eaccess(progpathname_0, 1 as i32) == 0 as i32 {
                    let mut statbuf_0: stat = stat {
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
                    if stat(progpathname_0, &mut statbuf_0) >= 0 as i32 {
                        if !(statbuf_0.st_mode & 0o170000 as i32 as u32
                            == 0o40000 as i32 as u32)
                        {
                            if strcmp(progpathname_0, progname) == 0 as i32 {
                                free(progpathname_0 as *mut libc::c_void);
                                progpathname_0 = malloc(
                                    (2 as i32 as u64)
                                        .wrapping_add(strlen(progname))
                                        .wrapping_add(1 as i32 as u64),
                                ) as *mut i8;
                                if progpathname_0.is_null() {
                                    failure_errno_0 = *__errno_location();
                                    free(dir_as_prefix_to_free as *mut libc::c_void);
                                    break 's_188;
                                } else {
                                    *progpathname_0.offset(0 as i32 as isize) = '.' as i32
                                        as i8;
                                    *progpathname_0.offset(1 as i32 as isize) = '/' as i32
                                        as i8;
                                    memcpy(
                                        progpathname_0.offset(2 as i32 as isize)
                                            as *mut libc::c_void,
                                        progname as *const libc::c_void,
                                        (strlen(progname)).wrapping_add(1 as i32 as u64),
                                    );
                                }
                            }
                            free(dir_as_prefix_to_free as *mut libc::c_void);
                            free(path_copy as *mut libc::c_void);
                            return progpathname_0;
                        } else {
                            *__errno_location() = 13 as i32;
                        }
                    }
                }
                if *__errno_location() != 2 as i32 {
                    failure_errno_0 = *__errno_location();
                }
                free(progpathname_0 as *mut libc::c_void);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        }
        free(dir_as_prefix_to_free as *mut libc::c_void);
        if last {
            break;
        }
        path_rest = cp.offset(1 as i32 as isize);
    }
    free(path_copy as *mut libc::c_void);
    *__errno_location() = failure_errno_0;
    return 0 as *const i8;
}