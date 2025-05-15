use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static str_array_func: array_funcs_t;
    fn r_unref(tmp: *mut NODE);
    fn assoc_info(subs: *mut NODE, val: *mut NODE, p: *mut NODE, aname: *const i8);
    fn make_aname(symbol: *const NODE) -> *const i8;
    static int_array_func: array_funcs_t;
    fn flags2str(_: i32) -> *const i8;
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn make_array() -> *mut NODE;
    fn assoc_copy(symbol: *mut NODE, newsymb: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn more_blocks(id: i32) -> *mut libc::c_void;
    static mut nextfree: [block_header; 2];
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    static mut success_node: *mut NODE;
    fn new_array_element() -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn getenv_long(name: *const i8) -> i64;
    static mut do_flags: do_flag_values;
    fn is_identchar(c: i32) -> bool;
    fn is_letter(c: i32) -> bool;
    static mut output_fp: *mut FILE;
    fn indent(indent_level: i32);
    fn is_integer(symbol: *mut NODE, subs: *mut NODE) -> *mut *mut NODE;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __uint32_t = u32;
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
pub type uint32_t = __uint32_t;
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
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
static mut NHAT: i32 = 10 as i32;
static mut THRESHOLD: i64 = 0;
#[no_mangle]
pub static mut cint_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"cint\0" as *const u8 as *const i8,
            init: Some(
                cint_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: Some(
                is_uinteger
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            lookup: Some(
                cint_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                cint_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                cint_clear
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                cint_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                cint_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                cint_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                cint_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
static mut argv_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"argv\0" as *const u8 as *const i8,
            init: Some(
                cint_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: Some(
                is_uinteger
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            lookup: Some(
                cint_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                cint_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                cint_clear
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                cint_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                cint_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                cint_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                cint_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: Some(
                argv_store
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
        };
        init
    }
};
static mut power_two_table: [i64; 31] = [
    1 as i32 as i64,
    2 as i32 as i64,
    4 as i32 as i64,
    8 as i32 as i64,
    16 as i32 as i64,
    32 as i32 as i64,
    64 as i32 as i64,
    128 as i32 as i64,
    256 as i32 as i64,
    512 as i32 as i64,
    1024 as i32 as i64,
    2048 as i32 as i64,
    4096 as i32 as i64,
    8192 as i32 as i64,
    16384 as i32 as i64,
    32768 as i32 as i64,
    65536 as i32 as i64,
    131072 as i32 as i64,
    262144 as i32 as i64,
    524288 as i32 as i64,
    1048576 as i32 as i64,
    2097152 as i32 as i64,
    4194304 as i32 as i64,
    8388608 as i32 as i64,
    16777216 as i32 as i64,
    33554432 as i32 as i64,
    67108864 as i32 as i64,
    134217728 as i32 as i64,
    268435456 as i32 as i64,
    536870912 as i32 as i64,
    1073741824 as i32 as i64,
];
unsafe extern "C" fn cint_array_init(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if symbol.is_null() {
        let mut newval: i64 = 0;
        let mut nelems: size_t = (::core::mem::size_of::<[i64; 31]>() as u64)
            .wrapping_div(::core::mem::size_of::<i64>() as u64);
        newval = getenv_long(b"NHAT\0" as *const u8 as *const i8);
        if newval > 1 as i32 as i64 && newval < 32 as i32 as i64 {
            NHAT = newval as i32;
        }
        if NHAT as u64 > nelems.wrapping_sub(2 as i32 as u64) {
            NHAT = nelems.wrapping_sub(2 as i32 as u64) as i32;
        }
        THRESHOLD = power_two_table[(NHAT + 1 as i32) as usize];
    } else {
        null_array(symbol);
    }
    return &mut success_node;
}
unsafe extern "C" fn is_uinteger(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if !(is_integer(symbol, subs)).is_null()
        && (*subs).sub.val.fltnum >= 0 as i32 as libc::c_double
    {
        return &mut success_node;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn cint_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut k: i64 = 0;
    let mut h1: i32 = -(1 as i32);
    let mut m: i32 = 0;
    let mut li: i32 = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut cint_size: i64 = 0;
    let mut capacity: i64 = 0;
    k = -(1 as i32) as i64;
    if ((*subs).flags as u32 & flagvals::NUMINT as i32 as u32 != 0 as i32 as u32
        || !(is_integer(symbol, subs)).is_null())
        && (*subs).sub.val.fltnum >= 0 as i32 as libc::c_double
    {
        k = (*subs).sub.val.fltnum as i64;
        h1 = cint_hash(k);
        lhs = cint_find(symbol, k, h1);
        if !lhs.is_null() {
            return lhs;
        }
    }
    xn = (*symbol).sub.nodep.rn;
    if !xn.is_null()
        && {
            lhs = ((*(*xn).sub.nodep.l.lp).exists)
                .expect("non-null function pointer")(xn, subs);
            !lhs.is_null()
        }
    {
        return lhs;
    }
    if !(k < 0 as i32 as i64) {
        m = h1 - 1 as i32;
        li = if m > NHAT { m } else { NHAT };
        while li >= NHAT {
            li = (li + 1 as i32) / 2 as i32;
        }
        capacity = ((*symbol).sub.nodep.reserved)
            .wrapping_add(power_two_table[li as usize] as u64) as i64;
        cint_size = (if xn.is_null() {
            (*symbol).sub.nodep.reflags as u32
        } else {
            ((*symbol).sub.nodep.reflags as u32)
                .wrapping_sub((*xn).sub.nodep.reflags as u32)
        }) as i64;
        if !(capacity - cint_size > THRESHOLD) {
            if ((*symbol).sub.nodep.r.av).is_null() {
                (*symbol).sub.nodep.reserved = 0 as i32 as size_t;
                (*symbol).sub.nodep.r.av = ezalloc_real(
                    (32 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
                    b"cint_lookup\0" as *const u8 as *const i8,
                    b"symbol->nodes\0" as *const u8 as *const i8,
                    b"cint_array.c\0" as *const u8 as *const i8,
                    249 as i32,
                ) as *mut *mut NODE;
            }
            (*symbol).sub.nodep.reflags += 1;
            (*symbol).sub.nodep.reflags;
            tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
            if tn.is_null() {
                tn = make_node(nodevals::Node_array_tree);
                let ref mut fresh0 = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
                *fresh0 = tn;
            }
            if m < NHAT {
                return tree_lookup(symbol, tn, k, NHAT, 0 as i32 as i64);
            }
            return tree_lookup(symbol, tn, k, m, power_two_table[m as usize]);
        }
    }
    (*symbol).sub.nodep.reflags += 1;
    (*symbol).sub.nodep.reflags;
    if xn.is_null() {
        (*symbol).sub.nodep.rn = make_array();
        xn = (*symbol).sub.nodep.rn;
        (*xn).sub.nodep.name = (*symbol).sub.nodep.name;
        if !(is_integer(xn, subs)).is_null() {
            (*xn).sub.nodep.l.lp = &int_array_func;
        } else {
            (*xn).sub.nodep.l.lp = &str_array_func;
        }
        (*xn).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*xn).flags as u32 | flagvals::XARRAY as i32 as u32);
    }
    return ((*(*xn).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(xn, subs);
}
unsafe extern "C" fn cint_exists(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut xn: *mut NODE = 0 as *mut NODE;
    if ((*subs).flags as u32 & flagvals::NUMINT as i32 as u32 != 0 as i32 as u32
        || !(is_integer(symbol, subs)).is_null())
        && (*subs).sub.val.fltnum >= 0 as i32 as libc::c_double
    {
        let mut k: i64 = (*subs).sub.val.fltnum as i64;
        let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
        lhs = cint_find(symbol, k, cint_hash(k));
        if !lhs.is_null() {
            return lhs;
        }
    }
    xn = (*symbol).sub.nodep.rn;
    if xn.is_null() {
        return 0 as *mut *mut NODE;
    }
    return ((*(*xn).sub.nodep.l.lp).exists)
        .expect("non-null function pointer")(xn, subs);
}
unsafe extern "C" fn cint_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: size_t = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    if !((*symbol).sub.nodep.rn).is_null() {
        let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
        ((*(*xn).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(xn, 0 as *mut exp_node);
        let ref mut fresh1 = (*(xn as *mut block_item)).freep;
        *fresh1 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = xn as *mut block_item;
        (*symbol).sub.nodep.rn = 0 as *mut exp_node;
    }
    i = NHAT as size_t;
    while i < 32 as i32 as u64 {
        tn = *((*symbol).sub.nodep.r.av).offset(i as isize);
        if !tn.is_null() {
            tree_clear(tn);
            let ref mut fresh2 = (*(tn as *mut block_item)).freep;
            *fresh2 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = tn as *mut block_item;
        }
        i = i.wrapping_add(1);
        i;
    }
    pma_free((*symbol).sub.nodep.r.av as *mut libc::c_void);
    ((*(*symbol).sub.nodep.l.lp).init)
        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn cint_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut k: i64 = 0;
    let mut h1: i32 = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
    if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        return 0 as *mut *mut NODE;
    }
    if ((*subs).flags as u32 & flagvals::NUMINT as i32 as u32 != 0 as i32 as u32
        || !(is_integer(symbol, subs)).is_null())
        && (*subs).sub.val.fltnum >= 0 as i32 as libc::c_double
    {
        k = (*subs).sub.val.fltnum as i64;
        h1 = cint_hash(k);
        tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
        if !(tn.is_null() || tree_remove(symbol, tn, k) == 0) {
            if (*tn).sub.nodep.reflags as u32 == 0 as i32 as u32 {
                let ref mut fresh3 = (*(tn as *mut block_item)).freep;
                *fresh3 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = tn
                    as *mut block_item;
                let ref mut fresh4 = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
                *fresh4 = 0 as *mut exp_node;
            }
            (*symbol).sub.nodep.reflags -= 1;
            (*symbol).sub.nodep.reflags;
            if xn.is_null() && (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
                pma_free((*symbol).sub.nodep.r.av as *mut libc::c_void);
                ((*(*symbol).sub.nodep.l.lp).init)
                    .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
            } else if !xn.is_null()
                && (*symbol).sub.nodep.reflags as u32 == (*xn).sub.nodep.reflags as u32
            {
                (*xn).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >((*xn).flags as u32 & !(flagvals::XARRAY as i32) as u32);
                (*xn).sub.nodep.x.extra = (*symbol).sub.nodep.x.extra;
                pma_free((*symbol).sub.nodep.r.av as *mut libc::c_void);
                *symbol = *xn;
                let ref mut fresh5 = (*(xn as *mut block_item)).freep;
                *fresh5 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = xn
                    as *mut block_item;
            }
            return &mut success_node;
        }
    }
    xn = (*symbol).sub.nodep.rn;
    if xn.is_null()
        || (((*(*xn).sub.nodep.l.lp).remove)
            .expect("non-null function pointer")(xn, subs))
            .is_null()
    {
        return 0 as *mut *mut NODE;
    }
    if (*xn).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        let ref mut fresh6 = (*(xn as *mut block_item)).freep;
        *fresh6 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = xn as *mut block_item;
        (*symbol).sub.nodep.rn = 0 as *mut exp_node;
    }
    (*symbol).sub.nodep.reflags -= 1;
    (*symbol).sub.nodep.reflags;
    return &mut success_node;
}
unsafe extern "C" fn cint_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut *mut NODE {
    let mut old: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut new: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: size_t = 0;
    new = ezalloc_real(
        (32 as i32 as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"cint_copy\0" as *const u8 as *const i8,
        b"new\0" as *const u8 as *const i8,
        b"cint_array.c\0" as *const u8 as *const i8,
        407 as i32,
    ) as *mut *mut NODE;
    old = (*symbol).sub.nodep.r.av;
    i = NHAT as size_t;
    while i < 32 as i32 as u64 {
        if !(*old.offset(i as isize)).is_null() {
            let ref mut fresh7 = *new.offset(i as isize);
            *fresh7 = make_node(nodevals::Node_array_tree);
            tree_copy(newsymb, *old.offset(i as isize), *new.offset(i as isize));
        }
        i = i.wrapping_add(1);
        i;
    }
    if !((*symbol).sub.nodep.rn).is_null() {
        let mut xn: *mut NODE = 0 as *mut NODE;
        let mut n: *mut NODE = 0 as *mut NODE;
        xn = (*symbol).sub.nodep.rn;
        n = make_array();
        (*n).sub.nodep.name = (*newsymb).sub.nodep.name;
        ((*(*xn).sub.nodep.l.lp).copy).expect("non-null function pointer")(xn, n);
        (*newsymb).sub.nodep.rn = n;
    } else {
        (*newsymb).sub.nodep.rn = 0 as *mut exp_node;
    }
    (*newsymb).sub.nodep.r.av = new;
    (*newsymb).sub.nodep.reflags = (*symbol).sub.nodep.reflags;
    (*newsymb).sub.nodep.reserved = (*symbol).sub.nodep.reserved;
    (*newsymb).flags = (*symbol).flags;
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn cint_list(
    mut symbol: *mut NODE,
    mut t: *mut NODE,
) -> *mut *mut NODE {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut k: u64 = 0 as i32 as u64;
    let mut num_elems: u64 = 0;
    let mut list_size: u64 = 0;
    let mut j: size_t = 0;
    let mut ja: size_t = 0;
    let mut jd: size_t = 0;
    let mut elem_size: i32 = 1 as i32;
    let mut assoc_kind: assoc_kind_t = assoc_kind_t::ANONE;
    num_elems = (*symbol).sub.nodep.reflags as u64;
    if num_elems == 0 as i32 as u64 {
        return 0 as *mut *mut NODE;
    }
    assoc_kind = (*t).flags as assoc_kind_t;
    if assoc_kind as u32
        & (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32
            | assoc_kind_t::ADELETE as i32) as u32
        == (assoc_kind_t::AINDEX as i32 | assoc_kind_t::ADELETE as i32) as u32
    {
        num_elems = 1 as i32 as u64;
    }
    if assoc_kind as u32
        & (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32
        == (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32
    {
        elem_size = 2 as i32;
    }
    list_size = num_elems.wrapping_mul(elem_size as u64);
    if !((*symbol).sub.nodep.rn).is_null() {
        xn = (*symbol).sub.nodep.rn;
        list = ((*(*xn).sub.nodep.l.lp).list).expect("non-null function pointer")(xn, t);
        assoc_kind = ::core::mem::transmute::<
            u32,
            assoc_kind_t,
        >(
            assoc_kind as u32
                & !(assoc_kind_t::AASC as i32 | assoc_kind_t::ADESC as i32) as u32,
        );
        (*t).flags = flagvals::from_libc_c_uint(assoc_kind as u32 as u32);
        if num_elems == 1 as i32 as u64 || num_elems == (*xn).sub.nodep.reflags as u64 {
            return list;
        }
        list = erealloc_real(
            list as *mut libc::c_void,
            list_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"cint_list\0" as *const u8 as *const i8,
            b"list\0" as *const u8 as *const i8,
            b"cint_array.c\0" as *const u8 as *const i8,
            467 as i32,
        ) as *mut *mut NODE;
        k = (elem_size as u32).wrapping_mul((*xn).sub.nodep.reflags as u32) as u64;
    } else {
        list = emalloc_real(
            list_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"cint_list\0" as *const u8 as *const i8,
            b"list\0" as *const u8 as *const i8,
            b"cint_array.c\0" as *const u8 as *const i8,
            470 as i32,
        ) as *mut *mut NODE;
    }
    if assoc_kind as u32 & assoc_kind_t::AINUM as i32 as u32 == 0 as i32 as u32 {
        assoc_kind = ::core::mem::transmute::<
            u32,
            assoc_kind_t,
        >(
            assoc_kind as u32
                & !(assoc_kind_t::AASC as i32 | assoc_kind_t::ADESC as i32) as u32,
        );
        (*t).flags = flagvals::from_libc_c_uint(assoc_kind as u32 as u32);
    }
    ja = NHAT as size_t;
    jd = (32 as i32 - 1 as i32) as size_t;
    while ja < 32 as i32 as u64 && jd >= NHAT as u64 {
        j = if assoc_kind as u32 & assoc_kind_t::ADESC as i32 as u32 != 0 as i32 as u32 {
            let fresh8 = jd;
            jd = jd.wrapping_sub(1);
            fresh8
        } else {
            let fresh9 = ja;
            ja = ja.wrapping_add(1);
            fresh9
        };
        tn = *((*symbol).sub.nodep.r.av).offset(j as isize);
        if tn.is_null() {
            continue;
        }
        k = k.wrapping_add(tree_list(tn, list.offset(k as isize), assoc_kind) as u64);
        if k >= list_size {
            return list;
        }
    }
    return list;
}
unsafe extern "C" fn cint_dump(
    mut symbol: *mut NODE,
    mut ndump: *mut NODE,
) -> *mut *mut NODE {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut indent_level: i32 = 0;
    let mut i: size_t = 0;
    let mut cint_size: i64 = 0 as i32 as i64;
    let mut xsize: i64 = 0 as i32 as i64;
    let mut kb: libc::c_double = 0 as i32 as libc::c_double;
    extern "C" {
        #[link_name = "int_kilobytes"]
        fn int_kilobytes_0(symbol_0: *mut NODE) -> libc::c_double;
    }
    extern "C" {
        #[link_name = "str_kilobytes"]
        fn str_kilobytes_0(symbol_0: *mut NODE) -> libc::c_double;
    }
    indent_level = (*ndump).sub.nodep.x.xl as i32;
    if !((*symbol).sub.nodep.rn).is_null() {
        xn = (*symbol).sub.nodep.rn;
        xsize = (*xn).sub.nodep.reflags as i64;
    }
    cint_size = (*symbol).sub.nodep.reflags as i64 - xsize;
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
    fprintf(output_fp, b"array_func: cint_array_func\n\0" as *const u8 as *const i8);
    if (*symbol).flags as u32 != 0 as i32 as u32 {
        indent(indent_level);
        fprintf(
            output_fp,
            b"flags: %s\n\0" as *const u8 as *const i8,
            flags2str((*symbol).flags as i32),
        );
    }
    indent(indent_level);
    fprintf(output_fp, b"NHAT: %d\n\0" as *const u8 as *const i8, NHAT);
    indent(indent_level);
    fprintf(output_fp, b"THRESHOLD: %ld\n\0" as *const u8 as *const i8, THRESHOLD);
    indent(indent_level);
    fprintf(
        output_fp,
        b"table_size: %lu (total), %ld (cint), %ld (int + str)\n\0" as *const u8
            as *const i8,
        (*symbol).sub.nodep.reflags as u64,
        cint_size,
        xsize,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_capacity: %lu\n\0" as *const u8 as *const i8,
        (*symbol).sub.nodep.reserved,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"Load Factor: %.2g\n\0" as *const u8 as *const i8,
        cint_size as libc::c_double / (*symbol).sub.nodep.reserved as libc::c_double,
    );
    i = NHAT as size_t;
    while i < 32 as i32 as u64 {
        tn = *((*symbol).sub.nodep.r.av).offset(i as isize);
        if !tn.is_null() {
            kb
                += (::core::mem::size_of::<NODE>() as u64)
                    .wrapping_add(tree_kilobytes(tn)) as libc::c_double / 1024.0f64;
        }
        i = i.wrapping_add(1);
        i;
    }
    kb
        += (32 as i32 as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64)
            as libc::c_double / 1024.0f64;
    kb
        += ((*symbol).sub.nodep.reserved)
            .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64) as libc::c_double
            / 1024.0f64;
    if !xn.is_null() {
        if (*xn).sub.nodep.l.lp == &int_array_func as *const array_funcs_t {
            kb += int_kilobytes_0(xn);
        } else {
            kb += str_kilobytes_0(xn);
        }
    }
    indent(indent_level);
    fprintf(output_fp, b"memory: %.2g kB (total)\n\0" as *const u8 as *const i8, kb);
    if (*ndump).sub.nodep.l.ll >= 0 as i32 as i64 {
        let mut aname: *const i8 = 0 as *const i8;
        fprintf(output_fp, b"\n\0" as *const u8 as *const i8);
        aname = make_aname(symbol);
        i = NHAT as size_t;
        while i < 32 as i32 as u64 {
            tn = *((*symbol).sub.nodep.r.av).offset(i as isize);
            if !tn.is_null() {
                tree_info(tn, ndump, aname);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if !xn.is_null() {
        fprintf(output_fp, b"\n\0" as *const u8 as *const i8);
        ((*(*xn).sub.nodep.l.lp).dump).expect("non-null function pointer")(xn, ndump);
    }
    return 0 as *mut *mut NODE;
}
#[inline]
unsafe extern "C" fn cint_hash(mut k: i64) -> i32 {
    let mut num: uint32_t = 0;
    let mut r: uint32_t = 0;
    let mut shift: uint32_t = 0;
    if k == 0 as i32 as i64 {
        return NHAT;
    }
    num = k as uint32_t;
    r = (((num > 0xffff as i32 as u32) as i32) << 4 as i32) as uint32_t;
    num >>= r;
    shift = (((num > 0xff as i32 as u32) as i32) << 3 as i32) as uint32_t;
    num >>= shift;
    r |= shift;
    shift = (((num > 0xf as i32 as u32) as i32) << 2 as i32) as uint32_t;
    num >>= shift;
    r |= shift;
    shift = (((num > 0x3 as i32 as u32) as i32) << 1 as i32) as uint32_t;
    num >>= shift;
    r |= shift;
    r |= num >> 1 as i32;
    if r < NHAT as u32 {
        return NHAT;
    }
    return (1 as i32 as u32).wrapping_add(r) as i32;
}
#[inline]
unsafe extern "C" fn cint_find(
    mut symbol: *mut NODE,
    mut k: i64,
    mut h1: i32,
) -> *mut *mut NODE {
    let mut tn: *mut NODE = 0 as *mut NODE;
    if ((*symbol).sub.nodep.r.av).is_null()
        || {
            tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
            tn.is_null()
        }
    {
        return 0 as *mut *mut NODE;
    }
    return tree_exists(tn, k);
}
#[inline]
unsafe extern "C" fn make_node(mut type_0: NODETYPE) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n as *mut block_item))
            .freep;
    } else {
        n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(n as *mut libc::c_void, '\0' as i32, ::core::mem::size_of::<NODE>() as u64);
    (*n).type_0 = type_0;
    return n;
}
unsafe extern "C" fn tree_lookup(
    mut symbol: *mut NODE,
    mut tree: *mut NODE,
    mut k: i64,
    mut m: i32,
    mut base: i64,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut size: size_t = 0;
    let mut num: i64 = k;
    n = (m + 1 as i32) / 2 as i32;
    if (*tree).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        let mut actual_size: size_t = 0;
        let mut table: *mut *mut NODE = 0 as *mut *mut NODE;
        actual_size = power_two_table[n as usize] as size_t;
        size = actual_size;
        (*tree).sub.nodep.l.ll = base;
        (*tree).sub.nodep.cnt = size;
        (*tree).sub.nodep.reflags = reflagvals::from_libc_c_uint(0 as u32);
        if n > m / 2 as i32 {
            actual_size = (actual_size as u64).wrapping_div(2 as i32 as u64) as size_t
                as size_t;
            (*tree).flags = ::core::mem::transmute::<
                u32,
                flagvals,
            >((*tree).flags as u32 | flagvals::HALFHAT as i32 as u32);
        }
        table = ezalloc_real(
            actual_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"tree_lookup\0" as *const u8 as *const i8,
            b"table\0" as *const u8 as *const i8,
            b"cint_array.c\0" as *const u8 as *const i8,
            773 as i32,
        ) as *mut *mut NODE;
        (*tree).sub.nodep.r.av = table;
    } else {
        size = (*tree).sub.nodep.cnt;
    }
    num -= (*tree).sub.nodep.l.ll;
    i = (num as u64).wrapping_div(size) as i32;
    lhs = tree_find(tree, k, i);
    if !lhs.is_null() {
        return lhs;
    }
    (*tree).sub.nodep.reflags += 1;
    (*tree).sub.nodep.reflags;
    base = (base as u64).wrapping_add(size.wrapping_mul(i as u64)) as i64 as i64;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if n > NHAT {
        if tn.is_null() {
            let ref mut fresh10 = *((*tree).sub.nodep.r.av).offset(i as isize);
            *fresh10 = make_node(nodevals::Node_array_tree);
            tn = *fresh10;
        }
        return tree_lookup(symbol, tn, k, n, base);
    } else {
        if tn.is_null() {
            let ref mut fresh11 = *((*tree).sub.nodep.r.av).offset(i as isize);
            *fresh11 = make_node(nodevals::Node_array_leaf);
            tn = *fresh11;
        }
        return leaf_lookup(symbol, tn, k, size as i64, base);
    };
}
unsafe extern "C" fn tree_exists(mut tree: *mut NODE, mut k: i64) -> *mut *mut NODE {
    let mut i: i32 = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    i = ((k - (*tree).sub.nodep.l.ll) as u64).wrapping_div((*tree).sub.nodep.cnt) as i32;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if tn.is_null() {
        return 0 as *mut *mut NODE;
    }
    if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32 {
        return tree_exists(tn, k);
    }
    return leaf_exists(tn, k);
}
unsafe extern "C" fn tree_clear(mut tree: *mut NODE) {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as u32 & flagvals::HALFHAT as i32 as u32 != 0 as i32 as u32 {
        hsize = (hsize as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
    }
    j = 0 as i32 as size_t;
    while j < hsize {
        tn = *((*tree).sub.nodep.r.av).offset(j as isize);
        if !tn.is_null() {
            if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32 {
                tree_clear(tn);
            } else {
                leaf_clear(tn);
            }
            let ref mut fresh12 = (*(tn as *mut block_item)).freep;
            *fresh12 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = tn as *mut block_item;
        }
        j = j.wrapping_add(1);
        j;
    }
    pma_free((*tree).sub.nodep.r.av as *mut libc::c_void);
    memset(
        tree as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    (*tree).type_0 = nodevals::Node_array_tree;
}
unsafe extern "C" fn tree_remove(
    mut symbol: *mut NODE,
    mut tree: *mut NODE,
    mut k: i64,
) -> i32 {
    let mut i: i32 = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    i = ((k - (*tree).sub.nodep.l.ll) as u64).wrapping_div((*tree).sub.nodep.cnt) as i32;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if tn.is_null() {
        return 0 as i32;
    }
    if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32
        && tree_remove(symbol, tn, k) == 0
    {
        return 0 as i32
    } else if (*tn).type_0 as u32 == nodevals::Node_array_leaf as i32 as u32
        && leaf_remove(symbol, tn, k) == 0
    {
        return 0 as i32
    }
    if (*tn).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        let ref mut fresh13 = (*(tn as *mut block_item)).freep;
        *fresh13 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = tn as *mut block_item;
        let ref mut fresh14 = *((*tree).sub.nodep.r.av).offset(i as isize);
        *fresh14 = 0 as *mut exp_node;
    }
    (*tree).sub.nodep.reflags -= 1;
    if (*tree).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        pma_free((*tree).sub.nodep.r.av as *mut libc::c_void);
        memset(
            tree as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<NODE>() as u64,
        );
        (*tree).type_0 = nodevals::Node_array_tree;
    }
    return 1 as i32;
}
#[inline]
unsafe extern "C" fn tree_find(
    mut tree: *mut NODE,
    mut k: i64,
    mut i: i32,
) -> *mut *mut NODE {
    let mut tn: *mut NODE = 0 as *mut NODE;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if !tn.is_null() {
        if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32 {
            return tree_exists(tn, k);
        }
        return leaf_exists(tn, k);
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn tree_list(
    mut tree: *mut NODE,
    mut list: *mut *mut NODE,
    mut assoc_kind: assoc_kind_t,
) -> i64 {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut cj: size_t = 0;
    let mut hsize: size_t = 0;
    let mut k: i64 = 0 as i32 as i64;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as u32 & flagvals::HALFHAT as i32 as u32 != 0 as i32 as u32 {
        hsize = (hsize as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
    }
    j = 0 as i32 as size_t;
    while j < hsize {
        cj = if assoc_kind as u32 & assoc_kind_t::ADESC as i32 as u32 != 0 as i32 as u32
        {
            hsize.wrapping_sub(1 as i32 as u64).wrapping_sub(j)
        } else {
            j
        };
        tn = *((*tree).sub.nodep.r.av).offset(cj as isize);
        if !tn.is_null() {
            if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32 {
                k += tree_list(tn, list.offset(k as isize), assoc_kind);
            } else {
                k += leaf_list(tn, list.offset(k as isize), assoc_kind);
            }
            if assoc_kind as u32 & assoc_kind_t::ADELETE as i32 as u32 != 0 as i32 as u32
                && k >= 1 as i32 as i64
            {
                return k;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    return k;
}
unsafe extern "C" fn tree_copy(
    mut newsymb: *mut NODE,
    mut tree: *mut NODE,
    mut newtree: *mut NODE,
) {
    let mut old: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut new: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as u32 & flagvals::HALFHAT as i32 as u32 != 0 as i32 as u32 {
        hsize = (hsize as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
    }
    new = ezalloc_real(
        hsize.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"tree_copy\0" as *const u8 as *const i8,
        b"new\0" as *const u8 as *const i8,
        b"cint_array.c\0" as *const u8 as *const i8,
        946 as i32,
    ) as *mut *mut NODE;
    (*newtree).sub.nodep.r.av = new;
    (*newtree).sub.nodep.l.ll = (*tree).sub.nodep.l.ll;
    (*newtree).sub.nodep.cnt = (*tree).sub.nodep.cnt;
    (*newtree).sub.nodep.reflags = (*tree).sub.nodep.reflags;
    (*newtree).flags = (*tree).flags;
    old = (*tree).sub.nodep.r.av;
    j = 0 as i32 as size_t;
    while j < hsize {
        if !(*old.offset(j as isize)).is_null() {
            if (**old.offset(j as isize)).type_0 as u32
                == nodevals::Node_array_tree as i32 as u32
            {
                let ref mut fresh15 = *new.offset(j as isize);
                *fresh15 = make_node(nodevals::Node_array_tree);
                tree_copy(newsymb, *old.offset(j as isize), *new.offset(j as isize));
            } else {
                let ref mut fresh16 = *new.offset(j as isize);
                *fresh16 = make_node(nodevals::Node_array_leaf);
                leaf_copy(newsymb, *old.offset(j as isize), *new.offset(j as isize));
            }
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn tree_info(
    mut tree: *mut NODE,
    mut ndump: *mut NODE,
    mut aname: *const i8,
) {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as u32 & flagvals::HALFHAT as i32 as u32 != 0 as i32 as u32 {
        hsize = (hsize as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
    }
    j = 0 as i32 as size_t;
    while j < hsize {
        tn = *((*tree).sub.nodep.r.av).offset(j as isize);
        if !tn.is_null() {
            if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32 {
                tree_info(tn, ndump, aname);
            } else {
                leaf_info(tn, ndump, aname);
            }
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn tree_kilobytes(mut tree: *mut NODE) -> size_t {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    let mut sz: size_t = 0 as i32 as size_t;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as u32 & flagvals::HALFHAT as i32 as u32 != 0 as i32 as u32 {
        hsize = (hsize as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
    }
    j = 0 as i32 as size_t;
    while j < hsize {
        tn = *((*tree).sub.nodep.r.av).offset(j as isize);
        if !tn.is_null() {
            sz = (sz as u64).wrapping_add(::core::mem::size_of::<NODE>() as u64)
                as size_t as size_t;
            if (*tn).type_0 as u32 == nodevals::Node_array_tree as i32 as u32 {
                sz = (sz as u64).wrapping_add(tree_kilobytes(tn)) as size_t as size_t;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    sz = (sz as u64)
        .wrapping_add(hsize.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64))
        as size_t as size_t;
    return sz;
}
#[inline]
unsafe extern "C" fn leaf_lookup(
    mut symbol: *mut NODE,
    mut array: *mut NODE,
    mut k: i64,
    mut size: i64,
    mut base: i64,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    if ((*array).sub.nodep.r.av).is_null() {
        (*array).sub.nodep.reflags = reflagvals::from_libc_c_uint(0 as u32);
        (*array).sub.nodep.cnt = size as u64;
        (*array).sub.nodep.l.ll = base;
        (*array).sub.nodep.r.av = ezalloc_real(
            (size as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"leaf_lookup\0" as *const u8 as *const i8,
            b"array->nodes\0" as *const u8 as *const i8,
            b"cint_array.c\0" as *const u8 as *const i8,
            1064 as i32,
        ) as *mut *mut NODE;
        (*symbol).sub.nodep.reserved = ((*symbol).sub.nodep.reserved as u64)
            .wrapping_add(size as u64) as size_t as size_t;
    }
    lhs = ((*array).sub.nodep.r.av).offset((k - base) as isize);
    if (*lhs).is_null() {
        (*array).sub.nodep.reflags += 1;
        (*array).sub.nodep.reflags;
        *lhs = new_array_element();
    }
    return lhs;
}
#[inline]
unsafe extern "C" fn leaf_exists(mut array: *mut NODE, mut k: i64) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    lhs = ((*array).sub.nodep.r.av).offset((k - (*array).sub.nodep.l.ll) as isize);
    return if !(*lhs).is_null() { lhs } else { 0 as *mut *mut NODE };
}
unsafe extern "C" fn leaf_clear(mut array: *mut NODE) {
    let mut i: i64 = 0;
    let mut size: i64 = (*array).sub.nodep.cnt as i64;
    let mut r: *mut NODE = 0 as *mut NODE;
    i = 0 as i32 as i64;
    while i < size {
        r = *((*array).sub.nodep.r.av).offset(i as isize);
        if !r.is_null() {
            if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                ((*(*r).sub.nodep.l.lp).clear)
                    .expect("non-null function pointer")(r, 0 as *mut exp_node);
                pma_free((*r).sub.nodep.name as *mut libc::c_void);
                let ref mut fresh17 = (*(r as *mut block_item)).freep;
                *fresh17 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r
                    as *mut block_item;
            } else {
                unref(r);
            }
        }
        i += 1;
        i;
    }
    pma_free((*array).sub.nodep.r.av as *mut libc::c_void);
    (*array).sub.nodep.r.av = 0 as *mut *mut exp_node;
    (*array).sub.nodep.reflags = reflagvals::from_libc_c_uint(0 as u32);
    (*array).sub.nodep.cnt = (*array).sub.nodep.reflags as u64;
}
unsafe extern "C" fn leaf_remove(
    mut symbol: *mut NODE,
    mut array: *mut NODE,
    mut k: i64,
) -> i32 {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    lhs = ((*array).sub.nodep.r.av).offset((k - (*array).sub.nodep.l.ll) as isize);
    if (*lhs).is_null() {
        return 0 as i32;
    }
    *lhs = 0 as *mut NODE;
    (*array).sub.nodep.reflags -= 1;
    if (*array).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        pma_free((*array).sub.nodep.r.av as *mut libc::c_void);
        (*array).sub.nodep.r.av = 0 as *mut *mut exp_node;
        (*symbol).sub.nodep.reserved = ((*symbol).sub.nodep.reserved as u64)
            .wrapping_sub((*array).sub.nodep.cnt) as size_t as size_t;
        (*array).sub.nodep.cnt = 0 as i32 as u64;
    }
    return 1 as i32;
}
unsafe extern "C" fn leaf_copy(
    mut newsymb: *mut NODE,
    mut array: *mut NODE,
    mut newarray: *mut NODE,
) {
    let mut old: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut new: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut size: i64 = 0;
    let mut i: i64 = 0;
    size = (*array).sub.nodep.cnt as i64;
    new = ezalloc_real(
        (size as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"leaf_copy\0" as *const u8 as *const i8,
        b"new\0" as *const u8 as *const i8,
        b"cint_array.c\0" as *const u8 as *const i8,
        1143 as i32,
    ) as *mut *mut NODE;
    (*newarray).sub.nodep.r.av = new;
    (*newarray).sub.nodep.cnt = size as u64;
    (*newarray).sub.nodep.l.ll = (*array).sub.nodep.l.ll;
    (*newarray).flags = (*array).flags;
    (*newarray).sub.nodep.reflags = (*array).sub.nodep.reflags;
    old = (*array).sub.nodep.r.av;
    i = 0 as i32 as i64;
    while i < size {
        if !(*old.offset(i as isize)).is_null() {
            if (**old.offset(i as isize)).type_0 as u32
                == nodevals::Node_val as i32 as u32
            {
                let ref mut fresh18 = *new.offset(i as isize);
                *fresh18 = dupnode(*old.offset(i as isize));
            } else {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = make_array();
                (*r).sub.nodep.name = estrdup(
                    (**old.offset(i as isize)).sub.nodep.name,
                    strlen((**old.offset(i as isize)).sub.nodep.name),
                );
                (*r).sub.nodep.x.extra = newsymb;
                let ref mut fresh19 = *new.offset(i as isize);
                *fresh19 = assoc_copy(*old.offset(i as isize), r);
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn leaf_list(
    mut array: *mut NODE,
    mut list: *mut *mut NODE,
    mut assoc_kind: assoc_kind_t,
) -> i64 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut num: i64 = 0;
    let mut i: i64 = 0;
    let mut ci: i64 = 0;
    let mut k: i64 = 0 as i32 as i64;
    let mut size: i64 = (*array).sub.nodep.cnt as i64;
    static mut buf: [i8; 100] = [0; 100];
    i = 0 as i32 as i64;
    while i < size {
        ci = if assoc_kind as u32 & assoc_kind_t::ADESC as i32 as u32 != 0 as i32 as u32
        {
            size - 1 as i32 as i64 - i
        } else {
            i
        };
        r = *((*array).sub.nodep.r.av).offset(ci as isize);
        if !r.is_null() {
            num = (*array).sub.nodep.l.ll + ci;
            if assoc_kind as u32 & assoc_kind_t::AISTR as i32 as u32 != 0 as i32 as u32 {
                sprintf(buf.as_mut_ptr(), b"%ld\0" as *const u8 as *const i8, num);
                subs = make_str_node(
                    buf.as_mut_ptr(),
                    strlen(buf.as_mut_ptr()),
                    0 as i32,
                );
                (*subs).sub.val.fltnum = num as libc::c_double;
                (*subs).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >(
                    (*subs).flags as u32
                        | (flagvals::NUMCUR as i32 | flagvals::NUMINT as i32) as u32,
                );
            } else {
                subs = make_number
                    .expect("non-null function pointer")(num as libc::c_double);
                (*subs).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >(
                    (*subs).flags as u32
                        | (flagvals::INTIND as i32 | flagvals::NUMINT as i32) as u32,
                );
            }
            let fresh20 = k;
            k = k + 1;
            let ref mut fresh21 = *list.offset(fresh20 as isize);
            *fresh21 = subs;
            if assoc_kind as u32 & assoc_kind_t::AVALUE as i32 as u32 != 0 as i32 as u32
            {
                if (*r).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    if assoc_kind as u32 & assoc_kind_t::AVNUM as i32 as u32
                        != 0 as i32 as u32
                    {
                        force_number(r);
                    } else if assoc_kind as u32 & assoc_kind_t::AVSTR as i32 as u32
                        != 0 as i32 as u32
                    {
                        r = force_string_fmt(r, CONVFMT, CONVFMTidx);
                    }
                }
                let fresh22 = k;
                k = k + 1;
                let ref mut fresh23 = *list.offset(fresh22 as isize);
                *fresh23 = r;
            }
            if assoc_kind as u32 & assoc_kind_t::ADELETE as i32 as u32 != 0 as i32 as u32
                && k >= 1 as i32 as i64
            {
                return k;
            }
        }
        i += 1;
        i;
    }
    return k;
}
unsafe extern "C" fn leaf_info(
    mut array: *mut NODE,
    mut ndump: *mut NODE,
    mut aname: *const i8,
) {
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut i: size_t = 0;
    let mut size: size_t = 0;
    size = (*array).sub.nodep.cnt;
    subs = make_number.expect("non-null function pointer")(0.0f64);
    (*subs).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*subs).flags as u32 | (flagvals::INTIND as i32 | flagvals::NUMINT as i32) as u32);
    i = 0 as i32 as size_t;
    while i < size {
        val = *((*array).sub.nodep.r.av).offset(i as isize);
        if !val.is_null() {
            (*subs).sub.val.fltnum = ((*array).sub.nodep.l.ll as u64).wrapping_add(i)
                as libc::c_double;
            assoc_info(subs, val, ndump, aname);
        }
        i = i.wrapping_add(1);
        i;
    }
    unref(subs);
}
static mut argv_shadow_array: *mut NODE = 0 as *const NODE as *mut NODE;
unsafe extern "C" fn argv_store(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut val: *mut *mut NODE = cint_exists(symbol, subs);
    let mut newval: *mut NODE = *val;
    let mut cp: *mut i8 = 0 as *mut i8;
    if (*newval).sub.val.slen == 0 as i32 as u64 {
        return val;
    }
    cp = strchr((*newval).sub.val.sp, '=' as i32);
    if cp.is_null() {
        if (in_array(argv_shadow_array, newval)).is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"cint_array.c\0" as *const u8 as *const i8, 1268 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot add a new file (%.*s) to ARGV in sandbox mode\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*newval).sub.val.slen as i32,
                (*newval).sub.val.sp,
            );
        }
    } else {
        let mut badvar: bool = 0 as i32 != 0;
        let mut arg: *mut i8 = (*newval).sub.val.sp;
        let mut cp2: *mut i8 = 0 as *mut i8;
        *cp = '\0' as i32 as i8;
        if !is_letter(*arg.offset(0 as i32 as isize) as u8 as i32) {
            badvar = 1 as i32 != 0;
        } else {
            cp2 = arg.offset(1 as i32 as isize);
            while *cp2 != 0 {
                if !is_identchar(*cp2 as u8 as i32) && *cp2 as i32 != ':' as i32 {
                    badvar = 1 as i32 != 0;
                    break;
                } else {
                    cp2 = cp2.offset(1);
                    cp2;
                }
            }
        }
        if !badvar {
            let mut cp_0: *mut i8 = strchr(arg, ':' as i32);
            if !cp_0.is_null()
                && (*cp_0.offset(1 as i32 as isize) as i32 != ':' as i32
                    || !(strchr(cp_0.offset(2 as i32 as isize), ':' as i32)).is_null())
            {
                badvar = 1 as i32 != 0;
            }
        }
        *cp = '=' as i32 as i8;
        if badvar as i32 != 0 && (in_array(argv_shadow_array, newval)).is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"cint_array.c\0" as *const u8 as *const i8, 1296 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot add a new file (%.*s) to ARGV in sandbox mode\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*newval).sub.val.slen as i32,
                (*newval).sub.val.sp,
            );
        }
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn init_argv_array(
    mut argv_node: *mut NODE,
    mut shadow_node: *mut NODE,
) {
    if do_flags as u32 & do_flag_values::DO_SANDBOX as i32 as u32 == 0 {
        return;
    }
    (*argv_node).sub.nodep.l.lp = &argv_array_func;
    argv_shadow_array = shadow_node;
}