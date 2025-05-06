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
    pub type buffer;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(_: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const i8,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const i8;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const i8,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn rpl_regfree(__preg: *mut regex_t);
    fn dfaalloc() -> *mut dfa;
    fn dfasyntax(_: *mut dfa, _: *const localeinfo, _: reg_syntax_t, _: i32);
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
    fn panic(str: *const i8, _: ...);
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn get_buffer(b: *const buffer) -> *mut i8;
    fn size_buffer(b: *const buffer) -> size_t;
    fn bad_prog(why: *const i8);
    fn normalize_text(text: *mut i8, len: size_t, buftype: text_types) -> size_t;
    static mut buffer_delimiter: i8;
    static mut extended_regexp_flags: i32;
    static mut localeinfo: localeinfo;
    static mut posixicity: posixicity_types;
}
pub type size_t = u64;
pub type wint_t = u32;
pub type ptrdiff_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type idx_t = ptrdiff_t;
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
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
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
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
pub enum C2RustUnnamed {
    DFA_PLUS_WARN = 32,
    DFA_STAR_WARN = 16,
    DFA_STRAY_BACKSLASH_WARN = 8,
    DFA_CONFUSING_BRACKETS_ERROR = 4,
    DFA_EOL_NUL = 2,
    DFA_ANCHOR = 1,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::DFA_PLUS_WARN => 32,
            C2RustUnnamed::DFA_STAR_WARN => 16,
            C2RustUnnamed::DFA_STRAY_BACKSLASH_WARN => 8,
            C2RustUnnamed::DFA_CONFUSING_BRACKETS_ERROR => 4,
            C2RustUnnamed::DFA_EOL_NUL => 2,
            C2RustUnnamed::DFA_ANCHOR => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            32 => C2RustUnnamed::DFA_PLUS_WARN,
            16 => C2RustUnnamed::DFA_STAR_WARN,
            8 => C2RustUnnamed::DFA_STRAY_BACKSLASH_WARN,
            4 => C2RustUnnamed::DFA_CONFUSING_BRACKETS_ERROR,
            2 => C2RustUnnamed::DFA_EOL_NUL,
            1 => C2RustUnnamed::DFA_ANCHOR,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: regex_t,
    pub flags: i32,
    pub sz: size_t,
    pub dfa: *mut dfa,
    pub begline: bool,
    pub endline: bool,
    pub re: [i8; 1],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum text_types {
    TEXT_BUFFER,
    TEXT_REPLACEMENT,
    TEXT_REGEX,
}
impl text_types {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            text_types::TEXT_BUFFER => 0,
            text_types::TEXT_REPLACEMENT => 1,
            text_types::TEXT_REGEX => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> text_types {
        match value {
            0 => text_types::TEXT_BUFFER,
            1 => text_types::TEXT_REPLACEMENT,
            2 => text_types::TEXT_REGEX,
            _ => panic!("Invalid value for text_types: {}", value),
        }
    }
}
impl AddAssign<u32> for text_types {
    fn add_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for text_types {
    fn sub_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for text_types {
    fn mul_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for text_types {
    fn div_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for text_types {
    fn rem_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for text_types {
    type Output = text_types;
    fn add(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for text_types {
    type Output = text_types;
    fn sub(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for text_types {
    type Output = text_types;
    fn mul(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for text_types {
    type Output = text_types;
    fn div(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for text_types {
    type Output = text_types;
    fn rem(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum posixicity_types {
    POSIXLY_EXTENDED,
    POSIXLY_CORRECT,
    POSIXLY_BASIC,
}
impl posixicity_types {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            posixicity_types::POSIXLY_EXTENDED => 0,
            posixicity_types::POSIXLY_CORRECT => 1,
            posixicity_types::POSIXLY_BASIC => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> posixicity_types {
        match value {
            0 => posixicity_types::POSIXLY_EXTENDED,
            1 => posixicity_types::POSIXLY_CORRECT,
            2 => posixicity_types::POSIXLY_BASIC,
            _ => panic!("Invalid value for posixicity_types: {}", value),
        }
    }
}
impl AddAssign<u32> for posixicity_types {
    fn add_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for posixicity_types {
    fn sub_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for posixicity_types {
    fn mul_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for posixicity_types {
    fn div_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for posixicity_types {
    fn rem_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for posixicity_types {
    type Output = posixicity_types;
    fn add(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for posixicity_types {
    type Output = posixicity_types;
    fn sub(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for posixicity_types {
    type Output = posixicity_types;
    fn mul(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for posixicity_types {
    type Output = posixicity_types;
    fn div(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for posixicity_types {
    type Output = posixicity_types;
    fn rem(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
static mut errors: [i8; 72] = unsafe {
    *::core::mem::transmute::<
        &[u8; 72],
        &[i8; 72],
    >(b"no previous regular expression\0cannot specify modifiers on empty regexp\0")
};
#[no_mangle]
pub unsafe extern "C" fn dfaerror(mut mesg: *const i8) {
    panic(b"%s\0" as *const u8 as *const i8, mesg);
}
#[no_mangle]
pub unsafe extern "C" fn dfawarn(mut mesg: *const i8) {
    if (getenv(b"posixicity_types::POSIXLY_CORRECT\0" as *const u8 as *const i8))
        .is_null()
    {
        dfaerror(mesg);
    }
}
unsafe extern "C" fn compile_regex_1(mut new_regex: *mut regex, mut needed_sub: i32) {
    let mut error: *const i8 = 0 as *const i8;
    let mut syntax: i32 = (if extended_regexp_flags & 1 as i32 != 0 {
        ((1 as i32 as u64) << 1 as i32) << 1 as i32
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
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
    } else {
        ((1 as i32 as u64) << 1 as i32) << 1 as i32
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
                << 1 as i32) << 1 as i32) << 1 as i32 | (1 as i32 as u64) << 1 as i32
            | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32
    }) as i32;
    syntax = (syntax as u64
        & !((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
            << 1 as i32) << 1 as i32) << 1 as i32)) as i32;
    syntax = (syntax as u64
        | ((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
            << 1 as i32) << 1 as i32) << 1 as i32) as i32;
    match posixicity as u32 {
        0 => {
            syntax = (syntax as u64
                & !((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32))
                as i32;
        }
        1 => {
            syntax = (syntax as u64
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                as i32;
        }
        2 => {
            syntax = (syntax as u64
                | ((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32)) as i32;
            if extended_regexp_flags & 1 as i32 == 0 {
                syntax = (syntax as u64
                    | ((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) as i32;
            }
        }
        _ => {}
    }
    if (*new_regex).flags & (1 as i32) << 1 as i32 != 0 {
        syntax = (syntax as u64
            | ((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32;
    } else {
        (*new_regex).pattern.fastmap = malloc(
            ((1 as i32)
                << (::core::mem::size_of::<i8>() as u64).wrapping_mul(8 as i32 as u64))
                as u64,
        ) as *mut i8;
    }
    syntax = (syntax as u64
        | if needed_sub != 0 {
            0 as i32 as u64
        } else {
            (((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32
        }) as i32;
    if (*new_regex).flags & (1 as i32) << 2 as i32 != 0 {
        syntax = (syntax as u64
            & !(((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32)) as i32;
        syntax = (syntax as u64
            | ((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32;
    }
    rpl_re_set_syntax(syntax as reg_syntax_t);
    error = rpl_re_compile_pattern(
        ((*new_regex).re).as_mut_ptr(),
        (*new_regex).sz,
        &mut (*new_regex).pattern,
    );
    ((*new_regex).pattern)
        .set_newline_anchor(
            (buffer_delimiter as i32 == '\n' as i32
                && (*new_regex).flags & (1 as i32) << 2 as i32 != 0 as i32) as i32 as u32,
        );
    (*new_regex).pattern.translate = 0 as *mut u8;
    if !error.is_null() {
        bad_prog(error);
    }
    if needed_sub != 0 && (*new_regex).pattern.re_nsub < (needed_sub - 1 as i32) as u64
        && posixicity as u32 == posixicity_types::POSIXLY_EXTENDED as i32 as u32
    {
        let mut buf: [i8; 200] = [0; 200];
        sprintf(
            buf.as_mut_ptr(),
            dcgettext(
                0 as *const i8,
                b"invalid reference \\%d on `s' command's RHS\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            needed_sub - 1 as i32,
        );
        bad_prog(buf.as_mut_ptr());
    }
    let mut dfaopts: i32 = if buffer_delimiter as i32 == '\n' as i32 {
        0 as i32
    } else {
        C2RustUnnamed::DFA_EOL_NUL as i32
    };
    (*new_regex).dfa = dfaalloc();
    dfasyntax((*new_regex).dfa, &mut localeinfo, syntax as reg_syntax_t, dfaopts);
    dfacomp(
        ((*new_regex).re).as_mut_ptr(),
        (*new_regex).sz as idx_t,
        (*new_regex).dfa,
        1 as i32 != 0,
    );
    if (*new_regex).sz == 1 as i32 as u64 {
        if *((*new_regex).re).as_mut_ptr().offset(0 as i32 as isize) as i32 == '^' as i32
        {
            (*new_regex).begline = 1 as i32 != 0;
        }
        if *((*new_regex).re).as_mut_ptr().offset(0 as i32 as isize) as i32 == '$' as i32
        {
            (*new_regex).endline = 1 as i32 != 0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn compile_regex(
    mut b: *mut buffer,
    mut flags: i32,
    mut needed_sub: i32,
) -> *mut regex {
    let mut new_regex: *mut regex = 0 as *mut regex;
    let mut re_len: size_t = 0;
    if size_buffer(b) == 0 as i32 as u64 {
        if flags > 0 as i32 {
            bad_prog(
                dcgettext(
                    0 as *const i8,
                    errors
                        .as_ptr()
                        .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize),
                    5 as i32,
                ),
            );
        }
        return 0 as *mut regex;
    }
    re_len = size_buffer(b);
    new_regex = xzalloc(
        (::core::mem::size_of::<regex>() as u64)
            .wrapping_add(re_len)
            .wrapping_sub(1 as i32 as u64),
    ) as *mut regex;
    (*new_regex).flags = flags;
    memcpy(
        ((*new_regex).re).as_mut_ptr() as *mut libc::c_void,
        get_buffer(b) as *const libc::c_void,
        re_len,
    );
    (*new_regex).sz = normalize_text(
        ((*new_regex).re).as_mut_ptr(),
        re_len,
        text_types::TEXT_REGEX,
    );
    compile_regex_1(new_regex, needed_sub);
    return new_regex;
}
#[no_mangle]
pub unsafe extern "C" fn match_regex(
    mut regex: *mut regex,
    mut buf: *mut i8,
    mut buflen: size_t,
    mut buf_start_offset: size_t,
    mut regarray: *mut re_registers,
    mut regsize: i32,
) -> i32 {
    let mut ret: i32 = 0;
    static mut regex_last: *mut regex = 0 as *const regex as *mut regex;
    if regex.is_null() {
        regex = regex_last;
        if regex_last.is_null() {
            bad_prog(dcgettext(0 as *const i8, errors.as_ptr(), 5 as i32));
        }
    } else {
        regex_last = regex;
    }
    if buflen >= 2147483647 as i32 as u64 {
        panic(
            dcgettext(
                0 as *const i8,
                b"regex input buffer length larger than INT_MAX\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    if ((*regex).pattern).no_sub() as i32 != 0 && regsize != 0 {
        if !((*regex).dfa).is_null() {
            dfafree((*regex).dfa);
            rpl_free((*regex).dfa as *mut libc::c_void);
            (*regex).dfa = 0 as *mut dfa;
        }
        rpl_regfree(&mut (*regex).pattern);
        compile_regex_1(regex, regsize);
    }
    ((*regex).pattern).set_regs_allocated(1 as i32 as u32);
    if (*regex).begline as i32 != 0 || (*regex).endline as i32 != 0 {
        let mut offset: size_t = 0;
        if (*regex).endline {
            let mut p: *const i8 = 0 as *const i8;
            if (*regex).flags & (1 as i32) << 2 as i32 != 0 {
                p = memchr(
                    buf.offset(buf_start_offset as isize) as *const libc::c_void,
                    buffer_delimiter as i32,
                    buflen.wrapping_sub(buf_start_offset),
                ) as *const i8;
            }
            offset = if !p.is_null() {
                p.offset_from(buf) as i64 as u64
            } else {
                buflen
            };
        } else if buf_start_offset == 0 as i32 as u64 {
            offset = 0 as i32 as size_t;
        } else if (*regex).flags & (1 as i32) << 2 as i32 == 0 {
            return 0 as i32
        } else if *buf.offset(buf_start_offset.wrapping_sub(1 as i32 as u64) as isize)
            as i32 == buffer_delimiter as i32
        {
            offset = buf_start_offset;
        } else {
            let mut p_0: *const i8 = memchr(
                buf.offset(buf_start_offset as isize) as *const libc::c_void,
                buffer_delimiter as i32,
                buflen.wrapping_sub(buf_start_offset),
            ) as *const i8;
            if p_0.is_null() {
                return 0 as i32;
            }
            offset = (p_0.offset_from(buf) as i64 + 1 as i32 as i64) as size_t;
        }
        if regsize != 0 {
            let mut i: size_t = 0;
            if ((*regarray).start).is_null() {
                (*regarray).start = (if ::core::mem::size_of::<regoff_t>() as u64
                    == 1 as i32 as u64
                {
                    xzalloc(1 as i32 as size_t)
                } else {
                    xcalloc(
                        1 as i32 as size_t,
                        ::core::mem::size_of::<regoff_t>() as u64,
                    )
                }) as *mut regoff_t;
                (*regarray).end = (if ::core::mem::size_of::<regoff_t>() as u64
                    == 1 as i32 as u64
                {
                    xzalloc(1 as i32 as size_t)
                } else {
                    xcalloc(
                        1 as i32 as size_t,
                        ::core::mem::size_of::<regoff_t>() as u64,
                    )
                }) as *mut regoff_t;
                (*regarray).num_regs = 1 as i32 as __re_size_t;
            }
            *((*regarray).start).offset(0 as i32 as isize) = offset as regoff_t;
            *((*regarray).end).offset(0 as i32 as isize) = offset as regoff_t;
            i = 1 as i32 as size_t;
            while i < (*regarray).num_regs {
                let ref mut fresh0 = *((*regarray).end).offset(i as isize);
                *fresh0 = -(1 as i32) as regoff_t;
                *((*regarray).start).offset(i as isize) = *fresh0;
                i = i.wrapping_add(1);
                i;
            }
        }
        return 1 as i32;
    }
    if buf_start_offset == 0 as i32 as u64 {
        let mut superset: *mut dfa = dfasuperset((*regex).dfa);
        if !superset.is_null()
            && (dfaexec(
                superset,
                buf,
                buf.offset(buflen as isize),
                1 as i32 != 0,
                0 as *mut idx_t,
                0 as *mut bool,
            ))
                .is_null()
        {
            return 0 as i32;
        }
        if regsize == 0 && (*regex).flags & (1 as i32) << 2 as i32 != 0
            || superset.is_null() && dfaisfast((*regex).dfa) as i32 != 0
        {
            let mut backref: bool = 0 as i32 != 0;
            if (dfaexec(
                (*regex).dfa,
                buf,
                buf.offset(buflen as isize),
                1 as i32 != 0,
                0 as *mut idx_t,
                &mut backref,
            ))
                .is_null()
            {
                return 0 as i32;
            }
            if regsize == 0 && (*regex).flags & (1 as i32) << 2 as i32 != 0 && !backref {
                return 1 as i32;
            }
        }
    }
    if (*regex).flags & (1 as i32) << 2 as i32 != 0
        && buffer_delimiter as i32 != '\n' as i32
    {
        let mut beg: *const i8 = 0 as *const i8;
        let mut end: *const i8 = 0 as *const i8;
        let mut start: *const i8 = 0 as *const i8;
        beg = buf;
        if buf_start_offset > 0 as i32 as u64 {
            let mut eol: *const i8 = memrchr(
                buf as *const libc::c_void,
                buffer_delimiter as i32,
                buf_start_offset,
            ) as *const i8;
            if !eol.is_null() {
                beg = eol.offset(1 as i32 as isize);
            }
        }
        start = buf.offset(buf_start_offset as isize);
        loop {
            end = memchr(
                beg as *const libc::c_void,
                buffer_delimiter as i32,
                buf.offset(buflen as isize).offset_from(beg) as i64 as u64,
            ) as *const i8;
            if end.is_null() {
                end = buf.offset(buflen as isize);
            }
            ret = rpl_re_search(
                &mut (*regex).pattern,
                beg,
                end.offset_from(beg) as i64,
                start.offset_from(beg) as i64,
                end.offset_from(start) as i64,
                if regsize != 0 { regarray } else { 0 as *mut re_registers },
            ) as i32;
            if ret > -(1 as i32) {
                let mut i_0: size_t = 0;
                ret = (ret as i64 + beg.offset_from(buf) as i64) as i32;
                if regsize != 0 {
                    i_0 = 0 as i32 as size_t;
                    while i_0 < (*regarray).num_regs {
                        if *((*regarray).start).offset(i_0 as isize) > -(1 as i32) as i64
                        {
                            let ref mut fresh1 = *((*regarray).start)
                                .offset(i_0 as isize);
                            *fresh1 += beg.offset_from(buf) as i64;
                        }
                        if *((*regarray).end).offset(i_0 as isize) > -(1 as i32) as i64 {
                            let ref mut fresh2 = *((*regarray).end).offset(i_0 as isize);
                            *fresh2 += beg.offset_from(buf) as i64;
                        }
                        i_0 = i_0.wrapping_add(1);
                        i_0;
                    }
                }
                break;
            } else {
                if end == buf.offset(buflen as isize) {
                    break;
                }
                start = end.offset(1 as i32 as isize);
                beg = start;
            }
        }
    } else {
        ret = rpl_re_search(
            &mut (*regex).pattern,
            buf,
            buflen as regoff_t,
            buf_start_offset as regoff_t,
            buflen.wrapping_sub(buf_start_offset) as regoff_t,
            if regsize != 0 { regarray } else { 0 as *mut re_registers },
        ) as i32;
    }
    return (ret > -(1 as i32)) as i32;
}