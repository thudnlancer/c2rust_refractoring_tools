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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const i8, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    static mut NF: i64;
    static mut IGNORECASE: bool;
    static mut RS_is_null: bool;
    static mut OFS: *mut i8;
    static mut OFSlen: i32;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut FIELDWIDTHS_node: *mut NODE;
    static mut FS_node: *mut NODE;
    static mut NF_node: *mut NODE;
    static mut RS_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut FPAT_node: *mut NODE;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: i32;
    static mut stack_ptr: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn check_symtab_functab(dest: *mut NODE, fname: *const i8, msg: *const i8);
    fn check_args_min_max(nargs: i32, fname: *const i8, min: i32, max: i32);
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn research(
        rp: *mut Regexp,
        str: *mut i8,
        start: i32,
        len: size_t,
        flags: i32,
    ) -> i32;
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn r_free_wstr(n: *mut NODE);
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    fn make_regexp(
        s: *const i8,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn refree(rp: *mut Regexp);
}
pub type size_t = u64;
pub type wchar_t = i32;
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
pub type mbstate_t = __mbstate_t;
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
pub type Func_ptr = Option<unsafe extern "C" fn() -> ()>;
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
pub type Setfunc = Option<unsafe extern "C" fn(i64, *mut i8, i64, *mut NODE) -> ()>;
pub type parse_field_func_t = Option<
    unsafe extern "C" fn(
        i64,
        *mut *mut i8,
        i32,
        *mut NODE,
        *mut Regexp,
        Setfunc,
        *mut NODE,
        *mut NODE,
        bool,
    ) -> i64,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum field_sep_type {
    Using_FS,
    Using_FIELDWIDTHS,
    Using_FPAT,
    Using_API,
}
impl field_sep_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            field_sep_type::Using_FS => 0,
            field_sep_type::Using_FIELDWIDTHS => 1,
            field_sep_type::Using_FPAT => 2,
            field_sep_type::Using_API => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> field_sep_type {
        match value {
            0 => field_sep_type::Using_FS,
            1 => field_sep_type::Using_FIELDWIDTHS,
            2 => field_sep_type::Using_FPAT,
            3 => field_sep_type::Using_API,
            _ => panic!("Invalid value for field_sep_type: {}", value),
        }
    }
}
impl AddAssign<u32> for field_sep_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for field_sep_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for field_sep_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for field_sep_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for field_sep_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for field_sep_type {
    type Output = field_sep_type;
    fn add(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for field_sep_type {
    type Output = field_sep_type;
    fn sub(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for field_sep_type {
    type Output = field_sep_type;
    fn mul(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for field_sep_type {
    type Output = field_sep_type;
    fn div(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for field_sep_type {
    type Output = field_sep_type;
    fn rem(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
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
unsafe extern "C" fn is_blank(mut c: i32) -> i32 {
    return (c == ' ' as i32 || c == '\t' as i32) as i32;
}
static mut api_parser_override: bool = 0 as i32 != 0;
static mut parse_field: parse_field_func_t = None;
static mut normal_parse_field: parse_field_func_t = None;
static mut api_fw: *const awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t;
static mut parse_extent: *mut i8 = 0 as *const i8 as *mut i8;
static mut parse_high_water: i64 = 0 as i32 as i64;
static mut nf_high_water: i64 = 0 as i32 as i64;
static mut resave_fs: bool = false;
static mut save_FS: *mut NODE = 0 as *const NODE as *mut NODE;
static mut save_FPAT: *mut NODE = 0 as *const NODE as *mut NODE;
static mut FIELDWIDTHS: *mut awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t
    as *mut awk_fieldwidth_info_t;
#[no_mangle]
pub static mut fields_arr: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
#[no_mangle]
pub static mut field0_valid: bool = false;
static mut default_FS: i32 = 0;
static mut FS_re_yes_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FS_re_no_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FS_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_re_yes_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_re_no_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
#[no_mangle]
pub static mut Null_field: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub unsafe extern "C" fn init_fields() {
    fields_arr = emalloc_real(
        ::core::mem::size_of::<*mut NODE>() as u64,
        b"init_fields\0" as *const u8 as *const i8,
        b"fields_arr\0" as *const u8 as *const i8,
        b"field.c\0" as *const u8 as *const i8,
        100 as i32,
    ) as *mut *mut NODE;
    let ref mut fresh2 = *fields_arr.offset(0 as i32 as isize);
    *fresh2 = make_str_node(
        b"\0" as *const u8 as *const i8,
        0 as i32 as size_t,
        0 as i32,
    );
    let ref mut fresh3 = (**fields_arr.offset(0 as i32 as isize)).flags;
    *fresh3 = ::core::mem::transmute::<
        u32,
        flagvals,
    >(*fresh3 as u32 | flagvals::NULL_FIELD as i32 as u32);
    parse_extent = (**fields_arr.offset(0 as i32 as isize)).sub.val.sp;
    save_FS = dupnode((*FS_node).sub.nodep.l.lptr);
    Null_field = make_str_node(
        b"\0" as *const u8 as *const i8,
        0 as i32 as size_t,
        0 as i32,
    );
    (*Null_field).flags = flagvals::from_libc_c_uint(
        (flagvals::STRCUR as i32 | flagvals::STRING as i32 | flagvals::NULL_FIELD as i32)
            as u32,
    );
    field0_valid = 1 as i32 != 0;
}
unsafe extern "C" fn grow_fields_arr(mut num: i64) {
    let mut t: i32 = 0;
    let mut n: *mut NODE = 0 as *mut NODE;
    fields_arr = erealloc_real(
        fields_arr as *mut libc::c_void,
        ((num + 1 as i32 as i64) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"grow_fields_arr\0" as *const u8 as *const i8,
        b"fields_arr\0" as *const u8 as *const i8,
        b"field.c\0" as *const u8 as *const i8,
        122 as i32,
    ) as *mut *mut NODE;
    t = (nf_high_water + 1 as i32 as i64) as i32;
    while t as i64 <= num {
        n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
        if !n.is_null() {
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n
                as *mut block_item))
                .freep;
        } else {
            n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
        };
        *n = *Null_field;
        let ref mut fresh4 = *fields_arr.offset(t as isize);
        *fresh4 = n;
        t += 1;
        t;
    }
    nf_high_water = num;
}
unsafe extern "C" fn set_field(
    mut num: i64,
    mut str: *mut i8,
    mut len: i64,
    mut dummy: *mut NODE,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    if num > nf_high_water {
        grow_fields_arr(num);
    }
    n = *fields_arr.offset(num as isize);
    (*n).sub.val.sp = str;
    (*n).sub.val.slen = len as size_t;
    (*n).flags = flagvals::from_libc_c_uint(
        (flagvals::STRCUR as i32 | flagvals::STRING as i32 | flagvals::USER_INPUT as i32)
            as u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rebuild_record() {
    let mut tlen: u64 = 0;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ops: *mut i8 = 0 as *mut i8;
    let mut cops: *mut i8 = 0 as *mut i8;
    let mut i: i64 = 0;
    tlen = 0 as i32 as u64;
    i = NF;
    while i > 0 as i32 as i64 {
        tmp = *fields_arr.offset(i as isize);
        tmp = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        tlen = tlen.wrapping_add((*tmp).sub.val.slen);
        i -= 1;
        i;
    }
    tlen = tlen.wrapping_add(((NF - 1 as i32 as i64) * OFSlen as i64) as u64);
    if (tlen as i64) < 0 as i32 as i64 {
        tlen = 0 as i32 as u64;
    }
    ops = emalloc_real(
        tlen.wrapping_add(1 as i32 as u64),
        b"rebuild_record\0" as *const u8 as *const i8,
        b"ops\0" as *const u8 as *const i8,
        b"field.c\0" as *const u8 as *const i8,
        177 as i32,
    ) as *mut i8;
    cops = ops;
    *ops.offset(0 as i32 as isize) = '\0' as i32 as i8;
    i = 1 as i32 as i64;
    while i <= NF {
        if (**fields_arr.offset(i as isize)).flags as u32
            & flagvals::WSTRCUR as i32 as u32 != 0
        {
            r_free_wstr(*fields_arr.offset(i as isize));
        }
        tmp = *fields_arr.offset(i as isize);
        if (*tmp).sub.val.slen == 1 as i32 as u64 {
            let fresh5 = cops;
            cops = cops.offset(1);
            *fresh5 = *((*tmp).sub.val.sp).offset(0 as i32 as isize);
        } else if (*tmp).sub.val.slen != 0 as i32 as u64 {
            memcpy(
                cops as *mut libc::c_void,
                (*tmp).sub.val.sp as *const libc::c_void,
                (*tmp).sub.val.slen,
            );
            cops = cops.offset((*tmp).sub.val.slen as isize);
        }
        if i != NF {
            if OFSlen == 1 as i32 {
                let fresh6 = cops;
                cops = cops.offset(1);
                *fresh6 = *OFS;
            } else if OFSlen != 0 as i32 {
                memcpy(
                    cops as *mut libc::c_void,
                    OFS as *const libc::c_void,
                    OFSlen as u64,
                );
                cops = cops.offset(OFSlen as isize);
            }
        }
        i += 1;
        i;
    }
    tmp = make_str_node(ops, tlen, 2 as i32);
    cops = ops;
    i = 1 as i32 as i64;
    while i <= NF {
        let mut r: *mut NODE = *fields_arr.offset(i as isize);
        if (*r).sub.val.slen > 0 as i32 as u64
            && (*r).flags as u32 & flagvals::MALLOC as i32 as u32 == 0 as i32 as u32
        {
            let mut n: *mut NODE = 0 as *mut NODE;
            n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n
                    as *mut block_item))
                    .freep;
            } else {
                n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
            };
            *n = *r;
            if (*r).valref > 1 as i32 as i64 {
                (*r).sub.val.sp = emalloc_real(
                    ((*r).sub.val.slen).wrapping_add(1 as i32 as u64),
                    b"rebuild_record\0" as *const u8 as *const i8,
                    b"r->stptr\0" as *const u8 as *const i8,
                    b"field.c\0" as *const u8 as *const i8,
                    226 as i32,
                ) as *mut i8;
                memcpy(
                    (*r).sub.val.sp as *mut libc::c_void,
                    cops as *const libc::c_void,
                    (*r).sub.val.slen,
                );
                *((*r).sub.val.sp).offset((*r).sub.val.slen as isize) = '\0' as i32
                    as i8;
                (*r).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >((*r).flags as u32 | flagvals::MALLOC as i32 as u32);
                (*n).valref = 1 as i32 as i64;
            }
            (*n).sub.val.sp = cops;
            (*n).flags = ::core::mem::transmute::<
                u32,
                flagvals,
            >(
                (*n).flags as u32
                    & !(flagvals::MPFN as i32 | flagvals::MPZN as i32
                        | flagvals::NUMCUR as i32) as u32,
            );
            unref(r);
            let ref mut fresh7 = *fields_arr.offset(i as isize);
            *fresh7 = n;
        }
        cops = cops
            .offset(
                ((**fields_arr.offset(i as isize)).sub.val.slen)
                    .wrapping_add(OFSlen as u64) as isize,
            );
        i += 1;
        i;
    }
    unref(*fields_arr.offset(0 as i32 as isize));
    let ref mut fresh8 = *fields_arr.offset(0 as i32 as isize);
    *fresh8 = tmp;
    field0_valid = 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_record(
    mut buf: *const i8,
    mut cnt: size_t,
    mut fw: *const awk_fieldwidth_info_t,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    static mut databuf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut databuf_size: size_t = 0;
    purge_record();
    if databuf_size == 0 as i32 as u64 {
        databuf = ezalloc_real(
            512 as i32 as size_t,
            b"set_record\0" as *const u8 as *const i8,
            b"databuf\0" as *const u8 as *const i8,
            b"field.c\0" as *const u8 as *const i8,
            276 as i32,
        ) as *mut i8;
        databuf_size = 512 as i32 as size_t;
    }
    if cnt >= databuf_size {
        loop {
            if databuf_size > (!(0 as i32) as size_t).wrapping_div(2 as i32 as u64) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"field.c\0" as *const u8 as *const i8, 287 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"input record too large\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            databuf_size = (databuf_size as u64).wrapping_mul(2 as i32 as u64) as size_t
                as size_t;
            if !(cnt >= databuf_size) {
                break;
            }
        }
        databuf = erealloc_real(
            databuf as *mut libc::c_void,
            databuf_size,
            b"set_record\0" as *const u8 as *const i8,
            b"databuf\0" as *const u8 as *const i8,
            b"field.c\0" as *const u8 as *const i8,
            290 as i32,
        ) as *mut i8;
        memset(databuf as *mut libc::c_void, '\0' as i32, databuf_size);
    }
    if cnt != 0 as i32 as u64 {
        memcpy(databuf as *mut libc::c_void, buf as *const libc::c_void, cnt);
    }
    *databuf.offset(cnt as isize) = '\0' as i32 as i8;
    unref(*fields_arr.offset(0 as i32 as isize));
    n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n as *mut block_item))
            .freep;
    } else {
        n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*n).sub.val.sp = databuf;
    (*n).sub.val.slen = cnt;
    (*n).valref = 1 as i32 as i64;
    (*n).type_0 = nodevals::Node_val;
    (*n).sub.val.idx = -(1 as i32);
    (*n).flags = flagvals::from_libc_c_uint(
        (flagvals::STRING as i32 | flagvals::STRCUR as i32 | flagvals::USER_INPUT as i32)
            as u32,
    );
    let ref mut fresh9 = *fields_arr.offset(0 as i32 as isize);
    *fresh9 = n;
    if fw != api_fw {
        api_fw = fw;
        if !api_fw.is_null() {
            if !api_parser_override {
                api_parser_override = 1 as i32 != 0;
                parse_field = Some(
                    fw_parse_field
                        as unsafe extern "C" fn(
                            i64,
                            *mut *mut i8,
                            i32,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> i64,
                );
                update_PROCINFO_str(
                    b"FS\0" as *const u8 as *const i8,
                    b"API\0" as *const u8 as *const i8,
                );
            }
        } else if api_parser_override {
            api_parser_override = 0 as i32 != 0;
            parse_field = normal_parse_field;
            update_PROCINFO_str(
                b"FS\0" as *const u8 as *const i8,
                current_field_sep_str(),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn reset_record() {
    let ref mut fresh10 = *fields_arr.offset(0 as i32 as isize);
    *fresh10 = force_string_fmt(
        *fields_arr.offset(0 as i32 as isize),
        CONVFMT,
        CONVFMTidx,
    );
    purge_record();
    if api_parser_override {
        api_parser_override = 0 as i32 != 0;
        parse_field = normal_parse_field;
        update_PROCINFO_str(b"FS\0" as *const u8 as *const i8, current_field_sep_str());
    }
}
unsafe extern "C" fn purge_record() {
    let mut i: i32 = 0;
    NF = -(1 as i32) as i64;
    i = 1 as i32;
    while i as i64 <= parse_high_water {
        let mut n: *mut NODE = 0 as *mut NODE;
        let mut r: *mut NODE = *fields_arr.offset(i as isize);
        if (*r).flags as u32 & flagvals::MALLOC as i32 as u32 == 0 as i32 as u32
            && (*r).valref > 1 as i32 as i64
        {
            let mut save: *const i8 = (*r).sub.val.sp;
            (*r).sub.val.sp = emalloc_real(
                ((*r).sub.val.slen).wrapping_add(1 as i32 as u64),
                b"purge_record\0" as *const u8 as *const i8,
                b"r->stptr\0" as *const u8 as *const i8,
                b"field.c\0" as *const u8 as *const i8,
                370 as i32,
            ) as *mut i8;
            memcpy(
                (*r).sub.val.sp as *mut libc::c_void,
                save as *const libc::c_void,
                (*r).sub.val.slen,
            );
            *((*r).sub.val.sp).offset((*r).sub.val.slen as isize) = '\0' as i32 as i8;
            (*r).flags = ::core::mem::transmute::<
                u32,
                flagvals,
            >((*r).flags as u32 | flagvals::MALLOC as i32 as u32);
        }
        unref(r);
        n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
        if !n.is_null() {
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n
                as *mut block_item))
                .freep;
        } else {
            n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
        };
        *n = *Null_field;
        let ref mut fresh11 = *fields_arr.offset(i as isize);
        *fresh11 = n;
        i += 1;
        i;
    }
    parse_high_water = 0 as i32 as i64;
    if resave_fs {
        resave_fs = 0 as i32 != 0;
        unref(save_FS);
        save_FS = dupnode((*FS_node).sub.nodep.l.lptr);
    }
    field0_valid = 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_NF() {
    let mut i: i32 = 0;
    let mut nf: i64 = 0;
    let mut n: *mut NODE = 0 as *mut NODE;
    force_number((*NF_node).sub.nodep.l.lptr);
    nf = (*(*NF_node).sub.nodep.l.lptr).sub.val.fltnum as i64;
    if nf < 0 as i32 as i64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 408 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"NF set to negative value\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    static mut warned: bool = 0 as i32 != 0;
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && NF > nf && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 413 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"decrementing NF is not portable to many awk versions\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    NF = nf;
    if NF > nf_high_water {
        grow_fields_arr(NF);
    }
    if parse_high_water < NF {
        i = (parse_high_water + 1 as i32 as i64) as i32;
        while i >= 0 as i32 && i as i64 <= NF {
            unref(*fields_arr.offset(i as isize));
            n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n
                    as *mut block_item))
                    .freep;
            } else {
                n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
            };
            *n = *Null_field;
            let ref mut fresh12 = *fields_arr.offset(i as isize);
            *fresh12 = n;
            i += 1;
            i;
        }
        parse_high_water = NF;
    } else if parse_high_water > 0 as i32 as i64 {
        i = (NF + 1 as i32 as i64) as i32;
        while i >= 0 as i32 && i as i64 <= parse_high_water {
            unref(*fields_arr.offset(i as isize));
            n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n
                    as *mut block_item))
                    .freep;
            } else {
                n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
            };
            *n = *Null_field;
            let ref mut fresh13 = *fields_arr.offset(i as isize);
            *fresh13 = n;
            i += 1;
            i;
        }
        parse_high_water = NF;
    }
    field0_valid = 0 as i32 != 0;
}
unsafe extern "C" fn re_parse_field(
    mut up_to: i64,
    mut buf: *mut *mut i8,
    mut len: i32,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> i64 {
    let mut scan: *mut i8 = *buf;
    let mut nf: i64 = parse_high_water;
    let mut field: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = scan.offset(len as isize);
    let mut regex_flags: i32 = 1 as i32;
    let mut sep: *mut i8 = 0 as *mut i8;
    let mut mbclen: size_t = 0 as i32 as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    if in_middle {
        regex_flags |= 2 as i32;
    }
    if up_to == 9223372036854775807 as i64 {
        nf = 0 as i32 as i64;
    }
    if len == 0 as i32 {
        return nf;
    }
    let mut default_field_splitting: bool = RS_is_null as i32 != 0 && default_FS != 0;
    if default_field_splitting {
        sep = scan;
        while scan < end
            && (*scan as i32 == ' ' as i32 || *scan as i32 == '\t' as i32
                || *scan as i32 == '\n' as i32)
        {
            scan = scan.offset(1);
            scan;
        }
        if !sep_arr.is_null() && sep < scan {
            set_element(nf, sep, scan.offset_from(sep) as i64, sep_arr);
        }
    }
    if rp.is_null() {
        rp = FS_regexp;
    }
    field = scan;
    while scan < end
        && research(
            rp,
            scan,
            0 as i32,
            end.offset_from(scan) as i64 as size_t,
            regex_flags,
        ) != -(1 as i32) && nf < up_to
    {
        regex_flags |= 2 as i32;
        if *((*rp).regs.end).offset(0 as i32 as isize)
            == *((*rp).regs.start).offset(0 as i32 as isize)
        {
            if gawk_mb_cur_max > 1 as i32 {
                mbclen = mbrlen(scan, end.offset_from(scan) as i64 as size_t, &mut mbs);
                if mbclen == 1 as i32 as u64 || mbclen == -(1 as i32) as size_t
                    || mbclen == -(2 as i32) as size_t || mbclen == 0 as i32 as u64
                {
                    mbclen = 1 as i32 as size_t;
                }
                scan = scan.offset(mbclen as isize);
            } else {
                scan = scan.offset(1);
                scan;
            }
            if !(scan == end) {
                continue;
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(nf, field, scan.offset_from(field) as i64, n);
            up_to = nf;
            break;
        } else {
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                nf,
                field,
                scan
                    .offset(*((*rp).regs.start).offset(0 as i32 as isize) as isize)
                    .offset_from(field) as i64,
                n,
            );
            if !sep_arr.is_null() {
                set_element(
                    nf,
                    scan.offset(*((*rp).regs.start).offset(0 as i32 as isize) as isize),
                    (*((*rp).regs.end).offset(0 as i32 as isize)
                        - *((*rp).regs.start).offset(0 as i32 as isize)) as i64,
                    sep_arr,
                );
            }
            scan = scan.offset(*((*rp).regs.end).offset(0 as i32 as isize) as isize);
            field = scan;
            if scan == end && !default_field_splitting {
                nf += 1;
                (Some(set.expect("non-null function pointer")))
                    .expect("non-null function pointer")(nf, field, 0 as i64, n);
            }
        }
    }
    if nf != up_to && scan < end {
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nf, scan, end.offset_from(scan) as i64, n);
        scan = end;
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn def_parse_field(
    mut up_to: i64,
    mut buf: *mut *mut i8,
    mut len: i32,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> i64 {
    let mut scan: *mut i8 = *buf;
    let mut nf: i64 = parse_high_water;
    let mut field: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = scan.offset(len as isize);
    let mut sav: i8 = 0;
    let mut sep: *mut i8 = 0 as *mut i8;
    if up_to == 9223372036854775807 as i64 {
        nf = 0 as i32 as i64;
    }
    if len == 0 as i32 {
        return nf;
    }
    if (*fs).sub.val.slen == 0 as i32 as u64 {
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect("non-null function pointer")(nf, *buf, len as i64, n);
        *buf = (*buf).offset(len as isize);
        return nf;
    }
    sav = *end;
    *end = ' ' as i32 as i8;
    sep = scan;
    while nf < up_to {
        while scan < end
            && (*scan as i32 == ' ' as i32 || *scan as i32 == '\t' as i32
                || *scan as i32 == '\n' as i32)
        {
            scan = scan.offset(1);
            scan;
        }
        if !sep_arr.is_null() && scan > sep {
            set_element(nf, sep, scan.offset_from(sep) as i64, sep_arr);
        }
        if scan >= end {
            break;
        }
        field = scan;
        while *scan as i32 != ' ' as i32 && *scan as i32 != '\t' as i32
            && *scan as i32 != '\n' as i32
        {
            scan = scan.offset(1);
            scan;
        }
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nf, field, scan.offset_from(field) as i64, n);
        if scan == end {
            break;
        }
        sep = scan;
        scan = scan.offset(1);
        scan;
    }
    *end = sav;
    *buf = scan;
    return nf;
}
unsafe extern "C" fn null_parse_field(
    mut up_to: i64,
    mut buf: *mut *mut i8,
    mut len: i32,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> i64 {
    let mut scan: *mut i8 = *buf;
    let mut nf: i64 = parse_high_water;
    let mut end: *mut i8 = scan.offset(len as isize);
    if up_to == 9223372036854775807 as i64 {
        nf = 0 as i32 as i64;
    }
    if len == 0 as i32 {
        return nf;
    }
    if gawk_mb_cur_max > 1 as i32 {
        let mut mbs: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        while nf < up_to && scan < end {
            let mut mbclen: size_t = mbrlen(
                scan,
                end.offset_from(scan) as i64 as size_t,
                &mut mbs,
            );
            if mbclen == 1 as i32 as u64 || mbclen == -(1 as i32) as size_t
                || mbclen == -(2 as i32) as size_t || mbclen == 0 as i32 as u64
            {
                mbclen = 1 as i32 as size_t;
            }
            if !sep_arr.is_null() && nf > 0 as i32 as i64 {
                set_element(nf, scan, 0 as i64, sep_arr);
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, mbclen as i64, n);
            scan = scan.offset(mbclen as isize);
        }
    } else {
        while nf < up_to && scan < end {
            if !sep_arr.is_null() && nf > 0 as i32 as i64 {
                set_element(nf, scan, 0 as i64, sep_arr);
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, 1 as i64, n);
            scan = scan.offset(1);
            scan;
        }
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn sc_parse_field(
    mut up_to: i64,
    mut buf: *mut *mut i8,
    mut len: i32,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> i64 {
    let mut scan: *mut i8 = *buf;
    let mut fschar: i8 = 0;
    let mut nf: i64 = parse_high_water;
    let mut field: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = scan.offset(len as isize);
    let mut sav: i8 = 0;
    let mut mbclen: size_t = 0 as i32 as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    if up_to == 9223372036854775807 as i64 {
        nf = 0 as i32 as i64;
    }
    if len == 0 as i32 {
        return nf;
    }
    if RS_is_null as i32 != 0 && (*fs).sub.val.slen == 0 as i32 as u64 {
        fschar = '\n' as i32 as i8;
    } else {
        fschar = *((*fs).sub.val.sp).offset(0 as i32 as isize);
    }
    sav = *end;
    *end = fschar;
    while nf < up_to {
        field = scan;
        if gawk_mb_cur_max > 1 as i32 {
            while *scan as i32 != fschar as i32 {
                mbclen = mbrlen(scan, end.offset_from(scan) as i64 as size_t, &mut mbs);
                if mbclen == 1 as i32 as u64 || mbclen == -(1 as i32) as size_t
                    || mbclen == -(2 as i32) as size_t || mbclen == 0 as i32 as u64
                {
                    mbclen = 1 as i32 as size_t;
                }
                scan = scan.offset(mbclen as isize);
            }
        } else {
            while *scan as i32 != fschar as i32 {
                scan = scan.offset(1);
                scan;
            }
        }
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nf, field, scan.offset_from(field) as i64, n);
        if scan == end {
            break;
        }
        if !sep_arr.is_null() {
            set_element(nf, scan, 1 as i64, sep_arr);
        }
        scan = scan.offset(1);
        scan;
        if !(scan == end) {
            continue;
        }
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect("non-null function pointer")(nf, field, 0 as i64, n);
        break;
    }
    *end = sav;
    *buf = scan;
    return nf;
}
unsafe extern "C" fn calc_mbslen(
    mut scan: *mut i8,
    mut end: *mut i8,
    mut len: size_t,
    mut mbs: *mut mbstate_t,
) -> size_t {
    let mut mbclen: size_t = 0;
    let mut mbscan: *mut i8 = scan;
    loop {
        let fresh14 = len;
        len = len.wrapping_sub(1);
        if !(fresh14 > 0 as i32 as u64 && mbscan < end) {
            break;
        }
        mbclen = mbrlen(mbscan, end.offset_from(mbscan) as i64 as size_t, mbs);
        if !(mbclen > 0 as i32 as u64
            && mbclen <= end.offset_from(mbscan) as i64 as size_t)
        {
            mbclen = 1 as i32 as size_t;
        }
        mbscan = mbscan.offset(mbclen as isize);
    }
    return mbscan.offset_from(scan) as i64 as size_t;
}
unsafe extern "C" fn fw_parse_field(
    mut up_to: i64,
    mut buf: *mut *mut i8,
    mut len: i32,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut dummy: *mut NODE,
    mut in_middle: bool,
) -> i64 {
    let mut scan: *mut i8 = *buf;
    let mut nf: i64 = parse_high_water;
    let mut end: *mut i8 = scan.offset(len as isize);
    let mut fw: *const awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut skiplen: size_t = 0;
    let mut flen: size_t = 0;
    fw = if api_parser_override as i32 != 0 { api_fw } else { FIELDWIDTHS };
    if up_to == 9223372036854775807 as i64 {
        nf = 0 as i32 as i64;
    }
    if len == 0 as i32 {
        return nf;
    }
    if gawk_mb_cur_max > 1 as i32 && (*fw).use_chars as u32 != 0 {
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        while nf < up_to && scan < end {
            if nf as u64 >= (*fw).nf {
                *buf = end;
                return nf;
            }
            scan = scan
                .offset(
                    calc_mbslen(
                        scan,
                        end,
                        (*((*fw).fields).as_ptr().offset(nf as isize)).skip,
                        &mut mbs,
                    ) as isize,
                );
            flen = calc_mbslen(
                scan,
                end,
                (*((*fw).fields).as_ptr().offset(nf as isize)).len,
                &mut mbs,
            );
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, flen as i64, n);
            scan = scan.offset(flen as isize);
        }
    } else {
        while nf < up_to && scan < end {
            if nf as u64 >= (*fw).nf {
                *buf = end;
                return nf;
            }
            skiplen = (*((*fw).fields).as_ptr().offset(nf as isize)).skip;
            if skiplen > end.offset_from(scan) as i64 as u64 {
                skiplen = end.offset_from(scan) as i64 as size_t;
            }
            scan = scan.offset(skiplen as isize);
            flen = (*((*fw).fields).as_ptr().offset(nf as isize)).len;
            if flen > end.offset_from(scan) as i64 as u64 {
                flen = end.offset_from(scan) as i64 as size_t;
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, flen as i64, n);
            scan = scan.offset(flen as isize);
        }
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn invalidate_field0() {
    field0_valid = 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_field(
    mut requested: i64,
    mut assign: *mut Func_ptr,
) -> *mut *mut NODE {
    let mut in_middle: bool = 0 as i32 != 0;
    static mut warned: bool = 0 as i32 != 0;
    extern "C" {
        static mut currule: i32;
    }
    let mut saved_fs: *mut NODE = 0 as *mut NODE;
    let mut fs_regexp: *mut Regexp = 0 as *mut Regexp;
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && currule == defrule::END as i32 && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 861 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"accessing fields from an defrule::END rule may not be portable\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if requested == 0 as i32 as i64 {
        if !field0_valid {
            if NF == -(1 as i32) as i64 {
                in_middle = parse_high_water != 0 as i32 as i64;
                if current_field_sep() as u32 == field_sep_type::Using_FPAT as i32 as u32
                {
                    saved_fs = save_FPAT;
                    fs_regexp = FPAT_regexp;
                } else {
                    saved_fs = save_FS;
                    fs_regexp = FS_regexp;
                }
                NF = (Some(parse_field.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    9223372036854775807 as i64 - 1 as i32 as i64,
                    &mut parse_extent,
                    ((**fields_arr.offset(0 as i32 as isize)).sub.val.slen)
                        .wrapping_sub(
                            parse_extent
                                .offset_from(
                                    (**fields_arr.offset(0 as i32 as isize)).sub.val.sp,
                                ) as i64 as u64,
                        ) as i32,
                    saved_fs,
                    fs_regexp,
                    Some(
                        set_field
                            as unsafe extern "C" fn(i64, *mut i8, i64, *mut NODE) -> (),
                    ),
                    0 as *mut libc::c_void as *mut NODE,
                    0 as *mut libc::c_void as *mut NODE,
                    in_middle,
                );
                parse_high_water = NF;
            }
            rebuild_record();
        }
        if !assign.is_null() {
            *assign = Some(reset_record as unsafe extern "C" fn() -> ());
        }
        return &mut *fields_arr.offset(0 as i32 as isize) as *mut *mut NODE;
    }
    if !assign.is_null() {
        *assign = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Func_ptr,
        >(
            Some(
                ::core::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(invalidate_field0),
            ),
        );
    }
    if requested <= parse_high_water {
        return &mut *fields_arr.offset(requested as isize) as *mut *mut NODE;
    }
    if NF == -(1 as i32) as i64 {
        if parse_high_water == 0 as i32 as i64 {
            parse_extent = (**fields_arr.offset(0 as i32 as isize)).sub.val.sp;
        } else {
            in_middle = 1 as i32 != 0;
        }
        parse_high_water = (Some(parse_field.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            requested,
            &mut parse_extent,
            ((**fields_arr.offset(0 as i32 as isize)).sub.val.slen)
                .wrapping_sub(
                    parse_extent
                        .offset_from((**fields_arr.offset(0 as i32 as isize)).sub.val.sp)
                        as i64 as u64,
                ) as i32,
            save_FS,
            0 as *mut Regexp,
            Some(set_field as unsafe extern "C" fn(i64, *mut i8, i64, *mut NODE) -> ()),
            0 as *mut libc::c_void as *mut NODE,
            0 as *mut libc::c_void as *mut NODE,
            in_middle,
        );
        if parse_extent
            == ((**fields_arr.offset(0 as i32 as isize)).sub.val.sp)
                .offset((**fields_arr.offset(0 as i32 as isize)).sub.val.slen as isize)
        {
            NF = parse_high_water;
        }
        if requested == 9223372036854775807 as i64 - 1 as i32 as i64 {
            requested = parse_high_water;
        }
    }
    if parse_high_water < requested {
        if !assign.is_null() {
            if requested > nf_high_water {
                grow_fields_arr(requested);
            }
            NF = requested;
            parse_high_water = requested;
        } else {
            return &mut Null_field
        }
    }
    return &mut *fields_arr.offset(requested as isize) as *mut *mut NODE;
}
unsafe extern "C" fn set_element(
    mut num: i64,
    mut s: *mut i8,
    mut len: i64,
    mut n: *mut NODE,
) {
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    it = make_str_node(s, len as size_t, 0 as i32);
    (*it).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*it).flags as u32 | flagvals::USER_INPUT as i32 as u32);
    sub = make_number.expect("non-null function pointer")(num as libc::c_double);
    assoc_set(n, sub, it);
}
#[no_mangle]
pub unsafe extern "C" fn do_split(mut nargs: i32) -> *mut NODE {
    let mut src: *mut NODE = 0 as *mut NODE;
    let mut arr: *mut NODE = 0 as *mut NODE;
    let mut sep: *mut NODE = 0 as *mut NODE;
    let mut fs: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sep_arr: *mut NODE = 0 as *mut NODE;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut parseit: Option<
        unsafe extern "C" fn(
            i64,
            *mut *mut i8,
            i32,
            *mut NODE,
            *mut Regexp,
            Setfunc,
            *mut NODE,
            *mut NODE,
            bool,
        ) -> i64,
    > = None;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    check_args_min_max(nargs, b"split\0" as *const u8 as *const i8, 3 as i32, 4 as i32);
    if nargs == 4 as i32 {
        static mut warned: bool = 0 as i32 != 0;
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
            || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 988 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"split: fourth argument is a gawk extension\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        sep_arr = POP_PARAM();
        if (*sep_arr).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 992 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"split: fourth argument is not an array\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        check_symtab_functab(
            sep_arr,
            b"split\0" as *const u8 as *const i8,
            dcgettext(
                0 as *const i8,
                b"%s: cannot use %s as fourth argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if (do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
            || do_flags as u32 & do_flag_values::DO_LINT_OLD as i32 as u32 != 0)
            && !warned
        {
            warned = 1 as i32 != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 997 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"split: fourth argument is a gawk extension\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    let fresh15 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    sep = (*fresh15).rptr;
    arr = POP_PARAM();
    if (*arr).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 1004 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"split: second argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    check_symtab_functab(
        arr,
        b"split\0" as *const u8 as *const i8,
        dcgettext(
            0 as *const i8,
            b"%s: cannot use %s as second argument\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    if !sep_arr.is_null() {
        if sep_arr == arr {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 1010 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"split: cannot use the same array for second and fourth args\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        tmp = (*sep_arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"field.c\0" as *const u8 as *const i8, 1015 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"split: cannot use a subarray of second arg for fourth arg\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        tmp = (*arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == sep_arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"field.c\0" as *const u8 as *const i8, 1018 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"split: cannot use a subarray of fourth arg for second arg\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        ((*(*sep_arr).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(sep_arr, 0 as *mut exp_node);
    }
    ((*(*arr).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(arr, 0 as *mut exp_node);
    src = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
    if (*src).sub.val.slen == 0 as i32 as u64 {
        tmp = POP_SCALAR();
        DEREF(tmp);
        return make_number
            .expect("non-null function pointer")(0 as i32 as libc::c_double);
    }
    if (*sep).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
        sep = (*sep).sub.val.typre;
    }
    if (*sep).sub.nodep.reflags as u32 & reflagvals::FS_DFLT as i32 as u32
        != 0 as i32 as u32
        && current_field_sep() as u32 == field_sep_type::Using_FS as i32 as u32
        && !RS_is_null
    {
        parseit = parse_field;
        fs = force_string_fmt((*FS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
        rp = FS_regexp;
    } else {
        fs = (*sep).sub.nodep.x.extra;
        if (*fs).sub.val.slen == 0 as i32 as u64 {
            static mut warned_0: bool = 0 as i32 != 0;
            parseit = Some(
                null_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            );
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0 && !warned_0
            {
                warned_0 = 1 as i32 != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"field.c\0" as *const u8 as *const i8, 1052 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"split: null string for third arg is a non-standard extension\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
        } else if (*fs).sub.val.slen == 1 as i32 as u64
            && (*sep).sub.nodep.reflags as u32 & reflagvals::CONSTANT as i32 as u32
                == 0 as i32 as u32
        {
            if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 == ' ' as i32 {
                parseit = Some(
                    def_parse_field
                        as unsafe extern "C" fn(
                            i64,
                            *mut *mut i8,
                            i32,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> i64,
                );
            } else {
                parseit = Some(
                    sc_parse_field
                        as unsafe extern "C" fn(
                            i64,
                            *mut *mut i8,
                            i32,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> i64,
                );
            }
        } else {
            parseit = Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            );
            rp = re_update(sep);
        }
    }
    s = (*src).sub.val.sp;
    tmp = make_number
        .expect(
            "non-null function pointer",
        )(
        (Some(parseit.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            9223372036854775807 as i64,
            &mut s,
            (*src).sub.val.slen as i32,
            fs,
            rp,
            Some(
                set_element as unsafe extern "C" fn(i64, *mut i8, i64, *mut NODE) -> (),
            ),
            arr,
            sep_arr,
            0 as i32 != 0,
        ) as libc::c_double,
    );
    src = POP_SCALAR();
    DEREF(src);
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn do_patsplit(mut nargs: i32) -> *mut NODE {
    let mut src: *mut NODE = 0 as *mut NODE;
    let mut arr: *mut NODE = 0 as *mut NODE;
    let mut sep: *mut NODE = 0 as *mut NODE;
    let mut fpat: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sep_arr: *mut NODE = 0 as *mut NODE;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    check_args_min_max(
        nargs,
        b"patsplit\0" as *const u8 as *const i8,
        3 as i32,
        4 as i32,
    );
    if nargs == 4 as i32 {
        sep_arr = POP_PARAM();
        if (*sep_arr).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 1091 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"patsplit: fourth argument is not an array\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        check_symtab_functab(
            sep_arr,
            b"patsplit\0" as *const u8 as *const i8,
            dcgettext(
                0 as *const i8,
                b"%s: cannot use %s as fourth argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    let fresh16 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    sep = (*fresh16).rptr;
    arr = POP_PARAM();
    if (*arr).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 1098 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"patsplit: second argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    check_symtab_functab(
        arr,
        b"patsplit\0" as *const u8 as *const i8,
        dcgettext(
            0 as *const i8,
            b"%s: cannot use %s as second argument\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    src = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
    if (*sep).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
        sep = (*sep).sub.val.typre;
    }
    fpat = (*sep).sub.nodep.x.extra;
    if (*fpat).sub.val.slen == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 1109 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"patsplit: third argument must be non-null\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if !sep_arr.is_null() {
        if sep_arr == arr {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 1113 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"patsplit: cannot use the same array for second and fourth args\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        tmp = (*sep_arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"field.c\0" as *const u8 as *const i8, 1118 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"patsplit: cannot use a subarray of second arg for fourth arg\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        tmp = (*arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == sep_arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"field.c\0" as *const u8 as *const i8, 1121 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"patsplit: cannot use a subarray of fourth arg for second arg\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        ((*(*sep_arr).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(sep_arr, 0 as *mut exp_node);
    }
    ((*(*arr).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(arr, 0 as *mut exp_node);
    if (*src).sub.val.slen == 0 as i32 as u64 {
        tmp = make_number
            .expect("non-null function pointer")(0 as i32 as libc::c_double);
    } else {
        rp = re_update(sep);
        s = (*src).sub.val.sp;
        tmp = make_number
            .expect(
                "non-null function pointer",
            )(
            fpat_parse_field(
                9223372036854775807 as i64,
                &mut s,
                (*src).sub.val.slen as i32,
                fpat,
                rp,
                Some(
                    set_element
                        as unsafe extern "C" fn(i64, *mut i8, i64, *mut NODE) -> (),
                ),
                arr,
                sep_arr,
                0 as i32 != 0,
            ) as libc::c_double,
        );
    }
    src = POP_SCALAR();
    DEREF(src);
    return tmp;
}
unsafe extern "C" fn set_parser(mut func: parse_field_func_t) {
    normal_parse_field = func;
    if !api_parser_override && parse_field != func {
        parse_field = func;
        update_PROCINFO_str(b"FS\0" as *const u8 as *const i8, current_field_sep_str());
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_FIELDWIDTHS() {
    let mut scan: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    static mut fw_alloc: i32 = 4 as i32;
    static mut warned: bool = 0 as i32 != 0;
    let mut fatal_error: bool = 0 as i32 != 0;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0 && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 1171 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"`FIELDWIDTHS' is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        return;
    }
    if !fields_arr.is_null() {
        get_field(9223372036854775807 as i64 - 1 as i32 as i64, 0 as *mut Func_ptr);
    }
    set_parser(
        Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    i64,
                    *mut *mut i8,
                    i32,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> i64,
        ),
    );
    tmp = force_string_fmt((*FIELDWIDTHS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    scan = (*tmp).sub.val.sp;
    if FIELDWIDTHS.is_null() {
        FIELDWIDTHS = emalloc_real(
            (::core::mem::size_of::<awk_fieldwidth_info_t>() as u64)
                .wrapping_add(
                    ((fw_alloc - 1 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<awk_field_info>() as u64),
                ),
            b"set_FIELDWIDTHS\0" as *const u8 as *const i8,
            b"FIELDWIDTHS\0" as *const u8 as *const i8,
            b"field.c\0" as *const u8 as *const i8,
            1188 as i32,
        ) as *mut awk_fieldwidth_info_t;
        (*FIELDWIDTHS).use_chars = awk_bool::awk_true;
    }
    (*FIELDWIDTHS).nf = 0 as i32 as size_t;
    i = 0 as i32;
    loop {
        let mut tmp_0: u64 = 0;
        if i >= fw_alloc {
            fw_alloc *= 2 as i32;
            FIELDWIDTHS = erealloc_real(
                FIELDWIDTHS as *mut libc::c_void,
                (::core::mem::size_of::<awk_fieldwidth_info_t>() as u64)
                    .wrapping_add(
                        ((fw_alloc - 1 as i32) as u64)
                            .wrapping_mul(
                                ::core::mem::size_of::<awk_field_info>() as u64,
                            ),
                    ),
                b"set_FIELDWIDTHS\0" as *const u8 as *const i8,
                b"FIELDWIDTHS\0" as *const u8 as *const i8,
                b"field.c\0" as *const u8 as *const i8,
                1196 as i32,
            ) as *mut awk_fieldwidth_info_t;
        }
        while is_blank(*scan as i32) != 0 {
            scan = scan.offset(1);
            scan;
        }
        if *scan as i32 == '-' as i32 {
            fatal_error = 1 as i32 != 0;
            break;
        } else {
            if *scan as i32 == '\0' as i32 {
                break;
            }
            *__errno_location() = 0 as i32;
            tmp_0 = strtoul(scan, &mut end, 10 as i32);
            if *__errno_location() == 0 as i32 && *end as i32 == ':' as i32
                && ((0 as i32 as u64) < tmp_0
                    && tmp_0
                        <= (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64)
            {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).skip = tmp_0;
                scan = end.offset(1 as i32 as isize);
                if *scan as i32 == '-' as i32 || is_blank(*scan as i32) != 0 {
                    fatal_error = 1 as i32 != 0;
                    break;
                } else {
                    tmp_0 = strtoul(scan, &mut end, 10 as i32);
                }
            } else {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).skip = 0
                    as i32 as size_t;
            }
            if *__errno_location() != 0 as i32
                || *end as i32 != '\0' as i32 && is_blank(*end as i32) == 0
                || !((0 as i32 as u64) < tmp_0
                    && tmp_0
                        <= (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64)
            {
                if *scan as i32 == '*' as i32 {
                    scan = scan.offset(1);
                    scan;
                    while is_blank(*scan as i32) != 0 {
                        scan = scan.offset(1);
                        scan;
                    }
                    if *scan as i32 != '\0' as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"field.c\0" as *const u8 as *const i8,
                            1240 as i32,
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
                                b"`*' must be the last designator in FIELDWIDTHS\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).len = (2147483647
                        as i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as size_t;
                    (*FIELDWIDTHS).nf = (i + 1 as i32) as size_t;
                } else {
                    fatal_error = 1 as i32 != 0;
                }
                break;
            } else {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).len = tmp_0;
                (*FIELDWIDTHS).nf = (i + 1 as i32) as size_t;
                scan = end;
                while is_blank(*scan as i32) != 0 {
                    scan = scan.offset(1);
                    scan;
                }
                if *scan as i32 == '\0' as i32 {
                    break;
                }
                i += 1;
                i;
            }
        }
    }
    if fatal_error {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 1261 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"invalid FIELDWIDTHS value, for field %d, near `%s'\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            i + 1 as i32,
            scan,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_FS() {
    let mut buf: [i8; 10] = [0; 10];
    let mut fs: *mut NODE = 0 as *mut NODE;
    static mut save_fs: *mut NODE = 0 as *const NODE as *mut NODE;
    static mut save_rs: *mut NODE = 0 as *const NODE as *mut NODE;
    let mut remake_re: bool = 1 as i32 != 0;
    if !fields_arr.is_null() {
        get_field(9223372036854775807 as i64 - 1 as i32 as i64, 0 as *mut Func_ptr);
    }
    if !save_fs.is_null()
        && (*(*FS_node).sub.nodep.l.lptr).sub.val.slen == (*save_fs).sub.val.slen
        && memcmp(
            (*(*FS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_fs).sub.val.sp as *const libc::c_void,
            (*save_fs).sub.val.slen,
        ) == 0 as i32 && !save_rs.is_null()
        && (*(*RS_node).sub.nodep.l.lptr).sub.val.slen == (*save_rs).sub.val.slen
        && memcmp(
            (*(*RS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.slen,
        ) == 0 as i32
    {
        if !FS_regexp.is_null() {
            FS_regexp = if IGNORECASE as i32 != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
        }
        if current_field_sep() as u32 == field_sep_type::Using_FS as i32 as u32 {
            return
        } else {
            remake_re = 0 as i32 != 0;
        }
    } else {
        unref(save_fs);
        save_fs = dupnode((*FS_node).sub.nodep.l.lptr);
        unref(save_rs);
        save_rs = dupnode((*RS_node).sub.nodep.l.lptr);
        resave_fs = 1 as i32 != 0;
        refree(FS_re_yes_case);
        refree(FS_re_no_case);
        FS_regexp = 0 as *mut Regexp;
        FS_re_no_case = FS_regexp;
        FS_re_yes_case = FS_re_no_case;
    }
    buf[0 as i32 as usize] = '\0' as i32 as i8;
    default_FS = 0 as i32;
    fs = force_string_fmt((*FS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
        && (*fs).sub.val.slen == 0 as i32 as u64
    {
        static mut warned: bool = 0 as i32 != 0;
        set_parser(
            Some(
                null_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            ),
        );
        if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
            && !warned
        {
            warned = 1 as i32 != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 1334 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"null string for `FS' is a gawk extension\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
    } else if (*fs).sub.val.slen > 1 as i32 as u64
        || (*fs).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32
    {
        if do_flags as u32 & do_flag_values::DO_LINT_OLD as i32 as u32 != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"field.c\0" as *const u8 as *const i8, 1338 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"old awk does not support regexps as value of `FS'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        set_parser(
            Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            ),
        );
    } else if RS_is_null {
        set_parser(
            Some(
                sc_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            ),
        );
        if (*fs).sub.val.slen == 1 as i32 as u64 {
            if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 == ' ' as i32 {
                default_FS = 1 as i32;
                strcpy(buf.as_mut_ptr(), b"[ \t\n]+\0" as *const u8 as *const i8);
            } else if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 == '\\' as i32
            {
                strcpy(buf.as_mut_ptr(), b"[\\\\\n]\0" as *const u8 as *const i8);
            } else if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 == '\0' as i32
            {
                strcpy(buf.as_mut_ptr(), b"[\\000\n]\0" as *const u8 as *const i8);
            } else if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 != '\n' as i32
            {
                sprintf(
                    buf.as_mut_ptr(),
                    b"[%c\n]\0" as *const u8 as *const i8,
                    *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32,
                );
            }
        }
    } else {
        set_parser(
            Some(
                def_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            ),
        );
        if (*fs).sub.val.slen == 1 as i32 as u64 {
            if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 == ' ' as i32 {
                default_FS = 1 as i32;
            } else if *((*fs).sub.val.sp).offset(0 as i32 as isize) as i32 == '\\' as i32
            {
                strcpy(buf.as_mut_ptr(), b"[\\\\]\0" as *const u8 as *const i8);
            } else {
                set_parser(
                    Some(
                        sc_parse_field
                            as unsafe extern "C" fn(
                                i64,
                                *mut *mut i8,
                                i32,
                                *mut NODE,
                                *mut Regexp,
                                Setfunc,
                                *mut NODE,
                                *mut NODE,
                                bool,
                            ) -> i64,
                    ),
                );
            }
        }
    }
    if remake_re {
        refree(FS_re_yes_case);
        refree(FS_re_no_case);
        FS_regexp = 0 as *mut Regexp;
        FS_re_no_case = FS_regexp;
        FS_re_yes_case = FS_re_no_case;
        if buf[0 as i32 as usize] as i32 != '\0' as i32 {
            FS_re_yes_case = make_regexp(
                buf.as_mut_ptr(),
                strlen(buf.as_mut_ptr()),
                0 as i32 != 0,
                1 as i32 != 0,
                1 as i32 != 0,
            );
            FS_re_no_case = make_regexp(
                buf.as_mut_ptr(),
                strlen(buf.as_mut_ptr()),
                1 as i32 != 0,
                1 as i32 != 0,
                1 as i32 != 0,
            );
            FS_regexp = if IGNORECASE as i32 != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
            set_parser(
                Some(
                    re_parse_field
                        as unsafe extern "C" fn(
                            i64,
                            *mut *mut i8,
                            i32,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> i64,
                ),
            );
        } else if parse_field
            == Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            )
        {
            FS_re_yes_case = make_regexp(
                (*fs).sub.val.sp,
                (*fs).sub.val.slen,
                0 as i32 != 0,
                1 as i32 != 0,
                1 as i32 != 0,
            );
            FS_re_no_case = make_regexp(
                (*fs).sub.val.sp,
                (*fs).sub.val.slen,
                1 as i32 != 0,
                1 as i32 != 0,
                1 as i32 != 0,
            );
            FS_regexp = if IGNORECASE as i32 != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
        } else {
            FS_regexp = 0 as *mut Regexp;
            FS_re_no_case = FS_regexp;
            FS_re_yes_case = FS_re_no_case;
        }
    }
    if (*fs).sub.val.slen == 1 as i32 as u64
        && parse_field
            == Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        i64,
                        *mut *mut i8,
                        i32,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> i64,
            )
    {
        FS_regexp = FS_re_yes_case;
    }
}
#[no_mangle]
pub unsafe extern "C" fn current_field_sep() -> field_sep_type {
    if api_parser_override {
        return field_sep_type::Using_API
    } else if parse_field
        == Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    i64,
                    *mut *mut i8,
                    i32,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> i64,
        )
    {
        return field_sep_type::Using_FIELDWIDTHS
    } else if parse_field
        == Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    i64,
                    *mut *mut i8,
                    i32,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> i64,
        )
    {
        return field_sep_type::Using_FPAT
    } else {
        return field_sep_type::Using_FS
    };
}
#[no_mangle]
pub unsafe extern "C" fn current_field_sep_str() -> *const i8 {
    if api_parser_override {
        return b"API\0" as *const u8 as *const i8
    } else if parse_field
        == Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    i64,
                    *mut *mut i8,
                    i32,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> i64,
        )
    {
        return b"FIELDWIDTHS\0" as *const u8 as *const i8
    } else if parse_field
        == Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    i64,
                    *mut *mut i8,
                    i32,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> i64,
        )
    {
        return b"FPAT\0" as *const u8 as *const i8
    } else {
        return b"FS\0" as *const u8 as *const i8
    };
}
#[no_mangle]
pub unsafe extern "C" fn update_PROCINFO_str(
    mut subscript: *const i8,
    mut str: *const i8,
) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if PROCINFO_node.is_null() {
        return;
    }
    tmp = make_str_node(subscript, strlen(subscript), 0 as i32);
    assoc_set(PROCINFO_node, tmp, make_str_node(str, strlen(str), 0 as i32));
}
#[no_mangle]
pub unsafe extern "C" fn update_PROCINFO_num(
    mut subscript: *const i8,
    mut val: libc::c_double,
) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if PROCINFO_node.is_null() {
        return;
    }
    tmp = make_str_node(subscript, strlen(subscript), 0 as i32);
    assoc_set(PROCINFO_node, tmp, make_number.expect("non-null function pointer")(val));
}
#[no_mangle]
pub unsafe extern "C" fn set_FPAT() {
    static mut warned: bool = 0 as i32 != 0;
    let mut remake_re: bool = 1 as i32 != 0;
    let mut fpat: *mut NODE = 0 as *mut NODE;
    if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0 && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"field.c\0" as *const u8 as *const i8, 1464 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"`FPAT' is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        return;
    }
    if !fields_arr.is_null() {
        get_field(9223372036854775807 as i64 - 1 as i32 as i64, 0 as *mut Func_ptr);
    }
    if !save_FPAT.is_null()
        && (*(*FPAT_node).sub.nodep.l.lptr).sub.val.slen == (*save_FPAT).sub.val.slen
        && memcmp(
            (*(*FPAT_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_FPAT).sub.val.sp as *const libc::c_void,
            (*save_FPAT).sub.val.slen,
        ) == 0 as i32
    {
        if !FPAT_regexp.is_null() {
            FPAT_regexp = if IGNORECASE as i32 != 0 {
                FPAT_re_no_case
            } else {
                FPAT_re_yes_case
            };
        }
        if current_field_sep() as u32 == field_sep_type::Using_FPAT as i32 as u32 {
            return
        } else {
            remake_re = 0 as i32 != 0;
        }
    } else {
        unref(save_FPAT);
        save_FPAT = dupnode((*FPAT_node).sub.nodep.l.lptr);
        refree(FPAT_re_yes_case);
        refree(FPAT_re_no_case);
        FPAT_regexp = 0 as *mut Regexp;
        FPAT_re_no_case = FPAT_regexp;
        FPAT_re_yes_case = FPAT_re_no_case;
    }
    fpat = force_string_fmt((*FPAT_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    set_parser(
        Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    i64,
                    *mut *mut i8,
                    i32,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> i64,
        ),
    );
    if remake_re {
        refree(FPAT_re_yes_case);
        refree(FPAT_re_no_case);
        FPAT_regexp = 0 as *mut Regexp;
        FPAT_re_no_case = FPAT_regexp;
        FPAT_re_yes_case = FPAT_re_no_case;
        FPAT_re_yes_case = make_regexp(
            (*fpat).sub.val.sp,
            (*fpat).sub.val.slen,
            0 as i32 != 0,
            1 as i32 != 0,
            1 as i32 != 0,
        );
        FPAT_re_no_case = make_regexp(
            (*fpat).sub.val.sp,
            (*fpat).sub.val.slen,
            1 as i32 != 0,
            1 as i32 != 0,
            1 as i32 != 0,
        );
        FPAT_regexp = if IGNORECASE as i32 != 0 {
            FPAT_re_no_case
        } else {
            FPAT_re_yes_case
        };
    }
}
unsafe extern "C" fn incr_scan(
    mut scanp: *mut *mut i8,
    mut len: size_t,
    mut mbs: *mut mbstate_t,
) {
    let mut mbclen: size_t = 0 as i32 as size_t;
    if gawk_mb_cur_max > 1 as i32 {
        mbclen = mbrlen(*scanp, len, mbs);
        if mbclen == 1 as i32 as u64 || mbclen == -(1 as i32) as size_t
            || mbclen == -(2 as i32) as size_t || mbclen == 0 as i32 as u64
        {
            mbclen = 1 as i32 as size_t;
        }
        *scanp = (*scanp).offset(mbclen as isize);
    } else {
        *scanp = (*scanp).offset(1);
        *scanp;
    };
}
unsafe extern "C" fn fpat_parse_field(
    mut up_to: i64,
    mut buf: *mut *mut i8,
    mut len: i32,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> i64 {
    let mut scan: *mut i8 = *buf;
    let mut nf: i64 = parse_high_water;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = scan.offset(len as isize);
    let mut regex_flags: i32 = 1 as i32;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut field_start: *mut i8 = 0 as *mut i8;
    let mut field_found: bool = 0 as i32 != 0;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    if up_to == 9223372036854775807 as i64 {
        nf = 0 as i32 as i64;
    }
    if len == 0 as i32 {
        return nf;
    }
    if rp.is_null() {
        rp = FPAT_regexp;
    }
    while scan < end && nf < up_to {
        start = scan;
        field_found = research(
            rp,
            scan,
            0 as i32,
            end.offset_from(scan) as i64 as size_t,
            regex_flags,
        ) != -(1 as i32);
        if nf > 0 as i32 as i64 && field_found as i32 != 0
            && *((*rp).regs.end).offset(0 as i32 as isize) == 0 as i32
        {
            incr_scan(&mut scan, end.offset_from(scan) as i64 as size_t, &mut mbs);
            field_found = research(
                rp,
                scan,
                0 as i32,
                end.offset_from(scan) as i64 as size_t,
                regex_flags,
            ) != -(1 as i32);
        }
        if field_found {
            field_start = scan
                .offset(*((*rp).regs.start).offset(0 as i32 as isize) as isize);
            if !sep_arr.is_null() {
                if field_start == start {
                    set_element(nf, start, 0 as i64, sep_arr);
                } else {
                    set_element(
                        nf,
                        start,
                        field_start.offset_from(start) as i64,
                        sep_arr,
                    );
                }
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                nf,
                field_start,
                (*((*rp).regs.end).offset(0 as i32 as isize)
                    - *((*rp).regs.start).offset(0 as i32 as isize)) as i64,
                n,
            );
            scan = scan.offset(*((*rp).regs.end).offset(0 as i32 as isize) as isize);
        } else {
            if !sep_arr.is_null() {
                set_element(nf, start, end.offset_from(start) as i64, sep_arr);
            }
            scan = end;
        }
    }
    if !sep_arr.is_null() && scan == end && field_found as i32 != 0 {
        set_element(nf, scan, 0 as i64, sep_arr);
    }
    *buf = scan;
    return nf;
}