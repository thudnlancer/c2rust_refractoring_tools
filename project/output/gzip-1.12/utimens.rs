use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __errno_location() -> *mut i32;
    fn utimensat(
        __fd: i32,
        __path: *const i8,
        __times: *const timespec,
        __flags: i32,
    ) -> i32;
    fn futimens(__fd: i32, __times: *const timespec) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn utimes(__file: *const i8, __tvp: *const timeval) -> i32;
    fn futimesat(__fd: i32, __file: *const i8, __tvp: *const timeval) -> i32;
    fn gettime(_: *mut timespec);
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type time_t = __time_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    TIMESPEC_HZ = 1000000000,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::TIMESPEC_HZ => 1000000000,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            1000000000 => C2RustUnnamed::TIMESPEC_HZ,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_atime_ns(mut st: *const stat) -> i64 {
    return (*st).st_atim.tv_nsec;
}
#[inline]
unsafe extern "C" fn get_stat_mtime_ns(mut st: *const stat) -> i64 {
    return (*st).st_mtim.tv_nsec;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut utimensat_works_really: i32 = 0;
static mut lutimensat_works_really: i32 = 0;
unsafe extern "C" fn validate_timespec(mut timespec: *mut timespec) -> i32 {
    let mut result: i32 = 0 as i32;
    let mut utime_omit_count: i32 = 0 as i32;
    if (*timespec.offset(0 as i32 as isize)).tv_nsec
        != ((1 as i64) << 30 as i32) - 1 as i64
        && (*timespec.offset(0 as i32 as isize)).tv_nsec
            != ((1 as i64) << 30 as i32) - 2 as i64
        && !(0 as i32 as i64 <= (*timespec.offset(0 as i32 as isize)).tv_nsec
            && (*timespec.offset(0 as i32 as isize)).tv_nsec
                < C2RustUnnamed::TIMESPEC_HZ as i32 as i64)
        || (*timespec.offset(1 as i32 as isize)).tv_nsec
            != ((1 as i64) << 30 as i32) - 1 as i64
            && (*timespec.offset(1 as i32 as isize)).tv_nsec
                != ((1 as i64) << 30 as i32) - 2 as i64
            && !(0 as i32 as i64 <= (*timespec.offset(1 as i32 as isize)).tv_nsec
                && (*timespec.offset(1 as i32 as isize)).tv_nsec
                    < C2RustUnnamed::TIMESPEC_HZ as i32 as i64)
    {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if (*timespec.offset(0 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 1 as i64
        || (*timespec.offset(0 as i32 as isize)).tv_nsec
            == ((1 as i64) << 30 as i32) - 2 as i64
    {
        (*timespec.offset(0 as i32 as isize)).tv_sec = 0 as i32 as __time_t;
        result = 1 as i32;
        if (*timespec.offset(0 as i32 as isize)).tv_nsec
            == ((1 as i64) << 30 as i32) - 2 as i64
        {
            utime_omit_count += 1;
            utime_omit_count;
        }
    }
    if (*timespec.offset(1 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 1 as i64
        || (*timespec.offset(1 as i32 as isize)).tv_nsec
            == ((1 as i64) << 30 as i32) - 2 as i64
    {
        (*timespec.offset(1 as i32 as isize)).tv_sec = 0 as i32 as __time_t;
        result = 1 as i32;
        if (*timespec.offset(1 as i32 as isize)).tv_nsec
            == ((1 as i64) << 30 as i32) - 2 as i64
        {
            utime_omit_count += 1;
            utime_omit_count;
        }
    }
    return result + (utime_omit_count == 1 as i32) as i32;
}
unsafe extern "C" fn update_timespec(
    mut statbuf: *const stat,
    mut ts: *mut *mut timespec,
) -> bool {
    let mut timespec: *mut timespec = *ts;
    if (*timespec.offset(0 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 2 as i64
        && (*timespec.offset(1 as i32 as isize)).tv_nsec
            == ((1 as i64) << 30 as i32) - 2 as i64
    {
        return 1 as i32 != 0;
    }
    if (*timespec.offset(0 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 1 as i64
        && (*timespec.offset(1 as i32 as isize)).tv_nsec
            == ((1 as i64) << 30 as i32) - 1 as i64
    {
        *ts = 0 as *mut timespec;
        return 0 as i32 != 0;
    }
    if (*timespec.offset(0 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 2 as i64
    {
        *timespec.offset(0 as i32 as isize) = get_stat_atime(statbuf);
    } else if (*timespec.offset(0 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 1 as i64
    {
        gettime(&mut *timespec.offset(0 as i32 as isize));
    }
    if (*timespec.offset(1 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 2 as i64
    {
        *timespec.offset(1 as i32 as isize) = get_stat_mtime(statbuf);
    } else if (*timespec.offset(1 as i32 as isize)).tv_nsec
        == ((1 as i64) << 30 as i32) - 1 as i64
    {
        gettime(&mut *timespec.offset(1 as i32 as isize));
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fdutimens(
    mut fd: i32,
    mut file: *const i8,
    mut timespec: *const timespec,
) -> i32 {
    let mut adjusted_timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut ts: *mut timespec = if !timespec.is_null() {
        adjusted_timespec.as_mut_ptr()
    } else {
        0 as *mut timespec
    };
    let mut adjustment_needed: i32 = 0 as i32;
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
    if !ts.is_null() {
        adjusted_timespec[0 as i32 as usize] = *timespec.offset(0 as i32 as isize);
        adjusted_timespec[1 as i32 as usize] = *timespec.offset(1 as i32 as isize);
        adjustment_needed = validate_timespec(ts);
    }
    if adjustment_needed < 0 as i32 {
        return -(1 as i32);
    }
    if fd < 0 as i32 && file.is_null() {
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    if 0 as i32 <= utimensat_works_really {
        let mut result: i32 = 0;
        if adjustment_needed == 2 as i32 {
            if if fd < 0 as i32 { stat(file, &mut st) } else { fstat(fd, &mut st) } != 0
            {
                return -(1 as i32);
            }
            if (*ts.offset(0 as i32 as isize)).tv_nsec
                == ((1 as i64) << 30 as i32) - 2 as i64
            {
                *ts.offset(0 as i32 as isize) = get_stat_atime(&mut st);
            } else if (*ts.offset(1 as i32 as isize)).tv_nsec
                == ((1 as i64) << 30 as i32) - 2 as i64
            {
                *ts.offset(1 as i32 as isize) = get_stat_mtime(&mut st);
            }
            adjustment_needed += 1;
            adjustment_needed;
        }
        if fd < 0 as i32 {
            result = utimensat(-(100 as i32), file, ts as *const timespec, 0 as i32);
            if (0 as i32) < result {
                *__errno_location() = 38 as i32;
            }
            if result == 0 as i32 || *__errno_location() != 38 as i32 {
                utimensat_works_really = 1 as i32;
                return result;
            }
        }
        if 0 as i32 <= fd {
            result = futimens(fd, ts as *const timespec);
            if (0 as i32) < result {
                *__errno_location() = 38 as i32;
            }
            if result == 0 as i32 || *__errno_location() != 38 as i32 {
                utimensat_works_really = 1 as i32;
                return result;
            }
        }
    }
    utimensat_works_really = -(1 as i32);
    lutimensat_works_really = -(1 as i32);
    if adjustment_needed != 0 || 0 as i32 != 0 && fd < 0 as i32 {
        if adjustment_needed != 3 as i32
            && (if fd < 0 as i32 { stat(file, &mut st) } else { fstat(fd, &mut st) })
                != 0
        {
            return -(1 as i32);
        }
        if !ts.is_null() && update_timespec(&mut st, &mut ts) as i32 != 0 {
            return 0 as i32;
        }
    }
    let mut timeval: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    let mut t: *mut timeval = 0 as *mut timeval;
    if !ts.is_null() {
        timeval[0 as i32 as usize].tv_sec = (*ts.offset(0 as i32 as isize)).tv_sec;
        timeval[0 as i32 as usize].tv_usec = (*ts.offset(0 as i32 as isize)).tv_nsec
            / 1000 as i32 as i64;
        timeval[1 as i32 as usize].tv_sec = (*ts.offset(1 as i32 as isize)).tv_sec;
        timeval[1 as i32 as usize].tv_usec = (*ts.offset(1 as i32 as isize)).tv_nsec
            / 1000 as i32 as i64;
        t = timeval.as_mut_ptr();
    } else {
        t = 0 as *mut timeval;
    }
    if fd < 0 as i32 {
        return futimesat(-(100 as i32), file, t as *const timeval)
    } else if futimesat(fd, 0 as *const i8, t as *const timeval) == 0 as i32 {
        if !t.is_null() {
            let mut abig: bool = 500000 as i32 as i64
                <= (*t.offset(0 as i32 as isize)).tv_usec;
            let mut mbig: bool = 500000 as i32 as i64
                <= (*t.offset(1 as i32 as isize)).tv_usec;
            if abig as i32 | mbig as i32 != 0 && fstat(fd, &mut st) == 0 as i32 {
                let mut adiff: time_t = st.st_atim.tv_sec
                    - (*t.offset(0 as i32 as isize)).tv_sec;
                let mut mdiff: time_t = st.st_mtim.tv_sec
                    - (*t.offset(1 as i32 as isize)).tv_sec;
                let mut tt: *mut timeval = 0 as *mut timeval;
                let mut truncated_timeval: [timeval; 2] = [timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                }; 2];
                truncated_timeval[0 as i32 as usize] = *t.offset(0 as i32 as isize);
                truncated_timeval[1 as i32 as usize] = *t.offset(1 as i32 as isize);
                if abig as i32 != 0 && adiff == 1 as i32 as i64
                    && get_stat_atime_ns(&mut st) == 0 as i32 as i64
                {
                    tt = truncated_timeval.as_mut_ptr();
                    (*tt.offset(0 as i32 as isize)).tv_usec = 0 as i32 as __suseconds_t;
                }
                if mbig as i32 != 0 && mdiff == 1 as i32 as i64
                    && get_stat_mtime_ns(&mut st) == 0 as i32 as i64
                {
                    tt = truncated_timeval.as_mut_ptr();
                    (*tt.offset(1 as i32 as isize)).tv_usec = 0 as i32 as __suseconds_t;
                }
                if !tt.is_null() {
                    futimesat(fd, 0 as *const i8, tt as *const timeval);
                }
            }
        }
        return 0 as i32;
    }
    if file.is_null() {
        return -(1 as i32);
    }
    return utimes(file, t as *const timeval);
}
#[no_mangle]
pub unsafe extern "C" fn utimens(
    mut file: *const i8,
    mut timespec: *const timespec,
) -> i32 {
    return fdutimens(-(1 as i32), file, timespec);
}
#[no_mangle]
pub unsafe extern "C" fn lutimens(
    mut file: *const i8,
    mut timespec: *const timespec,
) -> i32 {
    let mut adjusted_timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut ts: *mut timespec = if !timespec.is_null() {
        adjusted_timespec.as_mut_ptr()
    } else {
        0 as *mut timespec
    };
    let mut adjustment_needed: i32 = 0 as i32;
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
    if !ts.is_null() {
        adjusted_timespec[0 as i32 as usize] = *timespec.offset(0 as i32 as isize);
        adjusted_timespec[1 as i32 as usize] = *timespec.offset(1 as i32 as isize);
        adjustment_needed = validate_timespec(ts);
    }
    if adjustment_needed < 0 as i32 {
        return -(1 as i32);
    }
    if 0 as i32 <= lutimensat_works_really {
        let mut result: i32 = 0;
        if adjustment_needed == 2 as i32 {
            if lstat(file, &mut st) != 0 {
                return -(1 as i32);
            }
            if (*ts.offset(0 as i32 as isize)).tv_nsec
                == ((1 as i64) << 30 as i32) - 2 as i64
            {
                *ts.offset(0 as i32 as isize) = get_stat_atime(&mut st);
            } else if (*ts.offset(1 as i32 as isize)).tv_nsec
                == ((1 as i64) << 30 as i32) - 2 as i64
            {
                *ts.offset(1 as i32 as isize) = get_stat_mtime(&mut st);
            }
            adjustment_needed += 1;
            adjustment_needed;
        }
        result = utimensat(-(100 as i32), file, ts as *const timespec, 0x100 as i32);
        if (0 as i32) < result {
            *__errno_location() = 38 as i32;
        }
        if result == 0 as i32 || *__errno_location() != 38 as i32 {
            utimensat_works_really = 1 as i32;
            lutimensat_works_really = 1 as i32;
            return result;
        }
    }
    lutimensat_works_really = -(1 as i32);
    if adjustment_needed != 0 || 0 as i32 != 0 {
        if adjustment_needed != 3 as i32 && lstat(file, &mut st) != 0 {
            return -(1 as i32);
        }
        if !ts.is_null() && update_timespec(&mut st, &mut ts) as i32 != 0 {
            return 0 as i32;
        }
    }
    if !(adjustment_needed != 0 || 0 as i32 != 0) && lstat(file, &mut st) != 0 {
        return -(1 as i32);
    }
    if !(st.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32) {
        return fdutimens(-(1 as i32), file, ts as *const timespec);
    }
    *__errno_location() = 38 as i32;
    return -(1 as i32);
}