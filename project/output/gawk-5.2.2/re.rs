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
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn re_compile_pattern(
        __pattern: *const i8,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const i8;
    fn re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const i8,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn regfree(__preg: *mut regex_t);
    fn dfaalloc() -> *mut dfa;
    fn dfasyntax(_: *mut dfa, _: *const localeinfo, _: reg_syntax_t, _: i32);
    fn dfacopysyntax(_: *mut dfa, _: *const dfa);
    fn dfacomp(_: *const i8, _: idx_t, _: *mut dfa, _: bool);
    fn dfaexec(
        d: *mut dfa,
        begin: *const i8,
        end: *mut i8,
        allow_nl: bool,
        count: *mut idx_t,
        backref: *mut bool,
    ) -> *mut i8;
    fn dfasuperset(d: *const dfa) -> *mut dfa;
    fn dfaisfast(_: *const dfa) -> bool;
    fn dfafree(_: *mut dfa);
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    static mut IGNORECASE: bool;
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: i32;
    static mut casetable: [i8; 0];
    fn r_unref(tmp: *mut NODE);
    fn cmp_nodes(t1: *mut NODE, t2: *mut NODE, use_strcmp: bool) -> i32;
    fn genflags2str(flagval: i32, tab: *const flagtab) -> *const i8;
    fn error(mesg: *const i8, _: ...);
    fn r_warning(mesg: *const i8, _: ...);
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn parse_escape(string_ptr: *mut *const i8) -> i32;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const i8, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn init_localeinfo(_: *mut localeinfo);
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
pub type regex_t = re_pattern_buffer;
pub type regoff_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type ptrdiff_t = i64;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    DFA_PLUS_WARN = 32,
    DFA_STAR_WARN = 16,
    DFA_STRAY_BACKSLASH_WARN = 8,
    DFA_CONFUSING_BRACKETS_ERROR = 4,
    DFA_EOL_NUL = 2,
    DFA_ANCHOR = 1,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::DFA_PLUS_WARN => 32,
            C2RustUnnamed_1::DFA_STAR_WARN => 16,
            C2RustUnnamed_1::DFA_STRAY_BACKSLASH_WARN => 8,
            C2RustUnnamed_1::DFA_CONFUSING_BRACKETS_ERROR => 4,
            C2RustUnnamed_1::DFA_EOL_NUL => 2,
            C2RustUnnamed_1::DFA_ANCHOR => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            32 => C2RustUnnamed_1::DFA_PLUS_WARN,
            16 => C2RustUnnamed_1::DFA_STAR_WARN,
            8 => C2RustUnnamed_1::DFA_STRAY_BACKSLASH_WARN,
            4 => C2RustUnnamed_1::DFA_CONFUSING_BRACKETS_ERROR,
            2 => C2RustUnnamed_1::DFA_EOL_NUL,
            1 => C2RustUnnamed_1::DFA_ANCHOR,
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
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub sub: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
    pub nodep: C2RustUnnamed_5,
    pub val: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub l: C2RustUnnamed_12,
    pub r: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub d: C2RustUnnamed_9,
    pub x: C2RustUnnamed_8,
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
pub union C2RustUnnamed_8 {
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
pub union C2RustUnnamed_9 {
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
    pub hs: C2RustUnnamed_11,
    pub hi: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bucket_item,
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
pub struct flagtab {
    pub val: i32,
    pub name: *const i8,
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
pub struct reclass {
    pub name: *const i8,
    pub len: size_t,
    pub warned: bool,
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
static mut syn: reg_syntax_t = 0;
static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
#[no_mangle]
pub unsafe extern "C" fn make_regexp(
    mut s: *const i8,
    mut len: size_t,
    mut ignorecase: bool,
    mut dfa: bool,
    mut canfatal: bool,
) -> *mut Regexp {
    static mut metas: [i8; 15] = unsafe {
        *::core::mem::transmute::<&[u8; 15], &mut [i8; 15]>(b".*+(){}[]|?^$\\\0")
    };
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut rerr: *const i8 = 0 as *const i8;
    let mut src: *const i8 = s;
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut buflen: size_t = 0;
    let mut end: *const i8 = s.offset(len as isize);
    let mut dest: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    let mut c2: i32 = 0;
    static mut first: bool = 1 as i32 != 0;
    static mut no_dfa: bool = 0 as i32 != 0;
    let mut i: i32 = 0;
    static mut dfaregs: [*mut dfa; 2] = [
        0 as *const dfa as *mut dfa,
        0 as *const dfa as *mut dfa,
    ];
    static mut nul_warned: bool = 0 as i32 != 0;
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && !nul_warned
        && !(memchr(s as *const libc::c_void, '\0' as i32, len)).is_null()
    {
        nul_warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"re.c\0" as *const u8 as *const i8, 58 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"behavior of matching a regexp containing NUL characters is not defined by POSIX\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    let mut is_multibyte: size_t = 0 as i32 as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    if first {
        no_dfa = !(getenv(b"GAWK_NO_DFA\0" as *const u8 as *const i8)).is_null();
    }
    check_bracket_exp(s as *mut i8, len);
    if buf.is_null() {
        buf = emalloc_real(
            len.wrapping_add(1 as i32 as u64),
            b"make_regexp\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"re.c\0" as *const u8 as *const i8,
            87 as i32,
        ) as *mut i8;
        buflen = len;
    } else if len > buflen {
        buf = erealloc_real(
            buf as *mut libc::c_void,
            len.wrapping_add(1 as i32 as u64),
            b"make_regexp\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"re.c\0" as *const u8 as *const i8,
            90 as i32,
        ) as *mut i8;
        buflen = len;
    }
    dest = buf;
    while src < end {
        if gawk_mb_cur_max > 1 as i32 && is_multibyte == 0 {
            is_multibyte = mbrlen(src, end.offset_from(src) as i64 as size_t, &mut mbs);
            if is_multibyte == 1 as i32 as u64 || is_multibyte == -(1 as i32) as size_t
                || is_multibyte == -(2 as i32) as size_t
                || is_multibyte == 0 as i32 as u64
            {
                is_multibyte = 0 as i32 as size_t;
            }
        }
        let mut ok_to_escape: *const i8 = 0 as *const i8;
        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
            ok_to_escape = b"{}()|*+?.^$\\[]/-\0" as *const u8 as *const i8;
        } else if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
            ok_to_escape = b"()|*+?.^$\\[]/-\0" as *const u8 as *const i8;
        } else {
            ok_to_escape = b"<>`'BywWsS{}()|*+?.^$\\[]/-\0" as *const u8 as *const i8;
        }
        if (gawk_mb_cur_max == 1 as i32 || is_multibyte == 0)
            && *src as i32 == '\\' as i32
        {
            src = src.offset(1);
            c = *src as i32;
            let mut current_block_61: u64;
            match c {
                0 => {
                    if src >= s.offset(len as isize) {
                        let fresh0 = dest;
                        dest = dest.offset(1);
                        *fresh0 = '\\' as i32 as i8;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"re.c\0" as *const u8 as *const i8, 127 as i32);
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"invalid NUL byte in dynamic regexp\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    current_block_61 = 2631791190359682872;
                }
                97 | 98 | 102 | 110 | 114 | 116 | 118 | 120 | 48 | 49 | 50 | 51 | 52 | 53
                | 54 | 55 => {
                    c2 = parse_escape(&mut src);
                    if c2 < 0 as i32 {
                        r_fatal(
                            b"internal error: file %s, line %d: received bad result %d from parse_escape()\0"
                                as *const u8 as *const i8,
                            b"re.c\0" as *const u8 as *const i8,
                            147 as i32,
                            c2,
                        );
                    }
                    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                        != 0
                        && do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
                        && (*(*__ctype_b_loc()).offset(c as isize) as i32
                            & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                            != 0 || c == 'x' as i32)
                        && !(strchr(b"()|*+?.^$\\[]\0" as *const u8 as *const i8, c2))
                            .is_null()
                    {
                        let fresh1 = dest;
                        dest = dest.offset(1);
                        *fresh1 = '\\' as i32 as i8;
                    }
                    let fresh2 = dest;
                    dest = dest.offset(1);
                    *fresh2 = c2 as i8;
                    if do_flags as u32
                        & (do_flag_values::DO_LINT_INVALID as i32
                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                        && !nul_warned && c2 == '\0' as i32
                    {
                        nul_warned = 1 as i32 != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"re.c\0" as *const u8 as *const i8, 163 as i32);
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"behavior of matching a regexp containing NUL characters is not defined by POSIX\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    current_block_61 = 2631791190359682872;
                }
                56 | 57 => {
                    let fresh3 = dest;
                    dest = dest.offset(1);
                    *fresh3 = c as i8;
                    src = src.offset(1);
                    src;
                    static mut warned: [bool; 2] = [false; 2];
                    if !warned[(c - '8' as i32) as usize] {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"re.c\0" as *const u8 as *const i8, 174 as i32);
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
                                b"regexp escape sequence `\\%c' treated as plain `%c'\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            c,
                            c,
                        );
                        warned[(c - '8' as i32) as usize] = 1 as i32 != 0;
                    }
                    current_block_61 = 2631791190359682872;
                }
                121 => {
                    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                        == 0
                    {
                        let fresh4 = dest;
                        dest = dest.offset(1);
                        *fresh4 = '\\' as i32 as i8;
                        let fresh5 = dest;
                        dest = dest.offset(1);
                        *fresh5 = 'b' as i32 as i8;
                        src = src.offset(1);
                        src;
                        current_block_61 = 2631791190359682872;
                    } else {
                        current_block_61 = 2764395577077883216;
                    }
                }
                _ => {
                    current_block_61 = 2764395577077883216;
                }
            }
            match current_block_61 {
                2764395577077883216 => {
                    if (strchr(ok_to_escape, c)).is_null() {
                        static mut warned_0: [bool; 256] = [false; 256];
                        if !warned_0[(c & 0xff as i32) as usize] {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(b"re.c\0" as *const u8 as *const i8, 193 as i32);
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
                                    b"regexp escape sequence `\\%c' is not a known regexp operator\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                c,
                            );
                            warned_0[(c & 0xff as i32) as usize] = 1 as i32 != 0;
                        }
                    }
                    let fresh6 = dest;
                    dest = dest.offset(1);
                    *fresh6 = '\\' as i32 as i8;
                    let fresh7 = dest;
                    dest = dest.offset(1);
                    *fresh7 = c as i8;
                    src = src.offset(1);
                    src;
                }
                _ => {}
            }
        } else {
            c = *src as i32;
            let fresh8 = src;
            src = src.offset(1);
            let fresh9 = dest;
            dest = dest.offset(1);
            *fresh9 = *fresh8;
        }
        if gawk_mb_cur_max > 1 as i32 && is_multibyte != 0 {
            is_multibyte = is_multibyte.wrapping_sub(1);
            is_multibyte;
        }
    }
    *dest = '\0' as i32 as i8;
    len = dest.offset_from(buf) as i64 as size_t;
    rp = ezalloc_real(
        ::core::mem::size_of::<Regexp>() as u64,
        b"make_regexp\0" as *const u8 as *const i8,
        b"rp\0" as *const u8 as *const i8,
        b"re.c\0" as *const u8 as *const i8,
        213 as i32,
    ) as *mut Regexp;
    (*rp).pat.allocated = 0 as i32 as __re_long_size_t;
    (*rp).pat.fastmap = emalloc_real(
        256 as i32 as size_t,
        b"make_regexp\0" as *const u8 as *const i8,
        b"rp->pat.fastmap\0" as *const u8 as *const i8,
        b"re.c\0" as *const u8 as *const i8,
        215 as i32,
    ) as *mut i8;
    ignorecase = ignorecase;
    if ignorecase {
        if gawk_mb_cur_max > 1 as i32 {
            syn
                |= ((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32;
            (*rp).pat.translate = 0 as *mut u8;
        } else {
            syn
                &= !(((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32);
            (*rp).pat.translate = casetable.as_mut_ptr() as *mut u8;
        }
    } else {
        (*rp).pat.translate = 0 as *mut u8;
        syn
            &= !(((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32);
    }
    if first {
        first = 0 as i32 != 0;
        dfaregs[0 as i32 as usize] = dfaalloc();
        dfaregs[1 as i32 as usize] = dfaalloc();
        dfasyntax(
            dfaregs[0 as i32 as usize],
            &mut localeinfo,
            syn,
            C2RustUnnamed_1::DFA_ANCHOR as i32,
        );
        dfasyntax(
            dfaregs[1 as i32 as usize],
            &mut localeinfo,
            syn
                | ((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32,
            C2RustUnnamed_1::DFA_ANCHOR as i32,
        );
    }
    re_set_syntax(syn);
    rerr = re_compile_pattern(buf, len, &mut (*rp).pat);
    if !rerr.is_null() {
        refree(rp);
        if !canfatal {
            error(b"%s: /%.*s/\0" as *const u8 as *const i8, rerr, len as i32, s);
            return 0 as *mut Regexp;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"re.c\0" as *const u8 as *const i8, 264 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"invalid regexp: %s: /%.*s/\0" as *const u8 as *const i8,
            rerr,
            len as i32,
            s,
        );
    }
    ((*rp).pat).set_newline_anchor(0 as i32 as u32);
    if dfa as i32 != 0 && !no_dfa {
        (*rp).dfareg = dfaalloc();
        dfacopysyntax((*rp).dfareg, dfaregs[ignorecase as usize]);
        dfacomp(buf, len as idx_t, (*rp).dfareg, 1 as i32 != 0);
    } else {
        (*rp).dfareg = 0 as *mut dfa;
    }
    i = 0 as i32;
    while (i as u64) < len {
        if !(strchr(metas.as_mut_ptr(), *buf.offset(i as isize) as i32)).is_null() {
            (*rp).has_meta = 1 as i32 != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = len.wrapping_sub(1 as i32 as u64) as i32;
    while i >= 0 as i32 {
        if !(strchr(
            b"*+|?{}\0" as *const u8 as *const i8,
            *buf.offset(i as isize) as i32,
        ))
            .is_null()
        {
            (*rp).maybe_long = 1 as i32 != 0;
            break;
        } else {
            i -= 1;
            i;
        }
    }
    return rp;
}
#[no_mangle]
pub unsafe extern "C" fn research(
    mut rp: *mut Regexp,
    mut str: *mut i8,
    mut start: i32,
    mut len: size_t,
    mut flags: i32,
) -> i32 {
    let mut ret: *const i8 = str;
    let mut try_backref: bool = 0 as i32 != 0;
    let mut need_start: i32 = 0;
    let mut no_bol: i32 = 0;
    let mut res: i32 = 0;
    need_start = (flags & 1 as i32 != 0 as i32) as i32;
    no_bol = (flags & 2 as i32 != 0 as i32) as i32;
    if no_bol != 0 {
        ((*rp).pat).set_not_bol(1 as i32 as u32);
    }
    if !((*rp).dfareg).is_null() && no_bol == 0 && need_start == 0 {
        let mut superset: *mut dfa = dfasuperset((*rp).dfareg);
        if !superset.is_null() {
            ret = dfaexec(
                superset,
                str.offset(start as isize),
                str.offset(start as isize).offset(len as isize),
                1 as i32 != 0,
                0 as *mut idx_t,
                0 as *mut bool,
            );
        }
        if !ret.is_null()
            && (need_start == 0
                || superset.is_null() && dfaisfast((*rp).dfareg) as i32 != 0)
        {
            ret = dfaexec(
                (*rp).dfareg,
                str.offset(start as isize),
                str.offset(start as isize).offset(len as isize),
                1 as i32 != 0,
                0 as *mut idx_t,
                &mut try_backref,
            );
        }
    }
    if !ret.is_null() {
        if ((*rp).dfareg).is_null() || start != 0 as i32 || no_bol != 0
            || need_start != 0 || try_backref as i32 != 0
        {
            res = re_search(
                &mut (*rp).pat,
                str,
                (start as u64).wrapping_add(len) as regoff_t,
                start,
                len as regoff_t,
                if need_start != 0 { &mut (*rp).regs } else { 0 as *mut re_registers },
            );
        } else {
            res = 1 as i32;
        }
    } else {
        res = -(1 as i32);
    }
    ((*rp).pat).set_not_bol(0 as i32 as u32);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn refree(mut rp: *mut Regexp) {
    if rp.is_null() {
        return;
    }
    (*rp).pat.translate = 0 as *mut u8;
    regfree(&mut (*rp).pat);
    if !((*rp).regs.start).is_null() {
        pma_free((*rp).regs.start as *mut libc::c_void);
    }
    if !((*rp).regs.end).is_null() {
        pma_free((*rp).regs.end as *mut libc::c_void);
    }
    if !((*rp).dfareg).is_null() {
        dfafree((*rp).dfareg);
        pma_free((*rp).dfareg as *mut libc::c_void);
    }
    pma_free(rp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dfaerror(mut s: *const i8) -> ! {
    (set_loc
        as unsafe extern "C" fn(
            *const i8,
            i32,
        ) -> ())(b"re.c\0" as *const u8 as *const i8, 390 as i32);
    (Some(
        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(b"%s\0" as *const u8 as *const i8, s);
    exit(2 as i32);
}
#[inline]
unsafe extern "C" fn re_cache_get(mut t: *mut NODE) -> *mut Regexp {
    if ((*t).sub.nodep.r.preg[IGNORECASE as usize]).is_null() {
        (*t).sub.nodep.r.preg[IGNORECASE as usize] = make_regexp(
            (*(*t).sub.nodep.x.extra).sub.val.sp,
            (*(*t).sub.nodep.x.extra).sub.val.slen,
            IGNORECASE,
            (*t).flags as u64 != 0,
            1 as i32 != 0,
        );
    }
    return (*t).sub.nodep.r.preg[IGNORECASE as usize];
}
#[no_mangle]
pub unsafe extern "C" fn re_update(mut t: *mut NODE) -> *mut Regexp {
    let mut t1: *mut NODE = 0 as *mut NODE;
    if (*t).type_0 as u32 == nodevals::Node_val as i32 as u32
        && (*t).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32
    {
        return re_cache_get((*t).sub.val.typre);
    }
    if (*t).sub.nodep.reflags as u32 & reflagvals::CONSTANT as i32 as u32
        != 0 as i32 as u32
    {
        return re_cache_get(t);
    }
    t1 = (*t).sub.nodep.x.extra;
    if !((*t).sub.nodep.l.lptr).is_null() {
        if cmp_nodes((*t).sub.nodep.l.lptr, t1, 1 as i32 != 0) == 0 as i32 {
            return re_cache_get(t);
        }
        unref((*t).sub.nodep.l.lptr);
    }
    (*t).sub.nodep.l.lptr = dupnode(t1);
    if !((*t).sub.nodep.r.preg[0 as i32 as usize]).is_null() {
        refree((*t).sub.nodep.r.preg[0 as i32 as usize]);
        (*t).sub.nodep.r.preg[0 as i32 as usize] = 0 as *mut Regexp;
    }
    if !((*t).sub.nodep.r.preg[1 as i32 as usize]).is_null() {
        refree((*t).sub.nodep.r.preg[1 as i32 as usize]);
        (*t).sub.nodep.r.preg[1 as i32 as usize] = 0 as *mut Regexp;
    }
    if (*t).flags as u32 > 0 as i32 as u32
        && {
            (*t).flags += 1;
            (*t).flags as u32 > 10 as i32 as u32
        }
    {
        (*t).flags = flagvals::from_libc_c_uint(0 as u32);
    }
    if ((*t).sub.nodep.l.lptr).is_null() {
        t1 = (*t).sub.nodep.x.extra;
        unref((*t).sub.nodep.l.lptr);
        (*t).sub.nodep.l.lptr = dupnode(t1);
    }
    return re_cache_get(t);
}
#[no_mangle]
pub unsafe extern "C" fn resetup() {
    init_localeinfo(&mut localeinfo);
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
        syn = ((1 as i32 as u64) << 1 as i32) << 1 as i32
            | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
            | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
            | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
            | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
            | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32 | 1 as i32 as u64
            | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32;
    } else if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        syn = 1 as i32 as u64
            | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | ((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
            | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
            | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
            | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
            | ((1 as i32 as u64) << 1 as i32) << 1 as i32
            | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32;
    } else {
        syn = (((1 as i32 as u64) << 1 as i32) << 1 as i32
            | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
            | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
            | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
            | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
            | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
            | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
            | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32 | 1 as i32 as u64
            | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
            & !((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32);
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        syn
            |= (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32
                | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32;
    }
    re_set_syntax(syn);
}
#[no_mangle]
pub unsafe extern "C" fn using_utf8() -> bool {
    return localeinfo.using_utf8;
}
#[no_mangle]
pub unsafe extern "C" fn reisstring(
    mut text: *const i8,
    mut len: size_t,
    mut re: *mut Regexp,
    mut buf: *const i8,
) -> i32 {
    let mut res: i32 = 0;
    let mut matched: *const i8 = 0 as *const i8;
    if (*re).has_meta {
        return 0 as i32;
    }
    matched = &*buf.offset(*((*re).regs.start).offset(0 as i32 as isize) as isize)
        as *const i8;
    res = (memcmp(text as *const libc::c_void, matched as *const libc::c_void, len)
        == 0 as i32) as i32;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn reflags2str(mut flagval: i32) -> *const i8 {
    static mut values: [flagtab; 26] = [
        {
            let mut init = flagtab {
                val: 1 as i32 as u64 as i32,
                name: b"RE_BACKSLASH_ESCAPE_IN_LISTS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((1 as i32 as u64) << 1 as i32) as i32,
                name: b"RE_BK_PLUS_QM\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((1 as i32 as u64) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_CHAR_CLASSES\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_CONTEXT_INDEP_ANCHORS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) as i32,
                name: b"RE_CONTEXT_INDEP_OPS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
                name: b"RE_CONTEXT_INVALID_OPS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_DOT_NEWLINE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_DOT_NOT_NULL\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    as i32,
                name: b"RE_HAT_LISTS_NOT_NEWLINE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) as i32,
                name: b"RE_INTERVALS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
                name: b"RE_LIMITED_OPS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NEWLINE_ALT\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NO_BK_BRACES\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) as i32,
                name: b"RE_NO_BK_PARENS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NO_BK_REFS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NO_BK_VBAR\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NO_EMPTY_RANGES\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    as i32,
                name: b"RE_UNMATCHED_RIGHT_PAREN_ORD\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) as i32,
                name: b"RE_NO_POSIX_BACKTRACKING\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NO_GNU_OPS\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_INVALID_INTERVAL_ORD\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    as i32,
                name: b"RE_ICASE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) as i32,
                name: b"RE_CARET_ANCHORS_HERE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
                name: b"RE_CONTEXT_INVALID_DUP\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                name: b"RE_NO_SUB\0" as *const u8 as *const i8,
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
    if flagval == 0 as i32 {
        return b"RE_SYNTAX_EMACS\0" as *const u8 as *const i8;
    }
    return genflags2str(flagval, values.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn dfawarn(mut dfa_warning: *const i8) {}
unsafe extern "C" fn check_bracket_exp(mut s: *mut i8, mut length: size_t) {
    static mut classes: [reclass; 13] = [
        {
            let mut init = reclass {
                name: b"[:alpha:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:digit:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:alnum:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:upper:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:lower:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:space:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:xdigit:]\0" as *const u8 as *const i8,
                len: 10 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:punct:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:print:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:graph:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:cntrl:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:blank:]\0" as *const u8 as *const i8,
                len: 9 as i32 as size_t,
                warned: 0 as i32 != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: 0 as *const i8,
                len: 0 as i32 as size_t,
                warned: false,
            };
            init
        },
    ];
    let mut i: i32 = 0;
    let mut found: bool = 0 as i32 != 0;
    let mut save: i8 = 0;
    let mut sp: *mut i8 = 0 as *mut i8;
    let mut sp2: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut count: i32 = 0 as i32;
    if length == 0 as i32 as u64 {
        return;
    }
    end = s.offset(length as isize);
    save = *s.offset(length as isize);
    *s.offset(length as isize) = '\0' as i32 as i8;
    sp = s;
    loop {
        sp2 = memchr(
            sp as *const libc::c_void,
            '[' as i32,
            end.offset_from(sp) as i64 as u64,
        ) as *mut i8;
        sp = sp2;
        if sp.is_null() {
            break;
        }
        count += 1;
        count;
        sp = sp.offset(1);
        sp;
        while *sp as i32 != '\0' as i32 {
            if *sp as i32 == '[' as i32 {
                count += 1;
                count;
            }
            if *sp as i32 == ']' as i32 && sp > sp2 {
                if !(*sp.offset(-(1 as i32) as isize) as i32 != '[' as i32
                    && *sp.offset(-(1 as i32) as isize) as i32 != '\\' as i32)
                {
                    if !(sp.offset_from(sp2) as i64 >= 2 as i32 as i64
                        && *sp.offset(-(1 as i32) as isize) as i32 == '^' as i32
                        && *sp.offset(-(2 as i32) as isize) as i32 == '[' as i32)
                    {
                        count -= 1;
                        count;
                    }
                }
            }
            if count == 0 as i32 {
                sp = sp.offset(1);
                sp;
                break;
            } else {
                sp = sp.offset(1);
                sp;
            }
        }
        if count > 0 as i32 {
            break;
        }
        i = 0 as i32;
        while !(classes[i as usize].name).is_null() {
            if !classes[i as usize].warned {
                len = classes[i as usize].len as i32;
                if len as i64 == sp.offset_from(sp2) as i64
                    && memcmp(
                        sp2 as *const libc::c_void,
                        classes[i as usize].name as *const libc::c_void,
                        len as u64,
                    ) == 0 as i32
                {
                    found = 1 as i32 != 0;
                    break;
                }
            }
            i += 1;
            i;
        }
        if found as i32 != 0 && !classes[i as usize].warned {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"re.c\0" as *const u8 as *const i8, 669 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"regexp component `%.*s' should probably be `[%.*s]'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                len,
                sp2,
                len,
                sp2,
            );
            classes[i as usize].warned = 1 as i32 != 0;
        }
        if !(sp < end) {
            break;
        }
        found = 0 as i32 != 0;
    }
    *s.offset(length as isize) = save;
}