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
pub type mode_t = __mode_t;
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
unsafe extern "C" fn ftypelet(mut bits: mode_t) -> i8 {
    if bits & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
        return '-' as i32 as i8;
    }
    if bits & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
        return 'd' as i32 as i8;
    }
    if bits & 0o170000 as i32 as u32 == 0o60000 as i32 as u32 {
        return 'b' as i32 as i8;
    }
    if bits & 0o170000 as i32 as u32 == 0o20000 as i32 as u32 {
        return 'c' as i32 as i8;
    }
    if bits & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
        return 'l' as i32 as i8;
    }
    if bits & 0o170000 as i32 as u32 == 0o10000 as i32 as u32 {
        return 'p' as i32 as i8;
    }
    if bits & 0o170000 as i32 as u32 == 0o140000 as i32 as u32 {
        return 's' as i32 as i8;
    }
    if 0 as i32 != 0 || 0 as i32 != 0 || 0 as i32 != 0 {
        return 'm' as i32 as i8;
    }
    return '?' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn strmode(mut mode: mode_t, mut str: *mut i8) {
    *str.offset(0 as i32 as isize) = ftypelet(mode);
    *str.offset(1 as i32 as isize) = (if mode & 0o400 as i32 as u32 != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(2 as i32 as isize) = (if mode & 0o200 as i32 as u32 != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(3 as i32 as isize) = (if mode & 0o4000 as i32 as u32 != 0 {
        if mode & 0o100 as i32 as u32 != 0 { 's' as i32 } else { 'S' as i32 }
    } else if mode & 0o100 as i32 as u32 != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(4 as i32 as isize) = (if mode & (0o400 as i32 >> 3 as i32) as u32 != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(5 as i32 as isize) = (if mode & (0o200 as i32 >> 3 as i32) as u32 != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(6 as i32 as isize) = (if mode & 0o2000 as i32 as u32 != 0 {
        if mode & (0o100 as i32 >> 3 as i32) as u32 != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & (0o100 as i32 >> 3 as i32) as u32 != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(7 as i32 as isize) = (if mode
        & (0o400 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0
    {
        'r' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(8 as i32 as isize) = (if mode
        & (0o200 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0
    {
        'w' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(9 as i32 as isize) = (if mode & 0o1000 as i32 as u32 != 0 {
        if mode & (0o100 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0 {
            't' as i32
        } else {
            'T' as i32
        }
    } else if mode & (0o100 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as i8;
    *str.offset(10 as i32 as isize) = ' ' as i32 as i8;
    *str.offset(11 as i32 as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn filemodestring(mut statp: *const stat, mut str: *mut i8) {
    strmode((*statp).st_mode, str);
    if ((*statp).st_mode).wrapping_sub((*statp).st_mode) != 0 {
        *str.offset(0 as i32 as isize) = 'F' as i32 as i8;
    } else if ((*statp).st_mode).wrapping_sub((*statp).st_mode) != 0 {
        *str.offset(0 as i32 as isize) = 'Q' as i32 as i8;
    } else if ((*statp).st_mode).wrapping_sub((*statp).st_mode) != 0 {
        *str.offset(0 as i32 as isize) = 'S' as i32 as i8;
    }
}