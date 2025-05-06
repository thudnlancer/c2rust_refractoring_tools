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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn mkdir(__path: *const i8, __mode: __mode_t) -> i32;
    fn getuid() -> __uid_t;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn stat_error(_: *const i8);
    static mut newdir_umask: mode_t;
    fn delay_set_stat(file_name: *const i8, st: *mut stat, invert_permissions: mode_t);
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn make_path(
    mut argpath: *const i8,
    mut owner: uid_t,
    mut group: gid_t,
    mut verbose_fmt_string: *const i8,
) -> i32 {
    let mut dirpath: *mut i8 = 0 as *mut i8;
    let mut stats: stat = stat {
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
    let mut retval: i32 = 0 as i32;
    let mut tmpmode: mode_t = 0;
    let mut invert_permissions: mode_t = 0;
    let mut we_are_root: i32 = (getuid() == 0 as i32 as u32) as i32;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (strlen(argpath)).wrapping_add(1 as i32 as u64) as usize,
    );
    dirpath = fresh0.as_mut_ptr() as *mut i8;
    strcpy(dirpath, argpath);
    if stat(dirpath, &mut stats) != 0 {
        tmpmode = (0o100 as i32 | 0o100 as i32 >> 3 as i32
            | 0o100 as i32 >> 3 as i32 >> 3 as i32
            | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                    | 0o400 as i32 >> 3 as i32 >> 3 as i32))) as u32 & !newdir_umask;
        invert_permissions = if we_are_root != 0 {
            0 as i32 as u32
        } else {
            (0o200 as i32 | 0o100 as i32) as u32 & !tmpmode
        };
        let mut slash: *mut i8 = dirpath;
        while *slash as i32 == '/' as i32 {
            slash = slash.offset(1);
            slash;
        }
        loop {
            slash = strchr(slash, '/' as i32);
            if slash.is_null() {
                break;
            }
            *slash = '\0' as i32 as i8;
            if stat(dirpath, &mut stats) != 0 {
                if mkdir(dirpath, tmpmode ^ invert_permissions) != 0 {
                    error(
                        0 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"cannot make directory `%s'\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        dirpath,
                    );
                    return 1 as i32;
                } else {
                    if !verbose_fmt_string.is_null() {
                        error(0 as i32, 0 as i32, verbose_fmt_string, dirpath);
                    }
                    if stat(dirpath, &mut stats) != 0 {
                        stat_error(dirpath);
                    } else {
                        if owner != -(1 as i32) as u32 {
                            stats.st_uid = owner;
                        }
                        if group != -(1 as i32) as u32 {
                            stats.st_gid = group;
                        }
                        delay_set_stat(dirpath, &mut stats, invert_permissions);
                    }
                }
            } else if !(stats.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
            {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"`%s' exists but is not a directory\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    dirpath,
                );
                return 1 as i32;
            }
            let fresh1 = slash;
            slash = slash.offset(1);
            *fresh1 = '/' as i32 as i8;
            while *slash as i32 == '/' as i32 {
                slash = slash.offset(1);
                slash;
            }
        }
        if mkdir(dirpath, tmpmode ^ invert_permissions) != 0 {
            if *__errno_location() != 17 as i32 || stat(dirpath, &mut stats) != 0 as i32
                || !(stats.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
            {
                error(
                    0 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"cannot make directory `%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    dirpath,
                );
                return 1 as i32;
            }
        } else if stat(dirpath, &mut stats) != 0 {
            stat_error(dirpath);
        } else {
            if owner != -(1 as i32) as u32 {
                stats.st_uid = owner;
            }
            if group != -(1 as i32) as u32 {
                stats.st_gid = group;
            }
            delay_set_stat(dirpath, &mut stats, invert_permissions);
        }
        if !verbose_fmt_string.is_null() {
            error(0 as i32, 0 as i32, verbose_fmt_string, dirpath);
        }
    } else if !(stats.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"`%s' exists but is not a directory\0" as *const u8 as *const i8,
                5 as i32,
            ),
            dirpath,
        );
        return 1 as i32;
    }
    return retval;
}