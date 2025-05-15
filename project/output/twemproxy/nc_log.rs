use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn abort() -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn _safe_vsnprintf(
        to: *mut i8,
        size: size_t,
        format: *const i8,
        ap: ::core::ffi::VaList,
    ) -> i32;
    fn _safe_snprintf(to: *mut i8, n: size_t, fmt: *const i8, _: ...) -> i32;
    fn nc_stacktrace_fd(fd: i32);
    fn _scnprintf(buf: *mut i8, size: size_t, fmt: *const i8, _: ...) -> i32;
    fn _vscnprintf(
        buf: *mut i8,
        size: size_t,
        fmt: *const i8,
        args: ::core::ffi::VaList,
    ) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type va_list = __builtin_va_list;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logger {
    pub name: *const i8,
    pub level: i32,
    pub fd: i32,
    pub nerror: i32,
}
static mut logger: logger = logger {
    name: 0 as *const i8,
    level: 0,
    fd: 0,
    nerror: 0,
};
#[no_mangle]
pub unsafe extern "C" fn log_init(mut level: i32, mut name: *const i8) -> i32 {
    let mut l: *mut logger = &mut logger;
    (*l).level = if 0 as i32 > (if level < 11 as i32 { level } else { 11 as i32 }) {
        0 as i32
    } else if level < 11 as i32 {
        level
    } else {
        11 as i32
    };
    (*l).name = name;
    if name.is_null() || strlen(name) == 0 {
        (*l).fd = 2 as i32;
    } else {
        (*l).fd = open(name, 0o1 as i32 | 0o2000 as i32 | 0o100 as i32, 0o644 as i32);
        if (*l).fd < 0 as i32 {
            _log_stderr(
                b"opening log file '%s' failed: %s\0" as *const u8 as *const i8,
                name,
                strerror(*__errno_location()),
            );
            return -(1 as i32);
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn log_deinit() {
    let mut l: *mut logger = &mut logger;
    if (*l).fd < 0 as i32 || (*l).fd == 2 as i32 {
        return;
    }
    close((*l).fd);
}
#[no_mangle]
pub unsafe extern "C" fn log_reopen() {
    let mut l: *mut logger = &mut logger;
    if (*l).fd != 2 as i32 {
        close((*l).fd);
        (*l).fd = open(
            (*l).name,
            0o1 as i32 | 0o2000 as i32 | 0o100 as i32,
            0o644 as i32,
        );
        if (*l).fd < 0 as i32 {
            _log_stderr_safe(
                b"reopening log file '%s' failed, ignored: %s\0" as *const u8
                    as *const i8,
                (*l).name,
                strerror(*__errno_location()),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_level_up() {
    let mut l: *mut logger = &mut logger;
    if (*l).level < 11 as i32 {
        (*l).level += 1;
        (*l).level;
        _log_safe(b"up log level to %d\0" as *const u8 as *const i8, (*l).level);
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_level_down() {
    let mut l: *mut logger = &mut logger;
    if (*l).level > 0 as i32 {
        (*l).level -= 1;
        (*l).level;
        _log_safe(b"down log level to %d\0" as *const u8 as *const i8, (*l).level);
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_level_set(mut level: i32) {
    let mut l: *mut logger = &mut logger;
    (*l).level = if 0 as i32 > (if level < 11 as i32 { level } else { 11 as i32 }) {
        0 as i32
    } else if level < 11 as i32 {
        level
    } else {
        11 as i32
    };
    _log(
        b"nc_log.c\0" as *const u8 as *const i8,
        105 as i32,
        0 as i32,
        b"set log level to %d\0" as *const u8 as *const i8,
        (*l).level,
    );
}
#[no_mangle]
pub unsafe extern "C" fn log_stacktrace() {
    let mut l: *mut logger = &mut logger;
    if (*l).fd < 0 as i32 {
        return;
    }
    nc_stacktrace_fd((*l).fd);
}
#[no_mangle]
pub unsafe extern "C" fn log_loggable(mut level: i32) -> i32 {
    let mut l: *mut logger = &mut logger;
    if level > (*l).level {
        return 0 as i32;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _log(
    mut file: *const i8,
    mut line: i32,
    mut panic: i32,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut l: *mut logger = &mut logger;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut errno_save: i32 = 0;
    let mut buf: [i8; 256] = [0; 256];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if (*l).fd < 0 as i32 {
        return;
    }
    errno_save = *__errno_location();
    len = 0 as i32;
    size = 256 as i32;
    gettimeofday(&mut tv, 0 as *mut timezone);
    let fresh0 = len;
    len = len + 1;
    buf[fresh0 as usize] = '[' as i32 as i8;
    len
        += strftime(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"%Y-%m-%d %H:%M:%S.\0" as *const u8 as *const i8,
            localtime(&mut tv.tv_sec),
        ) as i32;
    len
        += _scnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"%03ld\0" as *const u8 as *const i8,
            tv.tv_usec / 1000 as i32 as i64,
        );
    len
        += _scnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"] %s:%d \0" as *const u8 as *const i8,
            file,
            line,
        );
    args_0 = args.clone();
    len
        += _vscnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            fmt,
            args_0.as_va_list(),
        );
    let fresh1 = len;
    len = len + 1;
    buf[fresh1 as usize] = '\n' as i32 as i8;
    n = write((*l).fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as i32 as i64 {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
    if panic != 0 {
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn _log_stderr(mut fmt: *const i8, mut args: ...) {
    let mut l: *mut logger = &mut logger;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut errno_save: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    errno_save = *__errno_location();
    len = 0 as i32;
    size = 4 as i32 * 256 as i32;
    args_0 = args.clone();
    len += _vscnprintf(buf.as_mut_ptr(), size as size_t, fmt, args_0.as_va_list());
    let fresh2 = len;
    len = len + 1;
    buf[fresh2 as usize] = '\n' as i32 as i8;
    n = write(2 as i32, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as i32 as i64 {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
}
#[no_mangle]
pub unsafe extern "C" fn _log_hexdump(
    mut file: *const i8,
    mut line: i32,
    mut data: *const i8,
    mut datalen: i32,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut l: *mut logger = &mut logger;
    let mut buf: [i8; 2048] = [0; 2048];
    let mut i: i32 = 0;
    let mut off: i32 = 0;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut errno_save: i32 = 0;
    let mut n: ssize_t = 0;
    if (*l).fd < 0 as i32 {
        return;
    }
    errno_save = *__errno_location();
    off = 0 as i32;
    len = 0 as i32;
    size = 8 as i32 * 256 as i32;
    while datalen != 0 as i32 && len < size - 1 as i32 {
        let mut save: *const i8 = 0 as *const i8;
        let mut str: *const i8 = 0 as *const i8;
        let mut c: u8 = 0;
        let mut savelen: i32 = 0;
        len
            += _scnprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as size_t,
                b"%08x  \0" as *const u8 as *const i8,
                off,
            );
        save = data;
        savelen = datalen;
        i = 0 as i32;
        while datalen != 0 as i32 && i < 16 as i32 {
            c = *data as u8;
            str = if i == 7 as i32 {
                b"  \0" as *const u8 as *const i8
            } else {
                b" \0" as *const u8 as *const i8
            };
            len
                += _scnprintf(
                    buf.as_mut_ptr().offset(len as isize),
                    (size - len) as size_t,
                    b"%02x%s\0" as *const u8 as *const i8,
                    c as i32,
                    str,
                );
            data = data.offset(1);
            data;
            datalen -= 1;
            datalen;
            i += 1;
            i;
        }
        while i < 16 as i32 {
            str = if i == 7 as i32 {
                b"  \0" as *const u8 as *const i8
            } else {
                b" \0" as *const u8 as *const i8
            };
            len
                += _scnprintf(
                    buf.as_mut_ptr().offset(len as isize),
                    (size - len) as size_t,
                    b"  %s\0" as *const u8 as *const i8,
                    str,
                );
            i += 1;
            i;
        }
        data = save;
        datalen = savelen;
        len
            += _scnprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as size_t,
                b"  |\0" as *const u8 as *const i8,
            );
        i = 0 as i32;
        while datalen != 0 as i32 && i < 16 as i32 {
            c = (if *(*__ctype_b_loc()).offset(*data as i32 as isize) as i32
                & C2RustUnnamed::_ISprint as i32 as libc::c_ushort as i32 != 0
            {
                *data as i32
            } else {
                '.' as i32
            }) as u8;
            len
                += _scnprintf(
                    buf.as_mut_ptr().offset(len as isize),
                    (size - len) as size_t,
                    b"%c\0" as *const u8 as *const i8,
                    c as i32,
                );
            data = data.offset(1);
            data;
            datalen -= 1;
            datalen;
            i += 1;
            i;
        }
        len
            += _scnprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as size_t,
                b"|\n\0" as *const u8 as *const i8,
            );
        off += 16 as i32;
    }
    n = write((*l).fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as i32 as i64 {
        (*l).nerror += 1;
        (*l).nerror;
    }
    if len >= size - 1 as i32 {
        n = write(
            (*l).fd,
            b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
            1 as i32 as size_t,
        );
        if n < 0 as i32 as i64 {
            (*l).nerror += 1;
            (*l).nerror;
        }
    }
    *__errno_location() = errno_save;
}
#[no_mangle]
pub unsafe extern "C" fn _log_safe(mut fmt: *const i8, mut args: ...) {
    let mut l: *mut logger = &mut logger;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut errno_save: i32 = 0;
    let mut buf: [i8; 256] = [0; 256];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    if (*l).fd < 0 as i32 {
        return;
    }
    errno_save = *__errno_location();
    len = 0 as i32;
    size = 256 as i32;
    len
        += _safe_snprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"[.......................] \0" as *const u8 as *const i8,
        );
    args_0 = args.clone();
    len
        += _safe_vsnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            fmt,
            args_0.as_va_list(),
        );
    let fresh3 = len;
    len = len + 1;
    buf[fresh3 as usize] = '\n' as i32 as i8;
    n = write((*l).fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as i32 as i64 {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
}
#[no_mangle]
pub unsafe extern "C" fn _log_stderr_safe(mut fmt: *const i8, mut args: ...) {
    let mut l: *mut logger = &mut logger;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut errno_save: i32 = 0;
    let mut buf: [i8; 256] = [0; 256];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    errno_save = *__errno_location();
    len = 0 as i32;
    size = 256 as i32;
    len
        += _safe_snprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"[.......................] \0" as *const u8 as *const i8,
        );
    args_0 = args.clone();
    len
        += _safe_vsnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            fmt,
            args_0.as_va_list(),
        );
    let fresh4 = len;
    len = len + 1;
    buf[fresh4 as usize] = '\n' as i32 as i8;
    n = write(2 as i32, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as i32 as i64 {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
}