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
    fn pma_get_root() -> *mut libc::c_void;
    fn pma_set_root(ptr: *mut libc::c_void);
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn refree(rp: *mut Regexp);
    static mut PROCINFO_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut sourceline: i32;
    static mut source: *mut i8;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static mut using_persistent_malloc: bool;
    fn r_unref(tmp: *mut NODE);
    fn null_array(symbol: *mut NODE);
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const i8,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn valinfo(n: *mut NODE, print_func: Func_print, fp: *mut FILE);
    fn nodetype2str(type_0: NODETYPE) -> *const i8;
    fn opcode2str(type_0: OPCODE) -> *const i8;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn update_global_values();
    fn error(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn r_fatal(mesg: *const i8, _: ...);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn more_blocks(id: i32) -> *mut libc::c_void;
    static mut rule_list: *mut INSTRUCTION;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub sub: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub nodep: C2RustUnnamed_2,
    pub val: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
    pub l: C2RustUnnamed_9,
    pub r: C2RustUnnamed_4,
    pub x: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub d: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
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
pub union C2RustUnnamed_5 {
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
pub union C2RustUnnamed_6 {
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
    pub hs: C2RustUnnamed_8,
    pub hi: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub next: *mut bucket_item,
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
pub struct instruction_pool {
    pub pool: [instruction_mem_pool; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_mem_pool {
    pub block_list: *mut instruction_block,
    pub free_space: *mut INSTRUCTION,
    pub free_list: *mut INSTRUCTION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_block {
    pub next: *mut instruction_block,
    pub i: [INSTRUCTION; 126],
}
pub type INSTRUCTION_POOL = instruction_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub pools: INSTRUCTION_POOL,
    pub symbols: NODE,
    pub rule_list: INSTRUCTION,
    pub srcfiles: SRCFILE,
    pub sourceline: i32,
    pub source: *mut i8,
    pub install_func: Option<unsafe extern "C" fn(*mut NODE) -> ()>,
    pub prev: *mut context,
}
pub type AWK_CONTEXT = context;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct root_pointers {
    pub global_table: *mut NODE,
    pub func_table: *mut NODE,
    pub symbol_table: *mut NODE,
    pub nextfree: [block_header; 2],
    pub mpfr: i32,
    pub first: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum SYMBOL_TYPE {
    FUNCTION = 1,
    VARIABLE,
}
impl SYMBOL_TYPE {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            SYMBOL_TYPE::FUNCTION => 1,
            SYMBOL_TYPE::VARIABLE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> SYMBOL_TYPE {
        match value {
            1 => SYMBOL_TYPE::FUNCTION,
            2 => SYMBOL_TYPE::VARIABLE,
            _ => panic!("Invalid value for SYMBOL_TYPE: {}", value),
        }
    }
}
impl AddAssign<u32> for SYMBOL_TYPE {
    fn add_assign(&mut self, rhs: u32) {
        *self = SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for SYMBOL_TYPE {
    fn sub_assign(&mut self, rhs: u32) {
        *self = SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for SYMBOL_TYPE {
    fn mul_assign(&mut self, rhs: u32) {
        *self = SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for SYMBOL_TYPE {
    fn div_assign(&mut self, rhs: u32) {
        *self = SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for SYMBOL_TYPE {
    fn rem_assign(&mut self, rhs: u32) {
        *self = SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for SYMBOL_TYPE {
    type Output = SYMBOL_TYPE;
    fn add(self, rhs: u32) -> SYMBOL_TYPE {
        SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for SYMBOL_TYPE {
    type Output = SYMBOL_TYPE;
    fn sub(self, rhs: u32) -> SYMBOL_TYPE {
        SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for SYMBOL_TYPE {
    type Output = SYMBOL_TYPE;
    fn mul(self, rhs: u32) -> SYMBOL_TYPE {
        SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for SYMBOL_TYPE {
    type Output = SYMBOL_TYPE;
    fn div(self, rhs: u32) -> SYMBOL_TYPE {
        SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for SYMBOL_TYPE {
    type Output = SYMBOL_TYPE;
    fn rem(self, rhs: u32) -> SYMBOL_TYPE {
        SYMBOL_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
static mut symbol_list: *mut NODE = 0 as *const NODE as *mut NODE;
static mut install_func: Option<unsafe extern "C" fn(*mut NODE) -> ()> = None;
static mut curr_ctxt: *mut AWK_CONTEXT = 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT;
static mut ctxt_level: i32 = 0;
static mut global_table: *mut NODE = 0 as *const NODE as *mut NODE;
static mut param_table: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut symbol_table: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut func_table: *mut NODE = 0 as *const NODE as *mut NODE;
static mut installing_specials: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut root_pointers: *mut root_pointers = 0 as *const root_pointers
    as *mut root_pointers;
unsafe extern "C" fn init_the_tables() {
    global_table = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !global_table.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(global_table
            as *mut block_item))
            .freep;
    } else {
        global_table = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(
        global_table as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    null_array(global_table);
    param_table = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !param_table.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(param_table
            as *mut block_item))
            .freep;
    } else {
        param_table = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(
        param_table as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    null_array(param_table);
    installing_specials = 1 as i32 != 0;
    func_table = install_symbol(
        estrdup(b"FUNCTAB\0" as *const u8 as *const i8, 7 as i32 as size_t),
        nodevals::Node_var_array,
    );
    symbol_table = install_symbol(
        estrdup(b"SYMTAB\0" as *const u8 as *const i8, 6 as i32 as size_t),
        nodevals::Node_var_array,
    );
    installing_specials = 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn init_symbol_table() {
    if !using_persistent_malloc {
        init_the_tables();
        return;
    }
    root_pointers = pma_get_root() as *mut root_pointers;
    if root_pointers.is_null() {
        init_the_tables();
        root_pointers = emalloc_real(
            ::core::mem::size_of::<root_pointers>() as u64,
            b"init_symbol_table\0" as *const u8 as *const i8,
            b"root_pointers\0" as *const u8 as *const i8,
            b"symbol.c\0" as *const u8 as *const i8,
            100 as i32,
        ) as *mut root_pointers;
        memset(
            root_pointers as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<root_pointers>() as u64,
        );
        (*root_pointers).global_table = global_table;
        (*root_pointers).func_table = func_table;
        (*root_pointers).symbol_table = symbol_table;
        (*root_pointers).first = 1 as i32 != 0;
        (*root_pointers).mpfr = 0 as i32;
        pma_set_root(root_pointers as *mut libc::c_void);
    } else {
        global_table = (*root_pointers).global_table;
        func_table = (*root_pointers).func_table;
        symbol_table = (*root_pointers).symbol_table;
        memcpy(
            nextfree.as_mut_ptr() as *mut libc::c_void,
            ((*root_pointers).nextfree).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[block_header; 2]>() as u64,
        );
        param_table = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
        if !param_table.is_null() {
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(param_table
                as *mut block_item))
                .freep;
        } else {
            param_table = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
        };
        memset(
            param_table as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<NODE>() as u64,
        );
        null_array(param_table);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pma_mpfr_check() {
    if !using_persistent_malloc {
        return;
    }
    if (*root_pointers).first {
        (*root_pointers).first = 0 as i32 != 0;
        (*root_pointers).mpfr = (do_flags as u32 & do_flag_values::DO_MPFR as i32 as u32)
            as i32;
        return;
    }
    if (*root_pointers).mpfr as u32
        != do_flags as u32 & do_flag_values::DO_MPFR as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"symbol.c\0" as *const u8 as *const i8, 137 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"current setting of -M/--bignum does not match saved setting in PMA backing file\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn pma_save_free_lists() {
    if !using_persistent_malloc {
        return;
    }
    memcpy(
        ((*root_pointers).nextfree).as_mut_ptr() as *mut libc::c_void,
        nextfree.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[block_header; 2]>() as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn install_symbol(
    mut name: *const i8,
    mut type_0: NODETYPE,
) -> *mut NODE {
    return install(name, 0 as *mut NODE, type_0);
}
#[no_mangle]
pub unsafe extern "C" fn lookup(mut name: *const i8) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut tables: [*mut NODE; 5] = [0 as *mut NODE; 5];
    let mut i: i32 = 0;
    tables[0 as i32 as usize] = param_table;
    tables[1 as i32 as usize] = global_table;
    tables[2 as i32 as usize] = func_table;
    tables[3 as i32 as usize] = symbol_table;
    tables[4 as i32 as usize] = 0 as *mut NODE;
    tmp = get_name_from_awk_ns(name);
    n = 0 as *mut NODE;
    i = 0 as i32;
    while !(tables[i as usize]).is_null() {
        if !((*tables[i as usize]).sub.nodep.reflags as u32 == 0 as i32 as u32) {
            if !((do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
                || do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0)
                && tables[i as usize] == global_table)
            {
                n = in_array(tables[i as usize], tmp);
                if !n.is_null() {
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    unref(tmp);
    if n.is_null() || (*n).type_0 as u32 == nodevals::Node_val as i32 as u32 {
        return 0 as *mut NODE;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn make_params(
    mut pnames: *mut *mut i8,
    mut pcount: i32,
) -> *mut NODE {
    let mut p: *mut NODE = 0 as *mut NODE;
    let mut parms: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    if pcount <= 0 as i32 || pnames.is_null() {
        return 0 as *mut NODE;
    }
    parms = ezalloc_real(
        (pcount as u64).wrapping_mul(::core::mem::size_of::<NODE>() as u64),
        b"make_params\0" as *const u8 as *const i8,
        b"parms\0" as *const u8 as *const i8,
        b"symbol.c\0" as *const u8 as *const i8,
        218 as i32,
    ) as *mut NODE;
    i = 0 as i32;
    p = parms;
    while i < pcount {
        (*p).type_0 = nodevals::Node_param_list;
        (*p).sub.nodep.name = *pnames.offset(i as isize);
        (*p).sub.nodep.l.ll = i as i64;
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    return parms;
}
#[no_mangle]
pub unsafe extern "C" fn install_params(mut func: *mut NODE) {
    let mut i: i32 = 0;
    let mut pcount: i32 = 0;
    let mut parms: *mut NODE = 0 as *mut NODE;
    if func.is_null() {
        return;
    }
    pcount = (*func).sub.nodep.l.ll as i32;
    if pcount <= 0 as i32
        || {
            parms = (*func).sub.nodep.rn;
            parms.is_null()
        }
    {
        return;
    }
    i = 0 as i32;
    while i < pcount {
        install(
            (*parms.offset(i as isize)).sub.nodep.name,
            parms.offset(i as isize),
            nodevals::Node_param_list,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn remove_params(mut func: *mut NODE) {
    let mut parms: *mut NODE = 0 as *mut NODE;
    let mut p: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    let mut pcount: i32 = 0;
    if func.is_null() {
        return;
    }
    pcount = (*func).sub.nodep.l.ll as i32;
    if pcount <= 0 as i32
        || {
            parms = (*func).sub.nodep.rn;
            parms.is_null()
        }
    {
        return;
    }
    i = pcount - 1 as i32;
    while i >= 0 as i32 {
        let mut tmp: *mut NODE = 0 as *mut NODE;
        let mut tmp2: *mut NODE = 0 as *mut NODE;
        p = parms.offset(i as isize);
        tmp = make_str_node((*p).sub.nodep.name, strlen((*p).sub.nodep.name), 0 as i32);
        tmp2 = in_array(param_table, tmp);
        if !tmp2.is_null() && !((*tmp2).sub.nodep.r.rptr).is_null() {
            (*tmp2).sub.nodep.r.rptr = (*(*tmp2).sub.nodep.r.rptr).sub.nodep.r.rptr;
        } else {
            ((*(*param_table).sub.nodep.l.lp).remove)
                .expect("non-null function pointer")(param_table, tmp);
        }
        unref(tmp);
        i -= 1;
        i;
    }
    ((*(*param_table).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(param_table, 0 as *mut exp_node);
}
#[no_mangle]
pub unsafe extern "C" fn remove_symbol(mut r: *mut NODE) -> *mut NODE {
    let mut n: *mut NODE = in_array(symbol_table, r);
    if n.is_null() {
        return n;
    }
    n = dupnode(n);
    ((*(*symbol_table).sub.nodep.l.lp).remove)
        .expect("non-null function pointer")(symbol_table, r);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_symbol(mut r: *mut NODE) {
    r = remove_symbol(r);
    if r.is_null() {
        return;
    }
    match (*r).type_0 as u32 {
        9 => {
            if (*r).sub.nodep.l.ll > 0 as i32 as i64 {
                let mut n: *mut NODE = 0 as *mut NODE;
                let mut i: i32 = 0;
                let mut pcount: i32 = (*r).sub.nodep.l.ll as i32;
                i = 0 as i32;
                while i < pcount {
                    n = ((*r).sub.nodep.rn).offset(i as isize);
                    pma_free((*n).sub.nodep.name as *mut libc::c_void);
                    i += 1;
                    i;
                }
                pma_free((*r).sub.nodep.rn as *mut libc::c_void);
            }
        }
        10 => {
            bcfree((*r).sub.nodep.r.iptr);
        }
        5 => {
            ((*(*r).sub.nodep.l.lp).clear)
                .expect("non-null function pointer")(r, 0 as *mut exp_node);
        }
        4 => {
            unref((*r).sub.nodep.l.lptr);
        }
        _ => {}
    }
    pma_free((*r).sub.nodep.name as *mut libc::c_void);
    let ref mut fresh0 = (*(r as *mut block_item)).freep;
    *fresh0 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r as *mut block_item;
}
unsafe extern "C" fn make_symbol(
    mut name: *const i8,
    mut type_0: NODETYPE,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r as *mut block_item))
            .freep;
    } else {
        r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(r as *mut libc::c_void, '\0' as i32, ::core::mem::size_of::<NODE>() as u64);
    if type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        null_array(r);
    } else if type_0 as u32 == nodevals::Node_var as i32 as u32 {
        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
    }
    (*r).sub.nodep.name = name as *mut i8;
    (*r).type_0 = type_0;
    (*r).valref = 1 as i32 as i64;
    return r;
}
unsafe extern "C" fn install(
    mut name: *const i8,
    mut parm: *mut NODE,
    mut type_0: NODETYPE,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut table: *mut NODE = 0 as *mut NODE;
    let mut n_name: *mut NODE = 0 as *mut NODE;
    let mut prev: *mut NODE = 0 as *mut NODE;
    n_name = get_name_from_awk_ns(name);
    table = symbol_table;
    if type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
        table = param_table;
    } else if type_0 as u32 == nodevals::Node_func as i32 as u32
        || type_0 as u32 == nodevals::Node_ext_func as i32 as u32
        || type_0 as u32 == nodevals::Node_builtin_func as i32 as u32
    {
        table = func_table;
    } else if installing_specials {
        table = global_table;
    }
    if !parm.is_null() {
        r = parm;
    } else {
        r = make_symbol(name, type_0);
    }
    let mut current_block_19: u64;
    if type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
        prev = in_array(table, n_name);
        if prev.is_null() {
            current_block_19 = 11863073866293662409;
        } else {
            (*r).sub.nodep.r.rptr = (*prev).sub.nodep.r.rptr;
            (*prev).sub.nodep.r.rptr = r;
            unref(n_name);
            current_block_19 = 2838571290723028321;
        }
    } else {
        current_block_19 = 11863073866293662409;
    }
    match current_block_19 {
        11863073866293662409 => {
            assoc_set(table, n_name, r);
        }
        _ => {}
    }
    if install_func.is_some() {
        (Some(install_func.expect("non-null function pointer")))
            .expect("non-null function pointer")(r);
    }
    return r;
}
unsafe extern "C" fn comp_symbol(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> i32 {
    let mut npp1: *const *const NODE = 0 as *const *const NODE;
    let mut npp2: *const *const NODE = 0 as *const *const NODE;
    let mut n1: *const NODE = 0 as *const NODE;
    let mut n2: *const NODE = 0 as *const NODE;
    npp1 = v1 as *const *const NODE;
    npp2 = v2 as *const *const NODE;
    n1 = *npp1;
    n2 = *npp2;
    let mut n1_is_in_ns: bool = !(strchr((*n1).sub.nodep.name, ':' as i32)).is_null();
    let mut n2_is_in_ns: bool = !(strchr((*n2).sub.nodep.name, ':' as i32)).is_null();
    if n1_is_in_ns as i32 != 0 && n2_is_in_ns as i32 != 0 {
        return strcmp((*n1).sub.nodep.name, (*n2).sub.nodep.name)
    } else if n1_is_in_ns as i32 != 0 && !n2_is_in_ns {
        return 1 as i32
    } else if !n1_is_in_ns && n2_is_in_ns as i32 != 0 {
        return -(1 as i32)
    } else {
        return strcmp((*n1).sub.nodep.name, (*n2).sub.nodep.name)
    };
}
unsafe extern "C" fn get_symbols(
    mut what: SYMBOL_TYPE,
    mut sort: bool,
) -> *mut *mut NODE {
    let mut i: i32 = 0;
    let mut table: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut count: i64 = 0 as i32 as i64;
    let mut max: i64 = 0;
    let mut the_table: *mut NODE = 0 as *mut NODE;
    if what as u32 == SYMBOL_TYPE::FUNCTION as i32 as u32 {
        the_table = func_table;
        max = ((*the_table).sub.nodep.reflags as u32).wrapping_mul(2 as i32 as u32)
            as i64;
        list = assoc_list(
            the_table,
            b"@unsorted\0" as *const u8 as *const i8,
            sort_context_t::ASORTI,
        );
        table = emalloc_real(
            (((*the_table).sub.nodep.reflags as u32).wrapping_add(1 as i32 as u32)
                as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"get_symbols\0" as *const u8 as *const i8,
            b"table\0" as *const u8 as *const i8,
            b"symbol.c\0" as *const u8 as *const i8,
            483 as i32,
        ) as *mut *mut NODE;
        count = 0 as i32 as i64;
        i = count as i32;
        while (i as i64) < max {
            r = *list.offset((i + 1 as i32) as isize);
            if !((*r).type_0 as u32 == nodevals::Node_ext_func as i32 as u32
                || (*r).type_0 as u32 == nodevals::Node_builtin_func as i32 as u32)
            {
                let fresh1 = count;
                count = count + 1;
                let ref mut fresh2 = *table.offset(fresh1 as isize);
                *fresh2 = r;
            }
            i += 2 as i32;
        }
    } else {
        update_global_values();
        the_table = symbol_table;
        max = ((*the_table).sub.nodep.reflags as u32).wrapping_mul(2 as i32 as u32)
            as i64;
        list = assoc_list(
            the_table,
            b"@unsorted\0" as *const u8 as *const i8,
            sort_context_t::ASORTI,
        );
        table = emalloc_real(
            (((*the_table).sub.nodep.reflags as u32)
                .wrapping_add(1 as i32 as u32)
                .wrapping_add(1 as i32 as u32)
                .wrapping_add(1 as i32 as u32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"get_symbols\0" as *const u8 as *const i8,
            b"table\0" as *const u8 as *const i8,
            b"symbol.c\0" as *const u8 as *const i8,
            500 as i32,
        ) as *mut *mut NODE;
        count = 0 as i32 as i64;
        i = count as i32;
        while (i as i64) < max {
            r = *list.offset((i + 1 as i32) as isize);
            if !((*r).type_0 as u32 == nodevals::Node_val as i32 as u32) {
                let fresh3 = count;
                count = count + 1;
                let ref mut fresh4 = *table.offset(fresh3 as isize);
                *fresh4 = r;
            }
            i += 2 as i32;
        }
        let fresh5 = count;
        count = count + 1;
        let ref mut fresh6 = *table.offset(fresh5 as isize);
        *fresh6 = func_table;
        let fresh7 = count;
        count = count + 1;
        let ref mut fresh8 = *table.offset(fresh7 as isize);
        *fresh8 = symbol_table;
    }
    pma_free(list as *mut libc::c_void);
    if sort as i32 != 0 && count > 1 as i32 as i64 {
        qsort(
            table as *mut libc::c_void,
            count as size_t,
            ::core::mem::size_of::<*mut NODE>() as u64,
            Some(
                comp_symbol
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
    }
    let ref mut fresh9 = *table.offset(count as isize);
    *fresh9 = 0 as *mut NODE;
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn variable_list() -> *mut *mut NODE {
    return get_symbols(SYMBOL_TYPE::VARIABLE, 1 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn function_list(mut sort: bool) -> *mut *mut NODE {
    return get_symbols(SYMBOL_TYPE::FUNCTION, sort);
}
#[no_mangle]
pub unsafe extern "C" fn print_vars(
    mut table: *mut *mut NODE,
    mut print_func: Option<unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32>,
    mut fp: *mut FILE,
) {
    let mut i: i32 = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    i = 0 as i32;
    loop {
        r = *table.offset(i as isize);
        if r.is_null() {
            break;
        }
        if !((*r).type_0 as u32 == nodevals::Node_func as i32 as u32
            || (*r).type_0 as u32 == nodevals::Node_ext_func as i32 as u32)
        {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"%s: \0" as *const u8 as *const i8, (*r).sub.nodep.name);
            if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"array, %ld elements\n\0" as *const u8 as *const i8,
                    (*r).sub.nodep.reflags as u32,
                );
            } else if (*r).type_0 as u32 == nodevals::Node_var_new as i32 as u32 {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"untyped variable\n\0" as *const u8 as *const i8);
            } else if (*r).type_0 as u32 == nodevals::Node_var as i32 as u32 {
                valinfo((*r).sub.nodep.l.lptr, print_func, fp);
            } else {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected node type: %s\0"
                        as *const u8 as *const i8,
                    b"symbol.c\0" as *const u8 as *const i8,
                    559 as i32,
                    nodetype2str((*r).type_0),
                );
            }
        }
        i += 1;
        i;
    };
}
#[no_mangle]
pub unsafe extern "C" fn foreach_func(
    mut table: *mut *mut NODE,
    mut pfunc: Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut i: i32 = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret: i32 = 0 as i32;
    i = 0 as i32;
    loop {
        r = *table.offset(i as isize);
        if r.is_null() {
            break;
        }
        ret = pfunc.expect("non-null function pointer")((*r).sub.nodep.r.iptr, data);
        if ret != 0 as i32 {
            break;
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn release_all_vars() {
    ((*(*symbol_table).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(symbol_table, 0 as *mut exp_node);
    ((*(*func_table).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(func_table, 0 as *mut exp_node);
    ((*(*global_table).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(global_table, 0 as *mut exp_node);
}
#[no_mangle]
pub unsafe extern "C" fn append_symbol(mut r: *mut NODE) {
    let mut p: *mut NODE = 0 as *mut NODE;
    p = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !p.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(p as *mut block_item))
            .freep;
    } else {
        p = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*p).sub.nodep.l.lptr = r;
    (*p).sub.nodep.r.rptr = (*symbol_list).sub.nodep.r.rptr;
    (*symbol_list).sub.nodep.r.rptr = p;
}
#[no_mangle]
pub unsafe extern "C" fn release_symbols(mut symlist: *mut NODE, mut keep_globals: i32) {
    let mut p: *mut NODE = 0 as *mut NODE;
    let mut next: *mut NODE = 0 as *mut NODE;
    p = (*symlist).sub.nodep.r.rptr;
    while !p.is_null() {
        if keep_globals == 0 {
            destroy_symbol((*p).sub.nodep.l.lptr);
        }
        next = (*p).sub.nodep.r.rptr;
        let ref mut fresh10 = (*(p as *mut block_item)).freep;
        *fresh10 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = p as *mut block_item;
        p = next;
    }
    (*symlist).sub.nodep.r.rptr = 0 as *mut exp_node;
}
#[no_mangle]
pub unsafe extern "C" fn load_symbols() {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sym_array: *mut NODE = 0 as *mut NODE;
    let mut aptr: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut max: i64 = 0;
    let mut user: *mut NODE = 0 as *mut NODE;
    let mut extension: *mut NODE = 0 as *mut NODE;
    let mut untyped: *mut NODE = 0 as *mut NODE;
    let mut scalar: *mut NODE = 0 as *mut NODE;
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut built_in: *mut NODE = 0 as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tables: [*mut NODE; 4] = [0 as *mut NODE; 4];
    if PROCINFO_node.is_null() {
        return;
    }
    tables[0 as i32 as usize] = func_table;
    tables[1 as i32 as usize] = symbol_table;
    tables[2 as i32 as usize] = global_table;
    tables[3 as i32 as usize] = 0 as *mut NODE;
    tmp = make_str_node(
        b"identifiers\0" as *const u8 as *const i8,
        11 as i32 as size_t,
        0 as i32,
    );
    aptr = ((*(*PROCINFO_node).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(PROCINFO_node, tmp);
    sym_array = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !sym_array.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(sym_array
            as *mut block_item))
            .freep;
    } else {
        sym_array = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(
        sym_array as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    null_array(sym_array);
    unref(tmp);
    unref(*aptr);
    *aptr = sym_array;
    (*sym_array).sub.nodep.x.extra = PROCINFO_node;
    (*sym_array).sub.nodep.name = estrdup(
        b"identifiers\0" as *const u8 as *const i8,
        11 as i32 as size_t,
    );
    user = make_str_node(
        b"user\0" as *const u8 as *const i8,
        4 as i32 as size_t,
        0 as i32,
    );
    extension = make_str_node(
        b"extension\0" as *const u8 as *const i8,
        9 as i32 as size_t,
        0 as i32,
    );
    scalar = make_str_node(
        b"scalar\0" as *const u8 as *const i8,
        6 as i32 as size_t,
        0 as i32,
    );
    untyped = make_str_node(
        b"untyped\0" as *const u8 as *const i8,
        7 as i32 as size_t,
        0 as i32,
    );
    array = make_str_node(
        b"array\0" as *const u8 as *const i8,
        5 as i32 as size_t,
        0 as i32,
    );
    built_in = make_str_node(
        b"builtin\0" as *const u8 as *const i8,
        7 as i32 as size_t,
        0 as i32,
    );
    i = 0 as i32 as i64;
    while !(tables[i as usize]).is_null() {
        list = assoc_list(
            tables[i as usize],
            b"@unsorted\0" as *const u8 as *const i8,
            sort_context_t::ASORTI,
        );
        max = ((*tables[i as usize]).sub.nodep.reflags as u32)
            .wrapping_mul(2 as i32 as u32) as i64;
        if !(max == 0 as i32 as i64) {
            j = 0 as i32 as i64;
            while j < max {
                r = *list.offset((j + 1 as i32 as i64) as isize);
                if (*r).type_0 as u32 == nodevals::Node_ext_func as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_func as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_builtin_func as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_var as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                {
                    if strncmp(
                        (*r).sub.nodep.name,
                        b"awk::\0" as *const u8 as *const i8,
                        5 as i32 as u64,
                    ) == 0 as i32
                    {
                        tmp = make_str_node(
                            ((*r).sub.nodep.name).offset(5 as i32 as isize),
                            (strlen((*r).sub.nodep.name)).wrapping_sub(5 as i32 as u64),
                            0 as i32,
                        );
                    } else {
                        tmp = make_str_node(
                            (*r).sub.nodep.name,
                            strlen((*r).sub.nodep.name),
                            0 as i32,
                        );
                    }
                    aptr = ((*(*sym_array).sub.nodep.l.lp).lookup)
                        .expect("non-null function pointer")(sym_array, tmp);
                    unref(tmp);
                    unref(*aptr);
                    match (*r).type_0 as u32 {
                        10 => {
                            *aptr = dupnode(extension);
                        }
                        9 => {
                            *aptr = dupnode(user);
                        }
                        11 => {
                            *aptr = dupnode(built_in);
                        }
                        4 => {
                            *aptr = dupnode(scalar);
                        }
                        5 => {
                            *aptr = dupnode(array);
                        }
                        6 => {
                            *aptr = dupnode(untyped);
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: unexpected node type %s\0"
                                    as *const u8 as *const i8,
                                b"symbol.c\0" as *const u8 as *const i8,
                                712 as i32,
                                nodetype2str((*r).type_0),
                            );
                        }
                    }
                }
                j += 2 as i32 as i64;
            }
            pma_free(list as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    unref(user);
    unref(extension);
    unref(scalar);
    unref(untyped);
    unref(array);
    unref(built_in);
}
#[no_mangle]
pub unsafe extern "C" fn check_param_names() -> bool {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut max: i64 = 0;
    let mut result: bool = 1 as i32 != 0;
    let mut n: NODE = NODE {
        sub: C2RustUnnamed_0 {
            nodep: C2RustUnnamed_2 {
                l: C2RustUnnamed_9 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_4 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_3 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut i8,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: reflagvals::from_libc_c_uint(0 as u32),
            },
        },
        type_0: nodevals::Node_illegal,
        flags: flagvals::from_libc_c_uint(0 as u32),
        valref: 0,
    };
    if (*func_table).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        return result;
    }
    max = ((*func_table).sub.nodep.reflags as u32).wrapping_mul(2 as i32 as u32) as i64;
    memset(
        &mut n as *mut NODE as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    n.type_0 = nodevals::Node_val;
    n.flags = flagvals::from_libc_c_uint(
        (flagvals::STRING as i32 | flagvals::STRCUR as i32) as u32,
    );
    n.sub.val.idx = -(1 as i32);
    list = assoc_list(
        func_table,
        b"@unsorted\0" as *const u8 as *const i8,
        sort_context_t::ASORTI,
    );
    i = 0 as i32;
    while (i as i64) < max {
        f = *list.offset((i + 1 as i32) as isize);
        if !((*f).type_0 as u32 == nodevals::Node_builtin_func as i32 as u32
            || (*f).sub.nodep.l.ll == 0 as i32 as i64)
        {
            j = 0 as i32;
            while (j as i64) < (*f).sub.nodep.l.ll {
                n.sub.val.sp = (*((*f).sub.nodep.rn).offset(j as isize)).sub.nodep.name;
                n.sub.val.slen = strlen(
                    (*((*f).sub.nodep.rn).offset(j as isize)).sub.nodep.name,
                );
                if !(in_array(func_table, &mut n)).is_null() {
                    error(
                        dcgettext(
                            0 as *const i8,
                            b"function `%s': cannot use function `%s' as a parameter name\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (**list.offset(i as isize)).sub.val.sp,
                        (*((*f).sub.nodep.rn).offset(j as isize)).sub.nodep.name,
                    );
                    result = 0 as i32 != 0;
                }
                j += 1;
                j;
            }
        }
        i += 2 as i32;
    }
    pma_free(list as *mut libc::c_void);
    return result;
}
static mut pools: *mut INSTRUCTION_POOL = 0 as *const INSTRUCTION_POOL
    as *mut INSTRUCTION_POOL;
#[no_mangle]
pub unsafe extern "C" fn bcfree(mut cp: *mut INSTRUCTION) {
    (*cp).opcode = opcodeval::Op_illegal;
    (*cp).nexti = (*pools).pool[((*cp).pool_size as i32 - 1 as i32) as usize].free_list;
    (*pools).pool[((*cp).pool_size as i32 - 1 as i32) as usize].free_list = cp;
}
#[no_mangle]
pub unsafe extern "C" fn bcalloc(
    mut op: OPCODE,
    mut size: i32,
    mut srcline: i32,
) -> *mut INSTRUCTION {
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut pool: *mut instruction_mem_pool = 0 as *mut instruction_mem_pool;
    pool = &mut *((*pools).pool).as_mut_ptr().offset((size - 1 as i32) as isize)
        as *mut instruction_mem_pool;
    if !((*pool).free_list).is_null() {
        cp = (*pool).free_list;
        (*pool).free_list = (*cp).nexti;
    } else if !((*pool).free_space).is_null()
        && ((*pool).free_space).offset(size as isize)
            <= &mut *((*(*pool).block_list).i)
                .as_mut_ptr()
                .offset((2 as i32 * 3 as i32 * 21 as i32) as isize) as *mut INSTRUCTION
    {
        cp = (*pool).free_space;
        (*pool).free_space = ((*pool).free_space).offset(size as isize);
    } else {
        let mut block: *mut instruction_block = 0 as *mut instruction_block;
        block = emalloc_real(
            ::core::mem::size_of::<instruction_block>() as u64,
            b"bcalloc\0" as *const u8 as *const i8,
            b"block\0" as *const u8 as *const i8,
            b"symbol.c\0" as *const u8 as *const i8,
            837 as i32,
        ) as *mut instruction_block;
        (*block).next = (*pool).block_list;
        (*pool).block_list = block;
        cp = &mut *((*block).i).as_mut_ptr().offset(0 as i32 as isize)
            as *mut INSTRUCTION;
        (*pool).free_space = &mut *((*block).i).as_mut_ptr().offset(size as isize)
            as *mut INSTRUCTION;
    }
    memset(
        cp as *mut libc::c_void,
        0 as i32,
        (size as u64).wrapping_mul(::core::mem::size_of::<INSTRUCTION>() as u64),
    );
    (*cp).pool_size = size as libc::c_short;
    (*cp).opcode = op;
    (*cp).source_line = srcline as libc::c_short;
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn new_context() -> *mut AWK_CONTEXT {
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    ctxt = ezalloc_real(
        ::core::mem::size_of::<AWK_CONTEXT>() as u64,
        b"new_context\0" as *const u8 as *const i8,
        b"ctxt\0" as *const u8 as *const i8,
        b"symbol.c\0" as *const u8 as *const i8,
        858 as i32,
    ) as *mut AWK_CONTEXT;
    (*ctxt).srcfiles.prev = &mut (*ctxt).srcfiles;
    (*ctxt).srcfiles.next = (*ctxt).srcfiles.prev;
    (*ctxt).rule_list.opcode = opcodeval::Op_list;
    (*ctxt).rule_list.d.di = &mut (*ctxt).rule_list;
    return ctxt;
}
unsafe extern "C" fn set_context(mut ctxt: *mut AWK_CONTEXT) {
    pools = &mut (*ctxt).pools;
    symbol_list = &mut (*ctxt).symbols;
    srcfiles = &mut (*ctxt).srcfiles;
    rule_list = &mut (*ctxt).rule_list;
    install_func = (*ctxt).install_func;
    curr_ctxt = ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn push_context(mut ctxt: *mut AWK_CONTEXT) {
    (*ctxt).prev = curr_ctxt;
    if !curr_ctxt.is_null() {
        (*curr_ctxt).sourceline = sourceline;
        (*curr_ctxt).source = source;
    }
    sourceline = 0 as i32;
    source = 0 as *mut i8;
    set_context(ctxt);
    ctxt_level += 1;
    ctxt_level;
}
#[no_mangle]
pub unsafe extern "C" fn pop_context() {
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    if ((*curr_ctxt).prev).is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"symbol.c\0" as *const u8 as *const i8, 910 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"cannot pop main context\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    ctxt = (*curr_ctxt).prev;
    sourceline = (*ctxt).sourceline;
    source = (*ctxt).source;
    set_context(ctxt);
    ctxt_level -= 1;
    ctxt_level;
}
#[no_mangle]
pub unsafe extern "C" fn in_main_context() -> i32 {
    return (ctxt_level == 1 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn free_context(
    mut ctxt: *mut AWK_CONTEXT,
    mut keep_globals: bool,
) {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut sn: *mut SRCFILE = 0 as *mut SRCFILE;
    if ctxt.is_null() {
        return;
    }
    free_bcpool(&mut (*ctxt).pools);
    release_symbols(&mut (*ctxt).symbols, keep_globals as i32);
    s = &mut (*ctxt).srcfiles;
    while s != &mut (*ctxt).srcfiles as *mut SRCFILE {
        sn = (*s).next;
        if (*s).stype as u32 != srctype::SRC_CMDLINE as i32 as u32
            && (*s).stype as u32 != srctype::SRC_STDIN as i32 as u32
        {
            pma_free((*s).fullpath as *mut libc::c_void);
        }
        pma_free((*s).src as *mut libc::c_void);
        pma_free(s as *mut libc::c_void);
        s = sn;
    }
    pma_free(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn free_bc_internal(mut cp: *mut INSTRUCTION) {
    let mut m: *mut NODE = 0 as *mut NODE;
    match (*cp).opcode as u32 {
        73 => {
            if !((*cp).d.name).is_null() {
                pma_free((*cp).d.name as *mut libc::c_void);
            }
        }
        79 | 49 | 48 | 50 => {
            m = (*cp).d.dn;
            if !((*m).sub.nodep.r.preg[0 as i32 as usize]).is_null() {
                refree((*m).sub.nodep.r.preg[0 as i32 as usize]);
            }
            if !((*m).sub.nodep.r.preg[1 as i32 as usize]).is_null() {
                refree((*m).sub.nodep.r.preg[1 as i32 as usize]);
            }
            if !((*m).sub.nodep.x.extra).is_null() {
                unref((*m).sub.nodep.x.extra);
            }
            if !((*m).sub.nodep.l.lptr).is_null() {
                unref((*m).sub.nodep.l.lptr);
            }
            let ref mut fresh11 = (*(m as *mut block_item)).freep;
            *fresh11 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = m as *mut block_item;
        }
        109 => {
            if !((*cp).d.name).is_null() {
                pma_free((*cp).d.name as *mut libc::c_void);
            }
        }
        78 => {
            m = (*cp).d.dn;
            unref(m);
        }
        27 => {
            m = (*cp).x.xn;
            if !m.is_null() {
                unref(m);
            }
        }
        0 => {
            r_fatal(
                b"internal error: file %s, line %d: unexpected opcode %s\0" as *const u8
                    as *const i8,
                b"symbol.c\0" as *const u8 as *const i8,
                1003 as i32,
                opcode2str((*cp).opcode),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn free_bc_mempool(
    mut pool: *mut instruction_mem_pool,
    mut size: i32,
) {
    let mut first: bool = 1 as i32 != 0;
    let mut block: *mut instruction_block = 0 as *mut instruction_block;
    let mut next: *mut instruction_block = 0 as *mut instruction_block;
    block = (*pool).block_list;
    while !block.is_null() {
        let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        let mut end: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        end = if first as i32 != 0 {
            (*pool).free_space
        } else {
            &mut *((*block).i)
                .as_mut_ptr()
                .offset((2 as i32 * 3 as i32 * 21 as i32) as isize) as *mut INSTRUCTION
        };
        cp = &mut *((*block).i).as_mut_ptr().offset(0 as i32 as isize)
            as *mut INSTRUCTION;
        while cp.offset(size as isize) <= end {
            if (*cp).opcode as u32 != opcodeval::Op_illegal as i32 as u32 {
                free_bc_internal(cp);
            }
            cp = cp.offset(size as isize);
        }
        next = (*block).next;
        pma_free(block as *mut libc::c_void);
        first = 0 as i32 != 0;
        block = next;
    }
}
unsafe extern "C" fn free_bcpool(mut pl: *mut INSTRUCTION_POOL) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        free_bc_mempool(
            &mut *((*pl).pool).as_mut_ptr().offset(i as isize),
            i + 1 as i32,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_all_upper(mut name: *const i8) -> bool {
    while *name as i32 != '\0' as i32 {
        match *name as i32 {
            65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
            | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {}
            _ => return 0 as i32 != 0,
        }
        name = name.offset(1);
        name;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn get_name_from_awk_ns(mut name: *const i8) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if strncmp(name, b"awk::\0" as *const u8 as *const i8, 5 as i32 as u64) == 0 as i32 {
        tmp = make_str_node(
            name.offset(5 as i32 as isize),
            (strlen(name)).wrapping_sub(5 as i32 as u64),
            0 as i32,
        );
    } else {
        tmp = make_str_node(name, strlen(name), 0 as i32);
    }
    return tmp;
}