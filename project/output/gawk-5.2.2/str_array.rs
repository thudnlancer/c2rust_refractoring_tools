use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn unsetenv(__name: *const i8) -> i32;
    fn setenv(__name: *const i8, __value: *const i8, __replace: i32) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    static mut environ: *mut *mut i8;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut Nnull_string: *mut NODE;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    fn assoc_info(subs: *mut NODE, val: *mut NODE, p: *mut NODE, aname: *const i8);
    fn make_aname(symbol: *const NODE) -> *const i8;
    fn flags2str(_: i32) -> *const i8;
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn assoc_copy(symbol: *mut NODE, newsymb: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn make_array() -> *mut NODE;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn more_blocks(id: i32) -> *mut libc::c_void;
    static mut nextfree: [block_header; 2];
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    static mut success_node: *mut NODE;
    fn r_unref(tmp: *mut NODE);
    fn new_array_element() -> *mut NODE;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn getenv_long(name: *const i8) -> i64;
    static mut do_flags: do_flag_values;
    static mut output_fp: *mut FILE;
    fn indent(indent_level: i32);
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub enum assoc_kind_t {
    ANONE = 0x0,
    AINDEX = 0x1,
    AVALUE = 0x2,
    AINUM = 0x4,
    AISTR = 0x8,
    AVNUM = 0x10,
    AVSTR = 0x20,
    AASC = 0x40,
    ADESC = 0x80,
    ADELETE = 0x100,
}
impl assoc_kind_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            assoc_kind_t::ANONE => 0x0,
            assoc_kind_t::AINDEX => 0x1,
            assoc_kind_t::AVALUE => 0x2,
            assoc_kind_t::AINUM => 0x4,
            assoc_kind_t::AISTR => 0x8,
            assoc_kind_t::AVNUM => 0x10,
            assoc_kind_t::AVSTR => 0x20,
            assoc_kind_t::AASC => 0x40,
            assoc_kind_t::ADESC => 0x80,
            assoc_kind_t::ADELETE => 0x100,
        }
    }
    fn from_libc_c_uint(value: u32) -> assoc_kind_t {
        match value {
            0x0 => assoc_kind_t::ANONE,
            0x1 => assoc_kind_t::AINDEX,
            0x2 => assoc_kind_t::AVALUE,
            0x4 => assoc_kind_t::AINUM,
            0x8 => assoc_kind_t::AISTR,
            0x10 => assoc_kind_t::AVNUM,
            0x20 => assoc_kind_t::AVSTR,
            0x40 => assoc_kind_t::AASC,
            0x80 => assoc_kind_t::ADESC,
            0x100 => assoc_kind_t::ADELETE,
            _ => panic!("Invalid value for assoc_kind_t: {}", value),
        }
    }
}
impl AddAssign<u32> for assoc_kind_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for assoc_kind_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for assoc_kind_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for assoc_kind_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for assoc_kind_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn add(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn sub(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn mul(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn div(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn rem(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
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
unsafe extern "C" fn str_terminate_f(mut n: *mut NODE, mut savep: *mut i8) {
    *savep = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as i8;
}
static mut STR_CHAIN_MAX: size_t = 2 as i32 as size_t;
#[no_mangle]
pub static mut str_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"str\0" as *const u8 as *const i8,
            init: Some(
                str_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: None,
            lookup: Some(
                str_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                str_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                str_clear as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                str_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                str_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                str_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                str_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
static mut env_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"env\0" as *const u8 as *const i8,
            init: Some(
                str_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: None,
            lookup: Some(
                str_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                str_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                env_clear as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                env_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                str_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                str_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                str_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: Some(
                env_store as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut hash: Option<
    unsafe extern "C" fn(*const i8, size_t, u64, *mut size_t) -> u64,
> = unsafe {
    Some(awk_hash as unsafe extern "C" fn(*const i8, size_t, u64, *mut size_t) -> u64)
};
unsafe extern "C" fn str_array_init(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if symbol.is_null() {
        let mut newval: i64 = 0;
        let mut val: *const i8 = 0 as *const i8;
        newval = getenv_long(b"STR_CHAIN_MAX\0" as *const u8 as *const i8);
        if newval > 0 as i32 as i64 {
            STR_CHAIN_MAX = newval as size_t;
        }
        val = getenv(b"AWK_HASH\0" as *const u8 as *const i8);
        if !val.is_null() {
            if strcmp(val, b"gst\0" as *const u8 as *const i8) == 0 as i32 {
                hash = Some(
                    gst_hash_string
                        as unsafe extern "C" fn(
                            *const i8,
                            size_t,
                            u64,
                            *mut size_t,
                        ) -> u64,
                );
            } else if strcmp(val, b"fnv1a\0" as *const u8 as *const i8) == 0 as i32 {
                hash = Some(
                    fnv1a_hash_string
                        as unsafe extern "C" fn(
                            *const i8,
                            size_t,
                            u64,
                            *mut size_t,
                        ) -> u64,
                );
            }
        }
    } else {
        null_array(symbol);
    }
    return &mut success_node;
}
unsafe extern "C" fn str_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: u64 = 0;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut code1: size_t = 0;
    subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
    if ((*symbol).sub.nodep.r.bv).is_null() {
        grow_table(symbol);
    }
    hash1 = hash
        .expect(
            "non-null function pointer",
        )((*subs).sub.val.sp, (*subs).sub.val.slen, (*symbol).sub.nodep.cnt, &mut code1);
    lhs = str_find(symbol, subs, code1, hash1);
    if !lhs.is_null() {
        return lhs;
    }
    (*symbol).sub.nodep.reflags += 1;
    (*symbol).sub.nodep.reflags;
    if (*symbol).flags as u32 & flagvals::ARRAYMAXED as i32 as u32 == 0 as i32 as u32
        && ((*symbol).sub.nodep.reflags as u64).wrapping_div((*symbol).sub.nodep.cnt)
            > STR_CHAIN_MAX
    {
        grow_table(symbol);
        hash1 = code1.wrapping_rem((*symbol).sub.nodep.cnt);
    }
    if (*subs).sub.val.idx != -(1 as i32) || subs == Nnull_string
        || (*subs).flags as u32 & flagvals::STRING as i32 as u32 == 0 as i32 as u32
        || (*subs).flags as u32 & flagvals::NULL_FIELD as i32 as u32 != 0 as i32 as u32
    {
        let mut tmp: *mut NODE = 0 as *mut NODE;
        tmp = make_str_node((*subs).sub.val.sp, (*subs).sub.val.slen, 0 as i32);
        if (*subs).flags as u32
            & (flagvals::MPFN as i32 | flagvals::MPZN as i32 | flagvals::NUMCUR as i32)
                as u32 == flagvals::NUMCUR as i32 as u32
        {
            (*tmp).sub.val.fltnum = (*subs).sub.val.fltnum;
            (*tmp).flags = ::core::mem::transmute::<
                u32,
                flagvals,
            >((*tmp).flags as u32 | flagvals::NUMCUR as i32 as u32);
        }
        subs = tmp;
    } else {
        subs = dupnode(subs);
    }
    b = nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep as *mut BUCKET;
    if !b.is_null() {
        nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep = (*(b
            as *mut block_item))
            .freep;
    } else {
        b = more_blocks(block_id::BLOCK_BUCKET as i32) as *mut BUCKET;
    };
    (*b).hs.next = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    let ref mut fresh0 = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    *fresh0 = b;
    (*b).hs.name = subs;
    (*b).hs.str_0 = (*subs).sub.val.sp;
    (*b).hs.len = (*subs).sub.val.slen;
    (*b).hs.val = new_array_element();
    (*b).hs.code = code1;
    return &mut (*b).hs.val;
}
unsafe extern "C" fn str_exists(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: u64 = 0;
    let mut code1: size_t = 0;
    if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        return 0 as *mut *mut NODE;
    }
    subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
    hash1 = hash
        .expect(
            "non-null function pointer",
        )((*subs).sub.val.sp, (*subs).sub.val.slen, (*symbol).sub.nodep.cnt, &mut code1);
    return str_find(symbol, subs, code1, hash1);
}
unsafe extern "C" fn str_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: u64 = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut next: *mut BUCKET = 0 as *mut BUCKET;
    let mut r: *mut NODE = 0 as *mut NODE;
    i = 0 as i32 as u64;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            next = (*b).hs.next;
            r = (*b).hs.val;
            if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                ((*(*r).sub.nodep.l.lp).clear)
                    .expect("non-null function pointer")(r, 0 as *mut exp_node);
                pma_free((*r).sub.nodep.name as *mut libc::c_void);
                let ref mut fresh1 = (*(r as *mut block_item)).freep;
                *fresh1 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r
                    as *mut block_item;
            } else {
                unref(r);
            }
            unref((*b).hs.name);
            let ref mut fresh2 = (*(b as *mut block_item)).freep;
            *fresh2 = nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep;
            nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep = b
                as *mut block_item;
            b = next;
        }
        let ref mut fresh3 = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        *fresh3 = 0 as *mut BUCKET;
        i = i.wrapping_add(1);
        i;
    }
    if !((*symbol).sub.nodep.r.bv).is_null() {
        pma_free((*symbol).sub.nodep.r.bv as *mut libc::c_void);
    }
    ((*(*symbol).sub.nodep.l.lp).init)
        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn str_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: u64 = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut prev: *mut BUCKET = 0 as *mut BUCKET;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut s1_len: size_t = 0;
    if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        return 0 as *mut *mut NODE;
    }
    s2 = force_string_fmt(subs, CONVFMT, CONVFMTidx);
    hash1 = hash
        .expect(
            "non-null function pointer",
        )(
        (*s2).sub.val.sp,
        (*s2).sub.val.slen,
        (*symbol).sub.nodep.cnt,
        0 as *mut size_t,
    );
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    prev = 0 as *mut BUCKET;
    while !b.is_null() {
        s1_len = (*b).hs.len;
        if !(s1_len != (*s2).sub.val.slen) {
            if s1_len == 0 as i32 as u64
                || memcmp(
                    (*b).hs.str_0 as *const libc::c_void,
                    (*s2).sub.val.sp as *const libc::c_void,
                    s1_len,
                ) == 0 as i32
            {
                unref((*b).hs.name);
                if !prev.is_null() {
                    (*prev).hs.next = (*b).hs.next;
                } else {
                    let ref mut fresh4 = *((*symbol).sub.nodep.r.bv)
                        .offset(hash1 as isize);
                    *fresh4 = (*b).hs.next;
                }
                let ref mut fresh5 = (*(b as *mut block_item)).freep;
                *fresh5 = nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep;
                nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep = b
                    as *mut block_item;
                (*symbol).sub.nodep.reflags -= 1;
                if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
                    if !((*symbol).sub.nodep.r.bv).is_null() {
                        pma_free((*symbol).sub.nodep.r.bv as *mut libc::c_void);
                    }
                    ((*(*symbol).sub.nodep.l.lp).init)
                        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
                }
                return &mut success_node;
            }
        }
        prev = b;
        b = (*b).hs.next;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn str_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut *mut NODE {
    let mut old: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut new: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut pnew: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut chain: *mut BUCKET = 0 as *mut BUCKET;
    let mut newchain: *mut BUCKET = 0 as *mut BUCKET;
    let mut cursize: u64 = 0;
    let mut i: u64 = 0;
    cursize = (*symbol).sub.nodep.cnt;
    new = ezalloc_real(
        cursize.wrapping_mul(::core::mem::size_of::<*mut BUCKET>() as u64),
        b"str_copy\0" as *const u8 as *const i8,
        b"new\0" as *const u8 as *const i8,
        b"str_array.c\0" as *const u8 as *const i8,
        342 as i32,
    ) as *mut *mut BUCKET;
    old = (*symbol).sub.nodep.r.bv;
    i = 0 as i32 as u64;
    while i < cursize {
        chain = *old.offset(i as isize);
        pnew = &mut *new.offset(i as isize) as *mut *mut BUCKET;
        while !chain.is_null() {
            let mut oldval: *mut NODE = 0 as *mut NODE;
            let mut newsubs: *mut NODE = 0 as *mut NODE;
            newchain = nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep
                as *mut BUCKET;
            if !newchain.is_null() {
                nextfree[block_id::BLOCK_BUCKET as i32 as usize].freep = (*(newchain
                    as *mut block_item))
                    .freep;
            } else {
                newchain = more_blocks(block_id::BLOCK_BUCKET as i32) as *mut BUCKET;
            };
            (*newchain).hs.name = dupnode((*chain).hs.name);
            newsubs = (*newchain).hs.name;
            (*newchain).hs.str_0 = (*newsubs).sub.val.sp;
            (*newchain).hs.len = (*newsubs).sub.val.slen;
            oldval = (*chain).hs.val;
            if (*oldval).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                (*newchain).hs.val = dupnode(oldval);
            } else {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = make_array();
                (*r).sub.nodep.name = estrdup(
                    (*oldval).sub.nodep.name,
                    strlen((*oldval).sub.nodep.name),
                );
                (*r).sub.nodep.x.extra = newsymb;
                (*newchain).hs.val = assoc_copy(oldval, r);
            }
            (*newchain).hs.code = (*chain).hs.code;
            *pnew = newchain;
            (*newchain).hs.next = 0 as *mut bucket_item;
            pnew = &mut (*newchain).hs.next;
            chain = (*chain).hs.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*newsymb).sub.nodep.reflags = (*symbol).sub.nodep.reflags;
    (*newsymb).sub.nodep.r.bv = new;
    (*newsymb).sub.nodep.cnt = cursize;
    (*newsymb).flags = (*symbol).flags;
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn str_list(
    mut symbol: *mut NODE,
    mut t: *mut NODE,
) -> *mut *mut NODE {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut num_elems: u64 = 0;
    let mut list_size: u64 = 0;
    let mut i: u64 = 0;
    let mut k: u64 = 0 as i32 as u64;
    let mut elem_size: i32 = 1 as i32;
    let mut assoc_kind: assoc_kind_t = assoc_kind_t::ANONE;
    if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        return 0 as *mut *mut NODE;
    }
    assoc_kind = (*t).flags as assoc_kind_t;
    if assoc_kind as u32
        & (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32
        == (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32
    {
        elem_size = 2 as i32;
    }
    num_elems = (*symbol).sub.nodep.reflags as u64;
    if assoc_kind as u32
        & (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32
            | assoc_kind_t::ADELETE as i32) as u32
        == (assoc_kind_t::AINDEX as i32 | assoc_kind_t::ADELETE as i32) as u32
    {
        num_elems = 1 as i32 as u64;
    }
    list_size = (elem_size as u64).wrapping_mul(num_elems);
    list = emalloc_real(
        list_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"str_list\0" as *const u8 as *const i8,
        b"list\0" as *const u8 as *const i8,
        b"str_array.c\0" as *const u8 as *const i8,
        415 as i32,
    ) as *mut *mut NODE;
    i = 0 as i32 as u64;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            subs = (*b).hs.name;
            if assoc_kind as u32 & assoc_kind_t::AINUM as i32 as u32 != 0 as i32 as u32 {
                force_number(subs);
            }
            let fresh6 = k;
            k = k.wrapping_add(1);
            let ref mut fresh7 = *list.offset(fresh6 as isize);
            *fresh7 = dupnode(subs);
            if assoc_kind as u32 & assoc_kind_t::AVALUE as i32 as u32 != 0 as i32 as u32
            {
                val = (*b).hs.val;
                if (*val).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    if assoc_kind as u32 & assoc_kind_t::AVNUM as i32 as u32
                        != 0 as i32 as u32
                    {
                        force_number(val);
                    } else if assoc_kind as u32 & assoc_kind_t::AVSTR as i32 as u32
                        != 0 as i32 as u32
                    {
                        val = force_string_fmt(val, CONVFMT, CONVFMTidx);
                    }
                }
                let fresh8 = k;
                k = k.wrapping_add(1);
                let ref mut fresh9 = *list.offset(fresh8 as isize);
                *fresh9 = val;
            }
            if k >= list_size {
                return list;
            }
            b = (*b).hs.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn str_kilobytes(mut symbol: *mut NODE) -> libc::c_double {
    let mut bucket_cnt: u64 = 0;
    let mut kb: libc::c_double = 0.;
    bucket_cnt = (*symbol).sub.nodep.reflags as u64;
    kb = (bucket_cnt as libc::c_double
        * ::core::mem::size_of::<BUCKET>() as u64 as libc::c_double
        + (*symbol).sub.nodep.cnt as libc::c_double
            * ::core::mem::size_of::<*mut BUCKET>() as u64 as libc::c_double)
        / 1024.0f64;
    return kb;
}
unsafe extern "C" fn str_dump(
    mut symbol: *mut NODE,
    mut ndump: *mut NODE,
) -> *mut *mut NODE {
    let mut indent_level: i32 = 0;
    let mut i: u64 = 0;
    let mut bucket_cnt: u64 = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    static mut hash_dist: [size_t; 32] = [0; 32];
    indent_level = (*ndump).sub.nodep.x.xl as i32;
    if (*symbol).flags as u32 & flagvals::XARRAY as i32 as u32 == 0 as i32 as u32 {
        fprintf(
            output_fp,
            b"%s `%s'\n\0" as *const u8 as *const i8,
            if ((*symbol).sub.nodep.x.extra).is_null() {
                b"array\0" as *const u8 as *const i8
            } else {
                b"sub-array\0" as *const u8 as *const i8
            },
            array_vname(symbol),
        );
    }
    indent_level += 1;
    indent_level;
    indent(indent_level);
    fprintf(output_fp, b"array_func: str_array_func\n\0" as *const u8 as *const i8);
    if (*symbol).flags as u32 != 0 as i32 as u32 {
        indent(indent_level);
        fprintf(
            output_fp,
            b"flags: %s\n\0" as *const u8 as *const i8,
            flags2str((*symbol).flags as i32),
        );
    }
    indent(indent_level);
    fprintf(
        output_fp,
        b"STR_CHAIN_MAX: %lu\n\0" as *const u8 as *const i8,
        STR_CHAIN_MAX,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_size: %lu\n\0" as *const u8 as *const i8,
        (*symbol).sub.nodep.cnt,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"table_size: %lu\n\0" as *const u8 as *const i8,
        (*symbol).sub.nodep.reflags as u64,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"Avg # of items per chain: %.2g\n\0" as *const u8 as *const i8,
        (*symbol).sub.nodep.reflags as libc::c_double
            / (*symbol).sub.nodep.cnt as libc::c_double,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"memory: %.2g kB\n\0" as *const u8 as *const i8,
        str_kilobytes(symbol),
    );
    memset(
        hash_dist.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ((31 as i32 + 1 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<size_t>() as u64),
    );
    i = 0 as i32 as u64;
    while i < (*symbol).sub.nodep.cnt {
        bucket_cnt = 0 as i32 as u64;
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            bucket_cnt = bucket_cnt.wrapping_add(1);
            bucket_cnt;
            b = (*b).hs.next;
        }
        if bucket_cnt >= 31 as i32 as u64 {
            bucket_cnt = 31 as i32 as u64;
        }
        hash_dist[bucket_cnt as usize] = (hash_dist[bucket_cnt as usize])
            .wrapping_add(1);
        hash_dist[bucket_cnt as usize];
        i = i.wrapping_add(1);
        i;
    }
    indent(indent_level);
    fprintf(output_fp, b"Hash distribution:\n\0" as *const u8 as *const i8);
    indent_level += 1;
    indent_level;
    i = 0 as i32 as u64;
    while i <= 31 as i32 as u64 {
        if hash_dist[i as usize] > 0 as i32 as u64 {
            indent(indent_level);
            if i == 31 as i32 as u64 {
                fprintf(
                    output_fp,
                    b"[>=%lu]:%lu\n\0" as *const u8 as *const i8,
                    31 as i32 as u64,
                    hash_dist[i as usize],
                );
            } else {
                fprintf(
                    output_fp,
                    b"[%lu]:%lu\n\0" as *const u8 as *const i8,
                    i,
                    hash_dist[i as usize],
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    indent_level -= 1;
    indent_level;
    if (*ndump).sub.nodep.l.ll >= 0 as i32 as i64 {
        let mut aname: *const i8 = 0 as *const i8;
        fprintf(output_fp, b"\n\0" as *const u8 as *const i8);
        aname = make_aname(symbol);
        i = 0 as i32 as u64;
        while i < (*symbol).sub.nodep.cnt {
            b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
            while !b.is_null() {
                assoc_info((*b).hs.name, (*b).hs.val, ndump, aname);
                b = (*b).hs.next;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn awk_hash(
    mut s: *const i8,
    mut len: size_t,
    mut hsize: u64,
    mut code: *mut size_t,
) -> u64 {
    let mut h: u64 = 0 as i32 as u64;
    let mut htmp: u64 = 0;
    h = 0 as i32 as u64;
    if len > 0 as i32 as u64 {
        let mut loop_0: size_t = len
            .wrapping_add(8 as i32 as u64)
            .wrapping_sub(1 as i32 as u64) >> 3 as i32;
        's_139: {
            let mut current_block_32: u64;
            match len & (8 as i32 - 1 as i32) as u64 {
                0 => {
                    current_block_32 = 10680521327981672866;
                }
                7 => {
                    current_block_32 = 5499897176314894263;
                }
                6 => {
                    current_block_32 = 14343490084333691418;
                }
                5 => {
                    current_block_32 = 3909645888961320850;
                }
                4 => {
                    current_block_32 = 10260081853797976808;
                }
                3 => {
                    current_block_32 = 3347075717393403157;
                }
                2 => {
                    current_block_32 = 12327483733956094797;
                }
                1 => {
                    current_block_32 = 11609910033890719537;
                }
                _ => {
                    current_block_32 = 14763689060501151050;
                }
            }
            loop {
                match current_block_32 {
                    14763689060501151050 => {
                        break 's_139;
                    }
                    12327483733956094797 => {
                        htmp = h << 6 as i32;
                        let fresh16 = s;
                        s = s.offset(1);
                        h = (*fresh16 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 11609910033890719537;
                    }
                    3347075717393403157 => {
                        htmp = h << 6 as i32;
                        let fresh15 = s;
                        s = s.offset(1);
                        h = (*fresh15 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 12327483733956094797;
                    }
                    10260081853797976808 => {
                        htmp = h << 6 as i32;
                        let fresh14 = s;
                        s = s.offset(1);
                        h = (*fresh14 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 3347075717393403157;
                    }
                    3909645888961320850 => {
                        htmp = h << 6 as i32;
                        let fresh13 = s;
                        s = s.offset(1);
                        h = (*fresh13 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 10260081853797976808;
                    }
                    14343490084333691418 => {
                        htmp = h << 6 as i32;
                        let fresh12 = s;
                        s = s.offset(1);
                        h = (*fresh12 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 3909645888961320850;
                    }
                    5499897176314894263 => {
                        htmp = h << 6 as i32;
                        let fresh11 = s;
                        s = s.offset(1);
                        h = (*fresh11 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 14343490084333691418;
                    }
                    10680521327981672866 => {
                        htmp = h << 6 as i32;
                        let fresh10 = s;
                        s = s.offset(1);
                        h = (*fresh10 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        current_block_32 = 5499897176314894263;
                    }
                    _ => {
                        htmp = h << 6 as i32;
                        let fresh17 = s;
                        s = s.offset(1);
                        h = (*fresh17 as u64)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as i32)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as u32 as u64;
                        h &= 0xffffffff as u32 as u64;
                        loop_0 = loop_0.wrapping_sub(1);
                        if loop_0 != 0 {
                            current_block_32 = 10680521327981672866;
                        } else {
                            current_block_32 = 14763689060501151050;
                        }
                    }
                }
            }
        }
    }
    if !code.is_null() {
        *code = h;
    }
    if h >= hsize {
        h = h.wrapping_rem(hsize);
    }
    return h;
}
#[inline]
unsafe extern "C" fn str_find(
    mut symbol: *mut NODE,
    mut s1: *mut NODE,
    mut code1: size_t,
    mut hash1: u64,
) -> *mut *mut NODE {
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut s2_len: size_t = 0;
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    while !b.is_null() {
        s2_len = (*b).hs.len;
        if code1 == (*b).hs.code && (*s1).sub.val.slen == s2_len
            && (s2_len == 0 as i32 as u64
                || memcmp(
                    (*s1).sub.val.sp as *const libc::c_void,
                    (*b).hs.str_0 as *const libc::c_void,
                    s2_len,
                ) == 0 as i32)
        {
            return &mut (*b).hs.val;
        }
        b = (*b).hs.next;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn grow_table(mut symbol: *mut NODE) {
    let mut old: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut new: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut chain: *mut BUCKET = 0 as *mut BUCKET;
    let mut next: *mut BUCKET = 0 as *mut BUCKET;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut oldsize: u64 = 0;
    let mut newsize: u64 = 0;
    let mut k: u64 = 0;
    let mut hash1: u64 = 0;
    static mut sizes: [u64; 21] = [
        13 as i32 as u64,
        127 as i32 as u64,
        1021 as i32 as u64,
        8191 as i32 as u64,
        16381 as i32 as u64,
        32749 as i32 as u64,
        65497 as i32 as u64,
        131101 as i32 as u64,
        262147 as i32 as u64,
        524309 as i32 as u64,
        1048583 as i32 as u64,
        2097169 as i32 as u64,
        4194319 as i32 as u64,
        8388617 as i32 as u64,
        16777259 as i32 as u64,
        33554467 as i32 as u64,
        67108879 as i32 as u64,
        134217757 as i32 as u64,
        268435459 as i32 as u64,
        536870923 as i32 as u64,
        1073741827 as i32 as u64,
    ];
    oldsize = (*symbol).sub.nodep.cnt;
    newsize = oldsize;
    i = 0 as i32;
    j = (::core::mem::size_of::<[u64; 21]>() as u64)
        .wrapping_div(::core::mem::size_of::<u64>() as u64) as i32;
    while i < j {
        if oldsize < sizes[i as usize] {
            newsize = sizes[i as usize];
            break;
        } else {
            i += 1;
            i;
        }
    }
    if newsize == oldsize {
        (*symbol).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*symbol).flags as u32 | flagvals::ARRAYMAXED as i32 as u32);
        return;
    }
    new = ezalloc_real(
        newsize.wrapping_mul(::core::mem::size_of::<*mut BUCKET>() as u64),
        b"grow_table\0" as *const u8 as *const i8,
        b"new\0" as *const u8 as *const i8,
        b"str_array.c\0" as *const u8 as *const i8,
        682 as i32,
    ) as *mut *mut BUCKET;
    old = (*symbol).sub.nodep.r.bv;
    (*symbol).sub.nodep.r.bv = new;
    (*symbol).sub.nodep.cnt = newsize;
    if old.is_null() {
        (*symbol).sub.nodep.reflags = reflagvals::from_libc_c_uint(0 as u32);
        return;
    }
    k = 0 as i32 as u64;
    while k < oldsize {
        chain = *old.offset(k as isize);
        while !chain.is_null() {
            next = (*chain).hs.next;
            hash1 = ((*chain).hs.code).wrapping_rem(newsize);
            (*chain).hs.next = *new.offset(hash1 as isize);
            let ref mut fresh18 = *new.offset(hash1 as isize);
            *fresh18 = chain;
            chain = next;
        }
        k = k.wrapping_add(1);
        k;
    }
    pma_free(old as *mut libc::c_void);
}
unsafe extern "C" fn gst_hash_string(
    mut str: *const i8,
    mut len: size_t,
    mut hsize: u64,
    mut code: *mut size_t,
) -> u64 {
    let mut hashVal: u64 = 1497032417 as i32 as u64;
    let mut ret: u64 = 0;
    loop {
        let fresh19 = len;
        len = len.wrapping_sub(1);
        if !(fresh19 != 0) {
            break;
        }
        let fresh20 = str;
        str = str.offset(1);
        hashVal = hashVal.wrapping_add(*fresh20 as u64);
        hashVal = hashVal.wrapping_add(hashVal << 10 as i32);
        hashVal ^= hashVal >> 6 as i32;
    }
    ret = scramble(hashVal);
    if !code.is_null() {
        *code = ret;
    }
    if ret >= hsize {
        ret = ret.wrapping_rem(hsize);
    }
    return ret;
}
unsafe extern "C" fn scramble(mut x: u64) -> u64 {
    if ::core::mem::size_of::<i64>() as u64 == 4 as i32 as u64 {
        let mut y: i32 = !x as i32;
        x = x.wrapping_add((y << 10 as i32 | y >> 22 as i32) as u64);
        x = x.wrapping_add(x << 6 as i32 | x >> 26 as i32);
        x = x.wrapping_sub(x << 16 as i32 | x >> 16 as i32);
    } else {
        x ^= !x >> 31 as i32;
        x = x.wrapping_add(x << 21 as i32 | x >> 11 as i32);
        x = x.wrapping_add(x << 5 as i32 | x >> 27 as i32);
        x = x.wrapping_add(x << 27 as i32 | x >> 5 as i32);
        x = x.wrapping_add(x << 31 as i32);
    }
    return x;
}
unsafe extern "C" fn fnv1a_hash_string(
    mut str: *const i8,
    mut len: size_t,
    mut hsize: u64,
    mut code: *mut size_t,
) -> u64 {
    let mut ret: u32 = 2166136261 as u32;
    while len > 0 as i32 as u64 {
        let fresh21 = str;
        str = str.offset(1);
        ret ^= *fresh21 as u8 as u32;
        ret = ret.wrapping_mul(16777619 as u32);
        len = len.wrapping_sub(1);
        len;
    }
    if !code.is_null() {
        *code = ret as size_t;
    }
    if ret as u64 >= hsize {
        ret = (ret as u64).wrapping_rem(hsize) as u32 as u32;
    }
    return ret as u64;
}
unsafe extern "C" fn env_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut val: *mut *mut NODE = str_remove(symbol, subs);
    let mut save: i8 = 0;
    if !val.is_null() {
        str_terminate_f(subs, &mut save);
        unsetenv((*subs).sub.val.sp);
        *((*subs).sub.val.sp).offset((*subs).sub.val.slen as isize) = save;
    }
    return val;
}
unsafe extern "C" fn env_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    extern "C" {
        #[link_name = "environ"]
        static mut environ_0: *mut *mut i8;
    }
    let mut val: *mut *mut NODE = str_clear(symbol, subs);
    environ = 0 as *mut *mut i8;
    (*symbol).sub.nodep.l.lp = &env_array_func;
    return val;
}
unsafe extern "C" fn env_store(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut val: *mut *mut NODE = str_exists(symbol, subs);
    let mut newval: *const i8 = 0 as *const i8;
    newval = (**val).sub.val.sp;
    if newval.is_null() {
        newval = b"\0" as *const u8 as *const i8;
    }
    setenv((*subs).sub.val.sp, newval, 1 as i32);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn init_env_array(mut env_node: *mut NODE) {
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
        return;
    }
    (*env_node).sub.nodep.l.lp = &env_array_func;
}