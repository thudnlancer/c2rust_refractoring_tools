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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn copysign(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn wcslen(_: *const i32) -> u64;
    fn __isnanl(__value: f128::f128) -> i32;
    fn __isnanf(__value: libc::c_float) -> i32;
    fn __isnan(__value: libc::c_double) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strcoll(__s1: *const i8, __s2: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn wcscoll(__s1: *const wchar_t, __s2: *const wchar_t) -> i32;
    fn getpid() -> __pid_t;
    static ruletab: [*const i8; 0];
    static mut NF: i64;
    static mut NR: i64;
    static mut FNR: i64;
    static mut BINMODE: i32;
    static mut IGNORECASE: bool;
    static mut OFS: *mut i8;
    static mut ORS: *mut i8;
    static mut OFMT: *mut i8;
    static mut CONVFMT: *const i8;
    static mut TEXTDOMAIN: *mut i8;
    static mut BINMODE_node: *mut NODE;
    static mut CONVFMT_node: *mut NODE;
    static mut FNR_node: *mut NODE;
    static mut IGNORECASE_node: *mut NODE;
    static mut NF_node: *mut NODE;
    static mut NR_node: *mut NODE;
    static mut OFMT_node: *mut NODE;
    static mut OFS_node: *mut NODE;
    static mut ORS_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut LINT_node: *mut NODE;
    static mut ERRNO_node: *mut NODE;
    static mut TEXTDOMAIN_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut fields_arr: *mut *mut NODE;
    static mut sourceline: i32;
    static mut source: *mut i8;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut cmp_numbers: Option<
        unsafe extern "C" fn(*const NODE, *const NODE) -> i32,
    >;
    static mut nextfree: [block_header; 2];
    static mut field0_valid: bool;
    static mut do_itrace: bool;
    static mut do_flags: do_flag_values;
    static mut exit_val: i32;
    static mut gawk_mb_cur_max: i32;
    fn r_unref(tmp: *mut NODE);
    fn make_array() -> *mut NODE;
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn concat_exp(nargs: i32, do_subsep: bool) -> *mut NODE;
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const i8,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn do_delete(symbol: *mut NODE, nsubs: i32);
    fn do_delete_loop(symbol: *mut NODE, lhs: *mut *mut NODE);
    fn lookup_builtin(name: *const i8) -> builtin_func_t;
    fn do_printf(nargs: i32, redirtype: i32);
    fn do_print(nargs: i32, redirtype: i32);
    fn do_print_rec(args: i32, redirtype: i32);
    fn do_match(nargs: i32) -> *mut NODE;
    fn do_sub(nargs: i32, flags: u32) -> *mut NODE;
    fn call_sub(name: *const i8, nargs: i32) -> *mut NODE;
    fn call_match(nargs: i32) -> *mut NODE;
    fn call_split_func(name: *const i8, nargs: i32) -> *mut NODE;
    fn strncasecmpmbs(_: *const u8, _: *const u8, _: size_t) -> i32;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn in_main_context() -> i32;
    fn nextfile(curfile_0: *mut *mut IOBUF, skipping: bool) -> i32;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn update_PROCINFO_num(subscript: *const i8, val: libc::c_double);
    fn inrec(iop: *mut IOBUF, errcode: *mut i32) -> bool;
    fn after_beginfile(curfile_0: *mut *mut IOBUF);
    fn do_getline(intovar: i32, iop: *mut IOBUF) -> *mut NODE;
    fn do_getline_redir(intovar: i32, redirtype: redirval) -> *mut NODE;
    fn r_warning(mesg: *const i8, _: ...);
    fn lookup(name: *const i8) -> *mut NODE;
    fn do_patsplit(nargs: i32) -> *mut NODE;
    fn do_split(nargs: i32) -> *mut NODE;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    fn research(
        rp: *mut Regexp,
        str: *mut i8,
        start: i32,
        len: size_t,
        flags: i32,
    ) -> i32;
    fn get_field(num: i64, assign: *mut Func_ptr) -> *mut *mut NODE;
    fn free_api_string_copies();
    fn awk_value_to_node(_: *const awk_value_t) -> *mut NODE;
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn str2wstr(n: *mut NODE, ptr: *mut *mut size_t) -> *mut NODE;
    fn r_free_wstr(n: *mut NODE);
    fn reset_record();
    fn update_global_values();
    static mut symbol_table: *mut NODE;
    static mut func_table: *mut NODE;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn close_extensions();
    fn close_io(stdio_problem: *mut bool, got_EPIPE: *mut bool) -> i32;
    fn set_record(buf: *const i8, cnt: size_t, _: *const awk_fieldwidth_info_t);
    fn getenv_long(name: *const i8) -> i64;
    fn set_RS();
    fn rebuild_record();
    fn update_ext_api();
    fn frame_popped();
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __int32_t = i32;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
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
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub enum defrule {
    BEGIN = 1,
    Rule,
    END,
    BEGINFILE,
    ENDFILE,
    MAXRULE,
}
impl defrule {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            defrule::BEGIN => 1,
            defrule::Rule => 2,
            defrule::END => 3,
            defrule::BEGINFILE => 4,
            defrule::ENDFILE => 5,
            defrule::MAXRULE => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> defrule {
        match value {
            1 => defrule::BEGIN,
            2 => defrule::Rule,
            3 => defrule::END,
            4 => defrule::BEGINFILE,
            5 => defrule::ENDFILE,
            6 => defrule::MAXRULE,
            _ => panic!("Invalid value for defrule: {}", value),
        }
    }
}
impl AddAssign<u32> for defrule {
    fn add_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for defrule {
    fn sub_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for defrule {
    fn mul_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for defrule {
    fn div_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for defrule {
    fn rem_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for defrule {
    type Output = defrule;
    fn add(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for defrule {
    type Output = defrule;
    fn sub(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for defrule {
    type Output = defrule;
    fn mul(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for defrule {
    type Output = defrule;
    fn div(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for defrule {
    type Output = defrule;
    fn rem(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
    pub sub: C2RustUnnamed_1,
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
pub union C2RustUnnamed_1 {
    pub nodep: C2RustUnnamed_3,
    pub val: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_5,
    pub x: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub d: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
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
pub union C2RustUnnamed_7 {
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
    pub hs: C2RustUnnamed_9,
    pub hi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum lintvals {
    LINT_no_effect = 2,
    LINT_assign_in_cond = 1,
    LINT_illegal = 0,
}
impl lintvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            lintvals::LINT_no_effect => 2,
            lintvals::LINT_assign_in_cond => 1,
            lintvals::LINT_illegal => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> lintvals {
        match value {
            2 => lintvals::LINT_no_effect,
            1 => lintvals::LINT_assign_in_cond,
            0 => lintvals::LINT_illegal,
            _ => panic!("Invalid value for lintvals: {}", value),
        }
    }
}
impl AddAssign<u32> for lintvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for lintvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for lintvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for lintvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for lintvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for lintvals {
    type Output = lintvals;
    fn add(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for lintvals {
    type Output = lintvals;
    fn sub(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for lintvals {
    type Output = lintvals;
    fn mul(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for lintvals {
    type Output = lintvals;
    fn div(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for lintvals {
    type Output = lintvals;
    fn rem(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redirval {
    redirect_twoway = 6,
    redirect_input = 5,
    redirect_pipein = 4,
    redirect_pipe = 3,
    redirect_append = 2,
    redirect_output = 1,
    redirect_none = 0,
}
impl redirval {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            redirval::redirect_twoway => 6,
            redirval::redirect_input => 5,
            redirval::redirect_pipein => 4,
            redirval::redirect_pipe => 3,
            redirval::redirect_append => 2,
            redirval::redirect_output => 1,
            redirval::redirect_none => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> redirval {
        match value {
            6 => redirval::redirect_twoway,
            5 => redirval::redirect_input,
            4 => redirval::redirect_pipein,
            3 => redirval::redirect_pipe,
            2 => redirval::redirect_append,
            1 => redirval::redirect_output,
            0 => redirval::redirect_none,
            _ => panic!("Invalid value for redirval: {}", value),
        }
    }
}
impl AddAssign<u32> for redirval {
    fn add_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redirval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redirval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redirval {
    fn div_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redirval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redirval {
    type Output = redirval;
    fn add(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redirval {
    type Output = redirval;
    fn sub(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redirval {
    type Output = redirval;
    fn mul(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redirval {
    type Output = redirval;
    fn div(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redirval {
    type Output = redirval;
    fn rem(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type INSTRUCTION = exp_instruction;
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
pub struct flagtab {
    pub val: i32,
    pub name: *const i8,
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
pub type Func_pre_exec = Option<unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32>;
pub type Func_post_exec = Option<unsafe extern "C" fn(*mut INSTRUCTION) -> ()>;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sort_context_t {
    SORTED_IN = 1,
    ASORT,
    ASORTI,
}
impl sort_context_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sort_context_t::SORTED_IN => 1,
            sort_context_t::ASORT => 2,
            sort_context_t::ASORTI => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> sort_context_t {
        match value {
            1 => sort_context_t::SORTED_IN,
            2 => sort_context_t::ASORT,
            3 => sort_context_t::ASORTI,
            _ => panic!("Invalid value for sort_context_t: {}", value),
        }
    }
}
impl AddAssign<u32> for sort_context_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sort_context_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sort_context_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sort_context_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sort_context_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sort_context_t {
    type Output = sort_context_t;
    fn add(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sort_context_t {
    type Output = sort_context_t;
    fn sub(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sort_context_t {
    type Output = sort_context_t;
    fn mul(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sort_context_t {
    type Output = sort_context_t;
    fn div(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sort_context_t {
    type Output = sort_context_t;
    fn rem(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type builtin_func_t = Option<unsafe extern "C" fn(i32) -> *mut NODE>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optypetab {
    pub desc: *mut i8,
    pub operator: *mut i8,
}
pub type EXEC_STATE = exec_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_state {
    pub next: *mut exec_state,
    pub cptr: *mut INSTRUCTION,
    pub rule: i32,
    pub stack_size: i64,
    pub source: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum scalar_cmp_t {
    SCALAR_EQ,
    SCALAR_NEQ,
    SCALAR_LT,
    SCALAR_LE,
    SCALAR_GT,
    SCALAR_GE,
}
impl scalar_cmp_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            scalar_cmp_t::SCALAR_EQ => 0,
            scalar_cmp_t::SCALAR_NEQ => 1,
            scalar_cmp_t::SCALAR_LT => 2,
            scalar_cmp_t::SCALAR_LE => 3,
            scalar_cmp_t::SCALAR_GT => 4,
            scalar_cmp_t::SCALAR_GE => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> scalar_cmp_t {
        match value {
            0 => scalar_cmp_t::SCALAR_EQ,
            1 => scalar_cmp_t::SCALAR_NEQ,
            2 => scalar_cmp_t::SCALAR_LT,
            3 => scalar_cmp_t::SCALAR_LE,
            4 => scalar_cmp_t::SCALAR_GT,
            5 => scalar_cmp_t::SCALAR_GE,
            _ => panic!("Invalid value for scalar_cmp_t: {}", value),
        }
    }
}
impl AddAssign<u32> for scalar_cmp_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for scalar_cmp_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for scalar_cmp_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for scalar_cmp_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for scalar_cmp_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for scalar_cmp_t {
    type Output = scalar_cmp_t;
    fn add(self, rhs: u32) -> scalar_cmp_t {
        scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for scalar_cmp_t {
    type Output = scalar_cmp_t;
    fn sub(self, rhs: u32) -> scalar_cmp_t {
        scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for scalar_cmp_t {
    type Output = scalar_cmp_t;
    fn mul(self, rhs: u32) -> scalar_cmp_t {
        scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for scalar_cmp_t {
    type Output = scalar_cmp_t;
    fn div(self, rhs: u32) -> scalar_cmp_t {
        scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for scalar_cmp_t {
    type Output = scalar_cmp_t;
    fn rem(self, rhs: u32) -> scalar_cmp_t {
        scalar_cmp_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as i32 as i64 {
        return;
    }
    r_unref(r);
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
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn TOP_SCALAR() -> *mut NODE {
    let mut t: *mut NODE = (*stack_ptr).rptr;
    if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 1896 as i32);
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
        (*stack_ptr).rptr = t;
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
unsafe extern "C" fn ezalloc_real(
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
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2070 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_calloc(1 as i32 as size_t, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2074 as i32);
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
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn POP_ARRAY(mut check_for_untyped: bool) -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    static mut warned: bool = 0 as i32 != 0;
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && !warned && check_for_untyped as i32 != 0
        && ((*t).type_0 as u32 == nodevals::Node_var_new as i32 as u32
            || (*t).type_0 as u32 == nodevals::Node_elem_new as i32 as u32)
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 1857 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"behavior of `for' loop on untyped variable is not defined by POSIX\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    return if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        t
    } else {
        force_array(t, 1 as i32 != 0)
    };
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
#[no_mangle]
pub static mut fcall_list: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
#[no_mangle]
pub static mut fcall_count: i64 = 0 as i32 as i64;
#[no_mangle]
pub static mut currule: i32 = 0 as i32;
#[no_mangle]
pub static mut curfile: *mut IOBUF = 0 as *const IOBUF as *mut IOBUF;
#[no_mangle]
pub static mut exiting: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut interpret: Option<unsafe extern "C" fn(*mut INSTRUCTION) -> i32> = None;
static mut num_exec_hook: i32 = 0 as i32;
static mut pre_execute: [Func_pre_exec; 10] = [None; 10];
static mut post_execute: Func_post_exec = None;
#[no_mangle]
pub static mut OFSlen: i32 = 0;
#[no_mangle]
pub static mut ORSlen: i32 = 0;
#[no_mangle]
pub static mut OFMTidx: i32 = 0;
#[no_mangle]
pub static mut CONVFMTidx: i32 = 0;
static mut node_Boolean: [*mut NODE; 2] = [0 as *const NODE as *mut NODE; 2];
#[no_mangle]
pub static mut casetable: [i8; 256] = [
    '\0' as i32 as i8,
    '\u{1}' as i32 as i8,
    '\u{2}' as i32 as i8,
    '\u{3}' as i32 as i8,
    '\u{4}' as i32 as i8,
    '\u{5}' as i32 as i8,
    '\u{6}' as i32 as i8,
    '\u{7}' as i32 as i8,
    '\u{8}' as i32 as i8,
    '\t' as i32 as i8,
    '\n' as i32 as i8,
    '\u{b}' as i32 as i8,
    '\u{c}' as i32 as i8,
    '\r' as i32 as i8,
    '\u{e}' as i32 as i8,
    '\u{f}' as i32 as i8,
    '\u{10}' as i32 as i8,
    '\u{11}' as i32 as i8,
    '\u{12}' as i32 as i8,
    '\u{13}' as i32 as i8,
    '\u{14}' as i32 as i8,
    '\u{15}' as i32 as i8,
    '\u{16}' as i32 as i8,
    '\u{17}' as i32 as i8,
    '\u{18}' as i32 as i8,
    '\u{19}' as i32 as i8,
    '\u{1a}' as i32 as i8,
    '\u{1b}' as i32 as i8,
    '\u{1c}' as i32 as i8,
    '\u{1d}' as i32 as i8,
    '\u{1e}' as i32 as i8,
    '\u{1f}' as i32 as i8,
    ' ' as i32 as i8,
    '!' as i32 as i8,
    '"' as i32 as i8,
    '#' as i32 as i8,
    '$' as i32 as i8,
    '%' as i32 as i8,
    '&' as i32 as i8,
    '\'' as i32 as i8,
    '(' as i32 as i8,
    ')' as i32 as i8,
    '*' as i32 as i8,
    '+' as i32 as i8,
    ',' as i32 as i8,
    '-' as i32 as i8,
    '.' as i32 as i8,
    '/' as i32 as i8,
    '0' as i32 as i8,
    '1' as i32 as i8,
    '2' as i32 as i8,
    '3' as i32 as i8,
    '4' as i32 as i8,
    '5' as i32 as i8,
    '6' as i32 as i8,
    '7' as i32 as i8,
    '8' as i32 as i8,
    '9' as i32 as i8,
    ':' as i32 as i8,
    ';' as i32 as i8,
    '<' as i32 as i8,
    '=' as i32 as i8,
    '>' as i32 as i8,
    '?' as i32 as i8,
    '@' as i32 as i8,
    'a' as i32 as i8,
    'b' as i32 as i8,
    'c' as i32 as i8,
    'd' as i32 as i8,
    'e' as i32 as i8,
    'f' as i32 as i8,
    'g' as i32 as i8,
    'h' as i32 as i8,
    'i' as i32 as i8,
    'j' as i32 as i8,
    'k' as i32 as i8,
    'l' as i32 as i8,
    'm' as i32 as i8,
    'n' as i32 as i8,
    'o' as i32 as i8,
    'p' as i32 as i8,
    'q' as i32 as i8,
    'r' as i32 as i8,
    's' as i32 as i8,
    't' as i32 as i8,
    'u' as i32 as i8,
    'v' as i32 as i8,
    'w' as i32 as i8,
    'x' as i32 as i8,
    'y' as i32 as i8,
    'z' as i32 as i8,
    '[' as i32 as i8,
    '\\' as i32 as i8,
    ']' as i32 as i8,
    '^' as i32 as i8,
    '_' as i32 as i8,
    '`' as i32 as i8,
    'a' as i32 as i8,
    'b' as i32 as i8,
    'c' as i32 as i8,
    'd' as i32 as i8,
    'e' as i32 as i8,
    'f' as i32 as i8,
    'g' as i32 as i8,
    'h' as i32 as i8,
    'i' as i32 as i8,
    'j' as i32 as i8,
    'k' as i32 as i8,
    'l' as i32 as i8,
    'm' as i32 as i8,
    'n' as i32 as i8,
    'o' as i32 as i8,
    'p' as i32 as i8,
    'q' as i32 as i8,
    'r' as i32 as i8,
    's' as i32 as i8,
    't' as i32 as i8,
    'u' as i32 as i8,
    'v' as i32 as i8,
    'w' as i32 as i8,
    'x' as i32 as i8,
    'y' as i32 as i8,
    'z' as i32 as i8,
    '{' as i32 as i8,
    '|' as i32 as i8,
    '}' as i32 as i8,
    '~' as i32 as i8,
    '\u{7f}' as i32 as i8,
    -128i32 as i8,
    -127i32 as i8,
    -126i32 as i8,
    -125i32 as i8,
    -124i32 as i8,
    -123i32 as i8,
    -122i32 as i8,
    -121i32 as i8,
    -120i32 as i8,
    -119i32 as i8,
    -118i32 as i8,
    -117i32 as i8,
    -116i32 as i8,
    -115i32 as i8,
    -114i32 as i8,
    -113i32 as i8,
    -112i32 as i8,
    -111i32 as i8,
    -110i32 as i8,
    -109i32 as i8,
    -108i32 as i8,
    -107i32 as i8,
    -106i32 as i8,
    -105i32 as i8,
    -104i32 as i8,
    -103i32 as i8,
    -102i32 as i8,
    -101i32 as i8,
    -100i32 as i8,
    -99i32 as i8,
    -98i32 as i8,
    -97i32 as i8,
    -96i32 as i8,
    -95i32 as i8,
    -94i32 as i8,
    -93i32 as i8,
    -92i32 as i8,
    -91i32 as i8,
    -90i32 as i8,
    -89i32 as i8,
    -88i32 as i8,
    -87i32 as i8,
    -86i32 as i8,
    -85i32 as i8,
    -84i32 as i8,
    -83i32 as i8,
    -82i32 as i8,
    -81i32 as i8,
    -80i32 as i8,
    -79i32 as i8,
    -78i32 as i8,
    -77i32 as i8,
    -76i32 as i8,
    -75i32 as i8,
    -74i32 as i8,
    -73i32 as i8,
    -72i32 as i8,
    -71i32 as i8,
    -70i32 as i8,
    -69i32 as i8,
    -68i32 as i8,
    -67i32 as i8,
    -66i32 as i8,
    -65i32 as i8,
    -32i32 as i8,
    -31i32 as i8,
    -30i32 as i8,
    -29i32 as i8,
    -28i32 as i8,
    -27i32 as i8,
    -26i32 as i8,
    -25i32 as i8,
    -24i32 as i8,
    -23i32 as i8,
    -22i32 as i8,
    -21i32 as i8,
    -20i32 as i8,
    -19i32 as i8,
    -18i32 as i8,
    -17i32 as i8,
    -16i32 as i8,
    -15i32 as i8,
    -14i32 as i8,
    -13i32 as i8,
    -12i32 as i8,
    -11i32 as i8,
    -10i32 as i8,
    -41i32 as i8,
    -8i32 as i8,
    -7i32 as i8,
    -6i32 as i8,
    -5i32 as i8,
    -4i32 as i8,
    -3i32 as i8,
    -2i32 as i8,
    -33i32 as i8,
    -32i32 as i8,
    -31i32 as i8,
    -30i32 as i8,
    -29i32 as i8,
    -28i32 as i8,
    -27i32 as i8,
    -26i32 as i8,
    -25i32 as i8,
    -24i32 as i8,
    -23i32 as i8,
    -22i32 as i8,
    -21i32 as i8,
    -20i32 as i8,
    -19i32 as i8,
    -18i32 as i8,
    -17i32 as i8,
    -16i32 as i8,
    -15i32 as i8,
    -14i32 as i8,
    -13i32 as i8,
    -12i32 as i8,
    -11i32 as i8,
    -10i32 as i8,
    -9i32 as i8,
    -8i32 as i8,
    -7i32 as i8,
    -6i32 as i8,
    -5i32 as i8,
    -4i32 as i8,
    -3i32 as i8,
    -2i32 as i8,
    -1i32 as i8,
];
#[no_mangle]
pub unsafe extern "C" fn load_casetable() {
    let mut i: i32 = 0;
    static mut loaded: bool = 0 as i32 != 0;
    if loaded as i32 != 0
        || do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
    {
        return;
    }
    loaded = 1 as i32 != 0;
    i = 0o200 as i32;
    while i <= 0o377 as i32 {
        if *(*__ctype_b_loc()).offset(i as isize) as i32
            & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 != 0
            && *(*__ctype_b_loc()).offset(i as isize) as i32
                & C2RustUnnamed::_ISlower as i32 as libc::c_ushort as i32 != 0
            && i
                != ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = i;
                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(i);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(i as isize);
                    }
                    __res
                })
        {
            casetable[i as usize] = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = i;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(i);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(i as isize);
                }
                __res
            }) as i8;
        } else {
            casetable[i as usize] = i as i8;
        }
        i += 1;
        i;
    }
}
static mut nodetypes: [*const i8; 21] = [
    b"nodevals::Node_illegal\0" as *const u8 as *const i8,
    b"nodevals::Node_val\0" as *const u8 as *const i8,
    b"nodevals::Node_regex\0" as *const u8 as *const i8,
    b"nodevals::Node_dynregex\0" as *const u8 as *const i8,
    b"nodevals::Node_var\0" as *const u8 as *const i8,
    b"nodevals::Node_var_array\0" as *const u8 as *const i8,
    b"nodevals::Node_var_new\0" as *const u8 as *const i8,
    b"nodevals::Node_elem_new\0" as *const u8 as *const i8,
    b"nodevals::Node_param_list\0" as *const u8 as *const i8,
    b"nodevals::Node_func\0" as *const u8 as *const i8,
    b"nodevals::Node_ext_func\0" as *const u8 as *const i8,
    b"nodevals::Node_builtin_func\0" as *const u8 as *const i8,
    b"nodevals::Node_array_ref\0" as *const u8 as *const i8,
    b"nodevals::Node_array_tree\0" as *const u8 as *const i8,
    b"nodevals::Node_array_leaf\0" as *const u8 as *const i8,
    b"nodevals::Node_dump_array\0" as *const u8 as *const i8,
    b"nodevals::Node_arrayfor\0" as *const u8 as *const i8,
    b"nodevals::Node_frame\0" as *const u8 as *const i8,
    b"nodevals::Node_instruction\0" as *const u8 as *const i8,
    b"nodevals::Node_final --- this should never appear\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut optypes: [optypetab; 124] = [
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_illegal\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_times\0" as *const u8 as *const i8 as *mut i8,
            operator: b" * \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_times_i\0" as *const u8 as *const i8 as *mut i8,
            operator: b" * \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_quotient\0" as *const u8 as *const i8 as *mut i8,
            operator: b" / \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_quotient_i\0" as *const u8 as *const i8 as *mut i8,
            operator: b" / \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_mod\0" as *const u8 as *const i8 as *mut i8,
            operator: b" % \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_mod_i\0" as *const u8 as *const i8 as *mut i8,
            operator: b" % \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_plus\0" as *const u8 as *const i8 as *mut i8,
            operator: b" + \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_plus_i\0" as *const u8 as *const i8 as *mut i8,
            operator: b" + \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_minus\0" as *const u8 as *const i8 as *mut i8,
            operator: b" - \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_minus_i\0" as *const u8 as *const i8 as *mut i8,
            operator: b" - \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_exp\0" as *const u8 as *const i8 as *mut i8,
            operator: b" ^ \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_exp_i\0" as *const u8 as *const i8 as *mut i8,
            operator: b" ^ \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_concat\0" as *const u8 as *const i8 as *mut i8,
            operator: b" \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_line_range\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_cond_pair\0" as *const u8 as *const i8 as *mut i8,
            operator: b", \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_subscript\0" as *const u8 as *const i8 as *mut i8,
            operator: b"[]\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_sub_array\0" as *const u8 as *const i8 as *mut i8,
            operator: b"[]\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_preincrement\0" as *const u8 as *const i8 as *mut i8,
            operator: b"++\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_predecrement\0" as *const u8 as *const i8 as *mut i8,
            operator: b"--\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_postincrement\0" as *const u8 as *const i8 as *mut i8,
            operator: b"++\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_postdecrement\0" as *const u8 as *const i8 as *mut i8,
            operator: b"--\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_unary_minus\0" as *const u8 as *const i8 as *mut i8,
            operator: b"-\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_unary_plus\0" as *const u8 as *const i8 as *mut i8,
            operator: b"+\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_field_spec\0" as *const u8 as *const i8 as *mut i8,
            operator: b"$\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_not\0" as *const u8 as *const i8 as *mut i8,
            operator: b"! \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign\0" as *const u8 as *const i8 as *mut i8,
            operator: b" = \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_store_var\0" as *const u8 as *const i8 as *mut i8,
            operator: b" = \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_store_sub\0" as *const u8 as *const i8 as *mut i8,
            operator: b" = \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_store_field\0" as *const u8 as *const i8 as *mut i8,
            operator: b" = \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_store_field_exp\0" as *const u8 as *const i8
                as *mut i8,
            operator: b" = \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_times\0" as *const u8 as *const i8 as *mut i8,
            operator: b" *= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_quotient\0" as *const u8 as *const i8
                as *mut i8,
            operator: b" /= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_mod\0" as *const u8 as *const i8 as *mut i8,
            operator: b" %= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_plus\0" as *const u8 as *const i8 as *mut i8,
            operator: b" += \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_minus\0" as *const u8 as *const i8 as *mut i8,
            operator: b" -= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_exp\0" as *const u8 as *const i8 as *mut i8,
            operator: b" ^= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_assign_concat\0" as *const u8 as *const i8 as *mut i8,
            operator: b" \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_and\0" as *const u8 as *const i8 as *mut i8,
            operator: b" && \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_and_final\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_or\0" as *const u8 as *const i8 as *mut i8,
            operator: b" || \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_or_final\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_equal\0" as *const u8 as *const i8 as *mut i8,
            operator: b" == \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_notequal\0" as *const u8 as *const i8 as *mut i8,
            operator: b" != \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_less\0" as *const u8 as *const i8 as *mut i8,
            operator: b" < \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_greater\0" as *const u8 as *const i8 as *mut i8,
            operator: b" > \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_leq\0" as *const u8 as *const i8 as *mut i8,
            operator: b" <= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_geq\0" as *const u8 as *const i8 as *mut i8,
            operator: b" >= \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_match\0" as *const u8 as *const i8 as *mut i8,
            operator: b" ~ \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_match_rec\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_nomatch\0" as *const u8 as *const i8 as *mut i8,
            operator: b" !~ \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_rule\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_case\0" as *const u8 as *const i8 as *mut i8,
            operator: b"case\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_default\0" as *const u8 as *const i8 as *mut i8,
            operator: b"default\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_break\0" as *const u8 as *const i8 as *mut i8,
            operator: b"break\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_continue\0" as *const u8 as *const i8 as *mut i8,
            operator: b"continue\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_print\0" as *const u8 as *const i8 as *mut i8,
            operator: b"print\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_print_rec\0" as *const u8 as *const i8 as *mut i8,
            operator: b"print\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_printf\0" as *const u8 as *const i8 as *mut i8,
            operator: b"printf\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_next\0" as *const u8 as *const i8 as *mut i8,
            operator: b"next\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_exit\0" as *const u8 as *const i8 as *mut i8,
            operator: b"exit\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_return\0" as *const u8 as *const i8 as *mut i8,
            operator: b"return\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_return_from_eval\0" as *const u8 as *const i8
                as *mut i8,
            operator: b"return\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_delete\0" as *const u8 as *const i8 as *mut i8,
            operator: b"delete\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_delete_loop\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_getline_redir\0" as *const u8 as *const i8
                as *mut i8,
            operator: b"getline\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_getline\0" as *const u8 as *const i8 as *mut i8,
            operator: b"getline\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_nextfile\0" as *const u8 as *const i8 as *mut i8,
            operator: b"nextfile\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_namespace\0" as *const u8 as *const i8 as *mut i8,
            operator: b"@namespace\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_builtin\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_sub_builtin\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_ext_builtin\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_in_array\0" as *const u8 as *const i8 as *mut i8,
            operator: b" in \0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_func_call\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_indirect_func_call\0" as *const u8 as *const i8
                as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_arg\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_arg_untyped\0" as *const u8 as *const i8
                as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_i\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_re\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_array\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_param\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_push_lhs\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_subscript_lhs\0" as *const u8 as *const i8 as *mut i8,
            operator: b"[]\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_field_spec_lhs\0" as *const u8 as *const i8 as *mut i8,
            operator: b"$\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_no_op\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_pop\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_jmp\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_jmp_true\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_jmp_false\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_get_record\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_newfile\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_arrayfor_init\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_arrayfor_incr\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_arrayfor_final\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_var_update\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_var_assign\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_field_assign\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_subscript_assign\0" as *const u8 as *const i8
                as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_after_beginfile\0" as *const u8 as *const i8
                as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_after_endfile\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_func\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_comment\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_exec_count\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_breakpoint\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_lint\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_lint_plus\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_atexit\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_stop\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_token\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_symbol\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_list\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_do\0" as *const u8 as *const i8 as *mut i8,
            operator: b"do\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_for\0" as *const u8 as *const i8 as *mut i8,
            operator: b"for\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_arrayfor\0" as *const u8 as *const i8 as *mut i8,
            operator: b"for\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_while\0" as *const u8 as *const i8 as *mut i8,
            operator: b"while\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_switch\0" as *const u8 as *const i8 as *mut i8,
            operator: b"switch\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_if\0" as *const u8 as *const i8 as *mut i8,
            operator: b"if\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_else\0" as *const u8 as *const i8 as *mut i8,
            operator: b"else\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_K_function\0" as *const u8 as *const i8 as *mut i8,
            operator: b"function\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_cond_exp\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_parens\0" as *const u8 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"opcodeval::Op_final --- this should never appear\0" as *const u8
                as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: 0 as *const i8 as *mut i8,
            operator: 0 as *const i8 as *mut i8,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn nodetype2str(mut type_0: NODETYPE) -> *const i8 {
    static mut buf: [i8; 40] = [0; 40];
    if type_0 as u32 >= nodevals::Node_illegal as i32 as u32
        && type_0 as u32 <= nodevals::Node_final as i32 as u32
    {
        return nodetypes[type_0 as i32 as usize];
    }
    sprintf(
        buf.as_mut_ptr(),
        dcgettext(
            0 as *const i8,
            b"unknown nodetype %d\0" as *const u8 as *const i8,
            5 as i32,
        ),
        type_0 as i32,
    );
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn opcode2str(mut op: OPCODE) -> *const i8 {
    if op as u32 >= opcodeval::Op_illegal as i32 as u32
        && (op as u32) < opcodeval::Op_final as i32 as u32
    {
        return optypes[op as i32 as usize].desc;
    }
    (set_loc
        as unsafe extern "C" fn(
            *const i8,
            i32,
        ) -> ())(b"eval.c\0" as *const u8 as *const i8, 416 as i32);
    (Some(
        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        dcgettext(
            0 as *const i8,
            b"unknown opcode %d\0" as *const u8 as *const i8,
            5 as i32,
        ),
        op as i32,
    );
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn op2str(mut op: OPCODE) -> *const i8 {
    if op as u32 >= opcodeval::Op_illegal as i32 as u32
        && (op as u32) < opcodeval::Op_final as i32 as u32
    {
        if !(optypes[op as i32 as usize].operator).is_null() {
            return optypes[op as i32 as usize].operator
        } else {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"eval.c\0" as *const u8 as *const i8, 429 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"opcode %s not an operator or keyword\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                optypes[op as i32 as usize].desc,
            );
        }
    } else {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 432 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"unknown opcode %d\0" as *const u8 as *const i8,
                5 as i32,
            ),
            op as i32,
        );
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn flags2str(mut flagval: i32) -> *const i8 {
    static mut values: [flagtab; 21] = [
        {
            let mut init = flagtab {
                val: flagvals::MALLOC as i32,
                name: b"flagvals::MALLOC\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::STRING as i32,
                name: b"flagvals::STRING\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::STRCUR as i32,
                name: b"flagvals::STRCUR\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::NUMCUR as i32,
                name: b"flagvals::NUMCUR\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::NUMBER as i32,
                name: b"flagvals::NUMBER\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::USER_INPUT as i32,
                name: b"flagvals::USER_INPUT\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::BOOLVAL as i32,
                name: b"BOOL\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::INTLSTR as i32,
                name: b"flagvals::INTLSTR\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::NUMINT as i32,
                name: b"flagvals::NUMINT\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::INTIND as i32,
                name: b"flagvals::INTIND\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::WSTRCUR as i32,
                name: b"flagvals::WSTRCUR\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::MPFN as i32,
                name: b"flagvals::MPFN\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::MPZN as i32,
                name: b"flagvals::MPZN\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::NO_EXT_SET as i32,
                name: b"flagvals::NO_EXT_SET\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::NULL_FIELD as i32,
                name: b"flagvals::NULL_FIELD\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::ARRAYMAXED as i32,
                name: b"flagvals::ARRAYMAXED\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::HALFHAT as i32,
                name: b"flagvals::HALFHAT\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::XARRAY as i32,
                name: b"flagvals::XARRAY\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::NUMCONSTSTR as i32,
                name: b"flagvals::NUMCONSTSTR\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: flagvals::REGEX as i32,
                name: b"flagvals::REGEX\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: 0 as i32,
                name: 0 as *const i8,
            };
            init
        },
    ];
    return genflags2str(flagval, values.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn genflags2str(
    mut flagval: i32,
    mut tab: *const flagtab,
) -> *const i8 {
    static mut buffer: [i8; 8192] = [0; 8192];
    let mut sp: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut space_left: i32 = 0;
    let mut space_needed: i32 = 0;
    sp = buffer.as_mut_ptr();
    space_left = 8192 as i32;
    i = 0 as i32;
    while !((*tab.offset(i as isize)).name).is_null() {
        if flagval & (*tab.offset(i as isize)).val != 0 as i32 {
            space_needed = (strlen((*tab.offset(i as isize)).name))
                .wrapping_add((sp != buffer.as_mut_ptr()) as i32 as u64) as i32;
            if space_left <= space_needed {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"eval.c\0" as *const u8 as *const i8, 488 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"buffer overflow in genflags2str\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            if sp != buffer.as_mut_ptr() {
                let fresh2 = sp;
                sp = sp.offset(1);
                *fresh2 = '|' as i32 as i8;
                space_left -= 1;
                space_left;
            }
            strcpy(sp, (*tab.offset(i as isize)).name);
            space_left = (space_left as u64).wrapping_sub(strlen(sp)) as i32 as i32;
            sp = sp.offset(strlen(sp) as isize);
        }
        i += 1;
        i;
    }
    *sp = '\0' as i32 as i8;
    return buffer.as_mut_ptr();
}
unsafe extern "C" fn posix_compare(mut s1: *mut NODE, mut s2: *mut NODE) -> i32 {
    let mut ret: i32 = 0;
    if gawk_mb_cur_max == 1 as i32 {
        let mut save1: i8 = 0;
        let mut save2: i8 = 0;
        let mut p1: *const i8 = 0 as *const i8;
        let mut p2: *const i8 = 0 as *const i8;
        save1 = *((*s1).sub.val.sp).offset((*s1).sub.val.slen as isize);
        *((*s1).sub.val.sp).offset((*s1).sub.val.slen as isize) = '\0' as i32 as i8;
        save2 = *((*s2).sub.val.sp).offset((*s2).sub.val.slen as isize);
        *((*s2).sub.val.sp).offset((*s2).sub.val.slen as isize) = '\0' as i32 as i8;
        p1 = (*s1).sub.val.sp;
        p2 = (*s2).sub.val.sp;
        loop {
            let mut len: size_t = 0;
            ret = strcoll(p1, p2);
            if ret != 0 as i32 {
                break;
            }
            len = strlen(p1);
            p1 = p1.offset(len.wrapping_add(1 as i32 as u64) as isize);
            p2 = p2.offset(len.wrapping_add(1 as i32 as u64) as isize);
            if p1
                == ((*s1).sub.val.sp)
                    .offset((*s1).sub.val.slen as isize)
                    .offset(1 as i32 as isize)
            {
                if p2
                    != ((*s2).sub.val.sp)
                        .offset((*s2).sub.val.slen as isize)
                        .offset(1 as i32 as isize)
                {
                    ret = -(1 as i32);
                }
                break;
            } else {
                if !(p2
                    == ((*s2).sub.val.sp)
                        .offset((*s2).sub.val.slen as isize)
                        .offset(1 as i32 as isize))
                {
                    continue;
                }
                ret = 1 as i32;
                break;
            }
        }
        *((*s1).sub.val.sp).offset((*s1).sub.val.slen as isize) = save1;
        *((*s2).sub.val.sp).offset((*s2).sub.val.slen as isize) = save2;
    } else {
        let mut p1_0: *const wchar_t = 0 as *const wchar_t;
        let mut p2_0: *const wchar_t = 0 as *const wchar_t;
        str2wstr(s1, 0 as *mut *mut size_t);
        str2wstr(s2, 0 as *mut *mut size_t);
        p1_0 = (*s1).sub.val.wsp;
        p2_0 = (*s2).sub.val.wsp;
        loop {
            let mut len_0: size_t = 0;
            ret = wcscoll(p1_0, p2_0);
            if ret != 0 as i32 {
                break;
            }
            len_0 = wcslen(p1_0);
            p1_0 = p1_0.offset(len_0.wrapping_add(1 as i32 as u64) as isize);
            p2_0 = p2_0.offset(len_0.wrapping_add(1 as i32 as u64) as isize);
            if p1_0
                == ((*s1).sub.val.wsp)
                    .offset((*s1).sub.val.wslen as isize)
                    .offset(1 as i32 as isize)
            {
                if p2_0
                    != ((*s2).sub.val.wsp)
                        .offset((*s2).sub.val.wslen as isize)
                        .offset(1 as i32 as isize)
                {
                    ret = -(1 as i32);
                }
                break;
            } else {
                if !(p2_0
                    == ((*s2).sub.val.wsp)
                        .offset((*s2).sub.val.wslen as isize)
                        .offset(1 as i32 as isize))
                {
                    continue;
                }
                ret = 1 as i32;
                break;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cmp_nodes(
    mut t1: *mut NODE,
    mut t2: *mut NODE,
    mut use_strcmp: bool,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut l: i32 = 0;
    let mut ldiff: i32 = 0;
    if t1 == t2 {
        return 0 as i32;
    }
    fixtype(t1);
    fixtype(t2);
    if (*t1).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
        && (*t2).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
    {
        return cmp_numbers.expect("non-null function pointer")(t1, t2);
    }
    force_string_fmt(t1, CONVFMT, CONVFMTidx);
    force_string_fmt(t2, CONVFMT, CONVFMTidx);
    len1 = (*t1).sub.val.slen;
    len2 = (*t2).sub.val.slen;
    ldiff = len1.wrapping_sub(len2) as i32;
    if len1 == 0 as i32 as u64 || len2 == 0 as i32 as u64 {
        return ldiff;
    }
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 && !use_strcmp {
        return posix_compare(t1, t2);
    }
    l = (if ldiff <= 0 as i32 { len1 } else { len2 }) as i32;
    if IGNORECASE {
        let mut cp1: *const u8 = (*t1).sub.val.sp as *const u8;
        let mut cp2: *const u8 = (*t2).sub.val.sp as *const u8;
        let mut save1: i8 = *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize);
        let mut save2: i8 = *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize);
        if gawk_mb_cur_max > 1 as i32 {
            let ref mut fresh3 = *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize);
            *fresh3 = '\0' as i32 as i8;
            *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = *fresh3;
            ret = strncasecmpmbs(cp1, cp2, l as size_t);
            *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
            *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save2;
        } else {
            ret = 0 as i32;
            loop {
                let fresh4 = l;
                l = l - 1;
                if !(fresh4 > 0 as i32 && ret == 0 as i32) {
                    break;
                }
                ret = casetable[*cp1 as usize] as i32 - casetable[*cp2 as usize] as i32;
                cp1 = cp1.offset(1);
                cp1;
                cp2 = cp2.offset(1);
                cp2;
            }
        }
    } else {
        ret = memcmp(
            (*t1).sub.val.sp as *const libc::c_void,
            (*t2).sub.val.sp as *const libc::c_void,
            l as u64,
        );
    }
    ret = if ret == 0 as i32 { ldiff } else { ret };
    return ret;
}
unsafe extern "C" fn push_frame(mut f: *mut NODE) {
    static mut max_fcall: i64 = 0;
    fcall_count += 1;
    fcall_count;
    if fcall_list.is_null() {
        max_fcall = 10 as i32 as i64;
        fcall_list = emalloc_real(
            ((max_fcall + 1 as i32 as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"push_frame\0" as *const u8 as *const i8,
            b"fcall_list\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            654 as i32,
        ) as *mut *mut NODE;
    } else if fcall_count == max_fcall {
        max_fcall *= 2 as i32 as i64;
        fcall_list = erealloc_real(
            fcall_list as *mut libc::c_void,
            ((max_fcall + 1 as i32 as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"push_frame\0" as *const u8 as *const i8,
            b"fcall_list\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            657 as i32,
        ) as *mut *mut NODE;
    }
    if fcall_count > 1 as i32 as i64 {
        memmove(
            fcall_list.offset(2 as i32 as isize) as *mut libc::c_void,
            fcall_list.offset(1 as i32 as isize) as *const libc::c_void,
            ((fcall_count - 1 as i32 as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        );
    }
    let ref mut fresh5 = *fcall_list.offset(1 as i32 as isize);
    *fresh5 = f;
}
unsafe extern "C" fn pop_frame() {
    if fcall_count > 1 as i32 as i64 {
        memmove(
            fcall_list.offset(1 as i32 as isize) as *mut libc::c_void,
            fcall_list.offset(2 as i32 as isize) as *const libc::c_void,
            ((fcall_count - 1 as i32 as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        );
    }
    fcall_count -= 1;
    fcall_count;
    if do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0 {
        frame_popped();
    }
}
#[no_mangle]
pub unsafe extern "C" fn dump_fcall_stack(mut fp: *mut FILE) {
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut i: i64 = 0 as i32 as i64;
    let mut k: i64 = 0 as i32 as i64;
    if fcall_count == 0 as i32 as i64 {
        return;
    }
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"\n\t# Function Call Stack:\n\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    func = (*frame_ptr).sub.nodep.x.extra;
    let fresh6 = k;
    k = k + 1;
    fprintf(
        fp,
        b"\t# %3ld. %s\n\0" as *const u8 as *const i8,
        fresh6,
        (*func).sub.nodep.name,
    );
    i = 1 as i32 as i64;
    while i < fcall_count {
        f = *fcall_list.offset(i as isize);
        func = (*f).sub.nodep.x.extra;
        let fresh7 = k;
        k = k + 1;
        fprintf(
            fp,
            b"\t# %3ld. %s\n\0" as *const u8 as *const i8,
            fresh7,
            (*func).sub.nodep.name,
        );
        i += 1;
        i;
    }
    fprintf(fp, b"\t# %3ld. -- main --\n\0" as *const u8 as *const i8, k);
}
#[no_mangle]
pub unsafe extern "C" fn set_IGNORECASE() {
    static mut warned: bool = 0 as i32 != 0;
    if (do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0)
        && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 716 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"`IGNORECASE' is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        IGNORECASE = 0 as i32 != 0;
    } else {
        IGNORECASE = boolval((*IGNORECASE_node).sub.nodep.l.lptr);
    }
    set_RS();
}
#[no_mangle]
pub unsafe extern "C" fn set_BINMODE() {
    let mut current_block: u64;
    static mut warned: bool = 0 as i32 != 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut v: *mut NODE = fixtype((*BINMODE_node).sub.nodep.l.lptr);
    if (do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0)
        && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 737 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"`BINMODE' is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        BINMODE = binmode_values::TEXT_TRANSLATE as i32;
    } else if (*v).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32 {
        BINMODE = (*v).sub.val.fltnum as i64 as i32;
        if BINMODE < binmode_values::TEXT_TRANSLATE as i32 {
            BINMODE = binmode_values::TEXT_TRANSLATE as i32;
        } else if BINMODE > binmode_values::BINMODE_BOTH as i32 {
            BINMODE = binmode_values::BINMODE_BOTH as i32;
        }
    } else if (*v).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32 {
        p = (*v).sub.val.sp;
        match (*v).sub.val.slen {
            1 => {
                match *p.offset(0 as i32 as isize) as i32 {
                    48 | 49 | 50 | 51 => {
                        BINMODE = *p.offset(0 as i32 as isize) as i32 - '0' as i32;
                        current_block = 18377268871191777778;
                    }
                    114 => {
                        BINMODE = binmode_values::BINMODE_INPUT as i32;
                        current_block = 18377268871191777778;
                    }
                    119 => {
                        BINMODE = binmode_values::BINMODE_OUTPUT as i32;
                        current_block = 18377268871191777778;
                    }
                    _ => {
                        BINMODE = binmode_values::BINMODE_BOTH as i32;
                        current_block = 14035247036432658994;
                    }
                }
            }
            2 => {
                match *p.offset(0 as i32 as isize) as i32 {
                    114 => {
                        current_block = 11561196344241271117;
                        match current_block {
                            14784417845850896246 => {
                                BINMODE = binmode_values::BINMODE_BOTH as i32;
                                if *p.offset(1 as i32 as isize) as i32 != 'r' as i32 {
                                    current_block = 14035247036432658994;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                            _ => {
                                BINMODE = binmode_values::BINMODE_BOTH as i32;
                                if *p.offset(1 as i32 as isize) as i32 != 'w' as i32 {
                                    current_block = 14035247036432658994;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                        }
                    }
                    119 => {
                        current_block = 14784417845850896246;
                        match current_block {
                            14784417845850896246 => {
                                BINMODE = binmode_values::BINMODE_BOTH as i32;
                                if *p.offset(1 as i32 as isize) as i32 != 'r' as i32 {
                                    current_block = 14035247036432658994;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                            _ => {
                                BINMODE = binmode_values::BINMODE_BOTH as i32;
                                if *p.offset(1 as i32 as isize) as i32 != 'w' as i32 {
                                    current_block = 14035247036432658994;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                        }
                    }
                    _ => {
                        current_block = 18377268871191777778;
                    }
                }
            }
            _ => {
                current_block = 14035247036432658994;
            }
        }
        match current_block {
            18377268871191777778 => {}
            _ => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"eval.c\0" as *const u8 as *const i8, 794 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"BINMODE value `%s' is invalid, treated as 3\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    p,
                );
            }
        }
    } else {
        BINMODE = 3 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_OFS() {
    static mut first: bool = 1 as i32 != 0;
    let mut new_ofs_len: size_t = 0;
    if first {
        first = 0 as i32 != 0;
    } else if !field0_valid {
        get_field(9223372036854775807 as i64 - 1 as i32 as i64, 0 as *mut Func_ptr);
        rebuild_record();
    }
    (*OFS_node).sub.nodep.l.lptr = force_string_fmt(
        (*OFS_node).sub.nodep.l.lptr,
        CONVFMT,
        CONVFMTidx,
    );
    new_ofs_len = (*(*OFS_node).sub.nodep.l.lptr).sub.val.slen;
    if OFS.is_null() {
        OFS = emalloc_real(
            new_ofs_len.wrapping_add(1 as i32 as u64),
            b"set_OFS\0" as *const u8 as *const i8,
            b"OFS\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            829 as i32,
        ) as *mut i8;
    } else if (OFSlen as u64) < new_ofs_len {
        OFS = erealloc_real(
            OFS as *mut libc::c_void,
            new_ofs_len.wrapping_add(1 as i32 as u64),
            b"set_OFS\0" as *const u8 as *const i8,
            b"OFS\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            831 as i32,
        ) as *mut i8;
    }
    memcpy(
        OFS as *mut libc::c_void,
        (*(*OFS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
        (*(*OFS_node).sub.nodep.l.lptr).sub.val.slen,
    );
    OFSlen = new_ofs_len as i32;
    *OFS.offset(OFSlen as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn set_ORS() {
    (*ORS_node).sub.nodep.l.lptr = force_string_fmt(
        (*ORS_node).sub.nodep.l.lptr,
        CONVFMT,
        CONVFMTidx,
    );
    ORS = (*(*ORS_node).sub.nodep.l.lptr).sub.val.sp;
    ORSlen = (*(*ORS_node).sub.nodep.l.lptr).sub.val.slen as i32;
}
#[no_mangle]
pub static mut fmt_list: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
unsafe extern "C" fn fmt_ok(mut n: *mut NODE) -> i32 {
    let mut tmp: *mut NODE = force_string_fmt(n, CONVFMT, CONVFMTidx);
    let mut p: *const i8 = (*tmp).sub.val.sp;
    static mut float_formats: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"efgEFG\0")
    };
    static mut flags: [i8; 6] = unsafe {
        *::core::mem::transmute::<&[u8; 6], &[i8; 6]>(b" +-#'\0")
    };
    let fresh8 = p;
    p = p.offset(1);
    if *fresh8 as i32 != '%' as i32 {
        return 0 as i32;
    }
    while *p as i32 != 0 && !(strchr(flags.as_ptr(), *p as i32)).is_null() {
        p = p.offset(1);
        p;
    }
    while *p as i32 != 0
        && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        p = p.offset(1);
        p;
    }
    if *p as i32 == '\0' as i32
        || *p as i32 != '.' as i32
            && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 == 0
    {
        return 0 as i32;
    }
    if *p as i32 == '.' as i32 {
        p = p.offset(1);
        p;
    }
    while *p as i32 != 0
        && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        p = p.offset(1);
        p;
    }
    if *p as i32 == '\0' as i32 || (strchr(float_formats.as_ptr(), *p as i32)).is_null()
    {
        return 0 as i32;
    }
    p = p.offset(1);
    if *p as i32 != '\0' as i32 {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn fmt_index(mut n: *mut NODE) -> i32 {
    let mut ix: i32 = 0 as i32;
    static mut fmt_num: i32 = 4 as i32;
    static mut fmt_hiwater: i32 = 0 as i32;
    let mut save: i8 = 0;
    if fmt_list.is_null() {
        fmt_list = emalloc_real(
            (fmt_num as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"fmt_index\0" as *const u8 as *const i8,
            b"fmt_list\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            903 as i32,
        ) as *mut *mut NODE;
    }
    n = force_string_fmt(n, CONVFMT, CONVFMTidx);
    save = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as i8;
    while ix < fmt_hiwater {
        if cmp_nodes(*fmt_list.offset(ix as isize), n, 1 as i32 != 0) == 0 as i32 {
            return ix;
        }
        ix += 1;
        ix;
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && fmt_ok(n) == 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 917 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"bad `%sFMT' specification `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            if n == (*CONVFMT_node).sub.nodep.l.lptr {
                b"CONV\0" as *const u8 as *const i8
            } else if n == (*OFMT_node).sub.nodep.l.lptr {
                b"O\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            (*n).sub.val.sp,
        );
    }
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = save;
    if fmt_hiwater >= fmt_num {
        fmt_num *= 2 as i32;
        fmt_list = erealloc_real(
            fmt_list as *mut libc::c_void,
            (fmt_num as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"fmt_index\0" as *const u8 as *const i8,
            b"fmt_list\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            926 as i32,
        ) as *mut *mut NODE;
    }
    let ref mut fresh9 = *fmt_list.offset(fmt_hiwater as isize);
    *fresh9 = dupnode(n);
    let fresh10 = fmt_hiwater;
    fmt_hiwater = fmt_hiwater + 1;
    return fresh10;
}
#[no_mangle]
pub unsafe extern "C" fn set_OFMT() {
    OFMTidx = fmt_index((*OFMT_node).sub.nodep.l.lptr);
    OFMT = (**fmt_list.offset(OFMTidx as isize)).sub.val.sp;
}
#[no_mangle]
pub unsafe extern "C" fn set_CONVFMT() {
    CONVFMTidx = fmt_index((*CONVFMT_node).sub.nodep.l.lptr);
    CONVFMT = (**fmt_list.offset(CONVFMTidx as isize)).sub.val.sp;
}
#[no_mangle]
pub unsafe extern "C" fn set_LINT() {
    let mut old_lint: i32 = (do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32) as i32;
    let mut n: *mut NODE = fixtype((*LINT_node).sub.nodep.l.lptr);
    lintfunc = Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ());
    do_flags = ::core::mem::transmute::<
        u32,
        do_flag_values,
    >(
        do_flags as u32
            & !(do_flag_values::DO_LINT_ALL as i32
                | do_flag_values::DO_LINT_INVALID as i32) as u32,
    );
    if (*n).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32 {
        let mut lintval: *const i8 = 0 as *const i8;
        let mut lintlen: size_t = 0;
        lintval = (*n).sub.val.sp;
        lintlen = (*n).sub.val.slen;
        if lintlen > 0 as i32 as u64 {
            if lintlen == 7 as i32 as u64
                && strncmp(
                    lintval,
                    b"invalid\0" as *const u8 as *const i8,
                    7 as i32 as u64,
                ) == 0 as i32
            {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_LINT_INVALID as i32 as u32);
            } else if lintlen == 6 as i32 as u64
                && strncmp(
                    lintval,
                    b"no-ext\0" as *const u8 as *const i8,
                    6 as i32 as u64,
                ) == 0 as i32
            {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 & !(do_flag_values::DO_LINT_EXTENSIONS as i32) as u32);
            } else {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_LINT_ALL as i32 as u32);
                if lintlen == 5 as i32 as u64
                    && strncmp(
                        lintval,
                        b"fatal\0" as *const u8 as *const i8,
                        5 as i32 as u64,
                    ) == 0 as i32
                {
                    lintfunc = Some(
                        r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                    );
                }
            }
        }
    } else if !((*n).sub.val.fltnum == 0.0f64) {
        do_flags = ::core::mem::transmute::<
            u32,
            do_flag_values,
        >(do_flags as u32 | do_flag_values::DO_LINT_ALL as i32 as u32);
    }
    if old_lint as u32
        != do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 && old_lint != 0
        && do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 == 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 987 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"turning off `--lint' due to assignment to `LINT'\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    update_ext_api();
}
#[no_mangle]
pub unsafe extern "C" fn set_TEXTDOMAIN() {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    (*TEXTDOMAIN_node).sub.nodep.l.lptr = force_string_fmt(
        (*TEXTDOMAIN_node).sub.nodep.l.lptr,
        CONVFMT,
        CONVFMTidx,
    );
    tmp = (*TEXTDOMAIN_node).sub.nodep.l.lptr;
    TEXTDOMAIN = (*tmp).sub.val.sp;
}
#[no_mangle]
pub unsafe extern "C" fn update_ERRNO_int(mut errcode: i32) {
    let mut cp: *const i8 = 0 as *const i8;
    update_PROCINFO_num(b"errno\0" as *const u8 as *const i8, errcode as libc::c_double);
    if errcode != 0 {
        cp = strerror(errcode);
        cp = dcgettext(0 as *const i8, cp, 5 as i32);
    } else {
        cp = b"\0" as *const u8 as *const i8;
    }
    unref((*ERRNO_node).sub.nodep.l.lptr);
    (*ERRNO_node).sub.nodep.l.lptr = make_str_node(cp, strlen(cp), 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn update_ERRNO_string(mut string: *const i8) {
    update_PROCINFO_num(
        b"errno\0" as *const u8 as *const i8,
        0 as i32 as libc::c_double,
    );
    unref((*ERRNO_node).sub.nodep.l.lptr);
    let mut len: size_t = strlen(string);
    (*ERRNO_node).sub.nodep.l.lptr = make_str_node(string, len, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn unset_ERRNO() {
    update_PROCINFO_num(
        b"errno\0" as *const u8 as *const i8,
        0 as i32 as libc::c_double,
    );
    unref((*ERRNO_node).sub.nodep.l.lptr);
    (*ERRNO_node).sub.nodep.l.lptr = dupnode(Nnull_string);
}
#[no_mangle]
pub unsafe extern "C" fn update_NR() {
    if (*(*NR_node).sub.nodep.l.lptr).sub.val.fltnum != NR as libc::c_double {
        unref((*NR_node).sub.nodep.l.lptr);
        (*NR_node).sub.nodep.l.lptr = make_number
            .expect("non-null function pointer")(NR as libc::c_double);
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_NF() {
    let mut l: i64 = 0;
    l = (*(*NF_node).sub.nodep.l.lptr).sub.val.fltnum as i64;
    if NF == -(1 as i32) as i64 || l != NF {
        if NF == -(1 as i32) as i64 {
            get_field(9223372036854775807 as i64 - 1 as i32 as i64, 0 as *mut Func_ptr);
        }
        unref((*NF_node).sub.nodep.l.lptr);
        (*NF_node).sub.nodep.l.lptr = make_number
            .expect("non-null function pointer")(NF as libc::c_double);
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_FNR() {
    if (*(*FNR_node).sub.nodep.l.lptr).sub.val.fltnum != FNR as libc::c_double {
        unref((*FNR_node).sub.nodep.l.lptr);
        (*FNR_node).sub.nodep.l.lptr = make_number
            .expect("non-null function pointer")(FNR as libc::c_double);
    }
}
#[no_mangle]
pub static mut frame_ptr: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut stack_ptr: *mut STACK_ITEM = 0 as *const STACK_ITEM as *mut STACK_ITEM;
#[no_mangle]
pub static mut stack_bottom: *mut STACK_ITEM = 0 as *const STACK_ITEM as *mut STACK_ITEM;
#[no_mangle]
pub static mut stack_top: *mut STACK_ITEM = 0 as *const STACK_ITEM as *mut STACK_ITEM;
static mut STACK_SIZE: u64 = 256 as i32 as u64;
#[no_mangle]
pub static mut max_args: i32 = 0 as i32;
#[no_mangle]
pub static mut args_array: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
#[no_mangle]
pub unsafe extern "C" fn grow_stack() -> *mut STACK_ITEM {
    STACK_SIZE = STACK_SIZE.wrapping_mul(2 as i32 as u64);
    stack_bottom = erealloc_real(
        stack_bottom as *mut libc::c_void,
        STACK_SIZE.wrapping_mul(::core::mem::size_of::<STACK_ITEM>() as u64),
        b"grow_stack\0" as *const u8 as *const i8,
        b"stack_bottom\0" as *const u8 as *const i8,
        b"eval.c\0" as *const u8 as *const i8,
        1132 as i32,
    ) as *mut STACK_ITEM;
    stack_top = stack_bottom.offset(STACK_SIZE as isize).offset(-(1 as i32 as isize));
    stack_ptr = stack_bottom.offset(STACK_SIZE.wrapping_div(2 as i32 as u64) as isize);
    return stack_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn r_get_lhs(
    mut n: *mut NODE,
    mut reference: bool,
) -> *mut *mut NODE {
    let mut isparam: bool = 0 as i32 != 0;
    if (*n).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
        isparam = 1 as i32 != 0;
        n = *((*frame_ptr).sub.nodep.r.av).offset((*n).sub.nodep.l.ll as isize);
    }
    let mut current_block_19: u64;
    match (*n).type_0 as u32 {
        5 => {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1156 as i32);
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
                array_vname(n),
            );
            return 0 as *mut *mut NODE;
        }
        12 => {
            if (*(*n).sub.nodep.l.lptr).type_0 as u32
                == nodevals::Node_var_array as i32 as u32
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1161 as i32);
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
                    array_vname(n),
                );
            }
            if (*(*n).sub.nodep.l.lptr).type_0 as u32 != nodevals::Node_var as i32 as u32
            {
                (*(*n).sub.nodep.l.lptr).type_0 = nodevals::Node_var;
                (*(*n).sub.nodep.l.lptr).sub.nodep.l.lptr = dupnode(Nnull_string);
            }
            current_block_19 = 14934394078603883031;
        }
        6 => {
            current_block_19 = 14934394078603883031;
        }
        7 => {
            pma_free((*n).sub.val.sp as *mut libc::c_void);
            (*n).sub.val.sp = 0 as *mut i8;
            (*n).sub.val.slen = 0 as i32 as size_t;
            (*n).type_0 = nodevals::Node_var;
            (*n).sub.nodep.l.lptr = dupnode(Nnull_string);
            current_block_19 = 5948590327928692120;
        }
        4 => {
            current_block_19 = 5948590327928692120;
        }
        _ => {
            r_fatal(
                b"internal error: file %s, line %d: unexpected variable type %s\0"
                    as *const u8 as *const i8,
                b"eval.c\0" as *const u8 as *const i8,
                1185 as i32,
                nodetype2str((*n).type_0),
            );
            current_block_19 = 5948590327928692120;
        }
    }
    match current_block_19 {
        14934394078603883031 => {
            (*n).type_0 = nodevals::Node_var;
            (*n).sub.nodep.l.lptr = dupnode(Nnull_string);
        }
        _ => {}
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && reference as i32 != 0 && (*n).sub.nodep.l.lptr == Nnull_string
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1189 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            if isparam as i32 != 0 {
                dcgettext(
                    0 as *const i8,
                    b"reference to uninitialized argument `%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"reference to uninitialized variable `%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                )
            },
            (*n).sub.nodep.name,
        );
    }
    return &mut (*n).sub.nodep.l.lptr;
}
#[no_mangle]
pub unsafe extern "C" fn r_get_field(
    mut n: *mut NODE,
    mut assign: *mut Func_ptr,
    mut reference: bool,
) -> *mut *mut NODE {
    let mut field_num: i64 = 0;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    if !assign.is_null() {
        *assign = None;
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if (*fixtype(n)).flags as u32 & flagvals::NUMBER as i32 as u32 == 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1209 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"attempt to field reference from non-numeric value\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            if (*n).sub.val.slen == 0 as i32 as u64 {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1211 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"attempt to field reference from null string\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
        }
    }
    force_number(n);
    field_num = (*n).sub.val.fltnum as i64;
    if field_num < 0 as i32 as i64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1219 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to access field %ld\0" as *const u8 as *const i8,
                5 as i32,
            ),
            field_num,
        );
    }
    if field_num == 0 as i32 as i64 && field0_valid as i32 != 0 {
        lhs = &mut *fields_arr.offset(0 as i32 as isize) as *mut *mut NODE;
        if !assign.is_null() {
            *assign = Some(reset_record as unsafe extern "C" fn() -> ());
        }
    } else {
        lhs = get_field(field_num, assign);
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && reference as i32 != 0
        && (**lhs).flags as u32 & flagvals::NULL_FIELD as i32 as u32 != 0 as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1228 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"reference to uninitialized field `$%ld'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            field_num,
        );
    }
    return lhs;
}
unsafe extern "C" fn calc_exp_posint(
    mut x: libc::c_double,
    mut n: i64,
) -> libc::c_double {
    let mut mult: libc::c_double = 1 as i32 as libc::c_double;
    while n > 1 as i32 as i64 {
        if n % 2 as i32 as i64 == 1 as i32 as i64 {
            mult *= x;
        }
        x *= x;
        n /= 2 as i32 as i64;
    }
    return mult * x;
}
#[no_mangle]
pub unsafe extern "C" fn calc_exp(
    mut x1: libc::c_double,
    mut x2: libc::c_double,
) -> libc::c_double {
    let mut lx: i64 = 0;
    lx = x2 as i64;
    if lx as libc::c_double == x2 {
        if lx == 0 as i32 as i64 {
            return 1 as i32 as libc::c_double;
        }
        return if lx > 0 as i32 as i64 {
            calc_exp_posint(x1, lx)
        } else {
            1.0f64 / calc_exp_posint(x1, -lx)
        };
    }
    return pow(x1, x2);
}
unsafe extern "C" fn setup_frame(mut pc: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut fp: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut pcount: i32 = 0;
    let mut arg_count: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    f = (*pc).x.xn;
    pcount = (*f).sub.nodep.l.ll as i32;
    fp = (*f).sub.nodep.rn;
    arg_count = (*pc.offset(1 as i32 as isize)).x.xl as i32;
    if pcount > 0 as i32 {
        sp = ezalloc_real(
            (pcount as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"setup_frame\0" as *const u8 as *const i8,
            b"sp\0" as *const u8 as *const i8,
            b"eval.c\0" as *const u8 as *const i8,
            1286 as i32,
        ) as *mut *mut NODE;
    }
    if arg_count > pcount {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1291 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"function `%s' called with more arguments than declared\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*f).sub.nodep.name,
        );
        loop {
            let fresh11 = stack_ptr;
            stack_ptr = stack_ptr.offset(-1);
            r = (*fresh11).rptr;
            if (*r).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                DEREF(r);
            }
            arg_count -= 1;
            if !(arg_count > pcount) {
                break;
            }
        }
    }
    i = 0 as i32;
    j = arg_count - 1 as i32;
    while i < pcount {
        r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
        if !r.is_null() {
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r
                as *mut block_item))
                .freep;
        } else {
            r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
        };
        memset(r as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<NODE>() as u64);
        let ref mut fresh12 = *sp.offset(i as isize);
        *fresh12 = r;
        if i >= arg_count {
            (*r).type_0 = nodevals::Node_var_new;
            (*r).sub.nodep.name = (*fp.offset(i as isize)).sub.nodep.name;
        } else {
            m = (*stack_ptr.offset(-(j as isize))).rptr;
            if (*m).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
                m = *((*frame_ptr).sub.nodep.r.av).offset((*m).sub.nodep.l.ll as isize);
            }
            if m == *fields_arr.offset(0 as i32 as isize) {
                DEREF(m);
                m = dupnode(m);
            }
            match (*m).type_0 as u32 {
                6 | 5 | 7 => {
                    (*r).type_0 = nodevals::Node_array_ref;
                    (*r).sub.nodep.r.rptr = m;
                    (*r).sub.nodep.l.lptr = (*r).sub.nodep.r.rptr;
                }
                12 => {
                    (*r).type_0 = nodevals::Node_array_ref;
                    (*r).sub.nodep.l.lptr = (*m).sub.nodep.l.lptr;
                    (*r).sub.nodep.r.rptr = m;
                }
                4 => {
                    (*r).type_0 = nodevals::Node_var;
                    (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                }
                1 => {
                    (*r).type_0 = nodevals::Node_var;
                    (*r).sub.nodep.l.lptr = m;
                }
                9 | 11 | 10 => {
                    (*r).type_0 = nodevals::Node_var;
                    (*r).sub.nodep.l.lptr = make_str_node(
                        (*m).sub.nodep.name,
                        strlen((*m).sub.nodep.name),
                        0 as i32,
                    );
                }
                _ => {
                    r_fatal(
                        b"internal error: file %s, line %d: unexpected parameter type %s\0"
                            as *const u8 as *const i8,
                        b"eval.c\0" as *const u8 as *const i8,
                        1360 as i32,
                        nodetype2str((*m).type_0),
                    );
                }
            }
            (*r).sub.nodep.name = (*fp.offset(i as isize)).sub.nodep.name;
        }
        i += 1;
        i;
        j -= 1;
        j;
    }
    stack_ptr = stack_ptr.offset(-arg_count as isize);
    if (*pc).opcode as u32 == opcodeval::Op_indirect_func_call as i32 as u32 {
        let fresh13 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        r = (*fresh13).rptr;
        DEREF(r);
    }
    (*frame_ptr).sub.nodep.name = source;
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0
    {
        push_frame(frame_ptr);
    }
    let ref mut fresh14 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh14 = frame_ptr;
    frame_ptr = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !frame_ptr.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(frame_ptr
            as *mut block_item))
            .freep;
    } else {
        frame_ptr = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*frame_ptr).type_0 = nodevals::Node_frame;
    (*frame_ptr).sub.nodep.r.av = sp;
    (*frame_ptr).sub.nodep.reflags = reflagvals::from_libc_c_uint(
        stack_ptr.offset_from(stack_bottom) as i64 as u32,
    );
    (*frame_ptr).sub.nodep.x.extra = f;
    (*frame_ptr).sub.nodep.name = 0 as *mut i8;
    (*frame_ptr).sub.nodep.l.li = pc;
    return (*f).sub.nodep.r.iptr;
}
unsafe extern "C" fn restore_frame(mut fp: *mut NODE) -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut n: i32 = 0;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut ri: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    func = (*frame_ptr).sub.nodep.x.extra;
    n = (*func).sub.nodep.l.ll as i32;
    sp = (*frame_ptr).sub.nodep.r.av;
    while n > 0 as i32 {
        let fresh15 = sp;
        sp = sp.offset(1);
        r = *fresh15;
        if (*r).type_0 as u32 == nodevals::Node_var as i32 as u32 {
            DEREF((*r).sub.nodep.l.lptr);
        } else if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            ((*(*r).sub.nodep.l.lp).clear)
                .expect("non-null function pointer")(r, 0 as *mut exp_node);
        }
        let ref mut fresh16 = (*(r as *mut block_item)).freep;
        *fresh16 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r as *mut block_item;
        n -= 1;
        n;
    }
    if !((*frame_ptr).sub.nodep.r.av).is_null() {
        pma_free((*frame_ptr).sub.nodep.r.av as *mut libc::c_void);
    }
    ri = (*frame_ptr).sub.nodep.l.li;
    let ref mut fresh17 = (*(frame_ptr as *mut block_item)).freep;
    *fresh17 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = frame_ptr as *mut block_item;
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0
    {
        pop_frame();
    }
    frame_ptr = fp;
    source = (*fp).sub.nodep.name;
    (*fp).sub.nodep.name = 0 as *mut i8;
    return (*ri).nexti;
}
#[inline]
unsafe extern "C" fn free_arrayfor(mut r: *mut NODE) {
    if !((*r).sub.nodep.r.av).is_null() {
        let mut n: *mut NODE = 0 as *mut NODE;
        let mut num_elems: size_t = (*r).sub.nodep.reflags as size_t;
        let mut list: *mut *mut NODE = (*r).sub.nodep.r.av;
        while num_elems > 0 as i32 as u64 {
            num_elems = num_elems.wrapping_sub(1);
            n = *list.offset(num_elems as isize);
            unref(n);
        }
        pma_free(list as *mut libc::c_void);
    }
    let ref mut fresh18 = (*(r as *mut block_item)).freep;
    *fresh18 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r as *mut block_item;
}
#[no_mangle]
pub unsafe extern "C" fn unwind_stack(mut n: i64) -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut sp: *mut STACK_ITEM = 0 as *mut STACK_ITEM;
    if stack_ptr < stack_bottom {
        return 0 as *mut INSTRUCTION;
    }
    sp = stack_bottom.offset(n as isize);
    if stack_ptr < sp {
        return 0 as *mut INSTRUCTION;
    }
    loop {
        let fresh19 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        r = (*fresh19).rptr;
        if r.is_null() {
            break;
        }
        match (*r).type_0 as u32 {
            17 => {
                cp = restore_frame(r);
            }
            16 => {
                free_arrayfor(r);
            }
            1 => {
                DEREF(r);
            }
            18 => {
                let ref mut fresh20 = (*(r as *mut block_item)).freep;
                *fresh20 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r
                    as *mut block_item;
            }
            _ => {
                if in_main_context() != 0 && !exiting {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1497 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"unwind_stack: unexpected type `%s'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        nodetype2str((*r).type_0),
                    );
                }
            }
        }
        if stack_ptr < sp {
            break;
        }
    }
    return cp;
}
#[inline]
unsafe extern "C" fn eval_condition(mut t: *mut NODE) -> bool {
    if t == node_Boolean[0 as i32 as usize] {
        return 0 as i32 != 0;
    }
    if t == node_Boolean[1 as i32 as usize] {
        return 1 as i32 != 0;
    }
    return boolval(t);
}
unsafe extern "C" fn cmp_scalars(mut comparison_type: scalar_cmp_t) -> bool {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut di: i32 = 0;
    let mut ret: bool = false;
    t2 = POP_SCALAR();
    t1 = (*stack_ptr).rptr;
    t1 = elem_new_to_scalar(t1);
    t2 = elem_new_to_scalar(t2);
    if (*t1).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        DEREF(t2);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1555 as i32);
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
    if (*t1).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
        || (*t2).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
    {
        let mut use_strcmp: bool = comparison_type as u32
            == scalar_cmp_t::SCALAR_EQ as i32 as u32
            || comparison_type as u32 == scalar_cmp_t::SCALAR_NEQ as i32 as u32;
        di = cmp_nodes(t1, t2, use_strcmp);
        match comparison_type as u32 {
            0 => {
                ret = di == 0 as i32;
            }
            1 => {
                ret = di != 0 as i32;
            }
            2 => {
                ret = di < 0 as i32;
            }
            3 => {
                ret = di <= 0 as i32;
            }
            4 => {
                ret = di > 0 as i32;
            }
            5 => {
                ret = di >= 0 as i32;
            }
            _ => {}
        }
    } else {
        fixtype(t1);
        fixtype(t2);
        ret = cmp_doubles(t1, t2, comparison_type);
    }
    DEREF(t1);
    DEREF(t2);
    return ret;
}
unsafe extern "C" fn cmp_doubles(
    mut t1: *const NODE,
    mut t2: *const NODE,
    mut comparison_type: scalar_cmp_t,
) -> bool {
    let mut t1_nan: bool = if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf((*t1).sub.val.fltnum as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isnan((*t1).sub.val.fltnum)
    } else {
        __isnanl(f128::f128::new((*t1).sub.val.fltnum))
    } != 0;
    let mut t2_nan: bool = if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf((*t2).sub.val.fltnum as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isnan((*t2).sub.val.fltnum)
    } else {
        __isnanl(f128::f128::new((*t2).sub.val.fltnum))
    } != 0;
    let mut ret: i32 = 0;
    if (t1_nan as i32 != 0 || t2_nan as i32 != 0)
        && comparison_type as u32 != scalar_cmp_t::SCALAR_NEQ as i32 as u32
    {
        return 0 as i32 != 0;
    }
    match comparison_type as u32 {
        0 => {
            ret = ((*t1).sub.val.fltnum == (*t2).sub.val.fltnum) as i32;
        }
        1 => {
            ret = ((*t1).sub.val.fltnum != (*t2).sub.val.fltnum) as i32;
        }
        2 => {
            ret = ((*t1).sub.val.fltnum < (*t2).sub.val.fltnum) as i32;
        }
        3 => {
            ret = ((*t1).sub.val.fltnum <= (*t2).sub.val.fltnum) as i32;
        }
        4 => {
            ret = ((*t1).sub.val.fltnum > (*t2).sub.val.fltnum) as i32;
        }
        5 => {
            ret = ((*t1).sub.val.fltnum >= (*t2).sub.val.fltnum) as i32;
        }
        _ => {}
    }
    return ret != 0;
}
unsafe extern "C" fn op_assign(mut op: OPCODE) {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut x: libc::c_double = 0.0f64;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let fresh21 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    lhs = (*fresh21).lptr;
    t1 = *lhs;
    x1 = (*force_number(t1)).sub.val.fltnum;
    t2 = TOP_SCALAR();
    x2 = (*force_number(t2)).sub.val.fltnum;
    DEREF(t2);
    match op as u32 {
        34 => {
            x = x1 + x2;
        }
        35 => {
            x = x1 - x2;
        }
        31 => {
            x = x1 * x2;
        }
        32 => {
            if x2 == 0 as i32 as libc::c_double {
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1672 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"division by zero attempted in `/='\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            x = x1 / x2;
        }
        33 => {
            if x2 == 0 as i32 as libc::c_double {
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"eval.c\0" as *const u8 as *const i8, 1679 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"division by zero attempted in `%%='\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            x = fmod(x1, x2);
        }
        36 => {
            x = calc_exp(x1, x2);
        }
        _ => {}
    }
    if (*t1).valref == 1 as i32 as i64
        && (*t1).flags as u32
            == (flagvals::MALLOC as i32 | flagvals::NUMCUR as i32
                | flagvals::NUMBER as i32) as u32
    {
        (*t1).sub.val.fltnum = x;
    } else {
        unref(t1);
        *lhs = make_number.expect("non-null function pointer")(x);
        t1 = *lhs;
    }
    (*t1).valref += 1;
    (*t1).valref;
    (*stack_ptr).rptr = t1;
}
#[no_mangle]
pub unsafe extern "C" fn PUSH_CODE(mut cp: *mut INSTRUCTION) {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r as *mut block_item))
            .freep;
    } else {
        r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*r).type_0 = nodevals::Node_instruction;
    (*r).sub.nodep.r.iptr = cp;
    let ref mut fresh22 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh22 = r;
}
#[no_mangle]
pub unsafe extern "C" fn POP_CODE() -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let fresh23 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    r = (*fresh23).rptr;
    cp = (*r).sub.nodep.r.iptr;
    let ref mut fresh24 = (*(r as *mut block_item)).freep;
    *fresh24 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r as *mut block_item;
    return cp;
}
static mut exec_state_stack: EXEC_STATE = EXEC_STATE {
    next: 0 as *const exec_state as *mut exec_state,
    cptr: 0 as *const INSTRUCTION as *mut INSTRUCTION,
    rule: 0,
    stack_size: 0,
    source: 0 as *const i8,
};
unsafe extern "C" fn push_exec_state(
    mut cp: *mut INSTRUCTION,
    mut rule: i32,
    mut src: *mut i8,
    mut sp: *mut STACK_ITEM,
) {
    let mut es: *mut EXEC_STATE = 0 as *mut EXEC_STATE;
    es = emalloc_real(
        ::core::mem::size_of::<EXEC_STATE>() as u64,
        b"push_exec_state\0" as *const u8 as *const i8,
        b"es\0" as *const u8 as *const i8,
        b"eval.c\0" as *const u8 as *const i8,
        1769 as i32,
    ) as *mut EXEC_STATE;
    (*es).rule = rule;
    (*es).cptr = cp;
    (*es).stack_size = sp.offset_from(stack_bottom) as i64 + 1 as i32 as i64;
    (*es).source = src;
    (*es).next = exec_state_stack.next;
    exec_state_stack.next = es;
}
unsafe extern "C" fn pop_exec_state(
    mut rule: *mut i32,
    mut src: *mut *mut i8,
    mut sz: *mut i64,
) -> *mut INSTRUCTION {
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut es: *mut EXEC_STATE = 0 as *mut EXEC_STATE;
    es = exec_state_stack.next;
    if es.is_null() {
        return 0 as *mut INSTRUCTION;
    }
    cp = (*es).cptr;
    if !rule.is_null() {
        *rule = (*es).rule;
    }
    if !src.is_null() {
        *src = (*es).source as *mut i8;
    }
    if !sz.is_null() {
        *sz = (*es).stack_size;
    }
    exec_state_stack.next = (*es).next;
    pma_free(es as *mut libc::c_void);
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn register_exec_hook(
    mut preh: Func_pre_exec,
    mut posth: Func_post_exec,
) -> i32 {
    let mut pos: i32 = 0 as i32;
    if preh.is_none() || post_execute.is_some() && posth.is_some() {
        return 0 as i32;
    }
    if num_exec_hook == 10 as i32 {
        return 0 as i32;
    }
    if num_exec_hook > 0 as i32 {
        pos = (do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0) as i32;
        if num_exec_hook > pos {
            memmove(
                pre_execute.as_mut_ptr().offset(pos as isize).offset(1 as i32 as isize)
                    as *mut libc::c_void,
                pre_execute.as_mut_ptr().offset(pos as isize) as *const libc::c_void,
                ((num_exec_hook - pos) as u64)
                    .wrapping_mul(::core::mem::size_of::<Func_pre_exec>() as u64),
            );
        }
    }
    pre_execute[pos as usize] = preh;
    num_exec_hook += 1;
    num_exec_hook;
    if posth.is_some() {
        post_execute = posth;
    }
    return 1 as i32;
}
#[inline]
unsafe extern "C" fn unfield(mut l: *mut *mut NODE, mut r: *mut *mut NODE) {
    if (**r).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32
        || (**r).valref == 1 as i32 as i64
    {
        *l = *r;
    } else {
        *l = dupnode(*r);
        DEREF(*r);
    };
}
#[no_mangle]
pub unsafe extern "C" fn h_interpret(mut code: *mut INSTRUCTION) -> i32 {
    let mut current_block: u64;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut op: OPCODE = opcodeval::Op_illegal;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut ni: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut x: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut di: i32 = 0;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut set_array: *mut NODE = 0 as *mut NODE;
    let mut set_idx: *mut NODE = 0 as *mut NODE;
    let mut in_indirect_call: bool = 0 as i32 != 0;
    pc = code;
    '_top: loop {
        if (*pc).source_line as i32 > 0 as i32 {
            sourceline = (*pc).source_line as i32;
        }
        di = 0 as i32;
        while di < num_exec_hook {
            if (pre_execute[di as usize]).expect("non-null function pointer")(&mut pc)
                == 0
            {
                continue '_top;
            }
            di += 1;
            di;
        }
        op = (*pc).opcode;
        if do_itrace {
            fprintf(stderr, b"+ %s\n\0" as *const u8 as *const i8, opcode2str(op));
            fflush(stderr);
        }
        match op as u32 {
            51 => {
                currule = (*pc).x.xl as i32;
                if currule == defrule::BEGINFILE as i32 {
                    set_record(
                        b"\0" as *const u8 as *const i8,
                        0 as i32 as size_t,
                        0 as *const awk_fieldwidth_info_t,
                    );
                }
                current_block = 16275318387282786645;
            }
            101 => {
                current_block = 16275318387282786645;
            }
            107 => {
                let mut stdio_problem: bool = 0 as i32 != 0;
                let mut got_EPIPE: bool = 0 as i32 != 0;
                source = 0 as *mut i8;
                sourceline = 0 as i32;
                nextfile(&mut curfile, 1 as i32 != 0);
                close_io(&mut stdio_problem, &mut got_EPIPE);
                if stdio_problem as i32 != 0 && !exiting && exit_val == 0 as i32 {
                    exit_val = 1 as i32;
                }
                close_extensions();
                if got_EPIPE {
                    signal(13 as i32, None);
                    kill(getpid(), 13 as i32);
                }
                current_block = 3518619798157913413;
            }
            108 => {
                break;
            }
            78 => {
                m = (*pc).d.dn;
                if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                    && (*m).flags as u32 & flagvals::INTLSTR as i32 as u32
                        != 0 as i32 as u32
                {
                    let mut orig: *mut i8 = 0 as *mut i8;
                    let mut trans: *mut i8 = 0 as *mut i8;
                    let mut save: i8 = 0;
                    save = *((*m).sub.val.sp).offset((*m).sub.val.slen as isize);
                    *((*m).sub.val.sp).offset((*m).sub.val.slen as isize) = '\0' as i32
                        as i8;
                    orig = (*m).sub.val.sp;
                    trans = dcgettext(TEXTDOMAIN, orig, 5 as i32);
                    *((*m).sub.val.sp).offset((*m).sub.val.slen as isize) = save;
                    if trans != orig {
                        m = make_str_node(trans, strlen(trans), 0 as i32);
                    } else {
                        (*m).valref += 1;
                        (*m).valref;
                    }
                } else {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh25 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh25 = m;
                current_block = 3518619798157913413;
            }
            75 | 76 | 77 => {
                let mut current_block_67: u64;
                let mut save_symbol: *mut NODE = 0 as *mut NODE;
                let mut isparam: bool = 0 as i32 != 0;
                m = (*pc).d.dn;
                save_symbol = m;
                if (*m).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
                    isparam = 1 as i32 != 0;
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                    save_symbol = m;
                    if (*m).type_0 as u32 == nodevals::Node_array_ref as i32 as u32 {
                        if (*(*m).sub.nodep.l.lptr).type_0 as u32
                            == nodevals::Node_var as i32 as u32
                        {
                            current_block_67 = 5634311649589774485;
                        } else {
                            m = (*m).sub.nodep.l.lptr;
                            current_block_67 = 3160140712158701372;
                        }
                    } else {
                        current_block_67 = 3160140712158701372;
                    }
                } else {
                    current_block_67 = 3160140712158701372;
                }
                match current_block_67 {
                    3160140712158701372 => {
                        match (*m).type_0 as u32 {
                            4 => {
                                current_block_67 = 16500597064430046275;
                                match current_block_67 {
                                    15911032364363321810 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    16500597064430046275 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    17647237853557353373 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                            6 => {
                                current_block_67 = 5634311649589774485;
                            }
                            7 => {
                                current_block_67 = 4797674137308262284;
                                match current_block_67 {
                                    15911032364363321810 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    16500597064430046275 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    17647237853557353373 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                            5 => {
                                current_block_67 = 17647237853557353373;
                                match current_block_67 {
                                    15911032364363321810 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    16500597064430046275 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    17647237853557353373 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                            _ => {
                                current_block_67 = 15911032364363321810;
                                match current_block_67 {
                                    15911032364363321810 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    16500597064430046275 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    17647237853557353373 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block_67 {
                    5634311649589774485 => {
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                216 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                if isparam as i32 != 0 {
                                    dcgettext(
                                        0 as *const i8,
                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    )
                                } else {
                                    dcgettext(
                                        0 as *const i8,
                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    )
                                },
                                (*save_symbol).sub.nodep.name,
                            );
                        }
                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32 {
                            (*m).type_0 = nodevals::Node_var;
                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                            m = dupnode(Nnull_string);
                        }
                        (*m).valref += 1;
                        (*m).valref;
                        let ref mut fresh27 = (*if stack_ptr < stack_top {
                            stack_ptr = stack_ptr.offset(1);
                            stack_ptr
                        } else {
                            grow_stack()
                        })
                            .rptr;
                        *fresh27 = m;
                    }
                    _ => {}
                }
                current_block = 3518619798157913413;
            }
            81 => {
                m = (*pc).d.dn;
                if (*m).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                }
                if (*m).type_0 as u32 == nodevals::Node_var as i32 as u32 {
                    m = (*m).sub.nodep.l.lptr;
                    (*m).valref += 1;
                    (*m).valref;
                    let ref mut fresh30 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh30 = m;
                    current_block = 3518619798157913413;
                } else {
                    current_block = 1528899951983446718;
                }
            }
            80 => {
                current_block = 1528899951983446718;
            }
            82 => {
                lhs = if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, (*pc).x.xl != 0)
                };
                let ref mut fresh32 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh32 = lhs;
                current_block = 3518619798157913413;
            }
            16 => {
                t2 = if (*pc).d.dl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as i32, 1 as i32 != 0)
                };
                t1 = POP_ARRAY(0 as i32 != 0);
                if (in_array(t1, t2)).is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            296 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            299 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"SYMTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            302 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as i32 as u64 {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                305 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if t1 == func_table {
                    static mut warned: bool = 0 as i32 != 0;
                    if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32
                        != 0 && !warned
                    {
                        warned = 1 as i32 != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            317 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"FUNCTAB is a gawk extension\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    r = t2;
                } else {
                    if t1 == symbol_table {
                        update_global_values();
                    }
                    r = *((*(*t1).sub.nodep.l.lp).lookup)
                        .expect("non-null function pointer")(t1, t2);
                }
                DEREF(t2);
                if t1 == symbol_table {
                    static mut warned_0: bool = 0 as i32 != 0;
                    if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32
                        != 0 && !warned_0
                    {
                        warned_0 = 1 as i32 != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            335 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"SYMTAB is a gawk extension\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    if (*r).type_0 as u32 == nodevals::Node_var as i32 as u32 {
                        r = (*r).sub.nodep.l.lptr;
                    } else if (*r).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    {
                        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                        r = (*r).sub.nodep.l.lptr;
                    }
                }
                if (*r).type_0 as u32 == nodevals::Node_val as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_var as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
                {
                    (*r).valref += 1;
                    (*r).valref;
                }
                let ref mut fresh33 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh33 = r;
                current_block = 3518619798157913413;
            }
            17 => {
                t2 = if (*pc).d.dl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as i32, 1 as i32 != 0)
                };
                t1 = POP_ARRAY(0 as i32 != 0);
                r = in_array(t1, t2);
                if r.is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            362 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            365 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"SYMTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            368 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as i32 as u64 {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                371 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if r.is_null() {
                    r = make_array();
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                    assoc_set(t1, t2, r);
                } else if (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
                    r = force_array(r, 0 as i32 != 0);
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                } else if (*r).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        388 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use scalar `%s[\"%.*s\"]' as an array\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as i32,
                        (*t2).sub.val.sp,
                    );
                } else {
                    DEREF(t2);
                }
                let ref mut fresh34 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh34 = r;
                current_block = 3518619798157913413;
            }
            83 => {
                t2 = if (*pc).d.dl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as i32, 1 as i32 != 0)
                };
                t1 = POP_ARRAY(0 as i32 != 0);
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    && (in_array(t1, t2)).is_null()
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if (*pc).x.xl != 0 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            402 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    }
                    if (*t2).sub.val.slen == 0 as i32 as u64 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            405 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"subscript of array `%s' is null string\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                        );
                    }
                }
                lhs = ((*(*t1).sub.nodep.l.lp).lookup)
                    .expect("non-null function pointer")(t1, t2);
                if (**lhs).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        411 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as i32,
                        (*t2).sub.val.sp,
                    );
                }
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        429 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as u32 == nodevals::Node_var as i32 as u32
                        || (**lhs).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    {
                        update_global_values();
                        (**lhs).type_0 = nodevals::Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            437 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                }
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    set_array = t1;
                    set_idx = t2;
                } else {
                    DEREF(t2);
                }
                let ref mut fresh35 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh35 = lhs;
                current_block = 3518619798157913413;
            }
            24 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, 0 as *mut Func_ptr, 1 as i32 != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                r = *lhs;
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh36 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh36 = r;
                current_block = 3518619798157913413;
            }
            84 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut (*(*pc).d.di).x.aptr, (*pc).x.xl != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                let ref mut fresh37 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh37 = lhs;
                current_block = 3518619798157913413;
            }
            105 => {
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    match (*pc).d.dl {
                        1 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                474 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"assignment used in conditional context\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: unexpected lint type value %d\0"
                                    as *const u8 as *const i8,
                                b"./interpret.h\0" as *const u8 as *const i8,
                                478 as i32,
                                (*pc).d.dl as i32,
                            );
                        }
                    }
                }
                current_block = 3518619798157913413;
            }
            106 => {
                t1 = (*stack_ptr).rptr;
                t2 = (*stack_ptr.offset(-(1 as i32 as isize))).rptr;
                if (*t1).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
                    && (*t2).flags as u32 & flagvals::STRING as i32 as u32
                        != 0 as i32 as u32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        489 as i32,
                    );
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"operator `+' used on two string values\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                current_block = 3518619798157913413;
            }
            54 | 55 | 87 => {
                if post_execute.is_some() {
                    post_execute.expect("non-null function pointer")(pc);
                }
                pc = (*pc).d.di;
                continue;
            }
            89 => {
                r = POP_SCALAR();
                di = eval_condition(r) as i32;
                DEREF(r);
                if di == 0 {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            88 => {
                r = POP_SCALAR();
                di = eval_condition(r) as i32;
                DEREF(r);
                if di != 0 {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            38 | 40 => {
                t1 = POP_SCALAR();
                di = eval_condition(t1) as i32;
                DEREF(t1);
                if op as u32 == opcodeval::Op_and as i32 as u32 && di != 0
                    || op as u32 == opcodeval::Op_or as i32 as u32 && di == 0
                {
                    current_block = 3518619798157913413;
                } else {
                    r = node_Boolean[di as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    let ref mut fresh38 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh38 = r;
                    ni = (*pc).d.di;
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*ni).nexti;
                    continue;
                }
            }
            39 | 41 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[eval_condition(t1) as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            25 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[!eval_condition(t1) as i32 as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            42 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_EQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            43 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_NEQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            44 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_LT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            45 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_GT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            46 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_LE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            47 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_GE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            8 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 2235334700535380937;
            }
            7 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 2235334700535380937;
            }
            10 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 5088478517898830498;
            }
            9 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 5088478517898830498;
            }
            2 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 16240336253164271420;
            }
            1 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 16240336253164271420;
            }
            12 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 17966935000565432821;
            }
            11 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 17966935000565432821;
            }
            4 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 7294098017659915583;
            }
            3 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 7294098017659915583;
            }
            6 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 11956501910097409974;
            }
            5 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 11956501910097409974;
            }
            18 | 19 => {
                x = if op as u32 == opcodeval::Op_preincrement as i32 as u32 {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                if (*t1).valref == 1 as i32 as i64
                    && (*t1).flags as u32
                        == (flagvals::MALLOC as i32 | flagvals::NUMCUR as i32
                            | flagvals::NUMBER as i32) as u32
                {
                    (*t1).sub.val.fltnum += x;
                    r = t1;
                } else {
                    *lhs = make_number
                        .expect("non-null function pointer")((*t1).sub.val.fltnum + x);
                    r = *lhs;
                    unref(t1);
                }
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            20 | 21 => {
                x = if op as u32 == opcodeval::Op_postincrement as i32 as u32 {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum);
                if (*t1).valref == 1 as i32 as i64
                    && (*t1).flags as u32
                        == (flagvals::MALLOC as i32 | flagvals::NUMCUR as i32
                            | flagvals::NUMBER as i32) as u32
                {
                    (*t1).sub.val.fltnum += x;
                } else {
                    *lhs = make_number
                        .expect("non-null function pointer")((*t1).sub.val.fltnum + x);
                    unref(t1);
                }
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            22 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")(-(*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            23 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            28 => {
                t1 = force_array((*pc).d.dn, 1 as i32 != 0);
                t2 = if (*pc).x.xl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as i32, 1 as i32 != 0)
                };
                lhs = ((*(*t1).sub.nodep.l.lp).lookup)
                    .expect("non-null function pointer")(t1, t2);
                if (**lhs).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        737 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as i32,
                        (*t2).sub.val.sp,
                    );
                }
                DEREF(t2);
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        755 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as u32 == nodevals::Node_var as i32 as u32
                        || (**lhs).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    {
                        update_global_values();
                        (**lhs).type_0 = nodevals::Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            763 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                }
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    (Some(
                        ((*(*t1).sub.nodep.l.lp).store)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(t1, t2);
                }
                DEREF(t2);
                current_block = 3518619798157913413;
            }
            27 => {
                lhs = if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as i32 != 0)
                };
                unref(*lhs);
                r = (*pc).x.xn;
                if !r.is_null() {
                    (*r).valref += 1;
                    (*r).valref;
                    *lhs = r;
                } else {
                    r = POP_SCALAR();
                    unfield(lhs, &mut r);
                }
                current_block = 3518619798157913413;
            }
            29 | 30 => {
                let mut assign: Func_ptr = None;
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut assign, 0 as i32 != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                assign.expect("non-null function pointer")();
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                if op as u32 == opcodeval::Op_store_field_exp as i32 as u32 {
                    (**lhs).valref += 1;
                    (**lhs).valref;
                    let ref mut fresh39 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh39 = *lhs;
                }
                current_block = 3518619798157913413;
            }
            37 => {
                lhs = if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as i32 != 0)
                };
                t1 = force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                if t1 != *lhs {
                    unref(*lhs);
                    if (*t1).valref == 1 as i32 as i64 {
                        *lhs = t1;
                    } else {
                        *lhs = dupnode(t1);
                    }
                }
                if t1 != t2 && (*t1).valref == 1 as i32 as i64
                    && (*t1).flags as u32
                        & (flagvals::MALLOC as i32 | flagvals::MPFN as i32
                            | flagvals::MPZN as i32) as u32
                        == flagvals::MALLOC as i32 as u32
                {
                    let mut nlen: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    (*t1).sub.val.sp = erealloc_real(
                        (*t1).sub.val.sp as *mut libc::c_void,
                        nlen.wrapping_add(1 as i32 as u64),
                        b"r_interpret\0" as *const u8 as *const i8,
                        b"t1->stptr\0" as *const u8 as *const i8,
                        b"./interpret.h\0" as *const u8 as *const i8,
                        843 as i32,
                    ) as *mut i8;
                    memcpy(
                        ((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize)
                            as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    (*t1).sub.val.slen = nlen;
                    *((*t1).sub.val.sp).offset(nlen as isize) = '\0' as i32 as i8;
                    (*t1).flags = ::core::mem::transmute::<
                        u32,
                        flagvals,
                    >((*t1).flags as u32 & flagvals::WSTRCUR as i32 as u32);
                    (*t1).flags = ::core::mem::transmute::<
                        u32,
                        flagvals,
                    >(
                        (*t1).flags as u32
                            | (flagvals::MALLOC as i32 | flagvals::STRING as i32
                                | flagvals::STRCUR as i32) as u32,
                    );
                    (*t1).sub.val.idx = -(1 as i32);
                    if (*t1).flags as u32 & flagvals::WSTRCUR as i32 as u32
                        != 0 as i32 as u32
                        && (*t2).flags as u32 & flagvals::WSTRCUR as i32 as u32
                            != 0 as i32 as u32
                    {
                        let mut wlen: size_t = ((*t1).sub.val.wslen)
                            .wrapping_add((*t2).sub.val.wslen);
                        (*t1).sub.val.wsp = erealloc_real(
                            (*t1).sub.val.wsp as *mut libc::c_void,
                            (::core::mem::size_of::<wchar_t>() as u64)
                                .wrapping_mul(wlen.wrapping_add(1 as i32 as u64)),
                            b"r_interpret\0" as *const u8 as *const i8,
                            b"t1->wstptr\0" as *const u8 as *const i8,
                            b"./interpret.h\0" as *const u8 as *const i8,
                            860 as i32,
                        ) as *mut wchar_t;
                        memcpy(
                            ((*t1).sub.val.wsp).offset((*t1).sub.val.wslen as isize)
                                as *mut libc::c_void,
                            (*t2).sub.val.wsp as *const libc::c_void,
                            ((*t2).sub.val.wslen)
                                .wrapping_mul(::core::mem::size_of::<wchar_t>() as u64),
                        );
                        (*t1).sub.val.wslen = wlen;
                        *((*t1).sub.val.wsp).offset(wlen as isize) = '\0' as i32;
                    } else if (**lhs).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0
                    {
                        r_free_wstr(*lhs);
                    }
                } else {
                    let mut nlen_0: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    let mut p: *mut i8 = 0 as *mut i8;
                    p = emalloc_real(
                        nlen_0.wrapping_add(1 as i32 as u64),
                        b"r_interpret\0" as *const u8 as *const i8,
                        b"p\0" as *const u8 as *const i8,
                        b"./interpret.h\0" as *const u8 as *const i8,
                        870 as i32,
                    ) as *mut i8;
                    memcpy(
                        p as *mut libc::c_void,
                        (*t1).sub.val.sp as *const libc::c_void,
                        (*t1).sub.val.slen,
                    );
                    memcpy(
                        p.offset((*t1).sub.val.slen as isize) as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    unref(*lhs);
                    *lhs = make_str_node(p, nlen_0, 2 as i32);
                    t1 = *lhs;
                }
                DEREF(t2);
                current_block = 3518619798157913413;
            }
            26 => {
                let fresh40 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh40).lptr;
                r = TOP_SCALAR();
                unref(*lhs);
                if (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
                    DEREF(r);
                    r = dupnode(Nnull_string);
                }
                (*r).valref += 1;
                (*r).valref;
                unfield(lhs, &mut r);
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            98 => {
                if !set_idx.is_null() {
                    di = 1 as i32;
                    if (*pc).d.dl == opcodeval::Op_sub_builtin as i32 as i64
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        } && (*r).sub.val.fltnum as i64 == 0 as i32 as i64
                    {
                        di = 0 as i32;
                    } else if ((*pc).d.dl == opcodeval::Op_K_getline as i32 as i64
                        || (*pc).d.dl == opcodeval::Op_K_getline_redir as i32 as i64)
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        } && (*r).sub.val.fltnum as i64 <= 0 as i32 as i64
                    {
                        di = 0 as i32;
                    }
                    if di != 0 {
                        (Some(
                            ((*(*set_array).sub.nodep.l.lp).store)
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(set_array, set_idx);
                    }
                    unref(set_idx);
                    set_idx = 0 as *mut NODE;
                }
                current_block = 3518619798157913413;
            }
            34 | 35 | 31 | 32 | 33 | 36 => {
                op_assign(op);
                current_block = 3518619798157913413;
            }
            95 => {
                ((*pc).x.aptr).expect("non-null function pointer")();
                current_block = 3518619798157913413;
            }
            96 | 97 => {
                r = (*stack_ptr).rptr;
                if !((*pc).d.dl == opcodeval::Op_sub_builtin as i32 as i64
                    && (*r).sub.val.fltnum as i64 == 0 as i32 as i64)
                {
                    if !(((*pc).d.dl == opcodeval::Op_K_getline as i32 as i64
                        || (*pc).d.dl == opcodeval::Op_K_getline_redir as i32 as i64)
                        && (*r).sub.val.fltnum as i64 <= 0 as i32 as i64)
                    {
                        if op as u32 == opcodeval::Op_var_assign as i32 as u32 {
                            ((*pc).x.aptr).expect("non-null function pointer")();
                        } else {
                            ((*pc).x.aptr).expect("non-null function pointer")();
                        }
                    }
                }
                current_block = 3518619798157913413;
            }
            13 => {
                r = concat_exp((*pc).x.xl as i32, (*pc).d.dl & 1 as i32 as i64 != 0);
                let ref mut fresh41 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh41 = r;
                current_block = 3518619798157913413;
            }
            52 => {
                if (*pc.offset(1 as i32 as isize)).x.xl != 0 {
                    let fresh42 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    m = (*fresh42).rptr;
                    t2 = TOP_SCALAR();
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    rp = re_update(m);
                    di = (research(
                        rp,
                        (*t2).sub.val.sp,
                        0 as i32,
                        (*t2).sub.val.slen,
                        0 as i32,
                    ) >= 0 as i32) as i32;
                } else {
                    t1 = POP_SCALAR();
                    t2 = TOP_SCALAR();
                    di = (cmp_nodes(t2, t1, 1 as i32 != 0) == 0 as i32) as i32;
                    DEREF(t1);
                }
                if di != 0 {
                    t2 = POP_SCALAR();
                    DEREF(t2);
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            63 => {
                t1 = POP_ARRAY(0 as i32 != 0);
                do_delete(t1, (*pc).x.xl as i32);
                stack_ptr = stack_ptr.offset(-(*pc).x.xl as isize);
                current_block = 3518619798157913413;
            }
            64 => {
                t1 = POP_ARRAY(0 as i32 != 0);
                let fresh43 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh43).lptr;
                do_delete_loop(t1, lhs);
                current_block = 3518619798157913413;
            }
            72 => {
                t1 = POP_ARRAY(0 as i32 != 0);
                t2 = if (*pc).x.xl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as i32, 1 as i32 != 0)
                };
                r = node_Boolean[(in_array(t1, t2)
                    != 0 as *mut libc::c_void as *mut NODE) as i32 as usize];
                DEREF(t2);
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh44 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh44 = r;
                current_block = 3518619798157913413;
            }
            92 => {
                let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
                let mut array: *mut NODE = 0 as *mut NODE;
                let mut sort_str: *mut NODE = 0 as *mut NODE;
                let mut num_elems: size_t = 0 as i32 as size_t;
                static mut sorted_in: *mut NODE = 0 as *const NODE as *mut NODE;
                let mut how_to_sort: *const i8 = b"@unsorted\0" as *const u8
                    as *const i8;
                let mut save_0: i8 = 0;
                let mut saved_end: bool = 0 as i32 != 0;
                array = POP_ARRAY(1 as i32 != 0);
                num_elems = (*array).sub.nodep.reflags as size_t;
                if !(num_elems == 0 as i32 as u64) {
                    if sorted_in.is_null() {
                        sorted_in = make_str_node(
                            b"sorted_in\0" as *const u8 as *const i8,
                            9 as i32 as size_t,
                            0 as i32,
                        );
                    }
                    sort_str = 0 as *mut NODE;
                    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
                        && !PROCINFO_node.is_null()
                    {
                        sort_str = in_array(PROCINFO_node, sorted_in);
                    }
                    if !sort_str.is_null() {
                        sort_str = force_string_fmt(sort_str, CONVFMT, CONVFMTidx);
                        if (*sort_str).sub.val.slen > 0 as i32 as u64 {
                            how_to_sort = (*sort_str).sub.val.sp;
                            str_terminate_f(sort_str, &mut save_0);
                            saved_end = 1 as i32 != 0;
                        }
                    }
                    list = assoc_list(array, how_to_sort, sort_context_t::SORTED_IN);
                    if saved_end {
                        *((*sort_str).sub.val.sp)
                            .offset((*sort_str).sub.val.slen as isize) = save_0;
                    }
                }
                r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
                if !r.is_null() {
                    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r
                        as *mut block_item))
                        .freep;
                } else {
                    r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
                };
                (*r).type_0 = nodevals::Node_arrayfor;
                (*r).sub.nodep.r.av = list;
                (*r).sub.nodep.reflags = reflagvals::from_libc_c_uint(num_elems as u32);
                (*r).sub.nodep.l.ll = -(1 as i32) as i64;
                (*r).sub.nodep.rn = array;
                let ref mut fresh45 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh45 = r;
                if num_elems == 0 as i32 as u64 {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            93 => {
                r = (*stack_ptr).rptr;
                (*r).sub.nodep.l.ll += 1;
                if (*r).sub.nodep.l.ll == (*r).sub.nodep.reflags as i64 {
                    let mut array_0: *mut NODE = 0 as *mut NODE;
                    array_0 = (*r).sub.nodep.rn;
                    if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        && (*array_0).sub.nodep.reflags as u32
                            != (*r).sub.nodep.reflags as u32
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            1070 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"for loop: array `%s' changed size from %ld to %ld during loop execution\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(array_0),
                            (*r).sub.nodep.reflags as i64,
                            (*array_0).sub.nodep.reflags as i64,
                        );
                    }
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    t1 = *((*r).sub.nodep.r.av).offset((*r).sub.nodep.l.ll as isize);
                    lhs = if (*(*pc).x.xn).type_0 as u32
                        == nodevals::Node_var as i32 as u32
                        && !((*(*pc).x.xn).sub.nodep.l.lptr == Nnull_string)
                    {
                        &mut (*(*pc).x.xn).sub.nodep.l.lptr
                    } else {
                        r_get_lhs((*pc).x.xn, 0 as i32 != 0)
                    };
                    unref(*lhs);
                    *lhs = dupnode(t1);
                }
                current_block = 3518619798157913413;
            }
            94 => {
                let fresh46 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                r = (*fresh46).rptr;
                free_arrayfor(r);
                current_block = 3518619798157913413;
            }
            69 => {
                r = ((*pc).d.fptr)
                    .expect("non-null function pointer")((*pc).x.xl as i32);
                let ref mut fresh47 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh47 = r;
                current_block = 3518619798157913413;
            }
            71 => {
                let mut arg_count: size_t = (*pc).x.xl as size_t;
                let mut f: *mut awk_ext_func_t = (*pc.offset(1 as i32 as isize)).x.exf;
                let mut min_req: size_t = (*f).min_required_args;
                let mut max_expect: size_t = (*f).max_expected_args;
                let mut result: awk_value_t = awk_value_t {
                    val_type: awk_valtype_t::AWK_UNDEFINED,
                    u: C2RustUnnamed_0 {
                        s: awk_string_t {
                            str_0: 0 as *mut i8,
                            len: 0,
                        },
                    },
                };
                if arg_count < min_req {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1101 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"%s: called with %lu arguments, expecting at least %lu\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*pc.offset(1 as i32 as isize)).d.name,
                        arg_count,
                        min_req,
                    );
                }
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    && (*f).suppress_lint as u64 == 0 && arg_count > max_expect
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1107 as i32,
                    );
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"%s: called with %lu arguments, expecting no more than %lu\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*pc.offset(1 as i32 as isize)).d.name,
                        arg_count,
                        max_expect,
                    );
                }
                PUSH_CODE(pc);
                let mut ef_ret: *mut awk_value_t = ((*pc).d.efptr)
                    .expect(
                        "non-null function pointer",
                    )(arg_count as i32, &mut result, f);
                r = awk_value_to_node(ef_ret);
                POP_CODE();
                loop {
                    let fresh48 = arg_count;
                    arg_count = arg_count.wrapping_sub(1);
                    if !(fresh48 > 0 as i32 as u64) {
                        break;
                    }
                    let fresh49 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    t1 = (*fresh49).rptr;
                    if (*t1).type_0 as u32 == nodevals::Node_val as i32 as u32
                        || (*t1).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
                    {
                        DEREF(t1);
                    }
                }
                free_api_string_copies();
                if in_indirect_call {
                    let fresh50 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    let mut fname: *mut NODE = (*fresh50).rptr;
                    DEREF(fname);
                    in_indirect_call = 0 as i32 != 0;
                }
                let ref mut fresh51 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh51 = r;
                current_block = 3518619798157913413;
            }
            70 => {
                r = do_sub((*pc).x.xl as i32, (*pc).d.dl as u32);
                let ref mut fresh52 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh52 = r;
                current_block = 3518619798157913413;
            }
            56 => {
                do_print((*pc).x.xl as i32, (*pc).d.dl as i32);
                current_block = 3518619798157913413;
            }
            58 => {
                do_printf((*pc).x.xl as i32, (*pc).d.dl as i32);
                current_block = 3518619798157913413;
            }
            57 => {
                do_print_rec((*pc).x.xl as i32, (*pc).d.dl as i32);
                current_block = 3518619798157913413;
            }
            79 => {
                m = (*pc).d.dn;
                if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                    r = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = r;
                } else if (*m).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh53 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh53 = m;
                current_block = 3518619798157913413;
            }
            49 => {
                m = (*pc).d.dn;
                t1 = *get_field(0 as i32 as i64, 0 as *mut Func_ptr);
                current_block = 16958959661683804599;
            }
            50 | 48 => {
                m = (*pc).d.dn;
                t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = t1;
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                }
                current_block = 16958959661683804599;
            }
            74 => {
                let mut f_0: *mut NODE = 0 as *mut NODE;
                let mut arg_count_0: i32 = 0;
                let mut save_1: i8 = 0;
                let mut function_name: *mut NODE = 0 as *mut NODE;
                arg_count_0 = (*pc.offset(1 as i32 as isize)).x.xl as i32;
                t1 = (*stack_ptr.offset(-(arg_count_0 as isize))).rptr;
                if (*t1).type_0 as u32 != nodevals::Node_val as i32 as u32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1209 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"indirect function call requires a simple scalar value\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
                str_terminate_f(t1, &mut save_1);
                if (*t1).sub.val.slen > 0 as i32 as u64 {
                    f_0 = (*pc).x.xn;
                    if !f_0.is_null()
                        && strcmp((*f_0).sub.nodep.name, (*t1).sub.val.sp) == 0 as i32
                    {
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        if post_execute.is_some() {
                            post_execute.expect("non-null function pointer")(pc);
                        }
                        pc = ni;
                        continue;
                    } else {
                        f_0 = lookup((*t1).sub.val.sp);
                    }
                }
                if f_0.is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1227 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`%s' is not a function, so it cannot be called indirectly\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*t1).sub.val.sp,
                    );
                    current_block = 8195495264928745804;
                } else if (*f_0).type_0 as u32
                    == nodevals::Node_builtin_func as i32 as u32
                {
                    let mut arg_count_1: i32 = (*pc.offset(1 as i32 as isize)).x.xl
                        as i32;
                    let mut the_func: builtin_func_t = lookup_builtin((*t1).sub.val.sp);
                    if the_func
                        == ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(i32, u32) -> *mut NODE>,
                            builtin_func_t,
                        >(Some(do_sub as unsafe extern "C" fn(i32, u32) -> *mut NODE))
                    {
                        r = call_sub((*t1).sub.val.sp, arg_count_1);
                    } else if the_func
                        == Some(do_match as unsafe extern "C" fn(i32) -> *mut NODE)
                    {
                        r = call_match(arg_count_1);
                    } else if the_func
                        == Some(do_split as unsafe extern "C" fn(i32) -> *mut NODE)
                        || the_func
                            == Some(
                                do_patsplit as unsafe extern "C" fn(i32) -> *mut NODE,
                            )
                    {
                        r = call_split_func((*t1).sub.val.sp, arg_count_1);
                    } else {
                        r = the_func.expect("non-null function pointer")(arg_count_1);
                    }
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    let fresh55 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    function_name = (*fresh55).rptr;
                    DEREF(function_name);
                    let ref mut fresh56 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh56 = r;
                    current_block = 3518619798157913413;
                } else if (*f_0).type_0 as u32 != nodevals::Node_func as i32 as u32 {
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    if (*f_0).type_0 as u32 == nodevals::Node_ext_func as i32 as u32 {
                        let mut bc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                        let mut fname_0: *mut i8 = (*pc).d.name;
                        let mut arg_count_2: i32 = (*pc.offset(1 as i32 as isize)).x.xl
                            as i32;
                        static mut npc: [INSTRUCTION; 2] = [INSTRUCTION {
                            nexti: 0 as *const exp_instruction as *mut exp_instruction,
                            d: C2RustUnnamed_7 {
                                dn: 0 as *const NODE as *mut NODE,
                            },
                            x: C2RustUnnamed_6 { xl: 0 },
                            comment: 0 as *const exp_instruction as *mut exp_instruction,
                            source_line: 0,
                            pool_size: 0,
                            opcode: opcodeval::Op_illegal,
                        }; 2];
                        npc[0 as i32 as usize] = *pc;
                        bc = (*f_0).sub.nodep.r.iptr;
                        npc[0 as i32 as usize].opcode = opcodeval::Op_ext_builtin;
                        npc[0 as i32 as usize].d.efptr = (*bc).d.efptr;
                        npc[0 as i32 as usize].x.xl = arg_count_2 as i64;
                        npc[1 as i32 as usize] = *pc.offset(1 as i32 as isize);
                        npc[1 as i32 as usize].d.name = fname_0;
                        npc[1 as i32 as usize].x.exf = (*bc).x.exf;
                        in_indirect_call = 1 as i32 != 0;
                        ni = npc.as_mut_ptr();
                        if post_execute.is_some() {
                            post_execute.expect("non-null function pointer")(pc);
                        }
                        pc = ni;
                        continue;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            1277 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"function called indirectly through `%s' does not exist\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*pc).d.name,
                        );
                    }
                    current_block = 8195495264928745804;
                } else {
                    current_block = 8195495264928745804;
                }
                match current_block {
                    3518619798157913413 => {}
                    _ => {
                        (*pc).x.xn = f_0;
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        if post_execute.is_some() {
                            post_execute.expect("non-null function pointer")(pc);
                        }
                        pc = ni;
                        continue;
                    }
                }
            }
            73 => {
                let mut f_1: *mut NODE = 0 as *mut NODE;
                f_1 = (*pc).x.xn;
                if f_1.is_null() {
                    f_1 = lookup((*pc).d.name);
                    if f_1.is_null()
                        || (*f_1).type_0 as u32 != nodevals::Node_func as i32 as u32
                            && (*f_1).type_0 as u32
                                != nodevals::Node_ext_func as i32 as u32
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            1296 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"function `%s' not defined\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*pc).d.name,
                        );
                    }
                    (*pc).x.xn = f_1;
                }
                if (*f_1).type_0 as u32 == nodevals::Node_ext_func as i32 as u32 {
                    let mut bc_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut fname_1: *mut i8 = (*pc).d.name;
                    let mut arg_count_3: i32 = (*pc.offset(1 as i32 as isize)).x.xl
                        as i32;
                    bc_0 = (*f_1).sub.nodep.r.iptr;
                    (*pc).opcode = opcodeval::Op_ext_builtin;
                    (*pc).d.efptr = (*bc_0).d.efptr;
                    (*pc).x.xl = arg_count_3 as i64;
                    let ref mut fresh57 = (*pc.offset(1 as i32 as isize)).d.name;
                    *fresh57 = fname_1;
                    let ref mut fresh58 = (*pc.offset(1 as i32 as isize)).x.exf;
                    *fresh58 = (*bc_0).x.exf;
                    ni = pc;
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = ni;
                    continue;
                } else {
                    ni = setup_frame(pc);
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = ni;
                    continue;
                }
            }
            62 => {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected opcode %s\0"
                        as *const u8 as *const i8,
                    b"./interpret.h\0" as *const u8 as *const i8,
                    1322 as i32,
                    opcode2str(op),
                );
                current_block = 3518619798157913413;
            }
            61 => {
                m = POP_SCALAR();
                ni = unwind_stack((*frame_ptr).sub.nodep.reflags as i64);
                let ref mut fresh59 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh59 = m;
                if post_execute.is_some() {
                    post_execute.expect("non-null function pointer")(pc);
                }
                pc = ni;
                continue;
            }
            65 => {
                r = do_getline_redir((*pc).x.xl as i32, (*pc).d.dl as redirval);
                let ref mut fresh60 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh60 = r;
                current_block = 3518619798157913413;
            }
            66 => {
                if currule == 0 || currule == defrule::BEGINFILE as i32
                    || currule == defrule::ENDFILE as i32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1342 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"non-redirected `getline' invalid inside `%s' rule\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                loop {
                    let mut ret: i32 = 0;
                    ret = nextfile(&mut curfile, 0 as i32 != 0);
                    if ret <= 0 as i32 {
                        r = do_getline((*pc).x.xl as i32, curfile);
                        if !r.is_null() {
                            break;
                        }
                    } else {
                        push_exec_state(pc, currule, source, stack_ptr);
                        if curfile.is_null() {
                            if post_execute.is_some() {
                                post_execute.expect("non-null function pointer")(pc);
                            }
                            pc = (*pc.offset(1 as i32 as isize)).x.xi;
                            continue '_top;
                        } else {
                            if post_execute.is_some() {
                                post_execute.expect("non-null function pointer")(pc);
                            }
                            pc = (*pc.offset(1 as i32 as isize)).d.di;
                            continue '_top;
                        }
                    }
                }
                let ref mut fresh61 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh61 = r;
                current_block = 3518619798157913413;
            }
            100 => {
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut i64);
                if post_execute.is_some() {
                    post_execute.expect("non-null function pointer")(pc);
                }
                pc = ni;
                continue;
            }
            99 => {
                after_beginfile(&mut curfile);
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut i64);
                if (*ni).opcode as u32 == opcodeval::Op_K_getline as i32 as u32
                    || curfile.is_null()
                {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = ni;
                    continue;
                }
                current_block = 3518619798157913413;
            }
            91 => {
                let mut ret_0: i32 = 0;
                ret_0 = nextfile(&mut curfile, 0 as i32 != 0);
                if ret_0 < 0 as i32 {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else if ret_0 == 0 as i32 {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc.offset(1 as i32 as isize)).x.xi;
                    continue;
                } else {
                    push_exec_state(pc, currule, source, stack_ptr);
                    if curfile.is_null() {
                        if post_execute.is_some() {
                            post_execute.expect("non-null function pointer")(pc);
                        }
                        pc = (*pc).x.xi;
                        continue;
                    }
                }
                current_block = 3518619798157913413;
            }
            90 => {
                let mut errcode: i32 = 0 as i32;
                ni = (*pc).d.di;
                if curfile.is_null() {
                    ni = (*ni).d.di;
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = ni;
                    continue;
                } else if !inrec(curfile, &mut errcode) {
                    if errcode > 0 as i32 {
                        update_ERRNO_int(errcode);
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            != 0 || (*pc).x.xl == 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                1433 as i32,
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
                                    b"error reading input file `%s': %s\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (*curfile).public.name,
                                strerror(errcode),
                            );
                        }
                    }
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = ni;
                    continue;
                }
                current_block = 3518619798157913413;
            }
            67 => {
                let mut ret_1: i32 = 0;
                if currule != defrule::Rule as i32
                    && currule != defrule::BEGINFILE as i32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1448 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`nextfile' cannot be called from a `%s' rule\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                ret_1 = nextfile(&mut curfile, 1 as i32 != 0);
                if currule == defrule::BEGINFILE as i32 {
                    let mut stack_size: i64 = 0 as i32 as i64;
                    ni = pop_exec_state(&mut currule, &mut source, &mut stack_size);
                    unwind_stack(stack_size);
                    if ret_1 == 0 as i32 {
                        if post_execute.is_some() {
                            post_execute.expect("non-null function pointer")(pc);
                        }
                        pc = ni;
                        continue;
                    } else {
                        push_exec_state(ni, currule, source, stack_ptr);
                        if post_execute.is_some() {
                            post_execute.expect("non-null function pointer")(pc);
                        }
                        pc = (*pc).x.xi;
                        continue;
                    }
                } else {
                    unwind_stack(0 as i32 as i64);
                    push_exec_state((*pc).d.di, currule, source, stack_ptr);
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).x.xi;
                    continue;
                }
            }
            60 => {
                if currule == 0 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1495 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`exit' cannot be called in the current context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                exiting = 1 as i32 != 0;
                t1 = force_number(POP_SCALAR());
                if t1 != Nnull_string {
                    exit_val = (*t1).sub.val.fltnum as i64 as i32;
                }
                DEREF(t1);
                if currule == defrule::BEGINFILE as i32
                    || currule == defrule::ENDFILE as i32
                {
                    pop_exec_state(&mut currule, &mut source, 0 as *mut i64);
                }
                unwind_stack(0 as i32 as i64);
                if currule == defrule::END as i32 {
                    ni = (*pc).x.xi;
                } else {
                    ni = (*pc).d.di;
                }
                if post_execute.is_some() {
                    post_execute.expect("non-null function pointer")(pc);
                }
                pc = ni;
                continue;
            }
            59 => {
                if currule != defrule::Rule as i32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1536 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`next' cannot be called from a `%s' rule\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                unwind_stack(0 as i32 as i64);
                if post_execute.is_some() {
                    post_execute.expect("non-null function pointer")(pc);
                }
                pc = (*pc).d.di;
                continue;
            }
            86 => {
                r = POP_SCALAR();
                DEREF(r);
                current_block = 3518619798157913413;
            }
            14 => {
                if (*pc).x.xl != 0 {
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            15 => {
                let mut result_0: i32 = 0;
                let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                t1 = TOP_SCALAR();
                di = (eval_condition(t1) as i32 != 0 as i32) as i32;
                DEREF(t1);
                ip = (*pc).x.xi;
                if (*ip).x.xl == 0 && di != 0 {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    (*ip).x.xl = 1 as i32 as i64;
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*ip).d.di;
                    continue;
                } else {
                    result_0 = ((*ip).x.xl != 0 || di != 0) as i32;
                    (*ip).x.xl ^= di as i64;
                    r = node_Boolean[result_0 as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    (*stack_ptr).rptr = r;
                    if post_execute.is_some() {
                        post_execute.expect("non-null function pointer")(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                }
            }
            103 => {
                if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
                    (*pc).d.ldl = ((*pc).d.ldl).wrapping_add(1);
                    (*pc).d.ldl;
                }
                current_block = 3518619798157913413;
            }
            85 | 112 | 115 | 113 | 114 | 116 | 53 | 117 | 118 | 120 | 102 | 121 => {
                current_block = 3518619798157913413;
            }
            _ => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"./interpret.h\0" as *const u8 as *const i8, 1599 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"Sorry, don't know how to interpret `%s'\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    opcode2str(op),
                );
                current_block = 3518619798157913413;
            }
        }
        match current_block {
            16958959661683804599 => {
                rp = re_update(m);
                di = research(
                    rp,
                    (*t1).sub.val.sp,
                    0 as i32,
                    (*t1).sub.val.slen,
                    0 as i32,
                );
                di = (di == -(1 as i32)) as i32
                    ^ (op as u32 != opcodeval::Op_nomatch as i32 as u32) as i32;
                if op as u32 != opcodeval::Op_match_rec as i32 as u32 {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    DEREF(t1);
                    if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                        DEREF((*m).sub.nodep.x.extra);
                        (*m).sub.nodep.x.extra = 0 as *mut exp_node;
                    }
                }
                r = node_Boolean[di as usize];
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh54 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh54 = r;
            }
            11956501910097409974 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as i32 as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        664 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"division by zero attempted in `%%'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                x = fmod((*t1).sub.val.fltnum, x2);
                r = make_number.expect("non-null function pointer")(x);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            7294098017659915583 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as i32 as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        648 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"division by zero attempted\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum / x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            17966935000565432821 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect(
                        "non-null function pointer",
                    )(calc_exp((*t1).sub.val.fltnum, x2));
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            16240336253164271420 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum * x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            5088478517898830498 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum - x2);
                (*r).sub.val.fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            2235334700535380937 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum + x2);
                (*r).sub.val.fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            16275318387282786645 => {
                source = (*pc).d.name;
            }
            1528899951983446718 => {
                let ref mut fresh31 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh31 = (*pc).d.dn;
            }
            _ => {}
        }
        if post_execute.is_some() {
            post_execute.expect("non-null function pointer")(pc);
        }
        pc = (*pc).nexti;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn r_interpret(mut code: *mut INSTRUCTION) -> i32 {
    let mut current_block: u64;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut op: OPCODE = opcodeval::Op_illegal;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut ni: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut x: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut di: i32 = 0;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut set_array: *mut NODE = 0 as *mut NODE;
    let mut set_idx: *mut NODE = 0 as *mut NODE;
    let mut in_indirect_call: bool = 0 as i32 != 0;
    pc = code;
    '_top: loop {
        if (*pc).source_line as i32 > 0 as i32 {
            sourceline = (*pc).source_line as i32;
        }
        op = (*pc).opcode;
        if do_itrace {
            fprintf(stderr, b"+ %s\n\0" as *const u8 as *const i8, opcode2str(op));
            fflush(stderr);
        }
        match op as u32 {
            51 => {
                currule = (*pc).x.xl as i32;
                if currule == defrule::BEGINFILE as i32 {
                    set_record(
                        b"\0" as *const u8 as *const i8,
                        0 as i32 as size_t,
                        0 as *const awk_fieldwidth_info_t,
                    );
                }
                current_block = 8953125900534742068;
            }
            101 => {
                current_block = 8953125900534742068;
            }
            107 => {
                let mut stdio_problem: bool = 0 as i32 != 0;
                let mut got_EPIPE: bool = 0 as i32 != 0;
                source = 0 as *mut i8;
                sourceline = 0 as i32;
                nextfile(&mut curfile, 1 as i32 != 0);
                close_io(&mut stdio_problem, &mut got_EPIPE);
                if stdio_problem as i32 != 0 && !exiting && exit_val == 0 as i32 {
                    exit_val = 1 as i32;
                }
                close_extensions();
                if got_EPIPE {
                    signal(13 as i32, None);
                    kill(getpid(), 13 as i32);
                }
                current_block = 2242099707034464334;
            }
            108 => {
                break;
            }
            78 => {
                m = (*pc).d.dn;
                if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                    && (*m).flags as u32 & flagvals::INTLSTR as i32 as u32
                        != 0 as i32 as u32
                {
                    let mut orig: *mut i8 = 0 as *mut i8;
                    let mut trans: *mut i8 = 0 as *mut i8;
                    let mut save: i8 = 0;
                    save = *((*m).sub.val.sp).offset((*m).sub.val.slen as isize);
                    *((*m).sub.val.sp).offset((*m).sub.val.slen as isize) = '\0' as i32
                        as i8;
                    orig = (*m).sub.val.sp;
                    trans = dcgettext(TEXTDOMAIN, orig, 5 as i32);
                    *((*m).sub.val.sp).offset((*m).sub.val.slen as isize) = save;
                    if trans != orig {
                        m = make_str_node(trans, strlen(trans), 0 as i32);
                    } else {
                        (*m).valref += 1;
                        (*m).valref;
                    }
                } else {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh62 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh62 = m;
                current_block = 2242099707034464334;
            }
            75 | 76 | 77 => {
                let mut current_block_66: u64;
                let mut save_symbol: *mut NODE = 0 as *mut NODE;
                let mut isparam: bool = 0 as i32 != 0;
                m = (*pc).d.dn;
                save_symbol = m;
                if (*m).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
                    isparam = 1 as i32 != 0;
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                    save_symbol = m;
                    if (*m).type_0 as u32 == nodevals::Node_array_ref as i32 as u32 {
                        if (*(*m).sub.nodep.l.lptr).type_0 as u32
                            == nodevals::Node_var as i32 as u32
                        {
                            current_block_66 = 12941253007842490363;
                        } else {
                            m = (*m).sub.nodep.l.lptr;
                            current_block_66 = 2543120759711851213;
                        }
                    } else {
                        current_block_66 = 2543120759711851213;
                    }
                } else {
                    current_block_66 = 2543120759711851213;
                }
                match current_block_66 {
                    2543120759711851213 => {
                        match (*m).type_0 as u32 {
                            4 => {
                                current_block_66 = 13362829586527803627;
                                match current_block_66 {
                                    8173679788479780328 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    13362829586527803627 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    18313894875582446089 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                            6 => {
                                current_block_66 = 12941253007842490363;
                            }
                            7 => {
                                current_block_66 = 7185288078922200361;
                                match current_block_66 {
                                    8173679788479780328 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    13362829586527803627 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    18313894875582446089 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                            5 => {
                                current_block_66 = 18313894875582446089;
                                match current_block_66 {
                                    8173679788479780328 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    13362829586527803627 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    18313894875582446089 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                            _ => {
                                current_block_66 = 8173679788479780328;
                                match current_block_66 {
                                    8173679788479780328 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const i8,
                                            b"./interpret.h\0" as *const u8 as *const i8,
                                            263 as i32,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    13362829586527803627 => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                204 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    18313894875582446089 => {
                                        if op as u32 == opcodeval::Op_push_arg as i32 as u32
                                            || op as u32 == opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                258 as i32,
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
                                                    b"attempt to use array `%s' in a scalar context\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                if isparam as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32
                                        {
                                            (*m).type_0 = nodevals::Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block_66 {
                    12941253007842490363 => {
                        if do_flags as u32
                            & (do_flag_values::DO_LINT_INVALID as i32
                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                216 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                if isparam as i32 != 0 {
                                    dcgettext(
                                        0 as *const i8,
                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    )
                                } else {
                                    dcgettext(
                                        0 as *const i8,
                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    )
                                },
                                (*save_symbol).sub.nodep.name,
                            );
                        }
                        if op as u32 != opcodeval::Op_push_arg_untyped as i32 as u32 {
                            (*m).type_0 = nodevals::Node_var;
                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                            m = dupnode(Nnull_string);
                        }
                        (*m).valref += 1;
                        (*m).valref;
                        let ref mut fresh64 = (*if stack_ptr < stack_top {
                            stack_ptr = stack_ptr.offset(1);
                            stack_ptr
                        } else {
                            grow_stack()
                        })
                            .rptr;
                        *fresh64 = m;
                    }
                    _ => {}
                }
                current_block = 2242099707034464334;
            }
            81 => {
                m = (*pc).d.dn;
                if (*m).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                }
                if (*m).type_0 as u32 == nodevals::Node_var as i32 as u32 {
                    m = (*m).sub.nodep.l.lptr;
                    (*m).valref += 1;
                    (*m).valref;
                    let ref mut fresh67 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh67 = m;
                    current_block = 2242099707034464334;
                } else {
                    current_block = 11503388136377843356;
                }
            }
            80 => {
                current_block = 11503388136377843356;
            }
            82 => {
                lhs = if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, (*pc).x.xl != 0)
                };
                let ref mut fresh69 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh69 = lhs;
                current_block = 2242099707034464334;
            }
            16 => {
                t2 = if (*pc).d.dl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as i32, 1 as i32 != 0)
                };
                t1 = POP_ARRAY(0 as i32 != 0);
                if (in_array(t1, t2)).is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            296 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            299 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"SYMTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            302 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as i32 as u64 {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                305 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if t1 == func_table {
                    static mut warned: bool = 0 as i32 != 0;
                    if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32
                        != 0 && !warned
                    {
                        warned = 1 as i32 != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            317 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"FUNCTAB is a gawk extension\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    r = t2;
                } else {
                    if t1 == symbol_table {
                        update_global_values();
                    }
                    r = *((*(*t1).sub.nodep.l.lp).lookup)
                        .expect("non-null function pointer")(t1, t2);
                }
                DEREF(t2);
                if t1 == symbol_table {
                    static mut warned_0: bool = 0 as i32 != 0;
                    if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32
                        != 0 && !warned_0
                    {
                        warned_0 = 1 as i32 != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            335 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"SYMTAB is a gawk extension\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    if (*r).type_0 as u32 == nodevals::Node_var as i32 as u32 {
                        r = (*r).sub.nodep.l.lptr;
                    } else if (*r).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    {
                        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                        r = (*r).sub.nodep.l.lptr;
                    }
                }
                if (*r).type_0 as u32 == nodevals::Node_val as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_var as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
                {
                    (*r).valref += 1;
                    (*r).valref;
                }
                let ref mut fresh70 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh70 = r;
                current_block = 2242099707034464334;
            }
            17 => {
                t2 = if (*pc).d.dl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as i32, 1 as i32 != 0)
                };
                t1 = POP_ARRAY(0 as i32 != 0);
                r = in_array(t1, t2);
                if r.is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            362 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            365 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            b"SYMTAB\0" as *const u8 as *const i8,
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    } else if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            368 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as i32 as u64 {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                371 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if r.is_null() {
                    r = make_array();
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                    assoc_set(t1, t2, r);
                } else if (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
                    r = force_array(r, 0 as i32 != 0);
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                } else if (*r).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        388 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use scalar `%s[\"%.*s\"]' as an array\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as i32,
                        (*t2).sub.val.sp,
                    );
                } else {
                    DEREF(t2);
                }
                let ref mut fresh71 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh71 = r;
                current_block = 2242099707034464334;
            }
            83 => {
                t2 = if (*pc).d.dl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as i32, 1 as i32 != 0)
                };
                t1 = POP_ARRAY(0 as i32 != 0);
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    && (in_array(t1, t2)).is_null()
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if (*pc).x.xl != 0 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            402 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as i32,
                            (*t2).sub.val.sp,
                        );
                    }
                    if (*t2).sub.val.slen == 0 as i32 as u64 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            405 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"subscript of array `%s' is null string\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            array_vname(t1),
                        );
                    }
                }
                lhs = ((*(*t1).sub.nodep.l.lp).lookup)
                    .expect("non-null function pointer")(t1, t2);
                if (**lhs).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        411 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as i32,
                        (*t2).sub.val.sp,
                    );
                }
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        429 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as u32 == nodevals::Node_var as i32 as u32
                        || (**lhs).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    {
                        update_global_values();
                        (**lhs).type_0 = nodevals::Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            437 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                }
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    set_array = t1;
                    set_idx = t2;
                } else {
                    DEREF(t2);
                }
                let ref mut fresh72 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh72 = lhs;
                current_block = 2242099707034464334;
            }
            24 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, 0 as *mut Func_ptr, 1 as i32 != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                r = *lhs;
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh73 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh73 = r;
                current_block = 2242099707034464334;
            }
            84 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut (*(*pc).d.di).x.aptr, (*pc).x.xl != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                let ref mut fresh74 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh74 = lhs;
                current_block = 2242099707034464334;
            }
            105 => {
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    match (*pc).d.dl {
                        1 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                474 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"assignment used in conditional context\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: unexpected lint type value %d\0"
                                    as *const u8 as *const i8,
                                b"./interpret.h\0" as *const u8 as *const i8,
                                478 as i32,
                                (*pc).d.dl as i32,
                            );
                        }
                    }
                }
                current_block = 2242099707034464334;
            }
            106 => {
                t1 = (*stack_ptr).rptr;
                t2 = (*stack_ptr.offset(-(1 as i32 as isize))).rptr;
                if (*t1).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
                    && (*t2).flags as u32 & flagvals::STRING as i32 as u32
                        != 0 as i32 as u32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        489 as i32,
                    );
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"operator `+' used on two string values\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                current_block = 2242099707034464334;
            }
            54 | 55 | 87 => {
                pc = (*pc).d.di;
                continue;
            }
            89 => {
                r = POP_SCALAR();
                di = eval_condition(r) as i32;
                DEREF(r);
                if di == 0 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            88 => {
                r = POP_SCALAR();
                di = eval_condition(r) as i32;
                DEREF(r);
                if di != 0 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            38 | 40 => {
                t1 = POP_SCALAR();
                di = eval_condition(t1) as i32;
                DEREF(t1);
                if op as u32 == opcodeval::Op_and as i32 as u32 && di != 0
                    || op as u32 == opcodeval::Op_or as i32 as u32 && di == 0
                {
                    current_block = 2242099707034464334;
                } else {
                    r = node_Boolean[di as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    let ref mut fresh75 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh75 = r;
                    ni = (*pc).d.di;
                    pc = (*ni).nexti;
                    continue;
                }
            }
            39 | 41 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[eval_condition(t1) as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            25 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[!eval_condition(t1) as i32 as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            42 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_EQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            43 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_NEQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            44 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_LT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            45 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_GT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            46 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_LE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            47 => {
                r = node_Boolean[cmp_scalars(scalar_cmp_t::SCALAR_GE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            8 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 9390321967027614628;
            }
            7 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 9390321967027614628;
            }
            10 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 8008614458410856352;
            }
            9 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 8008614458410856352;
            }
            2 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 6406384398533964377;
            }
            1 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 6406384398533964377;
            }
            12 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 16935727273394377761;
            }
            11 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 16935727273394377761;
            }
            4 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 5149334253309972495;
            }
            3 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 5149334253309972495;
            }
            6 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 11021467851294229602;
            }
            5 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 11021467851294229602;
            }
            18 | 19 => {
                x = if op as u32 == opcodeval::Op_preincrement as i32 as u32 {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                if (*t1).valref == 1 as i32 as i64
                    && (*t1).flags as u32
                        == (flagvals::MALLOC as i32 | flagvals::NUMCUR as i32
                            | flagvals::NUMBER as i32) as u32
                {
                    (*t1).sub.val.fltnum += x;
                    r = t1;
                } else {
                    *lhs = make_number
                        .expect("non-null function pointer")((*t1).sub.val.fltnum + x);
                    r = *lhs;
                    unref(t1);
                }
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            20 | 21 => {
                x = if op as u32 == opcodeval::Op_postincrement as i32 as u32 {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum);
                if (*t1).valref == 1 as i32 as i64
                    && (*t1).flags as u32
                        == (flagvals::MALLOC as i32 | flagvals::NUMCUR as i32
                            | flagvals::NUMBER as i32) as u32
                {
                    (*t1).sub.val.fltnum += x;
                } else {
                    *lhs = make_number
                        .expect("non-null function pointer")((*t1).sub.val.fltnum + x);
                    unref(t1);
                }
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            22 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")(-(*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            23 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            28 => {
                t1 = force_array((*pc).d.dn, 1 as i32 != 0);
                t2 = if (*pc).x.xl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as i32, 1 as i32 != 0)
                };
                lhs = ((*(*t1).sub.nodep.l.lp).lookup)
                    .expect("non-null function pointer")(t1, t2);
                if (**lhs).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        737 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as i32,
                        (*t2).sub.val.sp,
                    );
                }
                DEREF(t2);
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        755 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as u32 == nodevals::Node_var as i32 as u32
                        || (**lhs).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    {
                        update_global_values();
                        (**lhs).type_0 = nodevals::Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            763 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                }
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    (Some(
                        ((*(*t1).sub.nodep.l.lp).store)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(t1, t2);
                }
                DEREF(t2);
                current_block = 2242099707034464334;
            }
            27 => {
                lhs = if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as i32 != 0)
                };
                unref(*lhs);
                r = (*pc).x.xn;
                if !r.is_null() {
                    (*r).valref += 1;
                    (*r).valref;
                    *lhs = r;
                } else {
                    r = POP_SCALAR();
                    unfield(lhs, &mut r);
                }
                current_block = 2242099707034464334;
            }
            29 | 30 => {
                let mut assign: Func_ptr = None;
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut assign, 0 as i32 != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                assign.expect("non-null function pointer")();
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                if op as u32 == opcodeval::Op_store_field_exp as i32 as u32 {
                    (**lhs).valref += 1;
                    (**lhs).valref;
                    let ref mut fresh76 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh76 = *lhs;
                }
                current_block = 2242099707034464334;
            }
            37 => {
                lhs = if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as i32 != 0)
                };
                t1 = force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                if t1 != *lhs {
                    unref(*lhs);
                    if (*t1).valref == 1 as i32 as i64 {
                        *lhs = t1;
                    } else {
                        *lhs = dupnode(t1);
                    }
                }
                if t1 != t2 && (*t1).valref == 1 as i32 as i64
                    && (*t1).flags as u32
                        & (flagvals::MALLOC as i32 | flagvals::MPFN as i32
                            | flagvals::MPZN as i32) as u32
                        == flagvals::MALLOC as i32 as u32
                {
                    let mut nlen: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    (*t1).sub.val.sp = erealloc_real(
                        (*t1).sub.val.sp as *mut libc::c_void,
                        nlen.wrapping_add(1 as i32 as u64),
                        b"r_interpret\0" as *const u8 as *const i8,
                        b"t1->stptr\0" as *const u8 as *const i8,
                        b"./interpret.h\0" as *const u8 as *const i8,
                        843 as i32,
                    ) as *mut i8;
                    memcpy(
                        ((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize)
                            as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    (*t1).sub.val.slen = nlen;
                    *((*t1).sub.val.sp).offset(nlen as isize) = '\0' as i32 as i8;
                    (*t1).flags = ::core::mem::transmute::<
                        u32,
                        flagvals,
                    >((*t1).flags as u32 & flagvals::WSTRCUR as i32 as u32);
                    (*t1).flags = ::core::mem::transmute::<
                        u32,
                        flagvals,
                    >(
                        (*t1).flags as u32
                            | (flagvals::MALLOC as i32 | flagvals::STRING as i32
                                | flagvals::STRCUR as i32) as u32,
                    );
                    (*t1).sub.val.idx = -(1 as i32);
                    if (*t1).flags as u32 & flagvals::WSTRCUR as i32 as u32
                        != 0 as i32 as u32
                        && (*t2).flags as u32 & flagvals::WSTRCUR as i32 as u32
                            != 0 as i32 as u32
                    {
                        let mut wlen: size_t = ((*t1).sub.val.wslen)
                            .wrapping_add((*t2).sub.val.wslen);
                        (*t1).sub.val.wsp = erealloc_real(
                            (*t1).sub.val.wsp as *mut libc::c_void,
                            (::core::mem::size_of::<wchar_t>() as u64)
                                .wrapping_mul(wlen.wrapping_add(1 as i32 as u64)),
                            b"r_interpret\0" as *const u8 as *const i8,
                            b"t1->wstptr\0" as *const u8 as *const i8,
                            b"./interpret.h\0" as *const u8 as *const i8,
                            860 as i32,
                        ) as *mut wchar_t;
                        memcpy(
                            ((*t1).sub.val.wsp).offset((*t1).sub.val.wslen as isize)
                                as *mut libc::c_void,
                            (*t2).sub.val.wsp as *const libc::c_void,
                            ((*t2).sub.val.wslen)
                                .wrapping_mul(::core::mem::size_of::<wchar_t>() as u64),
                        );
                        (*t1).sub.val.wslen = wlen;
                        *((*t1).sub.val.wsp).offset(wlen as isize) = '\0' as i32;
                    } else if (**lhs).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0
                    {
                        r_free_wstr(*lhs);
                    }
                } else {
                    let mut nlen_0: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    let mut p: *mut i8 = 0 as *mut i8;
                    p = emalloc_real(
                        nlen_0.wrapping_add(1 as i32 as u64),
                        b"r_interpret\0" as *const u8 as *const i8,
                        b"p\0" as *const u8 as *const i8,
                        b"./interpret.h\0" as *const u8 as *const i8,
                        870 as i32,
                    ) as *mut i8;
                    memcpy(
                        p as *mut libc::c_void,
                        (*t1).sub.val.sp as *const libc::c_void,
                        (*t1).sub.val.slen,
                    );
                    memcpy(
                        p.offset((*t1).sub.val.slen as isize) as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    unref(*lhs);
                    *lhs = make_str_node(p, nlen_0, 2 as i32);
                    t1 = *lhs;
                }
                DEREF(t2);
                current_block = 2242099707034464334;
            }
            26 => {
                let fresh77 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh77).lptr;
                r = TOP_SCALAR();
                unref(*lhs);
                if (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
                    DEREF(r);
                    r = dupnode(Nnull_string);
                }
                (*r).valref += 1;
                (*r).valref;
                unfield(lhs, &mut r);
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            98 => {
                if !set_idx.is_null() {
                    di = 1 as i32;
                    if (*pc).d.dl == opcodeval::Op_sub_builtin as i32 as i64
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        } && (*r).sub.val.fltnum as i64 == 0 as i32 as i64
                    {
                        di = 0 as i32;
                    } else if ((*pc).d.dl == opcodeval::Op_K_getline as i32 as i64
                        || (*pc).d.dl == opcodeval::Op_K_getline_redir as i32 as i64)
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        } && (*r).sub.val.fltnum as i64 <= 0 as i32 as i64
                    {
                        di = 0 as i32;
                    }
                    if di != 0 {
                        (Some(
                            ((*(*set_array).sub.nodep.l.lp).store)
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(set_array, set_idx);
                    }
                    unref(set_idx);
                    set_idx = 0 as *mut NODE;
                }
                current_block = 2242099707034464334;
            }
            34 | 35 | 31 | 32 | 33 | 36 => {
                op_assign(op);
                current_block = 2242099707034464334;
            }
            95 => {
                ((*pc).x.aptr).expect("non-null function pointer")();
                current_block = 2242099707034464334;
            }
            96 | 97 => {
                r = (*stack_ptr).rptr;
                if (*pc).d.dl == opcodeval::Op_sub_builtin as i32 as i64
                    && (*r).sub.val.fltnum as i64 == 0 as i32 as i64
                {
                    current_block = 2242099707034464334;
                } else if ((*pc).d.dl == opcodeval::Op_K_getline as i32 as i64
                    || (*pc).d.dl == opcodeval::Op_K_getline_redir as i32 as i64)
                    && (*r).sub.val.fltnum as i64 <= 0 as i32 as i64
                {
                    current_block = 2242099707034464334;
                } else {
                    if op as u32 == opcodeval::Op_var_assign as i32 as u32 {
                        ((*pc).x.aptr).expect("non-null function pointer")();
                    } else {
                        ((*pc).x.aptr).expect("non-null function pointer")();
                    }
                    current_block = 2242099707034464334;
                }
            }
            13 => {
                r = concat_exp((*pc).x.xl as i32, (*pc).d.dl & 1 as i32 as i64 != 0);
                let ref mut fresh78 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh78 = r;
                current_block = 2242099707034464334;
            }
            52 => {
                if (*pc.offset(1 as i32 as isize)).x.xl != 0 {
                    let fresh79 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    m = (*fresh79).rptr;
                    t2 = TOP_SCALAR();
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    rp = re_update(m);
                    di = (research(
                        rp,
                        (*t2).sub.val.sp,
                        0 as i32,
                        (*t2).sub.val.slen,
                        0 as i32,
                    ) >= 0 as i32) as i32;
                } else {
                    t1 = POP_SCALAR();
                    t2 = TOP_SCALAR();
                    di = (cmp_nodes(t2, t1, 1 as i32 != 0) == 0 as i32) as i32;
                    DEREF(t1);
                }
                if di != 0 {
                    t2 = POP_SCALAR();
                    DEREF(t2);
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            63 => {
                t1 = POP_ARRAY(0 as i32 != 0);
                do_delete(t1, (*pc).x.xl as i32);
                stack_ptr = stack_ptr.offset(-(*pc).x.xl as isize);
                current_block = 2242099707034464334;
            }
            64 => {
                t1 = POP_ARRAY(0 as i32 != 0);
                let fresh80 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh80).lptr;
                do_delete_loop(t1, lhs);
                current_block = 2242099707034464334;
            }
            72 => {
                t1 = POP_ARRAY(0 as i32 != 0);
                t2 = if (*pc).x.xl == 1 as i32 as i64 {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as i32, 1 as i32 != 0)
                };
                r = node_Boolean[(in_array(t1, t2)
                    != 0 as *mut libc::c_void as *mut NODE) as i32 as usize];
                DEREF(t2);
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh81 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh81 = r;
                current_block = 2242099707034464334;
            }
            92 => {
                let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
                let mut array: *mut NODE = 0 as *mut NODE;
                let mut sort_str: *mut NODE = 0 as *mut NODE;
                let mut num_elems: size_t = 0 as i32 as size_t;
                static mut sorted_in: *mut NODE = 0 as *const NODE as *mut NODE;
                let mut how_to_sort: *const i8 = b"@unsorted\0" as *const u8
                    as *const i8;
                let mut save_0: i8 = 0;
                let mut saved_end: bool = 0 as i32 != 0;
                array = POP_ARRAY(1 as i32 != 0);
                num_elems = (*array).sub.nodep.reflags as size_t;
                if !(num_elems == 0 as i32 as u64) {
                    if sorted_in.is_null() {
                        sorted_in = make_str_node(
                            b"sorted_in\0" as *const u8 as *const i8,
                            9 as i32 as size_t,
                            0 as i32,
                        );
                    }
                    sort_str = 0 as *mut NODE;
                    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
                        && !PROCINFO_node.is_null()
                    {
                        sort_str = in_array(PROCINFO_node, sorted_in);
                    }
                    if !sort_str.is_null() {
                        sort_str = force_string_fmt(sort_str, CONVFMT, CONVFMTidx);
                        if (*sort_str).sub.val.slen > 0 as i32 as u64 {
                            how_to_sort = (*sort_str).sub.val.sp;
                            str_terminate_f(sort_str, &mut save_0);
                            saved_end = 1 as i32 != 0;
                        }
                    }
                    list = assoc_list(array, how_to_sort, sort_context_t::SORTED_IN);
                    if saved_end {
                        *((*sort_str).sub.val.sp)
                            .offset((*sort_str).sub.val.slen as isize) = save_0;
                    }
                }
                r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
                if !r.is_null() {
                    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r
                        as *mut block_item))
                        .freep;
                } else {
                    r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
                };
                (*r).type_0 = nodevals::Node_arrayfor;
                (*r).sub.nodep.r.av = list;
                (*r).sub.nodep.reflags = reflagvals::from_libc_c_uint(num_elems as u32);
                (*r).sub.nodep.l.ll = -(1 as i32) as i64;
                (*r).sub.nodep.rn = array;
                let ref mut fresh82 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh82 = r;
                if num_elems == 0 as i32 as u64 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            93 => {
                r = (*stack_ptr).rptr;
                (*r).sub.nodep.l.ll += 1;
                if (*r).sub.nodep.l.ll == (*r).sub.nodep.reflags as i64 {
                    let mut array_0: *mut NODE = 0 as *mut NODE;
                    array_0 = (*r).sub.nodep.rn;
                    if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        && (*array_0).sub.nodep.reflags as u32
                            != (*r).sub.nodep.reflags as u32
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            1070 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"for loop: array `%s' changed size from %ld to %ld during loop execution\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            array_vname(array_0),
                            (*r).sub.nodep.reflags as i64,
                            (*array_0).sub.nodep.reflags as i64,
                        );
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    t1 = *((*r).sub.nodep.r.av).offset((*r).sub.nodep.l.ll as isize);
                    lhs = if (*(*pc).x.xn).type_0 as u32
                        == nodevals::Node_var as i32 as u32
                        && !((*(*pc).x.xn).sub.nodep.l.lptr == Nnull_string)
                    {
                        &mut (*(*pc).x.xn).sub.nodep.l.lptr
                    } else {
                        r_get_lhs((*pc).x.xn, 0 as i32 != 0)
                    };
                    unref(*lhs);
                    *lhs = dupnode(t1);
                }
                current_block = 2242099707034464334;
            }
            94 => {
                let fresh83 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                r = (*fresh83).rptr;
                free_arrayfor(r);
                current_block = 2242099707034464334;
            }
            69 => {
                r = ((*pc).d.fptr)
                    .expect("non-null function pointer")((*pc).x.xl as i32);
                let ref mut fresh84 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh84 = r;
                current_block = 2242099707034464334;
            }
            71 => {
                let mut arg_count: size_t = (*pc).x.xl as size_t;
                let mut f: *mut awk_ext_func_t = (*pc.offset(1 as i32 as isize)).x.exf;
                let mut min_req: size_t = (*f).min_required_args;
                let mut max_expect: size_t = (*f).max_expected_args;
                let mut result: awk_value_t = awk_value_t {
                    val_type: awk_valtype_t::AWK_UNDEFINED,
                    u: C2RustUnnamed_0 {
                        s: awk_string_t {
                            str_0: 0 as *mut i8,
                            len: 0,
                        },
                    },
                };
                if arg_count < min_req {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1101 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"%s: called with %lu arguments, expecting at least %lu\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*pc.offset(1 as i32 as isize)).d.name,
                        arg_count,
                        min_req,
                    );
                }
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    && (*f).suppress_lint as u64 == 0 && arg_count > max_expect
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1107 as i32,
                    );
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"%s: called with %lu arguments, expecting no more than %lu\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*pc.offset(1 as i32 as isize)).d.name,
                        arg_count,
                        max_expect,
                    );
                }
                PUSH_CODE(pc);
                let mut ef_ret: *mut awk_value_t = ((*pc).d.efptr)
                    .expect(
                        "non-null function pointer",
                    )(arg_count as i32, &mut result, f);
                r = awk_value_to_node(ef_ret);
                POP_CODE();
                loop {
                    let fresh85 = arg_count;
                    arg_count = arg_count.wrapping_sub(1);
                    if !(fresh85 > 0 as i32 as u64) {
                        break;
                    }
                    let fresh86 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    t1 = (*fresh86).rptr;
                    if (*t1).type_0 as u32 == nodevals::Node_val as i32 as u32
                        || (*t1).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
                    {
                        DEREF(t1);
                    }
                }
                free_api_string_copies();
                if in_indirect_call {
                    let fresh87 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    let mut fname: *mut NODE = (*fresh87).rptr;
                    DEREF(fname);
                    in_indirect_call = 0 as i32 != 0;
                }
                let ref mut fresh88 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh88 = r;
                current_block = 2242099707034464334;
            }
            70 => {
                r = do_sub((*pc).x.xl as i32, (*pc).d.dl as u32);
                let ref mut fresh89 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh89 = r;
                current_block = 2242099707034464334;
            }
            56 => {
                do_print((*pc).x.xl as i32, (*pc).d.dl as i32);
                current_block = 2242099707034464334;
            }
            58 => {
                do_printf((*pc).x.xl as i32, (*pc).d.dl as i32);
                current_block = 2242099707034464334;
            }
            57 => {
                do_print_rec((*pc).x.xl as i32, (*pc).d.dl as i32);
                current_block = 2242099707034464334;
            }
            79 => {
                m = (*pc).d.dn;
                if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                    r = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = r;
                } else if (*m).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh90 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh90 = m;
                current_block = 2242099707034464334;
            }
            49 => {
                m = (*pc).d.dn;
                t1 = *get_field(0 as i32 as i64, 0 as *mut Func_ptr);
                current_block = 9616300735396971712;
            }
            50 | 48 => {
                m = (*pc).d.dn;
                t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = t1;
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                }
                current_block = 9616300735396971712;
            }
            74 => {
                let mut f_0: *mut NODE = 0 as *mut NODE;
                let mut arg_count_0: i32 = 0;
                let mut save_1: i8 = 0;
                let mut function_name: *mut NODE = 0 as *mut NODE;
                arg_count_0 = (*pc.offset(1 as i32 as isize)).x.xl as i32;
                t1 = (*stack_ptr.offset(-(arg_count_0 as isize))).rptr;
                if (*t1).type_0 as u32 != nodevals::Node_val as i32 as u32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1209 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"indirect function call requires a simple scalar value\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
                str_terminate_f(t1, &mut save_1);
                if (*t1).sub.val.slen > 0 as i32 as u64 {
                    f_0 = (*pc).x.xn;
                    if !f_0.is_null()
                        && strcmp((*f_0).sub.nodep.name, (*t1).sub.val.sp) == 0 as i32
                    {
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        pc = ni;
                        continue;
                    } else {
                        f_0 = lookup((*t1).sub.val.sp);
                    }
                }
                if f_0.is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1227 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`%s' is not a function, so it cannot be called indirectly\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*t1).sub.val.sp,
                    );
                    current_block = 4159430576127784976;
                } else if (*f_0).type_0 as u32
                    == nodevals::Node_builtin_func as i32 as u32
                {
                    let mut arg_count_1: i32 = (*pc.offset(1 as i32 as isize)).x.xl
                        as i32;
                    let mut the_func: builtin_func_t = lookup_builtin((*t1).sub.val.sp);
                    if the_func
                        == ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(i32, u32) -> *mut NODE>,
                            builtin_func_t,
                        >(Some(do_sub as unsafe extern "C" fn(i32, u32) -> *mut NODE))
                    {
                        r = call_sub((*t1).sub.val.sp, arg_count_1);
                    } else if the_func
                        == Some(do_match as unsafe extern "C" fn(i32) -> *mut NODE)
                    {
                        r = call_match(arg_count_1);
                    } else if the_func
                        == Some(do_split as unsafe extern "C" fn(i32) -> *mut NODE)
                        || the_func
                            == Some(
                                do_patsplit as unsafe extern "C" fn(i32) -> *mut NODE,
                            )
                    {
                        r = call_split_func((*t1).sub.val.sp, arg_count_1);
                    } else {
                        r = the_func.expect("non-null function pointer")(arg_count_1);
                    }
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    let fresh92 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    function_name = (*fresh92).rptr;
                    DEREF(function_name);
                    let ref mut fresh93 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh93 = r;
                    current_block = 2242099707034464334;
                } else if (*f_0).type_0 as u32 != nodevals::Node_func as i32 as u32 {
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    if (*f_0).type_0 as u32 == nodevals::Node_ext_func as i32 as u32 {
                        let mut bc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                        let mut fname_0: *mut i8 = (*pc).d.name;
                        let mut arg_count_2: i32 = (*pc.offset(1 as i32 as isize)).x.xl
                            as i32;
                        static mut npc: [INSTRUCTION; 2] = [INSTRUCTION {
                            nexti: 0 as *const exp_instruction as *mut exp_instruction,
                            d: C2RustUnnamed_7 {
                                dn: 0 as *const NODE as *mut NODE,
                            },
                            x: C2RustUnnamed_6 { xl: 0 },
                            comment: 0 as *const exp_instruction as *mut exp_instruction,
                            source_line: 0,
                            pool_size: 0,
                            opcode: opcodeval::Op_illegal,
                        }; 2];
                        npc[0 as i32 as usize] = *pc;
                        bc = (*f_0).sub.nodep.r.iptr;
                        npc[0 as i32 as usize].opcode = opcodeval::Op_ext_builtin;
                        npc[0 as i32 as usize].d.efptr = (*bc).d.efptr;
                        npc[0 as i32 as usize].x.xl = arg_count_2 as i64;
                        npc[1 as i32 as usize] = *pc.offset(1 as i32 as isize);
                        npc[1 as i32 as usize].d.name = fname_0;
                        npc[1 as i32 as usize].x.exf = (*bc).x.exf;
                        in_indirect_call = 1 as i32 != 0;
                        ni = npc.as_mut_ptr();
                        pc = ni;
                        continue;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            1277 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"function called indirectly through `%s' does not exist\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*pc).d.name,
                        );
                    }
                    current_block = 4159430576127784976;
                } else {
                    current_block = 4159430576127784976;
                }
                match current_block {
                    2242099707034464334 => {}
                    _ => {
                        (*pc).x.xn = f_0;
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        pc = ni;
                        continue;
                    }
                }
            }
            73 => {
                let mut f_1: *mut NODE = 0 as *mut NODE;
                f_1 = (*pc).x.xn;
                if f_1.is_null() {
                    f_1 = lookup((*pc).d.name);
                    if f_1.is_null()
                        || (*f_1).type_0 as u32 != nodevals::Node_func as i32 as u32
                            && (*f_1).type_0 as u32
                                != nodevals::Node_ext_func as i32 as u32
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const i8,
                            1296 as i32,
                        );
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"function `%s' not defined\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*pc).d.name,
                        );
                    }
                    (*pc).x.xn = f_1;
                }
                if (*f_1).type_0 as u32 == nodevals::Node_ext_func as i32 as u32 {
                    let mut bc_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut fname_1: *mut i8 = (*pc).d.name;
                    let mut arg_count_3: i32 = (*pc.offset(1 as i32 as isize)).x.xl
                        as i32;
                    bc_0 = (*f_1).sub.nodep.r.iptr;
                    (*pc).opcode = opcodeval::Op_ext_builtin;
                    (*pc).d.efptr = (*bc_0).d.efptr;
                    (*pc).x.xl = arg_count_3 as i64;
                    let ref mut fresh94 = (*pc.offset(1 as i32 as isize)).d.name;
                    *fresh94 = fname_1;
                    let ref mut fresh95 = (*pc.offset(1 as i32 as isize)).x.exf;
                    *fresh95 = (*bc_0).x.exf;
                    ni = pc;
                    pc = ni;
                    continue;
                } else {
                    ni = setup_frame(pc);
                    pc = ni;
                    continue;
                }
            }
            62 => {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected opcode %s\0"
                        as *const u8 as *const i8,
                    b"./interpret.h\0" as *const u8 as *const i8,
                    1322 as i32,
                    opcode2str(op),
                );
                current_block = 2242099707034464334;
            }
            61 => {
                m = POP_SCALAR();
                ni = unwind_stack((*frame_ptr).sub.nodep.reflags as i64);
                let ref mut fresh96 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh96 = m;
                pc = ni;
                continue;
            }
            65 => {
                r = do_getline_redir((*pc).x.xl as i32, (*pc).d.dl as redirval);
                let ref mut fresh97 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh97 = r;
                current_block = 2242099707034464334;
            }
            66 => {
                if currule == 0 || currule == defrule::BEGINFILE as i32
                    || currule == defrule::ENDFILE as i32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1342 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"non-redirected `getline' invalid inside `%s' rule\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                loop {
                    let mut ret: i32 = 0;
                    ret = nextfile(&mut curfile, 0 as i32 != 0);
                    if ret <= 0 as i32 {
                        r = do_getline((*pc).x.xl as i32, curfile);
                        if !r.is_null() {
                            break;
                        }
                    } else {
                        push_exec_state(pc, currule, source, stack_ptr);
                        if curfile.is_null() {
                            pc = (*pc.offset(1 as i32 as isize)).x.xi;
                            continue '_top;
                        } else {
                            pc = (*pc.offset(1 as i32 as isize)).d.di;
                            continue '_top;
                        }
                    }
                }
                let ref mut fresh98 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh98 = r;
                current_block = 2242099707034464334;
            }
            100 => {
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut i64);
                pc = ni;
                continue;
            }
            99 => {
                after_beginfile(&mut curfile);
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut i64);
                if (*ni).opcode as u32 == opcodeval::Op_K_getline as i32 as u32
                    || curfile.is_null()
                {
                    pc = ni;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            91 => {
                let mut ret_0: i32 = 0;
                ret_0 = nextfile(&mut curfile, 0 as i32 != 0);
                if ret_0 < 0 as i32 {
                    pc = (*pc).d.di;
                    continue;
                } else if ret_0 == 0 as i32 {
                    pc = (*pc.offset(1 as i32 as isize)).x.xi;
                    continue;
                } else {
                    push_exec_state(pc, currule, source, stack_ptr);
                    if curfile.is_null() {
                        pc = (*pc).x.xi;
                        continue;
                    }
                }
                current_block = 2242099707034464334;
            }
            90 => {
                let mut errcode: i32 = 0 as i32;
                ni = (*pc).d.di;
                if curfile.is_null() {
                    ni = (*ni).d.di;
                    pc = ni;
                    continue;
                } else if !inrec(curfile, &mut errcode) {
                    if errcode > 0 as i32 {
                        update_ERRNO_int(errcode);
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            != 0 || (*pc).x.xl == 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const i8,
                                1433 as i32,
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
                                    b"error reading input file `%s': %s\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (*curfile).public.name,
                                strerror(errcode),
                            );
                        }
                    }
                    pc = ni;
                    continue;
                }
                current_block = 2242099707034464334;
            }
            67 => {
                let mut ret_1: i32 = 0;
                if currule != defrule::Rule as i32
                    && currule != defrule::BEGINFILE as i32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1448 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`nextfile' cannot be called from a `%s' rule\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                ret_1 = nextfile(&mut curfile, 1 as i32 != 0);
                if currule == defrule::BEGINFILE as i32 {
                    let mut stack_size: i64 = 0 as i32 as i64;
                    ni = pop_exec_state(&mut currule, &mut source, &mut stack_size);
                    unwind_stack(stack_size);
                    if ret_1 == 0 as i32 {
                        pc = ni;
                        continue;
                    } else {
                        push_exec_state(ni, currule, source, stack_ptr);
                        pc = (*pc).x.xi;
                        continue;
                    }
                } else {
                    unwind_stack(0 as i32 as i64);
                    push_exec_state((*pc).d.di, currule, source, stack_ptr);
                    pc = (*pc).x.xi;
                    continue;
                }
            }
            60 => {
                if currule == 0 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1495 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`exit' cannot be called in the current context\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                exiting = 1 as i32 != 0;
                t1 = force_number(POP_SCALAR());
                if t1 != Nnull_string {
                    exit_val = (*t1).sub.val.fltnum as i64 as i32;
                }
                DEREF(t1);
                if currule == defrule::BEGINFILE as i32
                    || currule == defrule::ENDFILE as i32
                {
                    pop_exec_state(&mut currule, &mut source, 0 as *mut i64);
                }
                unwind_stack(0 as i32 as i64);
                if currule == defrule::END as i32 {
                    ni = (*pc).x.xi;
                } else {
                    ni = (*pc).d.di;
                }
                pc = ni;
                continue;
            }
            59 => {
                if currule != defrule::Rule as i32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        1536 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`next' cannot be called from a `%s' rule\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                unwind_stack(0 as i32 as i64);
                pc = (*pc).d.di;
                continue;
            }
            86 => {
                r = POP_SCALAR();
                DEREF(r);
                current_block = 2242099707034464334;
            }
            14 => {
                if (*pc).x.xl != 0 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            15 => {
                let mut result_0: i32 = 0;
                let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                t1 = TOP_SCALAR();
                di = (eval_condition(t1) as i32 != 0 as i32) as i32;
                DEREF(t1);
                ip = (*pc).x.xi;
                if (*ip).x.xl == 0 && di != 0 {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    (*ip).x.xl = 1 as i32 as i64;
                    pc = (*ip).d.di;
                    continue;
                } else {
                    result_0 = ((*ip).x.xl != 0 || di != 0) as i32;
                    (*ip).x.xl ^= di as i64;
                    r = node_Boolean[result_0 as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    (*stack_ptr).rptr = r;
                    pc = (*pc).d.di;
                    continue;
                }
            }
            103 => {
                if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
                    (*pc).d.ldl = ((*pc).d.ldl).wrapping_add(1);
                    (*pc).d.ldl;
                }
                current_block = 2242099707034464334;
            }
            85 | 112 | 115 | 113 | 114 | 116 | 53 | 117 | 118 | 120 | 102 | 121 => {
                current_block = 2242099707034464334;
            }
            _ => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"./interpret.h\0" as *const u8 as *const i8, 1599 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"Sorry, don't know how to interpret `%s'\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    opcode2str(op),
                );
                current_block = 2242099707034464334;
            }
        }
        match current_block {
            9616300735396971712 => {
                rp = re_update(m);
                di = research(
                    rp,
                    (*t1).sub.val.sp,
                    0 as i32,
                    (*t1).sub.val.slen,
                    0 as i32,
                );
                di = (di == -(1 as i32)) as i32
                    ^ (op as u32 != opcodeval::Op_nomatch as i32 as u32) as i32;
                if op as u32 != opcodeval::Op_match_rec as i32 as u32 {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    DEREF(t1);
                    if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                        DEREF((*m).sub.nodep.x.extra);
                        (*m).sub.nodep.x.extra = 0 as *mut exp_node;
                    }
                }
                r = node_Boolean[di as usize];
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh91 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh91 = r;
            }
            11021467851294229602 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as i32 as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        664 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"division by zero attempted in `%%'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                x = fmod((*t1).sub.val.fltnum, x2);
                r = make_number.expect("non-null function pointer")(x);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            5149334253309972495 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as i32 as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const i8,
                        648 as i32,
                    );
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"division by zero attempted\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum / x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            16935727273394377761 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect(
                        "non-null function pointer",
                    )(calc_exp((*t1).sub.val.fltnum, x2));
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            6406384398533964377 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum * x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            8008614458410856352 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum - x2);
                (*r).sub.val.fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            9390321967027614628 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number
                    .expect("non-null function pointer")((*t1).sub.val.fltnum + x2);
                (*r).sub.val.fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            8953125900534742068 => {
                source = (*pc).d.name;
            }
            11503388136377843356 => {
                let ref mut fresh68 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh68 = (*pc).d.dn;
            }
            _ => {}
        }
        pc = (*pc).nexti;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn init_interpret() {
    let mut newval: i64 = 0;
    newval = getenv_long(b"GAWK_STACKSIZE\0" as *const u8 as *const i8);
    if newval > 0 as i32 as i64 {
        STACK_SIZE = newval as u64;
    }
    stack_bottom = emalloc_real(
        STACK_SIZE.wrapping_mul(::core::mem::size_of::<STACK_ITEM>() as u64),
        b"grow_stack\0" as *const u8 as *const i8,
        b"stack_bottom\0" as *const u8 as *const i8,
        b"eval.c\0" as *const u8 as *const i8,
        1860 as i32,
    ) as *mut STACK_ITEM;
    stack_ptr = stack_bottom.offset(-(1 as i32 as isize));
    stack_top = stack_bottom.offset(STACK_SIZE as isize).offset(-(1 as i32 as isize));
    frame_ptr = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !frame_ptr.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(frame_ptr
            as *mut block_item))
            .freep;
    } else {
        frame_ptr = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*frame_ptr).type_0 = nodevals::Node_frame;
    (*frame_ptr).sub.nodep.r.av = 0 as *mut *mut exp_node;
    (*frame_ptr).sub.nodep.x.extra = 0 as *mut exp_node;
    (*frame_ptr).sub.nodep.name = 0 as *mut i8;
    node_Boolean[0 as i32 as usize] = make_number
        .expect("non-null function pointer")(0.0f64);
    node_Boolean[1 as i32 as usize] = make_number
        .expect("non-null function pointer")(1.0f64);
    if 0 as i32 == 0 {
        (*node_Boolean[0 as i32 as usize]).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >(
            (*node_Boolean[0 as i32 as usize]).flags as u32
                | flagvals::NUMINT as i32 as u32,
        );
        (*node_Boolean[1 as i32 as usize]).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >(
            (*node_Boolean[1 as i32 as usize]).flags as u32
                | flagvals::NUMINT as i32 as u32,
        );
    }
    if num_exec_hook > 0 as i32 {
        interpret = Some(h_interpret as unsafe extern "C" fn(*mut INSTRUCTION) -> i32);
    } else {
        interpret = Some(r_interpret as unsafe extern "C" fn(*mut INSTRUCTION) -> i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn elem_new_to_scalar(mut n: *mut NODE) -> *mut NODE {
    if (*n).type_0 as u32 != nodevals::Node_elem_new as i32 as u32 {
        return n;
    }
    if (*n).valref > 1 as i32 as i64 {
        unref(n);
        return dupnode(Nnull_string);
    }
    (*n).type_0 = nodevals::Node_val;
    return n;
}
unsafe extern "C" fn fix_nan_sign(
    mut left: libc::c_double,
    mut right: libc::c_double,
    mut result: libc::c_double,
) -> libc::c_double {
    if (if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf(left as libc::c_float)
    } else {
        (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_double>() as u64
        {
            __isnan(left)
        } else {
            __isnanl(f128::f128::new(left))
        })
    }) != 0
        && (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_float>() as u64
        {
            (left as libc::c_float).is_sign_negative() as i32
        } else {
            (if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                left.is_sign_negative() as i32
            } else {
                (f128::f128::new(left)).is_sign_negative() as i32
            })
        }) != 0
    {
        return copysign(result, -1.0f64)
    } else if (if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf(right as libc::c_float)
    } else {
        (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_double>() as u64
        {
            __isnan(right)
        } else {
            __isnanl(f128::f128::new(right))
        })
    }) != 0
        && (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_float>() as u64
        {
            (right as libc::c_float).is_sign_negative() as i32
        } else {
            (if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                right.is_sign_negative() as i32
            } else {
                (f128::f128::new(right)).is_sign_negative() as i32
            })
        }) != 0
    {
        return copysign(result, -1.0f64)
    } else {
        return result
    };
}