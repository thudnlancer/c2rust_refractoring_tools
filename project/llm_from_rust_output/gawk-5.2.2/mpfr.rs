use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_double, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct re_pattern_buffer {
    pub buffer: *mut c_void,
    pub allocated: c_ulong,
    pub used: c_ulong,
    pub syntax: c_ulong,
    pub fastmap: *mut c_char,
    pub translate: *mut c_uchar,
    pub re_nsub: c_ulong,
    pub flags: u8,
    _padding: [u8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct re_registers {
    pub num_regs: c_uint,
    pub start: *mut c_int,
    pub end: *mut c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut c_void,
    pub has_meta: bool,
    pub maybe_long: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct awk_string {
    pub str_: *mut c_char,
    pub len: c_ulong,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AWK_NUMBER_TYPE {
    DOUBLE = 0,
    MPFR = 1,
    MPZ = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct awk_number {
    pub d: c_double,
    pub type_: AWK_NUMBER_TYPE,
    pub ptr: *mut c_void,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum awk_valtype_t {
    UNDEFINED = 0,
    NUMBER = 1,
    STRING = 2,
    REGEX = 3,
    STRNUM = 4,
    ARRAY = 5,
    SCALAR = 6,
    VALUE_COOKIE = 7,
    BOOL = 8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union awk_value_union {
    pub s: awk_string,
    pub n: awk_number,
    pub a: *mut c_void,
    pub scl: *mut c_void,
    pub vc: *mut c_void,
    pub b: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: awk_value_union,
}

pub type awk_ext_func_t = awk_ext_func;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct awk_ext_func {
    pub name: *const c_char,
    pub function: Option<
        extern "C" fn(
            c_int,
            *mut awk_value,
            *mut awk_ext_func,
        ) -> *mut awk_value,
    >,
    pub max_expected_args: c_ulong,
    pub min_required_args: c_ulong,
    pub suppress_lint: c_uint,
    pub data: *mut c_void,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NODETYPE {
    ILLEGAL = 0,
    VAL = 1,
    REGEX = 2,
    DYNREGEX = 3,
    VAR = 4,
    VAR_ARRAY = 5,
    VAR_NEW = 6,
    ELEM_NEW = 7,
    PARAM_LIST = 8,
    FUNC = 9,
    EXT_FUNC = 10,
    BUILTIN_FUNC = 11,
    ARRAY_REF = 12,
    ARRAY_TREE = 13,
    ARRAY_LEAF = 14,
    DUMP_ARRAY = 15,
    ARRAYFOR = 16,
    FRAME = 17,
    INSTRUCTION = 18,
    FINAL = 19,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct exp_node_val {
    pub fltnum: c_double,
    pub sp: *mut c_char,
    pub slen: c_ulong,
    pub idx: c_int,
    pub wsp: *mut c_int,
    pub wslen: c_ulong,
    pub typre: *mut exp_node,
    pub comtype: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct exp_node_nodep {
    pub l: *mut exp_node,
    pub r: *mut exp_node,
    pub x: *mut exp_node,
    pub name: *mut c_char,
    pub reserved: c_ulong,
    pub rn: *mut exp_node,
    pub cnt: c_ulong,
    pub reflags: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union exp_node_sub {
    pub nodep: exp_node_nodep,
    pub val: exp_node_val,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct exp_node {
    pub sub: exp_node_sub,
    pub type_: NODETYPE,
    pub flags: c_uint,
    pub valref: c_long,
}

pub fn set_PREC() {}
pub fn set_ROUNDMODE() {}
pub fn mpfr_unset(_n: *mut exp_node) {}