#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn isatty(__fd: i32) -> i32;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn exit(_: i32) -> !;
    fn rpl_strtol(string: *const i8, endptr: *mut *mut i8, base: i32) -> i64;
    fn __uflow(_: *mut _IO_FILE) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __strtoul_internal(
        __nptr: *const i8,
        __endptr: *mut *mut i8,
        __base: i32,
        __group: i32,
    ) -> u64;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn argp_parse(
        __argp: *const argp,
        _: i32,
        _: *mut *mut i8,
        __flags: u32,
        __arg_index: *mut i32,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_version_setup(name: *const i8, authors: *const *const i8);
    fn full_write(fd: i32, buf: *const libc::c_void, count: size_t) -> size_t;
    fn set_program_name(argv0: *const i8);
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
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
pub type __uintmax_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __daddr_t = i32;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type error_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtget {
    pub mt_type: i64,
    pub mt_resid: i64,
    pub mt_dsreg: i64,
    pub mt_gstat: i64,
    pub mt_erreg: i64,
    pub mt_fileno: __daddr_t,
    pub mt_blkno: __daddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const i8,
    pub key: i32,
    pub arg: *const i8,
    pub flags: i32,
    pub doc: *const i8,
    pub group: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const i8,
    pub doc: *const i8,
    pub children: *const argp_child,
    pub help_filter: Option<
        unsafe extern "C" fn(i32, *const i8, *mut libc::c_void) -> *mut i8,
    >,
    pub argp_domain: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: i32,
    pub header: *const i8,
    pub group: i32,
}
pub type argp_parser_t = Option<
    unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: i32,
    pub argv: *mut *mut i8,
    pub next: i32,
    pub flags: u32,
    pub arg_num: u32,
    pub quoted: i32,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut i8,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmt_kw {
    pub name: *const i8,
    pub len: size_t,
    pub value: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    DEBUG_FILE_OPTION = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::DEBUG_FILE_OPTION => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            256 => C2RustUnnamed::DEBUG_FILE_OPTION,
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x10 as i32 != 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as i32);
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isblank(mut c: i32) -> bool {
    return c == ' ' as i32 || c == '\t' as i32;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[no_mangle]
pub static mut dbglev: i32 = 0;
#[no_mangle]
pub static mut dbgout: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn trimnl(mut str: *mut i8) {
    if !str.is_null() {
        let mut len: size_t = strlen(str);
        if len > 1 as i32 as u64
            && *str.offset(len.wrapping_sub(1 as i32 as u64) as isize) as i32
                == '\n' as i32
        {
            *str.offset(len.wrapping_sub(1 as i32 as u64) as isize) = 0 as i32 as i8;
        }
    }
}
#[no_mangle]
pub static mut input_buf_ptr: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut input_buf_size: size_t = 0 as i32 as size_t;
unsafe extern "C" fn rmt_read() -> *mut i8 {
    let mut rc: ssize_t = getline(&mut input_buf_ptr, &mut input_buf_size, stdin);
    if rc > 0 as i32 as i64 {
        if !dbgout.is_null() && 10 as i32 <= dbglev {
            fprintf(dbgout, b"C: %s\0" as *const u8 as *const i8, input_buf_ptr);
        }
        trimnl(input_buf_ptr);
        return input_buf_ptr;
    }
    if !dbgout.is_null() && 10 as i32 <= dbglev {
        fprintf(
            dbgout,
            b"%s\0" as *const u8 as *const i8,
            b"reached EOF\0" as *const u8 as *const i8,
        );
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn rmt_write(mut fmt: *const i8, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stdout, fmt, ap.as_va_list());
    fflush_unlocked(stdout);
    if !dbgout.is_null() && 10 as i32 <= dbglev {
        let mut aptr: ::core::ffi::VaListImpl;
        aptr = args.clone();
        fprintf(
            dbgout,
            b"%s\0" as *const u8 as *const i8,
            b"S: \0" as *const u8 as *const i8,
        );
        vfprintf(dbgout, fmt, aptr.as_va_list());
    }
}
unsafe extern "C" fn rmt_reply(mut code: uintmax_t) {
    rmt_write(b"A%ju\n\0" as *const u8 as *const i8, code);
}
unsafe extern "C" fn rmt_error_message(mut code: i32, mut msg: *const i8) {
    if !dbgout.is_null() && 10 as i32 <= dbglev {
        fprintf(dbgout, b"S: E%d\n\0" as *const u8 as *const i8, code);
    }
    if !dbgout.is_null() && 10 as i32 <= dbglev {
        fprintf(dbgout, b"S: %s\n\0" as *const u8 as *const i8, msg);
    }
    if !dbgout.is_null() && 1 as i32 <= dbglev {
        fprintf(dbgout, b"error: %s\n\0" as *const u8 as *const i8, msg);
    }
    fprintf(stdout, b"E%d\n%s\n\0" as *const u8 as *const i8, code, msg);
    fflush_unlocked(stdout);
}
unsafe extern "C" fn rmt_error(mut code: i32) {
    rmt_error_message(code, strerror(code));
}
#[no_mangle]
pub static mut record_buffer_ptr: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut record_buffer_size: size_t = 0;
unsafe extern "C" fn prepare_record_buffer(mut size: size_t) {
    if size > record_buffer_size {
        record_buffer_ptr = xrealloc(record_buffer_ptr as *mut libc::c_void, size)
            as *mut i8;
        record_buffer_size = size;
    }
}
#[no_mangle]
pub static mut device_fd: i32 = -(1 as i32);
unsafe extern "C" fn xlat_kw(
    mut s: *const i8,
    mut pfx: *const i8,
    mut kw: *const rmt_kw,
    mut valp: *mut i32,
    mut endp: *mut *const i8,
) -> i32 {
    let mut slen: size_t = strlen(s);
    if !pfx.is_null() {
        let mut pfxlen: size_t = strlen(pfx);
        if slen > pfxlen
            && memcmp(s as *const libc::c_void, pfx as *const libc::c_void, pfxlen)
                == 0 as i32
        {
            s = s.offset(pfxlen as isize);
            slen = (slen as u64).wrapping_sub(pfxlen) as size_t as size_t;
        }
    }
    while !((*kw).name).is_null() {
        if slen >= (*kw).len
            && memcmp(
                (*kw).name as *const libc::c_void,
                s as *const libc::c_void,
                (*kw).len,
            ) == 0 as i32
            && !(*s.offset((*kw).len as isize) as i32 != 0
                && c_isalnum(*s.offset((*kw).len as isize) as i32) as i32 != 0)
        {
            *valp = (*kw).value;
            *endp = s.offset((*kw).len as isize);
            return 0 as i32;
        }
        kw = kw.offset(1);
        kw;
    }
    return 1 as i32;
}
unsafe extern "C" fn skip_ws(mut s: *const i8) -> *const i8 {
    while *s as i32 != 0 && c_isblank(*s as i32) as i32 != 0 {
        s = s.offset(1);
        s;
    }
    return s;
}
static mut open_flag_kw: [rmt_kw; 14] = [rmt_kw {
    name: 0 as *const i8,
    len: 0,
    value: 0,
}; 14];
unsafe extern "C" fn decode_open_flag(mut mstr: *const i8, mut pmode: *mut i32) -> i32 {
    let mut numeric_mode: i32 = 0 as i32;
    let mut mode: i32 = 0 as i32;
    let mut p: *const i8 = 0 as *const i8;
    mstr = skip_ws(mstr);
    if c_isdigit(*mstr as i32) {
        numeric_mode = rpl_strtol(
            mstr,
            &mut p as *mut *const i8 as *mut *mut i8,
            10 as i32,
        ) as i32;
        mstr = skip_ws(p);
    }
    if *mstr != 0 {
        while !mstr.is_null() {
            let mut v: i32 = 0;
            mstr = skip_ws(mstr);
            if *mstr as i32 == 0 as i32 {
                break;
            }
            if c_isdigit(*mstr as i32) {
                v = rpl_strtol(mstr, &mut p as *mut *const i8 as *mut *mut i8, 10 as i32)
                    as i32;
            } else if xlat_kw(
                mstr,
                b"O_\0" as *const u8 as *const i8,
                open_flag_kw.as_ptr(),
                &mut v,
                &mut p,
            ) != 0
            {
                rmt_error_message(
                    22 as i32,
                    b"invalid open mode\0" as *const u8 as *const i8,
                );
                return 1 as i32;
            }
            mode |= v;
            if *p as i32 != 0 && c_isblank(*p as i32) as i32 != 0 {
                p = skip_ws(p);
            }
            if *p as i32 == 0 as i32 {
                break;
            }
            if *p as i32 == '|' as i32 {
                mstr = p.offset(1 as i32 as isize);
            } else {
                rmt_error_message(
                    22 as i32,
                    b"invalid open mode\0" as *const u8 as *const i8,
                );
                return 1 as i32;
            }
        }
    } else {
        mode = numeric_mode;
    }
    *pmode = mode;
    return 0 as i32;
}
unsafe extern "C" fn open_device(mut str: *mut i8) {
    let mut device: *mut i8 = xstrdup(str);
    let mut flag_str: *mut i8 = 0 as *mut i8;
    let mut flag: i32 = 0;
    flag_str = rmt_read();
    if flag_str.is_null() {
        if !dbgout.is_null() && 1 as i32 <= dbglev {
            fprintf(
                dbgout,
                b"%s\0" as *const u8 as *const i8,
                b"unexpected EOF\0" as *const u8 as *const i8,
            );
        }
        exit(1 as i32);
    }
    if decode_open_flag(flag_str, &mut flag) == 0 as i32 {
        if device_fd >= 0 as i32 {
            close(device_fd);
        }
        device_fd = open(
            device,
            flag,
            0o200 as i32 | 0o200 as i32 >> 3 as i32
                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                    | 0o400 as i32 >> 3 as i32 >> 3 as i32),
        );
        if device_fd < 0 as i32 {
            rmt_error(*__errno_location());
        } else {
            rmt_reply(0 as i32 as uintmax_t);
        }
    }
    rpl_free(device as *mut libc::c_void);
}
unsafe extern "C" fn close_device() {
    if close(device_fd) < 0 as i32 {
        rmt_error(*__errno_location());
    } else {
        device_fd = -(1 as i32);
        rmt_reply(0 as i32 as uintmax_t);
    };
}
static mut seek_whence_kw: [rmt_kw; 4] = [rmt_kw {
    name: 0 as *const i8,
    len: 0,
    value: 0,
}; 4];
unsafe extern "C" fn lseek_device(mut str: *const i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut whence: i32 = 0;
    let mut off: off_t = 0;
    let mut n: uintmax_t = 0;
    if *str.offset(0 as i32 as isize) as i32 != 0
        && *str.offset(1 as i32 as isize) as i32 == 0 as i32
    {
        match *str.offset(0 as i32 as isize) as i32 {
            48 => {
                whence = 0 as i32;
            }
            49 => {
                whence = 1 as i32;
            }
            50 => {
                whence = 2 as i32;
            }
            _ => {
                rmt_error_message(
                    22 as i32,
                    b"Seek direction out of range\0" as *const u8 as *const i8,
                );
                return;
            }
        }
    } else if xlat_kw(
        str,
        b"SEEK_\0" as *const u8 as *const i8,
        seek_whence_kw.as_ptr(),
        &mut whence,
        &mut p as *mut *mut i8 as *mut *const i8,
    ) != 0
    {
        rmt_error_message(
            22 as i32,
            b"Invalid seek direction\0" as *const u8 as *const i8,
        );
        return;
    }
    str = rmt_read();
    off = strtoumax(str, &mut p, 10 as i32) as off_t;
    n = off as uintmax_t;
    if *p != 0 {
        rmt_error_message(22 as i32, b"Invalid seek offset\0" as *const u8 as *const i8);
        return;
    }
    if n != off as u64 || *__errno_location() == 34 as i32 {
        rmt_error_message(
            22 as i32,
            b"Seek offset out of range\0" as *const u8 as *const i8,
        );
        return;
    }
    off = lseek(device_fd, off, whence);
    if off < 0 as i32 as i64 {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(off as uintmax_t);
    };
}
unsafe extern "C" fn read_device(mut str: *const i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut size: size_t = 0;
    let mut n: uintmax_t = 0;
    let mut status: size_t = 0;
    size = strtoumax(str, &mut p, 10 as i32);
    n = size;
    if *p != 0 {
        rmt_error_message(22 as i32, b"Invalid byte count\0" as *const u8 as *const i8);
        return;
    }
    if n != size || *__errno_location() == 34 as i32 {
        rmt_error_message(
            22 as i32,
            b"Byte count out of range\0" as *const u8 as *const i8,
        );
        return;
    }
    prepare_record_buffer(size);
    status = safe_read(device_fd, record_buffer_ptr as *mut libc::c_void, size);
    if status == -(1 as i32) as size_t {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(status);
        full_write(1 as i32, record_buffer_ptr as *const libc::c_void, status);
    };
}
unsafe extern "C" fn write_device(mut str: *const i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut size: size_t = 0;
    let mut n: uintmax_t = 0;
    let mut status: size_t = 0;
    size = strtoumax(str, &mut p, 10 as i32);
    n = size;
    if *p != 0 {
        rmt_error_message(22 as i32, b"Invalid byte count\0" as *const u8 as *const i8);
        return;
    }
    if n != size || *__errno_location() == 34 as i32 {
        rmt_error_message(
            22 as i32,
            b"Byte count out of range\0" as *const u8 as *const i8,
        );
        return;
    }
    prepare_record_buffer(size);
    if (if 0 != 0 && 0 != 0 && size.wrapping_mul(1 as i32 as size_t) <= 8 as i32 as u64
        && size != 0 as i32 as u64
    {
        ({
            let mut __ptr: *mut i8 = record_buffer_ptr;
            let mut __stream: *mut FILE = stdin;
            let mut __cnt: size_t = 0;
            __cnt = size.wrapping_mul(1 as i32 as size_t);
            while __cnt > 0 as i32 as u64 {
                let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                    >= (*__stream)._IO_read_end) as i32 as i64 != 0
                {
                    __uflow(__stream)
                } else {
                    let fresh0 = (*__stream)._IO_read_ptr;
                    (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr).offset(1);
                    *(fresh0 as *mut u8) as i32
                });
                if __c == -(1 as i32) {
                    break;
                }
                let fresh1 = __ptr;
                __ptr = __ptr.offset(1);
                *fresh1 = __c as i8;
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            size.wrapping_mul(1 as i32 as size_t).wrapping_sub(__cnt).wrapping_div(size)
        })
    } else {
        (if 0 != 0 && size == 0 as i32 as u64
            || 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
        {
            0 as i32 as size_t
        } else {
            fread_unlocked(
                record_buffer_ptr as *mut libc::c_void,
                size,
                1 as i32 as size_t,
                stdin,
            )
        })
    }) != 1 as i32 as u64
    {
        if feof_unlocked(stdin) != 0 {
            rmt_error_message(5 as i32, b"Premature eof\0" as *const u8 as *const i8);
        } else {
            rmt_error(*__errno_location());
        }
        return;
    }
    status = full_write(device_fd, record_buffer_ptr as *const libc::c_void, size);
    if status != size {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(status);
    };
}
unsafe extern "C" fn iocop_device(mut str: *const i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut opcode: i64 = 0;
    let mut count: off_t = 0;
    let mut n: uintmax_t = 0;
    opcode = rpl_strtol(str, &mut p, 10 as i32);
    if *p != 0 {
        rmt_error_message(
            22 as i32,
            b"Invalid operation code\0" as *const u8 as *const i8,
        );
        return;
    }
    str = rmt_read();
    count = strtoumax(str, &mut p, 10 as i32) as off_t;
    n = count as uintmax_t;
    if *p != 0 {
        rmt_error_message(22 as i32, b"Invalid byte count\0" as *const u8 as *const i8);
        return;
    }
    if n != count as u64 || *__errno_location() == 34 as i32 {
        rmt_error_message(
            22 as i32,
            b"Byte count out of range\0" as *const u8 as *const i8,
        );
        return;
    }
    let mut mtop: mtop = mtop { mt_op: 0, mt_count: 0 };
    mtop.mt_count = count as i32;
    if mtop.mt_count as i64 != count {
        rmt_error_message(
            22 as i32,
            b"Byte count out of range\0" as *const u8 as *const i8,
        );
        return;
    }
    mtop.mt_op = opcode as libc::c_short;
    if ioctl(
        device_fd,
        ((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | (('m' as i32) << 0 as i32 + 8 as i32) as u32
            | ((1 as i32) << 0 as i32) as u32) as u64
            | (::core::mem::size_of::<mtop>() as u64) << 0 as i32 + 8 as i32 + 8 as i32,
        &mut mtop as *mut mtop as *mut i8,
    ) < 0 as i32
    {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(0 as i32 as uintmax_t);
    };
}
unsafe extern "C" fn status_device(mut str: *const i8) {
    if *str != 0 {
        rmt_error_message(
            22 as i32,
            b"Unexpected arguments\0" as *const u8 as *const i8,
        );
        return;
    }
    let mut mtget: mtget = mtget {
        mt_type: 0,
        mt_resid: 0,
        mt_dsreg: 0,
        mt_gstat: 0,
        mt_erreg: 0,
        mt_fileno: 0,
        mt_blkno: 0,
    };
    if ioctl(
        device_fd,
        ((2 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | (('m' as i32) << 0 as i32 + 8 as i32) as u32
            | ((2 as i32) << 0 as i32) as u32) as u64
            | (::core::mem::size_of::<mtget>() as u64) << 0 as i32 + 8 as i32 + 8 as i32,
        &mut mtget as *mut mtget as *mut i8,
    ) < 0 as i32
    {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(::core::mem::size_of::<mtget>() as u64);
        full_write(
            1 as i32,
            &mut mtget as *mut mtget as *mut i8 as *const libc::c_void,
            ::core::mem::size_of::<mtget>() as u64,
        );
    };
}
#[no_mangle]
pub static mut argp_program_version: *const i8 = b"rmt (GNU cpio) 2.14\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut argp_program_bug_address: *const i8 = b"<bug-cpio@gnu.org>\0" as *const u8
    as *const i8;
static mut doc: [i8; 66] = unsafe {
    *::core::mem::transmute::<
        &[u8; 66],
        &[i8; 66],
    >(b"Manipulate a tape drive, accepting commands from a remote process\0")
};
static mut options: [argp_option; 3] = [
    {
        let mut init = argp_option {
            name: b"debug\0" as *const u8 as *const i8,
            key: 'd' as i32,
            arg: b"NUMBER\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"set debug level\0" as *const u8 as *const i8,
            group: 0 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"debug-file\0" as *const u8 as *const i8,
            key: C2RustUnnamed::DEBUG_FILE_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"set debug output file name\0" as *const u8 as *const i8,
            group: 0 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0,
            arg: 0 as *const i8,
            flags: 0,
            doc: 0 as *const i8,
            group: 0,
        };
        init
    },
];
unsafe extern "C" fn parse_opt(
    mut key: i32,
    mut arg: *mut i8,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        100 => {
            dbglev = rpl_strtol(arg, 0 as *mut *mut i8, 0 as i32) as i32;
        }
        256 => {
            dbgout = fopen(arg, b"w\0" as *const u8 as *const i8);
            if dbgout.is_null() {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"cannot open %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    arg,
                );
            }
        }
        16777223 => {
            if dbglev != 0 {
                if dbgout.is_null() {
                    dbgout = stderr;
                }
            } else if !dbgout.is_null() {
                dbglev = 1 as i32;
            }
        }
        _ => return 7 as i32,
    }
    return 0 as i32;
}
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
            ),
            args_doc: 0 as *const i8,
            doc: doc.as_ptr(),
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const i8,
        };
        init
    }
};
static mut rmt_authors: [*const i8; 2] = [
    b"Sergey Poznyakoff\0" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    rmt_error(12 as i32);
    exit(1 as i32);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut idx: i32 = 0;
    let mut stop: i32 = 0 as i32;
    set_program_name(*argv.offset(0 as i32 as isize));
    argp_version_setup(b"rmt\0" as *const u8 as *const i8, rmt_authors.as_mut_ptr());
    if isatty(1 as i32) != 0 {
        setlocale(6 as i32, b"\0" as *const u8 as *const i8);
        bindtextdomain(
            b"cpio\0" as *const u8 as *const i8,
            b"/usr/local/share/locale\0" as *const u8 as *const i8,
        );
        textdomain(b"cpio\0" as *const u8 as *const i8);
    }
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as i32 as u32,
        &mut idx,
        0 as *mut libc::c_void,
    ) != 0
    {
        exit(1 as i32);
    }
    if idx != argc {
        if idx != argc - 1 as i32 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"too many arguments\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        dbgout = fopen(*argv.offset(idx as isize), b"w\0" as *const u8 as *const i8);
        if dbgout.is_null() {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"cannot open %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *argv.offset(idx as isize),
            );
        }
        dbglev = 1 as i32;
    }
    while stop == 0
        && {
            buf = rmt_read();
            !buf.is_null()
        }
    {
        match *buf.offset(0 as i32 as isize) as i32 {
            67 => {
                close_device();
                stop = 1 as i32;
            }
            73 => {
                iocop_device(buf.offset(1 as i32 as isize));
            }
            76 => {
                lseek_device(buf.offset(1 as i32 as isize));
            }
            79 => {
                open_device(buf.offset(1 as i32 as isize));
            }
            82 => {
                read_device(buf.offset(1 as i32 as isize));
            }
            83 => {
                status_device(buf.offset(1 as i32 as isize));
            }
            87 => {
                write_device(buf.offset(1 as i32 as isize));
            }
            _ => {
                if !dbgout.is_null() && 1 as i32 <= dbglev {
                    fprintf(
                        dbgout,
                        b"garbage input %s\n\0" as *const u8 as *const i8,
                        buf,
                    );
                }
                rmt_error_message(
                    22 as i32,
                    b"Garbage command\0" as *const u8 as *const i8,
                );
                return 1 as i32;
            }
        }
    }
    if device_fd >= 0 as i32 {
        close_device();
    }
    rpl_free(input_buf_ptr as *mut libc::c_void);
    rpl_free(record_buffer_ptr as *mut libc::c_void);
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    open_flag_kw = [
        {
            let mut init = rmt_kw {
                name: b"APPEND\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o2000 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"CREAT\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o100 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"DSYNC\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o10000 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"EXCL\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o200 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"LARGEFILE\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 10]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"NOCTTY\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o400 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"NONBLOCK\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o4000 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"RDONLY\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"RDWR\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o2 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"RSYNC\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o4010000 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"SYNC\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o4010000 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"TRUNC\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o1000 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"WRONLY\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0o1 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: 0 as *const i8,
                len: 0,
                value: 0,
            };
            init
        },
    ];
    seek_whence_kw = [
        {
            let mut init = rmt_kw {
                name: b"SET\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 0 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"CUR\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 1 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"END\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                value: 2 as i32,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: 0 as *const i8,
                len: 0,
                value: 0,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];