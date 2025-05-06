#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use num_traits::Float;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __isnanl(__value: f128::f128) -> i32;
    fn __isinfl(__value: f128::f128) -> i32;
    fn __isnanf(__value: libc::c_float) -> i32;
    fn __isinff(__value: libc::c_float) -> i32;
    fn __isnan(__value: libc::c_double) -> i32;
    fn __isinf(__value: libc::c_double) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fileno(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn system(__command: *const i8) -> i32;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
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
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn timegm(__tp: *mut tm) -> time_t;
    fn __errno_location() -> *mut i32;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn wcrtomb(__s: *mut i8, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn __mbrlen(__s: *const i8, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn getpid() -> __pid_t;
    static mut BINMODE: i32;
    static mut IGNORECASE: bool;
    static mut OFS: *mut i8;
    static mut OFSlen: i32;
    static mut ORS: *mut i8;
    static mut ORSlen: i32;
    static mut OFMT: *mut i8;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut OFMTidx: i32;
    static mut TEXTDOMAIN: *mut i8;
    static mut FS_node: *mut NODE;
    static mut RLENGTH_node: *mut NODE;
    static mut RSTART_node: *mut NODE;
    static mut SUBSEP_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut FPAT_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut fields_arr: *mut *mut NODE;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut field0_valid: bool;
    static mut do_flags: do_flag_values;
    static mut use_lc_numeric: i32;
    static mut gawk_mb_cur_max: i32;
    static mut loc: lconv;
    static def_strftime_format: [i8; 0];
    static mut casetable: [i8; 0];
    static mut stack_ptr: *mut STACK_ITEM;
    static mut stack_top: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn is_alpha(c: i32) -> bool;
    fn make_regnode(type_0: NODETYPE, exp_0: *mut NODE) -> *mut NODE;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn update_ERRNO_int(_: i32);
    fn is_non_fatal_std(fp: *mut FILE) -> bool;
    fn is_non_fatal_redirect(str: *const i8, len: size_t) -> bool;
    fn os_maybe_set_errno();
    fn r_warning(mesg: *const i8, _: ...);
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn non_fatal_flush_std_file(fp: *mut FILE) -> bool;
    fn getredirect(str: *const i8, len: i32) -> *mut redirect;
    fn flush_io() -> i32;
    fn wstrstr(
        haystack: *const wchar_t,
        hs_len: size_t,
        needle: *const wchar_t,
        needle_len: size_t,
    ) -> *const wchar_t;
    fn wcasestrstr(
        haystack: *const wchar_t,
        hs_len: size_t,
        needle: *const wchar_t,
        needle_len: size_t,
    ) -> *const wchar_t;
    fn str2wstr(n: *mut NODE, ptr: *mut *mut size_t) -> *mut NODE;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn gawk_exit(status: i32);
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn msg(mesg: *const i8, _: ...);
    fn gawk_fwrite(
        buf: *const libc::c_void,
        size: size_t,
        count: size_t,
        fp: *mut FILE,
        opaque: *mut libc::c_void,
    ) -> size_t;
    fn close_rp(rp: *mut redirect, how: two_way_close_type) -> i32;
    fn redirect(
        redir_exp: *mut NODE,
        redirtype: i32,
        errflg: *mut i32,
        failure_fatal: bool,
    ) -> *mut redirect;
    fn os_setbinmode(fd: i32, mode: i32) -> i32;
    fn os_restore_mode(fd: i32);
    fn get_field(num: i64, assign: *mut Func_ptr) -> *mut *mut NODE;
    fn wstr2str(n: *mut NODE) -> *mut NODE;
    fn research(
        rp: *mut Regexp,
        str: *mut i8,
        start: i32,
        len: size_t,
        flags: i32,
    ) -> i32;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    static mut func_table: *mut NODE;
    static mut symbol_table: *mut NODE;
    fn make_typed_regex(re: *const i8, len: size_t) -> *mut NODE;
    fn refree(rp: *mut Regexp);
    fn reset_record();
    fn grow_stack() -> *mut STACK_ITEM;
    fn r_get_field(
        n: *mut NODE,
        assign: *mut Func_ptr,
        reference: bool,
    ) -> *mut *mut NODE;
    fn do_patsplit(nargs: i32) -> *mut NODE;
    fn do_split(nargs: i32) -> *mut NODE;
    fn adjust_uint(n: uintmax_t) -> uintmax_t;
    fn get_numbase(str: *const i8, len: size_t, use_locale: bool) -> i32;
    fn nodetype2str(type_0: NODETYPE) -> *const i8;
    fn flags2str(_: i32) -> *const i8;
    static mut btowc_cache: [wint_t; 0];
    fn make_bool_node(value: bool) -> *mut NODE;
    fn iswlower(__wc: wint_t) -> i32;
    fn iswupper(__wc: wint_t) -> i32;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn gawk_initstate(seed: u64, state_0: *mut i8, n: i64) -> *mut i8;
    fn gawk_setstate(state_0: *mut i8) -> *mut i8;
    fn gawk_random() -> i64;
    fn gawk_srandom(seed: u64);
    static mut args_array: *mut *mut NODE;
    static mut output_is_tty: bool;
    static mut output_fp: *mut FILE;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __int32_t = i32;
pub type __intmax_t = i64;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
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
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::_ISalnum => 8,
            C2RustUnnamed_0::_ISpunct => 4,
            C2RustUnnamed_0::_IScntrl => 2,
            C2RustUnnamed_0::_ISblank => 1,
            C2RustUnnamed_0::_ISgraph => 32768,
            C2RustUnnamed_0::_ISprint => 16384,
            C2RustUnnamed_0::_ISspace => 8192,
            C2RustUnnamed_0::_ISxdigit => 4096,
            C2RustUnnamed_0::_ISdigit => 2048,
            C2RustUnnamed_0::_ISalpha => 1024,
            C2RustUnnamed_0::_ISlower => 512,
            C2RustUnnamed_0::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            8 => C2RustUnnamed_0::_ISalnum,
            4 => C2RustUnnamed_0::_ISpunct,
            2 => C2RustUnnamed_0::_IScntrl,
            1 => C2RustUnnamed_0::_ISblank,
            32768 => C2RustUnnamed_0::_ISgraph,
            16384 => C2RustUnnamed_0::_ISprint,
            8192 => C2RustUnnamed_0::_ISspace,
            4096 => C2RustUnnamed_0::_ISxdigit,
            2048 => C2RustUnnamed_0::_ISdigit,
            1024 => C2RustUnnamed_0::_ISalpha,
            512 => C2RustUnnamed_0::_ISlower,
            256 => C2RustUnnamed_0::_ISupper,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut i8,
    pub thousands_sep: *mut i8,
    pub grouping: *mut i8,
    pub int_curr_symbol: *mut i8,
    pub currency_symbol: *mut i8,
    pub mon_decimal_point: *mut i8,
    pub mon_thousands_sep: *mut i8,
    pub mon_grouping: *mut i8,
    pub positive_sign: *mut i8,
    pub negative_sign: *mut i8,
    pub int_frac_digits: i8,
    pub frac_digits: i8,
    pub p_cs_precedes: i8,
    pub p_sep_by_space: i8,
    pub n_cs_precedes: i8,
    pub n_sep_by_space: i8,
    pub p_sign_posn: i8,
    pub n_sign_posn: i8,
    pub int_p_cs_precedes: i8,
    pub int_p_sep_by_space: i8,
    pub int_n_cs_precedes: i8,
    pub int_n_sep_by_space: i8,
    pub int_p_sign_posn: i8,
    pub int_n_sign_posn: i8,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
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
pub type wint_t = u32;
pub type mbstate_t = __mbstate_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
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
pub type __re_size_t = u32;
pub type __re_long_size_t = u64;
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
pub type regoff_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub has_meta: bool,
    pub maybe_long: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_bool {
    awk_false = 0,
    awk_true,
}
impl awk_bool {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            awk_bool::awk_false => 0,
            awk_bool::awk_true => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> awk_bool {
        match value {
            0 => awk_bool::awk_false,
            1 => awk_bool::awk_true,
            _ => panic!("Invalid value for awk_bool: {}", value),
        }
    }
}
impl AddAssign<u32> for awk_bool {
    fn add_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for awk_bool {
    fn sub_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for awk_bool {
    fn mul_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for awk_bool {
    fn div_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for awk_bool {
    fn rem_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for awk_bool {
    type Output = awk_bool;
    fn add(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for awk_bool {
    type Output = awk_bool;
    fn sub(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for awk_bool {
    type Output = awk_bool;
    fn mul(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for awk_bool {
    type Output = awk_bool;
    fn div(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for awk_bool {
    type Output = awk_bool;
    fn rem(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input {
    pub name: *const i8,
    pub fd: i32,
    pub opaque: *mut libc::c_void,
    pub get_record: Option<
        unsafe extern "C" fn(
            *mut *mut i8,
            *mut awk_input,
            *mut i32,
            *mut *mut i8,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> i32,
    >,
    pub read_func: Option<
        unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const i8,
    pub mode: *const i8,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
    pub gawk_ferror: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
    pub gawk_fclose: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut i8,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_MPZ,
}
impl AWK_NUMBER_TYPE {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE => 0,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR => 1,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> AWK_NUMBER_TYPE {
        match value {
            0 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE,
            1 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR,
            2 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ,
            _ => panic!("Invalid value for AWK_NUMBER_TYPE: {}", value),
        }
    }
}
impl AddAssign<u32> for AWK_NUMBER_TYPE {
    fn add_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AWK_NUMBER_TYPE {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AWK_NUMBER_TYPE {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AWK_NUMBER_TYPE {
    fn div_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AWK_NUMBER_TYPE {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn add(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn sub(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn mul(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn div(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn rem(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_valtype_t {
    AWK_UNDEFINED,
    AWK_NUMBER,
    AWK_STRING,
    AWK_REGEX,
    AWK_STRNUM,
    AWK_ARRAY,
    AWK_SCALAR,
    AWK_VALUE_COOKIE,
    AWK_BOOL,
}
impl awk_valtype_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            awk_valtype_t::AWK_UNDEFINED => 0,
            awk_valtype_t::AWK_NUMBER => 1,
            awk_valtype_t::AWK_STRING => 2,
            awk_valtype_t::AWK_REGEX => 3,
            awk_valtype_t::AWK_STRNUM => 4,
            awk_valtype_t::AWK_ARRAY => 5,
            awk_valtype_t::AWK_SCALAR => 6,
            awk_valtype_t::AWK_VALUE_COOKIE => 7,
            awk_valtype_t::AWK_BOOL => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> awk_valtype_t {
        match value {
            0 => awk_valtype_t::AWK_UNDEFINED,
            1 => awk_valtype_t::AWK_NUMBER,
            2 => awk_valtype_t::AWK_STRING,
            3 => awk_valtype_t::AWK_REGEX,
            4 => awk_valtype_t::AWK_STRNUM,
            5 => awk_valtype_t::AWK_ARRAY,
            6 => awk_valtype_t::AWK_SCALAR,
            7 => awk_valtype_t::AWK_VALUE_COOKIE,
            8 => awk_valtype_t::AWK_BOOL,
            _ => panic!("Invalid value for awk_valtype_t: {}", value),
        }
    }
}
impl AddAssign<u32> for awk_valtype_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for awk_valtype_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for awk_valtype_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for awk_valtype_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for awk_valtype_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn add(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn sub(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn mul(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn div(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn rem(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const i8,
    pub function: Option<
        unsafe extern "C" fn(
            i32,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nodevals {
    Node_illegal,
    Node_val,
    Node_regex,
    Node_dynregex,
    Node_var,
    Node_var_array,
    Node_var_new,
    Node_elem_new,
    Node_param_list,
    Node_func,
    Node_ext_func,
    Node_builtin_func,
    Node_array_ref,
    Node_array_tree,
    Node_array_leaf,
    Node_dump_array,
    Node_arrayfor,
    Node_frame,
    Node_instruction,
    Node_final,
}
impl nodevals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            nodevals::Node_illegal => 0,
            nodevals::Node_val => 1,
            nodevals::Node_regex => 2,
            nodevals::Node_dynregex => 3,
            nodevals::Node_var => 4,
            nodevals::Node_var_array => 5,
            nodevals::Node_var_new => 6,
            nodevals::Node_elem_new => 7,
            nodevals::Node_param_list => 8,
            nodevals::Node_func => 9,
            nodevals::Node_ext_func => 10,
            nodevals::Node_builtin_func => 11,
            nodevals::Node_array_ref => 12,
            nodevals::Node_array_tree => 13,
            nodevals::Node_array_leaf => 14,
            nodevals::Node_dump_array => 15,
            nodevals::Node_arrayfor => 16,
            nodevals::Node_frame => 17,
            nodevals::Node_instruction => 18,
            nodevals::Node_final => 19,
        }
    }
    fn from_libc_c_uint(value: u32) -> nodevals {
        match value {
            0 => nodevals::Node_illegal,
            1 => nodevals::Node_val,
            2 => nodevals::Node_regex,
            3 => nodevals::Node_dynregex,
            4 => nodevals::Node_var,
            5 => nodevals::Node_var_array,
            6 => nodevals::Node_var_new,
            7 => nodevals::Node_elem_new,
            8 => nodevals::Node_param_list,
            9 => nodevals::Node_func,
            10 => nodevals::Node_ext_func,
            11 => nodevals::Node_builtin_func,
            12 => nodevals::Node_array_ref,
            13 => nodevals::Node_array_tree,
            14 => nodevals::Node_array_leaf,
            15 => nodevals::Node_dump_array,
            16 => nodevals::Node_arrayfor,
            17 => nodevals::Node_frame,
            18 => nodevals::Node_instruction,
            19 => nodevals::Node_final,
            _ => panic!("Invalid value for nodevals: {}", value),
        }
    }
}
impl AddAssign<u32> for nodevals {
    fn add_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for nodevals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for nodevals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for nodevals {
    fn div_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for nodevals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for nodevals {
    type Output = nodevals;
    fn add(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for nodevals {
    type Output = nodevals;
    fn sub(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for nodevals {
    type Output = nodevals;
    fn mul(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for nodevals {
    type Output = nodevals;
    fn div(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for nodevals {
    type Output = nodevals;
    fn rem(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_2,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: i64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flagvals {
    REGEX = 524288,
    NUMCONSTSTR = 262144,
    XARRAY = 131072,
    HALFHAT = 65536,
    ARRAYMAXED = 32768,
    NULL_FIELD = 16384,
    NO_EXT_SET = 8192,
    MPZN = 4096,
    MPFN = 2048,
    WSTRCUR = 1024,
    INTIND = 512,
    NUMINT = 256,
    INTLSTR = 128,
    BOOLVAL = 64,
    USER_INPUT = 32,
    NUMBER = 16,
    NUMCUR = 8,
    STRCUR = 4,
    STRING = 2,
    MALLOC = 1,
}
impl flagvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            flagvals::REGEX => 524288,
            flagvals::NUMCONSTSTR => 262144,
            flagvals::XARRAY => 131072,
            flagvals::HALFHAT => 65536,
            flagvals::ARRAYMAXED => 32768,
            flagvals::NULL_FIELD => 16384,
            flagvals::NO_EXT_SET => 8192,
            flagvals::MPZN => 4096,
            flagvals::MPFN => 2048,
            flagvals::WSTRCUR => 1024,
            flagvals::INTIND => 512,
            flagvals::NUMINT => 256,
            flagvals::INTLSTR => 128,
            flagvals::BOOLVAL => 64,
            flagvals::USER_INPUT => 32,
            flagvals::NUMBER => 16,
            flagvals::NUMCUR => 8,
            flagvals::STRCUR => 4,
            flagvals::STRING => 2,
            flagvals::MALLOC => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> flagvals {
        match value {
            524288 => flagvals::REGEX,
            262144 => flagvals::NUMCONSTSTR,
            131072 => flagvals::XARRAY,
            65536 => flagvals::HALFHAT,
            32768 => flagvals::ARRAYMAXED,
            16384 => flagvals::NULL_FIELD,
            8192 => flagvals::NO_EXT_SET,
            4096 => flagvals::MPZN,
            2048 => flagvals::MPFN,
            1024 => flagvals::WSTRCUR,
            512 => flagvals::INTIND,
            256 => flagvals::NUMINT,
            128 => flagvals::INTLSTR,
            64 => flagvals::BOOLVAL,
            32 => flagvals::USER_INPUT,
            16 => flagvals::NUMBER,
            8 => flagvals::NUMCUR,
            4 => flagvals::STRCUR,
            2 => flagvals::STRING,
            1 => flagvals::MALLOC,
            _ => panic!("Invalid value for flagvals: {}", value),
        }
    }
}
impl AddAssign<u32> for flagvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for flagvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for flagvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for flagvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for flagvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for flagvals {
    type Output = flagvals;
    fn add(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for flagvals {
    type Output = flagvals;
    fn sub(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for flagvals {
    type Output = flagvals;
    fn mul(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for flagvals {
    type Output = flagvals;
    fn div(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for flagvals {
    type Output = flagvals;
    fn rem(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub nodep: C2RustUnnamed_4,
    pub val: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fltnum: libc::c_double,
    pub sp: *mut i8,
    pub slen: size_t,
    pub idx: i32,
    pub wsp: *mut wchar_t,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum commenttype {
    FOR_COMMENT = 3,
    BLOCK_COMMENT = 2,
    EOL_COMMENT = 1,
}
impl commenttype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            commenttype::FOR_COMMENT => 3,
            commenttype::BLOCK_COMMENT => 2,
            commenttype::EOL_COMMENT => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> commenttype {
        match value {
            3 => commenttype::FOR_COMMENT,
            2 => commenttype::BLOCK_COMMENT,
            1 => commenttype::EOL_COMMENT,
            _ => panic!("Invalid value for commenttype: {}", value),
        }
    }
}
impl AddAssign<u32> for commenttype {
    fn add_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for commenttype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for commenttype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for commenttype {
    fn div_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for commenttype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for commenttype {
    type Output = commenttype;
    fn add(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for commenttype {
    type Output = commenttype;
    fn sub(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for commenttype {
    type Output = commenttype;
    fn mul(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for commenttype {
    type Output = commenttype;
    fn div(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for commenttype {
    type Output = commenttype;
    fn rem(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub l: C2RustUnnamed_11,
    pub r: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
    pub name: *mut i8,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: u64,
    pub reflags: reflagvals,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum reflagvals {
    CONSTANT = 1,
    FS_DFLT = 2,
}
impl reflagvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            reflagvals::CONSTANT => 1,
            reflagvals::FS_DFLT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> reflagvals {
        match value {
            1 => reflagvals::CONSTANT,
            2 => reflagvals::FS_DFLT,
            _ => panic!("Invalid value for reflagvals: {}", value),
        }
    }
}
impl AddAssign<u32> for reflagvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for reflagvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for reflagvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for reflagvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for reflagvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for reflagvals {
    type Output = reflagvals;
    fn add(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for reflagvals {
    type Output = reflagvals;
    fn sub(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for reflagvals {
    type Output = reflagvals;
    fn mul(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for reflagvals {
    type Output = reflagvals;
    fn div(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for reflagvals {
    type Output = reflagvals;
    fn rem(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2RustUnnamed_8,
    pub x: C2RustUnnamed_7,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum opcodeval {
    Op_final = 122,
    Op_parens = 121,
    Op_cond_exp = 120,
    Op_K_function = 119,
    Op_K_else = 118,
    Op_K_if = 117,
    Op_K_switch = 116,
    Op_K_while = 115,
    Op_K_arrayfor = 114,
    Op_K_for = 113,
    Op_K_do = 112,
    Op_list = 111,
    Op_symbol = 110,
    Op_token = 109,
    Op_stop = 108,
    Op_atexit = 107,
    Op_lint_plus = 106,
    Op_lint = 105,
    Op_breakpoint = 104,
    Op_exec_count = 103,
    Op_comment = 102,
    Op_func = 101,
    Op_after_endfile = 100,
    Op_after_beginfile = 99,
    Op_subscript_assign = 98,
    Op_field_assign = 97,
    Op_var_assign = 96,
    Op_var_update = 95,
    Op_arrayfor_final = 94,
    Op_arrayfor_incr = 93,
    Op_arrayfor_init = 92,
    Op_newfile = 91,
    Op_get_record = 90,
    Op_jmp_false = 89,
    Op_jmp_true = 88,
    Op_jmp = 87,
    Op_pop = 86,
    Op_no_op = 85,
    Op_field_spec_lhs = 84,
    Op_subscript_lhs = 83,
    Op_push_lhs = 82,
    Op_push_param = 81,
    Op_push_array = 80,
    Op_push_re = 79,
    Op_push_i = 78,
    Op_push_arg_untyped = 77,
    Op_push_arg = 76,
    Op_push = 75,
    Op_indirect_func_call = 74,
    Op_func_call = 73,
    Op_in_array = 72,
    Op_ext_builtin = 71,
    Op_sub_builtin = 70,
    Op_builtin = 69,
    Op_K_namespace = 68,
    Op_K_nextfile = 67,
    Op_K_getline = 66,
    Op_K_getline_redir = 65,
    Op_K_delete_loop = 64,
    Op_K_delete = 63,
    Op_K_return_from_eval = 62,
    Op_K_return = 61,
    Op_K_exit = 60,
    Op_K_next = 59,
    Op_K_printf = 58,
    Op_K_print_rec = 57,
    Op_K_print = 56,
    Op_K_continue = 55,
    Op_K_break = 54,
    Op_K_default = 53,
    Op_K_case = 52,
    Op_rule = 51,
    Op_nomatch = 50,
    Op_match_rec = 49,
    Op_match = 48,
    Op_geq = 47,
    Op_leq = 46,
    Op_greater = 45,
    Op_less = 44,
    Op_notequal = 43,
    Op_equal = 42,
    Op_or_final = 41,
    Op_or = 40,
    Op_and_final = 39,
    Op_and = 38,
    Op_assign_concat = 37,
    Op_assign_exp = 36,
    Op_assign_minus = 35,
    Op_assign_plus = 34,
    Op_assign_mod = 33,
    Op_assign_quotient = 32,
    Op_assign_times = 31,
    Op_store_field_exp = 30,
    Op_store_field = 29,
    Op_store_sub = 28,
    Op_store_var = 27,
    Op_assign = 26,
    Op_not = 25,
    Op_field_spec = 24,
    Op_unary_plus = 23,
    Op_unary_minus = 22,
    Op_postdecrement = 21,
    Op_postincrement = 20,
    Op_predecrement = 19,
    Op_preincrement = 18,
    Op_sub_array = 17,
    Op_subscript = 16,
    Op_cond_pair = 15,
    Op_line_range = 14,
    Op_concat = 13,
    Op_exp_i = 12,
    Op_exp = 11,
    Op_minus_i = 10,
    Op_minus = 9,
    Op_plus_i = 8,
    Op_plus = 7,
    Op_mod_i = 6,
    Op_mod = 5,
    Op_quotient_i = 4,
    Op_quotient = 3,
    Op_times_i = 2,
    Op_times = 1,
    Op_illegal = 0,
}
impl opcodeval {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            opcodeval::Op_final => 122,
            opcodeval::Op_parens => 121,
            opcodeval::Op_cond_exp => 120,
            opcodeval::Op_K_function => 119,
            opcodeval::Op_K_else => 118,
            opcodeval::Op_K_if => 117,
            opcodeval::Op_K_switch => 116,
            opcodeval::Op_K_while => 115,
            opcodeval::Op_K_arrayfor => 114,
            opcodeval::Op_K_for => 113,
            opcodeval::Op_K_do => 112,
            opcodeval::Op_list => 111,
            opcodeval::Op_symbol => 110,
            opcodeval::Op_token => 109,
            opcodeval::Op_stop => 108,
            opcodeval::Op_atexit => 107,
            opcodeval::Op_lint_plus => 106,
            opcodeval::Op_lint => 105,
            opcodeval::Op_breakpoint => 104,
            opcodeval::Op_exec_count => 103,
            opcodeval::Op_comment => 102,
            opcodeval::Op_func => 101,
            opcodeval::Op_after_endfile => 100,
            opcodeval::Op_after_beginfile => 99,
            opcodeval::Op_subscript_assign => 98,
            opcodeval::Op_field_assign => 97,
            opcodeval::Op_var_assign => 96,
            opcodeval::Op_var_update => 95,
            opcodeval::Op_arrayfor_final => 94,
            opcodeval::Op_arrayfor_incr => 93,
            opcodeval::Op_arrayfor_init => 92,
            opcodeval::Op_newfile => 91,
            opcodeval::Op_get_record => 90,
            opcodeval::Op_jmp_false => 89,
            opcodeval::Op_jmp_true => 88,
            opcodeval::Op_jmp => 87,
            opcodeval::Op_pop => 86,
            opcodeval::Op_no_op => 85,
            opcodeval::Op_field_spec_lhs => 84,
            opcodeval::Op_subscript_lhs => 83,
            opcodeval::Op_push_lhs => 82,
            opcodeval::Op_push_param => 81,
            opcodeval::Op_push_array => 80,
            opcodeval::Op_push_re => 79,
            opcodeval::Op_push_i => 78,
            opcodeval::Op_push_arg_untyped => 77,
            opcodeval::Op_push_arg => 76,
            opcodeval::Op_push => 75,
            opcodeval::Op_indirect_func_call => 74,
            opcodeval::Op_func_call => 73,
            opcodeval::Op_in_array => 72,
            opcodeval::Op_ext_builtin => 71,
            opcodeval::Op_sub_builtin => 70,
            opcodeval::Op_builtin => 69,
            opcodeval::Op_K_namespace => 68,
            opcodeval::Op_K_nextfile => 67,
            opcodeval::Op_K_getline => 66,
            opcodeval::Op_K_getline_redir => 65,
            opcodeval::Op_K_delete_loop => 64,
            opcodeval::Op_K_delete => 63,
            opcodeval::Op_K_return_from_eval => 62,
            opcodeval::Op_K_return => 61,
            opcodeval::Op_K_exit => 60,
            opcodeval::Op_K_next => 59,
            opcodeval::Op_K_printf => 58,
            opcodeval::Op_K_print_rec => 57,
            opcodeval::Op_K_print => 56,
            opcodeval::Op_K_continue => 55,
            opcodeval::Op_K_break => 54,
            opcodeval::Op_K_default => 53,
            opcodeval::Op_K_case => 52,
            opcodeval::Op_rule => 51,
            opcodeval::Op_nomatch => 50,
            opcodeval::Op_match_rec => 49,
            opcodeval::Op_match => 48,
            opcodeval::Op_geq => 47,
            opcodeval::Op_leq => 46,
            opcodeval::Op_greater => 45,
            opcodeval::Op_less => 44,
            opcodeval::Op_notequal => 43,
            opcodeval::Op_equal => 42,
            opcodeval::Op_or_final => 41,
            opcodeval::Op_or => 40,
            opcodeval::Op_and_final => 39,
            opcodeval::Op_and => 38,
            opcodeval::Op_assign_concat => 37,
            opcodeval::Op_assign_exp => 36,
            opcodeval::Op_assign_minus => 35,
            opcodeval::Op_assign_plus => 34,
            opcodeval::Op_assign_mod => 33,
            opcodeval::Op_assign_quotient => 32,
            opcodeval::Op_assign_times => 31,
            opcodeval::Op_store_field_exp => 30,
            opcodeval::Op_store_field => 29,
            opcodeval::Op_store_sub => 28,
            opcodeval::Op_store_var => 27,
            opcodeval::Op_assign => 26,
            opcodeval::Op_not => 25,
            opcodeval::Op_field_spec => 24,
            opcodeval::Op_unary_plus => 23,
            opcodeval::Op_unary_minus => 22,
            opcodeval::Op_postdecrement => 21,
            opcodeval::Op_postincrement => 20,
            opcodeval::Op_predecrement => 19,
            opcodeval::Op_preincrement => 18,
            opcodeval::Op_sub_array => 17,
            opcodeval::Op_subscript => 16,
            opcodeval::Op_cond_pair => 15,
            opcodeval::Op_line_range => 14,
            opcodeval::Op_concat => 13,
            opcodeval::Op_exp_i => 12,
            opcodeval::Op_exp => 11,
            opcodeval::Op_minus_i => 10,
            opcodeval::Op_minus => 9,
            opcodeval::Op_plus_i => 8,
            opcodeval::Op_plus => 7,
            opcodeval::Op_mod_i => 6,
            opcodeval::Op_mod => 5,
            opcodeval::Op_quotient_i => 4,
            opcodeval::Op_quotient => 3,
            opcodeval::Op_times_i => 2,
            opcodeval::Op_times => 1,
            opcodeval::Op_illegal => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> opcodeval {
        match value {
            122 => opcodeval::Op_final,
            121 => opcodeval::Op_parens,
            120 => opcodeval::Op_cond_exp,
            119 => opcodeval::Op_K_function,
            118 => opcodeval::Op_K_else,
            117 => opcodeval::Op_K_if,
            116 => opcodeval::Op_K_switch,
            115 => opcodeval::Op_K_while,
            114 => opcodeval::Op_K_arrayfor,
            113 => opcodeval::Op_K_for,
            112 => opcodeval::Op_K_do,
            111 => opcodeval::Op_list,
            110 => opcodeval::Op_symbol,
            109 => opcodeval::Op_token,
            108 => opcodeval::Op_stop,
            107 => opcodeval::Op_atexit,
            106 => opcodeval::Op_lint_plus,
            105 => opcodeval::Op_lint,
            104 => opcodeval::Op_breakpoint,
            103 => opcodeval::Op_exec_count,
            102 => opcodeval::Op_comment,
            101 => opcodeval::Op_func,
            100 => opcodeval::Op_after_endfile,
            99 => opcodeval::Op_after_beginfile,
            98 => opcodeval::Op_subscript_assign,
            97 => opcodeval::Op_field_assign,
            96 => opcodeval::Op_var_assign,
            95 => opcodeval::Op_var_update,
            94 => opcodeval::Op_arrayfor_final,
            93 => opcodeval::Op_arrayfor_incr,
            92 => opcodeval::Op_arrayfor_init,
            91 => opcodeval::Op_newfile,
            90 => opcodeval::Op_get_record,
            89 => opcodeval::Op_jmp_false,
            88 => opcodeval::Op_jmp_true,
            87 => opcodeval::Op_jmp,
            86 => opcodeval::Op_pop,
            85 => opcodeval::Op_no_op,
            84 => opcodeval::Op_field_spec_lhs,
            83 => opcodeval::Op_subscript_lhs,
            82 => opcodeval::Op_push_lhs,
            81 => opcodeval::Op_push_param,
            80 => opcodeval::Op_push_array,
            79 => opcodeval::Op_push_re,
            78 => opcodeval::Op_push_i,
            77 => opcodeval::Op_push_arg_untyped,
            76 => opcodeval::Op_push_arg,
            75 => opcodeval::Op_push,
            74 => opcodeval::Op_indirect_func_call,
            73 => opcodeval::Op_func_call,
            72 => opcodeval::Op_in_array,
            71 => opcodeval::Op_ext_builtin,
            70 => opcodeval::Op_sub_builtin,
            69 => opcodeval::Op_builtin,
            68 => opcodeval::Op_K_namespace,
            67 => opcodeval::Op_K_nextfile,
            66 => opcodeval::Op_K_getline,
            65 => opcodeval::Op_K_getline_redir,
            64 => opcodeval::Op_K_delete_loop,
            63 => opcodeval::Op_K_delete,
            62 => opcodeval::Op_K_return_from_eval,
            61 => opcodeval::Op_K_return,
            60 => opcodeval::Op_K_exit,
            59 => opcodeval::Op_K_next,
            58 => opcodeval::Op_K_printf,
            57 => opcodeval::Op_K_print_rec,
            56 => opcodeval::Op_K_print,
            55 => opcodeval::Op_K_continue,
            54 => opcodeval::Op_K_break,
            53 => opcodeval::Op_K_default,
            52 => opcodeval::Op_K_case,
            51 => opcodeval::Op_rule,
            50 => opcodeval::Op_nomatch,
            49 => opcodeval::Op_match_rec,
            48 => opcodeval::Op_match,
            47 => opcodeval::Op_geq,
            46 => opcodeval::Op_leq,
            45 => opcodeval::Op_greater,
            44 => opcodeval::Op_less,
            43 => opcodeval::Op_notequal,
            42 => opcodeval::Op_equal,
            41 => opcodeval::Op_or_final,
            40 => opcodeval::Op_or,
            39 => opcodeval::Op_and_final,
            38 => opcodeval::Op_and,
            37 => opcodeval::Op_assign_concat,
            36 => opcodeval::Op_assign_exp,
            35 => opcodeval::Op_assign_minus,
            34 => opcodeval::Op_assign_plus,
            33 => opcodeval::Op_assign_mod,
            32 => opcodeval::Op_assign_quotient,
            31 => opcodeval::Op_assign_times,
            30 => opcodeval::Op_store_field_exp,
            29 => opcodeval::Op_store_field,
            28 => opcodeval::Op_store_sub,
            27 => opcodeval::Op_store_var,
            26 => opcodeval::Op_assign,
            25 => opcodeval::Op_not,
            24 => opcodeval::Op_field_spec,
            23 => opcodeval::Op_unary_plus,
            22 => opcodeval::Op_unary_minus,
            21 => opcodeval::Op_postdecrement,
            20 => opcodeval::Op_postincrement,
            19 => opcodeval::Op_predecrement,
            18 => opcodeval::Op_preincrement,
            17 => opcodeval::Op_sub_array,
            16 => opcodeval::Op_subscript,
            15 => opcodeval::Op_cond_pair,
            14 => opcodeval::Op_line_range,
            13 => opcodeval::Op_concat,
            12 => opcodeval::Op_exp_i,
            11 => opcodeval::Op_exp,
            10 => opcodeval::Op_minus_i,
            9 => opcodeval::Op_minus,
            8 => opcodeval::Op_plus_i,
            7 => opcodeval::Op_plus,
            6 => opcodeval::Op_mod_i,
            5 => opcodeval::Op_mod,
            4 => opcodeval::Op_quotient_i,
            3 => opcodeval::Op_quotient,
            2 => opcodeval::Op_times_i,
            1 => opcodeval::Op_times,
            0 => opcodeval::Op_illegal,
            _ => panic!("Invalid value for opcodeval: {}", value),
        }
    }
}
impl AddAssign<u32> for opcodeval {
    fn add_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for opcodeval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for opcodeval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for opcodeval {
    fn div_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for opcodeval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for opcodeval {
    type Output = opcodeval;
    fn add(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for opcodeval {
    type Output = opcodeval;
    fn sub(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for opcodeval {
    type Output = opcodeval;
    fn mul(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for opcodeval {
    type Output = opcodeval;
    fn div(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for opcodeval {
    type Output = opcodeval;
    fn rem(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub xl: i64,
    pub xn: *mut NODE,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
    pub efptr: Option<
        unsafe extern "C" fn(
            i32,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: i64,
    pub ldl: exec_count_t,
    pub name: *mut i8,
}
pub type exec_count_t = libc::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2RustUnnamed_10,
    pub hi: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: i64,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const i8,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub public: awk_input_buf_t,
    pub buf: *mut i8,
    pub off: *mut i8,
    pub dataend: *mut i8,
    pub end: *mut i8,
    pub readsize: size_t,
    pub size: size_t,
    pub count: ssize_t,
    pub scanoff: size_t,
    pub valid: bool,
    pub errcode: i32,
    pub flag: iobuf_flags,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum iobuf_flags {
    IOP_IS_TTY = 1,
    IOP_AT_EOF = 2,
    IOP_CLOSED = 4,
    IOP_AT_START = 8,
}
impl iobuf_flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            iobuf_flags::IOP_IS_TTY => 1,
            iobuf_flags::IOP_AT_EOF => 2,
            iobuf_flags::IOP_CLOSED => 4,
            iobuf_flags::IOP_AT_START => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> iobuf_flags {
        match value {
            1 => iobuf_flags::IOP_IS_TTY,
            2 => iobuf_flags::IOP_AT_EOF,
            4 => iobuf_flags::IOP_CLOSED,
            8 => iobuf_flags::IOP_AT_START,
            _ => panic!("Invalid value for iobuf_flags: {}", value),
        }
    }
}
impl AddAssign<u32> for iobuf_flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for iobuf_flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for iobuf_flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for iobuf_flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for iobuf_flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn add(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn sub(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn mul(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn div(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn rem(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type IOBUF = iobuf;
pub type Func_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub flag: redirect_flags,
    pub value: *mut i8,
    pub ifp: *mut FILE,
    pub iop: *mut IOBUF,
    pub pid: i32,
    pub status: i32,
    pub prev: *mut redirect,
    pub next: *mut redirect,
    pub mode: *const i8,
    pub output: awk_output_buf_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redirect_flags {
    RED_TCP = 2048,
    RED_SOCKET = 1024,
    RED_PTY = 512,
    RED_TWOWAY = 256,
    RED_EOF = 128,
    RED_USED = 64,
    RED_FLUSH = 32,
    RED_APPEND = 16,
    RED_WRITE = 8,
    RED_READ = 4,
    RED_PIPE = 2,
    RED_FILE = 1,
    RED_NONE = 0,
}
impl redirect_flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            redirect_flags::RED_TCP => 2048,
            redirect_flags::RED_SOCKET => 1024,
            redirect_flags::RED_PTY => 512,
            redirect_flags::RED_TWOWAY => 256,
            redirect_flags::RED_EOF => 128,
            redirect_flags::RED_USED => 64,
            redirect_flags::RED_FLUSH => 32,
            redirect_flags::RED_APPEND => 16,
            redirect_flags::RED_WRITE => 8,
            redirect_flags::RED_READ => 4,
            redirect_flags::RED_PIPE => 2,
            redirect_flags::RED_FILE => 1,
            redirect_flags::RED_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> redirect_flags {
        match value {
            2048 => redirect_flags::RED_TCP,
            1024 => redirect_flags::RED_SOCKET,
            512 => redirect_flags::RED_PTY,
            256 => redirect_flags::RED_TWOWAY,
            128 => redirect_flags::RED_EOF,
            64 => redirect_flags::RED_USED,
            32 => redirect_flags::RED_FLUSH,
            16 => redirect_flags::RED_APPEND,
            8 => redirect_flags::RED_WRITE,
            4 => redirect_flags::RED_READ,
            2 => redirect_flags::RED_PIPE,
            1 => redirect_flags::RED_FILE,
            0 => redirect_flags::RED_NONE,
            _ => panic!("Invalid value for redirect_flags: {}", value),
        }
    }
}
impl AddAssign<u32> for redirect_flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redirect_flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redirect_flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redirect_flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redirect_flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redirect_flags {
    type Output = redirect_flags;
    fn add(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redirect_flags {
    type Output = redirect_flags;
    fn sub(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redirect_flags {
    type Output = redirect_flags;
    fn mul(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redirect_flags {
    type Output = redirect_flags;
    fn div(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redirect_flags {
    type Output = redirect_flags;
    fn rem(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum binmode_values {
    TEXT_TRANSLATE = 0,
    BINMODE_INPUT = 1,
    BINMODE_OUTPUT = 2,
    BINMODE_BOTH = 3,
}
impl binmode_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            binmode_values::TEXT_TRANSLATE => 0,
            binmode_values::BINMODE_INPUT => 1,
            binmode_values::BINMODE_OUTPUT => 2,
            binmode_values::BINMODE_BOTH => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> binmode_values {
        match value {
            0 => binmode_values::TEXT_TRANSLATE,
            1 => binmode_values::BINMODE_INPUT,
            2 => binmode_values::BINMODE_OUTPUT,
            3 => binmode_values::BINMODE_BOTH,
            _ => panic!("Invalid value for binmode_values: {}", value),
        }
    }
}
impl AddAssign<u32> for binmode_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for binmode_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for binmode_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for binmode_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for binmode_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for binmode_values {
    type Output = binmode_values;
    fn add(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for binmode_values {
    type Output = binmode_values;
    fn sub(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for binmode_values {
    type Output = binmode_values;
    fn mul(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for binmode_values {
    type Output = binmode_values;
    fn div(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for binmode_values {
    type Output = binmode_values;
    fn rem(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_item {
    pub freep: *mut block_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_header {
    pub freep: *mut block_item,
    pub size: size_t,
    pub name: *const i8,
    pub highwater: i64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum block_id {
    BLOCK_NODE = 0,
    BLOCK_BUCKET,
    BLOCK_MAX,
}
impl block_id {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            block_id::BLOCK_NODE => 0,
            block_id::BLOCK_BUCKET => 1,
            block_id::BLOCK_MAX => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> block_id {
        match value {
            0 => block_id::BLOCK_NODE,
            1 => block_id::BLOCK_BUCKET,
            2 => block_id::BLOCK_MAX,
            _ => panic!("Invalid value for block_id: {}", value),
        }
    }
}
impl AddAssign<u32> for block_id {
    fn add_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for block_id {
    fn sub_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for block_id {
    fn mul_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for block_id {
    fn div_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for block_id {
    fn rem_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for block_id {
    type Output = block_id;
    fn add(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for block_id {
    type Output = block_id;
    fn sub(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for block_id {
    type Output = block_id;
    fn mul(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for block_id {
    type Output = block_id;
    fn div(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for block_id {
    type Output = block_id;
    fn rem(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum do_flag_values {
    DO_MPFR = 32768,
    DO_DEBUG = 16384,
    DO_PROFILE = 8192,
    DO_SANDBOX = 4096,
    DO_TIDY_MEM = 2048,
    DO_DUMP_VARS = 1024,
    DO_PRETTY_PRINT = 512,
    DO_INTERVALS = 256,
    DO_NON_DEC_DATA = 128,
    DO_INTL = 64,
    DO_POSIX = 32,
    DO_TRADITIONAL = 16,
    DO_LINT_OLD = 8,
    DO_LINT_ALL = 4,
    DO_LINT_EXTENSIONS = 2,
    DO_LINT_INVALID = 1,
    DO_FLAG_NONE = 0,
}
impl do_flag_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            do_flag_values::DO_MPFR => 32768,
            do_flag_values::DO_DEBUG => 16384,
            do_flag_values::DO_PROFILE => 8192,
            do_flag_values::DO_SANDBOX => 4096,
            do_flag_values::DO_TIDY_MEM => 2048,
            do_flag_values::DO_DUMP_VARS => 1024,
            do_flag_values::DO_PRETTY_PRINT => 512,
            do_flag_values::DO_INTERVALS => 256,
            do_flag_values::DO_NON_DEC_DATA => 128,
            do_flag_values::DO_INTL => 64,
            do_flag_values::DO_POSIX => 32,
            do_flag_values::DO_TRADITIONAL => 16,
            do_flag_values::DO_LINT_OLD => 8,
            do_flag_values::DO_LINT_ALL => 4,
            do_flag_values::DO_LINT_EXTENSIONS => 2,
            do_flag_values::DO_LINT_INVALID => 1,
            do_flag_values::DO_FLAG_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> do_flag_values {
        match value {
            32768 => do_flag_values::DO_MPFR,
            16384 => do_flag_values::DO_DEBUG,
            8192 => do_flag_values::DO_PROFILE,
            4096 => do_flag_values::DO_SANDBOX,
            2048 => do_flag_values::DO_TIDY_MEM,
            1024 => do_flag_values::DO_DUMP_VARS,
            512 => do_flag_values::DO_PRETTY_PRINT,
            256 => do_flag_values::DO_INTERVALS,
            128 => do_flag_values::DO_NON_DEC_DATA,
            64 => do_flag_values::DO_INTL,
            32 => do_flag_values::DO_POSIX,
            16 => do_flag_values::DO_TRADITIONAL,
            8 => do_flag_values::DO_LINT_OLD,
            4 => do_flag_values::DO_LINT_ALL,
            2 => do_flag_values::DO_LINT_EXTENSIONS,
            1 => do_flag_values::DO_LINT_INVALID,
            0 => do_flag_values::DO_FLAG_NONE,
            _ => panic!("Invalid value for do_flag_values: {}", value),
        }
    }
}
impl AddAssign<u32> for do_flag_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for do_flag_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for do_flag_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for do_flag_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for do_flag_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for do_flag_values {
    type Output = do_flag_values;
    fn add(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for do_flag_values {
    type Output = do_flag_values;
    fn sub(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for do_flag_values {
    type Output = do_flag_values;
    fn mul(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for do_flag_values {
    type Output = do_flag_values;
    fn div(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for do_flag_values {
    type Output = do_flag_values;
    fn rem(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub buf: *mut i8,
    pub bufsize: size_t,
    pub stackbuf: [i8; 30],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_13 {
    MP_FLOAT = 3,
    MP_INT_WITHOUT_PREC = 2,
    MP_INT_WITH_PREC = 1,
    MP_NONE = 0,
}
impl C2RustUnnamed_13 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_13::MP_FLOAT => 3,
            C2RustUnnamed_13::MP_INT_WITHOUT_PREC => 2,
            C2RustUnnamed_13::MP_INT_WITH_PREC => 1,
            C2RustUnnamed_13::MP_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_13 {
        match value {
            3 => C2RustUnnamed_13::MP_FLOAT,
            2 => C2RustUnnamed_13::MP_INT_WITHOUT_PREC,
            1 => C2RustUnnamed_13::MP_INT_WITH_PREC,
            0 => C2RustUnnamed_13::MP_NONE,
            _ => panic!("Invalid value for C2RustUnnamed_13: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_13 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_13 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_13 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_13 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_13 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn add(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn sub(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn mul(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn div(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn rem(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum two_way_close_type {
    CLOSE_ALL,
    CLOSE_TO,
    CLOSE_FROM,
}
impl two_way_close_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            two_way_close_type::CLOSE_ALL => 0,
            two_way_close_type::CLOSE_TO => 1,
            two_way_close_type::CLOSE_FROM => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> two_way_close_type {
        match value {
            0 => two_way_close_type::CLOSE_ALL,
            1 => two_way_close_type::CLOSE_TO,
            2 => two_way_close_type::CLOSE_FROM,
            _ => panic!("Invalid value for two_way_close_type: {}", value),
        }
    }
}
impl AddAssign<u32> for two_way_close_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for two_way_close_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for two_way_close_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for two_way_close_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for two_way_close_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn add(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn sub(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn mul(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn div(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn rem(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type gawk_uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct category_table {
    pub val: i32,
    pub name: *const i8,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const i8,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as i32 as i64 {
        return;
    }
    r_unref(r);
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & (flagvals::NUMCUR as i32 | flagvals::USER_INPUT as i32) as u32
        == flagvals::USER_INPUT as i32 as u32
    {
        return force_number(n);
    }
    if (*n).flags as u32 & flagvals::INTIND as i32 as u32 != 0 as i32 as u32 {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const i8,
    mut fmtidx: i32,
) -> *mut NODE {
    if (*s).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        (*s).type_0 = nodevals::Node_val;
        (*s).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*s).flags as u32 & !(flagvals::NUMBER as i32) as u32);
        return s;
    }
    if (*s).flags as u32 & flagvals::STRCUR as i32 as u32 != 0 as i32 as u32
        && ((*s).sub.val.idx == -(1 as i32) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.expect("non-null function pointer")(fmtstr, fmtidx, s);
}
#[inline]
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh0 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh0).rptr;
    if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 1881 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        t = elem_new_to_scalar(t);
    }
    return t;
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn boolval(mut t: *mut NODE) -> bool {
    if (*t).type_0 as u32 == nodevals::Node_var as i32 as u32 {
        t = (*t).sub.nodep.l.lptr;
    }
    fixtype(t);
    if (*t).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32 {
        return !((*t).sub.val.fltnum == 0.0f64);
    }
    return (*t).sub.val.slen > 0 as i32 as u64;
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2088 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2092 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2052 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2056 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn str_terminate_f(mut n: *mut NODE, mut savep: *mut i8) {
    *savep = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as i8;
}
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as i32 as i64
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn assoc_set(
    mut array: *mut NODE,
    mut sub: *mut NODE,
    mut value: *mut NODE,
) {
    let mut lhs: *mut *mut NODE = ((*(*array).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(array, sub);
    unref(*lhs);
    *lhs = value;
    if ((*(*array).sub.nodep.l.lp).store).is_some() {
        (Some(((*(*array).sub.nodep.l.lp).store).expect("non-null function pointer")))
            .expect("non-null function pointer")(array, sub);
    }
    unref(sub);
}
#[inline]
unsafe extern "C" fn POP_PARAM() -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    return if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        t
    } else {
        force_array(t, 0 as i32 != 0)
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_exact_args(
    mut nargs: i32,
    mut fname: *const i8,
    mut count: i32,
) {
    if nargs != count {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 98 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: called with %d arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
            nargs,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_args_min_max(
    mut nargs: i32,
    mut fname: *const i8,
    mut min: i32,
    mut max: i32,
) {
    if nargs < min || nargs > max {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 105 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: called with %d arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
            nargs,
        );
    }
}
unsafe extern "C" fn wrerror(
    mut fp: *mut FILE,
    mut from: *const i8,
    mut rp: *mut redirect,
) {
    os_maybe_set_errno();
    if fp == stdout && *__errno_location() == 32 as i32 {
        signal(13 as i32, None);
        kill(getpid(), 13 as i32);
    }
    if if !rp.is_null() {
        is_non_fatal_redirect((*rp).value, strlen((*rp).value)) as i32
    } else {
        is_non_fatal_std(fp) as i32
    } != 0
    {
        update_ERRNO_int(*__errno_location());
    } else {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 130 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s to \"%s\" failed: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            from,
            if !rp.is_null() {
                (*rp).value
            } else if fp == stdout {
                dcgettext(
                    0 as *const i8,
                    b"standard output\0" as *const u8 as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"standard error\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
            if *__errno_location() != 0 {
                strerror(*__errno_location())
            } else {
                dcgettext(
                    0 as *const i8,
                    b"reason unknown\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn efflush(
    mut fp: *mut FILE,
    mut from: *const i8,
    mut rp: *mut redirect,
) {
    *__errno_location() = 0 as i32;
    if !rp.is_null() {
        ((*rp).output.gawk_fflush)
            .expect("non-null function pointer")(fp, (*rp).output.opaque);
        if ((*rp).output.gawk_ferror)
            .expect("non-null function pointer")(fp, (*rp).output.opaque) != 0
        {
            wrerror(fp, from, rp);
        }
    } else {
        fflush(fp);
        if ferror(fp) != 0 {
            wrerror(fp, from, rp);
        }
    };
}
unsafe extern "C" fn efwrite(
    mut ptr: *const libc::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut FILE,
    mut from: *const i8,
    mut rp: *mut redirect,
    mut flush: bool,
) {
    *__errno_location() = 0 as i32;
    if !rp.is_null() {
        let mut err_on_write: bool = ((*rp).output.gawk_fwrite)
            .expect(
                "non-null function pointer",
            )(ptr, size, count, fp, (*rp).output.opaque) != count;
        let mut err_on_fp: bool = (*rp).output.gawk_fwrite
            == Some(
                gawk_fwrite
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        size_t,
                        *mut FILE,
                        *mut libc::c_void,
                    ) -> size_t,
            ) && ferror(fp) != 0;
        if err_on_write as i32 != 0 || err_on_fp as i32 != 0 {
            wrerror(fp, from, rp);
            return;
        }
    } else if (if 0 != 0 && 0 != 0 && size.wrapping_mul(count) <= 8 as i32 as u64
        && size != 0 as i32 as u64
    {
        ({
            let mut __ptr: *const i8 = ptr as *const i8;
            let mut __stream: *mut FILE = fp;
            let mut __cnt: size_t = 0;
            __cnt = size.wrapping_mul(count);
            while __cnt > 0 as i32 as u64 {
                if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32
                    as i64 != 0
                {
                    let fresh2 = __ptr;
                    __ptr = __ptr.offset(1);
                    __overflow(__stream, *fresh2 as u8 as i32)
                } else {
                    let fresh3 = __ptr;
                    __ptr = __ptr.offset(1);
                    let fresh4 = (*__stream)._IO_write_ptr;
                    (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
                    *fresh4 = *fresh3;
                    *fresh4 as u8 as i32
                }) == -(1 as i32)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            size.wrapping_mul(count).wrapping_sub(__cnt).wrapping_div(size)
        })
    } else {
        (if 0 != 0 && size == 0 as i32 as u64 || 0 != 0 && count == 0 as i32 as u64 {
            0 as i32 as size_t
        } else {
            fwrite_unlocked(ptr, size, count, fp)
        })
    }) != count || ferror(fp) != 0
    {
        wrerror(fp, from, rp);
        return;
    }
    if flush as i32 != 0
        && (fp == stdout && output_is_tty as i32 != 0
            || !rp.is_null()
                && (*rp).flag as u32 & redirect_flags::RED_FLUSH as i32 as u32
                    != 0 as i32 as u32)
    {
        efflush(fp, from, rp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_exp(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    let mut res: libc::c_double = 0.;
    check_exact_args(nargs, b"exp\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 211 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"exp\0" as *const u8 as *const i8,
        );
    }
    d = (*force_number(tmp)).sub.val.fltnum;
    DEREF(tmp);
    *__errno_location() = 0 as i32;
    res = exp(d);
    if *__errno_location() == 34 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 217 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"exp: argument %g is out of range\0" as *const u8 as *const i8,
                5 as i32,
            ),
            d,
        );
    }
    return make_number.expect("non-null function pointer")(res);
}
unsafe extern "C" fn stdfile(mut name: *const i8, mut len: size_t) -> *mut FILE {
    if len == 11 as i32 as u64 {
        if strncmp(name, b"/dev/stderr\0" as *const u8 as *const i8, 11 as i32 as u64)
            == 0 as i32
        {
            return stderr
        } else if strncmp(
            name,
            b"/dev/stdout\0" as *const u8 as *const i8,
            11 as i32 as u64,
        ) == 0 as i32
        {
            return stdout
        }
    }
    return 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn do_fflush(mut nargs: i32) -> *mut NODE {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut status: i32 = 0 as i32;
    let mut file: *const i8 = 0 as *const i8;
    let mut len: i32 = 0;
    check_args_min_max(nargs, b"fflush\0" as *const u8 as *const i8, 0 as i32, 1 as i32);
    if nargs == 0 as i32 {
        status = flush_io();
        return make_number.expect("non-null function pointer")(status as libc::c_double);
    }
    tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 281 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"fflush\0" as *const u8 as *const i8,
        );
    }
    file = (*tmp).sub.val.sp;
    len = (*tmp).sub.val.slen as i32;
    if (*tmp).sub.val.slen == 0 as i32 as u64 {
        status = flush_io();
        DEREF(tmp);
        return make_number.expect("non-null function pointer")(status as libc::c_double);
    }
    rp = getredirect((*tmp).sub.val.sp, (*tmp).sub.val.slen as i32);
    status = -(1 as i32);
    if !rp.is_null() {
        if (*rp).flag as u32
            & (redirect_flags::RED_WRITE as i32 | redirect_flags::RED_APPEND as i32)
                as u32 == 0 as i32 as u32
        {
            if (*rp).flag as u32 & redirect_flags::RED_PIPE as i32 as u32
                != 0 as i32 as u32
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 298 as i32);
                (Some(
                    (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"fflush: cannot flush: pipe `%.*s' opened for reading, not writing\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    len,
                    file,
                );
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 301 as i32);
                (Some(
                    (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"fflush: cannot flush: file `%.*s' opened for reading, not writing\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    len,
                    file,
                );
            }
            DEREF(tmp);
            return make_number
                .expect("non-null function pointer")(status as libc::c_double);
        }
        fp = (*rp).output.fp;
        if !fp.is_null() {
            status = ((*rp).output.gawk_fflush)
                .expect("non-null function pointer")(fp, (*rp).output.opaque);
            if status != 0 as i32 {
                if !is_non_fatal_redirect((*tmp).sub.val.sp, (*tmp).sub.val.slen) {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 312 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"fflush: cannot flush file `%.*s': %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        len,
                        file,
                        strerror(*__errno_location()),
                    );
                }
                update_ERRNO_int(*__errno_location());
            }
        } else if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
            != 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 317 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"fflush: cannot flush: two-way pipe `%.*s' has closed write end\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                len,
                file,
            );
        }
    } else {
        fp = stdfile((*tmp).sub.val.sp, (*tmp).sub.val.slen);
        if !fp.is_null() {
            status = (non_fatal_flush_std_file(fp) as i32 == 0 as i32) as i32;
        } else {
            status = -(1 as i32);
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 323 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"fflush: `%.*s' is not an open file, pipe or co-process\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                len,
                file,
            );
        }
    }
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(status as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn strncasecmpmbs(
    mut s1: *const u8,
    mut s2: *const u8,
    mut n: size_t,
) -> i32 {
    let mut i1: size_t = 0;
    let mut i2: size_t = 0;
    let mut mbclen1: size_t = 0;
    let mut mbclen2: size_t = 0;
    let mut gap: size_t = 0;
    let mut wc1: wchar_t = 0;
    let mut wc2: wchar_t = 0;
    let mut mbs1: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut mbs2: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs1 as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    memset(
        &mut mbs2 as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    i2 = 0 as i32 as size_t;
    i1 = i2;
    while i1 < n && i2 < n {
        if *btowc_cache
            .as_mut_ptr()
            .offset((*s1.offset(i1 as isize) as i32 & 0xff as i32) as isize)
            != 0xffffffff as u32
        {
            mbclen1 = 1 as i32 as size_t;
            wc1 = *btowc_cache
                .as_mut_ptr()
                .offset((*s1.offset(i1 as isize) as i32 & 0xff as i32) as isize)
                as wchar_t;
        } else {
            mbclen1 = mbrtowc(
                &mut wc1,
                (s1 as *const i8).offset(i1 as isize),
                n.wrapping_sub(i1),
                &mut mbs1,
            );
            if mbclen1 == -(1 as i32) as size_t || mbclen1 == -(2 as i32) as size_t
                || mbclen1 == 0 as i32 as u64
            {
                mbclen1 = 1 as i32 as size_t;
                wc1 = *btowc_cache
                    .as_mut_ptr()
                    .offset((*s1.offset(i1 as isize) as i32 & 0xff as i32) as isize)
                    as wchar_t;
            }
        }
        if *btowc_cache
            .as_mut_ptr()
            .offset((*s2.offset(i2 as isize) as i32 & 0xff as i32) as isize)
            != 0xffffffff as u32
        {
            mbclen2 = 1 as i32 as size_t;
            wc2 = *btowc_cache
                .as_mut_ptr()
                .offset((*s2.offset(i2 as isize) as i32 & 0xff as i32) as isize)
                as wchar_t;
        } else {
            mbclen2 = mbrtowc(
                &mut wc2,
                (s2 as *const i8).offset(i2 as isize),
                n.wrapping_sub(i2),
                &mut mbs2,
            );
            if mbclen2 == -(1 as i32) as size_t || mbclen2 == -(2 as i32) as size_t
                || mbclen2 == 0 as i32 as u64
            {
                mbclen2 = 1 as i32 as size_t;
                wc2 = *btowc_cache
                    .as_mut_ptr()
                    .offset((*s2.offset(i2 as isize) as i32 & 0xff as i32) as isize)
                    as wchar_t;
            }
        }
        gap = (towlower(wc1 as wint_t)).wrapping_sub(towlower(wc2 as wint_t)) as size_t;
        if gap != 0 as i32 as u64 {
            return gap as i32;
        }
        i1 = (i1 as u64).wrapping_add(mbclen1) as size_t as size_t;
        i2 = (i2 as u64).wrapping_add(mbclen2) as size_t as size_t;
    }
    return 0 as i32;
}
unsafe extern "C" fn index_multibyte_buffer(
    mut src: *mut i8,
    mut dest: *mut i8,
    mut len: i32,
) {
    let mut idx: i32 = 0;
    let mut prev_idx: i32 = 0;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut prevs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut prevs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    prev_idx = 0 as i32;
    idx = prev_idx;
    while idx < len {
        let mut mbclen: size_t = 0;
        mbs = prevs;
        mbclen = mbrlen(
            src.offset(prev_idx as isize),
            (idx - prev_idx + 1 as i32) as size_t,
            &mut mbs,
        );
        if mbclen == -(1 as i32) as size_t || mbclen == 1 as i32 as u64
            || mbclen == 0 as i32 as u64
        {
            mbclen = 1 as i32 as size_t;
            prev_idx = idx + 1 as i32;
        } else if mbclen == -(2 as i32) as size_t {
            mbclen = (idx - prev_idx + 1 as i32) as size_t;
        } else if mbclen > 1 as i32 as u64 {
            prev_idx = idx + 1 as i32;
            prevs = mbs;
        }
        *dest.offset(idx as isize) = mbclen as i8;
        idx += 1;
        idx;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_index(mut nargs: i32) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut p1: *const i8 = 0 as *const i8;
    let mut p2: *const i8 = 0 as *const i8;
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    let mut ret: i64 = 0;
    let mut do_single_byte: bool = 0 as i32 != 0;
    let mut mbs1: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut mbs2: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    check_exact_args(nargs, b"index\0" as *const u8 as *const i8, 2 as i32);
    if gawk_mb_cur_max > 1 as i32 {
        memset(
            &mut mbs1 as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        memset(
            &mut mbs2 as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    }
    s2 = POP_SCALAR();
    let fresh5 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    s1 = (*fresh5).rptr;
    if (*s1).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        DEREF(s2);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 428 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(s1),
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if (*fixtype(s1)).flags as u32
            & (flagvals::STRING as i32 | flagvals::USER_INPUT as i32) as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 432 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string first argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"index\0" as *const u8 as *const i8,
            );
        }
        if (*fixtype(s2)).flags as u32
            & (flagvals::STRING as i32 | flagvals::USER_INPUT as i32) as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 434 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string second argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"index\0" as *const u8 as *const i8,
            );
        }
    }
    s1 = force_string_fmt(s1, CONVFMT, CONVFMTidx);
    s2 = force_string_fmt(s2, CONVFMT, CONVFMTidx);
    p1 = (*s1).sub.val.sp;
    p2 = (*s2).sub.val.sp;
    l1 = (*s1).sub.val.slen;
    l2 = (*s2).sub.val.slen;
    ret = 0 as i32 as i64;
    if l2 == 0 as i32 as u64 {
        ret = 1 as i32 as i64;
    } else {
        if gawk_mb_cur_max > 1 as i32 {
            s1 = str2wstr(s1, 0 as *mut *mut size_t);
            s2 = str2wstr(s2, 0 as *mut *mut size_t);
            do_single_byte = (*s1).sub.val.wslen == 0 as i32 as u64
                && (*s1).sub.val.slen > 0 as i32 as u64
                || (*s2).sub.val.wslen == 0 as i32 as u64
                    && (*s2).sub.val.slen > 0 as i32 as u64;
        }
        if IGNORECASE {
            while l1 > 0 as i32 as u64 {
                if l2 > l1 {
                    break;
                }
                if !do_single_byte && gawk_mb_cur_max > 1 as i32 {
                    let mut pos: *const wchar_t = 0 as *const wchar_t;
                    pos = wcasestrstr(
                        (*s1).sub.val.wsp,
                        (*s1).sub.val.wslen,
                        (*s2).sub.val.wsp,
                        (*s2).sub.val.wslen,
                    );
                    if pos.is_null() {
                        ret = 0 as i32 as i64;
                    } else {
                        ret = pos.offset_from((*s1).sub.val.wsp) as i64
                            + 1 as i32 as i64;
                    }
                    break;
                } else if *casetable.as_mut_ptr().offset(*p1 as u8 as isize) as i32
                    == *casetable.as_mut_ptr().offset(*p2 as u8 as isize) as i32
                    && (l2 == 1 as i32 as u64 || strncasecmp(p1, p2, l2) == 0 as i32)
                {
                    ret = (1 as i32 as u64)
                        .wrapping_add((*s1).sub.val.slen)
                        .wrapping_sub(l1) as i64;
                    break;
                } else {
                    l1 = l1.wrapping_sub(1);
                    l1;
                    p1 = p1.offset(1);
                    p1;
                }
            }
        } else {
            while l1 > 0 as i32 as u64 {
                if l2 > l1 {
                    break;
                }
                if *p1 as i32 == *p2 as i32
                    && (l2 == 1 as i32 as u64
                        || l2 > 0 as i32 as u64
                            && memcmp(
                                p1 as *const libc::c_void,
                                p2 as *const libc::c_void,
                                l2,
                            ) == 0 as i32)
                {
                    ret = (1 as i32 as u64)
                        .wrapping_add((*s1).sub.val.slen)
                        .wrapping_sub(l1) as i64;
                    break;
                } else if !do_single_byte && gawk_mb_cur_max > 1 as i32 {
                    let mut pos_0: *const wchar_t = 0 as *const wchar_t;
                    pos_0 = wstrstr(
                        (*s1).sub.val.wsp,
                        (*s1).sub.val.wslen,
                        (*s2).sub.val.wsp,
                        (*s2).sub.val.wslen,
                    );
                    if pos_0.is_null() {
                        ret = 0 as i32 as i64;
                    } else {
                        ret = pos_0.offset_from((*s1).sub.val.wsp) as i64
                            + 1 as i32 as i64;
                    }
                    break;
                } else {
                    l1 = l1.wrapping_sub(1);
                    l1;
                    p1 = p1.offset(1);
                    p1;
                }
            }
        }
    }
    DEREF(s1);
    DEREF(s2);
    return make_number.expect("non-null function pointer")(ret as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn double_to_int(mut d: libc::c_double) -> libc::c_double {
    if d >= 0 as i32 as libc::c_double {
        d = floor(d);
    } else {
        d = ceil(d);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn do_int(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(nargs, b"int\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 549 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"int\0" as *const u8 as *const i8,
        );
    }
    d = (*force_number(tmp)).sub.val.fltnum;
    d = double_to_int(d);
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(d);
}
#[no_mangle]
pub unsafe extern "C" fn do_isarray(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ret: i32 = 1 as i32;
    check_exact_args(nargs, b"isarray\0" as *const u8 as *const i8, 1 as i32);
    let fresh6 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    tmp = (*fresh6).rptr;
    if (*tmp).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
        ret = 0 as i32;
        if (*tmp).type_0 as u32 == nodevals::Node_val as i32 as u32 {
            DEREF(tmp);
        }
    }
    return make_number.expect("non-null function pointer")(ret as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_length(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut len: size_t = 0;
    check_exact_args(nargs, b"length\0" as *const u8 as *const i8, 1 as i32);
    let fresh7 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    tmp = (*fresh7).rptr;
    if (*tmp).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        static mut warned: bool = 0 as i32 != 0;
        let mut size: u64 = 0;
        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 592 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"length: received array argument\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
            && !warned
        {
            warned = 1 as i32 != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 595 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"`length(array)' is a gawk extension\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        size = (*tmp).sub.nodep.reflags as u64;
        return make_number.expect("non-null function pointer")(size as libc::c_double);
    } else if (*tmp).type_0 as u32 == nodevals::Node_var_new as i32 as u32
        || (*tmp).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
    {
        DEREF(tmp);
        tmp = dupnode(Nnull_string);
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32
            & (flagvals::STRING as i32 | flagvals::USER_INPUT as i32) as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 618 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"length\0" as *const u8 as *const i8,
        );
    }
    tmp = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
    if gawk_mb_cur_max > 1 as i32 {
        tmp = str2wstr(tmp, 0 as *mut *mut size_t);
        len = (*tmp).sub.val.wslen;
        if len == 0 as i32 as u64 && (*tmp).sub.val.slen > 0 as i32 as u64 {
            len = (*tmp).sub.val.slen;
        }
    } else {
        len = (*tmp).sub.val.slen;
    }
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(len as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_log(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    let mut arg: libc::c_double = 0.;
    check_exact_args(nargs, b"log\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 649 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"log\0" as *const u8 as *const i8,
        );
    }
    arg = (*force_number(tmp)).sub.val.fltnum;
    if arg < 0.0f64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 652 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received negative argument %g\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"log\0" as *const u8 as *const i8,
            arg,
        );
    }
    d = log(arg);
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(d);
}
#[no_mangle]
pub unsafe extern "C" fn format_tree(
    mut fmt_string: *const i8,
    mut n0: size_t,
    mut the_args: *mut *mut NODE,
    mut num_args: i64,
) -> *mut NODE {
    let mut need_to_add_thousands: bool = false;
    let mut current_block: u64;
    let mut cur_arg: size_t = 0 as i32 as size_t;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    let mut nc: i32 = 0;
    let mut toofew: bool = 0 as i32 != 0;
    let mut obuf: *mut i8 = 0 as *mut i8;
    let mut obufout: *mut i8 = 0 as *mut i8;
    let mut osiz: size_t = 0;
    let mut ofre: size_t = 0;
    let mut olen_final: size_t = 0;
    let mut chbuf: *const i8 = 0 as *const i8;
    let mut s0: *const i8 = 0 as *const i8;
    let mut s1: *const i8 = 0 as *const i8;
    let mut cs1: i32 = 0;
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut fw: i64 = 0;
    let mut prec: i64 = 0;
    let mut argnum: i64 = 0;
    let mut used_dollar: bool = false;
    let mut lj: bool = false;
    let mut alt: bool = false;
    let mut have_prec: bool = false;
    let mut need_format: bool = false;
    let mut cur: *mut i64 = 0 as *mut i64;
    let mut uval: uintmax_t = 0;
    let mut sgn: bool = false;
    let mut base: i32 = 0;
    let mut cpbufs: [C2RustUnnamed_12; 2] = [C2RustUnnamed_12 {
        buf: 0 as *mut i8,
        bufsize: 0,
        stackbuf: [0; 30],
    }; 2];
    let mut cend: *mut i8 = &mut *((*cpbufs.as_mut_ptr().offset(0 as i32 as isize))
        .stackbuf)
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize) as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut fill: *const i8 = 0 as *const i8;
    let mut tmpval: libc::c_double = 0.0f64;
    let mut signchar: i8 = '\0' as i32 as i8;
    let mut len: size_t = 0;
    let mut zero_flag: bool = 0 as i32 != 0;
    let mut quote_flag: bool = 0 as i32 != 0;
    let mut ii: i32 = 0;
    let mut jj: i32 = 0;
    let mut chp: *mut i8 = 0 as *mut i8;
    let mut copy_count: size_t = 0;
    let mut char_count: size_t = 0;
    let mut nan_inf_val: *mut i8 = 0 as *mut i8;
    let mut magic_posix_flag: bool = false;
    let mut fmt_type: C2RustUnnamed_13 = C2RustUnnamed_13::MP_NONE;
    static mut sp: [i8; 2] = unsafe {
        *::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b" \0")
    };
    static mut zero_string: [i8; 2] = unsafe {
        *::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b"0\0")
    };
    static mut lchbuf: [i8; 17] = unsafe {
        *::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"0123456789abcdef\0")
    };
    static mut Uchbuf: [i8; 17] = unsafe {
        *::core::mem::transmute::<&[u8; 17], &[i8; 17]>(b"0123456789ABCDEF\0")
    };
    static mut bad_modifiers: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"hjlLtz\0")
    };
    static mut warned: [bool; 6] = [false; 6];
    let mut modifier_seen: [bool; 6] = [false; 6];
    obuf = emalloc_real(
        64 as i32 as size_t,
        b"format_tree\0" as *const u8 as *const i8,
        b"obuf\0" as *const u8 as *const i8,
        b"builtin.c\0" as *const u8 as *const i8,
        811 as i32,
    ) as *mut i8;
    obufout = obuf;
    osiz = 64 as i32 as size_t;
    ofre = osiz.wrapping_sub(1 as i32 as u64);
    cur_arg = 1 as i32 as size_t;
    let mut k: size_t = 0;
    k = 0 as i32 as size_t;
    while k
        < (::core::mem::size_of::<[C2RustUnnamed_12; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_12>() as u64)
    {
        cpbufs[k as usize].bufsize = ::core::mem::size_of::<[i8; 30]>() as u64;
        cpbufs[k as usize].buf = (cpbufs[k as usize].stackbuf).as_mut_ptr();
        k = k.wrapping_add(1);
        k;
    }
    need_format = 0 as i32 != 0;
    used_dollar = 0 as i32 != 0;
    s1 = fmt_string;
    s0 = s1;
    's_130: loop {
        let fresh8 = n0;
        n0 = n0.wrapping_sub(1);
        if !(fresh8 > 0 as i32 as u64) {
            current_block = 2782926371273512654;
            break;
        }
        if *s1 as i32 != '%' as i32 {
            s1 = s1.offset(1);
            s1;
        } else {
            need_format = 1 as i32 != 0;
            if s1.offset_from(s0) as i64 != 0 {
                while s1.offset_from(s0) as i64 as u64 > ofre {
                    let mut olen: size_t = obufout.offset_from(obuf) as i64 as size_t;
                    obuf = erealloc_real(
                        obuf as *mut libc::c_void,
                        osiz.wrapping_mul(2 as i32 as u64),
                        b"format_tree\0" as *const u8 as *const i8,
                        b"obuf\0" as *const u8 as *const i8,
                        b"builtin.c\0" as *const u8 as *const i8,
                        885 as i32,
                    ) as *mut i8;
                    ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                    osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                        as size_t;
                    obufout = obuf.offset(olen as isize);
                }
                memcpy(
                    obufout as *mut libc::c_void,
                    s0 as *const libc::c_void,
                    s1.offset_from(s0) as i64 as size_t,
                );
                obufout = obufout.offset(s1.offset_from(s0) as i64 as isize);
                ofre = (ofre as u64).wrapping_sub(s1.offset_from(s0) as i64 as u64)
                    as size_t as size_t;
            }
            s0 = s1;
            cur = &mut fw;
            fw = 0 as i32 as i64;
            prec = 0 as i32 as i64;
            base = 0 as i32;
            argnum = 0 as i32 as i64;
            base = 0 as i32;
            have_prec = 0 as i32 != 0;
            signchar = '\0' as i32 as i8;
            zero_flag = 0 as i32 != 0;
            quote_flag = 0 as i32 != 0;
            nan_inf_val = 0 as *mut i8;
            fmt_type = C2RustUnnamed_13::MP_NONE;
            alt = 0 as i32 != 0;
            lj = alt;
            memset(
                modifier_seen.as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<[bool; 6]>() as u64,
            );
            magic_posix_flag = 0 as i32 != 0;
            fill = sp.as_ptr();
            cp = cend;
            chbuf = lchbuf.as_ptr();
            s1 = s1.offset(1);
            s1;
            loop {
                let fresh9 = n0;
                n0 = n0.wrapping_sub(1);
                if fresh9 == 0 as i32 as u64 {
                    current_block = 2782926371273512654;
                    break 's_130;
                }
                let fresh10 = s1;
                s1 = s1.offset(1);
                cs1 = *fresh10 as i32;
                match cs1 {
                    -1 => {
                        current_block = 5601585718440621758;
                    }
                    37 => {
                        need_format = 0 as i32 != 0;
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        {
                            let mut msg_0: *const i8 = 0 as *const i8;
                            if fw != 0 && !have_prec {
                                msg_0 = dcgettext(
                                    0 as *const i8,
                                    b"field width is ignored for `%%' specifier\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                );
                            } else if fw == 0 as i32 as i64 && have_prec as i32 != 0 {
                                msg_0 = dcgettext(
                                    0 as *const i8,
                                    b"precision is ignored for `%%' specifier\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                );
                            } else if fw != 0 && have_prec as i32 != 0 {
                                msg_0 = dcgettext(
                                    0 as *const i8,
                                    b"field width and precision are ignored for `%%' specifier\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                );
                            }
                            if !msg_0.is_null() {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"builtin.c\0" as *const u8 as *const i8,
                                    942 as i32,
                                );
                                (Some(lintfunc.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(b"%s\0" as *const u8 as *const i8, msg_0);
                            }
                        }
                        if ofre < 1 as i32 as u64 {
                            let mut olen_0: size_t = obufout.offset_from(obuf) as i64
                                as size_t;
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_mul(2 as i32 as u64),
                                b"format_tree\0" as *const u8 as *const i8,
                                b"obuf\0" as *const u8 as *const i8,
                                b"builtin.c\0" as *const u8 as *const i8,
                                944 as i32,
                            ) as *mut i8;
                            ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                            osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                as size_t;
                            obufout = obuf.offset(olen_0 as isize);
                        }
                        let fresh11 = obufout;
                        obufout = obufout.offset(1);
                        *fresh11 = *(b"%\0" as *const u8 as *const i8);
                        ofre = ofre.wrapping_sub(1);
                        ofre;
                        s0 = s1;
                        current_block = 2749501120807187827;
                        break;
                    }
                    48 => {
                        if cur == &mut fw as *mut i64 {
                            zero_flag = 1 as i32 != 0;
                        }
                        if lj {
                            continue;
                        }
                        current_block = 17893294621977578213;
                    }
                    49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 17893294621977578213;
                    }
                    36 => {
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            != 0
                        {
                            msg(
                                dcgettext(
                                    0 as *const i8,
                                    b"fatal: `$' is not permitted in awk formats\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                            current_block = 6720548472269345400;
                            break 's_130;
                        } else if cur == &mut fw as *mut i64 {
                            argnum = fw;
                            fw = 0 as i32 as i64;
                            used_dollar = 1 as i32 != 0;
                            if argnum <= 0 as i32 as i64 {
                                msg(
                                    dcgettext(
                                        0 as *const i8,
                                        b"fatal: argument index with `$' must be > 0\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 6720548472269345400;
                                break 's_130;
                            } else {
                                if !(argnum >= num_args) {
                                    continue;
                                }
                                msg(
                                    dcgettext(
                                        0 as *const i8,
                                        b"fatal: argument index %ld greater than total number of supplied arguments\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    argnum,
                                );
                                current_block = 6720548472269345400;
                                break 's_130;
                            }
                        } else {
                            msg(
                                dcgettext(
                                    0 as *const i8,
                                    b"fatal: `$' not permitted after period in format\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            current_block = 6720548472269345400;
                            break 's_130;
                        }
                    }
                    42 => {
                        if cur.is_null() {
                            current_block = 2749501120807187827;
                            break;
                        }
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            == 0 && used_dollar as i32 != 0
                            && *(*__ctype_b_loc()).offset(*s1 as u8 as i32 as isize)
                                as i32
                                & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                == 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const i8,
                                1016 as i32,
                            );
                            (Some(
                                (Some(
                                    r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                                ))
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"fatal: must use `count$' on all formats or none\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            current_block = 2749501120807187827;
                            break;
                        } else {
                            if do_flags as u32
                                & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                                && *(*__ctype_b_loc()).offset(*s1 as u8 as i32 as isize)
                                    as i32
                                    & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                    != 0
                            {
                                let mut val: i32 = 0 as i32;
                                while n0 > 0 as i32 as u64 && *s1 as i32 != 0
                                    && *(*__ctype_b_loc()).offset(*s1 as u8 as i32 as isize)
                                        as i32
                                        & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                        != 0
                                {
                                    val *= 10 as i32;
                                    val += *s1 as i32 - '0' as i32;
                                    s1 = s1.offset(1);
                                    s1;
                                    n0 = n0.wrapping_sub(1);
                                    n0;
                                }
                                if *s1 as i32 != '$' as i32 {
                                    msg(
                                        dcgettext(
                                            0 as *const i8,
                                            b"fatal: no `$' supplied for positional field width or precision\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 6720548472269345400;
                                    break 's_130;
                                } else {
                                    s1 = s1.offset(1);
                                    s1;
                                    n0 = n0.wrapping_sub(1);
                                    n0;
                                    if val < 0 as i32 || val as i64 >= num_args {
                                        toofew = 1 as i32 != 0;
                                        current_block = 2749501120807187827;
                                        break;
                                    } else {
                                        arg = *the_args.offset(val as isize);
                                    }
                                }
                            } else if argnum > 0 as i32 as i64 {
                                if cur_arg > 1 as i32 as u64 {
                                    msg(
                                        dcgettext(
                                            0 as *const i8,
                                            b"fatal: must use `count$' on all formats or none\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 6720548472269345400;
                                    break 's_130;
                                } else {
                                    arg = *the_args.offset(argnum as isize);
                                }
                            } else if used_dollar {
                                msg(
                                    dcgettext(
                                        0 as *const i8,
                                        b"fatal: must use `count$' on all formats or none\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                arg = 0 as *mut NODE;
                                current_block = 6720548472269345400;
                                break 's_130;
                            } else if cur_arg >= num_args as u64 {
                                arg = 0 as *mut NODE;
                                toofew = 1 as i32 != 0;
                                current_block = 2749501120807187827;
                                break;
                            } else {
                                arg = *the_args.offset(cur_arg as isize);
                                cur_arg = cur_arg.wrapping_add(1);
                                cur_arg;
                            }
                            force_number(arg);
                            *cur = (*arg).sub.val.fltnum as i64;
                            if *cur < 0 as i32 as i64 && cur == &mut fw as *mut i64 {
                                *cur = -*cur;
                                lj = 1 as i32 != 0;
                            }
                            if cur == &mut prec as *mut i64 {
                                if *cur >= 0 as i32 as i64 {
                                    have_prec = 1 as i32 != 0;
                                } else {
                                    have_prec = 0 as i32 != 0;
                                }
                                cur = 0 as *mut i64;
                            }
                            continue;
                        }
                    }
                    32 => {
                        if signchar as i32 != 0 as i32 {
                            current_block = 5601585718440621758;
                        } else {
                            current_block = 8262146976874296118;
                        }
                    }
                    43 => {
                        current_block = 8262146976874296118;
                    }
                    45 => {
                        if prec < 0 as i32 as i64 {
                            current_block = 2749501120807187827;
                            break;
                        }
                        if cur == &mut prec as *mut i64 {
                            prec = -(1 as i32) as i64;
                            continue;
                        } else {
                            fill = sp.as_ptr();
                            lj = 1 as i32 != 0;
                        }
                        current_block = 5601585718440621758;
                    }
                    46 => {
                        if cur != &mut fw as *mut i64 {
                            current_block = 2749501120807187827;
                            break;
                        }
                        cur = &mut prec;
                        have_prec = 1 as i32 != 0;
                        continue;
                    }
                    35 => {
                        alt = 1 as i32 != 0;
                        current_block = 5601585718440621758;
                    }
                    39 => {
                        quote_flag = 1 as i32 != 0;
                        current_block = 5601585718440621758;
                    }
                    104 | 106 | 108 | 76 | 116 | 122 => {
                        if modifier_seen[(strchr(bad_modifiers.as_ptr(), cs1))
                            .offset_from(bad_modifiers.as_ptr()) as i64 as usize]
                        {
                            current_block = 2749501120807187827;
                            break;
                        }
                        let mut ind: i32 = (strchr(bad_modifiers.as_ptr(), cs1))
                            .offset_from(bad_modifiers.as_ptr()) as i64 as i32;
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            && !warned[ind as usize]
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const i8,
                                1104 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"`%c' is meaningless in awk formats; ignored\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                cs1,
                            );
                            warned[ind as usize] = 1 as i32 != 0;
                        }
                        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
                        {
                            msg(
                                dcgettext(
                                    0 as *const i8,
                                    b"fatal: `%c' is not permitted in POSIX awk formats\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                cs1,
                            );
                            current_block = 6720548472269345400;
                            break 's_130;
                        } else {
                            modifier_seen[(strchr(bad_modifiers.as_ptr(), cs1))
                                .offset_from(bad_modifiers.as_ptr()) as i64 as usize] = 1
                                as i32 != 0;
                            continue;
                        }
                    }
                    80 => {
                        if magic_posix_flag {
                            current_block = 2749501120807187827;
                            break;
                        }
                        magic_posix_flag = 1 as i32 != 0;
                        continue;
                    }
                    99 => {
                        need_format = 0 as i32 != 0;
                        if argnum > 0 as i32 as i64 {
                            current_block = 9822987968968565122;
                            break;
                        } else {
                            current_block = 10282596542094995802;
                            break;
                        }
                    }
                    115 => {
                        need_format = 0 as i32 != 0;
                        if argnum > 0 as i32 as i64 {
                            current_block = 1365401674109093360;
                            break;
                        } else {
                            current_block = 870304920170074103;
                            break;
                        }
                    }
                    100 | 105 => {
                        need_format = 0 as i32 != 0;
                        if argnum > 0 as i32 as i64 {
                            current_block = 11364608634565542496;
                            break;
                        } else {
                            current_block = 17342404680090366851;
                            break;
                        }
                    }
                    88 => {
                        chbuf = Uchbuf.as_ptr();
                        current_block = 13953176060137358773;
                        break;
                    }
                    120 => {
                        current_block = 13953176060137358773;
                        break;
                    }
                    117 => {
                        current_block = 767981726298143985;
                        break;
                    }
                    111 => {
                        current_block = 18261519013591099499;
                        break;
                    }
                    70 | 103 | 71 | 101 | 102 | 69 | 65 | 97 => {
                        static mut warned_0: bool = 0 as i32 != 0;
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            && ({
                                let mut __res: i32 = 0;
                                if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                    if 0 != 0 {
                                        let mut __c: i32 = cs1;
                                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower(cs1);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc()).offset(cs1 as isize);
                                }
                                __res
                            }) == 'a' as i32 && !warned_0
                        {
                            warned_0 = 1 as i32 != 0;
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const i8,
                                1577 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"%%%c format is POSIX standard but not portable to other awks\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                cs1,
                            );
                        }
                        need_format = 0 as i32 != 0;
                        if argnum > 0 as i32 as i64 {
                            current_block = 3889686208735362918;
                            break;
                        } else {
                            current_block = 7186168334825677637;
                            break;
                        }
                    }
                    _ => {
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            && is_alpha(cs1) as i32 != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const i8,
                                1688 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"ignoring unknown format specifier character `%c': no argument converted\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                cs1,
                            );
                        }
                        current_block = 2749501120807187827;
                        break;
                    }
                }
                match current_block {
                    17893294621977578213 => {
                        if cur.is_null() {
                            current_block = 2749501120807187827;
                            break;
                        }
                        if prec >= 0 as i32 as i64 {
                            *cur = (cs1 - '0' as i32) as i64;
                        }
                        while n0 > 0 as i32 as u64 && *s1 as i32 >= '0' as i32
                            && *s1 as i32 <= '9' as i32
                        {
                            n0 = n0.wrapping_sub(1);
                            n0;
                            let fresh12 = s1;
                            s1 = s1.offset(1);
                            *cur = *cur * 10 as i32 as i64 + *fresh12 as i64
                                - '0' as i32 as i64;
                        }
                        if prec < 0 as i32 as i64 {
                            have_prec = 0 as i32 != 0;
                        }
                        if cur == &mut prec as *mut i64 {
                            cur = 0 as *mut i64;
                        }
                        if n0 == 0 as i32 as u64 {
                            continue 's_130;
                        } else {
                            continue;
                        }
                    }
                    8262146976874296118 => {
                        signchar = cs1 as i8;
                    }
                    _ => {}
                }
                if cur != &mut fw as *mut i64 {
                    current_block = 2749501120807187827;
                    break;
                }
            }
            match current_block {
                17342404680090366851 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 6720548472269345400;
                        break;
                    } else if cur_arg >= num_args as u64 {
                        arg = 0 as *mut NODE;
                        toofew = 1 as i32 != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 1761421285656443595;
                    }
                }
                11364608634565542496 => {
                    if cur_arg > 1 as i32 as u64 {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        current_block = 6720548472269345400;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 1761421285656443595;
                }
                870304920170074103 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 6720548472269345400;
                        break;
                    } else if cur_arg >= num_args as u64 {
                        arg = 0 as *mut NODE;
                        toofew = 1 as i32 != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 12003943868717696208;
                    }
                }
                1365401674109093360 => {
                    if cur_arg > 1 as i32 as u64 {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        current_block = 6720548472269345400;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 12003943868717696208;
                }
                10282596542094995802 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 6720548472269345400;
                        break;
                    } else if cur_arg >= num_args as u64 {
                        arg = 0 as *mut NODE;
                        toofew = 1 as i32 != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 2346768750020253347;
                    }
                }
                9822987968968565122 => {
                    if cur_arg > 1 as i32 as u64 {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        current_block = 6720548472269345400;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 2346768750020253347;
                }
                7186168334825677637 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 6720548472269345400;
                        break;
                    } else if cur_arg >= num_args as u64 {
                        arg = 0 as *mut NODE;
                        toofew = 1 as i32 != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 2847482619380721777;
                    }
                }
                3889686208735362918 => {
                    if cur_arg > 1 as i32 as u64 {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        current_block = 6720548472269345400;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 2847482619380721777;
                }
                13953176060137358773 => {
                    base += 6 as i32;
                    current_block = 767981726298143985;
                }
                _ => {}
            }
            match current_block {
                1761421285656443595 => {
                    force_number(arg);
                    if out_of_range(arg) {
                        current_block = 12041914390200727874;
                    } else {
                        tmpval = double_to_int((*arg).sub.val.fltnum);
                        if have_prec as i32 != 0 && prec == 0 as i32 as i64
                            && tmpval == 0 as i32 as libc::c_double
                        {
                            current_block = 14418787953404077073;
                        } else {
                            if tmpval < 0 as i32 as libc::c_double {
                                tmpval = -tmpval;
                                sgn = 1 as i32 != 0;
                            } else {
                                if tmpval == -0.0f64 {
                                    tmpval = 0.0f64;
                                }
                                sgn = 0 as i32 != 0;
                            }
                            loop {
                                i = snprintf(
                                    cpbufs[1 as i32 as usize].buf,
                                    cpbufs[1 as i32 as usize].bufsize,
                                    b"%.0f\0" as *const u8 as *const i8,
                                    tmpval,
                                );
                                if !(i as u64 >= cpbufs[1 as i32 as usize].bufsize) {
                                    break;
                                }
                                if cpbufs[1 as i32 as usize].buf
                                    == (cpbufs[1 as i32 as usize].stackbuf).as_mut_ptr()
                                {
                                    cpbufs[1 as i32 as usize].buf = 0 as *mut i8;
                                }
                                if i > 0 as i32 {
                                    cpbufs[1 as i32 as usize].bufsize = (cpbufs[1 as i32
                                            as usize]
                                        .bufsize as u64)
                                        .wrapping_add(
                                            if i as u64 > cpbufs[1 as i32 as usize].bufsize {
                                                i as u64
                                            } else {
                                                cpbufs[1 as i32 as usize].bufsize
                                            },
                                        ) as size_t as size_t;
                                } else {
                                    cpbufs[1 as i32 as usize].bufsize = (cpbufs[1 as i32
                                            as usize]
                                        .bufsize as u64)
                                        .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                }
                                cpbufs[1 as i32 as usize].buf = erealloc_real(
                                    cpbufs[1 as i32 as usize].buf as *mut libc::c_void,
                                    cpbufs[1 as i32 as usize].bufsize,
                                    b"format_tree\0" as *const u8 as *const i8,
                                    b"cpbufs[1].buf\0" as *const u8 as *const i8,
                                    b"builtin.c\0" as *const u8 as *const i8,
                                    1266 as i32,
                                ) as *mut i8;
                            }
                            if i < 1 as i32 {
                                current_block = 12041914390200727874;
                            } else {
                                quote_flag = quote_flag as i32 != 0
                                    && *(loc.thousands_sep).offset(0 as i32 as isize) as i32
                                        != 0 as i32;
                                chp = &mut *((*cpbufs
                                    .as_mut_ptr()
                                    .offset(1 as i32 as isize))
                                    .buf)
                                    .offset((i - 1 as i32) as isize) as *mut i8;
                                jj = 0 as i32;
                                ii = jj;
                                loop {
                                    if cp == cpbufs[0 as i32 as usize].buf {
                                        let mut prev: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                            (2 as i32 as u64)
                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                            b"format_tree\0" as *const u8 as *const i8,
                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                            b"builtin.c\0" as *const u8 as *const i8,
                                            1276 as i32,
                                        ) as *mut i8;
                                        cp = (cpbufs[0 as i32 as usize].buf)
                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                        memcpy(
                                            cp as *mut libc::c_void,
                                            prev as *const libc::c_void,
                                            cpbufs[0 as i32 as usize].bufsize,
                                        );
                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                as usize]
                                            .bufsize as u64)
                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                        if prev != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                        {
                                            pma_free(prev as *mut libc::c_void);
                                        }
                                        cend = (cpbufs[0 as i32 as usize].buf)
                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                    }
                                    cp = cp.offset(-1);
                                    *cp = *chp;
                                    chp = chp.offset(-1);
                                    chp;
                                    i -= 1;
                                    i;
                                    if quote_flag as i32 != 0
                                        && *(loc.grouping).offset(ii as isize) as i32 != 0
                                        && {
                                            jj += 1;
                                            jj == *(loc.grouping).offset(ii as isize) as i32
                                        }
                                    {
                                        if i != 0 {
                                            let mut k_0: i32 = 0;
                                            let mut ts: *const i8 = loc.thousands_sep;
                                            k_0 = (strlen(ts)).wrapping_sub(1 as i32 as u64) as i32;
                                            while k_0 >= 0 as i32 {
                                                if cp == cpbufs[0 as i32 as usize].buf {
                                                    let mut prev_0: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                    cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                        (2 as i32 as u64)
                                                            .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                        b"format_tree\0" as *const u8 as *const i8,
                                                        b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                        b"builtin.c\0" as *const u8 as *const i8,
                                                        1285 as i32,
                                                    ) as *mut i8;
                                                    cp = (cpbufs[0 as i32 as usize].buf)
                                                        .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                    memcpy(
                                                        cp as *mut libc::c_void,
                                                        prev_0 as *const libc::c_void,
                                                        cpbufs[0 as i32 as usize].bufsize,
                                                    );
                                                    cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                            as usize]
                                                        .bufsize as u64)
                                                        .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                    if prev_0
                                                        != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                    {
                                                        pma_free(prev_0 as *mut libc::c_void);
                                                    }
                                                    cend = (cpbufs[0 as i32 as usize].buf)
                                                        .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                }
                                                cp = cp.offset(-1);
                                                *cp = *ts.offset(k_0 as isize);
                                                k_0 -= 1;
                                                k_0;
                                            }
                                        }
                                        if *(loc.grouping).offset((ii + 1 as i32) as isize) as i32
                                            == 0 as i32
                                        {
                                            jj = 0 as i32;
                                        } else if *(loc.grouping).offset((ii + 1 as i32) as isize)
                                            as i32 == 127 as i32
                                        {
                                            quote_flag = 0 as i32 != 0;
                                        } else {
                                            ii += 1;
                                            ii;
                                            jj = 0 as i32;
                                        }
                                    }
                                    if !(i > 0 as i32) {
                                        break;
                                    }
                                }
                                if have_prec {
                                    while (cend.offset_from(cp) as i64) < prec {
                                        if cp == cpbufs[0 as i32 as usize].buf {
                                            let mut prev_1: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                            cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                (2 as i32 as u64)
                                                    .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                b"format_tree\0" as *const u8 as *const i8,
                                                b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                b"builtin.c\0" as *const u8 as *const i8,
                                                1303 as i32,
                                            ) as *mut i8;
                                            cp = (cpbufs[0 as i32 as usize].buf)
                                                .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                            memcpy(
                                                cp as *mut libc::c_void,
                                                prev_1 as *const libc::c_void,
                                                cpbufs[0 as i32 as usize].bufsize,
                                            );
                                            cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                    as usize]
                                                .bufsize as u64)
                                                .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                            if prev_1
                                                != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                            {
                                                pma_free(prev_1 as *mut libc::c_void);
                                            }
                                            cend = (cpbufs[0 as i32 as usize].buf)
                                                .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                        }
                                        cp = cp.offset(-1);
                                        *cp = '0' as i32 as i8;
                                    }
                                }
                                if sgn {
                                    if cp == cpbufs[0 as i32 as usize].buf {
                                        let mut prev_2: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                            (2 as i32 as u64)
                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                            b"format_tree\0" as *const u8 as *const i8,
                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                            b"builtin.c\0" as *const u8 as *const i8,
                                            1307 as i32,
                                        ) as *mut i8;
                                        cp = (cpbufs[0 as i32 as usize].buf)
                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                        memcpy(
                                            cp as *mut libc::c_void,
                                            prev_2 as *const libc::c_void,
                                            cpbufs[0 as i32 as usize].bufsize,
                                        );
                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                as usize]
                                            .bufsize as u64)
                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                        if prev_2
                                            != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                        {
                                            pma_free(prev_2 as *mut libc::c_void);
                                        }
                                        cend = (cpbufs[0 as i32 as usize].buf)
                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                    }
                                    cp = cp.offset(-1);
                                    *cp = '-' as i32 as i8;
                                } else if signchar != 0 {
                                    if cp == cpbufs[0 as i32 as usize].buf {
                                        let mut prev_3: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                            (2 as i32 as u64)
                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                            b"format_tree\0" as *const u8 as *const i8,
                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                            b"builtin.c\0" as *const u8 as *const i8,
                                            1309 as i32,
                                        ) as *mut i8;
                                        cp = (cpbufs[0 as i32 as usize].buf)
                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                        memcpy(
                                            cp as *mut libc::c_void,
                                            prev_3 as *const libc::c_void,
                                            cpbufs[0 as i32 as usize].bufsize,
                                        );
                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                as usize]
                                            .bufsize as u64)
                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                        if prev_3
                                            != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                        {
                                            pma_free(prev_3 as *mut libc::c_void);
                                        }
                                        cend = (cpbufs[0 as i32 as usize].buf)
                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                    }
                                    cp = cp.offset(-1);
                                    *cp = signchar;
                                }
                                if !lj
                                    && (zero_flag as i32 != 0 && !have_prec
                                        || fw == 0 as i32 as i64 && have_prec as i32 != 0)
                                {
                                    fill = zero_string.as_ptr();
                                }
                                if prec > fw {
                                    fw = prec;
                                }
                                prec = cend.offset_from(cp) as i64;
                                if fw > prec && !lj && fill != sp.as_ptr()
                                    && (*cp as i32 == '-' as i32 || signchar as i32 != 0)
                                {
                                    if ofre < 1 as i32 as u64 {
                                        let mut olen_1: size_t = obufout.offset_from(obuf) as i64
                                            as size_t;
                                        obuf = erealloc_real(
                                            obuf as *mut libc::c_void,
                                            osiz.wrapping_mul(2 as i32 as u64),
                                            b"format_tree\0" as *const u8 as *const i8,
                                            b"obuf\0" as *const u8 as *const i8,
                                            b"builtin.c\0" as *const u8 as *const i8,
                                            1327 as i32,
                                        ) as *mut i8;
                                        ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                                        osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                            as size_t;
                                        obufout = obuf.offset(olen_1 as isize);
                                    }
                                    let fresh13 = obufout;
                                    obufout = obufout.offset(1);
                                    *fresh13 = *cp;
                                    ofre = ofre.wrapping_sub(1);
                                    ofre;
                                    cp = cp.offset(1);
                                    cp;
                                    prec -= 1;
                                    prec;
                                    fw -= 1;
                                    fw;
                                }
                                current_block = 14418787953404077073;
                            }
                        }
                    }
                }
                12003943868717696208 => {
                    arg = force_string_fmt(arg, CONVFMT, CONVFMTidx);
                    if fw == 0 as i32 as i64 && !have_prec {
                        prec = (*arg).sub.val.slen as i64;
                    } else {
                        char_count = mbc_char_count(
                            (*arg).sub.val.sp,
                            (*arg).sub.val.slen,
                        );
                        if !have_prec || prec as u64 > char_count {
                            prec = char_count as i64;
                        }
                    }
                    cp = (*arg).sub.val.sp;
                    current_block = 14418787953404077073;
                }
                2346768750020253347 => {
                    fixtype(arg);
                    if (*arg).flags as u32 & flagvals::NUMBER as i32 as u32
                        != 0 as i32 as u32
                    {
                        uval = (*arg).sub.val.fltnum as uintmax_t;
                        if gawk_mb_cur_max > 1 as i32 {
                            let mut buf: [i8; 100] = [0; 100];
                            let mut wc: wchar_t = 0;
                            let mut mbs: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            let mut count: size_t = 0;
                            memset(
                                &mut mbs as *mut mbstate_t as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<mbstate_t>() as u64,
                            );
                            if (::core::mem::size_of::<wchar_t>() as u64)
                                < 4 as i32 as u64 && uval > 0xffff as i32 as u64
                            {
                                if do_flags as u32
                                    & (do_flag_values::DO_LINT_INVALID as i32
                                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const i8,
                                            i32,
                                        ) -> ())(
                                        b"builtin.c\0" as *const u8 as *const i8,
                                        1138 as i32,
                                    );
                                    (Some(lintfunc.expect("non-null function pointer")))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            0 as *const i8,
                                            b"[s]printf: value %g is too big for %%c format\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                        (*arg).sub.val.fltnum,
                                    );
                                }
                                current_block = 12350242817162068077;
                            } else {
                                wc = uval as wchar_t;
                                count = wcrtomb(buf.as_mut_ptr(), wc, &mut mbs);
                                if count == 0 as i32 as u64
                                    || count == -(1 as i32) as size_t
                                {
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const i8,
                                                i32,
                                            ) -> ())(
                                            b"builtin.c\0" as *const u8 as *const i8,
                                            1151 as i32,
                                        );
                                        (Some(lintfunc.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            dcgettext(
                                                0 as *const i8,
                                                b"[s]printf: value %g is not a valid wide character\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                            (*arg).sub.val.fltnum,
                                        );
                                    }
                                    current_block = 12350242817162068077;
                                } else {
                                    memcpy(
                                        cpbufs[0 as i32 as usize].buf as *mut libc::c_void,
                                        buf.as_mut_ptr() as *const libc::c_void,
                                        count,
                                    );
                                    prec = count as i64;
                                    cp = cpbufs[0 as i32 as usize].buf;
                                    current_block = 14418787953404077073;
                                }
                            }
                        } else {
                            current_block = 12350242817162068077;
                        }
                        match current_block {
                            14418787953404077073 => {}
                            _ => {
                                *(cpbufs[0 as i32 as usize].buf)
                                    .offset(0 as i32 as isize) = uval as i8;
                                prec = 1 as i32 as i64;
                                cp = cpbufs[0 as i32 as usize].buf;
                            }
                        }
                    } else {
                        cp = (*arg).sub.val.sp;
                        prec = 1 as i32 as i64;
                        if gawk_mb_cur_max > 1 as i32 {
                            let mut state_0: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            let mut count_0: size_t = 0;
                            memset(
                                &mut state_0 as *mut mbstate_t as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<mbstate_t>() as u64,
                            );
                            count_0 = mbrlen(cp, (*arg).sub.val.slen, &mut state_0);
                            if count_0 != -(1 as i32) as size_t
                                && count_0 != -(2 as i32) as size_t
                                && count_0 > 0 as i32 as u64
                            {
                                prec = count_0 as i64;
                                if fw > 0 as i32 as i64 {
                                    fw = (fw as u64)
                                        .wrapping_add(count_0.wrapping_sub(1 as i32 as u64)) as i64
                                        as i64;
                                }
                            }
                        }
                    }
                    current_block = 14418787953404077073;
                }
                2847482619380721777 => {
                    force_number(arg);
                    if 0 as i32 == 0 {
                        tmpval = (*arg).sub.val.fltnum;
                    }
                    if out_of_range(arg) {
                        current_block = 12041914390200727874;
                    } else {
                        current_block = 17832854953326123394;
                    }
                }
                767981726298143985 => {
                    base += 2 as i32;
                    current_block = 18261519013591099499;
                }
                _ => {}
            }
            match current_block {
                18261519013591099499 => {
                    base += 8 as i32;
                    need_format = 0 as i32 != 0;
                    if argnum > 0 as i32 as i64 {
                        if cur_arg > 1 as i32 as u64 {
                            msg(
                                dcgettext(
                                    0 as *const i8,
                                    b"fatal: must use `count$' on all formats or none\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            current_block = 6720548472269345400;
                            break;
                        } else {
                            arg = *the_args.offset(argnum as isize);
                        }
                        current_block = 2589719962955437281;
                    } else if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const i8,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 6720548472269345400;
                        break;
                    } else if cur_arg >= num_args as u64 {
                        arg = 0 as *mut NODE;
                        toofew = 1 as i32 != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 2589719962955437281;
                    }
                    match current_block {
                        2749501120807187827 => {}
                        _ => {
                            force_number(arg);
                            if out_of_range(arg) {
                                current_block = 12041914390200727874;
                            } else {
                                tmpval = (*arg).sub.val.fltnum;
                                if !alt && have_prec as i32 != 0 && prec == 0 as i32 as i64
                                    && tmpval == 0 as i32 as libc::c_double
                                {
                                    current_block = 14418787953404077073;
                                } else {
                                    if tmpval < 0 as i32 as libc::c_double {
                                        uval = tmpval as intmax_t as uintmax_t;
                                        if uval as intmax_t as libc::c_double
                                            != double_to_int(tmpval)
                                        {
                                            current_block = 12041914390200727874;
                                        } else {
                                            current_block = 11060486315799977401;
                                        }
                                    } else {
                                        uval = tmpval as uintmax_t;
                                        if uval as libc::c_double != double_to_int(tmpval) {
                                            current_block = 12041914390200727874;
                                        } else {
                                            current_block = 11060486315799977401;
                                        }
                                    }
                                    match current_block {
                                        12041914390200727874 => {}
                                        _ => {
                                            quote_flag = quote_flag as i32 != 0
                                                && *(loc.thousands_sep).offset(0 as i32 as isize) as i32
                                                    != 0 as i32;
                                            if !lj
                                                && (zero_flag as i32 != 0 && !have_prec
                                                    || fw == 0 as i32 as i64 && have_prec as i32 != 0)
                                            {
                                                fill = zero_string.as_ptr();
                                            }
                                            jj = 0 as i32;
                                            ii = jj;
                                            loop {
                                                if cp == cpbufs[0 as i32 as usize].buf {
                                                    let mut prev_4: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                    cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                        (2 as i32 as u64)
                                                            .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                        b"format_tree\0" as *const u8 as *const i8,
                                                        b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                        b"builtin.c\0" as *const u8 as *const i8,
                                                        1463 as i32,
                                                    ) as *mut i8;
                                                    cp = (cpbufs[0 as i32 as usize].buf)
                                                        .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                    memcpy(
                                                        cp as *mut libc::c_void,
                                                        prev_4 as *const libc::c_void,
                                                        cpbufs[0 as i32 as usize].bufsize,
                                                    );
                                                    cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                            as usize]
                                                        .bufsize as u64)
                                                        .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                    if prev_4
                                                        != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                    {
                                                        pma_free(prev_4 as *mut libc::c_void);
                                                    }
                                                    cend = (cpbufs[0 as i32 as usize].buf)
                                                        .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                }
                                                cp = cp.offset(-1);
                                                *cp = *chbuf
                                                    .offset(uval.wrapping_rem(base as u64) as isize);
                                                uval = (uval as u64).wrapping_div(base as u64) as uintmax_t
                                                    as uintmax_t;
                                                if base == 10 as i32 && quote_flag as i32 != 0
                                                    && *(loc.grouping).offset(ii as isize) as i32 != 0
                                                    && {
                                                        jj += 1;
                                                        jj == *(loc.grouping).offset(ii as isize) as i32
                                                    }
                                                {
                                                    if uval != 0 {
                                                        let mut k_1: i32 = 0;
                                                        let mut ts_0: *const i8 = loc.thousands_sep;
                                                        k_1 = (strlen(ts_0)).wrapping_sub(1 as i32 as u64) as i32;
                                                        while k_1 >= 0 as i32 {
                                                            if cp == cpbufs[0 as i32 as usize].buf {
                                                                let mut prev_5: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                                cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                                    (2 as i32 as u64)
                                                                        .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                                    b"format_tree\0" as *const u8 as *const i8,
                                                                    b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                                    b"builtin.c\0" as *const u8 as *const i8,
                                                                    1472 as i32,
                                                                ) as *mut i8;
                                                                cp = (cpbufs[0 as i32 as usize].buf)
                                                                    .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                                memcpy(
                                                                    cp as *mut libc::c_void,
                                                                    prev_5 as *const libc::c_void,
                                                                    cpbufs[0 as i32 as usize].bufsize,
                                                                );
                                                                cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                                        as usize]
                                                                    .bufsize as u64)
                                                                    .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                                if prev_5
                                                                    != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                                {
                                                                    pma_free(prev_5 as *mut libc::c_void);
                                                                }
                                                                cend = (cpbufs[0 as i32 as usize].buf)
                                                                    .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                            }
                                                            cp = cp.offset(-1);
                                                            *cp = *ts_0.offset(k_1 as isize);
                                                            k_1 -= 1;
                                                            k_1;
                                                        }
                                                    }
                                                    if *(loc.grouping).offset((ii + 1 as i32) as isize) as i32
                                                        == 0 as i32
                                                    {
                                                        jj = 0 as i32;
                                                    } else if *(loc.grouping).offset((ii + 1 as i32) as isize)
                                                        as i32 == 127 as i32
                                                    {
                                                        quote_flag = 0 as i32 != 0;
                                                    } else {
                                                        ii += 1;
                                                        ii;
                                                        jj = 0 as i32;
                                                    }
                                                }
                                                if !(uval > 0 as i32 as u64) {
                                                    break;
                                                }
                                            }
                                            if have_prec {
                                                while (cend.offset_from(cp) as i64) < prec {
                                                    if cp == cpbufs[0 as i32 as usize].buf {
                                                        let mut prev_6: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                            (2 as i32 as u64)
                                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const i8,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                            b"builtin.c\0" as *const u8 as *const i8,
                                                            1490 as i32,
                                                        ) as *mut i8;
                                                        cp = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_6 as *const libc::c_void,
                                                            cpbufs[0 as i32 as usize].bufsize,
                                                        );
                                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                                as usize]
                                                            .bufsize as u64)
                                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                        if prev_6
                                                            != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            pma_free(prev_6 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = '0' as i32 as i8;
                                                }
                                            }
                                            if alt as i32 != 0 && tmpval != 0 as i32 as libc::c_double {
                                                if base == 16 as i32 {
                                                    if cp == cpbufs[0 as i32 as usize].buf {
                                                        let mut prev_7: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                            (2 as i32 as u64)
                                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const i8,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                            b"builtin.c\0" as *const u8 as *const i8,
                                                            1495 as i32,
                                                        ) as *mut i8;
                                                        cp = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_7 as *const libc::c_void,
                                                            cpbufs[0 as i32 as usize].bufsize,
                                                        );
                                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                                as usize]
                                                            .bufsize as u64)
                                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                        if prev_7
                                                            != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            pma_free(prev_7 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = cs1 as i8;
                                                    if cp == cpbufs[0 as i32 as usize].buf {
                                                        let mut prev_8: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                            (2 as i32 as u64)
                                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const i8,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                            b"builtin.c\0" as *const u8 as *const i8,
                                                            1496 as i32,
                                                        ) as *mut i8;
                                                        cp = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_8 as *const libc::c_void,
                                                            cpbufs[0 as i32 as usize].bufsize,
                                                        );
                                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                                as usize]
                                                            .bufsize as u64)
                                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                        if prev_8
                                                            != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            pma_free(prev_8 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = '0' as i32 as i8;
                                                    if fill != sp.as_ptr() {
                                                        while 2 as i32 as u64 > ofre {
                                                            let mut olen_2: size_t = obufout.offset_from(obuf) as i64
                                                                as size_t;
                                                            obuf = erealloc_real(
                                                                obuf as *mut libc::c_void,
                                                                osiz.wrapping_mul(2 as i32 as u64),
                                                                b"format_tree\0" as *const u8 as *const i8,
                                                                b"obuf\0" as *const u8 as *const i8,
                                                                b"builtin.c\0" as *const u8 as *const i8,
                                                                1498 as i32,
                                                            ) as *mut i8;
                                                            ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                                                            osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                                                as size_t;
                                                            obufout = obuf.offset(olen_2 as isize);
                                                        }
                                                        memcpy(
                                                            obufout as *mut libc::c_void,
                                                            cp as *const libc::c_void,
                                                            2 as i32 as size_t,
                                                        );
                                                        obufout = obufout.offset(2 as i32 as isize);
                                                        ofre = (ofre as u64).wrapping_sub(2 as i32 as u64) as size_t
                                                            as size_t;
                                                        cp = cp.offset(2 as i32 as isize);
                                                        fw -= 2 as i32 as i64;
                                                    }
                                                } else if base == 8 as i32 {
                                                    if cp == cpbufs[0 as i32 as usize].buf {
                                                        let mut prev_9: *mut i8 = cpbufs[0 as i32 as usize].buf;
                                                        cpbufs[0 as i32 as usize].buf = emalloc_real(
                                                            (2 as i32 as u64)
                                                                .wrapping_mul(cpbufs[0 as i32 as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const i8,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const i8,
                                                            b"builtin.c\0" as *const u8 as *const i8,
                                                            1503 as i32,
                                                        ) as *mut i8;
                                                        cp = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_9 as *const libc::c_void,
                                                            cpbufs[0 as i32 as usize].bufsize,
                                                        );
                                                        cpbufs[0 as i32 as usize].bufsize = (cpbufs[0 as i32
                                                                as usize]
                                                            .bufsize as u64)
                                                            .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                                                        if prev_9
                                                            != (cpbufs[0 as i32 as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            pma_free(prev_9 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as i32 as usize].buf)
                                                            .offset(cpbufs[0 as i32 as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = '0' as i32 as i8;
                                                }
                                            }
                                            base = 0 as i32;
                                            if prec > fw {
                                                fw = prec;
                                            }
                                            prec = cend.offset_from(cp) as i64;
                                            current_block = 14418787953404077073;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                14418787953404077073 => {
                    if !lj {
                        while fw > prec {
                            if ofre < 1 as i32 as u64 {
                                let mut olen_3: size_t = obufout.offset_from(obuf) as i64
                                    as size_t;
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_mul(2 as i32 as u64),
                                    b"format_tree\0" as *const u8 as *const i8,
                                    b"obuf\0" as *const u8 as *const i8,
                                    b"builtin.c\0" as *const u8 as *const i8,
                                    1512 as i32,
                                ) as *mut i8;
                                ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                                osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                    as size_t;
                                obufout = obuf.offset(olen_3 as isize);
                            }
                            let fresh14 = obufout;
                            obufout = obufout.offset(1);
                            *fresh14 = *fill;
                            ofre = ofre.wrapping_sub(1);
                            ofre;
                            fw -= 1;
                            fw;
                        }
                    }
                    copy_count = prec as size_t;
                    if !(fw == 0 as i32 as i64 && !have_prec) {
                        if gawk_mb_cur_max > 1 as i32 {
                            if cs1 == 's' as i32 {
                                copy_count = mbc_byte_count(
                                    (*arg).sub.val.sp,
                                    prec as size_t,
                                );
                            }
                        }
                    }
                    if copy_count != 0 {
                        while copy_count > ofre {
                            let mut olen_4: size_t = obufout.offset_from(obuf) as i64
                                as size_t;
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_mul(2 as i32 as u64),
                                b"format_tree\0" as *const u8 as *const i8,
                                b"obuf\0" as *const u8 as *const i8,
                                b"builtin.c\0" as *const u8 as *const i8,
                                1528 as i32,
                            ) as *mut i8;
                            ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                            osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                as size_t;
                            obufout = obuf.offset(olen_4 as isize);
                        }
                        memcpy(
                            obufout as *mut libc::c_void,
                            cp as *const libc::c_void,
                            copy_count,
                        );
                        obufout = obufout.offset(copy_count as isize);
                        ofre = (ofre as u64).wrapping_sub(copy_count) as size_t
                            as size_t;
                    }
                    while fw > prec {
                        if ofre < 1 as i32 as u64 {
                            let mut olen_5: size_t = obufout.offset_from(obuf) as i64
                                as size_t;
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_mul(2 as i32 as u64),
                                b"format_tree\0" as *const u8 as *const i8,
                                b"obuf\0" as *const u8 as *const i8,
                                b"builtin.c\0" as *const u8 as *const i8,
                                1530 as i32,
                            ) as *mut i8;
                            ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                            osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                as size_t;
                            obufout = obuf.offset(olen_5 as isize);
                        }
                        let fresh15 = obufout;
                        obufout = obufout.offset(1);
                        *fresh15 = *fill;
                        ofre = ofre.wrapping_sub(1);
                        ofre;
                        fw -= 1;
                        fw;
                    }
                    s0 = s1;
                    current_block = 2749501120807187827;
                }
                12041914390200727874 => {
                    nan_inf_val = format_nan_inf(arg, cs1 as i8);
                    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
                        || magic_posix_flag as i32 != 0 || nan_inf_val.is_null()
                    {
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            && do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32
                                == 0 && !magic_posix_flag
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const i8,
                                1544 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"[s]printf: value %g is out of range for `%%%c' format\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                tmpval,
                                cs1,
                            );
                        }
                        tmpval = (*arg).sub.val.fltnum;
                        if (strchr(b"aAeEfFgG\0" as *const u8 as *const i8, cs1))
                            .is_null()
                        {
                            cs1 = 'g' as i32;
                        }
                        current_block = 17832854953326123394;
                    } else {
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const i8,
                                1552 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"[s]printf: value %s is out of range for `%%%c' format\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                nan_inf_val,
                                cs1,
                            );
                        }
                        if strlen(nan_inf_val) != 0 {
                            while strlen(nan_inf_val) > ofre {
                                let mut olen_6: size_t = obufout.offset_from(obuf) as i64
                                    as size_t;
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_mul(2 as i32 as u64),
                                    b"format_tree\0" as *const u8 as *const i8,
                                    b"obuf\0" as *const u8 as *const i8,
                                    b"builtin.c\0" as *const u8 as *const i8,
                                    1554 as i32,
                                ) as *mut i8;
                                ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                                osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                                    as size_t;
                                obufout = obuf.offset(olen_6 as isize);
                            }
                            memcpy(
                                obufout as *mut libc::c_void,
                                nan_inf_val as *const libc::c_void,
                                strlen(nan_inf_val),
                            );
                            obufout = obufout.offset(strlen(nan_inf_val) as isize);
                            ofre = (ofre as u64).wrapping_sub(strlen(nan_inf_val))
                                as size_t as size_t;
                        }
                        s0 = s1;
                        current_block = 2749501120807187827;
                    }
                }
                _ => {}
            }
            match current_block {
                17832854953326123394 => {
                    if !have_prec {
                        prec = 6 as i32 as i64;
                    }
                    if (fw + prec + 11 as i32 as i64) as u64 >= ofre {
                        let mut olen_7: size_t = obufout.offset_from(obuf) as i64
                            as size_t;
                        let mut delta: size_t = osiz
                            .wrapping_add(fw as u64)
                            .wrapping_add(prec as u64)
                            .wrapping_add(11 as i32 as u64)
                            .wrapping_sub(ofre);
                        obuf = erealloc_real(
                            obuf as *mut libc::c_void,
                            osiz.wrapping_add(delta),
                            b"format_tree\0" as *const u8 as *const i8,
                            b"obuf\0" as *const u8 as *const i8,
                            b"builtin.c\0" as *const u8 as *const i8,
                            1607 as i32,
                        ) as *mut i8;
                        obufout = obuf.offset(olen_7 as isize);
                        ofre = (ofre as u64).wrapping_add(delta) as size_t as size_t;
                        osiz = (osiz as u64).wrapping_add(delta) as size_t as size_t;
                    }
                    cp = cpbufs[0 as i32 as usize].buf;
                    let fresh16 = cp;
                    cp = cp.offset(1);
                    *fresh16 = '%' as i32 as i8;
                    if lj {
                        let fresh17 = cp;
                        cp = cp.offset(1);
                        *fresh17 = '-' as i32 as i8;
                    }
                    if signchar != 0 {
                        let fresh18 = cp;
                        cp = cp.offset(1);
                        *fresh18 = signchar;
                    }
                    if alt {
                        let fresh19 = cp;
                        cp = cp.offset(1);
                        *fresh19 = '#' as i32 as i8;
                    }
                    if zero_flag {
                        let fresh20 = cp;
                        cp = cp.offset(1);
                        *fresh20 = '0' as i32 as i8;
                    }
                    if quote_flag {
                        let fresh21 = cp;
                        cp = cp.offset(1);
                        *fresh21 = '\'' as i32 as i8;
                    }
                    if quote_flag as i32 != 0 && use_lc_numeric == 0 {
                        setlocale(1 as i32, b"\0" as *const u8 as *const i8);
                    }
                    need_to_add_thousands = 0 as i32 != 0;
                    match fmt_type as u32 {
                        _ => {}
                    }
                    if have_prec as i32 != 0
                        || ({
                            let mut __res: i32 = 0;
                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                if 0 != 0 {
                                    let mut __c: i32 = cs1;
                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower(cs1);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc()).offset(cs1 as isize);
                            }
                            __res
                        }) != 'a' as i32
                    {
                        sprintf(cp, b"*.*%c\0" as *const u8 as *const i8, cs1);
                        loop {
                            nc = snprintf(
                                obufout,
                                ofre,
                                cpbufs[0 as i32 as usize].buf,
                                fw as i32,
                                prec as i32,
                                tmpval,
                            );
                            if !(nc >= ofre as i32) {
                                break;
                            }
                            if nc as u64 >= ofre {
                                let mut olen_8: size_t = obufout.offset_from(obuf) as i64
                                    as size_t;
                                let mut delta_0: size_t = osiz
                                    .wrapping_add(nc as u64)
                                    .wrapping_sub(ofre);
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_add(delta_0),
                                    b"format_tree\0" as *const u8 as *const i8,
                                    b"obuf\0" as *const u8 as *const i8,
                                    b"builtin.c\0" as *const u8 as *const i8,
                                    1656 as i32,
                                ) as *mut i8;
                                obufout = obuf.offset(olen_8 as isize);
                                ofre = (ofre as u64).wrapping_add(delta_0) as size_t
                                    as size_t;
                                osiz = (osiz as u64).wrapping_add(delta_0) as size_t
                                    as size_t;
                            }
                        }
                    } else {
                        sprintf(cp, b"*%c\0" as *const u8 as *const i8, cs1);
                        loop {
                            nc = snprintf(
                                obufout,
                                ofre,
                                cpbufs[0 as i32 as usize].buf,
                                fw as i32,
                                tmpval,
                            );
                            if !(nc >= ofre as i32) {
                                break;
                            }
                            if nc as u64 >= ofre {
                                let mut olen_9: size_t = obufout.offset_from(obuf) as i64
                                    as size_t;
                                let mut delta_1: size_t = osiz
                                    .wrapping_add(nc as u64)
                                    .wrapping_sub(ofre);
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_add(delta_1),
                                    b"format_tree\0" as *const u8 as *const i8,
                                    b"obuf\0" as *const u8 as *const i8,
                                    b"builtin.c\0" as *const u8 as *const i8,
                                    1664 as i32,
                                ) as *mut i8;
                                obufout = obuf.offset(olen_9 as isize);
                                ofre = (ofre as u64).wrapping_add(delta_1) as size_t
                                    as size_t;
                                osiz = (osiz as u64).wrapping_add(delta_1) as size_t
                                    as size_t;
                            }
                        }
                    }
                    if quote_flag as i32 != 0 && use_lc_numeric == 0 {
                        setlocale(1 as i32, b"C\0" as *const u8 as *const i8);
                    }
                    len = strlen(obufout);
                    if quote_flag as i32 != 0 && need_to_add_thousands as i32 != 0 {
                        let mut new_text: *const i8 = add_thousands(obufout, &mut loc);
                        len = strlen(new_text);
                        if len >= ofre {
                            let mut olen_10: size_t = obufout.offset_from(obuf) as i64
                                as size_t;
                            let mut delta_2: size_t = osiz
                                .wrapping_add(len)
                                .wrapping_sub(ofre);
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_add(delta_2),
                                b"format_tree\0" as *const u8 as *const i8,
                                b"obuf\0" as *const u8 as *const i8,
                                b"builtin.c\0" as *const u8 as *const i8,
                                1677 as i32,
                            ) as *mut i8;
                            obufout = obuf.offset(olen_10 as isize);
                            ofre = (ofre as u64).wrapping_add(delta_2) as size_t
                                as size_t;
                            osiz = (osiz as u64).wrapping_add(delta_2) as size_t
                                as size_t;
                        }
                        strcpy(obufout, new_text);
                        pma_free(new_text as *mut libc::c_void);
                    }
                    ofre = (ofre as u64).wrapping_sub(len) as size_t as size_t;
                    obufout = obufout.offset(len as isize);
                    s0 = s1;
                }
                _ => {}
            }
            if !toofew {
                continue;
            }
            msg(
                b"%s\n\t`%s'\n\t%*s%s\0" as *const u8 as *const i8,
                dcgettext(
                    0 as *const i8,
                    b"fatal: not enough arguments to satisfy format string\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                fmt_string,
                (s1.offset_from(fmt_string) as i64 - 1 as i32 as i64) as i32,
                b"\0" as *const u8 as *const i8,
                dcgettext(
                    0 as *const i8,
                    b"^ ran out for this one\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            current_block = 6720548472269345400;
            break;
        }
    }
    match current_block {
        2782926371273512654 => {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                if need_format {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1701 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"[s]printf: format specifier does not have control letter\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                if cur_arg < num_args as u64 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1704 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"too many arguments supplied for format string\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
            if s1.offset_from(s0) as i64 != 0 {
                while s1.offset_from(s0) as i64 as u64 > ofre {
                    let mut olen_11: size_t = obufout.offset_from(obuf) as i64 as size_t;
                    obuf = erealloc_real(
                        obuf as *mut libc::c_void,
                        osiz.wrapping_mul(2 as i32 as u64),
                        b"format_tree\0" as *const u8 as *const i8,
                        b"obuf\0" as *const u8 as *const i8,
                        b"builtin.c\0" as *const u8 as *const i8,
                        1707 as i32,
                    ) as *mut i8;
                    ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                    osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                        as size_t;
                    obufout = obuf.offset(olen_11 as isize);
                }
                memcpy(
                    obufout as *mut libc::c_void,
                    s0 as *const libc::c_void,
                    s1.offset_from(s0) as i64 as size_t,
                );
                obufout = obufout.offset(s1.offset_from(s0) as i64 as isize);
                ofre = (ofre as u64).wrapping_sub(s1.offset_from(s0) as i64 as u64)
                    as size_t as size_t;
            }
            olen_final = obufout.offset_from(obuf) as i64 as size_t;
            if ofre > (64 as i32 * 2 as i32) as u64 {
                obuf = erealloc_real(
                    obuf as *mut libc::c_void,
                    olen_final.wrapping_add(1 as i32 as u64),
                    b"format_tree\0" as *const u8 as *const i8,
                    b"obuf\0" as *const u8 as *const i8,
                    b"builtin.c\0" as *const u8 as *const i8,
                    1711 as i32,
                ) as *mut i8;
            }
            r = make_str_node(obuf, olen_final, 2 as i32);
            obuf = 0 as *mut i8;
        }
        _ => {}
    }
    let mut k_2: size_t = 0;
    let mut count_1: size_t = (::core::mem::size_of::<[C2RustUnnamed_12; 2]>() as u64)
        .wrapping_div(::core::mem::size_of::<C2RustUnnamed_12>() as u64);
    k_2 = 0 as i32 as size_t;
    while k_2 < count_1 {
        if cpbufs[k_2 as usize].buf != (cpbufs[k_2 as usize].stackbuf).as_mut_ptr() {
            pma_free(cpbufs[k_2 as usize].buf as *mut libc::c_void);
        }
        k_2 = k_2.wrapping_add(1);
        k_2;
    }
    if !obuf.is_null() {
        pma_free(obuf as *mut libc::c_void);
    }
    if r.is_null() {
        gawk_exit(2 as i32);
    }
    return r;
}
unsafe extern "C" fn printf_common(mut nargs: i32) -> *mut NODE {
    let mut i: i32 = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    i = 1 as i32;
    while i <= nargs {
        let fresh22 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        let ref mut fresh23 = *args_array.offset((nargs - i) as isize);
        *fresh23 = (*fresh22).rptr;
        tmp = *fresh23;
        if (*tmp).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            loop {
                i -= 1;
                if !(i > 0 as i32) {
                    break;
                }
                DEREF(*args_array.offset((nargs - i) as isize));
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1746 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                array_vname(tmp),
            );
        }
        i += 1;
        i;
    }
    let ref mut fresh24 = *args_array.offset(0 as i32 as isize);
    *fresh24 = force_string_fmt(
        *args_array.offset(0 as i32 as isize),
        CONVFMT,
        CONVFMTidx,
    );
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(*args_array.offset(0 as i32 as isize))).flags as u32
            & flagvals::STRING as i32 as u32 == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1752 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string format string argument\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            b"printf/sprintf\0" as *const u8 as *const i8,
        );
    }
    r = format_tree(
        (**args_array.offset(0 as i32 as isize)).sub.val.sp,
        (**args_array.offset(0 as i32 as isize)).sub.val.slen,
        args_array,
        nargs as i64,
    );
    i = 0 as i32;
    while i < nargs {
        DEREF(*args_array.offset(i as isize));
        i += 1;
        i;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn do_sprintf(mut nargs: i32) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    if nargs == 0 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1767 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"sprintf: no arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    r = printf_common(nargs);
    if r.is_null() {
        gawk_exit(2 as i32);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn do_printf(mut nargs: i32, mut redirtype: i32) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut errflg: i32 = 0 as i32;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    if nargs == 0 as i32 {
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1790 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"printf: no arguments\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            if redirtype != 0 as i32 {
                redir_exp = (*stack_ptr).rptr;
                if (*redir_exp).type_0 as u32 != nodevals::Node_val as i32 as u32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1794 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use array `%s' in a scalar context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(redir_exp),
                    );
                }
                rp = redirect(redir_exp, redirtype, &mut errflg, 1 as i32 != 0);
                DEREF(redir_exp);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
            }
            return;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1801 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"printf: no arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if redirtype != 0 as i32 {
        redir_exp = (*stack_ptr.offset(-(nargs as isize))).rptr;
        if (*redir_exp).type_0 as u32 != nodevals::Node_val as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1807 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                array_vname(redir_exp),
            );
        }
        rp = redirect(redir_exp, redirtype, &mut errflg, 1 as i32 != 0);
        if !rp.is_null() {
            if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                != 0 as i32 as u32 && ((*rp).output.fp).is_null()
            {
                if is_non_fatal_redirect(
                    (*redir_exp).sub.val.sp,
                    (*redir_exp).sub.val.slen,
                ) {
                    update_ERRNO_int(9 as i32);
                    return;
                }
                close_rp(rp, two_way_close_type::CLOSE_ALL);
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1816 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"printf: attempt to write to closed write end of two-way pipe\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            fp = (*rp).output.fp;
        } else if errflg != 0 {
            update_ERRNO_int(errflg);
            return;
        }
    } else if do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0 {
        fp = output_fp;
    } else {
        fp = stdout;
    }
    tmp = printf_common(nargs);
    if !redir_exp.is_null() {
        DEREF(redir_exp);
        stack_ptr = stack_ptr.offset(-1);
        stack_ptr;
    }
    if !tmp.is_null() {
        if fp.is_null() {
            DEREF(tmp);
            return;
        }
        efwrite(
            (*tmp).sub.val.sp as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            (*tmp).sub.val.slen,
            fp,
            b"printf\0" as *const u8 as *const i8,
            rp,
            1 as i32 != 0,
        );
        if !rp.is_null()
            && (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                != 0 as i32 as u32
        {
            ((*rp).output.gawk_fflush)
                .expect(
                    "non-null function pointer",
                )((*rp).output.fp, (*rp).output.opaque);
        }
        DEREF(tmp);
    } else {
        gawk_exit(2 as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_sqrt(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut arg: libc::c_double = 0.;
    check_exact_args(nargs, b"sqrt\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1859 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"sqrt\0" as *const u8 as *const i8,
        );
    }
    arg = (*force_number(tmp)).sub.val.fltnum;
    DEREF(tmp);
    if arg < 0.0f64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1863 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received negative argument %g\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"sqrt\0" as *const u8 as *const i8,
            arg,
        );
    }
    return make_number.expect("non-null function pointer")(sqrt(arg));
}
#[no_mangle]
pub unsafe extern "C" fn do_substr(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut indx: size_t = 0;
    let mut length: size_t = 0 as i32 as size_t;
    let mut d_index: libc::c_double = 0 as i32 as libc::c_double;
    let mut d_length: libc::c_double = 0 as i32 as libc::c_double;
    let mut src_len: size_t = 0;
    check_args_min_max(nargs, b"substr\0" as *const u8 as *const i8, 2 as i32, 3 as i32);
    if nargs == 3 as i32 {
        t1 = force_number(POP_SCALAR());
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(t1)).flags as u32 & flagvals::NUMBER as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1884 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric third argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"substr\0" as *const u8 as *const i8,
            );
        }
        d_length = (*t1).sub.val.fltnum;
        DEREF(t1);
    }
    t1 = force_number(POP_SCALAR());
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1891 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric second argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"substr\0" as *const u8 as *const i8,
        );
    }
    d_index = (*t1).sub.val.fltnum;
    DEREF(t1);
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32
            & (flagvals::STRING as i32 | flagvals::USER_INPUT as i32) as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1897 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string first argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"substr\0" as *const u8 as *const i8,
        );
    }
    if nargs == 3 as i32 {
        if !(d_length >= 1 as i32 as libc::c_double) {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32
                == do_flag_values::DO_LINT_ALL as i32 as u32
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1902 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"substr: length %g is not >= 1\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    d_length,
                );
            } else if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32
                == do_flag_values::DO_LINT_INVALID as i32 as u32
                && !(d_length >= 0 as i32 as libc::c_double)
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1904 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"substr: length %g is not >= 0\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    d_length,
                );
            }
            DEREF(t1);
            return make_str_node(
                b"\0" as *const u8 as *const i8,
                0 as i32 as size_t,
                0 as i32,
            );
        }
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            if double_to_int(d_length) != d_length {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1917 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"substr: non-integer length %g will be truncated\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    d_length,
                );
            }
            if d_length > 18446744073709551615 as u64 as libc::c_double {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1922 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"substr: length %g too big for string indexing, truncating to %g\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    d_length,
                    18446744073709551615 as u64 as libc::c_double,
                );
            }
        }
        if d_length < 18446744073709551615 as u64 as libc::c_double {
            length = d_length as size_t;
        } else {
            length = 18446744073709551615 as u64;
        }
    }
    if !(d_index >= 1 as i32 as libc::c_double) {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1935 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"substr: start index %g is invalid, using 1\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                d_index,
            );
        }
        d_index = 1 as i32 as libc::c_double;
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && double_to_int(d_index) != d_index
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1940 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"substr: non-integer start index %g will be truncated\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            d_index,
        );
    }
    if d_index <= 18446744073709551615 as u64 as libc::c_double {
        indx = (d_index - 1 as i32 as libc::c_double) as size_t;
    } else {
        indx = 18446744073709551615 as u64;
    }
    if nargs == 2 as i32 {
        length = ((*t1).sub.val.slen).wrapping_sub(indx);
        if gawk_mb_cur_max > 1 as i32 {
            t1 = str2wstr(t1, 0 as *mut *mut size_t);
            if (*t1).sub.val.wslen > 0 as i32 as u64 {
                length = ((*t1).sub.val.wslen).wrapping_sub(indx);
            }
        }
        d_length = length as libc::c_double;
    }
    if (*t1).sub.val.slen == 0 as i32 as u64 {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32
                == do_flag_values::DO_LINT_ALL as i32 as u32
                || indx | length != 0 as i32 as u64)
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1963 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"substr: source string is zero length\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        DEREF(t1);
        return make_str_node(
            b"\0" as *const u8 as *const i8,
            0 as i32 as size_t,
            0 as i32,
        );
    }
    if gawk_mb_cur_max > 1 as i32 {
        t1 = str2wstr(t1, 0 as *mut *mut size_t);
        src_len = (*t1).sub.val.wslen;
    } else {
        src_len = (*t1).sub.val.slen;
    }
    if indx >= src_len {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1977 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"substr: start index %g is past end of string\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                d_index,
            );
        }
        DEREF(t1);
        return make_str_node(
            b"\0" as *const u8 as *const i8,
            0 as i32 as size_t,
            0 as i32,
        );
    }
    if length > src_len.wrapping_sub(indx) {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 1984 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"substr: length %g at start index %g exceeds length of first argument (%lu)\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                d_length,
                d_index,
                src_len,
            );
        }
        length = src_len.wrapping_sub(indx);
    }
    if gawk_mb_cur_max == 1 as i32 || (*t1).sub.val.wslen == (*t1).sub.val.slen {
        r = make_str_node(((*t1).sub.val.sp).offset(indx as isize), length, 0 as i32);
    } else {
        let mut result: size_t = 0;
        let mut wp: *mut wchar_t = 0 as *mut wchar_t;
        let mut mbs: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        let mut substr: *mut i8 = 0 as *mut i8;
        let mut cp: *mut i8 = 0 as *mut i8;
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        substr = emalloc_real(
            length.wrapping_mul(gawk_mb_cur_max as u64).wrapping_add(1 as i32 as u64),
            b"do_substr\0" as *const u8 as *const i8,
            b"substr\0" as *const u8 as *const i8,
            b"builtin.c\0" as *const u8 as *const i8,
            2007 as i32,
        ) as *mut i8;
        wp = ((*t1).sub.val.wsp).offset(indx as isize);
        cp = substr;
        while length > 0 as i32 as u64 {
            result = wcrtomb(cp, *wp, &mut mbs);
            if result == -(1 as i32) as size_t {
                break;
            }
            cp = cp.offset(result as isize);
            wp = wp.offset(1);
            wp;
            length = length.wrapping_sub(1);
            length;
        }
        *cp = '\0' as i32 as i8;
        r = make_str_node(substr, cp.offset_from(substr) as i64 as size_t, 2 as i32);
    }
    DEREF(t1);
    return r;
}
static mut time_t_min: time_t = 0;
static mut time_t_max: time_t = 0;
#[no_mangle]
pub unsafe extern "C" fn do_strftime(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut t3: *mut NODE = 0 as *mut NODE;
    let mut ret: *mut NODE = 0 as *mut NODE;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut fclock: time_t = 0;
    let mut clock_val: libc::c_double = 0.;
    let mut bufp: *mut i8 = 0 as *mut i8;
    let mut buflen: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut buf: [i8; 8192] = [0; 8192];
    let mut format: *const i8 = 0 as *const i8;
    let mut formatlen: i32 = 0;
    let mut do_gmt: bool = false;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut save: i8 = '\0' as i32 as i8;
    format = def_strftime_format.as_ptr();
    formatlen = strlen(format) as i32;
    time(&mut fclock);
    do_gmt = 0 as i32 != 0;
    check_args_min_max(
        nargs,
        b"strftime\0" as *const u8 as *const i8,
        0 as i32,
        3 as i32,
    );
    if !PROCINFO_node.is_null() {
        sub = make_str_node(
            b"strftime\0" as *const u8 as *const i8,
            8 as i32 as size_t,
            0 as i32,
        );
        val = in_array(PROCINFO_node, sub);
        unref(sub);
        if !val.is_null() {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                && (*fixtype(val)).flags as u32
                    & (flagvals::STRING as i32 | flagvals::USER_INPUT as i32) as u32
                    == 0 as i32 as u32
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2060 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"strftime: format value in PROCINFO[\"strftime\"] has numeric type\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            val = force_string_fmt(val, CONVFMT, CONVFMTidx);
            format = (*val).sub.val.sp;
            formatlen = (*val).sub.val.slen as i32;
        }
    }
    t3 = 0 as *mut NODE;
    t2 = t3;
    t1 = t2;
    if nargs > 0 as i32 {
        let mut tmp: *mut NODE = 0 as *mut NODE;
        if nargs == 3 as i32 {
            t3 = POP_SCALAR();
            do_gmt = boolval(t3);
            DEREF(t3);
        }
        if nargs >= 2 as i32 {
            t2 = POP_SCALAR();
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                && (*fixtype(t2)).flags as u32 & flagvals::NUMBER as i32 as u32
                    == 0 as i32 as u32
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2081 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"%s: received non-numeric second argument\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    b"strftime\0" as *const u8 as *const i8,
                );
            }
            force_number(t2);
            clock_val = (*t2).sub.val.fltnum;
            fclock = clock_val as time_t;
            if clock_val < 0 as i32 as libc::c_double && fclock > 0 as i32 as i64 {
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2091 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"strftime: second argument less than 0 or too big for time_t\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                return make_str_node(
                    b"\0" as *const u8 as *const i8,
                    0 as i32 as size_t,
                    0 as i32,
                );
            }
            if clock_val < time_t_min as libc::c_double
                || clock_val > time_t_max as libc::c_double
            {
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2098 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"strftime: second argument out of range for time_t\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                return make_str_node(
                    b"\0" as *const u8 as *const i8,
                    0 as i32 as size_t,
                    0 as i32,
                );
            }
            DEREF(t2);
        }
        tmp = POP_SCALAR();
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(tmp)).flags as u32 & flagvals::STRING as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2107 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string first argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"strftime\0" as *const u8 as *const i8,
            );
        }
        t1 = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        format = (*t1).sub.val.sp;
        formatlen = (*t1).sub.val.slen as i32;
        if formatlen == 0 as i32 {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2114 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"strftime: received empty format string\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            DEREF(t1);
            return make_str_node(
                b"\0" as *const u8 as *const i8,
                0 as i32 as size_t,
                0 as i32,
            );
        }
        str_terminate_f(t1, &mut save);
    }
    if do_gmt {
        tm = gmtime(&mut fclock);
    } else {
        tm = localtime(&mut fclock);
    }
    if tm.is_null() {
        ret = make_str_node(
            b"\0" as *const u8 as *const i8,
            0 as i32 as size_t,
            0 as i32,
        );
    } else {
        bufp = buf.as_mut_ptr();
        bufsize = ::core::mem::size_of::<[i8; 8192]>() as u64;
        loop {
            *bufp = '\0' as i32 as i8;
            buflen = strftime(bufp, bufsize, format, tm);
            if buflen > 0 as i32 as u64 || bufsize >= (1024 as i32 * formatlen) as u64 {
                break;
            }
            bufsize = (bufsize as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
            if bufp == buf.as_mut_ptr() {
                bufp = emalloc_real(
                    bufsize,
                    b"do_strftime\0" as *const u8 as *const i8,
                    b"bufp\0" as *const u8 as *const i8,
                    b"builtin.c\0" as *const u8 as *const i8,
                    2148 as i32,
                ) as *mut i8;
            } else {
                bufp = erealloc_real(
                    bufp as *mut libc::c_void,
                    bufsize,
                    b"do_strftime\0" as *const u8 as *const i8,
                    b"bufp\0" as *const u8 as *const i8,
                    b"builtin.c\0" as *const u8 as *const i8,
                    2150 as i32,
                ) as *mut i8;
            }
        }
        ret = make_str_node(bufp, buflen, 0 as i32);
        if bufp != buf.as_mut_ptr() {
            pma_free(bufp as *mut libc::c_void);
        }
    }
    if !t1.is_null() {
        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save;
        DEREF(t1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn do_systime(mut nargs: i32) -> *mut NODE {
    let mut lclock: time_t = 0;
    check_exact_args(nargs, b"systime\0" as *const u8 as *const i8, 0 as i32);
    time(&mut lclock);
    return make_number.expect("non-null function pointer")(lclock as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_mktime(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut then: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    let mut year: i64 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;
    let mut second: i32 = 0;
    let mut count: i32 = 0;
    let mut dst: i32 = -(1 as i32);
    let mut then_stamp: time_t = 0;
    let mut save: i8 = 0;
    let mut do_gmt: bool = false;
    check_args_min_max(nargs, b"mktime\0" as *const u8 as *const i8, 1 as i32, 2 as i32);
    if nargs == 2 as i32 {
        t2 = POP_SCALAR();
        do_gmt = boolval(t2);
        DEREF(t2);
    } else {
        do_gmt = 0 as i32 != 0;
    }
    t1 = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2201 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"mktime\0" as *const u8 as *const i8,
        );
    }
    t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
    save = *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize);
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = '\0' as i32 as i8;
    count = sscanf(
        (*t1).sub.val.sp,
        b"%ld %d %d %d %d %d %d\0" as *const u8 as *const i8,
        &mut year as *mut i64,
        &mut month as *mut i32,
        &mut day as *mut i32,
        &mut hour as *mut i32,
        &mut minute as *mut i32,
        &mut second as *mut i32,
        &mut dst as *mut i32,
    );
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (second < 0 as i32 || second > 60 as i32
            || (minute < 0 as i32 || minute > 59 as i32)
            || (hour < 0 as i32 || hour > 23 as i32)
            || (day < 1 as i32 || day > 31 as i32)
            || (month < 1 as i32 || month > 12 as i32))
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2220 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"mktime: at least one of the values is out of the default range\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save;
    DEREF(t1);
    if count < 6 as i32 || month == -(2147483647 as i32) - 1 as i32
        || year < (-(2147483647 as i32) - 1 as i32 + 1900 as i32) as i64
        || year - 1900 as i32 as i64 > 2147483647 as i32 as i64
    {
        return make_number
            .expect("non-null function pointer")(-(1 as i32) as libc::c_double);
    }
    memset(
        &mut then as *mut tm as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<tm>() as u64,
    );
    then.tm_sec = second;
    then.tm_min = minute;
    then.tm_hour = hour;
    then.tm_mday = day;
    then.tm_mon = month - 1 as i32;
    then.tm_year = (year - 1900 as i32 as i64) as i32;
    then.tm_isdst = dst;
    then_stamp = if do_gmt as i32 != 0 { timegm(&mut then) } else { mktime(&mut then) };
    return make_number.expect("non-null function pointer")(then_stamp as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_system(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_double = 0 as i32 as libc::c_double;
    let mut cmd: *mut i8 = 0 as *mut i8;
    let mut save: i8 = 0;
    let mut status: i32 = 0;
    check_exact_args(nargs, b"system\0" as *const u8 as *const i8, 1 as i32);
    if do_flags as u32 & do_flag_values::DO_SANDBOX as i32 as u32 != 0 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2258 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"'system' function not allowed in sandbox mode\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    flush_io();
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2263 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"system\0" as *const u8 as *const i8,
        );
    }
    cmd = (*force_string_fmt(tmp, CONVFMT, CONVFMTidx)).sub.val.sp;
    if !cmd.is_null() && *cmd as i32 != 0 {
        save = *cmd.offset((*tmp).sub.val.slen as isize);
        *cmd.offset((*tmp).sub.val.slen as isize) = '\0' as i32 as i8;
        os_restore_mode(fileno(stdin));
        signal(13 as i32, None);
        status = system(cmd);
        ret = status as libc::c_double;
        if status != -(1 as i32) {
            if !(do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0) {
                if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
                    ret = status as libc::c_double / 256.0f64;
                } else {
                    ret = sanitize_exit_status(status) as libc::c_double;
                }
            }
        }
        if BINMODE & binmode_values::BINMODE_INPUT as i32 != 0 as i32 {
            os_setbinmode(fileno(stdin), 0 as i32);
        }
        signal(
            13 as i32,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as i32 as libc::intptr_t),
        );
        *cmd.offset((*tmp).sub.val.slen as isize) = save;
    }
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(ret);
}
#[no_mangle]
pub unsafe extern "C" fn do_print(mut nargs: i32, mut redirtype: i32) {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut errflg: i32 = 0 as i32;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if redirtype != 0 as i32 {
        redir_exp = (*stack_ptr.offset(-(nargs as isize))).rptr;
        if (*redir_exp).type_0 as u32 != nodevals::Node_val as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2323 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                array_vname(redir_exp),
            );
        }
        rp = redirect(redir_exp, redirtype, &mut errflg, 1 as i32 != 0);
        if !rp.is_null() {
            if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                != 0 as i32 as u32 && ((*rp).output.fp).is_null()
            {
                if is_non_fatal_redirect(
                    (*redir_exp).sub.val.sp,
                    (*redir_exp).sub.val.slen,
                ) {
                    update_ERRNO_int(9 as i32);
                    return;
                }
                close_rp(rp, two_way_close_type::CLOSE_ALL);
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2332 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"print: attempt to write to closed write end of two-way pipe\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            fp = (*rp).output.fp;
        } else if errflg != 0 {
            update_ERRNO_int(errflg);
            return;
        }
    } else if do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0 {
        fp = output_fp;
    } else {
        fp = stdout;
    }
    i = 1 as i32;
    while i <= nargs {
        let fresh25 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        let ref mut fresh26 = *args_array.offset(i as isize);
        *fresh26 = (*fresh25).rptr;
        tmp = *fresh26;
        if (*tmp).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            loop {
                i -= 1;
                if !(i > 0 as i32) {
                    break;
                }
                DEREF(*args_array.offset(i as isize));
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2350 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                array_vname(tmp),
            );
        }
        let ref mut fresh27 = *args_array.offset(i as isize);
        *fresh27 = force_string_fmt(tmp, OFMT, OFMTidx);
        if *args_array.offset(i as isize) != tmp {
            DEREF(tmp);
        }
        i += 1;
        i;
    }
    if !redir_exp.is_null() {
        DEREF(redir_exp);
        stack_ptr = stack_ptr.offset(-1);
        stack_ptr;
    }
    if fp.is_null() {
        i = nargs;
        while i > 0 as i32 {
            DEREF(*args_array.offset(i as isize));
            i -= 1;
            i;
        }
        return;
    }
    i = nargs;
    while i > 0 as i32 {
        efwrite(
            (**args_array.offset(i as isize)).sub.val.sp as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            (**args_array.offset(i as isize)).sub.val.slen,
            fp,
            b"print\0" as *const u8 as *const i8,
            rp,
            0 as i32 != 0,
        );
        DEREF(*args_array.offset(i as isize));
        if i != 1 as i32 && OFSlen > 0 as i32 {
            efwrite(
                OFS as *const libc::c_void,
                ::core::mem::size_of::<i8>() as u64,
                OFSlen as size_t,
                fp,
                b"print\0" as *const u8 as *const i8,
                rp,
                0 as i32 != 0,
            );
        }
        i -= 1;
        i;
    }
    if ORSlen > 0 as i32 {
        efwrite(
            ORS as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            ORSlen as size_t,
            fp,
            b"print\0" as *const u8 as *const i8,
            rp,
            1 as i32 != 0,
        );
    }
    if !rp.is_null()
        && (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
            != 0 as i32 as u32
    {
        ((*rp).output.gawk_fflush)
            .expect("non-null function pointer")((*rp).output.fp, (*rp).output.opaque);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_print_rec(mut nargs: i32, mut redirtype: i32) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut f0: *mut NODE = 0 as *mut NODE;
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut errflg: i32 = 0 as i32;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    if redirtype != 0 as i32 {
        redir_exp = (*stack_ptr).rptr;
        rp = redirect(redir_exp, redirtype, &mut errflg, 1 as i32 != 0);
        if !rp.is_null() {
            if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                != 0 as i32 as u32 && ((*rp).output.fp).is_null()
            {
                if is_non_fatal_redirect(
                    (*redir_exp).sub.val.sp,
                    (*redir_exp).sub.val.slen,
                ) {
                    update_ERRNO_int(9 as i32);
                    return;
                }
                close_rp(rp, two_way_close_type::CLOSE_ALL);
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2407 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"print: attempt to write to closed write end of two-way pipe\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            fp = (*rp).output.fp;
        }
        DEREF(redir_exp);
        stack_ptr = stack_ptr.offset(-1);
        stack_ptr;
    } else {
        fp = output_fp;
    }
    if errflg != 0 {
        update_ERRNO_int(errflg);
        return;
    }
    if fp.is_null() {
        return;
    }
    if !field0_valid
        || do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
    {
        get_field(0 as i64, 0 as *mut Func_ptr);
    }
    f0 = *fields_arr.offset(0 as i32 as isize);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*f0).flags as u32 & flagvals::NULL_FIELD as i32 as u32 != 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2430 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"reference to uninitialized field `$%d'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            0 as i32,
        );
    }
    efwrite(
        (*f0).sub.val.sp as *const libc::c_void,
        ::core::mem::size_of::<i8>() as u64,
        (*f0).sub.val.slen,
        fp,
        b"print\0" as *const u8 as *const i8,
        rp,
        0 as i32 != 0,
    );
    if ORSlen > 0 as i32 {
        efwrite(
            ORS as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            ORSlen as size_t,
            fp,
            b"print\0" as *const u8 as *const i8,
            rp,
            1 as i32 != 0,
        );
    }
    if !rp.is_null()
        && (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
            != 0 as i32 as u32
    {
        ((*rp).output.gawk_fflush)
            .expect("non-null function pointer")((*rp).output.fp, (*rp).output.opaque);
    }
}
unsafe extern "C" fn is_wupper(mut c: wchar_t) -> i32 {
    return iswupper(c as wint_t);
}
unsafe extern "C" fn is_wlower(mut c: wchar_t) -> i32 {
    return iswlower(c as wint_t);
}
unsafe extern "C" fn to_wlower(mut c: wchar_t) -> i32 {
    return towlower(c as wint_t) as i32;
}
unsafe extern "C" fn to_wupper(mut c: wchar_t) -> i32 {
    return towupper(c as wint_t) as i32;
}
unsafe extern "C" fn wide_change_case(
    mut wstr: *mut wchar_t,
    mut wlen: size_t,
    mut is_x: Option<unsafe extern "C" fn(wchar_t) -> i32>,
    mut to_y: Option<unsafe extern "C" fn(wchar_t) -> i32>,
) {
    let mut i: size_t = 0;
    let mut wcp: *mut wchar_t = 0 as *mut wchar_t;
    i = 0 as i32 as size_t;
    wcp = wstr;
    while i < wlen {
        if is_x.expect("non-null function pointer")(*wcp) != 0 {
            *wcp = to_y.expect("non-null function pointer")(*wcp);
        }
        i = i.wrapping_add(1);
        i;
        wcp = wcp.offset(1);
        wcp;
    }
}
unsafe extern "C" fn wide_toupper(mut wstr: *mut wchar_t, mut wlen: size_t) {
    wide_change_case(
        wstr,
        wlen,
        Some(is_wlower as unsafe extern "C" fn(wchar_t) -> i32),
        Some(to_wupper as unsafe extern "C" fn(wchar_t) -> i32),
    );
}
unsafe extern "C" fn wide_tolower(mut wstr: *mut wchar_t, mut wlen: size_t) {
    wide_change_case(
        wstr,
        wlen,
        Some(is_wupper as unsafe extern "C" fn(wchar_t) -> i32),
        Some(to_wlower as unsafe extern "C" fn(wchar_t) -> i32),
    );
}
#[no_mangle]
pub unsafe extern "C" fn do_tolower(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    check_exact_args(nargs, b"tolower\0" as *const u8 as *const i8, 1 as i32);
    t1 = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2517 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"tolower\0" as *const u8 as *const i8,
        );
    }
    t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
    t2 = make_str_node((*t1).sub.val.sp, (*t1).sub.val.slen, 0 as i32);
    if gawk_mb_cur_max == 1 as i32 {
        let mut cp: *mut u8 = 0 as *mut u8;
        let mut cp2: *mut u8 = 0 as *mut u8;
        cp = (*t2).sub.val.sp as *mut u8;
        cp2 = ((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) as *mut u8;
        while cp < cp2 {
            if *(*__ctype_b_loc()).offset(*cp as i32 as isize) as i32
                & C2RustUnnamed_0::_ISupper as i32 as libc::c_ushort as i32 != 0
            {
                *cp = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *cp as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*cp as i32);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(*cp as i32 as isize);
                    }
                    __res
                }) as u8;
            }
            cp = cp.offset(1);
            cp;
        }
    } else {
        str2wstr(t2, 0 as *mut *mut size_t);
        wide_tolower((*t2).sub.val.wsp, (*t2).sub.val.wslen);
        wstr2str(t2);
    }
    DEREF(t1);
    return t2;
}
#[no_mangle]
pub unsafe extern "C" fn do_toupper(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    check_exact_args(nargs, b"toupper\0" as *const u8 as *const i8, 1 as i32);
    t1 = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2550 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"toupper\0" as *const u8 as *const i8,
        );
    }
    t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
    t2 = make_str_node((*t1).sub.val.sp, (*t1).sub.val.slen, 0 as i32);
    if gawk_mb_cur_max == 1 as i32 {
        let mut cp: *mut u8 = 0 as *mut u8;
        let mut cp2: *mut u8 = 0 as *mut u8;
        cp = (*t2).sub.val.sp as *mut u8;
        cp2 = ((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) as *mut u8;
        while cp < cp2 {
            if *(*__ctype_b_loc()).offset(*cp as i32 as isize) as i32
                & C2RustUnnamed_0::_ISlower as i32 as libc::c_ushort as i32 != 0
            {
                *cp = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *cp as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*cp as i32);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(*cp as i32 as isize);
                    }
                    __res
                }) as u8;
            }
            cp = cp.offset(1);
            cp;
        }
    } else {
        str2wstr(t2, 0 as *mut *mut size_t);
        wide_toupper((*t2).sub.val.wsp, (*t2).sub.val.wslen);
        wstr2str(t2);
    }
    DEREF(t1);
    return t2;
}
#[no_mangle]
pub unsafe extern "C" fn do_atan2(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    check_exact_args(nargs, b"atan2\0" as *const u8 as *const i8, 2 as i32);
    t2 = POP_SCALAR();
    let fresh28 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    t1 = (*fresh28).rptr;
    if (*t1).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        DEREF(t2);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2582 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(t1),
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if (*fixtype(t1)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2585 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric first argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"atan2\0" as *const u8 as *const i8,
            );
        }
        if (*fixtype(t2)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2587 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric second argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"atan2\0" as *const u8 as *const i8,
            );
        }
    }
    d1 = (*force_number(t1)).sub.val.fltnum;
    d2 = (*force_number(t2)).sub.val.fltnum;
    DEREF(t1);
    DEREF(t2);
    return make_number.expect("non-null function pointer")(atan2(d1, d2));
}
#[no_mangle]
pub unsafe extern "C" fn do_sin(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(nargs, b"sin\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2608 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"sin\0" as *const u8 as *const i8,
        );
    }
    d = sin((*force_number(tmp)).sub.val.fltnum);
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(d);
}
#[no_mangle]
pub unsafe extern "C" fn do_cos(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(nargs, b"cos\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2626 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"cos\0" as *const u8 as *const i8,
        );
    }
    d = cos((*force_number(tmp)).sub.val.fltnum);
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(d);
}
static mut firstrand: bool = 1 as i32 != 0;
static mut istate: [gawk_uint32_t; 64] = [0; 64];
static mut state: *mut i8 = unsafe { istate.as_ptr() as *mut _ as *mut i8 };
#[no_mangle]
pub unsafe extern "C" fn do_rand(mut nargs: i32) -> *mut NODE {
    let mut tmprand: libc::c_double = 0.;
    check_exact_args(nargs, b"rand\0" as *const u8 as *const i8, 0 as i32);
    if firstrand {
        gawk_initstate(1 as i32 as u32 as u64, state, 256 as i32 as i64);
        firstrand = 0 as i32 != 0;
        gawk_setstate(state);
    }
    loop {
        let mut d1: i64 = 0;
        let mut d2: i64 = 0;
        d1 = gawk_random();
        d2 = gawk_random();
        tmprand = 0.5f64
            + (d1 as libc::c_double / (0x7fffffff as i64 as libc::c_double + 1.0f64)
                + d2 as libc::c_double) / (0x7fffffff as i64 as libc::c_double + 1.0f64);
        tmprand -= 0.5f64;
        if !(tmprand == 1.0f64) {
            break;
        }
    }
    return make_number.expect("non-null function pointer")(tmprand);
}
#[no_mangle]
pub unsafe extern "C" fn do_srand(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    static mut save_seed: i64 = 1 as i32 as i64;
    let mut ret: i64 = save_seed;
    if firstrand {
        gawk_initstate(1 as i32 as u32 as u64, state, 256 as i32 as i64);
        firstrand = 0 as i32 != 0;
        gawk_setstate(state);
    }
    check_args_min_max(nargs, b"srand\0" as *const u8 as *const i8, 0 as i32, 1 as i32);
    if nargs == 0 as i32 {
        save_seed = time(0 as *mut time_t);
        gawk_srandom(save_seed as u32 as u64);
    } else {
        tmp = POP_SCALAR();
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2745 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"srand\0" as *const u8 as *const i8,
            );
        }
        save_seed = (*force_number(tmp)).sub.val.fltnum as i64;
        gawk_srandom(save_seed as u32 as u64);
        DEREF(tmp);
    }
    return make_number.expect("non-null function pointer")(ret as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_match(mut nargs: i32) -> *mut NODE {
    let mut tre: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut dest: *mut NODE = 0 as *mut NODE;
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut rstart: i32 = 0;
    let mut len: i32 = 0;
    let mut ii: i32 = 0;
    let mut rlength: i32 = 0;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut s: regoff_t = 0;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buff: [i8; 100] = [0; 100];
    let mut amt: size_t = 0;
    let mut oldamt: size_t = 0 as i32 as size_t;
    let mut ilen: size_t = 0;
    let mut slen: size_t = 0;
    let mut subsepstr: *mut i8 = 0 as *mut i8;
    let mut subseplen: size_t = 0;
    check_args_min_max(nargs, b"match\0" as *const u8 as *const i8, 2 as i32, 3 as i32);
    dest = 0 as *mut NODE;
    if nargs == 3 as i32 {
        dest = POP_PARAM();
        if (*dest).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2778 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"match: third argument is not an array\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        check_symtab_functab(
            dest,
            b"match\0" as *const u8 as *const i8,
            dcgettext(
                0 as *const i8,
                b"%s: cannot use %s as third argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        ((*(*dest).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(dest, 0 as *mut exp_node);
    }
    let fresh29 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    tre = (*fresh29).rptr;
    rp = re_update(tre);
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32
            & (flagvals::STRING as i32 | flagvals::USER_INPUT as i32) as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 2787 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string first argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"match\0" as *const u8 as *const i8,
        );
    }
    rstart = research(rp, (*t1).sub.val.sp, 0 as i32, (*t1).sub.val.slen, 1 as i32);
    if rstart >= 0 as i32 {
        let mut wc_indices: *mut size_t = 0 as *mut size_t;
        rlength = *((*rp).regs.end).offset(0 as i32 as isize)
            - *((*rp).regs.start).offset(0 as i32 as isize);
        if rlength > 0 as i32 && gawk_mb_cur_max > 1 as i32 {
            t1 = str2wstr(t1, &mut wc_indices);
            rlength = (*wc_indices.offset((rstart + rlength - 1 as i32) as isize))
                .wrapping_sub(*wc_indices.offset(rstart as isize))
                .wrapping_add(1 as i32 as u64) as i32;
            rstart = *wc_indices.offset(rstart as isize) as i32;
        }
        rstart += 1;
        rstart;
        if !dest.is_null() {
            subsepstr = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.sp;
            subseplen = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.slen;
            ii = 0 as i32;
            while (ii as u32) < (*rp).regs.num_regs {
                s = *((*rp).regs.start).offset(ii as isize);
                if s != -(1 as i32) {
                    let mut subpat_start: size_t = 0;
                    let mut subpat_len: size_t = 0;
                    start = ((*t1).sub.val.sp).offset(s as isize);
                    subpat_start = s as size_t;
                    len = *((*rp).regs.end).offset(ii as isize) - s;
                    subpat_len = len as size_t;
                    if len > 0 as i32 && gawk_mb_cur_max > 1 as i32 {
                        subpat_start = *wc_indices.offset(s as isize);
                        subpat_len = (*wc_indices.offset((s + len - 1 as i32) as isize))
                            .wrapping_sub(subpat_start)
                            .wrapping_add(1 as i32 as u64);
                    }
                    it = make_str_node(start, len as size_t, 0 as i32);
                    (*it).flags = ::core::mem::transmute::<
                        u32,
                        flagvals,
                    >((*it).flags as u32 | flagvals::USER_INPUT as i32 as u32);
                    assoc_set(
                        dest,
                        make_number
                            .expect("non-null function pointer")(ii as libc::c_double),
                        it,
                    );
                    sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const i8, ii);
                    ilen = strlen(buff.as_mut_ptr());
                    amt = ilen
                        .wrapping_add(subseplen)
                        .wrapping_add(strlen(b"length\0" as *const u8 as *const i8))
                        .wrapping_add(1 as i32 as u64);
                    if oldamt == 0 as i32 as u64 {
                        buf = emalloc_real(
                            amt,
                            b"do_match\0" as *const u8 as *const i8,
                            b"buf\0" as *const u8 as *const i8,
                            b"builtin.c\0" as *const u8 as *const i8,
                            2833 as i32,
                        ) as *mut i8;
                    } else if amt > oldamt {
                        buf = erealloc_real(
                            buf as *mut libc::c_void,
                            amt,
                            b"do_match\0" as *const u8 as *const i8,
                            b"buf\0" as *const u8 as *const i8,
                            b"builtin.c\0" as *const u8 as *const i8,
                            2835 as i32,
                        ) as *mut i8;
                    }
                    oldamt = amt;
                    memcpy(
                        buf as *mut libc::c_void,
                        buff.as_mut_ptr() as *const libc::c_void,
                        ilen,
                    );
                    memcpy(
                        buf.offset(ilen as isize) as *mut libc::c_void,
                        subsepstr as *const libc::c_void,
                        subseplen,
                    );
                    memcpy(
                        buf.offset(ilen as isize).offset(subseplen as isize)
                            as *mut libc::c_void,
                        b"start\0" as *const u8 as *const i8 as *const libc::c_void,
                        6 as i32 as u64,
                    );
                    slen = ilen.wrapping_add(subseplen).wrapping_add(5 as i32 as u64);
                    assoc_set(
                        dest,
                        make_str_node(buf, slen, 0 as i32),
                        make_number
                            .expect(
                                "non-null function pointer",
                            )(
                            subpat_start as libc::c_double + 1 as i32 as libc::c_double,
                        ),
                    );
                    memcpy(
                        buf as *mut libc::c_void,
                        buff.as_mut_ptr() as *const libc::c_void,
                        ilen,
                    );
                    memcpy(
                        buf.offset(ilen as isize) as *mut libc::c_void,
                        subsepstr as *const libc::c_void,
                        subseplen,
                    );
                    memcpy(
                        buf.offset(ilen as isize).offset(subseplen as isize)
                            as *mut libc::c_void,
                        b"length\0" as *const u8 as *const i8 as *const libc::c_void,
                        7 as i32 as u64,
                    );
                    slen = ilen.wrapping_add(subseplen).wrapping_add(6 as i32 as u64);
                    assoc_set(
                        dest,
                        make_str_node(buf, slen, 0 as i32),
                        make_number
                            .expect(
                                "non-null function pointer",
                            )(subpat_len as libc::c_double),
                    );
                }
                ii += 1;
                ii;
            }
            pma_free(buf as *mut libc::c_void);
        }
        if !wc_indices.is_null() {
            pma_free(wc_indices as *mut libc::c_void);
        }
    } else {
        rstart = 0 as i32;
        rlength = -(1 as i32);
    }
    DEREF(t1);
    unref((*RSTART_node).sub.nodep.l.lptr);
    (*RSTART_node).sub.nodep.l.lptr = make_number
        .expect("non-null function pointer")(rstart as libc::c_double);
    unref((*RLENGTH_node).sub.nodep.l.lptr);
    (*RLENGTH_node).sub.nodep.l.lptr = make_number
        .expect("non-null function pointer")(rlength as libc::c_double);
    return make_number.expect("non-null function pointer")(rstart as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_sub(mut nargs: i32, mut flags: u32) -> *mut NODE {
    let mut scan: *mut i8 = 0 as *mut i8;
    let mut bp: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buflen: size_t = 0;
    let mut matchend: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut matchstart: *mut i8 = 0 as *mut i8;
    let mut text: *mut i8 = 0 as *mut i8;
    let mut textlen: size_t = 0 as i32 as size_t;
    let mut repl: *mut i8 = 0 as *mut i8;
    let mut replend: *mut i8 = 0 as *mut i8;
    let mut repllen: size_t = 0;
    let mut sofar: i32 = 0;
    let mut ampersands: i32 = 0;
    let mut matches: i32 = 0 as i32;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut rep_node: *mut NODE = 0 as *mut NODE;
    let mut target: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut how_many: i64 = 1 as i32 as i64;
    let mut global: bool = false;
    let mut current: i64 = 0;
    let mut lastmatchnonzero: bool = false;
    let mut mb_indices: *mut i8 = 0 as *mut i8;
    if flags & 0x2 as i32 as u32 != 0 as i32 as u32 {
        let mut d: libc::c_double = 0.;
        let mut glob_flag: *mut NODE = 0 as *mut NODE;
        check_exact_args(nargs, b"gensub\0" as *const u8 as *const i8, 4 as i32);
        tmp = (*stack_ptr.offset(-(3 as i32 as isize))).rptr;
        rp = re_update(tmp);
        target = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        glob_flag = POP_SCALAR();
        if (*glob_flag).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
            && (*glob_flag).sub.val.slen > 0 as i32 as u64
            && (*((*glob_flag).sub.val.sp).offset(0 as i32 as isize) as i32 == 'g' as i32
                || *((*glob_flag).sub.val.sp).offset(0 as i32 as isize) as i32
                    == 'G' as i32)
        {
            how_many = -(1 as i32) as i64;
        } else {
            force_number(glob_flag);
            d = (*glob_flag).sub.val.fltnum;
            if d < 1 as i32 as libc::c_double {
                how_many = 1 as i32 as i64;
            } else if d < 9223372036854775807 as i64 as libc::c_double {
                how_many = d as i64;
            } else {
                how_many = 9223372036854775807 as i64;
            }
            if d <= 0 as i32 as libc::c_double {
                force_string_fmt(glob_flag, CONVFMT, CONVFMTidx);
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3027 as i32);
                (Some(
                    (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"gensub: third argument `%.*s' treated as 1\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*glob_flag).sub.val.slen as i32,
                    (*glob_flag).sub.val.sp,
                );
            }
        }
        DEREF(glob_flag);
    } else {
        if flags & 0x1 as i32 as u32 != 0 as i32 as u32 {
            check_exact_args(nargs, b"gsub\0" as *const u8 as *const i8, 3 as i32);
        } else {
            check_exact_args(nargs, b"sub\0" as *const u8 as *const i8, 3 as i32);
        }
        tmp = (*stack_ptr.offset(-(2 as i32 as isize))).rptr;
        rp = re_update(tmp);
        if flags & 0x1 as i32 as u32 != 0 as i32 as u32 {
            how_many = -(1 as i32) as i64;
        }
        if flags & 0x4 as i32 as u32 != 0 as i32 as u32 {
            target = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        } else {
            let fresh30 = stack_ptr;
            stack_ptr = stack_ptr.offset(-1);
            lhs = (*fresh30).lptr;
            target = force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
        }
    }
    global = how_many == -(1 as i32) as i64;
    rep_node = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    stack_ptr = stack_ptr.offset(-1);
    stack_ptr;
    if !(research(rp, (*target).sub.val.sp, 0 as i32, (*target).sub.val.slen, 1 as i32)
        == -(1 as i32)
        || *((*rp).regs.start).offset(0 as i32 as isize) as u64 > (*target).sub.val.slen)
    {
        text = (*target).sub.val.sp;
        textlen = (*target).sub.val.slen;
        repl = (*rep_node).sub.val.sp;
        replend = repl.offset((*rep_node).sub.val.slen as isize);
        repllen = replend.offset_from(repl) as i64 as size_t;
        ampersands = 0 as i32;
        if gawk_mb_cur_max > 1 as i32 && repllen > 0 as i32 as u64 {
            mb_indices = emalloc_real(
                repllen.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                b"do_sub\0" as *const u8 as *const i8,
                b"mb_indices\0" as *const u8 as *const i8,
                b"builtin.c\0" as *const u8 as *const i8,
                3086 as i32,
            ) as *mut i8;
            index_multibyte_buffer(repl, mb_indices, repllen as i32);
        }
        scan = repl;
        while scan < replend {
            if (gawk_mb_cur_max == 1 as i32
                || repllen > 0 as i32 as u64
                    && *mb_indices.offset(scan.offset_from(repl) as i64 as isize) as i32
                        == 1 as i32) && *scan as i32 == '&' as i32
            {
                repllen = repllen.wrapping_sub(1);
                repllen;
                ampersands += 1;
                ampersands;
            } else if *scan as i32 == '\\' as i32 {
                if flags & 0x2 as i32 as u32 != 0 as i32 as u32 {
                    if *(*__ctype_b_loc())
                        .offset(*scan.offset(1 as i32 as isize) as u8 as i32 as isize)
                        as i32
                        & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        ampersands += 1;
                        ampersands;
                        scan = scan.offset(1);
                        scan;
                    } else {
                        repllen = repllen.wrapping_sub(1);
                        repllen;
                        scan = scan.offset(1);
                        scan;
                    }
                } else if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
                    if *scan.offset(1 as i32 as isize) as i32 == '&' as i32
                        || *scan.offset(1 as i32 as isize) as i32 == '\\' as i32
                    {
                        repllen = repllen.wrapping_sub(1);
                        repllen;
                        scan = scan.offset(1);
                        scan;
                    }
                } else if strncmp(
                    scan,
                    b"\\\\\\&\0" as *const u8 as *const i8,
                    4 as i32 as u64,
                ) == 0 as i32
                    || strncmp(
                        scan,
                        b"\\\\\\\\\0" as *const u8 as *const i8,
                        4 as i32 as u64,
                    ) == 0 as i32
                {
                    repllen = (repllen as u64).wrapping_sub(2 as i32 as u64) as size_t
                        as size_t;
                    scan = scan.offset(3 as i32 as isize);
                } else if strncmp(
                    scan,
                    b"\\\\&\0" as *const u8 as *const i8,
                    3 as i32 as u64,
                ) == 0 as i32
                {
                    ampersands += 1;
                    ampersands;
                    repllen = repllen.wrapping_sub(1);
                    repllen;
                    scan = scan.offset(2 as i32 as isize);
                } else if *scan.offset(1 as i32 as isize) as i32 == '&' as i32 {
                    repllen = repllen.wrapping_sub(1);
                    repllen;
                    scan = scan.offset(1);
                    scan;
                }
            }
            scan = scan.offset(1);
            scan;
        }
        lastmatchnonzero = 0 as i32 != 0;
        buflen = textlen
            .wrapping_add(((ampersands + 1 as i32) as u64).wrapping_mul(repllen))
            .wrapping_add(1 as i32 as u64);
        buf = emalloc_real(
            buflen.wrapping_add(1 as i32 as u64),
            b"do_sub\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"builtin.c\0" as *const u8 as *const i8,
            3139 as i32,
        ) as *mut i8;
        *buf.offset(buflen as isize) = '\0' as i32 as i8;
        bp = buf;
        current = 1 as i32 as i64;
        loop {
            matches += 1;
            matches;
            matchstart = ((*target).sub.val.sp)
                .offset(*((*rp).regs.start).offset(0 as i32 as isize) as isize);
            matchend = ((*target).sub.val.sp)
                .offset(*((*rp).regs.end).offset(0 as i32 as isize) as isize);
            len = (matchend.offset_from(text) as i64 as u64)
                .wrapping_add(repllen)
                .wrapping_add(
                    (ampersands as i64 * matchend.offset_from(matchstart) as i64) as u64,
                )
                .wrapping_add(1 as i32 as u64);
            sofar = bp.offset_from(buf) as i64 as i32;
            while buflen < (sofar as u64).wrapping_add(len).wrapping_add(1 as i32 as u64)
            {
                buflen = (buflen as u64).wrapping_mul(2 as i32 as u64) as size_t
                    as size_t;
                buf = erealloc_real(
                    buf as *mut libc::c_void,
                    buflen,
                    b"sub_common\0" as *const u8 as *const i8,
                    b"buf\0" as *const u8 as *const i8,
                    b"builtin.c\0" as *const u8 as *const i8,
                    3166 as i32,
                ) as *mut i8;
                bp = buf.offset(sofar as isize);
            }
            scan = text;
            while scan < matchstart {
                let fresh31 = bp;
                bp = bp.offset(1);
                *fresh31 = *scan;
                scan = scan.offset(1);
                scan;
            }
            if global as i32 != 0 || current == how_many {
                if matchstart == matchend && lastmatchnonzero as i32 != 0
                    && matchstart == text
                {
                    lastmatchnonzero = 0 as i32 != 0;
                    matches -= 1;
                    matches;
                } else {
                    scan = repl;
                    while scan < replend {
                        if *scan as i32 == '&' as i32
                            && (gawk_mb_cur_max == 1 as i32
                                || *mb_indices
                                    .offset(scan.offset_from(repl) as i64 as isize) as i32
                                    == 1 as i32)
                        {
                            cp = matchstart;
                            while cp < matchend {
                                let fresh32 = bp;
                                bp = bp.offset(1);
                                *fresh32 = *cp;
                                cp = cp.offset(1);
                                cp;
                            }
                        } else if *scan as i32 == '\\' as i32
                            && (gawk_mb_cur_max == 1 as i32
                                || repllen > 0 as i32 as u64
                                    && *mb_indices
                                        .offset(scan.offset_from(repl) as i64 as isize) as i32
                                        == 1 as i32)
                        {
                            if flags & 0x2 as i32 as u32 != 0 {
                                if *(*__ctype_b_loc())
                                    .offset(
                                        *scan.offset(1 as i32 as isize) as u8 as i32 as isize,
                                    ) as i32
                                    & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                    != 0
                                {
                                    let mut dig: i32 = *scan.offset(1 as i32 as isize) as i32
                                        - '0' as i32;
                                    if (dig as u32) < (*rp).regs.num_regs
                                        && *((*rp).regs.start).offset(dig as isize) != -(1 as i32)
                                    {
                                        let mut start: *mut i8 = 0 as *mut i8;
                                        let mut end: *mut i8 = 0 as *mut i8;
                                        start = ((*target).sub.val.sp)
                                            .offset(*((*rp).regs.start).offset(dig as isize) as isize);
                                        end = ((*target).sub.val.sp)
                                            .offset(*((*rp).regs.end).offset(dig as isize) as isize);
                                        cp = start;
                                        while cp < end {
                                            let fresh33 = bp;
                                            bp = bp.offset(1);
                                            *fresh33 = *cp;
                                            cp = cp.offset(1);
                                            cp;
                                        }
                                    }
                                    scan = scan.offset(1);
                                    scan;
                                } else {
                                    scan = scan.offset(1);
                                    let fresh34 = bp;
                                    bp = bp.offset(1);
                                    *fresh34 = *scan;
                                }
                            } else if do_flags as u32
                                & do_flag_values::DO_POSIX as i32 as u32 != 0
                            {
                                if *scan.offset(1 as i32 as isize) as i32 == '&' as i32
                                    || *scan.offset(1 as i32 as isize) as i32 == '\\' as i32
                                {
                                    scan = scan.offset(1);
                                    scan;
                                }
                                let fresh35 = bp;
                                bp = bp.offset(1);
                                *fresh35 = *scan;
                            } else if strncmp(
                                scan,
                                b"\\\\\\&\0" as *const u8 as *const i8,
                                4 as i32 as u64,
                            ) == 0 as i32
                                || strncmp(
                                    scan,
                                    b"\\\\\\\\\0" as *const u8 as *const i8,
                                    4 as i32 as u64,
                                ) == 0 as i32
                            {
                                let fresh36 = bp;
                                bp = bp.offset(1);
                                *fresh36 = '\\' as i32 as i8;
                                let fresh37 = bp;
                                bp = bp.offset(1);
                                *fresh37 = *scan.offset(3 as i32 as isize);
                                scan = scan.offset(3 as i32 as isize);
                            } else if strncmp(
                                scan,
                                b"\\\\&\0" as *const u8 as *const i8,
                                3 as i32 as u64,
                            ) == 0 as i32
                            {
                                let fresh38 = bp;
                                bp = bp.offset(1);
                                *fresh38 = '\\' as i32 as i8;
                                cp = matchstart;
                                while cp < matchend {
                                    let fresh39 = bp;
                                    bp = bp.offset(1);
                                    *fresh39 = *cp;
                                    cp = cp.offset(1);
                                    cp;
                                }
                                scan = scan.offset(2 as i32 as isize);
                            } else if *scan.offset(1 as i32 as isize) as i32
                                == '&' as i32
                            {
                                let fresh40 = bp;
                                bp = bp.offset(1);
                                *fresh40 = '&' as i32 as i8;
                                scan = scan.offset(1);
                                scan;
                            } else {
                                let fresh41 = bp;
                                bp = bp.offset(1);
                                *fresh41 = *scan;
                            }
                        } else {
                            let fresh42 = bp;
                            bp = bp.offset(1);
                            *fresh42 = *scan;
                        }
                        scan = scan.offset(1);
                        scan;
                    }
                    if matchstart != matchend {
                        lastmatchnonzero = 1 as i32 != 0;
                    }
                }
            } else {
                cp = matchstart;
                while cp < matchend {
                    let fresh43 = bp;
                    bp = bp.offset(1);
                    *fresh43 = *cp;
                    cp = cp.offset(1);
                    cp;
                }
            }
            if matchstart == matchend && matchend < text.offset(textlen as isize) {
                let fresh44 = bp;
                bp = bp.offset(1);
                *fresh44 = *matchend;
                matchend = matchend.offset(1);
                matchend;
            }
            textlen = text.offset(textlen as isize).offset_from(matchend) as i64
                as size_t;
            text = matchend;
            if current >= how_many && !global
                || textlen as i64 <= 0 as i32 as i64 && matchstart == matchend
                || research(
                    rp,
                    (*target).sub.val.sp,
                    text.offset_from((*target).sub.val.sp) as i64 as i32,
                    textlen,
                    1 as i32,
                ) == -(1 as i32)
            {
                break;
            }
            current += 1;
            current;
        }
        sofar = bp.offset_from(buf) as i64 as i32;
        if buflen < (sofar as u64).wrapping_add(textlen).wrapping_add(1 as i32 as u64) {
            buflen = (sofar as u64).wrapping_add(textlen).wrapping_add(1 as i32 as u64);
            buf = erealloc_real(
                buf as *mut libc::c_void,
                buflen,
                b"do_sub\0" as *const u8 as *const i8,
                b"buf\0" as *const u8 as *const i8,
                b"builtin.c\0" as *const u8 as *const i8,
                3285 as i32,
            ) as *mut i8;
            bp = buf.offset(sofar as isize);
        }
        scan = text;
        while scan < text.offset(textlen as isize) {
            let fresh45 = bp;
            bp = bp.offset(1);
            *fresh45 = *scan;
            scan = scan.offset(1);
            scan;
        }
        *bp = '\0' as i32 as i8;
        textlen = bp.offset_from(buf) as i64 as size_t;
        if !mb_indices.is_null() {
            pma_free(mb_indices as *mut libc::c_void);
        }
    }
    DEREF(rep_node);
    if (matches == 0 as i32 || flags & 0x4 as i32 as u32 != 0 as i32 as u32)
        && !buf.is_null()
    {
        pma_free(buf as *mut libc::c_void);
        buf = 0 as *mut i8;
    }
    if flags & 0x2 as i32 as u32 != 0 {
        if matches > 0 as i32 {
            DEREF(target);
            return make_str_node(buf, textlen, 2 as i32);
        } else if (*target).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
        {
            DEREF(target);
            return make_str_node((*target).sub.val.sp, (*target).sub.val.slen, 0 as i32);
        }
        return target;
    }
    if flags & 0x4 as i32 as u32 != 0 as i32 as u32 {
        DEREF(target);
    } else if matches > 0 as i32 {
        let mut is_regex: bool = 0 as i32 != 0;
        let mut target_0: *mut NODE = *lhs;
        if (*target_0).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
            is_regex = 1 as i32 != 0;
            if (*target_0).valref == 1 as i32 as i64 {
                refree((*(*target_0).sub.val.typre).sub.nodep.r.preg[0 as i32 as usize]);
                if !((*(*target_0).sub.val.typre).sub.nodep.r.preg[1 as i32 as usize])
                    .is_null()
                {
                    refree(
                        (*(*target_0).sub.val.typre).sub.nodep.r.preg[1 as i32 as usize],
                    );
                }
                let ref mut fresh46 = (*((*target_0).sub.val.typre as *mut block_item))
                    .freep;
                *fresh46 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*target_0)
                    .sub
                    .val
                    .typre as *mut block_item;
            }
        }
        unref(*lhs);
        if is_regex {
            *lhs = make_typed_regex(buf, textlen);
        } else {
            *lhs = make_str_node(buf, textlen, 2 as i32);
        }
    }
    return make_number.expect("non-null function pointer")(matches as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn call_sub(mut name: *const i8, mut nargs: i32) -> *mut NODE {
    let mut flags: u32 = 0 as i32 as u32;
    let mut regex: *mut NODE = 0 as *mut NODE;
    let mut replace: *mut NODE = 0 as *mut NODE;
    let mut glob_flag: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut rhs: *mut NODE = 0 as *mut NODE;
    let mut zero: *mut NODE = make_number.expect("non-null function pointer")(0.0f64);
    let mut result: *mut NODE = 0 as *mut NODE;
    if *name.offset(0 as i32 as isize) as i32 == 'g' as i32 {
        if *name.offset(1 as i32 as isize) as i32 == 'e' as i32 {
            flags = 0x2 as i32 as u32;
        } else {
            flags = 0x1 as i32 as u32;
        }
    }
    let mut need_free: bool = 0 as i32 != 0;
    if flags == 0 as i32 as u32 || flags == 0x1 as i32 as u32 {
        if nargs != 2 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3386 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: can be called indirectly only with two arguments\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                name,
            );
        }
        replace = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        let fresh47 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        regex = (*fresh47).rptr;
        if (*regex).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
            regex = (*regex).sub.val.typre;
        } else {
            regex = make_regnode(nodevals::Node_regex, regex);
            need_free = 1 as i32 != 0;
        }
        let ref mut fresh48 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh48 = regex;
        let ref mut fresh49 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh49 = replace;
        lhs = r_get_field(zero, 0 as *mut Func_ptr, 1 as i32 != 0);
        nargs += 1;
        nargs;
        let ref mut fresh50 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .lptr;
        *fresh50 = lhs;
    } else {
        if nargs < 3 as i32 || nargs > 4 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3409 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"indirect call to gensub requires three or four arguments\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if nargs == 4 as i32 {
            let fresh51 = stack_ptr;
            stack_ptr = stack_ptr.offset(-1);
            rhs = (*fresh51).rptr;
        } else {
            rhs = 0 as *mut NODE;
        }
        glob_flag = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        replace = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        let fresh52 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        regex = (*fresh52).rptr;
        if (*regex).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
            regex = (*regex).sub.val.typre;
        } else {
            regex = make_regnode(nodevals::Node_regex, regex);
            need_free = 1 as i32 != 0;
        }
        let ref mut fresh53 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh53 = regex;
        let ref mut fresh54 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh54 = replace;
        let ref mut fresh55 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh55 = glob_flag;
        if rhs.is_null() {
            lhs = r_get_field(zero, 0 as *mut Func_ptr, 1 as i32 != 0);
            rhs = *lhs;
            (*rhs).valref += 1;
            (*rhs).valref;
            let ref mut fresh56 = (*if stack_ptr < stack_top {
                stack_ptr = stack_ptr.offset(1);
                stack_ptr
            } else {
                grow_stack()
            })
                .rptr;
            *fresh56 = rhs;
            nargs += 1;
            nargs;
        } else {
            let ref mut fresh57 = (*if stack_ptr < stack_top {
                stack_ptr = stack_ptr.offset(1);
                stack_ptr
            } else {
                grow_stack()
            })
                .rptr;
            *fresh57 = rhs;
        }
    }
    unref(zero);
    result = do_sub(nargs, flags);
    if need_free {
        refree((*regex).sub.nodep.r.preg[0 as i32 as usize]);
        if !((*regex).sub.nodep.r.preg[1 as i32 as usize]).is_null() {
            refree((*regex).sub.nodep.r.preg[1 as i32 as usize]);
        }
        let ref mut fresh58 = (*(regex as *mut block_item)).freep;
        *fresh58 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = regex as *mut block_item;
    }
    if flags != 0x2 as i32 as u32 {
        reset_record();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn call_match(mut nargs: i32) -> *mut NODE {
    let mut regex: *mut NODE = 0 as *mut NODE;
    let mut text: *mut NODE = 0 as *mut NODE;
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut result: *mut NODE = 0 as *mut NODE;
    if nargs < 2 as i32 || nargs > 3 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3471 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"indirect call to match requires two or three arguments\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    array = 0 as *mut NODE;
    text = array;
    regex = text;
    if nargs == 3 as i32 {
        let fresh59 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        array = (*fresh59).rptr;
    }
    let fresh60 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    regex = (*fresh60).rptr;
    let mut need_free: bool = 0 as i32 != 0;
    if (*regex).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
        regex = (*regex).sub.val.typre;
    } else {
        regex = make_regnode(nodevals::Node_regex, regex);
        need_free = 1 as i32 != 0;
    }
    let ref mut fresh61 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh61 = regex;
    if !array.is_null() {
        let ref mut fresh62 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh62 = array;
    }
    result = do_match(nargs);
    if need_free {
        refree((*regex).sub.nodep.r.preg[0 as i32 as usize]);
        if !((*regex).sub.nodep.r.preg[1 as i32 as usize]).is_null() {
            refree((*regex).sub.nodep.r.preg[1 as i32 as usize]);
        }
        let ref mut fresh63 = (*(regex as *mut block_item)).freep;
        *fresh63 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = regex as *mut block_item;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn call_split_func(
    mut name: *const i8,
    mut nargs: i32,
) -> *mut NODE {
    let mut regex: *mut NODE = 0 as *mut NODE;
    let mut seps: *mut NODE = 0 as *mut NODE;
    let mut result: *mut NODE = 0 as *mut NODE;
    seps = 0 as *mut NODE;
    regex = seps;
    if nargs < 2 as i32 || nargs > 4 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3515 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"indirect call to %s requires two to four arguments\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            name,
        );
    }
    if nargs == 4 as i32 {
        let fresh64 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        seps = (*fresh64).rptr;
    }
    let mut need_free: bool = 0 as i32 != 0;
    if nargs >= 3 as i32 {
        regex = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if (*regex).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
            regex = (*regex).sub.val.typre;
        } else {
            regex = make_regnode(nodevals::Node_regex, regex);
            need_free = 1 as i32 != 0;
        }
    } else {
        if *name.offset(0 as i32 as isize) as i32 == 's' as i32 {
            regex = make_regnode(nodevals::Node_regex, (*FS_node).sub.nodep.l.lptr);
            (*regex).sub.nodep.reflags = ::core::mem::transmute::<
                u32,
                reflagvals,
            >((*regex).sub.nodep.reflags as u32 | reflagvals::FS_DFLT as i32 as u32);
        } else {
            regex = make_regnode(nodevals::Node_regex, (*FPAT_node).sub.nodep.l.lptr);
        }
        need_free = 1 as i32 != 0;
        nargs += 1;
        nargs;
    }
    let ref mut fresh65 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh65 = regex;
    if !seps.is_null() {
        let ref mut fresh66 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh66 = seps;
    }
    result = if *name.offset(0 as i32 as isize) as i32 == 's' as i32 {
        do_split(nargs)
    } else {
        do_patsplit(nargs)
    };
    if need_free {
        refree((*regex).sub.nodep.r.preg[0 as i32 as usize]);
        if !((*regex).sub.nodep.r.preg[1 as i32 as usize]).is_null() {
            refree((*regex).sub.nodep.r.preg[1 as i32 as usize]);
        }
        let ref mut fresh67 = (*(regex as *mut block_item)).freep;
        *fresh67 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = regex as *mut block_item;
    }
    return result;
}
unsafe extern "C" fn make_integer(mut n: uintmax_t) -> *mut NODE {
    n = adjust_uint(n);
    return make_number.expect("non-null function pointer")(n as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_lshift(mut nargs: i32) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut uval: uintmax_t = 0;
    let mut ushift: uintmax_t = 0;
    let mut res: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    let mut shift: libc::c_double = 0.;
    check_exact_args(nargs, b"lshift\0" as *const u8 as *const i8, 2 as i32);
    s2 = POP_SCALAR();
    let fresh68 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    s1 = (*fresh68).rptr;
    if (*s1).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        DEREF(s2);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3581 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(s1),
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if (*fixtype(s1)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3584 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric first argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"lshift\0" as *const u8 as *const i8,
            );
        }
        if (*fixtype(s2)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3586 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric second argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"lshift\0" as *const u8 as *const i8,
            );
        }
    }
    val = (*force_number(s1)).sub.val.fltnum;
    shift = (*force_number(s2)).sub.val.fltnum;
    if val < 0 as i32 as libc::c_double || shift < 0 as i32 as libc::c_double {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3592 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"lshift(%f, %f): negative values are not allowed\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            val,
            shift,
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if double_to_int(val) != val || double_to_int(shift) != shift {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3596 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"lshift(%f, %f): fractional values will be truncated\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                val,
                shift,
            );
        }
        if shift
            >= (::core::mem::size_of::<uintmax_t>() as u64).wrapping_mul(8 as i32 as u64)
                as libc::c_double
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3598 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"lshift(%f, %f): too large shift value will give strange results\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                val,
                shift,
            );
        }
    }
    DEREF(s1);
    DEREF(s2);
    uval = val as uintmax_t;
    ushift = shift as uintmax_t;
    res = uval << ushift;
    return make_integer(res);
}
#[no_mangle]
pub unsafe extern "C" fn do_rshift(mut nargs: i32) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut uval: uintmax_t = 0;
    let mut ushift: uintmax_t = 0;
    let mut res: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    let mut shift: libc::c_double = 0.;
    check_exact_args(nargs, b"rshift\0" as *const u8 as *const i8, 2 as i32);
    s2 = POP_SCALAR();
    let fresh69 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    s1 = (*fresh69).rptr;
    if (*s1).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        DEREF(s2);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3622 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(s1),
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if (*fixtype(s1)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3625 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric first argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"rshift\0" as *const u8 as *const i8,
            );
        }
        if (*fixtype(s2)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3627 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-numeric second argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"rshift\0" as *const u8 as *const i8,
            );
        }
    }
    val = (*force_number(s1)).sub.val.fltnum;
    shift = (*force_number(s2)).sub.val.fltnum;
    if val < 0 as i32 as libc::c_double || shift < 0 as i32 as libc::c_double {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3633 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"rshift(%f, %f): negative values are not allowed\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            val,
            shift,
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if double_to_int(val) != val || double_to_int(shift) != shift {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3637 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"rshift(%f, %f): fractional values will be truncated\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                val,
                shift,
            );
        }
        if shift
            >= (::core::mem::size_of::<uintmax_t>() as u64).wrapping_mul(8 as i32 as u64)
                as libc::c_double
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3639 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"rshift(%f, %f): too large shift value will give strange results\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                val,
                shift,
            );
        }
    }
    DEREF(s1);
    DEREF(s2);
    uval = val as uintmax_t;
    ushift = shift as uintmax_t;
    res = uval >> ushift;
    return make_integer(res);
}
#[no_mangle]
pub unsafe extern "C" fn do_and(mut nargs: i32) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut res: uintmax_t = 0;
    let mut uval: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    res = !(0 as i32 as uintmax_t);
    if nargs < 2 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3663 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: called with less than two arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"and\0" as *const u8 as *const i8,
        );
    }
    while nargs > 0 as i32 {
        s1 = POP_SCALAR();
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(s1)).flags as u32 & flagvals::NUMBER as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3668 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: argument %d is non-numeric\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"and\0" as *const u8 as *const i8,
                nargs,
            );
        }
        val = (*force_number(s1)).sub.val.fltnum;
        if val < 0 as i32 as libc::c_double {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3672 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: argument %d negative value %g is not allowed\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"and\0" as *const u8 as *const i8,
                nargs,
                val,
            );
        }
        uval = val as uintmax_t;
        res &= uval;
        DEREF(s1);
        nargs -= 1;
        nargs;
    }
    return make_integer(res);
}
#[no_mangle]
pub unsafe extern "C" fn do_or(mut nargs: i32) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut res: uintmax_t = 0;
    let mut uval: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    res = 0 as i32 as uintmax_t;
    if nargs < 2 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3694 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: called with less than two arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"or\0" as *const u8 as *const i8,
        );
    }
    while nargs > 0 as i32 {
        s1 = POP_SCALAR();
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(s1)).flags as u32 & flagvals::NUMBER as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3699 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: argument %d is non-numeric\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"or\0" as *const u8 as *const i8,
                nargs,
            );
        }
        val = (*force_number(s1)).sub.val.fltnum;
        if val < 0 as i32 as libc::c_double {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3703 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: argument %d negative value %g is not allowed\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"or\0" as *const u8 as *const i8,
                nargs,
                val,
            );
        }
        uval = val as uintmax_t;
        res |= uval;
        DEREF(s1);
        nargs -= 1;
        nargs;
    }
    return make_integer(res);
}
#[no_mangle]
pub unsafe extern "C" fn do_xor(mut nargs: i32) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut res: uintmax_t = 0;
    let mut uval: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    if nargs < 2 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3724 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: called with less than two arguments\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"xor\0" as *const u8 as *const i8,
        );
    }
    res = 0 as i32 as uintmax_t;
    while nargs > 0 as i32 {
        s1 = POP_SCALAR();
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(s1)).flags as u32 & flagvals::NUMBER as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3730 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: argument %d is non-numeric\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"xor\0" as *const u8 as *const i8,
                nargs,
            );
        }
        val = (*force_number(s1)).sub.val.fltnum;
        if val < 0 as i32 as libc::c_double {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3734 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: argument %d negative value %g is not allowed\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"xor\0" as *const u8 as *const i8,
                nargs,
                val,
            );
        }
        uval = val as uintmax_t;
        res ^= uval;
        DEREF(s1);
        nargs -= 1;
        nargs;
    }
    return make_integer(res);
}
#[no_mangle]
pub unsafe extern "C" fn do_compl(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    let mut uval: uintmax_t = 0;
    check_exact_args(nargs, b"compl\0" as *const u8 as *const i8, 1 as i32);
    tmp = POP_SCALAR();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(tmp)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3758 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"compl\0" as *const u8 as *const i8,
        );
    }
    d = (*force_number(tmp)).sub.val.fltnum;
    DEREF(tmp);
    if d < 0 as i32 as libc::c_double {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3763 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"compl(%f): negative value is not allowed\0" as *const u8 as *const i8,
                5 as i32,
            ),
            d,
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && double_to_int(d) != d
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3766 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"compl(%f): fractional value will be truncated\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            d,
        );
    }
    uval = d as uintmax_t;
    uval = !uval;
    return make_integer(uval);
}
#[no_mangle]
pub unsafe extern "C" fn do_strtonum(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(nargs, b"strtonum\0" as *const u8 as *const i8, 1 as i32);
    tmp = fixtype(POP_SCALAR());
    if (*tmp).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32 {
        d = (*tmp).sub.val.fltnum;
    } else if get_numbase((*tmp).sub.val.sp, (*tmp).sub.val.slen, use_lc_numeric != 0)
        != 10 as i32
    {
        d = nondec2awknum((*tmp).sub.val.sp, (*tmp).sub.val.slen, 0 as *mut *mut i8);
    } else {
        d = (*force_number(tmp)).sub.val.fltnum;
    }
    DEREF(tmp);
    return make_number.expect("non-null function pointer")(d);
}
#[no_mangle]
pub unsafe extern "C" fn nondec2awknum(
    mut str: *mut i8,
    mut len: size_t,
    mut endptr: *mut *mut i8,
) -> libc::c_double {
    let mut current_block: u64;
    let mut retval: libc::c_double = 0.0f64;
    let mut save: i8 = 0;
    let mut val: libc::c_short = 0;
    let mut start: *mut i8 = str;
    if len >= 2 as i32 as u64 && *str as i32 == '0' as i32
        && (*str.offset(1 as i32 as isize) as i32 == 'x' as i32
            || *str.offset(1 as i32 as isize) as i32 == 'X' as i32)
    {
        if len <= 2 as i32 as u64 {
            if !endptr.is_null() {
                *endptr = start;
            }
            return 0.0f64;
        }
        str = str.offset(2 as i32 as isize);
        len = (len as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
        loop {
            if !(len > 0 as i32 as u64) {
                current_block = 10043043949733653460;
                break;
            }
            match *str as i32 {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    val = (*str as i32 - '0' as i32) as libc::c_short;
                }
                97 | 98 | 99 | 100 | 101 | 102 => {
                    val = (*str as i32 - 'a' as i32 + 10 as i32) as libc::c_short;
                }
                65 | 66 | 67 | 68 | 69 | 70 => {
                    val = (*str as i32 - 'A' as i32 + 10 as i32) as libc::c_short;
                }
                _ => {
                    if !endptr.is_null() {
                        *endptr = str;
                    }
                    current_block = 8754876241739778530;
                    break;
                }
            }
            retval = retval * 16 as i32 as libc::c_double + val as i32 as libc::c_double;
            len = len.wrapping_sub(1);
            len;
            str = str.offset(1);
            str;
        }
        match current_block {
            8754876241739778530 => {}
            _ => {
                if !endptr.is_null() {
                    *endptr = str;
                }
            }
        }
    } else {
        if len >= 1 as i32 as u64 && *str as i32 == '0' as i32 {
            let mut l: i32 = 0;
            l = len as i32;
            loop {
                if !(l > 0 as i32) {
                    current_block = 1836292691772056875;
                    break;
                }
                if *(*__ctype_b_loc()).offset(*str as u8 as i32 as isize) as i32
                    & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32 == 0
                {
                    if !endptr.is_null() {
                        *endptr = str;
                    }
                    current_block = 8754876241739778530;
                    break;
                } else if *str as i32 == '8' as i32 || *str as i32 == '9' as i32 {
                    str = start;
                    current_block = 389752954149152658;
                    break;
                } else {
                    retval = retval * 8 as i32 as libc::c_double
                        + (*str as i32 - '0' as i32) as libc::c_double;
                    str = str.offset(1);
                    str;
                    l -= 1;
                    l;
                }
            }
            match current_block {
                8754876241739778530 => {}
                389752954149152658 => {}
                _ => {
                    if !endptr.is_null() {
                        *endptr = str;
                    }
                    current_block = 8754876241739778530;
                }
            }
        } else {
            current_block = 389752954149152658;
        }
        match current_block {
            8754876241739778530 => {}
            _ => {
                save = *str.offset(len as isize);
                *str.offset(len as isize) = '\0' as i32 as i8;
                retval = strtod(str, endptr);
                *str.offset(len as isize) = save;
            }
        }
    }
    return retval;
}
unsafe extern "C" fn localecategory_from_argument(mut t: *mut NODE) -> i32 {
    static mut cat_tab: [category_table; 7] = [
        {
            let mut init = category_table {
                val: 6 as i32,
                name: b"LC_ALL\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = category_table {
                val: 3 as i32,
                name: b"LC_COLLATE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = category_table {
                val: 0 as i32,
                name: b"LC_CTYPE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = category_table {
                val: 5 as i32,
                name: b"LC_MESSAGES\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = category_table {
                val: 4 as i32,
                name: b"LC_MONETARY\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = category_table {
                val: 1 as i32,
                name: b"LC_NUMERIC\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = category_table {
                val: 2 as i32,
                name: b"LC_TIME\0" as *const u8 as *const i8,
            };
            init
        },
    ];
    if !t.is_null() {
        let mut low: i32 = 0;
        let mut high: i32 = 0;
        let mut i: i32 = 0;
        let mut mid: i32 = 0;
        let mut category: *mut i8 = 0 as *mut i8;
        let mut lc_cat: i32 = -(1 as i32);
        let mut save: i8 = *((*t).sub.val.sp).offset((*t).sub.val.slen as isize);
        *((*t).sub.val.sp).offset((*t).sub.val.slen as isize) = '\0' as i32 as i8;
        category = (*t).sub.val.sp;
        low = 0 as i32;
        high = (::core::mem::size_of::<[category_table; 7]>() as u64)
            .wrapping_div(::core::mem::size_of::<category_table>() as u64)
            .wrapping_sub(1 as i32 as u64) as i32;
        while low <= high {
            mid = (low + high) / 2 as i32;
            i = strcmp(category, cat_tab[mid as usize].name);
            if i < 0 as i32 {
                high = mid - 1 as i32;
            } else if i > 0 as i32 {
                low = mid + 1 as i32;
            } else {
                lc_cat = cat_tab[mid as usize].val;
                break;
            }
        }
        *((*t).sub.val.sp).offset((*t).sub.val.slen as isize) = save;
        if lc_cat == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3954 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"dcgettext: `%s' is not a valid locale category\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                category,
            );
        }
        return lc_cat;
    } else {
        return 5 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_dcgettext(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut string: *mut i8 = 0 as *mut i8;
    let mut the_result: *mut i8 = 0 as *mut i8;
    let mut reslen: size_t = 0;
    let mut lc_cat: i32 = 0;
    let mut domain: *mut i8 = 0 as *mut i8;
    let mut save1: i8 = '\0' as i32 as i8;
    let mut save2: i8 = '\0' as i32 as i8;
    check_args_min_max(
        nargs,
        b"dcgettext\0" as *const u8 as *const i8,
        1 as i32,
        3 as i32,
    );
    if nargs == 3 as i32 {
        tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(tmp)).flags as u32 & flagvals::STRING as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3989 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string third argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"dcgettext\0" as *const u8 as *const i8,
            );
        }
        lc_cat = localecategory_from_argument(tmp);
        DEREF(tmp);
    } else {
        lc_cat = 5 as i32;
    }
    if nargs >= 2 as i32 {
        t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(t2)).flags as u32 & flagvals::STRING as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 3998 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string second argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"dcgettext\0" as *const u8 as *const i8,
            );
        }
        domain = (*t2).sub.val.sp;
        str_terminate_f(t2, &mut save2);
    } else {
        domain = TEXTDOMAIN;
    }
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4020 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string first argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"dcgettext\0" as *const u8 as *const i8,
        );
    }
    string = (*t1).sub.val.sp;
    str_terminate_f(t1, &mut save1);
    the_result = dcgettext(domain, string, lc_cat);
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
    if !t2.is_null() {
        *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save2;
        DEREF(t2);
    }
    reslen = strlen(the_result);
    DEREF(t1);
    return make_str_node(the_result, reslen, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn do_dcngettext(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut t3: *mut NODE = 0 as *mut NODE;
    let mut string1: *mut i8 = 0 as *mut i8;
    let mut string2: *mut i8 = 0 as *mut i8;
    let mut number: u64 = 0;
    let mut d: libc::c_double = 0.;
    let mut the_result: *mut i8 = 0 as *mut i8;
    let mut reslen: size_t = 0;
    let mut lc_cat: i32 = 0;
    let mut domain: *mut i8 = 0 as *mut i8;
    let mut save: i8 = '\0' as i32 as i8;
    let mut save1: i8 = '\0' as i32 as i8;
    let mut save2: i8 = '\0' as i32 as i8;
    let mut saved_end: bool = 0 as i32 != 0;
    check_args_min_max(
        nargs,
        b"dcngettext\0" as *const u8 as *const i8,
        3 as i32,
        5 as i32,
    );
    if nargs == 5 as i32 {
        tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(tmp)).flags as u32 & flagvals::STRING as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4062 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string fifth argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"dcngettext\0" as *const u8 as *const i8,
            );
        }
        lc_cat = localecategory_from_argument(tmp);
        DEREF(tmp);
    } else {
        lc_cat = 5 as i32;
    }
    t3 = 0 as *mut NODE;
    if nargs >= 4 as i32 {
        t3 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(t3)).flags as u32 & flagvals::STRING as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4072 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string fourth argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"dcngettext\0" as *const u8 as *const i8,
            );
        }
        domain = (*t3).sub.val.sp;
        save = *domain.offset((*t3).sub.val.slen as isize);
        *domain.offset((*t3).sub.val.slen as isize) = '\0' as i32 as i8;
        saved_end = 1 as i32 != 0;
    } else {
        domain = TEXTDOMAIN;
    }
    t2 = force_number(POP_SCALAR());
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t2)).flags as u32 & flagvals::NUMBER as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4096 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-numeric third argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"dcngettext\0" as *const u8 as *const i8,
        );
    }
    d = (*t2).sub.val.fltnum;
    DEREF(t2);
    number = double_to_int(d) as u64;
    t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t2)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4103 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string second argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"dcngettext\0" as *const u8 as *const i8,
        );
    }
    string2 = (*t2).sub.val.sp;
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4107 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string first argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"dcngettext\0" as *const u8 as *const i8,
        );
    }
    string1 = (*t1).sub.val.sp;
    str_terminate_f(t1, &mut save1);
    str_terminate_f(t2, &mut save2);
    the_result = dcngettext(domain, string1, string2, number, lc_cat);
    reslen = strlen(the_result);
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
    *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save2;
    if saved_end {
        *domain.offset((*t3).sub.val.slen as isize) = save;
    }
    if !t3.is_null() {
        DEREF(t3);
    }
    DEREF(t1);
    DEREF(t2);
    return make_str_node(the_result, reslen, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn do_bindtextdomain(mut nargs: i32) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut directory: *const i8 = 0 as *const i8;
    let mut domain: *const i8 = 0 as *const i8;
    let mut the_result: *const i8 = 0 as *const i8;
    check_args_min_max(
        nargs,
        b"bindtextdomain\0" as *const u8 as *const i8,
        1 as i32,
        2 as i32,
    );
    t2 = 0 as *mut NODE;
    t1 = t2;
    directory = 0 as *const i8;
    domain = TEXTDOMAIN;
    let mut save: i8 = '\0' as i32 as i8;
    let mut save1: i8 = '\0' as i32 as i8;
    if nargs == 2 as i32 {
        t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*fixtype(t2)).flags as u32 & flagvals::STRING as i32 as u32
                == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4165 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: received non-string second argument\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                b"bindtextdomain\0" as *const u8 as *const i8,
            );
        }
        domain = (*t2).sub.val.sp as *const i8;
        save = *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize);
        *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = '\0' as i32 as i8;
    }
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*fixtype(t1)).flags as u32 & flagvals::STRING as i32 as u32
            == 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4174 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: received non-string first argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"bindtextdomain\0" as *const u8 as *const i8,
        );
    }
    if (*t1).sub.val.slen > 0 as i32 as u64 {
        directory = (*t1).sub.val.sp as *const i8;
        str_terminate_f(t1, &mut save1);
    }
    the_result = bindtextdomain(domain, directory);
    if !directory.is_null() {
        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
    }
    DEREF(t1);
    if !t2.is_null() {
        *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save;
        DEREF(t2);
    }
    if the_result.is_null() {
        the_result = b"\0" as *const u8 as *const i8;
    }
    return make_str_node(the_result, strlen(the_result), 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn do_typeof(mut nargs: i32) -> *mut NODE {
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut res: *const i8 = 0 as *const i8;
    let mut deref: bool = 1 as i32 != 0;
    let mut dbg: *mut NODE = 0 as *mut NODE;
    check_args_min_max(nargs, b"typeof\0" as *const u8 as *const i8, 1 as i32, 2 as i32);
    if nargs == 2 as i32 {
        dbg = POP_PARAM();
        if (*dbg).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4277 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"typeof: second argument is not an array\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        ((*(*dbg).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(dbg, 0 as *mut exp_node);
    } else {
        dbg = 0 as *mut NODE;
    }
    let fresh70 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    arg = (*fresh70).rptr;
    let mut current_block_55: u64;
    match (*arg).type_0 as u32 {
        5 => {
            res = b"array\0" as *const u8 as *const i8;
            deref = 0 as i32 != 0;
            if !dbg.is_null() {
                assoc_set(
                    dbg,
                    make_str_node(
                        b"array_type\0" as *const u8 as *const i8,
                        10 as i32 as size_t,
                        0 as i32,
                    ),
                    make_str_node(
                        (*(*arg).sub.nodep.l.lp).name,
                        strlen((*(*arg).sub.nodep.l.lp).name),
                        0 as i32,
                    ),
                );
                if arg == PROCINFO_node {
                    let mut i: i32 = 0;
                    i = 0 as i32;
                    while i < block_id::BLOCK_MAX as i32 {
                        let mut p: *mut i8 = 0 as *mut i8;
                        let mut nl: size_t = strlen(nextfree[i as usize].name);
                        let mut hw: i64 = nextfree[i as usize].highwater;
                        let mut active: i64 = 0;
                        active = hw;
                        let mut ip: *mut block_item = 0 as *mut block_item;
                        ip = nextfree[i as usize].freep;
                        while !ip.is_null() {
                            active -= 1;
                            active;
                            ip = (*ip).freep;
                        }
                        let mut l: size_t = nl
                            .wrapping_add(::core::mem::size_of::<[i8; 10]>() as u64);
                        p = emalloc_real(
                            l.wrapping_add(1 as i32 as u64),
                            b"do_typeof\0" as *const u8 as *const i8,
                            b"p\0" as *const u8 as *const i8,
                            b"builtin.c\0" as *const u8 as *const i8,
                            4320 as i32,
                        ) as *mut i8;
                        sprintf(
                            p,
                            b"%s_highwater\0" as *const u8 as *const i8,
                            nextfree[i as usize].name,
                        );
                        assoc_set(
                            dbg,
                            make_str_node(p, l, 2 as i32),
                            make_number
                                .expect("non-null function pointer")(hw as libc::c_double),
                        );
                        let mut l_0: size_t = nl
                            .wrapping_add(::core::mem::size_of::<[i8; 7]>() as u64);
                        p = emalloc_real(
                            l_0.wrapping_add(1 as i32 as u64),
                            b"do_typeof\0" as *const u8 as *const i8,
                            b"p\0" as *const u8 as *const i8,
                            b"builtin.c\0" as *const u8 as *const i8,
                            4321 as i32,
                        ) as *mut i8;
                        sprintf(
                            p,
                            b"%s_active\0" as *const u8 as *const i8,
                            nextfree[i as usize].name,
                        );
                        assoc_set(
                            dbg,
                            make_str_node(p, l_0, 2 as i32),
                            make_number
                                .expect(
                                    "non-null function pointer",
                                )(active as libc::c_double),
                        );
                        i += 1;
                        i;
                    }
                }
            }
            current_block_55 = 6560072651652764009;
        }
        4 => {
            arg = (*arg).sub.nodep.l.lptr;
            current_block_55 = 4373102982633913728;
        }
        1 => {
            current_block_55 = 4373102982633913728;
        }
        6 | 7 => {
            res = b"untyped\0" as *const u8 as *const i8;
            deref = 0 as i32 != 0;
            current_block_55 = 6560072651652764009;
        }
        12 => {
            if (*(*arg).sub.nodep.l.lptr).type_0 as u32
                == nodevals::Node_var as i32 as u32
                && ((*(*arg).sub.nodep.l.lptr).sub.nodep.l.lptr == Nnull_string
                    || (*(*(*arg).sub.nodep.l.lptr).sub.nodep.l.lptr).flags as u32
                        & flagvals::NULL_FIELD as i32 as u32 != 0 as i32 as u32)
            {
                res = b"unassigned\0" as *const u8 as *const i8;
            } else {
                res = b"untyped\0" as *const u8 as *const i8;
            }
            deref = 0 as i32 != 0;
            current_block_55 = 6560072651652764009;
        }
        _ => {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4405 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"typeof: unknown argument type `%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                nodetype2str((*arg).type_0),
            );
            current_block_55 = 6560072651652764009;
        }
    }
    match current_block_55 {
        4373102982633913728 => {
            let mut current_block_42: u64;
            match (*fixtype(arg)).flags as u32
                & (flagvals::STRING as i32 | flagvals::NUMBER as i32
                    | flagvals::USER_INPUT as i32 | flagvals::REGEX as i32
                    | flagvals::BOOLVAL as i32) as u32
            {
                80 => {
                    res = b"number|bool\0" as *const u8 as *const i8;
                    current_block_42 = 9441801433784995173;
                }
                16 => {
                    res = b"number\0" as *const u8 as *const i8;
                    current_block_42 = 9441801433784995173;
                }
                48 => {
                    res = b"strnum\0" as *const u8 as *const i8;
                    current_block_42 = 9441801433784995173;
                }
                524288 => {
                    res = b"regexp\0" as *const u8 as *const i8;
                    current_block_42 = 9441801433784995173;
                }
                2 => {
                    res = b"string\0" as *const u8 as *const i8;
                    current_block_42 = 1583693679777395924;
                }
                18 => {
                    current_block_42 = 1583693679777395924;
                }
                _ => {
                    current_block_42 = 11129683967232696748;
                }
            }
            match current_block_42 {
                1583693679777395924 => {
                    if arg == Nnull_string
                        || (*arg).flags as u32 & flagvals::NULL_FIELD as i32 as u32
                            != 0 as i32 as u32
                    {
                        res = b"unassigned\0" as *const u8 as *const i8;
                        current_block_42 = 9441801433784995173;
                    } else {
                        current_block_42 = 11129683967232696748;
                    }
                }
                _ => {}
            }
            match current_block_42 {
                11129683967232696748 => {
                    if res.is_null() {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"builtin.c\0" as *const u8 as *const i8,
                            4367 as i32,
                        );
                        (Some(
                            (Some(
                                r_warning as unsafe extern "C" fn(*const i8, ...) -> (),
                            ))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"typeof detected invalid flags combination `%s'; please file a bug report\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            flags2str((*arg).flags as i32),
                        );
                        res = b"unknown\0" as *const u8 as *const i8;
                    }
                }
                _ => {}
            }
            if !dbg.is_null() {
                let mut s: *const i8 = flags2str((*arg).flags as i32);
                assoc_set(
                    dbg,
                    make_str_node(
                        b"flags\0" as *const u8 as *const i8,
                        5 as i32 as size_t,
                        0 as i32,
                    ),
                    make_str_node(s, strlen(s), 0 as i32),
                );
            }
        }
        _ => {}
    }
    if deref {
        DEREF(arg);
    }
    return make_str_node(res, strlen(res), 0 as i32);
}
unsafe extern "C" fn mbc_byte_count(mut ptr: *const i8, mut numchars: size_t) -> size_t {
    let mut cur_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut sum: size_t = 0 as i32 as size_t;
    let mut mb_len: i32 = 0;
    memset(
        &mut cur_state as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    mb_len = mbrlen(ptr, numchars.wrapping_mul(gawk_mb_cur_max as u64), &mut cur_state)
        as i32;
    if mb_len <= 0 as i32 {
        return numchars;
    }
    while numchars > 0 as i32 as u64 {
        mb_len = mbrlen(
            ptr,
            numchars.wrapping_mul(gawk_mb_cur_max as u64),
            &mut cur_state,
        ) as i32;
        if mb_len <= 0 as i32 {
            break;
        }
        sum = (sum as u64).wrapping_add(mb_len as u64) as size_t as size_t;
        ptr = ptr.offset(mb_len as isize);
        numchars = numchars.wrapping_sub(1);
        numchars;
    }
    return sum;
}
unsafe extern "C" fn mbc_char_count(mut ptr: *const i8, mut numbytes: size_t) -> size_t {
    let mut cur_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut sum: size_t = 0 as i32 as size_t;
    let mut mb_len: i32 = 0;
    if gawk_mb_cur_max == 1 as i32 {
        return numbytes;
    }
    memset(
        &mut cur_state as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    mb_len = mbrlen(ptr, numbytes, &mut cur_state) as i32;
    if mb_len <= 0 as i32 {
        return numbytes;
    }
    while numbytes > 0 as i32 as u64 {
        mb_len = mbrlen(ptr, numbytes, &mut cur_state) as i32;
        if mb_len <= 0 as i32 {
            break;
        }
        sum = sum.wrapping_add(1);
        sum;
        ptr = ptr.offset(mb_len as isize);
        numbytes = (numbytes as u64).wrapping_sub(mb_len as u64) as size_t as size_t;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn sanitize_exit_status(mut status: i32) -> i32 {
    let mut ret: i32 = 0 as i32;
    if status & 0x7f as i32 == 0 as i32 {
        ret = (status & 0xff00 as i32) >> 8 as i32;
    } else if ((status & 0x7f as i32) + 1 as i32) as libc::c_schar as i32 >> 1 as i32
        > 0 as i32
    {
        let mut coredumped: bool = 0 as i32 != 0;
        coredumped = status & 0x80 as i32 != 0;
        ret = (status & 0x7f as i32)
            + (if coredumped as i32 != 0 { 512 as i32 } else { 256 as i32 });
    } else {
        ret = 0 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn out_of_range(mut n: *mut NODE) -> bool {
    return (if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf((*n).sub.val.fltnum as libc::c_float)
    } else {
        (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_double>() as u64
        {
            __isnan((*n).sub.val.fltnum)
        } else {
            __isnanl(f128::f128::new((*n).sub.val.fltnum))
        })
    }) != 0
        || (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_float>() as u64
        {
            __isinff((*n).sub.val.fltnum as libc::c_float)
        } else {
            (if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isinf((*n).sub.val.fltnum)
            } else {
                __isinfl(f128::f128::new((*n).sub.val.fltnum))
            })
        }) != 0;
}
#[no_mangle]
pub unsafe extern "C" fn format_nan_inf(mut n: *mut NODE, mut format: i8) -> *mut i8 {
    static mut buf: [i8; 100] = [0; 100];
    let mut val: libc::c_double = (*n).sub.val.fltnum;
    if if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf(val as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isnan(val)
    } else {
        __isnanl(f128::f128::new(val))
    } != 0
    {
        strcpy(
            buf.as_mut_ptr(),
            if (if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                (val as libc::c_float).is_sign_negative() as i32
            } else {
                (if ::core::mem::size_of::<libc::c_double>() as u64
                    == ::core::mem::size_of::<libc::c_double>() as u64
                {
                    val.is_sign_negative() as i32
                } else {
                    (f128::f128::new(val)).is_sign_negative() as i32
                })
            }) != 0 as i32
            {
                b"-nan\0" as *const u8 as *const i8
            } else {
                b"+nan\0" as *const u8 as *const i8
            },
        );
    } else if if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isinff(val as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isinf(val)
    } else {
        __isinfl(f128::f128::new(val))
    } != 0
    {
        strcpy(
            buf.as_mut_ptr(),
            if val < 0 as i32 as libc::c_double {
                b"-inf\0" as *const u8 as *const i8
            } else {
                b"+inf\0" as *const u8 as *const i8
            },
        );
    } else {
        return 0 as *mut i8
    }
    if *(*__ctype_b_loc()).offset(format as i32 as isize) as i32
        & C2RustUnnamed_0::_ISupper as i32 as libc::c_ushort as i32 != 0
    {
        let mut i: i32 = 0;
        i = 0 as i32;
        while buf[i as usize] as i32 != '\0' as i32 {
            buf[i as usize] = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = buf[i as usize] as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(buf[i as usize] as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(buf[i as usize] as i32 as isize);
                }
                __res
            }) as i8;
            i += 1;
            i;
        }
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn check_symtab_functab(
    mut dest: *mut NODE,
    mut fname: *const i8,
    mut msg_0: *const i8,
) {
    if dest == symbol_table {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4566 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(msg_0, fname, b"SYMTAB\0" as *const u8 as *const i8);
    } else if dest == func_table {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"builtin.c\0" as *const u8 as *const i8, 4568 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(msg_0, fname, b"FUNCTAB\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_mkbool(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut result: bool = false;
    tmp = POP_SCALAR();
    result = boolval(tmp);
    DEREF(tmp);
    return make_bool_node(result);
}
unsafe extern "C" fn reverse(mut str: *mut i8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut tmp: i8 = 0;
    i = 0 as i32;
    j = (strlen(str)).wrapping_sub(1 as i32 as u64) as i32;
    while j > i {
        tmp = *str.offset(i as isize);
        *str.offset(i as isize) = *str.offset(j as isize);
        *str.offset(j as isize) = tmp;
        i += 1;
        i;
        j -= 1;
        j;
    }
}
unsafe extern "C" fn add_thousands(
    mut original: *const i8,
    mut loc_0: *mut lconv,
) -> *const i8 {
    let mut orig_len: size_t = strlen(original);
    let mut new_len: size_t = orig_len
        .wrapping_add(orig_len.wrapping_mul(strlen((*loc_0).thousands_sep)))
        .wrapping_add(1 as i32 as u64);
    let mut newbuf: *mut i8 = 0 as *mut i8;
    let mut decimal_point: i8 = '\0' as i32 as i8;
    let mut dec: *const i8 = 0 as *const i8;
    let mut src: *const i8 = 0 as *const i8;
    let mut dest: *mut i8 = 0 as *mut i8;
    newbuf = emalloc_real(
        new_len,
        b"add_thousands\0" as *const u8 as *const i8,
        b"newbuf\0" as *const u8 as *const i8,
        b"builtin.c\0" as *const u8 as *const i8,
        4621 as i32,
    ) as *mut i8;
    memset(newbuf as *mut libc::c_void, '\0' as i32, new_len);
    src = original.offset(strlen(original) as isize).offset(-(1 as i32 as isize));
    dest = newbuf;
    if *((*loc_0).decimal_point).offset(0 as i32 as isize) as i32 != '\0' as i32 {
        decimal_point = *((*loc_0).decimal_point).offset(0 as i32 as isize);
        dec = strchr(original, decimal_point as i32);
        if !dec.is_null() {
            while src >= dec {
                let fresh71 = src;
                src = src.offset(-1);
                let fresh72 = dest;
                dest = dest.offset(1);
                *fresh72 = *fresh71;
            }
        }
    }
    let mut ii: i32 = 0 as i32;
    let mut jj: i32 = 0 as i32;
    loop {
        let fresh73 = src;
        src = src.offset(-1);
        let fresh74 = dest;
        dest = dest.offset(1);
        *fresh74 = *fresh73;
        if *((*loc_0).grouping).offset(ii as isize) as i32 != 0
            && {
                jj += 1;
                jj == *((*loc_0).grouping).offset(ii as isize) as i32
            }
        {
            if src >= original {
                let mut ts: *const i8 = (*loc_0).thousands_sep;
                let mut k: i32 = 0;
                k = (strlen(ts)).wrapping_sub(1 as i32 as u64) as i32;
                while k >= 0 as i32 {
                    let fresh75 = dest;
                    dest = dest.offset(1);
                    *fresh75 = *ts.offset(k as isize);
                    k -= 1;
                    k;
                }
            }
            if *((*loc_0).grouping).offset((ii + 1 as i32) as isize) as i32 == 0 as i32 {
                jj = 0 as i32;
            } else if *((*loc_0).grouping).offset((ii + 1 as i32) as isize) as i32
                == 127 as i32
            {
                while src >= original {
                    let fresh76 = src;
                    src = src.offset(-1);
                    let fresh77 = dest;
                    dest = dest.offset(1);
                    *fresh77 = *fresh76;
                }
                break;
            } else {
                ii += 1;
                ii;
                jj = 0 as i32;
            }
        }
        if !(src >= original) {
            break;
        }
    }
    let fresh78 = dest;
    dest = dest.offset(1);
    *fresh78 = '\0' as i32 as i8;
    reverse(newbuf);
    return newbuf;
}
unsafe extern "C" fn run_static_initializers() {
    time_t_min = (if !((0 as i32 as time_t) < -(1 as i32) as time_t) {
        !(0 as i32 as uintmax_t)
            << (::core::mem::size_of::<time_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64)
    } else {
        0 as i32 as u64
    }) as time_t;
    time_t_max = !(0 as i32 as time_t)
        - (if !((0 as i32 as time_t) < -(1 as i32) as time_t) {
            !(0 as i32 as uintmax_t)
                << (::core::mem::size_of::<time_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64)
        } else {
            0 as i32 as u64
        }) as time_t;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];