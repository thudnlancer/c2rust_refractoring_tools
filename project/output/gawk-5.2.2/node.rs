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
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn __isnanl(__value: f128::f128) -> i32;
    fn __isnanf(__value: libc::c_float) -> i32;
    fn __isnan(__value: libc::c_double) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn using_utf8() -> bool;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn towlower(__wc: wint_t) -> wint_t;
    static mut Nnull_string: *mut NODE;
    static mut Null_field: *mut NODE;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn nondec2awknum(str: *mut i8, len: size_t, endptr: *mut *mut i8) -> libc::c_double;
    static mut loc: lconv;
    static mut do_flags: do_flag_values;
    fn is_alpha(c: i32) -> bool;
    fn format_tree(_: *const i8, _: size_t, _: *mut *mut NODE, _: i64) -> *mut NODE;
    fn double_to_int(d: libc::c_double) -> libc::c_double;
    fn format_nan_inf(n: *mut NODE, format: i8) -> *mut i8;
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn r_warning(mesg: *const i8, _: ...);
    static mut gawk_mb_cur_max: i32;
    fn out_of_range(n: *mut NODE) -> bool;
    fn mpfr_unset(n: *mut NODE);
    fn make_regnode(type_0: NODETYPE, exp: *mut NODE) -> *mut NODE;
    fn __btowc_alias(__c: i32) -> wint_t;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn wcrtomb(__s: *mut i8, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn __mbrlen(__s: *const i8, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    static mut fmt_list: *mut *mut NODE;
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
pub type wint_t = u32;
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
    DO_TRADITIONAL = 16,
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
    DO_LINT_OLD = 8,
    DO_LINT_ALL = 4,
    DO_LINT_EXTENSIONS = 2,
    DO_LINT_INVALID = 1,
    DO_FLAG_NONE = 0,
}
impl do_flag_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            do_flag_values::DO_TRADITIONAL => 16,
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
            do_flag_values::DO_LINT_OLD => 8,
            do_flag_values::DO_LINT_ALL => 4,
            do_flag_values::DO_LINT_EXTENSIONS => 2,
            do_flag_values::DO_LINT_INVALID => 1,
            do_flag_values::DO_FLAG_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> do_flag_values {
        match value {
            16 => do_flag_values::DO_TRADITIONAL,
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
unsafe extern "C" fn make_number_node(mut flags: u32) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r as *mut block_item))
            .freep;
    } else {
        r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(r as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<NODE>() as u64);
    (*r).type_0 = nodevals::Node_val;
    (*r).valref = 1 as i32 as i64;
    (*r).flags = flagvals::from_libc_c_uint(
        (flags | flagvals::MALLOC as i32 as u32 | flagvals::NUMBER as i32 as u32
            | flagvals::NUMCUR as i32 as u32) as u32,
    );
    return r;
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
unsafe extern "C" fn btowc(mut __c: i32) -> wint_t {
    return if 0 != 0 && __c >= '\0' as i32 && __c <= '\u{7f}' as i32 {
        __c as wint_t
    } else {
        __btowc_alias(__c)
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
#[no_mangle]
pub static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE> = unsafe {
    Some(r_make_number as unsafe extern "C" fn(libc::c_double) -> *mut NODE)
};
#[no_mangle]
pub static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE> = unsafe {
    Some(r_force_number as unsafe extern "C" fn(*mut NODE) -> *mut NODE)
};
#[no_mangle]
pub static mut format_val: Option<
    unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
> = unsafe {
    Some(r_format_val as unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE)
};
#[no_mangle]
pub static mut cmp_numbers: Option<
    unsafe extern "C" fn(*const NODE, *const NODE) -> i32,
> = unsafe {
    Some(cmp_awknums as unsafe extern "C" fn(*const NODE, *const NODE) -> i32)
};
unsafe extern "C" fn is_hex(mut str: *const i8, mut cpend: *const i8) -> bool {
    if *str as i32 == '-' as i32 || *str as i32 == '+' as i32 {
        str = str.offset(1);
        str;
    }
    if str.offset(1 as i32 as isize) < cpend
        && *str.offset(0 as i32 as isize) as i32 == '0' as i32
        && (*str.offset(1 as i32 as isize) as i32 == 'x' as i32
            || *str.offset(1 as i32 as isize) as i32 == 'X' as i32)
    {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn r_force_number(mut n: *mut NODE) -> *mut NODE {
    let mut current_block: u64;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cpend: *mut i8 = 0 as *mut i8;
    let mut save: i8 = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    if (*n).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        (*n).type_0 = nodevals::Node_val;
        (*n).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*n).flags as u32 & !(flagvals::STRING as i32) as u32);
        *((*n).sub.val.sp).offset(0 as i32 as isize) = '0' as i32 as i8;
        (*n).sub.val.slen = 1 as i32 as size_t;
        return n;
    }
    if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        return n;
    }
    (*n).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*n).flags as u32 | flagvals::NUMCUR as i32 as u32);
    (*n).sub.val.fltnum = 0.0f64;
    cp = (*n).sub.val.sp;
    cpend = cp.offset((*n).sub.val.slen as isize);
    while cp < cpend
        && *(*__ctype_b_loc()).offset(*cp as u8 as i32 as isize) as i32
            & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        cp = cp.offset(1);
        cp;
    }
    if !(cp == cpend) {
        while *(*__ctype_b_loc())
            .offset(*cpend.offset(-(1 as i32) as isize) as u8 as i32 as isize) as i32
            & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 != 0
        {
            cpend = cpend.offset(-1);
            cpend;
        }
        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0 {
            if is_alpha(*cp as u8 as i32) {
                current_block = 6914046712317957593;
            } else if is_ieee_magic_val(cp) {
                if cpend == cp.offset(4 as i32 as isize) {
                    (*n).sub.val.fltnum = get_ieee_magic_val(cp);
                    current_block = 9144628940032583806;
                } else {
                    current_block = 6914046712317957593;
                }
            } else {
                current_block = 13797916685926291137;
            }
        } else {
            current_block = 13797916685926291137;
        }
        match current_block {
            6914046712317957593 => {}
            _ => {
                match current_block {
                    13797916685926291137 => {
                        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
                            && (is_alpha(*cp as u8 as i32) as i32 != 0
                                || do_flags as u32
                                    & do_flag_values::DO_NON_DEC_DATA as i32 as u32 == 0
                                    && is_hex(cp, cpend) as i32 != 0)
                        {
                            current_block = 6914046712317957593;
                        } else if cpend.offset_from(cp) as i64 == 1 as i32 as i64 {
                            if *(*__ctype_b_loc()).offset(*cp as u8 as i32 as isize)
                                as i32
                                & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                != 0
                            {
                                (*n).sub.val.fltnum = (*cp as i32 - '0' as i32)
                                    as libc::c_double;
                                if (*n).sub.val.slen == 1 as i32 as u64 {
                                    (*n).flags = ::core::mem::transmute::<
                                        u32,
                                        flagvals,
                                    >((*n).flags as u32 | flagvals::NUMINT as i32 as u32);
                                }
                                current_block = 9144628940032583806;
                            } else {
                                current_block = 6914046712317957593;
                            }
                        } else {
                            *__errno_location() = 0 as i32;
                            if do_flags as u32
                                & do_flag_values::DO_NON_DEC_DATA as i32 as u32 != 0
                                && do_flags as u32
                                    & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                                && get_numbase(
                                    cp,
                                    cpend.offset_from(cp) as i64 as size_t,
                                    1 as i32 != 0,
                                ) != 10 as i32
                            {
                                (*n).sub.val.fltnum = nondec2awknum(
                                    cp,
                                    cpend.offset_from(cp) as i64 as size_t,
                                    &mut ptr,
                                );
                            } else {
                                save = *cpend;
                                *cpend = '\0' as i32 as i8;
                                (*n).sub.val.fltnum = strtod(cp as *const i8, &mut ptr);
                                *cpend = save;
                            }
                            if *__errno_location() == 0 as i32
                                || *__errno_location() == 34 as i32
                            {
                                *__errno_location() = 0 as i32;
                                if ptr == cpend {
                                    current_block = 9144628940032583806;
                                } else {
                                    current_block = 6914046712317957593;
                                }
                            } else {
                                *__errno_location() = 0 as i32;
                                (*n).sub.val.fltnum = 0 as i32 as libc::c_double;
                                current_block = 6914046712317957593;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    6914046712317957593 => {}
                    _ => {
                        if (if ::core::mem::size_of::<libc::c_double>() as u64
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
                        }) != 0 && *cp as i32 == '-' as i32
                            && (if ::core::mem::size_of::<libc::c_double>() as u64
                                == ::core::mem::size_of::<libc::c_float>() as u64
                            {
                                ((*n).sub.val.fltnum as libc::c_float).is_sign_negative()
                                    as i32
                            } else {
                                (if ::core::mem::size_of::<libc::c_double>() as u64
                                    == ::core::mem::size_of::<libc::c_double>() as u64
                                {
                                    ((*n).sub.val.fltnum).is_sign_negative() as i32
                                } else {
                                    (f128::f128::new((*n).sub.val.fltnum)).is_sign_negative()
                                        as i32
                                })
                            }) == 0 as i32
                        {
                            (*n).sub.val.fltnum = -(*n).sub.val.fltnum;
                        }
                        if (*n).flags as u32 & flagvals::USER_INPUT as i32 as u32
                            != 0 as i32 as u32
                        {
                            (*n).flags = ::core::mem::transmute::<
                                u32,
                                flagvals,
                            >((*n).flags as u32 & !(flagvals::STRING as i32) as u32);
                            (*n).flags = ::core::mem::transmute::<
                                u32,
                                flagvals,
                            >((*n).flags as u32 | flagvals::NUMBER as i32 as u32);
                        }
                        return n;
                    }
                }
            }
        }
    }
    (*n).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*n).flags as u32 & !(flagvals::USER_INPUT as i32) as u32);
    return n;
}
static mut values: [*const i8; 10] = [
    b"0\0" as *const u8 as *const i8,
    b"1\0" as *const u8 as *const i8,
    b"2\0" as *const u8 as *const i8,
    b"3\0" as *const u8 as *const i8,
    b"4\0" as *const u8 as *const i8,
    b"5\0" as *const u8 as *const i8,
    b"6\0" as *const u8 as *const i8,
    b"7\0" as *const u8 as *const i8,
    b"8\0" as *const u8 as *const i8,
    b"9\0" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn r_format_val(
    mut format: *const i8,
    mut index: i32,
    mut s: *mut NODE,
) -> *mut NODE {
    let mut buf: [i8; 8192] = [0; 8192];
    let mut sp: *mut i8 = buf.as_mut_ptr();
    let mut val: libc::c_double = 0.;
    if out_of_range(s) {
        let mut result: *const i8 = format_nan_inf(s, 'g' as i32 as i8);
        return make_str_node(result, strlen(result), 0 as i32);
    } else {
        val = double_to_int((*s).sub.val.fltnum);
        if val != (*s).sub.val.fltnum
            || val <= (-(9223372036854775807 as i64) - 1 as i64) as libc::c_double
            || val >= 9223372036854775807 as i64 as libc::c_double
        {
            let mut dummy: [*mut NODE; 2] = [0 as *mut NODE; 2];
            let mut r: *mut NODE = 0 as *mut NODE;
            let mut oflags: u32 = 0;
            dummy[1 as i32 as usize] = s;
            oflags = (*s).flags as u32;
            if val == (*s).sub.val.fltnum {
                r = format_tree(
                    b"%.0f\0" as *const u8 as *const i8,
                    4 as i32 as size_t,
                    dummy.as_mut_ptr(),
                    2 as i32 as i64,
                );
                (*s).sub.val.idx = -(1 as i32);
            } else {
                r = format_tree(
                    format,
                    (**fmt_list.offset(index as isize)).sub.val.slen,
                    dummy.as_mut_ptr(),
                    2 as i32 as i64,
                );
                (*s).sub.val.idx = index;
            }
            (*s).flags = flagvals::from_libc_c_uint(oflags as u32);
            (*s).sub.val.slen = (*r).sub.val.slen;
            if (*s).flags as u32
                & (flagvals::MALLOC as i32 | flagvals::STRCUR as i32) as u32
                == (flagvals::MALLOC as i32 | flagvals::STRCUR as i32) as u32
            {
                pma_free((*s).sub.val.sp as *mut libc::c_void);
            }
            (*s).sub.val.sp = (*r).sub.val.sp;
            let ref mut fresh0 = (*(r as *mut block_item)).freep;
            *fresh0 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r as *mut block_item;
        } else {
            let mut num: i64 = val as i64;
            if (num as u64)
                < (::core::mem::size_of::<[*const i8; 10]>() as u64)
                    .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
                && num >= 0 as i32 as i64
            {
                sp = values[num as usize] as *mut i8;
                (*s).sub.val.slen = 1 as i32 as size_t;
            } else {
                sprintf(sp, b"%ld\0" as *const u8 as *const i8, num);
                (*s).sub.val.slen = strlen(sp);
            }
            (*s).sub.val.idx = -(1 as i32);
            if (*s).flags as u32 & flagvals::INTIND as i32 as u32 != 0 as i32 as u32 {
                (*s).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >(
                    (*s).flags as u32
                        & !(flagvals::INTIND as i32 | flagvals::NUMBER as i32) as u32,
                );
                (*s).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >((*s).flags as u32 | flagvals::STRING as i32 as u32);
            }
            if (*s).flags as u32
                & (flagvals::MALLOC as i32 | flagvals::STRCUR as i32) as u32
                == (flagvals::MALLOC as i32 | flagvals::STRCUR as i32) as u32
            {
                pma_free((*s).sub.val.sp as *mut libc::c_void);
            }
            (*s).sub.val.sp = emalloc_real(
                ((*s).sub.val.slen).wrapping_add(1 as i32 as u64),
                b"format_val\0" as *const u8 as *const i8,
                b"s->stptr\0" as *const u8 as *const i8,
                b"node.c\0" as *const u8 as *const i8,
                300 as i32,
            ) as *mut i8;
            memcpy(
                (*s).sub.val.sp as *mut libc::c_void,
                sp as *const libc::c_void,
                ((*s).sub.val.slen).wrapping_add(1 as i32 as u64),
            );
        }
        (*s).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*s).flags as u32 | flagvals::STRCUR as i32 as u32);
        if (*s).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 {
            r_free_wstr(s);
        }
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn r_dupnode(mut n: *mut NODE) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r as *mut block_item))
            .freep;
    } else {
        r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    *r = *n;
    (*r).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*r).flags as u32 | flagvals::MALLOC as i32 as u32);
    (*r).valref = 1 as i32 as i64;
    (*r).sub.val.wsp = 0 as *mut wchar_t;
    (*r).sub.val.wslen = 0 as i32 as size_t;
    if (*n).flags as u32 & flagvals::STRCUR as i32 as u32 != 0 as i32 as u32 {
        (*r).sub.val.sp = emalloc_real(
            ((*n).sub.val.slen).wrapping_add(1 as i32 as u64),
            b"r_dupnode\0" as *const u8 as *const i8,
            b"r->stptr\0" as *const u8 as *const i8,
            b"node.c\0" as *const u8 as *const i8,
            349 as i32,
        ) as *mut i8;
        memcpy(
            (*r).sub.val.sp as *mut libc::c_void,
            (*n).sub.val.sp as *const libc::c_void,
            (*n).sub.val.slen,
        );
        *((*r).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as i8;
        (*r).sub.val.slen = (*n).sub.val.slen;
        if (*n).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 as i32 as u32 {
            (*r).sub.val.wslen = (*n).sub.val.wslen;
            (*r).sub.val.wsp = emalloc_real(
                (::core::mem::size_of::<wchar_t>() as u64)
                    .wrapping_mul(((*n).sub.val.wslen).wrapping_add(1 as i32 as u64)),
                b"r_dupnode\0" as *const u8 as *const i8,
                b"r->wstptr\0" as *const u8 as *const i8,
                b"node.c\0" as *const u8 as *const i8,
                355 as i32,
            ) as *mut wchar_t;
            memcpy(
                (*r).sub.val.wsp as *mut libc::c_void,
                (*n).sub.val.wsp as *const libc::c_void,
                ((*n).sub.val.wslen)
                    .wrapping_mul(::core::mem::size_of::<wchar_t>() as u64),
            );
            *((*r).sub.val.wsp).offset((*n).sub.val.wslen as isize) = '\0' as i32;
            (*r).flags = ::core::mem::transmute::<
                u32,
                flagvals,
            >((*r).flags as u32 | flagvals::WSTRCUR as i32 as u32);
        }
    }
    return r;
}
unsafe extern "C" fn r_make_number(mut x: libc::c_double) -> *mut NODE {
    let mut r: *mut NODE = make_number_node(0 as i32 as u32);
    (*r).sub.val.fltnum = x;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn cmp_awknums(mut t1: *const NODE, mut t2: *const NODE) -> i32 {
    if if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf((*t1).sub.val.fltnum as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isnan((*t1).sub.val.fltnum)
    } else {
        __isnanl(f128::f128::new((*t1).sub.val.fltnum))
    } != 0
    {
        return (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_float>() as u64
        {
            __isnanf((*t2).sub.val.fltnum as libc::c_float)
        } else if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_double>() as u64
        {
            __isnan((*t2).sub.val.fltnum)
        } else {
            __isnanl(f128::f128::new((*t2).sub.val.fltnum))
        } == 0) as i32;
    }
    if if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf((*t2).sub.val.fltnum as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isnan((*t2).sub.val.fltnum)
    } else {
        __isnanl(f128::f128::new((*t2).sub.val.fltnum))
    } != 0
    {
        return -(1 as i32);
    }
    if (*t1).sub.val.fltnum == (*t2).sub.val.fltnum {
        return 0 as i32;
    }
    if (*t1).sub.val.fltnum < (*t2).sub.val.fltnum {
        return -(1 as i32);
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn make_str_node(
    mut s: *const i8,
    mut len: size_t,
    mut flags: i32,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r as *mut block_item))
            .freep;
    } else {
        r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    (*r).type_0 = nodevals::Node_val;
    (*r).sub.val.fltnum = 0 as i32 as libc::c_double;
    (*r).flags = flagvals::from_libc_c_uint(
        (flagvals::MALLOC as i32 | flagvals::STRING as i32 | flagvals::STRCUR as i32)
            as u32,
    );
    (*r).valref = 1 as i32 as i64;
    (*r).sub.val.idx = -(1 as i32);
    (*r).sub.val.wsp = 0 as *mut wchar_t;
    (*r).sub.val.wslen = 0 as i32 as size_t;
    if flags & 2 as i32 != 0 as i32 {
        (*r).sub.val.sp = s as *mut i8;
    } else {
        (*r).sub.val.sp = emalloc_real(
            len.wrapping_add(1 as i32 as u64),
            b"make_str_node\0" as *const u8 as *const i8,
            b"r->stptr\0" as *const u8 as *const i8,
            b"node.c\0" as *const u8 as *const i8,
            422 as i32,
        ) as *mut i8;
        memcpy((*r).sub.val.sp as *mut libc::c_void, s as *const libc::c_void, len);
    }
    *((*r).sub.val.sp).offset(len as isize) = '\0' as i32 as i8;
    if flags & 1 as i32 != 0 as i32 {
        let mut pf: *const i8 = 0 as *const i8;
        let mut ptm: *mut i8 = 0 as *mut i8;
        let mut c: i32 = 0;
        let mut end: *const i8 = 0 as *const i8;
        let mut cur_state: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut cur_state as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        end = &mut *((*r).sub.val.sp).offset(len as isize) as *mut i8;
        ptm = (*r).sub.val.sp;
        pf = ptm;
        while pf < end {
            if gawk_mb_cur_max > 1 as i32 {
                let mut mblen: i32 = mbrlen(
                    pf,
                    end.offset_from(pf) as i64 as size_t,
                    &mut cur_state,
                ) as i32;
                if mblen > 1 as i32 {
                    let mut i: i32 = 0;
                    i = 0 as i32;
                    while i < mblen {
                        let fresh1 = pf;
                        pf = pf.offset(1);
                        let fresh2 = ptm;
                        ptm = ptm.offset(1);
                        *fresh2 = *fresh1;
                        i += 1;
                        i;
                    }
                    continue;
                }
            }
            let fresh3 = pf;
            pf = pf.offset(1);
            c = *fresh3 as i32;
            if c == '\\' as i32 {
                c = parse_escape(&mut pf);
                if c < 0 as i32 {
                    if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"node.c\0" as *const u8 as *const i8, 460 as i32);
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"backslash string continuation is not portable\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    if flags & 4 as i32 != 0 as i32 {
                        continue;
                    }
                    c = '\\' as i32;
                }
                let fresh4 = ptm;
                ptm = ptm.offset(1);
                *fresh4 = c as i8;
            } else {
                let fresh5 = ptm;
                ptm = ptm.offset(1);
                *fresh5 = c as i8;
            }
        }
        len = ptm.offset_from((*r).sub.val.sp) as i64 as size_t;
        (*r).sub.val.sp = erealloc_real(
            (*r).sub.val.sp as *mut libc::c_void,
            len.wrapping_add(1 as i32 as u64),
            b"make_str_node\0" as *const u8 as *const i8,
            b"r->stptr\0" as *const u8 as *const i8,
            b"node.c\0" as *const u8 as *const i8,
            470 as i32,
        ) as *mut i8;
        *((*r).sub.val.sp).offset(len as isize) = '\0' as i32 as i8;
    }
    (*r).sub.val.slen = len;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn make_typed_regex(
    mut re: *const i8,
    mut len: size_t,
) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut exp: *mut NODE = 0 as *mut NODE;
    let mut n2: *mut NODE = 0 as *mut NODE;
    exp = make_str_node(re, len, 2 as i32);
    n = make_regnode(nodevals::Node_regex, exp);
    if n.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"node.c\0" as *const u8 as *const i8, 488 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"could not make typed regex\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    n2 = make_str_node(re, len, 0 as i32);
    (*n2).sub.val.typre = n;
    (*n2).sub.val.fltnum = 0 as i32 as libc::c_double;
    (*n2).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >(
        (*n2).flags as u32
            | (flagvals::NUMCUR as i32 | flagvals::STRCUR as i32
                | flagvals::REGEX as i32) as u32,
    );
    (*n2).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*n2).flags as u32 & !(flagvals::STRING as i32 | flagvals::NUMBER as i32) as u32);
    return n2;
}
#[no_mangle]
pub unsafe extern "C" fn r_unref(mut tmp: *mut NODE) {
    if (*tmp).flags as u32 & (flagvals::MALLOC as i32 | flagvals::STRCUR as i32) as u32
        == (flagvals::MALLOC as i32 | flagvals::STRCUR as i32) as u32
    {
        pma_free((*tmp).sub.val.sp as *mut libc::c_void);
    }
    mpfr_unset(tmp);
    if (*tmp).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 {
        r_free_wstr(tmp);
    }
    let ref mut fresh6 = (*(tmp as *mut block_item)).freep;
    *fresh6 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = tmp as *mut block_item;
}
#[no_mangle]
pub unsafe extern "C" fn parse_escape(mut string_ptr: *mut *const i8) -> i32 {
    let fresh7 = *string_ptr;
    *string_ptr = (*string_ptr).offset(1);
    let mut c: i32 = *fresh7 as i32;
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut j: i32 = 0;
    let mut start: *const i8 = 0 as *const i8;
    if do_flags as u32 & do_flag_values::DO_LINT_OLD as i32 as u32 != 0 {
        match c {
            97 | 98 | 102 | 114 => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"node.c\0" as *const u8 as *const i8, 561 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"old awk does not support the `\\%c' escape sequence\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    c,
                );
            }
            _ => {}
        }
    }
    match c {
        97 => return '\u{7}' as i32,
        98 => return '\u{8}' as i32,
        102 => return '\u{c}' as i32,
        110 => return '\n' as i32,
        114 => return '\r' as i32,
        116 => return '\t' as i32,
        118 => return '\u{b}' as i32,
        10 => return -(2 as i32),
        0 => {
            *string_ptr = (*string_ptr).offset(-1);
            *string_ptr;
            return -(1 as i32);
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            i = c - '0' as i32;
            count = 0 as i32;
            loop {
                count += 1;
                if !(count < 3 as i32) {
                    break;
                }
                let fresh8 = *string_ptr;
                *string_ptr = (*string_ptr).offset(1);
                c = *fresh8 as i32;
                if c >= '0' as i32 && c <= '7' as i32 {
                    i *= 8 as i32;
                    i += c - '0' as i32;
                } else {
                    *string_ptr = (*string_ptr).offset(-1);
                    *string_ptr;
                    break;
                }
            }
            return i;
        }
        120 => {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                static mut warned: bool = 0 as i32 != 0;
                if !warned {
                    warned = 1 as i32 != 0;
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"node.c\0" as *const u8 as *const i8, 612 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"POSIX does not allow `\\x' escapes\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
            if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
                return 'x' as i32;
            }
            if *(*__ctype_b_loc())
                .offset(*(*string_ptr).offset(0 as i32 as isize) as u8 as i32 as isize)
                as i32 & C2RustUnnamed_0::_ISxdigit as i32 as libc::c_ushort as i32 == 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"node.c\0" as *const u8 as *const i8, 618 as i32);
                (Some(
                    (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"no hex digits in `\\x' escape sequence\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                return 'x' as i32;
            }
            start = *string_ptr;
            j = 0 as i32;
            i = j;
            while j < 2 as i32 {
                let fresh9 = *string_ptr;
                *string_ptr = (*string_ptr).offset(1);
                c = *fresh9 as u8 as i32;
                if *(*__ctype_b_loc()).offset(c as isize) as i32
                    & C2RustUnnamed_0::_ISxdigit as i32 as libc::c_ushort as i32 != 0
                {
                    i *= 16 as i32;
                    if *(*__ctype_b_loc()).offset(c as isize) as i32
                        & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        i += c - '0' as i32;
                    } else if *(*__ctype_b_loc()).offset(c as isize) as i32
                        & C2RustUnnamed_0::_ISupper as i32 as libc::c_ushort as i32 != 0
                    {
                        i += c - 'A' as i32 + 10 as i32;
                    } else {
                        i += c - 'a' as i32 + 10 as i32;
                    }
                    j += 1;
                    j;
                } else {
                    *string_ptr = (*string_ptr).offset(-1);
                    *string_ptr;
                    break;
                }
            }
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0 && j == 2 as i32
                && *(*__ctype_b_loc()).offset(**string_ptr as u8 as i32 as isize) as i32
                    & C2RustUnnamed_0::_ISxdigit as i32 as libc::c_ushort as i32 != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"node.c\0" as *const u8 as *const i8, 639 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"hex escape \\x%.*s of %d characters probably not interpreted the way you expect\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    3 as i32,
                    start,
                    3 as i32,
                );
            }
            return i;
        }
        92 | 34 => return c,
        _ => {
            static mut warned_0: [bool; 256] = [false; 256];
            let mut uc: u8 = c as u8;
            if !warned_0[uc as usize] {
                warned_0[uc as usize] = 1 as i32 != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"node.c\0" as *const u8 as *const i8, 654 as i32);
                (Some(
                    (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"escape sequence `\\%c' treated as plain `%c'\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    uc as i32,
                    uc as i32,
                );
            }
            return c;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_numbase(
    mut s: *const i8,
    mut len: size_t,
    mut use_locale: bool,
) -> i32 {
    let mut dec_point: i32 = '.' as i32;
    let mut str: *const i8 = s;
    if use_locale as i32 != 0 && !(loc.decimal_point).is_null()
        && *(loc.decimal_point).offset(0 as i32 as isize) as i32 != '\0' as i32
    {
        dec_point = *(loc.decimal_point).offset(0 as i32 as isize) as i32;
    }
    if len < 2 as i32 as u64 || *str.offset(0 as i32 as isize) as i32 != '0' as i32 {
        return 10 as i32;
    }
    if *str.offset(1 as i32 as isize) as i32 == 'x' as i32
        || *str.offset(1 as i32 as isize) as i32 == 'X' as i32
    {
        return 16 as i32;
    }
    while len > 0 as i32 as u64 {
        if *str as i32 == 'e' as i32 || *str as i32 == 'E' as i32
            || *str as i32 == dec_point
        {
            return 10 as i32
        } else {
            if *(*__ctype_b_loc()).offset(*str as u8 as i32 as isize) as i32
                & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32 == 0
            {
                break;
            }
            len = len.wrapping_sub(1);
            len;
            str = str.offset(1);
            str;
        }
    }
    if *(*__ctype_b_loc()).offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize)
        as i32 & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32 == 0
        || *s.offset(1 as i32 as isize) as i32 == '8' as i32
        || *s.offset(1 as i32 as isize) as i32 == '9' as i32
    {
        return 10 as i32;
    }
    return 8 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn str2wstr(
    mut n: *mut NODE,
    mut ptr: *mut *mut size_t,
) -> *mut NODE {
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    let mut src_count: size_t = 0;
    let mut sp: *mut i8 = 0 as *mut i8;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut wc: wchar_t = 0;
    let mut wsp: *mut wchar_t = 0 as *mut wchar_t;
    static mut warned: bool = 0 as i32 != 0;
    if n == Nnull_string || n == Null_field {
        return n;
    }
    if (*n).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 as i32 as u32 {
        if ptr.is_null() {
            return n;
        }
        if (*n).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 {
            r_free_wstr(n);
        }
    }
    (*n).sub.val.wsp = emalloc_real(
        (::core::mem::size_of::<wchar_t>() as u64)
            .wrapping_mul(((*n).sub.val.slen).wrapping_add(1 as i32 as u64)),
        b"str2wstr\0" as *const u8 as *const i8,
        b"n->wstptr\0" as *const u8 as *const i8,
        b"node.c\0" as *const u8 as *const i8,
        748 as i32,
    ) as *mut wchar_t;
    wsp = (*n).sub.val.wsp;
    if !ptr.is_null() {
        *ptr = ezalloc_real(
            (::core::mem::size_of::<size_t>() as u64).wrapping_mul((*n).sub.val.slen),
            b"str2wstr\0" as *const u8 as *const i8,
            b"*ptr\0" as *const u8 as *const i8,
            b"node.c\0" as *const u8 as *const i8,
            760 as i32,
        ) as *mut size_t;
    }
    sp = (*n).sub.val.sp;
    src_count = (*n).sub.val.slen;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    i = 0 as i32 as size_t;
    while src_count > 0 as i32 as u64 {
        if *btowc_cache.as_mut_ptr().offset((*sp as i32 & 0xff as i32) as isize)
            != 0xffffffff as u32
        {
            count = 1 as i32 as size_t;
            wc = *btowc_cache.as_mut_ptr().offset((*sp as i32 & 0xff as i32) as isize)
                as wchar_t;
        } else {
            count = mbrtowc(&mut wc, sp, src_count, &mut mbs);
        }
        let mut current_block_42: u64;
        match count {
            18446744073709551614 | 18446744073709551615 => {
                memset(
                    &mut mbs as *mut mbstate_t as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<mbstate_t>() as u64,
                );
                if !warned {
                    warned = 1 as i32 != 0;
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"node.c\0" as *const u8 as *const i8, 790 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"Invalid multibyte data detected. There may be a mismatch between your data and your locale\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                if using_utf8() {
                    count = 1 as i32 as size_t;
                    wc = 0xfffd as i32;
                    current_block_42 = 8138739325889939489;
                } else {
                    sp = sp.offset(1);
                    sp;
                    src_count = src_count.wrapping_sub(1);
                    src_count;
                    current_block_42 = 9007357115414505193;
                }
            }
            0 => {
                count = 1 as i32 as size_t;
                current_block_42 = 8138739325889939489;
            }
            _ => {
                current_block_42 = 8138739325889939489;
            }
        }
        match current_block_42 {
            8138739325889939489 => {
                let fresh10 = wsp;
                wsp = wsp.offset(1);
                *fresh10 = wc;
                src_count = (src_count as u64).wrapping_sub(count) as size_t as size_t;
                loop {
                    let fresh11 = count;
                    count = count.wrapping_sub(1);
                    if !(fresh11 != 0) {
                        break;
                    }
                    if !ptr.is_null() {
                        *(*ptr)
                            .offset(sp.offset_from((*n).sub.val.sp) as i64 as isize) = i;
                    }
                    sp = sp.offset(1);
                    sp;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    *wsp = '\0' as i32;
    (*n).sub.val.wslen = wsp.offset_from((*n).sub.val.wsp) as i64 as size_t;
    (*n).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*n).flags as u32 | flagvals::WSTRCUR as i32 as u32);
    if ((*n).sub.val.slen).wrapping_sub((*n).sub.val.wslen) > 100 as i32 as u64 {
        (*n).sub.val.wsp = erealloc_real(
            (*n).sub.val.wsp as *mut libc::c_void,
            (::core::mem::size_of::<wchar_t>() as u64)
                .wrapping_mul(((*n).sub.val.wslen).wrapping_add(1 as i32 as u64)),
            b"str2wstr\0" as *const u8 as *const i8,
            b"n->wstptr\0" as *const u8 as *const i8,
            b"node.c\0" as *const u8 as *const i8,
            837 as i32,
        ) as *mut wchar_t;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn wstr2str(mut n: *mut NODE) -> *mut NODE {
    let mut result: size_t = 0;
    let mut length: size_t = 0;
    let mut wp: *mut wchar_t = 0 as *mut wchar_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut newval: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    length = (*n).sub.val.wslen;
    newval = emalloc_real(
        length.wrapping_mul(gawk_mb_cur_max as u64).wrapping_add(1 as i32 as u64),
        b"wstr2str\0" as *const u8 as *const i8,
        b"newval\0" as *const u8 as *const i8,
        b"node.c\0" as *const u8 as *const i8,
        864 as i32,
    ) as *mut i8;
    wp = (*n).sub.val.wsp;
    cp = newval;
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
    pma_free((*n).sub.val.sp as *mut libc::c_void);
    (*n).sub.val.sp = newval;
    (*n).sub.val.slen = cp.offset_from(newval) as i64 as size_t;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn r_free_wstr(mut n: *mut NODE) {
    if (*n).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 as i32 as u32 {
        pma_free((*n).sub.val.wsp as *mut libc::c_void);
    }
    (*n).sub.val.wsp = 0 as *mut wchar_t;
    (*n).sub.val.wslen = 0 as i32 as size_t;
    (*n).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*n).flags as u32 & !(flagvals::WSTRCUR as i32) as u32);
}
#[no_mangle]
pub unsafe extern "C" fn wstrstr(
    mut haystack: *const wchar_t,
    mut hs_len: size_t,
    mut needle: *const wchar_t,
    mut needle_len: size_t,
) -> *const wchar_t {
    let mut i: size_t = 0;
    if haystack.is_null() || needle.is_null() || needle_len > hs_len {
        return 0 as *const wchar_t;
    }
    i = 0 as i32 as size_t;
    while i < hs_len {
        if *haystack.offset(i as isize) == *needle.offset(0 as i32 as isize)
            && i.wrapping_add(needle_len).wrapping_sub(1 as i32 as u64) < hs_len
            && *haystack
                .offset(
                    i.wrapping_add(needle_len).wrapping_sub(1 as i32 as u64) as isize,
                ) == *needle.offset(needle_len.wrapping_sub(1 as i32 as u64) as isize)
        {
            if memcmp(
                haystack.offset(i as isize) as *const libc::c_void,
                needle as *const libc::c_void,
                (::core::mem::size_of::<wchar_t>() as u64).wrapping_mul(needle_len),
            ) == 0 as i32
            {
                return haystack.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn wcasestrstr(
    mut haystack: *const wchar_t,
    mut hs_len: size_t,
    mut needle: *const wchar_t,
    mut needle_len: size_t,
) -> *const wchar_t {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if haystack.is_null() || needle.is_null() || needle_len > hs_len {
        return 0 as *const wchar_t;
    }
    i = 0 as i32 as size_t;
    while i < hs_len {
        let mut current_block_8: u64;
        if towlower(*haystack.offset(i as isize) as wint_t)
            == towlower(*needle.offset(0 as i32 as isize) as wint_t)
            && i.wrapping_add(needle_len).wrapping_sub(1 as i32 as u64) < hs_len
            && towlower(
                *haystack
                    .offset(
                        i.wrapping_add(needle_len).wrapping_sub(1 as i32 as u64) as isize,
                    ) as wint_t,
            )
                == towlower(
                    *needle.offset(needle_len.wrapping_sub(1 as i32 as u64) as isize)
                        as wint_t,
                )
        {
            let mut start: *const wchar_t = 0 as *const wchar_t;
            start = haystack.offset(i as isize);
            j = 0 as i32 as size_t;
            loop {
                if !(j < needle_len) {
                    current_block_8 = 1917311967535052937;
                    break;
                }
                let mut h: wchar_t = 0;
                let mut n: wchar_t = 0;
                h = towlower(*start as wint_t) as wchar_t;
                n = towlower(*needle.offset(j as isize) as wint_t) as wchar_t;
                if h != n {
                    current_block_8 = 1841672684692190573;
                    break;
                }
                j = j.wrapping_add(1);
                j;
                start = start.offset(1);
                start;
            }
            match current_block_8 {
                1841672684692190573 => {}
                _ => return haystack.offset(i as isize),
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn is_ieee_magic_val(mut val: *const i8) -> bool {
    return (*val.offset(0 as i32 as isize) as i32 == '+' as i32
        || *val.offset(0 as i32 as isize) as i32 == '-' as i32)
        && ((*val.offset(1 as i32 as isize) as i32 == 'i' as i32
            || *val.offset(1 as i32 as isize) as i32 == 'I' as i32)
            && (*val.offset(2 as i32 as isize) as i32 == 'n' as i32
                || *val.offset(2 as i32 as isize) as i32 == 'N' as i32)
            && (*val.offset(3 as i32 as isize) as i32 == 'f' as i32
                || *val.offset(3 as i32 as isize) as i32 == 'F' as i32)
            || (*val.offset(1 as i32 as isize) as i32 == 'n' as i32
                || *val.offset(1 as i32 as isize) as i32 == 'N' as i32)
                && (*val.offset(2 as i32 as isize) as i32 == 'a' as i32
                    || *val.offset(2 as i32 as isize) as i32 == 'A' as i32)
                && (*val.offset(3 as i32 as isize) as i32 == 'n' as i32
                    || *val.offset(3 as i32 as isize) as i32 == 'N' as i32));
}
unsafe extern "C" fn get_ieee_magic_val(mut val: *mut i8) -> libc::c_double {
    static mut first: bool = 1 as i32 != 0;
    static mut inf: libc::c_double = 0.;
    static mut nan: libc::c_double = 0.;
    let mut save: i8 = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    save = *val.offset(4 as i32 as isize);
    *val.offset(4 as i32 as isize) = '\0' as i32 as i8;
    let mut v: libc::c_double = strtod(val, &mut ptr);
    *val.offset(4 as i32 as isize) = save;
    if val == ptr {
        if first {
            first = 0 as i32 != 0;
            nan = sqrt(-1.0f64);
            inf = -log(0.0f64);
        }
        v = if *val.offset(1 as i32 as isize) as i32 == 'i' as i32
            || *val.offset(1 as i32 as isize) as i32 == 'I' as i32
        {
            inf
        } else {
            nan
        };
        if *val.offset(0 as i32 as isize) as i32 == '-' as i32 {
            v = -v;
        }
    }
    return v;
}
#[no_mangle]
pub static mut btowc_cache: [wint_t; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn init_btowc_cache() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <= 255 as i32 {
        btowc_cache[i as usize] = btowc(i);
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut nextfree: [block_header; 2] = [
    {
        let mut init = block_header {
            freep: 0 as *const block_item as *mut block_item,
            size: ::core::mem::size_of::<NODE>() as u64,
            name: b"node\0" as *const u8 as *const i8,
            highwater: 0,
        };
        init
    },
    {
        let mut init = block_header {
            freep: 0 as *const block_item as *mut block_item,
            size: ::core::mem::size_of::<BUCKET>() as u64,
            name: b"bucket\0" as *const u8 as *const i8,
            highwater: 0,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn more_blocks(mut id: i32) -> *mut libc::c_void {
    let mut freep: *mut block_item = 0 as *mut block_item;
    let mut np: *mut block_item = 0 as *mut block_item;
    let mut next: *mut block_item = 0 as *mut block_item;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut endp: *mut i8 = 0 as *mut i8;
    let mut size: size_t = 0;
    size = nextfree[id as usize].size;
    freep = emalloc_real(
        (100 as i32 as u64).wrapping_mul(size),
        b"more_blocks\0" as *const u8 as *const i8,
        b"freep\0" as *const u8 as *const i8,
        b"node.c\0" as *const u8 as *const i8,
        1075 as i32,
    ) as *mut block_item;
    p = freep as *mut i8;
    endp = p.offset((100 as i32 as u64).wrapping_mul(size) as isize);
    np = freep;
    loop {
        p = p.offset(size as isize);
        next = p as *mut block_item;
        if p >= endp {
            (*np).freep = 0 as *mut block_item;
            break;
        } else {
            (*np).freep = next;
            np = next;
        }
    }
    nextfree[id as usize].freep = (*freep).freep;
    nextfree[id as usize].highwater += 100 as i32 as i64;
    return freep as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn make_bool_node(mut value: bool) -> *mut NODE {
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut sval: *const i8 = 0 as *const i8;
    let mut nval: libc::c_double = 0.;
    sval = if value as i32 != 0 {
        b"1\0" as *const u8 as *const i8
    } else {
        b"0\0" as *const u8 as *const i8
    };
    nval = if value as i32 != 0 { 1.0f64 } else { 0.0f64 };
    val = make_number.expect("non-null function pointer")(nval);
    (*val).sub.val.sp = estrdup(sval, strlen(sval));
    (*val).sub.val.slen = strlen(sval);
    (*val).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >(
        (*val).flags as u32
            | (flagvals::NUMCUR as i32 | flagvals::STRCUR as i32
                | flagvals::BOOLVAL as i32) as u32,
    );
    (*val).sub.val.idx = -(1 as i32);
    return val;
}