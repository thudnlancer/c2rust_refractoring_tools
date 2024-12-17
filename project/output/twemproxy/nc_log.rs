#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
extern "C" {
    fn abort() -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
    fn _safe_vsnprintf(
        to: *mut libc::c_char,
        size: size_t,
        format: *const libc::c_char,
        ap: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn _safe_snprintf(
        to: *mut libc::c_char,
        n: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn nc_stacktrace_fd(fd: libc::c_int);
    fn _scnprintf(
        buf: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn _vscnprintf(
        buf: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logger {
    pub name: *const libc::c_char,
    pub level: libc::c_int,
    pub fd: libc::c_int,
    pub nerror: libc::c_int,
}
static mut logger: logger = logger {
    name: 0 as *const libc::c_char,
    level: 0,
    fd: 0,
    nerror: 0,
};
#[no_mangle]
pub unsafe extern "C" fn log_init(
    mut level: libc::c_int,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut l: *mut logger = &mut logger;
    (*l)
        .level = if 0 as libc::c_int
        > (if level < 11 as libc::c_int { level } else { 11 as libc::c_int })
    {
        0 as libc::c_int
    } else if level < 11 as libc::c_int {
        level
    } else {
        11 as libc::c_int
    };
    (*l).name = name;
    if name.is_null() || strlen(name) == 0 {
        (*l).fd = 2 as libc::c_int;
    } else {
        (*l)
            .fd = open(
            name,
            0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int,
            0o644 as libc::c_int,
        );
        if (*l).fd < 0 as libc::c_int {
            _log_stderr(
                b"opening log file '%s' failed: %s\0" as *const u8
                    as *const libc::c_char,
                name,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn log_deinit() {
    let mut l: *mut logger = &mut logger;
    if (*l).fd < 0 as libc::c_int || (*l).fd == 2 as libc::c_int {
        return;
    }
    close((*l).fd);
}
#[no_mangle]
pub unsafe extern "C" fn log_reopen() {
    let mut l: *mut logger = &mut logger;
    if (*l).fd != 2 as libc::c_int {
        close((*l).fd);
        (*l)
            .fd = open(
            (*l).name,
            0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int,
            0o644 as libc::c_int,
        );
        if (*l).fd < 0 as libc::c_int {
            _log_stderr_safe(
                b"reopening log file '%s' failed, ignored: %s\0" as *const u8
                    as *const libc::c_char,
                (*l).name,
                strerror(*__errno_location()),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_level_up() {
    let mut l: *mut logger = &mut logger;
    if (*l).level < 11 as libc::c_int {
        (*l).level += 1;
        (*l).level;
        _log_safe(
            b"up log level to %d\0" as *const u8 as *const libc::c_char,
            (*l).level,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_level_down() {
    let mut l: *mut logger = &mut logger;
    if (*l).level > 0 as libc::c_int {
        (*l).level -= 1;
        (*l).level;
        _log_safe(
            b"down log level to %d\0" as *const u8 as *const libc::c_char,
            (*l).level,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_level_set(mut level: libc::c_int) {
    let mut l: *mut logger = &mut logger;
    (*l)
        .level = if 0 as libc::c_int
        > (if level < 11 as libc::c_int { level } else { 11 as libc::c_int })
    {
        0 as libc::c_int
    } else if level < 11 as libc::c_int {
        level
    } else {
        11 as libc::c_int
    };
    _log(
        b"nc_log.c\0" as *const u8 as *const libc::c_char,
        105 as libc::c_int,
        0 as libc::c_int,
        b"set log level to %d\0" as *const u8 as *const libc::c_char,
        (*l).level,
    );
}
#[no_mangle]
pub unsafe extern "C" fn log_stacktrace() {
    let mut l: *mut logger = &mut logger;
    if (*l).fd < 0 as libc::c_int {
        return;
    }
    nc_stacktrace_fd((*l).fd);
}
#[no_mangle]
pub unsafe extern "C" fn log_loggable(mut level: libc::c_int) -> libc::c_int {
    let mut l: *mut logger = &mut logger;
    if level > (*l).level {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _log(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut panic: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut l: *mut logger = &mut logger;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut errno_save: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if (*l).fd < 0 as libc::c_int {
        return;
    }
    errno_save = *__errno_location();
    len = 0 as libc::c_int;
    size = 256 as libc::c_int;
    gettimeofday(&mut tv, 0 as *mut timezone);
    let fresh0 = len;
    len = len + 1;
    buf[fresh0 as usize] = '[' as i32 as libc::c_char;
    len
        += strftime(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"%Y-%m-%d %H:%M:%S.\0" as *const u8 as *const libc::c_char,
            localtime(&mut tv.tv_sec),
        ) as libc::c_int;
    len
        += _scnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"%03ld\0" as *const u8 as *const libc::c_char,
            tv.tv_usec / 1000 as libc::c_int as libc::c_long,
        );
    len
        += _scnprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"] %s:%d \0" as *const u8 as *const libc::c_char,
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
    buf[fresh1 as usize] = '\n' as i32 as libc::c_char;
    n = write((*l).fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as libc::c_int as libc::c_long {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
    if panic != 0 {
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn _log_stderr(mut fmt: *const libc::c_char, mut args: ...) {
    let mut l: *mut logger = &mut logger;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut errno_save: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    errno_save = *__errno_location();
    len = 0 as libc::c_int;
    size = 4 as libc::c_int * 256 as libc::c_int;
    args_0 = args.clone();
    len += _vscnprintf(buf.as_mut_ptr(), size as size_t, fmt, args_0.as_va_list());
    let fresh2 = len;
    len = len + 1;
    buf[fresh2 as usize] = '\n' as i32 as libc::c_char;
    n = write(2 as libc::c_int, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as libc::c_int as libc::c_long {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
}
#[no_mangle]
pub unsafe extern "C" fn _log_hexdump(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut data: *const libc::c_char,
    mut datalen: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut l: *mut logger = &mut logger;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut errno_save: libc::c_int = 0;
    let mut n: ssize_t = 0;
    if (*l).fd < 0 as libc::c_int {
        return;
    }
    errno_save = *__errno_location();
    off = 0 as libc::c_int;
    len = 0 as libc::c_int;
    size = 8 as libc::c_int * 256 as libc::c_int;
    while datalen != 0 as libc::c_int && len < size - 1 as libc::c_int {
        let mut save: *const libc::c_char = 0 as *const libc::c_char;
        let mut str: *const libc::c_char = 0 as *const libc::c_char;
        let mut c: libc::c_uchar = 0;
        let mut savelen: libc::c_int = 0;
        len
            += _scnprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as size_t,
                b"%08x  \0" as *const u8 as *const libc::c_char,
                off,
            );
        save = data;
        savelen = datalen;
        i = 0 as libc::c_int;
        while datalen != 0 as libc::c_int && i < 16 as libc::c_int {
            c = *data as libc::c_uchar;
            str = if i == 7 as libc::c_int {
                b"  \0" as *const u8 as *const libc::c_char
            } else {
                b" \0" as *const u8 as *const libc::c_char
            };
            len
                += _scnprintf(
                    buf.as_mut_ptr().offset(len as isize),
                    (size - len) as size_t,
                    b"%02x%s\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                    str,
                );
            data = data.offset(1);
            data;
            datalen -= 1;
            datalen;
            i += 1;
            i;
        }
        while i < 16 as libc::c_int {
            str = if i == 7 as libc::c_int {
                b"  \0" as *const u8 as *const libc::c_char
            } else {
                b" \0" as *const u8 as *const libc::c_char
            };
            len
                += _scnprintf(
                    buf.as_mut_ptr().offset(len as isize),
                    (size - len) as size_t,
                    b"  %s\0" as *const u8 as *const libc::c_char,
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
                b"  |\0" as *const u8 as *const libc::c_char,
            );
        i = 0 as libc::c_int;
        while datalen != 0 as libc::c_int && i < 16 as libc::c_int {
            c = (if *(*__ctype_b_loc()).offset(*data as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                *data as libc::c_int
            } else {
                '.' as i32
            }) as libc::c_uchar;
            len
                += _scnprintf(
                    buf.as_mut_ptr().offset(len as isize),
                    (size - len) as size_t,
                    b"%c\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
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
                b"|\n\0" as *const u8 as *const libc::c_char,
            );
        off += 16 as libc::c_int;
    }
    n = write((*l).fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as libc::c_int as libc::c_long {
        (*l).nerror += 1;
        (*l).nerror;
    }
    if len >= size - 1 as libc::c_int {
        n = write(
            (*l).fd,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        if n < 0 as libc::c_int as libc::c_long {
            (*l).nerror += 1;
            (*l).nerror;
        }
    }
    *__errno_location() = errno_save;
}
#[no_mangle]
pub unsafe extern "C" fn _log_safe(mut fmt: *const libc::c_char, mut args: ...) {
    let mut l: *mut logger = &mut logger;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut errno_save: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    if (*l).fd < 0 as libc::c_int {
        return;
    }
    errno_save = *__errno_location();
    len = 0 as libc::c_int;
    size = 256 as libc::c_int;
    len
        += _safe_snprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"[.......................] \0" as *const u8 as *const libc::c_char,
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
    buf[fresh3 as usize] = '\n' as i32 as libc::c_char;
    n = write((*l).fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as libc::c_int as libc::c_long {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
}
#[no_mangle]
pub unsafe extern "C" fn _log_stderr_safe(mut fmt: *const libc::c_char, mut args: ...) {
    let mut l: *mut logger = &mut logger;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut errno_save: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: ssize_t = 0;
    errno_save = *__errno_location();
    len = 0 as libc::c_int;
    size = 256 as libc::c_int;
    len
        += _safe_snprintf(
            buf.as_mut_ptr().offset(len as isize),
            (size - len) as size_t,
            b"[.......................] \0" as *const u8 as *const libc::c_char,
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
    buf[fresh4 as usize] = '\n' as i32 as libc::c_char;
    n = write(2 as libc::c_int, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    if n < 0 as libc::c_int as libc::c_long {
        (*l).nerror += 1;
        (*l).nerror;
    }
    *__errno_location() = errno_save;
}
