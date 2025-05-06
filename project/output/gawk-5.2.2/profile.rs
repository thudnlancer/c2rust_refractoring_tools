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
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fileno(__stream: *mut FILE) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
    static ruletab: [*const i8; 0];
    static mut Nnull_string: *mut NODE;
    static mut sourceline: i32;
    static mut source: *mut i8;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static awk_namespace: [i8; 0];
    static mut current_namespace: *const i8;
    static mut namespace_changed: bool;
    fn dump_funcs();
    fn getfname(
        _: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
        prepend_awk: bool,
    ) -> *const i8;
    fn nodetype2str(type_0: NODETYPE) -> *const i8;
    fn opcode2str(type_0: OPCODE) -> *const i8;
    fn op2str(type_0: OPCODE) -> *const i8;
    fn dump_fcall_stack(fp: *mut FILE);
    fn devopen_simple(name: *const i8, mode: *const i8, try_real_open: bool) -> i32;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn final_exit(status: i32) -> !;
    fn r_warning(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn r_fatal(mesg: *const i8, _: ...);
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn is_all_upper(name: *const i8) -> bool;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
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
pub type Func_print = Option<unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32>;
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
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut i8,
    pub fullpath: *mut i8,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: i32,
    pub bufsize: size_t,
    pub buf: *mut i8,
    pub line_offset: *mut i32,
    pub fd: i32,
    pub maxlen: i32,
    pub fini_func: Option<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut i8,
    pub lexend: *mut i8,
    pub lexeme: *mut i8,
    pub lexptr_begin: *mut i8,
    pub lasttok: i32,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum srctype {
    SRC_CMDLINE = 1,
    SRC_STDIN,
    SRC_FILE,
    SRC_INC,
    SRC_EXTLIB,
}
impl srctype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            srctype::SRC_CMDLINE => 1,
            srctype::SRC_STDIN => 2,
            srctype::SRC_FILE => 3,
            srctype::SRC_INC => 4,
            srctype::SRC_EXTLIB => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> srctype {
        match value {
            1 => srctype::SRC_CMDLINE,
            2 => srctype::SRC_STDIN,
            3 => srctype::SRC_FILE,
            4 => srctype::SRC_INC,
            5 => srctype::SRC_EXTLIB,
            _ => panic!("Invalid value for srctype: {}", value),
        }
    }
}
impl AddAssign<u32> for srctype {
    fn add_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for srctype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for srctype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for srctype {
    fn div_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for srctype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for srctype {
    type Output = srctype;
    fn add(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for srctype {
    type Output = srctype;
    fn sub(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for srctype {
    type Output = srctype;
    fn mul(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for srctype {
    type Output = srctype;
    fn div(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for srctype {
    type Output = srctype;
    fn rem(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type SRCFILE = srcfile;
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
static mut pp_stack: *mut NODE = 0 as *const NODE as *mut NODE;
static mut func_params: *mut NODE = 0 as *const NODE as *mut NODE;
static mut prof_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut indent_level: i64 = 0 as i32 as i64;
static mut tabs: [i8; 28] = unsafe {
    *::core::mem::transmute::<
        &[u8; 28],
        &[i8; 28],
    >(b"\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\0")
};
static mut tabs_len: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn set_prof_file(mut file: *const i8) {
    let mut fd: i32 = 0;
    fd = devopen_simple(file, b"w\0" as *const u8 as *const i8, 1 as i32 != 0);
    if fd == -(1 as i32) {
        prof_fp = 0 as *mut FILE;
    } else if fd == fileno(stdout) {
        prof_fp = stdout;
    } else if fd == fileno(stderr) {
        prof_fp = stderr;
    } else {
        prof_fp = fdopen(fd, b"w\0" as *const u8 as *const i8);
    }
    if prof_fp.is_null() {
        let mut e: i32 = *__errno_location();
        if fd != -(1 as i32) && fd != fileno(stdout) && fd != fileno(stderr) {
            close(fd);
        }
        *__errno_location() = e;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"profile.c\0" as *const u8 as *const i8, 110 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"could not open `%s' for writing: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file,
            strerror(*__errno_location()),
        );
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"profile.c\0" as *const u8 as *const i8, 112 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"sending profile to standard error\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        prof_fp = stderr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_profiling_signals() {
    signal(
        1 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(dump_and_exit as unsafe extern "C" fn(i32) -> !)),
    );
    signal(10 as i32, Some(just_dump as unsafe extern "C" fn(i32) -> ()));
}
unsafe extern "C" fn indent(mut count: exec_count_t) {
    let mut i: i32 = 0;
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        if count == 0 as i32 as libc::c_ulonglong {
            fprintf(prof_fp, b"\t\0" as *const u8 as *const i8);
        } else {
            fprintf(prof_fp, b"%6llu  \0" as *const u8 as *const i8, count);
        }
    }
    i = 0 as i32;
    while (i as i64) < indent_level {
        fprintf(prof_fp, b"\t\0" as *const u8 as *const i8);
        i += 1;
        i;
    }
}
unsafe extern "C" fn indent_in() {
    indent_level += 1;
    indent_level;
}
unsafe extern "C" fn indent_out() {
    indent_level -= 1;
    indent_level;
}
unsafe extern "C" fn pp_push(
    mut type_0: i32,
    mut s: *mut i8,
    mut flag: i32,
    mut comment: *mut INSTRUCTION,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n as *mut block_item))
            .freep;
    } else {
        n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*n).sub.nodep.name = s;
    (*n).sub.nodep.reserved = strlen(s);
    (*n).flags = flagvals::from_libc_c_uint(flag as u32);
    (*n).type_0 = type_0 as NODETYPE;
    (*n).sub.nodep.r.rptr = pp_stack;
    (*n).sub.nodep.x.cmnt = comment as *mut libc::c_void;
    pp_stack = n;
}
unsafe extern "C" fn pp_pop() -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = pp_stack;
    pp_stack = (*n).sub.nodep.r.rptr;
    return n;
}
unsafe extern "C" fn pp_free(mut n: *mut NODE) {
    if (*n).flags as u32 & 2 as i32 as u32 != 0 as i32 as u32 {
        pma_free((*n).sub.nodep.name as *mut libc::c_void);
    }
    let ref mut fresh0 = (*(n as *mut block_item)).freep;
    *fresh0 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = n as *mut block_item;
}
unsafe extern "C" fn pprint(
    mut startp: *mut INSTRUCTION,
    mut endp: *mut INSTRUCTION,
    mut flags: i32,
) {
    let mut current_block: u64;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut ip1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut i8 = 0 as *mut i8;
    let mut rule: i32 = 0;
    static mut rule_count: [i32; 6] = [0; 6];
    static mut skip_comment: bool = 0 as i32 != 0;
    pc = startp;
    while pc != endp {
        if (*pc).source_line as i32 > 0 as i32 {
            sourceline = (*pc).source_line as i32;
        }
        if (*pc).opcode as u32 == opcodeval::Op_comment as i32 as u32
            && (*(*pc).d.dn).sub.val.comtype as u32
                == commenttype::EOL_COMMENT as i32 as u32 && skip_comment as i32 != 0
        {
            skip_comment = 0 as i32 != 0;
        } else {
            skip_comment = 0 as i32 != 0;
            match (*pc).opcode as u32 {
                51 => {
                    source = (*pc).d.name;
                    rule = (*pc).x.xl as i32;
                    pp_namespace_list((*pc.offset(3 as i32 as isize)).nexti);
                    if rule != defrule::Rule as i32 {
                        if (*pc).nexti != (*pc.offset(1 as i32 as isize)).x.xi
                            && (*(*pc).nexti).opcode as u32
                                == opcodeval::Op_comment as i32 as u32
                            && (*(*(*pc).nexti).d.dn).sub.val.comtype as u32
                                == commenttype::BLOCK_COMMENT as i32 as u32
                        {
                            print_comment((*pc).nexti, -(1 as i32) as i64);
                        }
                        ip1 = (*pc.offset(1 as i32 as isize)).x.xi;
                        ip2 = (*pc.offset(1 as i32 as isize)).d.di;
                        if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32
                            != 0
                        {
                            let fresh1 = rule_count[rule as usize];
                            rule_count[rule as usize] = rule_count[rule as usize] + 1;
                            if fresh1 == 0 {
                                fprintf(
                                    prof_fp,
                                    dcgettext(
                                        0 as *const i8,
                                        b"\t# %s rule(s)\n\n\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    *ruletab.as_ptr().offset(rule as isize),
                                );
                            }
                            indent(0 as i32 as exec_count_t);
                        }
                        fprintf(
                            prof_fp,
                            b"%s {\0" as *const u8 as *const i8,
                            *ruletab.as_ptr().offset(rule as isize),
                        );
                        end_line(pc);
                        skip_comment = 1 as i32 != 0;
                    } else {
                        if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32
                            != 0
                            && {
                                let fresh2 = rule_count[rule as usize];
                                rule_count[rule as usize] = rule_count[rule as usize] + 1;
                                fresh2 == 0
                            }
                        {
                            fprintf(
                                prof_fp,
                                dcgettext(
                                    0 as *const i8,
                                    b"\t# defrule::Rule(s)\n\n\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        ip1 = (*pc).nexti;
                        indent((*ip1).d.ldl);
                        if ip1 != (*pc.offset(1 as i32 as isize)).x.xi {
                            pprint(
                                (*ip1).nexti,
                                (*pc.offset(1 as i32 as isize)).x.xi,
                                0 as i32,
                            );
                            if (*(*(*ip1).nexti).nexti).nexti
                                != (*pc.offset(1 as i32 as isize)).x.xi
                                || (*(*ip1).nexti).opcode as u32
                                    != opcodeval::Op_comment as i32 as u32
                            {
                                t1 = pp_pop();
                                fprintf(
                                    prof_fp,
                                    b"%s {\0" as *const u8 as *const i8,
                                    (*t1).sub.nodep.name,
                                );
                                pp_free(t1);
                            } else {
                                fprintf(prof_fp, b"{\0" as *const u8 as *const i8);
                            }
                            ip1 = (*pc.offset(1 as i32 as isize)).x.xi;
                            ip2 = (*pc.offset(1 as i32 as isize)).d.di;
                            if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32
                                != 0 && (*ip1).d.ldl > 0 as i32 as libc::c_ulonglong
                            {
                                fprintf(
                                    prof_fp,
                                    b" # %llu\0" as *const u8 as *const i8,
                                    (*ip1).d.ldl,
                                );
                            }
                            end_line(ip1);
                            skip_comment = 1 as i32 != 0;
                        } else {
                            fprintf(prof_fp, b"{\n\0" as *const u8 as *const i8);
                            ip1 = (*pc.offset(1 as i32 as isize)).x.xi;
                            ip2 = (*pc.offset(1 as i32 as isize)).d.di;
                        }
                        ip1 = (*ip1).nexti;
                    }
                    indent_in();
                    pprint(ip1, ip2, 0 as i32);
                    indent_out();
                    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
                        indent(0 as i32 as exec_count_t);
                    }
                    fprintf(prof_fp, b"}\n\n\0" as *const u8 as *const i8);
                    pc = (*pc.offset(1 as i32 as isize)).d.di;
                    current_block = 4014953649076014837;
                }
                108 => {
                    memset(
                        rule_count.as_mut_ptr() as *mut libc::c_void,
                        0 as i32,
                        (defrule::MAXRULE as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<i32>() as u64),
                    );
                    current_block = 4014953649076014837;
                }
                78 => {
                    m = (*pc).d.dn;
                    if m == Nnull_string {
                        pp_push(
                            (*pc).opcode as i32,
                            (*m).sub.val.sp,
                            1 as i32,
                            (*pc).comment,
                        );
                    } else if (*m).flags as u32 & flagvals::NUMBER as i32 as u32
                        != 0 as i32 as u32
                    {
                        pp_push(
                            (*pc).opcode as i32,
                            pp_number(m),
                            2 as i32,
                            (*pc).comment,
                        );
                    } else {
                        str = pp_string((*m).sub.val.sp, (*m).sub.val.slen, '"' as i32);
                        if (*m).flags as u32 & flagvals::INTLSTR as i32 as u32
                            != 0 as i32 as u32
                        {
                            let mut tmp_0: *mut i8 = str;
                            str = pp_group3(
                                b"_\0" as *const u8 as *const i8,
                                tmp_0,
                                b"\0" as *const u8 as *const i8,
                            );
                            pma_free(tmp_0 as *mut libc::c_void);
                        }
                        pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    }
                    current_block = 4014953649076014837;
                }
                27 => {
                    if !((*pc).x.xn).is_null() {
                        pp_push(
                            opcodeval::Op_push_i as i32,
                            pp_node((*pc).x.xn),
                            2 as i32,
                            (*pc).comment,
                        );
                    }
                    current_block = 17991450254610602456;
                }
                28 | 37 | 82 | 81 | 80 | 75 | 76 | 77 => {
                    current_block = 17991450254610602456;
                }
                17 | 83 | 16 => {
                    tmp = pp_list(
                        (*pc).d.dl as i32,
                        op2str((*pc).opcode),
                        b", \0" as *const u8 as *const i8,
                    );
                    t1 = pp_pop();
                    str = pp_group3(
                        (*t1).sub.nodep.name,
                        tmp,
                        b"\0" as *const u8 as *const i8,
                    );
                    pma_free(tmp as *mut libc::c_void);
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                38 | 40 => {
                    pprint((*pc).nexti, (*pc).d.di, flags);
                    t2 = pp_pop();
                    t1 = pp_pop();
                    parenthesize((*pc).opcode as i32, t1, t2);
                    if ((*pc).comment).is_null() {
                        str = pp_group3(
                            (*t1).sub.nodep.name,
                            op2str((*pc).opcode),
                            (*t2).sub.nodep.name,
                        );
                    } else {
                        if (indent_level + 1 as i32 as i64) as u64 > tabs_len {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const i8,
                                428 as i32,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        let mut len: size_t = (strlen((*t1).sub.nodep.name))
                            .wrapping_add(strlen(op2str((*pc).opcode)))
                            .wrapping_add(strlen((*t2).sub.nodep.name))
                            .wrapping_add(indent_level as u64)
                            .wrapping_add(1 as i32 as u64)
                            .wrapping_add((*(*(*pc).comment).d.dn).sub.val.slen)
                            .wrapping_add(3 as i32 as u64);
                        str = emalloc_real(
                            len,
                            b"pprint\0" as *const u8 as *const i8,
                            b"str\0" as *const u8 as *const i8,
                            b"profile.c\0" as *const u8 as *const i8,
                            435 as i32,
                        ) as *mut i8;
                        sprintf(
                            str,
                            b"%s%s%s%.*s %s\0" as *const u8 as *const i8,
                            (*t1).sub.nodep.name,
                            op2str((*pc).opcode),
                            (*(*(*pc).comment).d.dn).sub.val.sp,
                            (indent_level + 1 as i32 as i64) as i32,
                            tabs.as_ptr(),
                            (*t2).sub.nodep.name,
                        );
                    }
                    pp_free(t1);
                    pp_free(t2);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    pc = (*pc).d.di;
                    current_block = 4014953649076014837;
                }
                8 | 10 | 2 | 12 | 4 | 6 => {
                    m = (*pc).d.dn;
                    t1 = pp_pop();
                    if prec_level((*pc).opcode as i32) > prec_level((*t1).type_0 as i32)
                        && is_binary((*t1).type_0 as i32) as i32 != 0
                    {
                        pp_parenthesize(t1);
                    }
                    if (*m).flags as u32 & flagvals::NUMBER as i32 as u32
                        != 0 as i32 as u32
                    {
                        tmp = pp_number(m);
                    } else {
                        tmp = pp_string((*m).sub.val.sp, (*m).sub.val.slen, '"' as i32);
                    }
                    str = pp_group3((*t1).sub.nodep.name, op2str((*pc).opcode), tmp);
                    pma_free(tmp as *mut libc::c_void);
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                121 => {
                    t1 = pp_pop();
                    str = pp_group3(
                        b"(\0" as *const u8 as *const i8,
                        (*t1).sub.nodep.name,
                        b")\0" as *const u8 as *const i8,
                    );
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                7 | 9 | 1 | 11 | 3 | 5 | 42 | 43 | 44 | 45 | 46 | 47 => {
                    t2 = pp_pop();
                    t1 = pp_pop();
                    parenthesize((*pc).opcode as i32, t1, t2);
                    str = pp_group3(
                        (*t1).sub.nodep.name,
                        op2str((*pc).opcode),
                        (*t2).sub.nodep.name,
                    );
                    pp_free(t1);
                    pp_free(t2);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                18 | 19 | 20 | 21 => {
                    t1 = pp_pop();
                    if (*pc).opcode as u32 == opcodeval::Op_preincrement as i32 as u32
                        || (*pc).opcode as u32
                            == opcodeval::Op_predecrement as i32 as u32
                    {
                        str = pp_group3(
                            op2str((*pc).opcode),
                            (*t1).sub.nodep.name,
                            b"\0" as *const u8 as *const i8,
                        );
                    } else {
                        str = pp_group3(
                            (*t1).sub.nodep.name,
                            op2str((*pc).opcode),
                            b"\0" as *const u8 as *const i8,
                        );
                    }
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                24 | 84 | 22 | 23 | 25 => {
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as i32) as i32 != 0
                        || (*t1).type_0 as OPCODE as u32 == (*pc).opcode as u32
                            && ((*pc).opcode as u32
                                == opcodeval::Op_unary_minus as i32 as u32
                                || (*pc).opcode as u32
                                    == opcodeval::Op_unary_plus as i32 as u32)
                    {
                        pp_parenthesize(t1);
                    }
                    str = pp_group3(
                        op2str((*pc).opcode),
                        (*t1).sub.nodep.name,
                        b"\0" as *const u8 as *const i8,
                    );
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                26 | 34 | 35 | 31 | 32 | 33 | 36 => {
                    t2 = pp_pop();
                    t1 = pp_pop();
                    str = pp_group3(
                        (*t2).sub.nodep.name,
                        op2str((*pc).opcode),
                        (*t1).sub.nodep.name,
                    );
                    pp_free(t2);
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                29 | 30 => {
                    let mut assignment: *mut i8 = 0 as *mut i8;
                    let mut final_0: *mut i8 = 0 as *mut i8;
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as i32) {
                        pp_parenthesize(t1);
                    }
                    t2 = pp_pop();
                    assignment = pp_group3(
                        (*t1).sub.nodep.name,
                        op2str((*pc).opcode),
                        (*t2).sub.nodep.name,
                    );
                    final_0 = pp_group3(
                        b"$\0" as *const u8 as *const i8,
                        assignment,
                        b"\0" as *const u8 as *const i8,
                    );
                    pma_free(assignment as *mut libc::c_void);
                    pp_free(t2);
                    pp_free(t1);
                    if (*pc).opcode as u32 == opcodeval::Op_store_field_exp as i32 as u32
                    {
                        pp_push(
                            (*pc).opcode as i32,
                            final_0,
                            2 as i32,
                            0 as *mut INSTRUCTION,
                        );
                    } else {
                        fprintf(prof_fp, b"%s\0" as *const u8 as *const i8, final_0);
                        pma_free(final_0 as *mut libc::c_void);
                        if flags & 1 as i32 == 0 as i32 {
                            pc = end_line(pc);
                        }
                    }
                    current_block = 4014953649076014837;
                }
                13 => {
                    str = pp_concat((*pc).x.xl as i32);
                    pp_push(opcodeval::Op_concat as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                63 => {
                    let mut array: *mut i8 = 0 as *mut i8;
                    t1 = pp_pop();
                    array = (*t1).sub.nodep.name;
                    if (*pc).x.xl > 0 as i32 as i64 {
                        let mut sub: *mut i8 = 0 as *mut i8;
                        sub = pp_list(
                            (*pc).x.xl as i32,
                            0 as *const i8,
                            if (*pc).x.xl > 1 as i32 as i64 {
                                b"][\0" as *const u8 as *const i8
                            } else {
                                b", \0" as *const u8 as *const i8
                            },
                        );
                        fprintf(
                            prof_fp,
                            b"%s %s[%s]\0" as *const u8 as *const i8,
                            op2str(opcodeval::Op_K_delete),
                            array,
                            sub,
                        );
                        pma_free(sub as *mut libc::c_void);
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s %s\0" as *const u8 as *const i8,
                            op2str(opcodeval::Op_K_delete),
                            array,
                        );
                    }
                    if flags & 1 as i32 == 0 as i32 {
                        pc = end_line(pc);
                    }
                    pp_free(t1);
                    current_block = 4014953649076014837;
                }
                64 => {
                    r_fatal(
                        b"internal error: file %s, line %d: unexpected opcode %s\0"
                            as *const u8 as *const i8,
                        b"profile.c\0" as *const u8 as *const i8,
                        591 as i32,
                        opcode2str((*pc).opcode),
                    );
                    current_block = 4014953649076014837;
                }
                72 => {
                    let mut array_0: *mut i8 = 0 as *mut i8;
                    let mut sub_0: *mut i8 = 0 as *mut i8;
                    t1 = pp_pop();
                    array_0 = (*t1).sub.nodep.name;
                    if (*pc).x.xl > 1 as i32 as i64 {
                        sub_0 = pp_list(
                            (*pc).x.xl as i32,
                            b"()\0" as *const u8 as *const i8,
                            b", \0" as *const u8 as *const i8,
                        );
                        str = pp_group3(sub_0, op2str(opcodeval::Op_in_array), array_0);
                        pma_free(sub_0 as *mut libc::c_void);
                    } else {
                        t2 = pp_pop();
                        if prec_level((*t2).type_0 as i32)
                            < prec_level(opcodeval::Op_in_array as i32)
                        {
                            pp_parenthesize(t2);
                        }
                        sub_0 = (*t2).sub.nodep.name;
                        str = pp_group3(sub_0, op2str(opcodeval::Op_in_array), array_0);
                        pp_free(t2);
                    }
                    pp_free(t1);
                    pp_push(opcodeval::Op_in_array as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                70 => {
                    let mut fname: *const i8 = b"sub\0" as *const u8 as *const i8;
                    if (*pc).d.dl & 0x1 as i32 as i64 != 0 as i32 as i64 {
                        fname = b"gsub\0" as *const u8 as *const i8;
                    } else if (*pc).d.dl & 0x2 as i32 as i64 != 0 as i32 as i64 {
                        fname = b"gensub\0" as *const u8 as *const i8;
                    }
                    tmp = pp_list(
                        (*pc).x.xl as i32,
                        b"()\0" as *const u8 as *const i8,
                        b", \0" as *const u8 as *const i8,
                    );
                    str = pp_group3(fname, tmp, b"\0" as *const u8 as *const i8);
                    pma_free(tmp as *mut libc::c_void);
                    pp_push(
                        opcodeval::Op_sub_builtin as i32,
                        str,
                        2 as i32,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                69 | 71 => {
                    let mut fname_0: *const i8 = 0 as *const i8;
                    if (*pc).opcode as u32 == opcodeval::Op_builtin as i32 as u32 {
                        let mut prepend_awk: bool = current_namespace
                            != awk_namespace.as_ptr()
                            && strcmp(current_namespace, awk_namespace.as_ptr())
                                != 0 as i32;
                        fname_0 = getfname((*pc).d.fptr, prepend_awk);
                    } else {
                        fname_0 = (*pc.offset(1 as i32 as isize)).d.name;
                    }
                    if !fname_0.is_null() {
                        if (*pc).x.xl > 0 as i32 as i64 {
                            tmp = pp_list(
                                (*pc).x.xl as i32,
                                b"()\0" as *const u8 as *const i8,
                                b", \0" as *const u8 as *const i8,
                            );
                            str = pp_group3(
                                fname_0,
                                tmp,
                                b"\0" as *const u8 as *const i8,
                            );
                            pma_free(tmp as *mut libc::c_void);
                        } else {
                            str = pp_group3(
                                fname_0,
                                b"()\0" as *const u8 as *const i8,
                                b"\0" as *const u8 as *const i8,
                            );
                        }
                        pp_push(
                            opcodeval::Op_builtin as i32,
                            str,
                            2 as i32,
                            (*pc).comment,
                        );
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"profile.c\0" as *const u8 as *const i8,
                            670 as i32,
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
                                b"internal error: builtin with null fname\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    current_block = 4014953649076014837;
                }
                56 | 58 | 57 => {
                    if (*pc).opcode as u32 == opcodeval::Op_K_print_rec as i32 as u32 {
                        tmp = estrdup(
                            b"\0" as *const u8 as *const i8,
                            0 as i32 as size_t,
                        );
                    } else if (*pc).d.dl != 0 as i32 as i64 {
                        let mut n: *mut NODE = pp_stack;
                        if (*pc).x.xl == 1 as i32 as i64
                            && *((*n).sub.nodep.name).offset(0 as i32 as isize) as i32
                                == '(' as i32
                            && *((*n).sub.nodep.name)
                                .offset(
                                    ((*n).sub.nodep.reserved).wrapping_sub(1 as i32 as u64)
                                        as isize,
                                ) as i32 == ')' as i32
                        {
                            n = pp_pop();
                            tmp = estrdup(
                                (*n).sub.nodep.name,
                                strlen((*n).sub.nodep.name),
                            );
                            pp_free(n);
                        } else {
                            tmp = pp_list(
                                (*pc).x.xl as i32,
                                b"()\0" as *const u8 as *const i8,
                                b", \0" as *const u8 as *const i8,
                            );
                        }
                    } else {
                        tmp = pp_list(
                            (*pc).x.xl as i32,
                            b"  \0" as *const u8 as *const i8,
                            b", \0" as *const u8 as *const i8,
                        );
                        *tmp
                            .offset(
                                (strlen(tmp)).wrapping_sub(1 as i32 as u64) as isize,
                            ) = '\0' as i32 as i8;
                    }
                    if (*pc).d.dl != 0 as i32 as i64 {
                        t1 = pp_pop();
                        if is_binary((*t1).type_0 as i32) {
                            pp_parenthesize(t1);
                        }
                        fprintf(
                            prof_fp,
                            b"%s%s%s%s\0" as *const u8 as *const i8,
                            op2str((*pc).opcode),
                            tmp,
                            redir2str((*pc).d.dl as i32),
                            (*t1).sub.nodep.name,
                        );
                        pp_free(t1);
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s%s\0" as *const u8 as *const i8,
                            op2str((*pc).opcode),
                            tmp,
                        );
                    }
                    pma_free(tmp as *mut libc::c_void);
                    if flags & 1 as i32 == 0 as i32 {
                        pc = end_line(pc);
                    }
                    current_block = 4014953649076014837;
                }
                79 => {
                    if (*(*pc).d.dn).type_0 as u32 != nodevals::Node_regex as i32 as u32
                        && (*(*pc).d.dn).flags as u32 & flagvals::REGEX as i32 as u32
                            == 0 as i32 as u32
                    {
                        current_block = 4014953649076014837;
                    } else {
                        current_block = 1979616052774745799;
                    }
                }
                49 => {
                    current_block = 1979616052774745799;
                }
                50 | 48 => {
                    let mut restr: *mut i8 = 0 as *mut i8;
                    let mut txt: *mut i8 = 0 as *mut i8;
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as i32) {
                        pp_parenthesize(t1);
                    }
                    txt = (*t1).sub.nodep.name;
                    m = (*pc).d.dn;
                    if (*m).type_0 as u32 == nodevals::Node_dynregex as i32 as u32 {
                        restr = txt;
                        t2 = pp_pop();
                        if is_binary((*t2).type_0 as i32) {
                            pp_parenthesize(t2);
                        }
                        txt = (*t2).sub.nodep.name;
                        str = pp_group3(txt, op2str((*pc).opcode), restr);
                        pp_free(t2);
                    } else if (*m).type_0 as u32 == nodevals::Node_val as i32 as u32
                        && (*m).flags as u32 & flagvals::REGEX as i32 as u32
                            != 0 as i32 as u32
                    {
                        restr = pp_typed_regex(
                            (*m).sub.val.sp,
                            (*m).sub.val.slen,
                            '/' as i32,
                        );
                        str = pp_group3(txt, op2str((*pc).opcode), restr);
                        pma_free(restr as *mut libc::c_void);
                    } else {
                        let mut re_0: *mut NODE = (*m).sub.nodep.x.extra;
                        restr = pp_string(
                            (*re_0).sub.val.sp,
                            (*re_0).sub.val.slen,
                            '/' as i32,
                        );
                        str = pp_group3(txt, op2str((*pc).opcode), restr);
                        pma_free(restr as *mut libc::c_void);
                    }
                    pp_free(t1);
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                66 | 65 => {
                    if (*pc).x.xl != 0 {
                        t1 = pp_pop();
                        tmp = pp_group3(
                            op2str(opcodeval::Op_K_getline),
                            b" \0" as *const u8 as *const i8,
                            (*t1).sub.nodep.name,
                        );
                        pp_free(t1);
                    } else {
                        tmp = pp_group3(
                            op2str(opcodeval::Op_K_getline),
                            b"\0" as *const u8 as *const i8,
                            b"\0" as *const u8 as *const i8,
                        );
                    }
                    if (*pc).d.dl != 0 as i32 as i64 {
                        let mut before: i32 = ((*pc).d.dl
                            == redirval::redirect_pipein as i32 as i64
                            || (*pc).d.dl == redirval::redirect_twoway as i32 as i64)
                            as i32;
                        t2 = pp_pop();
                        if is_binary((*t2).type_0 as i32) {
                            pp_parenthesize(t2);
                        }
                        if before != 0 {
                            str = pp_group3(
                                (*t2).sub.nodep.name,
                                redir2str((*pc).d.dl as i32),
                                tmp,
                            );
                        } else {
                            str = pp_group3(
                                tmp,
                                redir2str((*pc).d.dl as i32),
                                (*t2).sub.nodep.name,
                            );
                        }
                        pma_free(tmp as *mut libc::c_void);
                        pp_free(t2);
                    } else {
                        str = tmp;
                    }
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    current_block = 4014953649076014837;
                }
                74 | 73 => {
                    let mut pre: *const i8 = 0 as *const i8;
                    let mut pcount: i32 = 0;
                    let mut malloced_0: bool = 0 as i32 != 0;
                    let mut fname_1: *mut i8 = adjust_namespace(
                        (*pc).d.name,
                        &mut malloced_0,
                    );
                    if (*pc).opcode as u32
                        == opcodeval::Op_indirect_func_call as i32 as u32
                    {
                        pre = b"@\0" as *const u8 as *const i8;
                    } else {
                        pre = b"\0" as *const u8 as *const i8;
                    }
                    pcount = (*pc.offset(1 as i32 as isize)).x.xl as i32;
                    if pcount > 0 as i32 {
                        tmp = pp_list(
                            pcount,
                            b"()\0" as *const u8 as *const i8,
                            b", \0" as *const u8 as *const i8,
                        );
                        str = pp_group3(pre, fname_1, tmp);
                        pma_free(tmp as *mut libc::c_void);
                    } else {
                        str = pp_group3(pre, fname_1, b"()\0" as *const u8 as *const i8);
                    }
                    if (*pc).opcode as u32
                        == opcodeval::Op_indirect_func_call as i32 as u32
                    {
                        t1 = pp_pop();
                        pp_free(t1);
                    }
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                    if malloced_0 {
                        pma_free(fname_1 as *mut libc::c_void);
                    }
                    current_block = 4014953649076014837;
                }
                55 | 54 | 67 | 59 => {
                    fprintf(
                        prof_fp,
                        b"%s\0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    pc = end_line(pc);
                    current_block = 4014953649076014837;
                }
                61 | 60 => {
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as i32) {
                        pp_parenthesize(t1);
                    }
                    if (*pc).source_line as i32 > 0 as i32 {
                        if *((*t1).sub.nodep.name).offset(0 as i32 as isize) as i32
                            != '\0' as i32
                        {
                            fprintf(
                                prof_fp,
                                b"%s %s\0" as *const u8 as *const i8,
                                op2str((*pc).opcode),
                                (*t1).sub.nodep.name,
                            );
                        } else {
                            fprintf(
                                prof_fp,
                                b"%s\0" as *const u8 as *const i8,
                                op2str((*pc).opcode),
                            );
                        }
                        pc = end_line(pc);
                    }
                    pp_free(t1);
                    current_block = 4014953649076014837;
                }
                86 => {
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s\0" as *const u8 as *const i8,
                        (*t1).sub.nodep.name,
                    );
                    if flags & 1 as i32 == 0 as i32 {
                        pc = end_line(pc);
                    }
                    pp_free(t1);
                    current_block = 4014953649076014837;
                }
                14 => {
                    ip1 = pc.offset(1 as i32 as isize);
                    pprint((*pc).nexti, (*ip1).d.di, 0 as i32);
                    pprint((*(*ip1).d.di).nexti, (*ip1).x.xi, 0 as i32);
                    t2 = pp_pop();
                    t1 = pp_pop();
                    str = pp_group3(
                        (*t1).sub.nodep.name,
                        b", \0" as *const u8 as *const i8,
                        (*t2).sub.nodep.name,
                    );
                    pp_free(t1);
                    pp_free(t2);
                    pp_push(
                        opcodeval::Op_line_range as i32,
                        str,
                        2 as i32,
                        (*pc).comment,
                    );
                    pc = (*ip1).x.xi;
                    current_block = 4014953649076014837;
                }
                115 => {
                    ip1 = pc.offset(1 as i32 as isize);
                    indent((*(*ip1).d.di).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    pprint((*pc).nexti, (*ip1).d.di, 0 as i32);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s) {\0" as *const u8 as *const i8,
                        (*t1).sub.nodep.name,
                    );
                    pp_free(t1);
                    (*ip1).d.di = end_line((*ip1).d.di);
                    indent_in();
                    pprint((*(*ip1).d.di).nexti, (*pc).x.xi, 0 as i32);
                    indent_out();
                    indent(0 as i32 as exec_count_t);
                    fprintf(prof_fp, b"}\0" as *const u8 as *const i8);
                    pc = end_line((*pc).x.xi);
                    current_block = 4014953649076014837;
                }
                112 => {
                    ip1 = pc.offset(1 as i32 as isize);
                    indent((*(*pc).nexti).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s {\0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    end_line((*pc).nexti);
                    skip_comment = 1 as i32 != 0;
                    indent_in();
                    pprint((*(*pc).nexti).nexti, (*ip1).d.di, 0 as i32);
                    indent_out();
                    pprint((*ip1).d.di, (*pc).x.xi, 0 as i32);
                    indent(0 as i32 as exec_count_t);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"} %s (%s)\0" as *const u8 as *const i8,
                        op2str(opcodeval::Op_K_while),
                        (*t1).sub.nodep.name,
                    );
                    if !((*pc).comment).is_null() {
                        fprintf(
                            prof_fp,
                            b"\t%s\0" as *const u8 as *const i8,
                            (*(*(*pc).comment).d.dn).sub.val.sp,
                        );
                    } else {
                        end_line((*pc).x.xi);
                        skip_comment = 1 as i32 != 0;
                    }
                    pp_free(t1);
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                113 => {
                    let mut comment1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut comment2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    if !((*pc).comment).is_null() {
                        comment1 = (*pc).comment;
                        (*pc).comment = 0 as *mut exp_instruction;
                        if !comment1.is_null() && !((*comment1).comment).is_null() {
                            comment2 = (*comment1).comment;
                            (*comment1).comment = 0 as *mut exp_instruction;
                        }
                        if comment2.is_null()
                            && (*(*comment1).d.dn).sub.val.comtype as u32
                                == commenttype::FOR_COMMENT as i32 as u32
                        {
                            comment2 = comment1;
                            (*(*comment2).d.dn).sub.val.comtype = commenttype::EOL_COMMENT;
                            comment1 = 0 as *mut INSTRUCTION;
                        }
                    }
                    ip1 = pc.offset(1 as i32 as isize);
                    indent((*(*ip1).x.xi).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    if (*(*pc).nexti).opcode as u32 == opcodeval::Op_no_op as i32 as u32
                        && (*ip1).d.di == (*pc).nexti
                        && (*(*pc).d.di).opcode as u32 == opcodeval::Op_jmp as i32 as u32
                        && comment1.is_null() && comment2.is_null()
                    {
                        fprintf(prof_fp, b";;\0" as *const u8 as *const i8);
                    } else {
                        pprint((*pc).nexti, (*ip1).d.di, 1 as i32);
                        fprintf(prof_fp, b"; \0" as *const u8 as *const i8);
                        if !comment1.is_null() {
                            print_comment(comment1, 0 as i32 as i64);
                            indent((*(*ip1).x.xi).d.ldl);
                            indent(1 as i32 as exec_count_t);
                        }
                        if (*(*ip1).d.di).opcode as u32
                            == opcodeval::Op_no_op as i32 as u32
                            && (*(*ip1).d.di).nexti == (*ip1).x.xi
                        {
                            fprintf(prof_fp, b"; \0" as *const u8 as *const i8);
                        } else {
                            pprint((*ip1).d.di, (*ip1).x.xi, 1 as i32);
                            t1 = pp_pop();
                            fprintf(
                                prof_fp,
                                b"%s; \0" as *const u8 as *const i8,
                                (*t1).sub.nodep.name,
                            );
                            pp_free(t1);
                        }
                        if !comment2.is_null() {
                            print_comment(comment2, 0 as i32 as i64);
                            indent((*(*ip1).x.xi).d.ldl);
                            indent(1 as i32 as exec_count_t);
                        }
                        pprint((*pc).d.di, (*pc).x.xi, 1 as i32);
                    }
                    fprintf(prof_fp, b") {\0" as *const u8 as *const i8);
                    end_line((*ip1).x.xi);
                    skip_comment = 1 as i32 != 0;
                    indent_in();
                    pprint((*(*ip1).x.xi).nexti, (*pc).d.di, 0 as i32);
                    indent_out();
                    indent(0 as i32 as exec_count_t);
                    fprintf(prof_fp, b"}\0" as *const u8 as *const i8);
                    end_line((*pc).x.xi);
                    skip_comment = 1 as i32 != 0;
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                114 => {
                    let mut array_1: *mut i8 = 0 as *mut i8;
                    let mut item: *const i8 = 0 as *const i8;
                    ip1 = pc.offset(1 as i32 as isize);
                    t1 = pp_pop();
                    array_1 = (*t1).sub.nodep.name;
                    m = (*(*ip1).d.di).x.xn;
                    if (*m).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
                        item = (*func_params.offset((*m).sub.nodep.l.ll as isize))
                            .sub
                            .nodep
                            .name;
                    } else {
                        item = (*m).sub.nodep.name;
                    }
                    indent((*(*ip1).x.xi).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s (%s%s%s) {\0" as *const u8 as *const i8,
                        op2str(opcodeval::Op_K_arrayfor),
                        item,
                        op2str(opcodeval::Op_in_array),
                        array_1,
                    );
                    end_line((*ip1).x.xi);
                    skip_comment = 1 as i32 != 0;
                    indent_in();
                    pp_free(t1);
                    pprint((*(*ip1).x.xi).nexti, (*pc).x.xi, 0 as i32);
                    indent_out();
                    indent(0 as i32 as exec_count_t);
                    fprintf(prof_fp, b"}\0" as *const u8 as *const i8);
                    end_line((*pc).x.xi);
                    skip_comment = 1 as i32 != 0;
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                116 => {
                    ip1 = pc.offset(1 as i32 as isize);
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    pprint((*pc).nexti, (*ip1).d.di, 0 as i32);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s) {\0" as *const u8 as *const i8,
                        (*t1).sub.nodep.name,
                    );
                    if !((*pc).comment).is_null() {
                        print_comment((*pc).comment, 0 as i32 as i64);
                    } else {
                        fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
                    }
                    pp_free(t1);
                    pprint((*ip1).d.di, (*ip1).x.xi, 0 as i32);
                    indent(0 as i32 as exec_count_t);
                    fprintf(prof_fp, b"}\n\0" as *const u8 as *const i8);
                    if !((*(*ip1).x.xi).comment).is_null() {
                        print_comment((*(*ip1).x.xi).comment, 0 as i32 as i64);
                    }
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                52 | 53 => {
                    indent((*(*pc).d.di).d.ldl);
                    if (*pc).opcode as u32 == opcodeval::Op_K_case as i32 as u32 {
                        t1 = pp_pop();
                        fprintf(
                            prof_fp,
                            b"%s %s:\0" as *const u8 as *const i8,
                            op2str((*pc).opcode),
                            (*t1).sub.nodep.name,
                        );
                        pp_free(t1);
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s:\0" as *const u8 as *const i8,
                            op2str((*pc).opcode),
                        );
                    }
                    indent_in();
                    if !((*pc).comment).is_null() {
                        if (*(*(*pc).comment).d.dn).sub.val.comtype as u32
                            == commenttype::EOL_COMMENT as i32 as u32
                        {
                            fprintf(
                                prof_fp,
                                b"\t%s\0" as *const u8 as *const i8,
                                (*(*(*pc).comment).d.dn).sub.val.sp,
                            );
                            if !((*(*pc).comment).comment).is_null() {
                                print_comment((*(*pc).comment).comment, indent_level);
                            }
                        } else {
                            fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
                            print_comment((*pc).comment, indent_level);
                        }
                    } else {
                        fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
                    }
                    pprint((*(*pc).d.di).nexti, (*(*pc).x.xi).nexti, 0 as i32);
                    indent_out();
                    current_block = 4014953649076014837;
                }
                117 => {
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    pprint((*pc).nexti, (*pc).d.di, 0 as i32);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s) {\0" as *const u8 as *const i8,
                        (*t1).sub.nodep.name,
                    );
                    pp_free(t1);
                    ip1 = (*pc).d.di;
                    if (*ip1).d.ldl > 0 as i32 as libc::c_ulonglong {
                        fprintf(
                            prof_fp,
                            b" # %llu\0" as *const u8 as *const i8,
                            (*ip1).d.ldl,
                        );
                    }
                    ip1 = end_line(ip1);
                    indent_in();
                    if !((*pc).comment).is_null() {
                        print_comment((*pc).comment, indent_level);
                    }
                    pprint((*ip1).nexti, (*pc).x.xi, 0 as i32);
                    indent_out();
                    pc = (*pc).x.xi;
                    if (*(*pc).nexti).opcode as u32 == opcodeval::Op_no_op as i32 as u32
                    {
                        indent(0 as i32 as exec_count_t);
                        fprintf(prof_fp, b"}\0" as *const u8 as *const i8);
                        if (*(*(*pc).nexti).nexti).opcode as u32
                            != opcodeval::Op_comment as i32 as u32
                            || (*(*(*(*pc).nexti).nexti).d.dn).sub.val.comtype as u32
                                == commenttype::BLOCK_COMMENT as i32 as u32
                        {
                            fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
                        }
                    }
                    flags &= !(2 as i32);
                    current_block = 4014953649076014837;
                }
                118 => {
                    fprintf(
                        prof_fp,
                        b"} %s \0" as *const u8 as *const i8,
                        op2str((*pc).opcode),
                    );
                    if (*(*(*pc).nexti).nexti).opcode as u32
                        == opcodeval::Op_K_if as i32 as u32
                        && (*pc).x.xi == (*(*(*(*pc).nexti).nexti).x.xi).d.di
                    {
                        pprint((*pc).nexti, (*pc).x.xi, 2 as i32);
                    } else {
                        fprintf(prof_fp, b"{\0" as *const u8 as *const i8);
                        end_line(pc);
                        skip_comment = 1 as i32 != 0;
                        indent_in();
                        if !((*pc).comment).is_null() {
                            print_comment((*pc).comment, indent_level);
                        }
                        pprint((*pc).nexti, (*pc).x.xi, 0 as i32);
                        indent_out();
                        indent(0 as i32 as exec_count_t);
                        fprintf(prof_fp, b"}\0" as *const u8 as *const i8);
                        end_line((*pc).x.xi);
                        skip_comment = 1 as i32 != 0;
                    }
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                120 => {
                    let mut f: *mut NODE = 0 as *mut NODE;
                    let mut t: *mut NODE = 0 as *mut NODE;
                    let mut cond: *mut NODE = 0 as *mut NODE;
                    let mut len_0: size_t = 0;
                    let mut qm_comment: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut colon_comment: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    qm_comment = (*pc).comment;
                    pprint((*pc).nexti, (*pc).d.di, 0 as i32);
                    ip1 = (*pc).d.di;
                    pprint((*ip1).nexti, (*pc).x.xi, 0 as i32);
                    ip1 = (*(*pc).x.xi).nexti;
                    pc = (*ip1).nexti;
                    colon_comment = (*pc).comment;
                    pprint((*pc).nexti, (*pc).x.xi, 0 as i32);
                    f = pp_pop();
                    t = pp_pop();
                    cond = pp_pop();
                    len_0 = ((*f).sub.nodep.reserved)
                        .wrapping_add((*t).sub.nodep.reserved)
                        .wrapping_add((*cond).sub.nodep.reserved)
                        .wrapping_add(12 as i32 as u64);
                    if qm_comment.is_null() && colon_comment.is_null() {
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const i8,
                            b"str\0" as *const u8 as *const i8,
                            b"profile.c\0" as *const u8 as *const i8,
                            1155 as i32,
                        ) as *mut i8;
                        sprintf(
                            str,
                            b"%s ? %s : %s\0" as *const u8 as *const i8,
                            (*cond).sub.nodep.name,
                            (*t).sub.nodep.name,
                            (*f).sub.nodep.name,
                        );
                    } else if !qm_comment.is_null() && !colon_comment.is_null() {
                        if (indent_level + 1 as i32 as i64) as u64 > tabs_len {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const i8,
                                1158 as i32,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        len_0 = (len_0 as u64)
                            .wrapping_add(
                                ((*(*qm_comment).d.dn).sub.val.slen)
                                    .wrapping_add((*(*colon_comment).d.dn).sub.val.slen)
                                    .wrapping_add(
                                        (2 as i32 as i64 * (indent_level + 1 as i32 as i64)) as u64,
                                    )
                                    .wrapping_add(3 as i32 as u64)
                                    .wrapping_add((*t).sub.nodep.reserved)
                                    .wrapping_add(6 as i32 as u64),
                            ) as size_t as size_t;
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const i8,
                            b"str\0" as *const u8 as *const i8,
                            b"profile.c\0" as *const u8 as *const i8,
                            1163 as i32,
                        ) as *mut i8;
                        sprintf(
                            str,
                            b"%s ? %s%.*s   %s : %s%.*s   %s\0" as *const u8
                                as *const i8,
                            (*cond).sub.nodep.name,
                            (*(*qm_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as i32 as i64) as i32,
                            tabs.as_ptr(),
                            (*t).sub.nodep.name,
                            (*(*colon_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as i32 as i64) as i32,
                            tabs.as_ptr(),
                            (*f).sub.nodep.name,
                        );
                    } else if !qm_comment.is_null() {
                        if (indent_level + 1 as i32 as i64) as u64 > tabs_len {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const i8,
                                1178 as i32,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        len_0 = (len_0 as u64)
                            .wrapping_add(
                                ((*(*qm_comment).d.dn).sub.val.slen)
                                    .wrapping_add(
                                        (1 as i32 as i64 * (indent_level + 1 as i32 as i64)) as u64,
                                    )
                                    .wrapping_add(3 as i32 as u64)
                                    .wrapping_add((*t).sub.nodep.reserved)
                                    .wrapping_add(3 as i32 as u64),
                            ) as size_t as size_t;
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const i8,
                            b"str\0" as *const u8 as *const i8,
                            b"profile.c\0" as *const u8 as *const i8,
                            1182 as i32,
                        ) as *mut i8;
                        sprintf(
                            str,
                            b"%s ? %s%.*s   %s : %s\0" as *const u8 as *const i8,
                            (*cond).sub.nodep.name,
                            (*(*qm_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as i32 as i64) as i32,
                            tabs.as_ptr(),
                            (*t).sub.nodep.name,
                            (*f).sub.nodep.name,
                        );
                    } else {
                        if (indent_level + 1 as i32 as i64) as u64 > tabs_len {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const i8,
                                1194 as i32,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        len_0 = (len_0 as u64)
                            .wrapping_add(
                                ((*(*colon_comment).d.dn).sub.val.slen)
                                    .wrapping_add(
                                        (1 as i32 as i64 * (indent_level + 1 as i32 as i64)) as u64,
                                    )
                                    .wrapping_add(3 as i32 as u64)
                                    .wrapping_add((*t).sub.nodep.reserved)
                                    .wrapping_add(3 as i32 as u64),
                            ) as size_t as size_t;
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const i8,
                            b"str\0" as *const u8 as *const i8,
                            b"profile.c\0" as *const u8 as *const i8,
                            1198 as i32,
                        ) as *mut i8;
                        sprintf(
                            str,
                            b"%s ? %s : %s%.*s   %s\0" as *const u8 as *const i8,
                            (*cond).sub.nodep.name,
                            (*t).sub.nodep.name,
                            (*(*colon_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as i32 as i64) as i32,
                            tabs.as_ptr(),
                            (*f).sub.nodep.name,
                        );
                    }
                    pp_free(cond);
                    pp_free(t);
                    pp_free(f);
                    pp_push(opcodeval::Op_cond_exp as i32, str, 2 as i32, (*pc).comment);
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                103 => {
                    if flags == 0 as i32 {
                        indent((*pc).d.ldl);
                    }
                    current_block = 4014953649076014837;
                }
                102 => {
                    print_comment(pc, 0 as i32 as i64);
                    current_block = 4014953649076014837;
                }
                107 | 95 | 96 | 97 | 98 | 92 | 93 | 94 | 91 | 90 | 105 | 87 | 89 | 88
                | 85 | 39 | 41 | 15 | 99 | 100 | 111 => {
                    current_block = 4014953649076014837;
                }
                _ => {
                    r_fatal(
                        b"internal error: file %s, line %d: unexpected opcode %s\0"
                            as *const u8 as *const i8,
                        b"profile.c\0" as *const u8 as *const i8,
                        1232 as i32,
                        opcode2str((*pc).opcode),
                    );
                    current_block = 4014953649076014837;
                }
            }
            match current_block {
                17991450254610602456 => {
                    m = (*pc).d.dn;
                    match (*m).type_0 as u32 {
                        8 => {
                            pp_push(
                                (*pc).opcode as i32,
                                (*func_params.offset((*m).sub.nodep.l.ll as isize))
                                    .sub
                                    .nodep
                                    .name,
                                1 as i32,
                                (*pc).comment,
                            );
                        }
                        4 | 6 | 5 => {
                            if !((*m).sub.nodep.name).is_null() {
                                let mut malloced: bool = 0 as i32 != 0;
                                let mut name: *mut i8 = adjust_namespace(
                                    (*m).sub.nodep.name,
                                    &mut malloced,
                                );
                                pp_push(
                                    (*pc).opcode as i32,
                                    name,
                                    if malloced as i32 != 0 { 2 as i32 } else { 1 as i32 },
                                    (*pc).comment,
                                );
                            } else {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"profile.c\0" as *const u8 as *const i8,
                                    366 as i32,
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
                                        b"internal error: %s with null vname\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                    nodetype2str((*m).type_0),
                                );
                            }
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: got unexpected type %s\0"
                                    as *const u8 as *const i8,
                                b"profile.c\0" as *const u8 as *const i8,
                                371 as i32,
                                nodetype2str((*m).type_0),
                            );
                        }
                    }
                    match (*pc).opcode as u32 {
                        27 => {
                            current_block = 9888928211123223282;
                            match current_block {
                                9888928211123223282 => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const i8,
                                        (*t2).sub.nodep.name,
                                        op2str((*pc).opcode),
                                        (*t1).sub.nodep.name,
                                    );
                                }
                                73983378755429342 => {
                                    t1 = pp_pop();
                                    tmp = pp_list(
                                        (*pc).x.xl as i32,
                                        op2str(opcodeval::Op_subscript),
                                        b", \0" as *const u8 as *const i8,
                                    );
                                    t2 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s%s\0" as *const u8 as *const i8,
                                        (*t1).sub.nodep.name,
                                        tmp,
                                        op2str((*pc).opcode),
                                        (*t2).sub.nodep.name,
                                    );
                                    pma_free(tmp as *mut libc::c_void);
                                }
                                _ => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    tmp = pp_group3(
                                        (*t2).sub.nodep.name,
                                        op2str(opcodeval::Op_concat),
                                        (*t1).sub.nodep.name,
                                    );
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const i8,
                                        (*t2).sub.nodep.name,
                                        op2str(opcodeval::Op_assign),
                                        tmp,
                                    );
                                    pma_free(tmp as *mut libc::c_void);
                                }
                            }
                            pp_free(t2);
                            pp_free(t1);
                            if flags & 1 as i32 == 0 as i32 {
                                pc = end_line(pc);
                            }
                        }
                        28 => {
                            current_block = 73983378755429342;
                            match current_block {
                                9888928211123223282 => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const i8,
                                        (*t2).sub.nodep.name,
                                        op2str((*pc).opcode),
                                        (*t1).sub.nodep.name,
                                    );
                                }
                                73983378755429342 => {
                                    t1 = pp_pop();
                                    tmp = pp_list(
                                        (*pc).x.xl as i32,
                                        op2str(opcodeval::Op_subscript),
                                        b", \0" as *const u8 as *const i8,
                                    );
                                    t2 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s%s\0" as *const u8 as *const i8,
                                        (*t1).sub.nodep.name,
                                        tmp,
                                        op2str((*pc).opcode),
                                        (*t2).sub.nodep.name,
                                    );
                                    pma_free(tmp as *mut libc::c_void);
                                }
                                _ => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    tmp = pp_group3(
                                        (*t2).sub.nodep.name,
                                        op2str(opcodeval::Op_concat),
                                        (*t1).sub.nodep.name,
                                    );
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const i8,
                                        (*t2).sub.nodep.name,
                                        op2str(opcodeval::Op_assign),
                                        tmp,
                                    );
                                    pma_free(tmp as *mut libc::c_void);
                                }
                            }
                            pp_free(t2);
                            pp_free(t1);
                            if flags & 1 as i32 == 0 as i32 {
                                pc = end_line(pc);
                            }
                        }
                        37 => {
                            current_block = 14538565572863007052;
                            match current_block {
                                9888928211123223282 => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const i8,
                                        (*t2).sub.nodep.name,
                                        op2str((*pc).opcode),
                                        (*t1).sub.nodep.name,
                                    );
                                }
                                73983378755429342 => {
                                    t1 = pp_pop();
                                    tmp = pp_list(
                                        (*pc).x.xl as i32,
                                        op2str(opcodeval::Op_subscript),
                                        b", \0" as *const u8 as *const i8,
                                    );
                                    t2 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s%s\0" as *const u8 as *const i8,
                                        (*t1).sub.nodep.name,
                                        tmp,
                                        op2str((*pc).opcode),
                                        (*t2).sub.nodep.name,
                                    );
                                    pma_free(tmp as *mut libc::c_void);
                                }
                                _ => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    tmp = pp_group3(
                                        (*t2).sub.nodep.name,
                                        op2str(opcodeval::Op_concat),
                                        (*t1).sub.nodep.name,
                                    );
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const i8,
                                        (*t2).sub.nodep.name,
                                        op2str(opcodeval::Op_assign),
                                        tmp,
                                    );
                                    pma_free(tmp as *mut libc::c_void);
                                }
                            }
                            pp_free(t2);
                            pp_free(t1);
                            if flags & 1 as i32 == 0 as i32 {
                                pc = end_line(pc);
                            }
                        }
                        _ => {}
                    }
                }
                1979616052774745799 => {
                    if (*(*pc).d.dn).type_0 as u32 == nodevals::Node_regex as i32 as u32
                    {
                        let mut re: *mut NODE = (*(*pc).d.dn).sub.nodep.x.extra;
                        str = pp_string(
                            (*re).sub.val.sp,
                            (*re).sub.val.slen,
                            '/' as i32,
                        );
                    } else {
                        str = pp_typed_regex(
                            (*(*pc).d.dn).sub.val.sp,
                            (*(*pc).d.dn).sub.val.slen,
                            '/' as i32,
                        );
                    }
                    pp_push((*pc).opcode as i32, str, 2 as i32, (*pc).comment);
                }
                _ => {}
            }
            if pc == endp {
                break;
            }
        }
        pc = (*pc).nexti;
    }
}
unsafe extern "C" fn end_line(mut ip: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut ret: *mut INSTRUCTION = ip;
    if (*(*ip).nexti).opcode as u32 == opcodeval::Op_comment as i32 as u32
        && (*(*(*ip).nexti).d.dn).sub.val.comtype as u32
            == commenttype::EOL_COMMENT as i32 as u32
    {
        fprintf(prof_fp, b"\t\0" as *const u8 as *const i8);
        print_comment((*ip).nexti, -(1 as i32) as i64);
        ret = (*ip).nexti;
    } else {
        fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn pp_string_fp(
    mut print_func: Func_print,
    mut fp: *mut FILE,
    mut in_str: *const i8,
    mut len: size_t,
    mut delim: i32,
    mut breaklines: bool,
) {
    let mut s: *mut i8 = pp_string(in_str, len, delim);
    let mut count: i32 = 0;
    let mut slen: size_t = 0;
    let mut str: *const i8 = s as *const i8;
    slen = strlen(str);
    count = 0 as i32;
    while slen > 0 as i32 as u64 {
        print_func
            .expect(
                "non-null function pointer",
            )(fp, b"%c\0" as *const u8 as *const i8, *str as i32);
        count += 1;
        if count >= 70 as i32 && breaklines as i32 != 0 {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"%c\n%c\0" as *const u8 as *const i8, delim, delim);
            count = 0 as i32;
        }
        slen = slen.wrapping_sub(1);
        slen;
        str = str.offset(1);
        str;
    }
    pma_free(s as *mut libc::c_void);
}
unsafe extern "C" fn just_dump(mut signum: i32) {
    extern "C" {
        static mut code_block: *mut INSTRUCTION;
    }
    dump_prog(code_block);
    dump_funcs();
    dump_fcall_stack(prof_fp);
    fflush(prof_fp);
    signal(signum, Some(just_dump as unsafe extern "C" fn(i32) -> ()));
}
unsafe extern "C" fn dump_and_exit(mut signum: i32) -> ! {
    just_dump(signum);
    final_exit(1 as i32);
}
unsafe extern "C" fn print_lib_list(mut prof_fp_0: *mut FILE) {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    static mut printed_header: bool = 0 as i32 != 0;
    let mut indent_0: *const i8 = b"\0" as *const u8 as *const i8;
    let mut found: bool = 0 as i32 != 0;
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        indent_0 = b"\t\0" as *const u8 as *const i8;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as u32 == srctype::SRC_EXTLIB as i32 as u32 {
            if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0
                && !printed_header
            {
                printed_header = 1 as i32 != 0;
                fprintf(
                    prof_fp_0,
                    dcgettext(
                        0 as *const i8,
                        b"%s# Loaded extensions (-l and/or @load)\n\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    indent_0,
                );
            }
            found = 1 as i32 != 0;
            fprintf(
                prof_fp_0,
                b"%s@load \"%s\"\0" as *const u8 as *const i8,
                indent_0,
                (*s).src,
            );
            if !((*s).comment).is_null() {
                fprintf(prof_fp_0, b"\t\0" as *const u8 as *const i8);
                print_comment((*s).comment, indent_level + 1 as i32 as i64);
            } else {
                fprintf(prof_fp_0, b"\n\0" as *const u8 as *const i8);
            }
        }
        s = (*s).next;
    }
    if found {
        fprintf(prof_fp_0, b"\n\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn print_include_list(mut prof_fp_0: *mut FILE) {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    static mut printed_header: bool = 0 as i32 != 0;
    let mut found: bool = 0 as i32 != 0;
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        return;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as u32 == srctype::SRC_INC as i32 as u32 {
            if !printed_header {
                printed_header = 1 as i32 != 0;
                fprintf(
                    prof_fp_0,
                    dcgettext(
                        0 as *const i8,
                        b"\n# Included files (-i and/or @include)\n\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            found = 1 as i32 != 0;
            fprintf(
                prof_fp_0,
                b"# @include \"%s\"\0" as *const u8 as *const i8,
                (*s).src,
            );
            if !((*s).comment).is_null() {
                fprintf(prof_fp_0, b"\t\0" as *const u8 as *const i8);
                print_comment((*s).comment, indent_level + 1 as i32 as i64);
            } else {
                fprintf(prof_fp_0, b"\n\0" as *const u8 as *const i8);
            }
        }
        s = (*s).next;
    }
    if found {
        fprintf(prof_fp_0, b"\n\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn print_comment(mut pc: *mut INSTRUCTION, mut in_0: i64) {
    let mut text: *mut i8 = 0 as *mut i8;
    let mut count: size_t = 0;
    let mut after_newline: bool = 0 as i32 != 0;
    count = (*(*pc).d.dn).sub.val.slen;
    text = (*(*pc).d.dn).sub.val.sp;
    if in_0 >= 0 as i32 as i64 {
        indent(in_0 as exec_count_t);
    }
    while count > 0 as i32 as u64 {
        if after_newline {
            indent(in_0 as exec_count_t);
            after_newline = 0 as i32 != 0;
        }
        _IO_putc(*text as i32, prof_fp);
        after_newline = *text as i32 == '\n' as i32;
        count = count.wrapping_sub(1);
        count;
        text = text.offset(1);
        text;
    }
    if !((*pc).comment).is_null() {
        if (*(*pc).d.dn).sub.val.comtype as u32 == commenttype::EOL_COMMENT as i32 as u32
        {
            in_0 += 1;
            in_0;
        }
        print_comment((*pc).comment, in_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dump_prog(mut code: *mut INSTRUCTION) {
    let mut now: time_t = 0;
    time(&mut now);
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        fprintf(
            prof_fp,
            dcgettext(
                0 as *const i8,
                b"\t# gawk profile, created %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            ctime(&mut now),
        );
    }
    print_lib_list(prof_fp);
    pprint(code, 0 as *mut INSTRUCTION, 0 as i32);
    print_include_list(prof_fp);
}
unsafe extern "C" fn prec_level(mut type_0: i32) -> i32 {
    match type_0 {
        82 | 81 | 80 | 75 | 78 | 79 | 49 | 16 | 83 | 73 | 64 | 69 => {
            return 16 as i32;
        }
        24 | 84 => return 15 as i32,
        18 | 19 | 20 | 21 => return 14 as i32,
        11 | 12 => return 13 as i32,
        22 | 23 | 25 => return 12 as i32,
        1 | 2 | 3 | 4 | 5 | 6 => return 11 as i32,
        7 | 8 | 9 | 10 => return 10 as i32,
        13 | 37 => return 9 as i32,
        42 | 43 | 45 | 44 | 46 | 47 => return 8 as i32,
        48 | 50 => return 7 as i32,
        66 | 65 => return 6 as i32,
        72 => return 5 as i32,
        38 => return 4 as i32,
        40 => return 3 as i32,
        120 => return 2 as i32,
        26 | 31 | 32 | 33 | 34 | 35 | 36 => return 1 as i32,
        _ => return 0 as i32,
    };
}
unsafe extern "C" fn is_scalar(mut type_0: i32) -> bool {
    match type_0 {
        82 | 81 | 80 | 75 | 78 | 79 | 16 | 83 | 73 | 69 | 24 | 84 | 18 | 19 | 20 | 21
        | 22 | 23 | 25 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
unsafe extern "C" fn is_binary(mut type_0: i32) -> bool {
    match type_0 {
        47 | 46 | 45 | 44 | 43 | 42 | 11 | 1 | 3 | 5 | 7 | 9 | 12 | 2 | 4 | 6 | 8 | 10
        | 13 | 37 | 48 | 50 | 26 | 31 | 32 | 33 | 34 | 35 | 36 | 120 | 38 | 40 | 72 | 65
        | 66 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
unsafe extern "C" fn pp_parenthesize(mut sp: *mut NODE) {
    let mut p: *mut i8 = (*sp).sub.nodep.name;
    let mut len: size_t = (*sp).sub.nodep.reserved;
    if *p.offset(0 as i32 as isize) as i32 == '(' as i32 {
        return;
    }
    p = emalloc_real(
        len.wrapping_add(3 as i32 as u64),
        b"pp_parenthesize\0" as *const u8 as *const i8,
        b"p\0" as *const u8 as *const i8,
        b"profile.c\0" as *const u8 as *const i8,
        1619 as i32,
    ) as *mut i8;
    *p = '(' as i32 as i8;
    memcpy(
        p.offset(1 as i32 as isize) as *mut libc::c_void,
        (*sp).sub.nodep.name as *const libc::c_void,
        len,
    );
    *p.offset(len.wrapping_add(1 as i32 as u64) as isize) = ')' as i32 as i8;
    *p.offset(len.wrapping_add(2 as i32 as u64) as isize) = '\0' as i32 as i8;
    if (*sp).flags as u32 & 2 as i32 as u32 != 0 as i32 as u32 {
        pma_free((*sp).sub.nodep.name as *mut libc::c_void);
    }
    (*sp).sub.nodep.name = p;
    (*sp).sub.nodep.reserved = ((*sp).sub.nodep.reserved as u64)
        .wrapping_add(2 as i32 as u64) as size_t as size_t;
    (*sp).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*sp).flags as u32 | 2 as i32 as u32);
}
unsafe extern "C" fn parenthesize(
    mut type_0: i32,
    mut left: *mut NODE,
    mut right: *mut NODE,
) {
    let mut rprec: i32 = prec_level((*right).type_0 as i32);
    let mut lprec: i32 = prec_level((*left).type_0 as i32);
    let mut prec: i32 = prec_level(type_0);
    if lprec < prec {
        pp_parenthesize(left);
    }
    if rprec < prec {
        pp_parenthesize(right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pp_string(
    mut in_str: *const i8,
    mut len: size_t,
    mut delim: i32,
) -> *mut i8 {
    return pp_string_or_typed_regex(in_str, len, delim, 0 as i32 != 0);
}
unsafe extern "C" fn pp_typed_regex(
    mut in_str: *const i8,
    mut len: size_t,
    mut delim: i32,
) -> *mut i8 {
    return pp_string_or_typed_regex(in_str, len, delim, 1 as i32 != 0);
}
unsafe extern "C" fn pp_string_or_typed_regex(
    mut in_str: *const i8,
    mut len: size_t,
    mut delim: i32,
    mut typed_regex: bool,
) -> *mut i8 {
    static mut str_escapes: [i8; 9] = unsafe {
        *::core::mem::transmute::<&[u8; 9], &mut [i8; 9]>(b"\x07\x08\x0C\n\r\t\x0B\\\0")
    };
    static mut str_printables: [i8; 9] = unsafe {
        *::core::mem::transmute::<&[u8; 9], &mut [i8; 9]>(b"abfnrtv\\\0")
    };
    static mut re_escapes: [i8; 8] = unsafe {
        *::core::mem::transmute::<&[u8; 8], &mut [i8; 8]>(b"\x07\x08\x0C\n\r\t\x0B\0")
    };
    static mut re_printables: [i8; 8] = unsafe {
        *::core::mem::transmute::<&[u8; 8], &mut [i8; 8]>(b"abfnrtv\0")
    };
    let mut escapes: *mut i8 = 0 as *mut i8;
    let mut printables: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut str: *const u8 = in_str as *const u8;
    let mut ofre: size_t = 0;
    let mut osiz: size_t = 0;
    let mut obuf: *mut i8 = 0 as *mut i8;
    let mut obufout: *mut i8 = 0 as *mut i8;
    if delim == '/' as i32 {
        escapes = re_escapes.as_mut_ptr();
        printables = re_printables.as_mut_ptr();
    } else {
        escapes = str_escapes.as_mut_ptr();
        printables = str_printables.as_mut_ptr();
    }
    osiz = len
        .wrapping_add(3 as i32 as u64)
        .wrapping_add(1 as i32 as u64)
        .wrapping_add((typed_regex as i32 == 1 as i32) as i32 as u64);
    obuf = emalloc_real(
        osiz,
        b"pp_string\0" as *const u8 as *const i8,
        b"obuf\0" as *const u8 as *const i8,
        b"profile.c\0" as *const u8 as *const i8,
        1700 as i32,
    ) as *mut i8;
    obufout = obuf;
    ofre = osiz.wrapping_sub(1 as i32 as u64);
    if typed_regex {
        let fresh3 = obufout;
        obufout = obufout.offset(1);
        *fresh3 = '@' as i32 as i8;
    }
    let fresh4 = obufout;
    obufout = obufout.offset(1);
    *fresh4 = delim as i8;
    while len > 0 as i32 as u64 {
        if 2 as i32 as u64 > ofre {
            let mut olen: i64 = obufout.offset_from(obuf) as i64;
            obuf = erealloc_real(
                obuf as *mut libc::c_void,
                osiz.wrapping_mul(2 as i32 as u64),
                b"pp_string\0" as *const u8 as *const i8,
                b"obuf\0" as *const u8 as *const i8,
                b"profile.c\0" as *const u8 as *const i8,
                1709 as i32,
            ) as *mut i8;
            obufout = obuf.offset(olen as isize);
            ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
            osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
        }
        ofre = (ofre as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
        if delim != '/' as i32 && *str as i32 == delim {
            let fresh5 = obufout;
            obufout = obufout.offset(1);
            *fresh5 = '\\' as i32 as i8;
            let fresh6 = obufout;
            obufout = obufout.offset(1);
            *fresh6 = delim as i8;
        } else if *str as i32 == '\0' as i32 {
            let fresh7 = obufout;
            obufout = obufout.offset(1);
            *fresh7 = '\\' as i32 as i8;
            let fresh8 = obufout;
            obufout = obufout.offset(1);
            *fresh8 = '0' as i32 as i8;
            if 2 as i32 as u64 > ofre {
                let mut olen_0: i64 = obufout.offset_from(obuf) as i64;
                obuf = erealloc_real(
                    obuf as *mut libc::c_void,
                    osiz.wrapping_mul(2 as i32 as u64),
                    b"pp_string\0" as *const u8 as *const i8,
                    b"obuf\0" as *const u8 as *const i8,
                    b"profile.c\0" as *const u8 as *const i8,
                    1716 as i32,
                ) as *mut i8;
                obufout = obuf.offset(olen_0 as isize);
                ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
            }
            ofre = (ofre as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
            let fresh9 = obufout;
            obufout = obufout.offset(1);
            *fresh9 = '0' as i32 as i8;
            let fresh10 = obufout;
            obufout = obufout.offset(1);
            *fresh10 = '0' as i32 as i8;
        } else {
            cp = strchr(escapes, *str as i32);
            if !cp.is_null() {
                i = cp.offset_from(escapes) as i64 as i32;
                let fresh11 = obufout;
                obufout = obufout.offset(1);
                *fresh11 = '\\' as i32 as i8;
                let fresh12 = obufout;
                obufout = obufout.offset(1);
                *fresh12 = *printables.offset(i as isize);
            } else if *str as i32 & !(0x7f as i32) == 0 as i32
                && *(*__ctype_b_loc()).offset(*str as i32 as isize) as i32
                    & C2RustUnnamed::_ISprint as i32 as libc::c_ushort as i32 != 0
            {
                let fresh13 = obufout;
                obufout = obufout.offset(1);
                *fresh13 = *str as i8;
                ofre = (ofre as u64).wrapping_add(1 as i32 as u64) as size_t as size_t;
            } else {
                let mut len_0: size_t = 0;
                if 8 as i32 as u64 > ofre {
                    let mut olen_1: i64 = obufout.offset_from(obuf) as i64;
                    obuf = erealloc_real(
                        obuf as *mut libc::c_void,
                        osiz.wrapping_mul(2 as i32 as u64),
                        b"pp_string\0" as *const u8 as *const i8,
                        b"obuf\0" as *const u8 as *const i8,
                        b"profile.c\0" as *const u8 as *const i8,
                        1730 as i32,
                    ) as *mut i8;
                    obufout = obuf.offset(olen_1 as isize);
                    ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
                    osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t
                        as size_t;
                }
                ofre = (ofre as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
                sprintf(
                    obufout,
                    b"\\%03o\0" as *const u8 as *const i8,
                    *str as i32 & 0xff as i32,
                );
                len_0 = strlen(obufout);
                ofre = (ofre as u64).wrapping_add((10 as i32 as u64).wrapping_sub(len_0))
                    as size_t as size_t;
                obufout = obufout.offset(len_0 as isize);
            }
        }
        len = len.wrapping_sub(1);
        len;
        str = str.offset(1);
        str;
    }
    if 2 as i32 as u64 > ofre {
        let mut olen_2: i64 = obufout.offset_from(obuf) as i64;
        obuf = erealloc_real(
            obuf as *mut libc::c_void,
            osiz.wrapping_mul(2 as i32 as u64),
            b"pp_string\0" as *const u8 as *const i8,
            b"obuf\0" as *const u8 as *const i8,
            b"profile.c\0" as *const u8 as *const i8,
            1738 as i32,
        ) as *mut i8;
        obufout = obuf.offset(olen_2 as isize);
        ofre = (ofre as u64).wrapping_add(osiz) as size_t as size_t;
        osiz = (osiz as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
    }
    ofre = (ofre as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
    let fresh14 = obufout;
    obufout = obufout.offset(1);
    *fresh14 = delim as i8;
    *obufout = '\0' as i32 as i8;
    return obuf;
}
#[no_mangle]
pub unsafe extern "C" fn pp_number(mut n: *mut NODE) -> *mut i8 {
    let mut str: *mut i8 = 0 as *mut i8;
    str = emalloc_real(
        ((*n).sub.val.slen).wrapping_add(1 as i32 as u64),
        b"pp_number\0" as *const u8 as *const i8,
        b"str\0" as *const u8 as *const i8,
        b"profile.c\0" as *const u8 as *const i8,
        1753 as i32,
    ) as *mut i8;
    strcpy(str, (*n).sub.val.sp);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pp_node(mut n: *mut NODE) -> *mut i8 {
    if (*n).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32 {
        return pp_number(n);
    }
    return pp_string((*n).sub.val.sp, (*n).sub.val.slen, '"' as i32);
}
static mut pp_args: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
static mut npp_args: i32 = 0;
unsafe extern "C" fn pp_list(
    mut nargs: i32,
    mut paren: *const i8,
    mut delim: *const i8,
) -> *mut i8 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut delimlen: size_t = 0;
    let mut i: i32 = 0;
    let mut comment: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if pp_args.is_null() {
        npp_args = nargs;
        pp_args = emalloc_real(
            ((nargs + 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"pp_list\0" as *const u8 as *const i8,
            b"pp_args\0" as *const u8 as *const i8,
            b"profile.c\0" as *const u8 as *const i8,
            1785 as i32,
        ) as *mut *mut NODE;
    } else if nargs > npp_args {
        npp_args = nargs;
        pp_args = erealloc_real(
            pp_args as *mut libc::c_void,
            ((nargs + 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"pp_list\0" as *const u8 as *const i8,
            b"pp_args\0" as *const u8 as *const i8,
            b"profile.c\0" as *const u8 as *const i8,
            1788 as i32,
        ) as *mut *mut NODE;
    }
    delimlen = strlen(delim);
    if nargs == 0 as i32 {
        len = 2 as i32 as size_t;
    } else {
        len = delimlen.wrapping_neg();
        i = 1 as i32;
        while i <= nargs {
            let ref mut fresh15 = *pp_args.offset(i as isize);
            *fresh15 = pp_pop();
            r = *fresh15;
            len = (len as u64)
                .wrapping_add(((*r).sub.nodep.reserved).wrapping_add(delimlen)) as size_t
                as size_t;
            if !((*r).sub.nodep.x.cmnt).is_null() {
                comment = (*r).sub.nodep.x.cmnt as *mut INSTRUCTION;
                len = (len as u64)
                    .wrapping_add(
                        ((*(*comment).d.dn).sub.val.slen)
                            .wrapping_add(indent_level as u64)
                            .wrapping_add(1 as i32 as u64),
                    ) as size_t as size_t;
            }
            i += 1;
            i;
        }
        if !paren.is_null() {
            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
        }
    }
    comment = 0 as *mut INSTRUCTION;
    str = emalloc_real(
        len.wrapping_add(1 as i32 as u64),
        b"pp_list\0" as *const u8 as *const i8,
        b"str\0" as *const u8 as *const i8,
        b"profile.c\0" as *const u8 as *const i8,
        1811 as i32,
    ) as *mut i8;
    s = str;
    if !paren.is_null() {
        let fresh16 = s;
        s = s.offset(1);
        *fresh16 = *paren.offset(0 as i32 as isize);
    }
    i = nargs;
    while i > 0 as i32 {
        r = *pp_args.offset(i as isize);
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
        if i > 1 as i32 && delimlen > 0 as i32 as u64 {
            memcpy(s as *mut libc::c_void, delim as *const libc::c_void, delimlen);
            s = s.offset(delimlen as isize);
        }
        if !((*r).sub.nodep.x.cmnt).is_null() {
            if (indent_level + 1 as i32 as i64) as u64 > tabs_len {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"profile.c\0" as *const u8 as *const i8, 1830 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"Program indentation level too deep. Consider refactoring your code\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            comment = (*r).sub.nodep.x.cmnt as *mut INSTRUCTION;
            memcpy(
                s as *mut libc::c_void,
                (*(*comment).d.dn).sub.val.sp as *const libc::c_void,
                (*(*comment).d.dn).sub.val.slen,
            );
            s = s.offset((*(*comment).d.dn).sub.val.slen as isize);
            memcpy(
                s as *mut libc::c_void,
                tabs.as_ptr() as *const libc::c_void,
                (indent_level + 1 as i32 as i64) as u64,
            );
            s = s.offset((indent_level + 1 as i32 as i64) as isize);
        }
        pp_free(r);
        i -= 1;
        i;
    }
    if !paren.is_null() {
        let fresh17 = s;
        s = s.offset(1);
        *fresh17 = *paren.offset(1 as i32 as isize);
    }
    *s = '\0' as i32 as i8;
    return str;
}
unsafe extern "C" fn is_unary_minus(mut str: *const i8) -> bool {
    return *str.offset(0 as i32 as isize) as i32 == '-' as i32
        && *str.offset(1 as i32 as isize) as i32 != '-' as i32;
}
unsafe extern "C" fn pp_concat(mut nargs: i32) -> *mut i8 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    static mut delimlen: size_t = 1 as i32 as size_t;
    let mut i: i32 = 0;
    let mut pl_l: i32 = 0;
    let mut pl_r: i32 = 0;
    if pp_args.is_null() {
        npp_args = nargs;
        pp_args = emalloc_real(
            ((nargs + 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"pp_concat\0" as *const u8 as *const i8,
            b"pp_args\0" as *const u8 as *const i8,
            b"profile.c\0" as *const u8 as *const i8,
            1868 as i32,
        ) as *mut *mut NODE;
    } else if nargs > npp_args {
        npp_args = nargs;
        pp_args = erealloc_real(
            pp_args as *mut libc::c_void,
            ((nargs + 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"pp_concat\0" as *const u8 as *const i8,
            b"pp_args\0" as *const u8 as *const i8,
            b"profile.c\0" as *const u8 as *const i8,
            1871 as i32,
        ) as *mut *mut NODE;
    }
    len = delimlen.wrapping_neg();
    i = nargs;
    while i >= 1 as i32 {
        let ref mut fresh18 = *pp_args.offset(i as isize);
        *fresh18 = pp_pop();
        r = *fresh18;
        len = (len as u64)
            .wrapping_add(
                ((*r).sub.nodep.reserved)
                    .wrapping_add(delimlen)
                    .wrapping_add(2 as i32 as u64),
            ) as size_t as size_t;
        i -= 1;
        i;
    }
    str = emalloc_real(
        len.wrapping_add(1 as i32 as u64),
        b"pp_concat\0" as *const u8 as *const i8,
        b"str\0" as *const u8 as *const i8,
        b"profile.c\0" as *const u8 as *const i8,
        1885 as i32,
    ) as *mut i8;
    s = str;
    i = 1 as i32;
    while i < nargs {
        r = *pp_args.offset(i as isize);
        if *((*r).sub.nodep.name).offset(0 as i32 as isize) as i32 != '(' as i32 {
            pl_l = prec_level((**pp_args.offset(i as isize)).type_0 as i32);
            pl_r = prec_level((**pp_args.offset((i + 1 as i32) as isize)).type_0 as i32);
            if i >= 2 as i32 && is_unary_minus((*r).sub.nodep.name) as i32 != 0 {
                let fresh19 = s;
                s = s.offset(1);
                *fresh19 = '(' as i32 as i8;
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
                let fresh20 = s;
                s = s.offset(1);
                *fresh20 = ')' as i32 as i8;
            } else if is_scalar((**pp_args.offset(i as isize)).type_0 as i32) as i32 != 0
                && is_scalar((**pp_args.offset((i + 1 as i32) as isize)).type_0 as i32)
                    as i32 != 0
            {
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
            } else if pl_l <= pl_r
                || is_scalar((**pp_args.offset((i + 1 as i32) as isize)).type_0 as i32)
                    as i32 != 0
            {
                let fresh21 = s;
                s = s.offset(1);
                *fresh21 = '(' as i32 as i8;
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
                let fresh22 = s;
                s = s.offset(1);
                *fresh22 = ')' as i32 as i8;
            } else {
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
            }
        } else {
            memcpy(
                s as *mut libc::c_void,
                (*r).sub.nodep.name as *const libc::c_void,
                (*r).sub.nodep.reserved,
            );
            s = s.offset((*r).sub.nodep.reserved as isize);
        }
        if i < nargs {
            let fresh23 = s;
            s = s.offset(1);
            *fresh23 = ' ' as i32 as i8;
        }
        i += 1;
        i;
    }
    pl_l = prec_level((**pp_args.offset((nargs - 1 as i32) as isize)).type_0 as i32);
    pl_r = prec_level((**pp_args.offset(nargs as isize)).type_0 as i32);
    r = *pp_args.offset(nargs as isize);
    if *((*r).sub.nodep.name).offset(0 as i32 as isize) as i32 == '(' as i32 {
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
    } else if is_unary_minus((*r).sub.nodep.name) as i32 != 0
        || pl_l >= pl_r && !is_scalar((**pp_args.offset(nargs as isize)).type_0 as i32)
    {
        let fresh24 = s;
        s = s.offset(1);
        *fresh24 = '(' as i32 as i8;
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
        let fresh25 = s;
        s = s.offset(1);
        *fresh25 = ')' as i32 as i8;
    } else {
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
    }
    i = nargs;
    while i >= 1 as i32 {
        pp_free(*pp_args.offset(i as isize));
        i -= 1;
        i;
    }
    *s = '\0' as i32 as i8;
    return str;
}
unsafe extern "C" fn pp_group3(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s3: *const i8,
) -> *mut i8 {
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut len3: size_t = 0;
    let mut l: size_t = 0;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    len1 = strlen(s1);
    len2 = strlen(s2);
    len3 = strlen(s3);
    l = len1.wrapping_add(len2).wrapping_add(len3).wrapping_add(1 as i32 as u64);
    str = emalloc_real(
        l,
        b"pp_group3\0" as *const u8 as *const i8,
        b"str\0" as *const u8 as *const i8,
        b"profile.c\0" as *const u8 as *const i8,
        1959 as i32,
    ) as *mut i8;
    s = str;
    if len1 > 0 as i32 as u64 {
        memcpy(s as *mut libc::c_void, s1 as *const libc::c_void, len1);
        s = s.offset(len1 as isize);
    }
    if len2 > 0 as i32 as u64 {
        memcpy(s as *mut libc::c_void, s2 as *const libc::c_void, len2);
        s = s.offset(len2 as isize);
    }
    if len3 > 0 as i32 as u64 {
        memcpy(s as *mut libc::c_void, s3 as *const libc::c_void, len3);
        s = s.offset(len3 as isize);
    }
    *s = '\0' as i32 as i8;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn pp_func(
    mut pc: *mut INSTRUCTION,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut j: i32 = 0;
    static mut first: bool = 1 as i32 != 0;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut pcount: i32 = 0;
    let mut fp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if first {
        first = 0 as i32 != 0;
        if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
            fprintf(
                prof_fp,
                dcgettext(
                    0 as *const i8,
                    b"\n\t# Functions, listed alphabetically\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    pp_namespace_list((*pc.offset(3 as i32 as isize)).nexti);
    fp = (*(*pc).nexti).nexti;
    func = (*pc).x.xn;
    fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
    if !((*pc).comment).is_null() {
        print_comment((*pc).comment, -(1 as i32) as i64);
    }
    indent((*(*pc).nexti).d.ldl);
    let mut malloced: bool = 0 as i32 != 0;
    let mut name: *mut i8 = adjust_namespace((*func).sub.nodep.name, &mut malloced);
    fprintf(
        prof_fp,
        b"%s %s(\0" as *const u8 as *const i8,
        op2str(opcodeval::Op_K_function),
        name,
    );
    if malloced {
        pma_free(name as *mut libc::c_void);
    }
    pcount = (*func).sub.nodep.l.ll as i32;
    func_params = (*func).sub.nodep.rn;
    j = 0 as i32;
    while j < pcount {
        fprintf(
            prof_fp,
            b"%s\0" as *const u8 as *const i8,
            (*func_params.offset(j as isize)).sub.nodep.name,
        );
        if j < pcount - 1 as i32 {
            fprintf(prof_fp, b", \0" as *const u8 as *const i8);
        }
        j += 1;
        j;
    }
    if (*fp).opcode as u32 == opcodeval::Op_comment as i32 as u32
        && (*(*fp).d.dn).sub.val.comtype as u32 == commenttype::EOL_COMMENT as i32 as u32
    {
        fprintf(prof_fp, b")\0" as *const u8 as *const i8);
        fp = end_line(fp);
    } else {
        fprintf(prof_fp, b")\n\0" as *const u8 as *const i8);
    }
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        indent(0 as i32 as exec_count_t);
    }
    fprintf(prof_fp, b"{\n\0" as *const u8 as *const i8);
    indent_in();
    pprint(fp, 0 as *mut INSTRUCTION, 0 as i32);
    indent_out();
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        indent(0 as i32 as exec_count_t);
    }
    fprintf(prof_fp, b"}\n\0" as *const u8 as *const i8);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn redir2str(mut redirtype: i32) -> *const i8 {
    static mut redirtab: [*const i8; 7] = [
        b"\0" as *const u8 as *const i8,
        b" > \0" as *const u8 as *const i8,
        b" >> \0" as *const u8 as *const i8,
        b" | \0" as *const u8 as *const i8,
        b" | \0" as *const u8 as *const i8,
        b" < \0" as *const u8 as *const i8,
        b" |& \0" as *const u8 as *const i8,
    ];
    if redirtype < 0 as i32 || redirtype > redirval::redirect_twoway as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"profile.c\0" as *const u8 as *const i8, 2052 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"redir2str: unknown redirection type %d\0" as *const u8 as *const i8,
                5 as i32,
            ),
            redirtype,
        );
    }
    return redirtab[redirtype as usize];
}
unsafe extern "C" fn pp_namespace(mut name: *const i8, mut comment: *mut INSTRUCTION) {
    if !namespace_changed {
        return;
    }
    if strcmp(current_namespace, name) == 0 as i32 {
        return;
    }
    current_namespace = name;
    fprintf(prof_fp, b"\n\0" as *const u8 as *const i8);
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        indent(0 as i32 as exec_count_t);
    }
    fprintf(prof_fp, b"@namespace \"%s\"\0" as *const u8 as *const i8, name);
    if !comment.is_null() {
        _IO_putc('\t' as i32, prof_fp);
        print_comment(comment, 0 as i32 as i64);
        _IO_putc('\n' as i32, prof_fp);
    } else {
        fprintf(prof_fp, b"\n\n\0" as *const u8 as *const i8);
    };
}
unsafe extern "C" fn pp_namespace_list(mut list: *mut INSTRUCTION) {
    if list.is_null() {
        return;
    }
    pp_namespace_list((*list).nexti);
    pp_namespace((*list).d.name, (*list).comment);
}
unsafe extern "C" fn adjust_namespace(
    mut name: *mut i8,
    mut malloced: *mut bool,
) -> *mut i8 {
    *malloced = 0 as i32 != 0;
    if (strchr(name, ':' as i32)).is_null()
        && current_namespace != awk_namespace.as_ptr()
        && strcmp(current_namespace, awk_namespace.as_ptr()) != 0 as i32
        && !is_all_upper(name)
    {
        let mut buf: *mut i8 = 0 as *mut i8;
        let mut len: size_t = (5 as i32 as u64)
            .wrapping_add(strlen(name))
            .wrapping_add(1 as i32 as u64);
        buf = emalloc_real(
            len,
            b"adjust_namespace\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"profile.c\0" as *const u8 as *const i8,
            2116 as i32,
        ) as *mut i8;
        sprintf(buf, b"awk::%s\0" as *const u8 as *const i8, name);
        *malloced = 1 as i32 != 0;
        return buf;
    }
    let mut len_0: size_t = strlen(current_namespace);
    if strncmp(current_namespace, name, len_0) == 0 as i32
        && *name.offset(len_0 as isize) as i32 == ':' as i32
        && *name.offset(len_0.wrapping_add(1 as i32 as u64) as isize) as i32
            == ':' as i32
    {
        let mut ret: *mut i8 = name.offset(len_0 as isize).offset(2 as i32 as isize);
        return ret;
    }
    return name;
}
unsafe extern "C" fn run_static_initializers() {
    tabs_len = (::core::mem::size_of::<[i8; 28]>() as u64).wrapping_sub(1 as i32 as u64);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];