#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
unsafe extern "C" fn ftypelet(mut bits: i64) -> i8 {
    if bits & 0o170000 as i32 as i64 == 0o60000 as i32 as i64 {
        return 'b' as i32 as i8;
    }
    if bits & 0o170000 as i32 as i64 == 0o20000 as i32 as i64 {
        return 'c' as i32 as i8;
    }
    if bits & 0o170000 as i32 as i64 == 0o40000 as i32 as i64 {
        return 'd' as i32 as i8;
    }
    if bits & 0o170000 as i32 as i64 == 0o100000 as i32 as i64 {
        return '-' as i32 as i8;
    }
    if bits & 0o170000 as i32 as i64 == 0o10000 as i32 as i64 {
        return 'p' as i32 as i8;
    }
    if bits & 0o170000 as i32 as i64 == 0o120000 as i32 as i64 {
        return 'l' as i32 as i8;
    }
    if bits & 0o170000 as i32 as i64 == 0o140000 as i32 as i64 {
        return 's' as i32 as i8;
    }
    return '?' as i32 as i8;
}
unsafe extern "C" fn rwx(mut bits: libc::c_ushort, mut chars: *mut i8) {
    *chars.offset(0 as i32 as isize) = (if bits as i32 & 0o400 as i32 != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as i8;
    *chars.offset(1 as i32 as isize) = (if bits as i32 & 0o200 as i32 != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as i8;
    *chars.offset(2 as i32 as isize) = (if bits as i32 & 0o100 as i32 != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as i8;
}
unsafe extern "C" fn setst(mut bits: libc::c_ushort, mut chars: *mut i8) {
    if bits as i32 & 0o4000 as i32 != 0 {
        if *chars.offset(3 as i32 as isize) as i32 != 'x' as i32 {
            *chars.offset(3 as i32 as isize) = 'S' as i32 as i8;
        } else {
            *chars.offset(3 as i32 as isize) = 's' as i32 as i8;
        }
    }
    if bits as i32 & 0o2000 as i32 != 0 {
        if *chars.offset(6 as i32 as isize) as i32 != 'x' as i32 {
            *chars.offset(6 as i32 as isize) = 'S' as i32 as i8;
        } else {
            *chars.offset(6 as i32 as isize) = 's' as i32 as i8;
        }
    }
    if bits as i32 & 0o1000 as i32 != 0 {
        if *chars.offset(9 as i32 as isize) as i32 != 'x' as i32 {
            *chars.offset(9 as i32 as isize) = 'T' as i32 as i8;
        } else {
            *chars.offset(9 as i32 as isize) = 't' as i32 as i8;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mode_string(mut mode: u32, mut str: *mut i8) {
    *str.offset(0 as i32 as isize) = ftypelet(mode as i64);
    rwx(
        ((mode & 0o700 as i32 as u32) << 0 as i32) as libc::c_ushort,
        &mut *str.offset(1 as i32 as isize),
    );
    rwx(
        ((mode & 0o70 as i32 as u32) << 3 as i32) as libc::c_ushort,
        &mut *str.offset(4 as i32 as isize),
    );
    rwx(
        ((mode & 0o7 as i32 as u32) << 6 as i32) as libc::c_ushort,
        &mut *str.offset(7 as i32 as isize),
    );
    setst(mode as libc::c_ushort, str);
}
#[no_mangle]
pub unsafe extern "C" fn filemodestring(mut statp: *mut stat, mut str: *mut i8) {
    mode_string((*statp).st_mode, str);
}