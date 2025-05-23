use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type quoting_options;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    static mut optarg: *mut i8;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    static mut optind: i32;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> i32;
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const i8,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const i8;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const i8,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> i32;
    fn rpl_fclose(stream: *mut FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strcasestr(__haystack: *const i8, __needle: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn mbscasestr(haystack: *const i8, needle: *const i8) -> *mut i8;
    fn mbsstr(haystack: *const i8, needle: *const i8) -> *mut i8;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn close(__fd: i32) -> i32;
    fn _exit(_: i32) -> !;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> i32;
    fn setgid(__gid: __gid_t) -> i32;
    fn isatty(__fd: i32) -> i32;
    fn xstrtoumax(
        _: *const i8,
        _: *mut *mut i8,
        _: i32,
        _: *mut uintmax_t,
        _: *const i8,
    ) -> strtol_error;
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
    static mut program_name: *const i8;
    fn set_program_name(argv0: *const i8);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn open_safer(_: *const i8, _: i32, _: ...) -> i32;
    fn human_readable(
        _: uintmax_t,
        _: *mut i8,
        _: i32,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut i8;
    fn last_component(filename: *const i8) -> *mut i8;
    fn close_stdout();
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn get_regex_type(s: *const i8) -> i32;
    fn xstrtol_fatal(_: strtol_error, _: i32, _: i8, _: *const option, _: *const i8);
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn explain_how_to_report_bugs(f: *mut FILE, program_name_0: *const i8) -> i32;
    fn display_findutils_version(official_name: *const i8);
    fn getword(
        fp: *mut FILE,
        filename: *const i8,
        maxvalue: size_t,
        endian_state_flag: *mut GetwordEndianState,
    ) -> i32;
    fn print_quoted(
        fp: *mut FILE,
        qopts: *const quoting_options,
        dest_is_tty: bool,
        format: *const i8,
        s: *const i8,
    ) -> i32;
    fn splitstring(
        s: *const i8,
        separators: *const i8,
        first: bool,
        pos: *mut size_t,
        len: *mut size_t,
    ) -> bool;
}
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
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
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut u8,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
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
pub type uintmax_t = __uintmax_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum strtol_error {
    LONGINT_OK = 0,
    LONGINT_OVERFLOW,
    LONGINT_INVALID_SUFFIX_CHAR,
    LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
    LONGINT_INVALID = 4,
}
impl strtol_error {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            strtol_error::LONGINT_OK => 0,
            strtol_error::LONGINT_OVERFLOW => 1,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR => 2,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => 3,
            strtol_error::LONGINT_INVALID => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> strtol_error {
        match value {
            0 => strtol_error::LONGINT_OK,
            1 => strtol_error::LONGINT_OVERFLOW,
            2 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR,
            3 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
            4 => strtol_error::LONGINT_INVALID,
            _ => panic!("Invalid value for strtol_error: {}", value),
        }
    }
}
impl AddAssign<u32> for strtol_error {
    fn add_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for strtol_error {
    fn sub_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for strtol_error {
    fn mul_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for strtol_error {
    fn div_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for strtol_error {
    fn rem_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for strtol_error {
    type Output = strtol_error;
    fn add(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for strtol_error {
    type Output = strtol_error;
    fn sub(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for strtol_error {
    type Output = strtol_error;
    fn mul(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for strtol_error {
    type Output = strtol_error;
    fn div(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for strtol_error {
    type Output = strtol_error;
    fn rem(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    human_ceiling = 0,
    human_round_to_nearest = 1,
    human_floor = 2,
    human_group_digits = 4,
    human_suppress_point_zero = 8,
    human_autoscale = 16,
    human_base_1024 = 32,
    human_space_before_unit = 64,
    human_SI = 128,
    human_B = 256,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::human_ceiling => 0,
            C2RustUnnamed_0::human_round_to_nearest => 1,
            C2RustUnnamed_0::human_floor => 2,
            C2RustUnnamed_0::human_group_digits => 4,
            C2RustUnnamed_0::human_suppress_point_zero => 8,
            C2RustUnnamed_0::human_autoscale => 16,
            C2RustUnnamed_0::human_base_1024 => 32,
            C2RustUnnamed_0::human_space_before_unit => 64,
            C2RustUnnamed_0::human_SI => 128,
            C2RustUnnamed_0::human_B => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            0 => C2RustUnnamed_0::human_ceiling,
            1 => C2RustUnnamed_0::human_round_to_nearest,
            2 => C2RustUnnamed_0::human_floor,
            4 => C2RustUnnamed_0::human_group_digits,
            8 => C2RustUnnamed_0::human_suppress_point_zero,
            16 => C2RustUnnamed_0::human_autoscale,
            32 => C2RustUnnamed_0::human_base_1024,
            64 => C2RustUnnamed_0::human_space_before_unit,
            128 => C2RustUnnamed_0::human_SI,
            256 => C2RustUnnamed_0::human_B,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum GetwordEndianState {
    GetwordEndianStateInitial = 0,
    GetwordEndianStateNative = 1,
    GetwordEndianStateSwab = 2,
}
impl GetwordEndianState {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            GetwordEndianState::GetwordEndianStateInitial => 0,
            GetwordEndianState::GetwordEndianStateNative => 1,
            GetwordEndianState::GetwordEndianStateSwab => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> GetwordEndianState {
        match value {
            0 => GetwordEndianState::GetwordEndianStateInitial,
            1 => GetwordEndianState::GetwordEndianStateNative,
            2 => GetwordEndianState::GetwordEndianStateSwab,
            _ => panic!("Invalid value for GetwordEndianState: {}", value),
        }
    }
}
impl AddAssign<u32> for GetwordEndianState {
    fn add_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for GetwordEndianState {
    fn sub_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for GetwordEndianState {
    fn mul_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for GetwordEndianState {
    fn div_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for GetwordEndianState {
    fn rem_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn add(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn sub(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn mul(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn div(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn rem(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum visit_result {
    VISIT_CONTINUE = 1,
    VISIT_ACCEPTED = 2,
    VISIT_REJECTED = 4,
    VISIT_ABORT = 8,
}
impl visit_result {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            visit_result::VISIT_CONTINUE => 1,
            visit_result::VISIT_ACCEPTED => 2,
            visit_result::VISIT_REJECTED => 4,
            visit_result::VISIT_ABORT => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> visit_result {
        match value {
            1 => visit_result::VISIT_CONTINUE,
            2 => visit_result::VISIT_ACCEPTED,
            4 => visit_result::VISIT_REJECTED,
            8 => visit_result::VISIT_ABORT,
            _ => panic!("Invalid value for visit_result: {}", value),
        }
    }
}
impl AddAssign<u32> for visit_result {
    fn add_assign(&mut self, rhs: u32) {
        *self = visit_result::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for visit_result {
    fn sub_assign(&mut self, rhs: u32) {
        *self = visit_result::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for visit_result {
    fn mul_assign(&mut self, rhs: u32) {
        *self = visit_result::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for visit_result {
    fn div_assign(&mut self, rhs: u32) {
        *self = visit_result::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for visit_result {
    fn rem_assign(&mut self, rhs: u32) {
        *self = visit_result::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for visit_result {
    type Output = visit_result;
    fn add(self, rhs: u32) -> visit_result {
        visit_result::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for visit_result {
    type Output = visit_result;
    fn sub(self, rhs: u32) -> visit_result {
        visit_result::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for visit_result {
    type Output = visit_result;
    fn mul(self, rhs: u32) -> visit_result {
        visit_result::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for visit_result {
    type Output = visit_result;
    fn div(self, rhs: u32) -> visit_result {
        visit_result::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for visit_result {
    type Output = visit_result;
    fn rem(self, rhs: u32) -> visit_result {
        visit_result::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ExistenceCheckType {
    ACCEPT_EITHER,
    ACCEPT_EXISTING,
    ACCEPT_NON_EXISTING,
}
impl ExistenceCheckType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ExistenceCheckType::ACCEPT_EITHER => 0,
            ExistenceCheckType::ACCEPT_EXISTING => 1,
            ExistenceCheckType::ACCEPT_NON_EXISTING => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> ExistenceCheckType {
        match value {
            0 => ExistenceCheckType::ACCEPT_EITHER,
            1 => ExistenceCheckType::ACCEPT_EXISTING,
            2 => ExistenceCheckType::ACCEPT_NON_EXISTING,
            _ => panic!("Invalid value for ExistenceCheckType: {}", value),
        }
    }
}
impl AddAssign<u32> for ExistenceCheckType {
    fn add_assign(&mut self, rhs: u32) {
        *self = ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ExistenceCheckType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ExistenceCheckType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ExistenceCheckType {
    fn div_assign(&mut self, rhs: u32) {
        *self = ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ExistenceCheckType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ExistenceCheckType {
    type Output = ExistenceCheckType;
    fn add(self, rhs: u32) -> ExistenceCheckType {
        ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ExistenceCheckType {
    type Output = ExistenceCheckType;
    fn sub(self, rhs: u32) -> ExistenceCheckType {
        ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ExistenceCheckType {
    type Output = ExistenceCheckType;
    fn mul(self, rhs: u32) -> ExistenceCheckType {
        ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ExistenceCheckType {
    type Output = ExistenceCheckType;
    fn div(self, rhs: u32) -> ExistenceCheckType {
        ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ExistenceCheckType {
    type Output = ExistenceCheckType;
    fn rem(self, rhs: u32) -> ExistenceCheckType {
        ExistenceCheckType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locate_limits {
    pub limit: uintmax_t,
    pub items_accepted: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct process_data {
    pub c: i32,
    pub count: i32,
    pub len: i32,
    pub original_filename: *mut i8,
    pub pathsize: size_t,
    pub munged_filename: *mut i8,
    pub fp: *mut FILE,
    pub dbfile: *const i8,
    pub endian_state: GetwordEndianState,
    pub bigram1: [i8; 128],
    pub bigram2: [i8; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locate_stats {
    pub compressed_bytes: uintmax_t,
    pub total_filename_count: uintmax_t,
    pub total_filename_length: uintmax_t,
    pub whitespace_count: uintmax_t,
    pub newline_count: uintmax_t,
    pub highbit_filename_count: uintmax_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    TIME_BUF_LEN = 20,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::TIME_BUF_LEN => 20,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            20 => C2RustUnnamed_1::TIME_BUF_LEN,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub uch: [u8; 4],
    pub ui: u32,
}
pub type processfunc = Option<unsafe extern "C" fn(*mut process_data) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visitor {
    pub inspector: visitfunc,
    pub context: *mut libc::c_void,
    pub next: *mut visitor,
}
pub type visitfunc = Option<
    unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regular_expression {
    pub regex: re_pattern_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
    MAX_DB_AGE = 129,
    REGEXTYPE_OPTION = 128,
}
impl C2RustUnnamed_11 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_11::MAX_DB_AGE => 129,
            C2RustUnnamed_11::REGEXTYPE_OPTION => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_11 {
        match value {
            129 => C2RustUnnamed_11::MAX_DB_AGE,
            128 => C2RustUnnamed_11::REGEXTYPE_OPTION,
            _ => panic!("Invalid value for C2RustUnnamed_11: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_11 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_11 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_11 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_11 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_11 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn add(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn sub(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn mul(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn div(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn rem(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: i32,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut warn_number_units: u32 = 8 as i32 as u32;
static mut warn_name_units: [i8; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"days\0")
};
static mut check_existence: ExistenceCheckType = ExistenceCheckType::ACCEPT_EITHER;
static mut follow_symlinks: i32 = 1 as i32;
static mut separator: i32 = '\n' as i32;
static mut quote_opts: *mut quoting_options = 0 as *const quoting_options
    as *mut quoting_options;
static mut stdout_is_a_tty: bool = false;
static mut print_quoted_filename: bool = false;
static mut results_were_filtered: bool = false;
static mut selected_secure_db: *const i8 = 0 as *const i8;
unsafe extern "C" fn set_max_db_age(mut s: *const i8) {
    let mut end: *mut i8 = 0 as *mut i8;
    let mut val: u64 = 0;
    if 0 as i32 == *s as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_9>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"The argument for option --max-database-age must not be empty\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"The argument for option --max-database-age must not be empty\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    *__errno_location() = 0 as i32;
    val = strtoul(s, &mut end, 10 as i32);
    if (9223372036854775807 as i64 as u64).wrapping_mul(2 as u64).wrapping_add(1 as u64)
        == val && 34 as i32 == *__errno_location()
        || 0 as i32 as u64 == val && 22 as i32 == *__errno_location()
    {
        if ::core::mem::size_of::<C2RustUnnamed_8>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    } else if *end != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_7>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    } else {
        warn_number_units = val as u32;
    };
}
unsafe extern "C" fn get_short(mut fp: *mut FILE) -> libc::c_short {
    let mut x: libc::c_short = 0;
    x = ((fgetc(fp) as libc::c_schar as i32) << 8 as i32) as libc::c_short;
    x = (x as i32 | fgetc(fp) & 0xff as i32) as libc::c_short;
    return x;
}
static mut metacharacters: *const i8 = b"*?[]\\\0" as *const u8 as *const i8;
unsafe extern "C" fn contains_metacharacter(mut s: *const i8) -> i32 {
    if (strpbrk(s, metacharacters)).is_null() {
        return 0 as i32
    } else {
        return 1 as i32
    };
}
unsafe extern "C" fn locate_read_str(
    mut buf: *mut *mut i8,
    mut siz: *mut size_t,
    mut fp: *mut FILE,
    mut delimiter: i32,
    mut offs: i32,
) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut sz: size_t = 0 as i32 as size_t;
    let mut nread: i32 = 0;
    let mut needed: size_t = 0;
    nread = getdelim(&mut p, &mut sz, delimiter, fp) as i32;
    if nread >= 0 as i32 {
        if !p.is_null() {} else {
            __assert_fail(
                b"p != NULL\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                245 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[i8; 57],
                >(b"int locate_read_str(char **, size_t *, FILE *, int, int)\0"))
                    .as_ptr(),
            );
        }
        'c_9929: {
            if !p.is_null() {} else {
                __assert_fail(
                    b"p != NULL\0" as *const u8 as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    245 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 57],
                        &[i8; 57],
                    >(b"int locate_read_str(char **, size_t *, FILE *, int, int)\0"))
                        .as_ptr(),
                );
            }
        };
        needed = ((offs + nread) as u32).wrapping_add(1 as u32) as size_t;
        if needed > *siz {
            let mut pnew: *mut i8 = realloc(*buf as *mut libc::c_void, needed)
                as *mut i8;
            if pnew.is_null() {
                return -(1 as i32)
            } else {
                *siz = needed;
                *buf = pnew;
            }
        }
        memcpy(
            (*buf).offset(offs as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            (nread + 1 as i32) as u64,
        );
        rpl_free(p as *mut libc::c_void);
    }
    return nread;
}
static mut limits: locate_limits = locate_limits {
    limit: 0,
    items_accepted: 0,
};
static mut statistics: locate_stats = locate_stats {
    compressed_bytes: 0,
    total_filename_count: 0,
    total_filename_length: 0,
    whitespace_count: 0,
    newline_count: 0,
    highbit_filename_count: 0,
};
static mut inspectors: *mut visitor = 0 as *const visitor as *mut visitor;
static mut lastinspector: *mut visitor = 0 as *const visitor as *mut visitor;
static mut past_pat_inspector: *mut visitor = 0 as *const visitor as *mut visitor;
#[inline]
unsafe extern "C" fn visit(
    mut p: *const visitor,
    mut accept_flags: i32,
    mut procdata: *mut process_data,
    stop: *const visitor,
) -> i32 {
    let mut result: i32 = accept_flags;
    while accept_flags & result != 0 && stop != p {
        result = ((*p).inspector)
            .expect("non-null function pointer")(procdata, (*p).context);
        p = (*p).next;
    }
    return result;
}
unsafe extern "C" fn process_simple(mut procdata: *mut process_data) -> i32 {
    return visit(
        inspectors,
        visit_result::VISIT_CONTINUE as i32 | visit_result::VISIT_ACCEPTED as i32,
        procdata,
        0 as *const visitor,
    );
}
unsafe extern "C" fn process_or(mut procdata: *mut process_data) -> i32 {
    let mut result: i32 = 0;
    result = visit(
        inspectors,
        visit_result::VISIT_CONTINUE as i32 | visit_result::VISIT_REJECTED as i32,
        procdata,
        past_pat_inspector,
    );
    if result == visit_result::VISIT_CONTINUE as i32 {
        result = visit_result::VISIT_REJECTED as i32;
    }
    if result & (visit_result::VISIT_ABORT as i32 | visit_result::VISIT_REJECTED as i32)
        != 0
    {
        return result;
    }
    result = visit(
        past_pat_inspector,
        visit_result::VISIT_CONTINUE as i32,
        procdata,
        0 as *const visitor,
    );
    if visit_result::VISIT_CONTINUE as i32 == result {
        return visit_result::VISIT_ACCEPTED as i32
    } else {
        return result
    };
}
unsafe extern "C" fn process_and(mut procdata: *mut process_data) -> i32 {
    let mut result: i32 = 0;
    result = visit(
        inspectors,
        visit_result::VISIT_CONTINUE as i32 | visit_result::VISIT_ACCEPTED as i32,
        procdata,
        past_pat_inspector,
    );
    if result == visit_result::VISIT_CONTINUE as i32 {
        result = visit_result::VISIT_REJECTED as i32;
    }
    if result & (visit_result::VISIT_ABORT as i32 | visit_result::VISIT_REJECTED as i32)
        != 0
    {
        return result;
    }
    result = visit(
        past_pat_inspector,
        visit_result::VISIT_CONTINUE as i32,
        procdata,
        0 as *const visitor,
    );
    if visit_result::VISIT_CONTINUE as i32 == result {
        return visit_result::VISIT_ACCEPTED as i32
    } else {
        return result
    };
}
static mut mainprocessor: processfunc = None;
unsafe extern "C" fn add_visitor(mut fn_0: visitfunc, mut context: *mut libc::c_void) {
    let mut p: *mut visitor = xmalloc(::core::mem::size_of::<visitor>() as u64)
        as *mut visitor;
    (*p).inspector = fn_0;
    (*p).context = context;
    (*p).next = 0 as *mut visitor;
    if lastinspector.is_null() {
        inspectors = p;
        lastinspector = inspectors;
    } else {
        (*lastinspector).next = p;
        lastinspector = p;
    };
}
unsafe extern "C" fn visit_justprint_quoted(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    print_quoted(
        stdout,
        quote_opts,
        stdout_is_a_tty,
        b"%s\0" as *const u8 as *const i8,
        (*procdata).original_filename,
    );
    putchar(separator);
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn visit_justprint_unquoted(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    fputs((*procdata).original_filename, stdout);
    putchar(separator);
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn toolong(mut procdata: *mut process_data) {
    if ::core::mem::size_of::<C2RustUnnamed_4>() as u64 != 0 {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"locate database %s contains a filename longer than locate can handle\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            (*procdata).dbfile,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"locate database %s contains a filename longer than locate can handle\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            (*procdata).dbfile,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn extend(
    mut procdata: *mut process_data,
    mut siz1: size_t,
    mut siz2: size_t,
) {
    if (18446744073709551615 as u64).wrapping_sub(siz1) < siz2 {
        toolong(procdata);
    } else if (*procdata).pathsize < siz1.wrapping_add(siz2) {
        (*procdata).pathsize = siz1.wrapping_add(siz2);
        (*procdata).original_filename = x2nrealloc(
            (*procdata).original_filename as *mut libc::c_void,
            &mut (*procdata).pathsize,
            1 as i32 as size_t,
        ) as *mut i8;
    }
}
unsafe extern "C" fn visit_old_format(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut i: size_t = 0;
    if -(1 as i32) == (*procdata).c {
        return visit_result::VISIT_ABORT as i32;
    }
    if (*procdata).c == 30 as i32 {
        let mut minval: i32 = 0;
        let mut maxval: i32 = 0;
        let mut word: i32 = 0;
        (*procdata).count -= 14 as i32;
        minval = 0 as i32 - (*procdata).count;
        if (*procdata).count >= 0 as i32 {
            maxval = (*procdata).len - (*procdata).count;
        } else {
            maxval = (*procdata).len - 0 as i32;
        }
        word = getword(
            (*procdata).fp,
            (*procdata).dbfile,
            maxval as size_t,
            &mut (*procdata).endian_state,
        );
        if word >= minval {} else {
            __assert_fail(
                b"word >= minval\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                478 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[i8; 52],
                >(b"int visit_old_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
        'c_9215: {
            if word >= minval {} else {
                __assert_fail(
                    b"word >= minval\0" as *const u8 as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    478 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 52],
                        &[i8; 52],
                    >(b"int visit_old_format(struct process_data *, void *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*procdata).count += word;
        if (*procdata).count >= 0 as i32 {} else {
            __assert_fail(
                b"procdata->count >= 0\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                480 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[i8; 52],
                >(b"int visit_old_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
        'c_9168: {
            if (*procdata).count >= 0 as i32 {} else {
                __assert_fail(
                    b"procdata->count >= 0\0" as *const u8 as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    480 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 52],
                        &[i8; 52],
                    >(b"int visit_old_format(struct process_data *, void *)\0"))
                        .as_ptr(),
                );
            }
        };
    } else {
        (*procdata).count += (*procdata).c - 14 as i32;
        if (*procdata).count >= 0 as i32 {} else {
            __assert_fail(
                b"procdata->count >= 0\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                485 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[i8; 52],
                >(b"int visit_old_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
        'c_9106: {
            if (*procdata).count >= 0 as i32 {} else {
                __assert_fail(
                    b"procdata->count >= 0\0" as *const u8 as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    485 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 52],
                        &[i8; 52],
                    >(b"int visit_old_format(struct process_data *, void *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    i = (*procdata).count as size_t;
    loop {
        (*procdata).c = _IO_getc((*procdata).fp);
        if !((*procdata).c > 30 as i32) {
            break;
        }
        if -(1 as i32) == (*procdata).c {
            break;
        }
        if (*procdata).c < 0o200 as i32 {
            extend(procdata, i, 1 as u32 as size_t);
            let fresh0 = i;
            i = i.wrapping_add(1);
            *((*procdata).original_filename).offset(fresh0 as isize) = (*procdata).c
                as i8;
        } else {
            extend(procdata, i, 2 as u32 as size_t);
            (*procdata).c &= 0o177 as i32;
            let fresh1 = i;
            i = i.wrapping_add(1);
            *((*procdata).original_filename).offset(fresh1 as isize) = (*procdata)
                .bigram1[(*procdata).c as usize];
            let fresh2 = i;
            i = i.wrapping_add(1);
            *((*procdata).original_filename).offset(fresh2 as isize) = (*procdata)
                .bigram2[(*procdata).c as usize];
        }
    }
    extend(procdata, i, 1 as u32 as size_t);
    *((*procdata).original_filename).offset(i as isize) = 0 as i32 as i8;
    (*procdata).len = i as i32;
    (*procdata).munged_filename = (*procdata).original_filename;
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn visit_locate02_format(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut nread: i32 = 0;
    if (*procdata).c == 0x80 as i32 {
        (*procdata).count += get_short((*procdata).fp) as i32;
    } else if (*procdata).c > 127 as i32 {
        (*procdata).count += (*procdata).c - 256 as i32;
    } else {
        (*procdata).count += (*procdata).c;
    }
    if (*procdata).count > (*procdata).len || (*procdata).count < 0 as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"locate database %s is corrupt or invalid\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(
                    0 as i32,
                    quoting_style::locale_quoting_style,
                    (*procdata).dbfile,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"locate database %s is corrupt or invalid\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(
                    0 as i32,
                    quoting_style::locale_quoting_style,
                    (*procdata).dbfile,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    nread = locate_read_str(
        &mut (*procdata).original_filename,
        &mut (*procdata).pathsize,
        (*procdata).fp,
        0 as i32,
        (*procdata).count,
    );
    if nread < 1 as i32 {
        return visit_result::VISIT_ABORT as i32;
    }
    (*procdata).c = _IO_getc((*procdata).fp);
    (*procdata).len = (*procdata).count + nread - 1 as i32;
    if (*procdata).len < 1 as i32 {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"locate database %s is corrupt or invalid\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quotearg_n_style(
                0 as i32,
                quoting_style::locale_quoting_style,
                (*procdata).dbfile,
            ),
        );
    }
    s = ((*procdata).original_filename)
        .offset((*procdata).len as isize)
        .offset(-(1 as i32 as isize));
    if *s.offset(0 as i32 as isize) as i32 != '\0' as i32 {} else {
        __assert_fail(
            b"s[0] != '\\0'\0" as *const u8 as *const i8,
            b"locate.c\0" as *const u8 as *const i8,
            568 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[i8; 57],
            >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_9665: {
        if *s.offset(0 as i32 as isize) as i32 != '\0' as i32 {} else {
            __assert_fail(
                b"s[0] != '\\0'\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                568 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[i8; 57],
                >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if *s.offset(1 as i32 as isize) as i32 == '\0' as i32 {} else {
        __assert_fail(
            b"s[1] == '\\0'\0" as *const u8 as *const i8,
            b"locate.c\0" as *const u8 as *const i8,
            569 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[i8; 57],
            >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_9621: {
        if *s.offset(1 as i32 as isize) as i32 == '\0' as i32 {} else {
            __assert_fail(
                b"s[1] == '\\0'\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                569 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[i8; 57],
                >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if *s.offset(2 as i32 as isize) as i32 == '\0' as i32 {} else {
        __assert_fail(
            b"s[2] == '\\0'\0" as *const u8 as *const i8,
            b"locate.c\0" as *const u8 as *const i8,
            570 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[i8; 57],
            >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_9575: {
        if *s.offset(2 as i32 as isize) as i32 == '\0' as i32 {} else {
            __assert_fail(
                b"s[2] == '\\0'\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                570 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[i8; 57],
                >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*procdata).munged_filename = (*procdata).original_filename;
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn visit_basename(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    (*procdata).munged_filename = last_component((*procdata).original_filename);
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn visit_existing_follow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
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
    if stat((*procdata).original_filename, &mut st) != 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_CONTINUE as i32
    };
}
unsafe extern "C" fn visit_non_existing_follow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
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
    if stat((*procdata).original_filename, &mut st) == 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_CONTINUE as i32
    };
}
unsafe extern "C" fn visit_existing_nofollow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
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
    if lstat((*procdata).original_filename, &mut st) != 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_CONTINUE as i32
    };
}
unsafe extern "C" fn visit_non_existing_nofollow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
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
    if lstat((*procdata).original_filename, &mut st) == 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_CONTINUE as i32
    };
}
unsafe extern "C" fn visit_substring_match_nocasefold_wide(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut pattern: *const i8 = context as *const i8;
    if !(mbsstr((*procdata).munged_filename, pattern)).is_null() {
        return visit_result::VISIT_ACCEPTED as i32
    } else {
        return visit_result::VISIT_REJECTED as i32
    };
}
unsafe extern "C" fn visit_substring_match_nocasefold_narrow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut pattern: *const i8 = context as *const i8;
    if __ctype_get_mb_cur_max() == 1 as i32 as u64 {} else {
        __assert_fail(
            b"MB_CUR_MAX == 1\0" as *const u8 as *const i8,
            b"locate.c\0" as *const u8 as *const i8,
            690 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"int visit_substring_match_nocasefold_narrow(struct process_data *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8212: {
        if __ctype_get_mb_cur_max() == 1 as i32 as u64 {} else {
            __assert_fail(
                b"MB_CUR_MAX == 1\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                690 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"int visit_substring_match_nocasefold_narrow(struct process_data *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(strstr((*procdata).munged_filename, pattern)).is_null() {
        return visit_result::VISIT_ACCEPTED as i32
    } else {
        return visit_result::VISIT_REJECTED as i32
    };
}
unsafe extern "C" fn visit_substring_match_casefold_wide(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut pattern: *const i8 = context as *const i8;
    if !(mbscasestr((*procdata).munged_filename, pattern)).is_null() {
        return visit_result::VISIT_ACCEPTED as i32
    } else {
        return visit_result::VISIT_REJECTED as i32
    };
}
unsafe extern "C" fn visit_substring_match_casefold_narrow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut pattern: *const i8 = context as *const i8;
    if __ctype_get_mb_cur_max() == 1 as i32 as u64 {} else {
        __assert_fail(
            b"MB_CUR_MAX == 1\0" as *const u8 as *const i8,
            b"locate.c\0" as *const u8 as *const i8,
            714 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"int visit_substring_match_casefold_narrow(struct process_data *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8288: {
        if __ctype_get_mb_cur_max() == 1 as i32 as u64 {} else {
            __assert_fail(
                b"MB_CUR_MAX == 1\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                714 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"int visit_substring_match_casefold_narrow(struct process_data *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(strcasestr((*procdata).munged_filename, pattern)).is_null() {
        return visit_result::VISIT_ACCEPTED as i32
    } else {
        return visit_result::VISIT_REJECTED as i32
    };
}
unsafe extern "C" fn visit_globmatch_nofold(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut glob: *const i8 = context as *const i8;
    if fnmatch(glob, (*procdata).munged_filename, 0 as i32) != 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_ACCEPTED as i32
    };
}
unsafe extern "C" fn visit_globmatch_casefold(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut glob: *const i8 = context as *const i8;
    if fnmatch(glob, (*procdata).munged_filename, (1 as i32) << 4 as i32) != 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_ACCEPTED as i32
    };
}
unsafe extern "C" fn visit_regex(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut p: *mut regular_expression = context as *mut regular_expression;
    let len: size_t = strlen((*procdata).munged_filename);
    let mut rv: i32 = rpl_re_search(
        &mut (*p).regex,
        (*procdata).munged_filename,
        len as regoff_t,
        0 as i32 as regoff_t,
        len as regoff_t,
        0 as *mut libc::c_void as *mut re_registers,
    ) as i32;
    if rv < 0 as i32 {
        return visit_result::VISIT_REJECTED as i32
    } else {
        return visit_result::VISIT_ACCEPTED as i32
    };
}
unsafe extern "C" fn visit_stats(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut p: *mut locate_stats = context as *mut locate_stats;
    let mut len: size_t = strlen((*procdata).original_filename);
    let mut s: *const i8 = 0 as *const i8;
    let mut highbit: i32 = 0;
    let mut whitespace: i32 = 0;
    let mut newline: i32 = 0;
    (*p).total_filename_count = ((*p).total_filename_count).wrapping_add(1);
    (*p).total_filename_count;
    (*p).total_filename_length = ((*p).total_filename_length as u64).wrapping_add(len)
        as uintmax_t as uintmax_t;
    newline = 0 as i32;
    whitespace = newline;
    highbit = whitespace;
    s = (*procdata).original_filename;
    while *s != 0 {
        if *s as i32 & 128 as i32 != 0 {
            highbit = 1 as i32;
        }
        if '\n' as i32 == *s as i32 {
            whitespace = 1 as i32;
            newline = whitespace;
        } else if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
        {
            whitespace = 1 as i32;
        }
        s = s.offset(1);
        s;
    }
    if highbit != 0 {
        (*p).highbit_filename_count = ((*p).highbit_filename_count).wrapping_add(1);
        (*p).highbit_filename_count;
    }
    if whitespace != 0 {
        (*p).whitespace_count = ((*p).whitespace_count).wrapping_add(1);
        (*p).whitespace_count;
    }
    if newline != 0 {
        (*p).newline_count = ((*p).newline_count).wrapping_add(1);
        (*p).newline_count;
    }
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn visit_limit(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut p: *mut locate_limits = context as *mut locate_limits;
    (*p).items_accepted = ((*p).items_accepted).wrapping_add(1);
    if (*p).items_accepted >= (*p).limit {
        return visit_result::VISIT_ABORT as i32
    } else {
        return visit_result::VISIT_CONTINUE as i32
    };
}
unsafe extern "C" fn visit_count(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> i32 {
    let mut p: *mut locate_limits = context as *mut locate_limits;
    (*p).items_accepted = ((*p).items_accepted).wrapping_add(1);
    (*p).items_accepted;
    return visit_result::VISIT_CONTINUE as i32;
}
unsafe extern "C" fn print_stats(
    mut argc: i32,
    mut database_file_size: size_t,
    mut database_mtime: *const timespec,
) {
    let mut hbuf1: [i8; 652] = [0; 652];
    let mut hbuf2: [i8; 652] = [0; 652];
    let mut hbuf3: [i8; 652] = [0; 652];
    let mut hbuf4: [i8; 652] = [0; 652];
    if !database_mtime.is_null() {
        let mut ptm: *const tm = localtime(&(*database_mtime).tv_sec);
        if !ptm.is_null() {
            let mut whenbuf: [i8; 20] = [0; 20];
            let mut printed: size_t = strftime(
                whenbuf.as_mut_ptr(),
                C2RustUnnamed_1::TIME_BUF_LEN as i32 as size_t,
                b"%Y:%m:%d %H:%M:%S\0" as *const u8 as *const i8,
                ptm,
            );
            if printed == (C2RustUnnamed_1::TIME_BUF_LEN as i32 - 1 as i32) as u64
            {} else {
                __assert_fail(
                    b"printed == C2RustUnnamed_1::TIME_BUF_LEN-1\0" as *const u8
                        as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    845 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_6927: {
                if printed == (C2RustUnnamed_1::TIME_BUF_LEN as i32 - 1 as i32) as u64
                {} else {
                    __assert_fail(
                        b"printed == C2RustUnnamed_1::TIME_BUF_LEN-1\0" as *const u8
                            as *const i8,
                        b"locate.c\0" as *const u8 as *const i8,
                        845 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[i8; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if whenbuf[(C2RustUnnamed_1::TIME_BUF_LEN as i32 - 1 as i32) as usize] as i32
                == 0 as i32
            {} else {
                __assert_fail(
                    b"whenbuf[C2RustUnnamed_1::TIME_BUF_LEN-1] == 0\0" as *const u8
                        as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    846 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_6879: {
                if whenbuf[(C2RustUnnamed_1::TIME_BUF_LEN as i32 - 1 as i32) as usize]
                    as i32 == 0 as i32
                {} else {
                    __assert_fail(
                        b"whenbuf[C2RustUnnamed_1::TIME_BUF_LEN-1] == 0\0" as *const u8
                            as *const i8,
                        b"locate.c\0" as *const u8 as *const i8,
                        846 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[i8; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if whenbuf[(C2RustUnnamed_1::TIME_BUF_LEN as i32 - 2 as i32) as usize] as i32
                != 0 as i32
            {} else {
                __assert_fail(
                    b"whenbuf[C2RustUnnamed_1::TIME_BUF_LEN-2] != 0\0" as *const u8
                        as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    847 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_6830: {
                if whenbuf[(C2RustUnnamed_1::TIME_BUF_LEN as i32 - 2 as i32) as usize]
                    as i32 != 0 as i32
                {} else {
                    __assert_fail(
                        b"whenbuf[C2RustUnnamed_1::TIME_BUF_LEN-2] != 0\0" as *const u8
                            as *const i8,
                        b"locate.c\0" as *const u8 as *const i8,
                        847 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[i8; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Database was last modified at %s.%09ld\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                whenbuf.as_mut_ptr(),
                (*database_mtime).tv_nsec,
            );
            printed = strftime(
                whenbuf.as_mut_ptr(),
                C2RustUnnamed_1::TIME_BUF_LEN as i32 as size_t,
                b"%z\0" as *const u8 as *const i8,
                ptm,
            );
            if printed == 5 as i32 as u64 {} else {
                __assert_fail(
                    b"printed == 5\0" as *const u8 as *const i8,
                    b"locate.c\0" as *const u8 as *const i8,
                    851 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_6725: {
                if printed == 5 as i32 as u64 {} else {
                    __assert_fail(
                        b"printed == 5\0" as *const u8 as *const i8,
                        b"locate.c\0" as *const u8 as *const i8,
                        851 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[i8; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            printf(b" %s\n\0" as *const u8 as *const i8, whenbuf.as_mut_ptr());
        }
    }
    printf(
        dcngettext(
            0 as *const i8,
            b"Locate database size: %s byte\n\0" as *const u8 as *const i8,
            b"Locate database size: %s bytes\n\0" as *const u8 as *const i8,
            database_file_size,
            5 as i32,
        ),
        human_readable(
            database_file_size,
            hbuf1.as_mut_ptr(),
            C2RustUnnamed_0::human_ceiling as i32,
            1 as i32 as uintmax_t,
            1 as i32 as uintmax_t,
        ),
    );
    printf(
        if results_were_filtered as i32 != 0 {
            dcgettext(
                0 as *const i8,
                b"Matching Filenames: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            )
        } else {
            dcgettext(
                0 as *const i8,
                b"All Filenames: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            )
        },
        human_readable(
            statistics.total_filename_count,
            hbuf1.as_mut_ptr(),
            C2RustUnnamed_0::human_ceiling as i32,
            1 as i32 as uintmax_t,
            1 as i32 as uintmax_t,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"File names have a cumulative length of %s bytes.\nOf those file names,\n\n\t%s contain whitespace, \n\t%s contain newline characters, \n\tand %s contain characters with the high bit set.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        human_readable(
            statistics.total_filename_length,
            hbuf1.as_mut_ptr(),
            C2RustUnnamed_0::human_ceiling as i32,
            1 as i32 as uintmax_t,
            1 as i32 as uintmax_t,
        ),
        human_readable(
            statistics.whitespace_count,
            hbuf2.as_mut_ptr(),
            C2RustUnnamed_0::human_ceiling as i32,
            1 as i32 as uintmax_t,
            1 as i32 as uintmax_t,
        ),
        human_readable(
            statistics.newline_count,
            hbuf3.as_mut_ptr(),
            C2RustUnnamed_0::human_ceiling as i32,
            1 as i32 as uintmax_t,
            1 as i32 as uintmax_t,
        ),
        human_readable(
            statistics.highbit_filename_count,
            hbuf4.as_mut_ptr(),
            C2RustUnnamed_0::human_ceiling as i32,
            1 as i32 as uintmax_t,
            1 as i32 as uintmax_t,
        ),
    );
    if argc == 0 {
        if results_were_filtered {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Some filenames may have been filtered out, so we cannot compute the compression ratio.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else if statistics.total_filename_length != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Compression ratio %4.2f%% (higher is better)\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                100.0f64
                    * (statistics.total_filename_length as libc::c_double
                        - database_file_size as libc::c_double)
                    / statistics.total_filename_length as libc::c_double,
            );
        } else {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Compression ratio is undefined\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    printf(b"\n\0" as *const u8 as *const i8);
}
unsafe extern "C" fn looking_at_gnu_locatedb(
    mut data: *const i8,
    mut len: size_t,
) -> i32 {
    if len < ::core::mem::size_of::<[i8; 10]>() as u64 {
        return 0 as i32
    } else if 0 as i32
        == memcmp(
            data as *const libc::c_void,
            b"\0LOCATE02\0" as *const u8 as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<[i8; 10]>() as u64,
        )
    {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn looking_at_slocate_locatedb(
    mut filename: *const i8,
    mut data: *const i8,
    mut len: size_t,
    mut seclevel: *mut i32,
) -> i32 {
    if len <= 2 as i32 as u64 {} else {
        __assert_fail(
            b"len <= 2\0" as *const u8 as *const i8,
            b"locate.c\0" as *const u8 as *const i8,
            935 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"int looking_at_slocate_locatedb(const char *, const char *, size_t, int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10540: {
        if len <= 2 as i32 as u64 {} else {
            __assert_fail(
                b"len <= 2\0" as *const u8 as *const i8,
                b"locate.c\0" as *const u8 as *const i8,
                935 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"int looking_at_slocate_locatedb(const char *, const char *, size_t, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if len < 2 as i32 as u64 {
        return 0 as i32
    } else if 0 as i32 == *data.offset(1 as i32 as isize) as i32 {
        if *(*__ctype_b_loc())
            .offset(*data.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            *seclevel = *data.offset(0 as i32 as isize) as i32 - '0' as i32;
            if *seclevel > 1 as i32 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"locate database %s looks like an slocate database but it seems to have security level %c, which GNU findutils does not currently support\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quotearg_n_style(
                        0 as i32,
                        quoting_style::locale_quoting_style,
                        filename,
                    ),
                    *data.offset(1 as i32 as isize) as i32,
                );
                return 1 as i32;
            } else {
                return 1 as i32
            }
        } else {
            return 0 as i32
        }
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn i_am_little_endian() -> i32 {
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { uch: [0; 4] };
    u.ui = 0 as u32;
    u.uch[0 as i32 as usize] = 1 as i32 as u8;
    u.uch[3 as i32 as usize] = 0 as i32 as u8;
    u.uch[2 as i32 as usize] = u.uch[3 as i32 as usize];
    u.uch[1 as i32 as usize] = u.uch[2 as i32 as usize];
    return (u.ui == 1 as i32 as u32) as i32;
}
unsafe extern "C" fn search_one_database(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut dbfile: *const i8,
    mut fp: *mut FILE,
    mut filesize: off_t,
    mut database_mtime: *const timespec,
    mut ignore_case: i32,
    mut enable_print: i32,
    mut basename_only: i32,
    mut use_limit: i32,
    mut plimit: *mut locate_limits,
    mut stats: i32,
    mut op_and: i32,
    mut regex: i32,
    mut regex_options: i32,
) -> u64 {
    let mut pathpart: *mut i8 = 0 as *mut i8;
    let mut argn: i32 = 0;
    let mut nread: i32 = 0;
    let mut procdata: process_data = process_data {
        c: 0,
        count: 0,
        len: 0,
        original_filename: 0 as *mut i8,
        pathsize: 0,
        munged_filename: 0 as *mut i8,
        fp: 0 as *mut FILE,
        dbfile: 0 as *const i8,
        endian_state: GetwordEndianState::GetwordEndianStateInitial,
        bigram1: [0; 128],
        bigram2: [0; 128],
    };
    let mut slocate_seclevel: i32 = 0;
    let mut oldformat: i32 = 0;
    let mut slocatedb_format: i32 = 0;
    let mut pvis: *mut visitor = 0 as *mut visitor;
    let mut format_name: *const i8 = 0 as *const i8;
    let mut do_check_existence: ExistenceCheckType = ExistenceCheckType::ACCEPT_EITHER;
    do_check_existence = check_existence;
    if ignore_case != 0 {
        regex_options = (regex_options as u64
            | ((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32;
    }
    oldformat = 0 as i32;
    procdata.endian_state = GetwordEndianState::GetwordEndianStateInitial;
    procdata.count = 0 as i32;
    procdata.len = procdata.count;
    procdata.dbfile = dbfile;
    procdata.fp = fp;
    inspectors = 0 as *mut visitor;
    lastinspector = 0 as *mut visitor;
    past_pat_inspector = 0 as *mut visitor;
    results_were_filtered = 0 as i32 != 0;
    procdata.pathsize = 128 as i32 as size_t;
    procdata.original_filename = xmalloc(procdata.pathsize) as *mut i8;
    nread = fread(
        procdata.original_filename as *mut libc::c_void,
        1 as i32 as size_t,
        2 as i32 as size_t,
        procdata.fp,
    ) as i32;
    slocate_seclevel = 0 as i32;
    if looking_at_slocate_locatedb(
        procdata.dbfile,
        procdata.original_filename,
        nread as size_t,
        &mut slocate_seclevel,
    ) != 0
    {
        if slocate_seclevel > 1 as i32 {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is an slocate database of unsupported security level %d; skipping it.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(
                    0 as i32,
                    quoting_style::locale_quoting_style,
                    procdata.dbfile,
                ),
                slocate_seclevel,
            );
            return 0 as i32 as u64;
        } else if slocate_seclevel > 0 as i32 {
            if ExistenceCheckType::ACCEPT_NON_EXISTING as i32 as u32
                == check_existence as u32
            {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"You specified the -E option, but that option cannot be used with slocate-format databases with a non-zero security level.  No results will be generated for this database.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as i32 as u64;
            }
            if ExistenceCheckType::ACCEPT_EXISTING as i32 as u32
                != do_check_existence as u32
            {
                if enable_print != 0 || stats != 0 {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"%s is an slocate database.  Turning on the '-e' option.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quotearg_n_style(
                            0 as i32,
                            quoting_style::locale_quoting_style,
                            procdata.dbfile,
                        ),
                    );
                }
                do_check_existence = ExistenceCheckType::ACCEPT_EXISTING;
            }
        }
        add_visitor(
            Some(
                visit_locate02_format
                    as unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> i32,
            ),
            0 as *mut libc::c_void,
        );
        format_name = b"slocate\0" as *const u8 as *const i8;
        slocatedb_format = 1 as i32;
    } else {
        let mut nread2: i32 = 0;
        slocatedb_format = 0 as i32;
        extend(
            &mut procdata,
            ::core::mem::size_of::<[i8; 10]>() as u64,
            0 as u32 as size_t,
        );
        nread2 = fread(
            (procdata.original_filename).offset(nread as isize) as *mut libc::c_void,
            1 as i32 as size_t,
            (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(nread as u64),
            procdata.fp,
        ) as i32;
        if looking_at_gnu_locatedb(
            procdata.original_filename,
            (nread + nread2) as size_t,
        ) != 0
        {
            add_visitor(
                Some(
                    visit_locate02_format
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                0 as *mut libc::c_void,
            );
            format_name = b"GNU LOCATE02\0" as *const u8 as *const i8;
        } else {
            let mut i: i32 = 0;
            nread += nread2;
            extend(&mut procdata, 256 as u32 as size_t, 0 as u32 as size_t);
            if nread < 256 as i32 {
                let mut more_read: i32 = fread(
                    (procdata.original_filename).offset(nread as isize)
                        as *mut libc::c_void,
                    1 as i32 as size_t,
                    (256 as i32 - nread) as size_t,
                    procdata.fp,
                ) as i32;
                if more_read + nread != 256 as i32 {
                    if ::core::mem::size_of::<C2RustUnnamed_5>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"Old-format locate database %s is too short to be valid\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            quotearg_n_style(
                                0 as i32,
                                quoting_style::locale_quoting_style,
                                dbfile,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"Old-format locate database %s is too short to be valid\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            quotearg_n_style(
                                0 as i32,
                                quoting_style::locale_quoting_style,
                                dbfile,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            i = 0 as i32;
            while i < 128 as i32 {
                procdata.bigram1[i as usize] = *(procdata.original_filename)
                    .offset((i << 1 as i32) as isize);
                procdata.bigram2[i as usize] = *(procdata.original_filename)
                    .offset(((i << 1 as i32) + 1 as i32) as isize);
                i += 1;
                i;
            }
            format_name = b"old\0" as *const u8 as *const i8;
            oldformat = 1 as i32;
            add_visitor(
                Some(
                    visit_old_format
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                0 as *mut libc::c_void,
            );
        }
    }
    if basename_only != 0 {
        add_visitor(
            Some(
                visit_basename
                    as unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> i32,
            ),
            0 as *mut libc::c_void,
        );
    }
    argn = 0 as i32;
    while argn < argc {
        results_were_filtered = 1 as i32 != 0;
        pathpart = *argv.offset(argn as isize);
        if regex != 0 {
            let mut p: *mut regular_expression = xmalloc(
                ::core::mem::size_of::<regular_expression>() as u64,
            ) as *mut regular_expression;
            let mut error_message: *const i8 = 0 as *const i8;
            memset(
                &mut (*p).regex as *mut re_pattern_buffer as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<re_pattern_buffer>() as u64,
            );
            rpl_re_set_syntax(regex_options as reg_syntax_t);
            (*p).regex.allocated = 100 as i32 as __re_long_size_t;
            (*p).regex.buffer = xmalloc((*p).regex.allocated) as *mut re_dfa_t;
            (*p).regex.fastmap = 0 as *mut i8;
            (*p).regex.syntax = regex_options as reg_syntax_t;
            (*p).regex.translate = 0 as *mut u8;
            error_message = rpl_re_compile_pattern(
                pathpart,
                strlen(pathpart),
                &mut (*p).regex,
            );
            if !error_message.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        b"%s\0" as *const u8 as *const i8,
                        error_message,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        0 as i32,
                        b"%s\0" as *const u8 as *const i8,
                        error_message,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                add_visitor(
                    Some(
                        visit_regex
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    p as *mut libc::c_void,
                );
            }
        } else if contains_metacharacter(pathpart) != 0 {
            if ignore_case != 0 {
                add_visitor(
                    Some(
                        visit_globmatch_casefold
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    pathpart as *mut libc::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_globmatch_nofold
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    pathpart as *mut libc::c_void,
                );
            }
        } else {
            let mut matcher: visitfunc = None;
            if 1 as i32 as u64 == __ctype_get_mb_cur_max() {
                matcher = if ignore_case != 0 {
                    Some(
                        visit_substring_match_casefold_narrow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    )
                } else {
                    Some(
                        visit_substring_match_nocasefold_narrow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    )
                };
            } else {
                matcher = if ignore_case != 0 {
                    Some(
                        visit_substring_match_casefold_wide
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    )
                } else {
                    Some(
                        visit_substring_match_nocasefold_wide
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    )
                };
            }
            add_visitor(matcher, pathpart as *mut libc::c_void);
        }
        argn += 1;
        argn;
    }
    pvis = lastinspector;
    match do_check_existence as u32 {
        1 => {
            results_were_filtered = 1 as i32 != 0;
            if follow_symlinks != 0 {
                add_visitor(
                    Some(
                        visit_existing_follow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    0 as *mut libc::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_existing_nofollow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    0 as *mut libc::c_void,
                );
            }
        }
        2 => {
            results_were_filtered = 1 as i32 != 0;
            if follow_symlinks != 0 {
                add_visitor(
                    Some(
                        visit_non_existing_follow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    0 as *mut libc::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_non_existing_nofollow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> i32,
                    ),
                    0 as *mut libc::c_void,
                );
            }
        }
        0 | _ => {}
    }
    if stats != 0 {
        add_visitor(
            Some(
                visit_stats
                    as unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> i32,
            ),
            &mut statistics as *mut locate_stats as *mut libc::c_void,
        );
    }
    if enable_print != 0 {
        if print_quoted_filename {
            add_visitor(
                Some(
                    visit_justprint_quoted
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                0 as *mut libc::c_void,
            );
        } else {
            add_visitor(
                Some(
                    visit_justprint_unquoted
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                0 as *mut libc::c_void,
            );
        }
    }
    if use_limit != 0 {
        add_visitor(
            Some(
                visit_limit
                    as unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> i32,
            ),
            plimit as *mut libc::c_void,
        );
    } else {
        add_visitor(
            Some(
                visit_count
                    as unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> i32,
            ),
            plimit as *mut libc::c_void,
        );
    }
    if argc > 1 as i32 {
        past_pat_inspector = (*pvis).next;
        if op_and != 0 {
            mainprocessor = Some(
                process_and as unsafe extern "C" fn(*mut process_data) -> i32,
            );
        } else {
            mainprocessor = Some(
                process_or as unsafe extern "C" fn(*mut process_data) -> i32,
            );
        }
    } else {
        mainprocessor = Some(
            process_simple as unsafe extern "C" fn(*mut process_data) -> i32,
        );
    }
    if stats != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"Database %s is in the %s format.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            procdata.dbfile,
            format_name,
        );
    }
    procdata.c = _IO_getc(procdata.fp);
    if slocatedb_format != 0 && procdata.c != -(1 as i32) {
        ungetc(procdata.c, procdata.fp);
        procdata.c = 0 as i32;
    }
    while procdata.c != -(1 as i32)
        && visit_result::VISIT_ABORT as i32
            != mainprocessor.expect("non-null function pointer")(&mut procdata)
    {}
    if stats != 0 {
        if oldformat != 0 {
            let mut host_little_endian: i32 = i_am_little_endian();
            let mut little: *const i8 = dcgettext(
                0 as *const i8,
                b"The database has little-endian machine-word encoding.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            );
            let mut big: *const i8 = dcgettext(
                0 as *const i8,
                b"The database has big-endian machine-word encoding.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            );
            if GetwordEndianState::GetwordEndianStateNative as i32 as u32
                == procdata.endian_state as u32
            {
                printf(
                    b"%s\0" as *const u8 as *const i8,
                    if host_little_endian != 0 { little } else { big },
                );
            } else if GetwordEndianState::GetwordEndianStateSwab as i32 as u32
                == procdata.endian_state as u32
            {
                printf(
                    b"%s\0" as *const u8 as *const i8,
                    if host_little_endian != 0 { big } else { little },
                );
            } else {
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"The database machine-word encoding order is not obvious.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
        }
        if filesize != 0 || !database_mtime.is_null() {
            print_stats(argc, filesize as size_t, database_mtime);
        }
    }
    if ferror(procdata.fp) != 0 {
        error(
            0 as i32,
            *__errno_location(),
            b"%s\0" as *const u8 as *const i8,
            quotearg_n_style(
                0 as i32,
                quoting_style::locale_quoting_style,
                procdata.dbfile,
            ),
        );
        return 0 as i32 as u64;
    }
    return (*plimit).items_accepted;
}
unsafe extern "C" fn usage(mut status: i32) -> ! {
    if status != 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Try '%s --help' for more information.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [-d path | --database=path] [-e | -E | --[non-]existing]\n      [-i | --ignore-case] [-w | --wholename] [-b | --basename] \n      [--limit=N | -l N] [-S | --statistics] [-0 | --null] [-c | --count]\n      [-P | -H | --nofollow] [-L | --follow] [-m | --mmap] [-s | --stdio]\n      [-A | --all] [-p | --print] [-r | --regex] [--regextype=TYPE]\n      [--max-database-age D] [--version] [--help]\n      pattern...\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        program_name,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
static mut longopts: [option; 23] = [
    {
        let mut init = option {
            name: b"database\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"existing\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"non-existing\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"all\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"count\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wholename\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wholepath\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"basename\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"print\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdio\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mmap\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"limit\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"regex\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"regextype\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: C2RustUnnamed_11::REGEXTYPE_OPTION as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"statistics\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"follow\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nofollow\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-database-age\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: C2RustUnnamed_11::MAX_DB_AGE as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 0 as i32,
        };
        init
    },
];
unsafe extern "C" fn drop_privs() -> i32 {
    let mut current_block: u64;
    let mut what: *const i8 = b"failed\0" as *const u8 as *const i8;
    let orig_euid: uid_t = geteuid();
    let uid: uid_t = getuid();
    let gid: gid_t = getgid();
    if 0 as i32 as u32 == orig_euid {
        let mut groups: [gid_t; 1] = [0; 1];
        groups[0 as i32 as usize] = gid;
        if 0 as i32 != setgroups(1 as u32 as size_t, groups.as_mut_ptr()) {
            what = dcgettext(
                0 as *const i8,
                b"failed to drop group privileges\0" as *const u8 as *const i8,
                5 as i32,
            );
            current_block = 1718665703484364020;
        } else {
            current_block = 6873731126896040597;
        }
    } else {
        current_block = 6873731126896040597;
    }
    match current_block {
        6873731126896040597 => {
            if uid != orig_euid {
                if 0 as i32 as u32 == uid {
                    current_block = 17407779659766490442;
                } else {
                    *__errno_location() = 0 as i32;
                    if 0 as i32 != setuid(getuid()) {
                        what = dcgettext(
                            0 as *const i8,
                            b"failed to drop setuid privileges\0" as *const u8
                                as *const i8,
                            5 as i32,
                        );
                        current_block = 1718665703484364020;
                    } else if 0 as i32 == setuid(0 as i32 as __uid_t) {
                        what = dcgettext(
                            0 as *const i8,
                            b"Failed to fully drop privileges\0" as *const u8
                                as *const i8,
                            5 as i32,
                        );
                        *__errno_location() = 0 as i32;
                        current_block = 1718665703484364020;
                    } else {
                        current_block = 17407779659766490442;
                    }
                }
            } else {
                current_block = 17407779659766490442;
            }
            match current_block {
                1718665703484364020 => {}
                _ => {
                    *__errno_location() = 0 as i32;
                    if 0 as i32 != setgid(gid) {
                        what = dcgettext(
                            0 as *const i8,
                            b"failed to drop setgid privileges\0" as *const u8
                                as *const i8,
                            5 as i32,
                        );
                    } else {
                        return 0 as i32
                    }
                }
            }
        }
        _ => {}
    }
    if ::core::mem::size_of::<C2RustUnnamed_12>() as u64 != 0 {
        error(
            1 as i32,
            *__errno_location(),
            b"%s\0" as *const u8 as *const i8,
            quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, what),
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as i32,
            *__errno_location(),
            b"%s\0" as *const u8 as *const i8,
            quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, what),
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
    abort();
}
unsafe extern "C" fn opendb(mut name: *const i8) -> i32 {
    let mut fd: i32 = open_safer(name, 0 as i32 | 0 as i32);
    if fd >= 0 as i32 {
        if 0 as i32 != rpl_fcntl(fd, 2 as i32, 1 as i32) {
            close(fd);
            fd = -(1 as i32);
        }
    }
    return fd;
}
unsafe extern "C" fn cleanup_quote_opts() {
    rpl_free(quote_opts as *mut libc::c_void);
}
unsafe extern "C" fn dolocate(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut secure_db_fd: i32,
) -> i32 {
    let mut path_element: *mut i8 = 0 as *mut i8;
    let mut path_element_pos: size_t = 0;
    let mut path_element_len: size_t = 0;
    let mut user_selected_locate_path: *const i8 = 0 as *const i8;
    let mut db_name: *const i8 = 0 as *const i8;
    let mut path_separators: *const i8 = b":\0" as *const u8 as *const i8;
    let mut found: u64 = 0 as u64;
    let mut ignore_case: i32 = 0 as i32;
    let mut print: i32 = 0 as i32;
    let mut just_count: i32 = 0 as i32;
    let mut basename_only: i32 = 0 as i32;
    let mut use_limit: i32 = 0 as i32;
    let mut regex: i32 = 0 as i32;
    let mut regex_options: i32 = 0 as i32;
    let mut stats: i32 = 0 as i32;
    let mut op_and: i32 = 0 as i32;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut did_stdin: bool = 0 as i32 != 0;
    if !(*argv.offset(0 as i32 as isize)).is_null() {
        set_program_name(*argv.offset(0 as i32 as isize));
    } else {
        set_program_name(b"locate\0" as *const u8 as *const i8);
    }
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"findutils\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"findutils\0" as *const u8 as *const i8);
    quote_opts = clone_quoting_options(0 as *mut quoting_options);
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0
        || atexit(Some(cleanup_quote_opts as unsafe extern "C" fn() -> ())) != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_10>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    limits.limit = 0 as i32 as uintmax_t;
    limits.items_accepted = 0 as i32 as uintmax_t;
    print_quoted_filename = 1 as i32 != 0;
    user_selected_locate_path = getenv(b"LOCATE_PATH\0" as *const u8 as *const i8);
    check_existence = ExistenceCheckType::ACCEPT_EITHER;
    loop {
        let mut opti: i32 = -(1 as i32);
        let mut optc: i32 = getopt_long(
            argc,
            argv,
            b"Abcd:eEil:prsm0SwHPL\0" as *const u8 as *const i8,
            longopts.as_ptr(),
            &mut opti,
        );
        if optc == -(1 as i32) {
            break;
        }
        match optc {
            48 => {
                separator = 0 as i32;
                print_quoted_filename = 0 as i32 != 0;
            }
            65 => {
                op_and = 1 as i32;
            }
            98 => {
                basename_only = 1 as i32;
            }
            99 => {
                just_count = 1 as i32;
            }
            100 => {
                user_selected_locate_path = optarg;
                if !optarg.is_null() {} else {
                    __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const i8,
                        b"locate.c\0" as *const u8 as *const i8,
                        1613 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 32],
                            &[i8; 32],
                        >(b"int dolocate(int, char **, int)\0"))
                            .as_ptr(),
                    );
                }
                'c_12231: {
                    if !optarg.is_null() {} else {
                        __assert_fail(
                            b"optarg != NULL\0" as *const u8 as *const i8,
                            b"locate.c\0" as *const u8 as *const i8,
                            1613 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 32],
                                &[i8; 32],
                            >(b"int dolocate(int, char **, int)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            101 => {
                check_existence = ExistenceCheckType::ACCEPT_EXISTING;
            }
            69 => {
                check_existence = ExistenceCheckType::ACCEPT_NON_EXISTING;
            }
            105 => {
                ignore_case = 1 as i32;
            }
            104 => {
                usage(0 as i32);
            }
            129 => {
                set_max_db_age(optarg);
            }
            112 => {
                print = 1 as i32;
            }
            118 => {
                display_findutils_version(b"locate\0" as *const u8 as *const i8);
                return 0 as i32;
            }
            119 => {
                basename_only = 0 as i32;
            }
            114 => {
                regex = 1 as i32;
            }
            128 => {
                regex_options = get_regex_type(optarg);
            }
            83 => {
                stats = 1 as i32;
            }
            76 => {
                follow_symlinks = 1 as i32;
            }
            80 | 72 => {
                follow_symlinks = 0 as i32;
            }
            108 => {
                let mut end: *mut i8 = optarg;
                let mut err: strtol_error = xstrtoumax(
                    optarg,
                    &mut end,
                    10 as i32,
                    &mut limits.limit,
                    0 as *const i8,
                );
                if strtol_error::LONGINT_OK as i32 as u32 != err as u32 {
                    xstrtol_fatal(err, opti, optc as i8, longopts.as_ptr(), optarg);
                }
                use_limit = 1 as i32;
            }
            115 => {}
            109 => {}
            _ => {
                usage(1 as i32);
            }
        }
    }
    if !user_selected_locate_path.is_null() {
        if secure_db_fd >= 0 as i32 {
            close(secure_db_fd);
            secure_db_fd = -(1 as i32);
        }
    }
    if just_count == 0 && stats == 0 {
        print = 1 as i32;
    }
    if stats != 0 {
        if optind == argc {
            use_limit = 0 as i32;
        }
    } else if just_count == 0 && optind == argc {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"pattern argument expected\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(1 as i32);
    }
    if 1 as i32 == isatty(1 as i32) {
        stdout_is_a_tty = 1 as i32 != 0;
    } else {
        stdout_is_a_tty = 0 as i32 != 0;
    }
    if !user_selected_locate_path.is_null() {
        splitstring(
            user_selected_locate_path,
            path_separators,
            1 as i32 != 0,
            &mut path_element_pos,
            &mut path_element_len,
        );
    }
    while use_limit == 0 || limits.limit > limits.items_accepted {
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
        let mut database_mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut have_mtime: i32 = 0;
        let mut fd: i32 = 0;
        let mut filesize: off_t = 0;
        statistics.highbit_filename_count = 0 as u32 as uintmax_t;
        statistics.newline_count = statistics.highbit_filename_count;
        statistics.whitespace_count = statistics.newline_count;
        statistics.total_filename_length = statistics.whitespace_count;
        statistics.total_filename_count = statistics.total_filename_length;
        statistics.compressed_bytes = statistics.total_filename_count;
        if !user_selected_locate_path.is_null() {
            if 1 as i32 as u64 == path_element_len
                && '-' as i32
                    == *user_selected_locate_path.offset(path_element_pos as isize)
                        as i32
            {
                if did_stdin {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"warning: the locate database can only be read from stdin once.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    return 0 as i32;
                } else {
                    db_name = b"<stdin>\0" as *const u8 as *const i8;
                    fd = 0 as i32;
                    did_stdin = 1 as i32 != 0;
                }
            } else {
                if 0 as i32 as u64 == path_element_len
                    || 1 as i32 as u64 == path_element_len
                        && '.' as i32
                            == *user_selected_locate_path
                                .offset(path_element_pos as isize) as i32
                {
                    db_name = b"/usr/local/var/locatedb\0" as *const u8 as *const i8;
                } else {
                    path_element = strndup(
                        &*user_selected_locate_path.offset(path_element_pos as isize),
                        path_element_len,
                    );
                    db_name = path_element;
                }
                fd = opendb(db_name);
                if fd < 0 as i32 {
                    error(
                        0 as i32,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const i8,
                        quotearg_n_style(
                            0 as i32,
                            quoting_style::locale_quoting_style,
                            db_name,
                        ),
                    );
                    return 0 as i32;
                }
            }
        } else if -(1 as i32) == secure_db_fd {
            break;
        } else {
            db_name = selected_secure_db;
            fd = secure_db_fd;
            secure_db_fd = -(1 as i32);
        }
        if fstat(fd, &mut st) != 0 {
            error(
                0 as i32,
                *__errno_location(),
                b"%s\0" as *const u8 as *const i8,
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, db_name),
            );
            filesize = 0 as i32 as off_t;
            have_mtime = 0 as i32;
        } else {
            let mut now: time_t = 0;
            filesize = st.st_size;
            database_mtime = get_stat_mtime(&mut st);
            have_mtime = 1 as i32;
            if -(1 as i32) as time_t == time(&mut now) {
                error(
                    0 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"time system call failed\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            } else {
                let mut age: libc::c_double = difftime(now, st.st_mtim.tv_sec);
                let mut warn_seconds: libc::c_double = ((60 as i32 * 60 as i32
                    * 24 as i32) as u32)
                    .wrapping_mul(warn_number_units) as libc::c_double;
                if age > warn_seconds {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"warning: database %s is more than %u %s old (actual age is %.1f %s)\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quotearg_n_style(
                            0 as i32,
                            quoting_style::locale_quoting_style,
                            db_name,
                        ),
                        warn_number_units,
                        dcgettext(0 as *const i8, warn_name_units.as_ptr(), 5 as i32),
                        age / (60 as i32 * 60 as i32 * 24 as i32) as libc::c_double,
                        dcgettext(0 as *const i8, warn_name_units.as_ptr(), 5 as i32),
                    );
                }
            }
        }
        fp = fdopen(fd, b"r\0" as *const u8 as *const i8);
        if fp.is_null() {
            error(
                0 as i32,
                *__errno_location(),
                b"%s\0" as *const u8 as *const i8,
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, db_name),
            );
            return 0 as i32;
        }
        found = search_one_database(
            argc - optind,
            &mut *argv.offset(optind as isize),
            db_name,
            fp,
            filesize,
            if have_mtime != 0 { &mut database_mtime } else { 0 as *mut timespec },
            ignore_case,
            print,
            basename_only,
            use_limit,
            &mut limits,
            stats,
            op_and,
            regex,
            regex_options,
        );
        if rpl_fclose(fp) == -(1 as i32) {
            error(
                0 as i32,
                *__errno_location(),
                b"%s\0" as *const u8 as *const i8,
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, db_name),
            );
            return 0 as i32;
        }
        if !path_element.is_null() {
            rpl_free(path_element as *mut libc::c_void);
            path_element = 0 as *mut i8;
        }
        if user_selected_locate_path.is_null() {
            break;
        }
        if !splitstring(
            user_selected_locate_path,
            path_separators,
            0 as i32 != 0,
            &mut path_element_pos,
            &mut path_element_len,
        ) {
            break;
        }
    }
    if just_count != 0 {
        printf(b"%lu\n\0" as *const u8 as *const i8, found);
    }
    if found != 0 || use_limit != 0 && limits.limit == 0 as i32 as u64 || stats != 0 {
        return 0 as i32
    } else {
        return 1 as i32
    };
}
unsafe extern "C" fn open_secure_db() -> i32 {
    let mut fd: i32 = 0;
    let mut i: i32 = 0;
    let mut secure_db_list: [*const i8; 3] = [
        b"/usr/local/var/locatedb\0" as *const u8 as *const i8,
        b"/var/lib/slocate/slocate.db\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    i = 0 as i32;
    while !(secure_db_list[i as usize]).is_null() {
        fd = opendb(secure_db_list[i as usize]);
        if fd >= 0 as i32 {
            selected_secure_db = secure_db_list[i as usize];
            return fd;
        }
        i += 1;
        i;
    }
    return -(1 as i32);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut dbfd: i32 = open_secure_db();
    drop_privs();
    return dolocate(argc, argv, dbfd);
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