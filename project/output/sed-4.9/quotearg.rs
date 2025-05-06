#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn abort() -> !;
    fn xcharalloc(n: size_t) -> *mut i8;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn locale_charset() -> *const i8;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswprint(__wc: wint_t) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type ptrdiff_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_flags {
    QA_ELIDE_NULL_BYTES = 0x1,
    QA_ELIDE_OUTER_QUOTES = 0x2,
    QA_SPLIT_TRIGRAPHS = 0x4,
}
impl quoting_flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_flags::QA_ELIDE_NULL_BYTES => 0x1,
            quoting_flags::QA_ELIDE_OUTER_QUOTES => 0x2,
            quoting_flags::QA_SPLIT_TRIGRAPHS => 0x4,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_flags {
        match value {
            0x1 => quoting_flags::QA_ELIDE_NULL_BYTES,
            0x2 => quoting_flags::QA_ELIDE_OUTER_QUOTES,
            0x4 => quoting_flags::QA_SPLIT_TRIGRAPHS,
            _ => panic!("Invalid value for quoting_flags: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_flags {
    type Output = quoting_flags;
    fn add(self, rhs: u32) -> quoting_flags {
        quoting_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_flags {
    type Output = quoting_flags;
    fn sub(self, rhs: u32) -> quoting_flags {
        quoting_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_flags {
    type Output = quoting_flags;
    fn mul(self, rhs: u32) -> quoting_flags {
        quoting_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_flags {
    type Output = quoting_flags;
    fn div(self, rhs: u32) -> quoting_flags {
        quoting_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_flags {
    type Output = quoting_flags;
    fn rem(self, rhs: u32) -> quoting_flags {
        quoting_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quoting_options {
    pub style: quoting_style,
    pub flags: i32,
    pub quote_these_too: [u32; 8],
    pub left_quote: *const i8,
    pub right_quote: *const i8,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type wint_t = u32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    _ISprint = 16384,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
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
            C2RustUnnamed_0::_ISprint => 16384,
            C2RustUnnamed_0::_ISalnum => 8,
            C2RustUnnamed_0::_ISpunct => 4,
            C2RustUnnamed_0::_IScntrl => 2,
            C2RustUnnamed_0::_ISblank => 1,
            C2RustUnnamed_0::_ISgraph => 32768,
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
            16384 => C2RustUnnamed_0::_ISprint,
            8 => C2RustUnnamed_0::_ISalnum,
            4 => C2RustUnnamed_0::_ISpunct,
            2 => C2RustUnnamed_0::_IScntrl,
            1 => C2RustUnnamed_0::_ISblank,
            32768 => C2RustUnnamed_0::_ISgraph,
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
pub struct slotvec {
    pub size: size_t,
    pub val: *mut i8,
}
pub type idx_t = ptrdiff_t;
#[inline]
unsafe extern "C" fn strcaseeq9(mut s1: *const i8, mut s2: *const i8) -> i32 {
    return (c_strcasecmp(s1.offset(9 as i32 as isize), s2.offset(9 as i32 as isize))
        == 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn strcaseeq8(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s28 as i32) as i32 != 0 {
        (*s1.offset(8 as i32 as isize) as i32 & !(0x20 as i32) == s28 as i32) as i32
    } else {
        (*s1.offset(8 as i32 as isize) as i32 == s28 as i32) as i32
    } != 0
    {
        if s28 as i32 == 0 as i32 { return 1 as i32 } else { return strcaseeq9(s1, s2) }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq7(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s27 as i32) as i32 != 0 {
        (*s1.offset(7 as i32 as isize) as i32 & !(0x20 as i32) == s27 as i32) as i32
    } else {
        (*s1.offset(7 as i32 as isize) as i32 == s27 as i32) as i32
    } != 0
    {
        if s27 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq8(s1, s2, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq6(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s26 as i32) as i32 != 0 {
        (*s1.offset(6 as i32 as isize) as i32 & !(0x20 as i32) == s26 as i32) as i32
    } else {
        (*s1.offset(6 as i32 as isize) as i32 == s26 as i32) as i32
    } != 0
    {
        if s26 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq7(s1, s2, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq5(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s25 as i32) as i32 != 0 {
        (*s1.offset(5 as i32 as isize) as i32 & !(0x20 as i32) == s25 as i32) as i32
    } else {
        (*s1.offset(5 as i32 as isize) as i32 == s25 as i32) as i32
    } != 0
    {
        if s25 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq6(s1, s2, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq4(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s24 as i32) as i32 != 0 {
        (*s1.offset(4 as i32 as isize) as i32 & !(0x20 as i32) == s24 as i32) as i32
    } else {
        (*s1.offset(4 as i32 as isize) as i32 == s24 as i32) as i32
    } != 0
    {
        if s24 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq5(s1, s2, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq3(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s23 as i32) as i32 != 0 {
        (*s1.offset(3 as i32 as isize) as i32 & !(0x20 as i32) == s23 as i32) as i32
    } else {
        (*s1.offset(3 as i32 as isize) as i32 == s23 as i32) as i32
    } != 0
    {
        if s23 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq4(s1, s2, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq2(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s22: i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s22 as i32) as i32 != 0 {
        (*s1.offset(2 as i32 as isize) as i32 & !(0x20 as i32) == s22 as i32) as i32
    } else {
        (*s1.offset(2 as i32 as isize) as i32 == s22 as i32) as i32
    } != 0
    {
        if s22 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq3(s1, s2, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq1(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s21: i8,
    mut s22: i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s21 as i32) as i32 != 0 {
        (*s1.offset(1 as i32 as isize) as i32 & !(0x20 as i32) == s21 as i32) as i32
    } else {
        (*s1.offset(1 as i32 as isize) as i32 == s21 as i32) as i32
    } != 0
    {
        if s21 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq2(s1, s2, s22, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn strcaseeq0(
    mut s1: *const i8,
    mut s2: *const i8,
    mut s20: i8,
    mut s21: i8,
    mut s22: i8,
    mut s23: i8,
    mut s24: i8,
    mut s25: i8,
    mut s26: i8,
    mut s27: i8,
    mut s28: i8,
) -> i32 {
    if if c_isupper(s20 as i32) as i32 != 0 {
        (*s1.offset(0 as i32 as isize) as i32 & !(0x20 as i32) == s20 as i32) as i32
    } else {
        (*s1.offset(0 as i32 as isize) as i32 == s20 as i32) as i32
    } != 0
    {
        if s20 as i32 == 0 as i32 {
            return 1 as i32
        } else {
            return strcaseeq1(s1, s2, s21, s22, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn c_isupper(mut c: i32) -> bool {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[no_mangle]
pub static mut quoting_style_args: [*const i8; 11] = [
    b"literal\0" as *const u8 as *const i8,
    b"shell\0" as *const u8 as *const i8,
    b"shell-always\0" as *const u8 as *const i8,
    b"shell-escape\0" as *const u8 as *const i8,
    b"shell-escape-always\0" as *const u8 as *const i8,
    b"c\0" as *const u8 as *const i8,
    b"c-maybe\0" as *const u8 as *const i8,
    b"escape\0" as *const u8 as *const i8,
    b"locale\0" as *const u8 as *const i8,
    b"clocale\0" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub static mut quoting_style_vals: [quoting_style; 10] = [
    quoting_style::literal_quoting_style,
    quoting_style::shell_quoting_style,
    quoting_style::shell_always_quoting_style,
    quoting_style::shell_escape_quoting_style,
    quoting_style::shell_escape_always_quoting_style,
    quoting_style::c_quoting_style,
    quoting_style::c_maybe_quoting_style,
    quoting_style::escape_quoting_style,
    quoting_style::locale_quoting_style,
    quoting_style::clocale_quoting_style,
];
static mut default_quoting_options: quoting_options = quoting_options {
    style: quoting_style::literal_quoting_style,
    flags: 0,
    quote_these_too: [0; 8],
    left_quote: 0 as *const i8,
    right_quote: 0 as *const i8,
};
#[no_mangle]
pub unsafe extern "C" fn clone_quoting_options(
    mut o: *mut quoting_options,
) -> *mut quoting_options {
    let mut e: i32 = *__errno_location();
    let mut p: *mut quoting_options = xmemdup(
        (if !o.is_null() {
            o
        } else {
            &mut default_quoting_options as *mut quoting_options
        }) as *const libc::c_void,
        ::core::mem::size_of::<quoting_options>() as u64,
    ) as *mut quoting_options;
    *__errno_location() = e;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn get_quoting_style(
    mut o: *const quoting_options,
) -> quoting_style {
    return (*if !o.is_null() { o } else { &mut default_quoting_options }).style;
}
#[no_mangle]
pub unsafe extern "C" fn set_quoting_style(
    mut o: *mut quoting_options,
    mut s: quoting_style,
) {
    (*if !o.is_null() { o } else { &mut default_quoting_options }).style = s;
}
#[no_mangle]
pub unsafe extern "C" fn set_char_quoting(
    mut o: *mut quoting_options,
    mut c: i8,
    mut i: i32,
) -> i32 {
    let mut uc: u8 = c as u8;
    let mut p: *mut u32 = ((*(if !o.is_null() {
        o
    } else {
        &mut default_quoting_options as *mut quoting_options
    }))
        .quote_these_too)
        .as_mut_ptr()
        .offset(
            (uc as u64)
                .wrapping_div(
                    (::core::mem::size_of::<i32>() as u64).wrapping_mul(8 as i32 as u64),
                ) as isize,
        );
    let mut shift: i32 = (uc as u64)
        .wrapping_rem(
            (::core::mem::size_of::<i32>() as u64).wrapping_mul(8 as i32 as u64),
        ) as i32;
    let mut r: i32 = (*p >> shift & 1 as i32 as u32) as i32;
    *p ^= ((i & 1 as i32 ^ r) << shift) as u32;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn set_quoting_flags(
    mut o: *mut quoting_options,
    mut i: i32,
) -> i32 {
    let mut r: i32 = 0;
    if o.is_null() {
        o = &mut default_quoting_options;
    }
    r = (*o).flags;
    (*o).flags = i;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn set_custom_quoting(
    mut o: *mut quoting_options,
    mut left_quote: *const i8,
    mut right_quote: *const i8,
) {
    if o.is_null() {
        o = &mut default_quoting_options;
    }
    (*o).style = quoting_style::custom_quoting_style;
    if left_quote.is_null() || right_quote.is_null() {
        abort();
    }
    (*o).left_quote = left_quote;
    (*o).right_quote = right_quote;
}
unsafe extern "C" fn quoting_options_from_style(
    mut style: quoting_style,
) -> quoting_options {
    let mut o: quoting_options = {
        let mut init = quoting_options {
            style: quoting_style::literal_quoting_style,
            flags: 0 as i32,
            quote_these_too: [0 as i32 as u32, 0, 0, 0, 0, 0, 0, 0],
            left_quote: 0 as *const i8,
            right_quote: 0 as *const i8,
        };
        init
    };
    if style as u32 == quoting_style::custom_quoting_style as i32 as u32 {
        abort();
    }
    o.style = style;
    return o;
}
unsafe extern "C" fn gettext_quote(
    mut msgid: *const i8,
    mut s: quoting_style,
) -> *const i8 {
    let mut translation: *const i8 = dcgettext(0 as *const i8, msgid, 5 as i32);
    let mut locale_code: *const i8 = 0 as *const i8;
    if translation != msgid {
        return translation;
    }
    locale_code = locale_charset();
    if strcaseeq0(
        locale_code,
        b"UTF-8\0" as *const u8 as *const i8,
        'U' as i32 as i8,
        'T' as i32 as i8,
        'F' as i32 as i8,
        '-' as i32 as i8,
        '8' as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
    ) != 0
    {
        return if *msgid.offset(0 as i32 as isize) as i32 == '`' as i32 {
            b"\xE2\x80\x98\0" as *const u8 as *const i8
        } else {
            b"\xE2\x80\x99\0" as *const u8 as *const i8
        };
    }
    if strcaseeq0(
        locale_code,
        b"GB18030\0" as *const u8 as *const i8,
        'G' as i32 as i8,
        'B' as i32 as i8,
        '1' as i32 as i8,
        '8' as i32 as i8,
        '0' as i32 as i8,
        '3' as i32 as i8,
        '0' as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
    ) != 0
    {
        return if *msgid.offset(0 as i32 as isize) as i32 == '`' as i32 {
            b"\xA1\x07e\0" as *const u8 as *const i8
        } else {
            b"\xA1\xAF\0" as *const u8 as *const i8
        };
    }
    return if s as u32 == quoting_style::clocale_quoting_style as i32 as u32 {
        b"\"\0" as *const u8 as *const i8
    } else {
        b"'\0" as *const u8 as *const i8
    };
}
unsafe extern "C" fn quotearg_buffer_restyled(
    mut buffer: *mut i8,
    mut buffersize: size_t,
    mut arg: *const i8,
    mut argsize: size_t,
    mut quoting_style: quoting_style,
    mut flags: i32,
    mut quote_these_too: *const u32,
    mut left_quote: *const i8,
    mut right_quote: *const i8,
) -> size_t {
    let mut current_block: u64;
    let mut i: size_t = 0;
    let mut len: size_t = 0 as i32 as size_t;
    let mut orig_buffersize: size_t = 0 as i32 as size_t;
    let mut quote_string: *const i8 = 0 as *const i8;
    let mut quote_string_len: size_t = 0 as i32 as size_t;
    let mut backslash_escapes: bool = 0 as i32 != 0;
    let mut unibyte_locale: bool = __ctype_get_mb_cur_max() == 1 as i32 as u64;
    let mut elide_outer_quotes: bool = flags
        & quoting_flags::QA_ELIDE_OUTER_QUOTES as i32 != 0 as i32;
    let mut pending_shell_escape_end: bool = 0 as i32 != 0;
    let mut encountered_single_quote: bool = 0 as i32 != 0;
    let mut all_c_and_shell_quote_compat: bool = 1 as i32 != 0;
    '_process_input: loop {
        let mut current_block_47: u64;
        match quoting_style as u32 {
            6 => {
                quoting_style = quoting_style::c_quoting_style;
                elide_outer_quotes = 1 as i32 != 0;
                current_block_47 = 4841276931187788602;
            }
            5 => {
                current_block_47 = 4841276931187788602;
            }
            7 => {
                backslash_escapes = 1 as i32 != 0;
                elide_outer_quotes = 0 as i32 != 0;
                current_block_47 = 14775119014532381840;
            }
            8 | 9 | 10 => {
                if quoting_style as u32
                    != quoting_style::custom_quoting_style as i32 as u32
                {
                    left_quote = gettext_quote(
                        b"`\0" as *const u8 as *const i8,
                        quoting_style,
                    );
                    right_quote = gettext_quote(
                        b"'\0" as *const u8 as *const i8,
                        quoting_style,
                    );
                }
                if !elide_outer_quotes {
                    quote_string = left_quote;
                    while *quote_string != 0 {
                        if len < buffersize {
                            *buffer.offset(len as isize) = *quote_string;
                        }
                        len = len.wrapping_add(1);
                        len;
                        quote_string = quote_string.offset(1);
                        quote_string;
                    }
                }
                backslash_escapes = 1 as i32 != 0;
                quote_string = right_quote;
                quote_string_len = strlen(quote_string);
                current_block_47 = 14775119014532381840;
            }
            3 => {
                backslash_escapes = 1 as i32 != 0;
                current_block_47 = 9214141432038618573;
            }
            1 => {
                current_block_47 = 9214141432038618573;
            }
            4 => {
                current_block_47 = 11414580168150322258;
            }
            2 => {
                current_block_47 = 2029534677406106212;
            }
            0 => {
                elide_outer_quotes = 0 as i32 != 0;
                current_block_47 = 14775119014532381840;
            }
            _ => {
                abort();
            }
        }
        match current_block_47 {
            4841276931187788602 => {
                if !elide_outer_quotes {
                    if len < buffersize {
                        *buffer.offset(len as isize) = '"' as i32 as i8;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                backslash_escapes = 1 as i32 != 0;
                quote_string = b"\"\0" as *const u8 as *const i8;
                quote_string_len = 1 as i32 as size_t;
                current_block_47 = 14775119014532381840;
            }
            9214141432038618573 => {
                elide_outer_quotes = 1 as i32 != 0;
                current_block_47 = 11414580168150322258;
            }
            _ => {}
        }
        match current_block_47 {
            11414580168150322258 => {
                if !elide_outer_quotes {
                    backslash_escapes = 1 as i32 != 0;
                }
                current_block_47 = 2029534677406106212;
            }
            _ => {}
        }
        match current_block_47 {
            2029534677406106212 => {
                quoting_style = quoting_style::shell_always_quoting_style;
                if !elide_outer_quotes {
                    if len < buffersize {
                        *buffer.offset(len as isize) = '\'' as i32 as i8;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                quote_string = b"'\0" as *const u8 as *const i8;
                quote_string_len = 1 as i32 as size_t;
            }
            _ => {}
        }
        i = 0 as i32 as size_t;
        while if argsize == 18446744073709551615 as u64 {
            (*arg.offset(i as isize) as i32 == '\0' as i32) as i32
        } else {
            (i == argsize) as i32
        } == 0
        {
            let mut c: u8 = 0;
            let mut esc: u8 = 0;
            let mut is_right_quote: bool = 0 as i32 != 0;
            let mut escaping: bool = 0 as i32 != 0;
            let mut c_and_shell_quote_compat: bool = 0 as i32 != 0;
            if backslash_escapes as i32 != 0
                && quoting_style as u32
                    != quoting_style::shell_always_quoting_style as i32 as u32
                && quote_string_len != 0
                && i.wrapping_add(quote_string_len)
                    <= (if argsize == 18446744073709551615 as u64
                        && (1 as i32 as u64) < quote_string_len
                    {
                        argsize = strlen(arg);
                        argsize
                    } else {
                        argsize
                    })
                && memcmp(
                    arg.offset(i as isize) as *const libc::c_void,
                    quote_string as *const libc::c_void,
                    quote_string_len,
                ) == 0 as i32
            {
                if elide_outer_quotes {
                    current_block = 3069965560318088815;
                    break '_process_input;
                }
                is_right_quote = 1 as i32 != 0;
            }
            c = *arg.offset(i as isize) as u8;
            match c as i32 {
                0 => {
                    if backslash_escapes {
                        if elide_outer_quotes {
                            current_block = 3069965560318088815;
                            break '_process_input;
                        }
                        escaping = 1 as i32 != 0;
                        if quoting_style as u32
                            == quoting_style::shell_always_quoting_style as i32 as u32
                            && !pending_shell_escape_end
                        {
                            if len < buffersize {
                                *buffer.offset(len as isize) = '\'' as i32 as i8;
                            }
                            len = len.wrapping_add(1);
                            len;
                            if len < buffersize {
                                *buffer.offset(len as isize) = '$' as i32 as i8;
                            }
                            len = len.wrapping_add(1);
                            len;
                            if len < buffersize {
                                *buffer.offset(len as isize) = '\'' as i32 as i8;
                            }
                            len = len.wrapping_add(1);
                            len;
                            pending_shell_escape_end = 1 as i32 != 0;
                        }
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\\' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if quoting_style as u32
                            != quoting_style::shell_always_quoting_style as i32 as u32
                            && i.wrapping_add(1 as i32 as u64) < argsize
                            && '0' as i32
                                <= *arg.offset(i.wrapping_add(1 as i32 as u64) as isize)
                                    as i32
                            && *arg.offset(i.wrapping_add(1 as i32 as u64) as isize)
                                as i32 <= '9' as i32
                        {
                            if len < buffersize {
                                *buffer.offset(len as isize) = '0' as i32 as i8;
                            }
                            len = len.wrapping_add(1);
                            len;
                            if len < buffersize {
                                *buffer.offset(len as isize) = '0' as i32 as i8;
                            }
                            len = len.wrapping_add(1);
                            len;
                        }
                        c = '0' as i32 as u8;
                        current_block = 12544326035781039657;
                    } else if flags & quoting_flags::QA_ELIDE_NULL_BYTES as i32 != 0 {
                        current_block = 16738040538446813684;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
                63 => {
                    match quoting_style as u32 {
                        2 => {
                            current_block = 891842611427793966;
                            match current_block {
                                891842611427793966 => {
                                    if elide_outer_quotes {
                                        current_block = 3069965560318088815;
                                        break '_process_input;
                                    }
                                }
                                _ => {
                                    if flags & quoting_flags::QA_SPLIT_TRIGRAPHS as i32 != 0
                                        && i.wrapping_add(2 as i32 as u64) < argsize
                                        && *arg.offset(i.wrapping_add(1 as i32 as u64) as isize)
                                            as i32 == '?' as i32
                                    {
                                        match *arg.offset(i.wrapping_add(2 as i32 as u64) as isize)
                                            as i32
                                        {
                                            33 | 39 | 40 | 41 | 45 | 47 | 60 | 61 | 62 => {
                                                if elide_outer_quotes {
                                                    current_block = 3069965560318088815;
                                                    break '_process_input;
                                                }
                                                c = *arg.offset(i.wrapping_add(2 as i32 as u64) as isize)
                                                    as u8;
                                                i = (i as u64).wrapping_add(2 as i32 as u64) as size_t
                                                    as size_t;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            current_block = 12544326035781039657;
                        }
                        5 => {
                            current_block = 2559025355513656;
                            match current_block {
                                891842611427793966 => {
                                    if elide_outer_quotes {
                                        current_block = 3069965560318088815;
                                        break '_process_input;
                                    }
                                }
                                _ => {
                                    if flags & quoting_flags::QA_SPLIT_TRIGRAPHS as i32 != 0
                                        && i.wrapping_add(2 as i32 as u64) < argsize
                                        && *arg.offset(i.wrapping_add(1 as i32 as u64) as isize)
                                            as i32 == '?' as i32
                                    {
                                        match *arg.offset(i.wrapping_add(2 as i32 as u64) as isize)
                                            as i32
                                        {
                                            33 | 39 | 40 | 41 | 45 | 47 | 60 | 61 | 62 => {
                                                if elide_outer_quotes {
                                                    current_block = 3069965560318088815;
                                                    break '_process_input;
                                                }
                                                c = *arg.offset(i.wrapping_add(2 as i32 as u64) as isize)
                                                    as u8;
                                                i = (i as u64).wrapping_add(2 as i32 as u64) as size_t
                                                    as size_t;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as i8;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            current_block = 12544326035781039657;
                        }
                        _ => {
                            current_block = 12544326035781039657;
                        }
                    }
                }
                7 => {
                    esc = 'a' as i32 as u8;
                    current_block = 16808685925051915651;
                }
                8 => {
                    esc = 'b' as i32 as u8;
                    current_block = 16808685925051915651;
                }
                12 => {
                    esc = 'f' as i32 as u8;
                    current_block = 16808685925051915651;
                }
                10 => {
                    esc = 'n' as i32 as u8;
                    current_block = 18092799362847696549;
                }
                13 => {
                    esc = 'r' as i32 as u8;
                    current_block = 18092799362847696549;
                }
                9 => {
                    esc = 't' as i32 as u8;
                    current_block = 18092799362847696549;
                }
                11 => {
                    esc = 'v' as i32 as u8;
                    current_block = 16808685925051915651;
                }
                92 => {
                    esc = c;
                    if quoting_style as u32
                        == quoting_style::shell_always_quoting_style as i32 as u32
                    {
                        if elide_outer_quotes {
                            current_block = 3069965560318088815;
                            break '_process_input;
                        }
                        current_block = 3935519630188155739;
                    } else if backslash_escapes as i32 != 0
                        && elide_outer_quotes as i32 != 0 && quote_string_len != 0
                    {
                        current_block = 3935519630188155739;
                    } else {
                        current_block = 18092799362847696549;
                    }
                }
                123 | 125 => {
                    if if argsize == 18446744073709551615 as u64 {
                        (*arg.offset(1 as i32 as isize) as i32 == '\0' as i32) as i32
                    } else {
                        (argsize == 1 as i32 as u64) as i32
                    } == 0
                    {
                        current_block = 12544326035781039657;
                    } else {
                        current_block = 9100190840740794623;
                    }
                }
                35 | 126 => {
                    current_block = 9100190840740794623;
                }
                32 => {
                    current_block = 15150018934656932653;
                }
                33 => {
                    current_block = 5951553903778974260;
                }
                34 | 36 | 38 | 40 | 41 | 42 | 59 | 60 | 61 => {
                    current_block = 7342189930505727164;
                }
                62 | 91 | 94 => {
                    current_block = 7122979066272050138;
                }
                96 | 124 => {
                    current_block = 6178868810057640957;
                }
                39 => {
                    encountered_single_quote = 1 as i32 != 0;
                    c_and_shell_quote_compat = 1 as i32 != 0;
                    if quoting_style as u32
                        == quoting_style::shell_always_quoting_style as i32 as u32
                    {
                        if elide_outer_quotes {
                            current_block = 3069965560318088815;
                            break '_process_input;
                        }
                        if buffersize != 0 && orig_buffersize == 0 {
                            orig_buffersize = buffersize;
                            buffersize = 0 as i32 as size_t;
                        }
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\\' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 0 as i32 != 0;
                        current_block = 12544326035781039657;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
                37 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56
                | 57 | 58 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76
                | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90
                | 93 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107
                | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119
                | 120 | 121 | 122 => {
                    c_and_shell_quote_compat = 1 as i32 != 0;
                    current_block = 12544326035781039657;
                }
                _ => {
                    let mut m: size_t = 0;
                    let mut printable: bool = false;
                    if unibyte_locale {
                        m = 1 as i32 as size_t;
                        printable = *(*__ctype_b_loc()).offset(c as i32 as isize) as i32
                            & C2RustUnnamed_0::_ISprint as i32 as libc::c_ushort as i32
                            != 0 as i32;
                    } else {
                        let mut mbstate: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: C2RustUnnamed { __wch: 0 },
                        };
                        memset(
                            &mut mbstate as *mut mbstate_t as *mut libc::c_void,
                            0 as i32,
                            ::core::mem::size_of::<mbstate_t>() as u64,
                        );
                        m = 0 as i32 as size_t;
                        printable = 1 as i32 != 0;
                        if argsize == 18446744073709551615 as u64 {
                            argsize = strlen(arg);
                        }
                        loop {
                            let mut w: wchar_t = 0;
                            let mut bytes: size_t = rpl_mbrtowc(
                                &mut w,
                                &*arg.offset(i.wrapping_add(m) as isize),
                                argsize.wrapping_sub(i.wrapping_add(m)),
                                &mut mbstate,
                            );
                            if bytes == 0 as i32 as u64 {
                                break;
                            }
                            if bytes == -(1 as i32) as size_t {
                                printable = 0 as i32 != 0;
                                break;
                            } else if bytes == -(2 as i32) as size_t {
                                printable = 0 as i32 != 0;
                                while i.wrapping_add(m) < argsize
                                    && *arg.offset(i.wrapping_add(m) as isize) as i32 != 0
                                {
                                    m = m.wrapping_add(1);
                                    m;
                                }
                                break;
                            } else {
                                if '[' as i32 == 0x5b as i32
                                    && elide_outer_quotes as i32 != 0
                                    && quoting_style as u32
                                        == quoting_style::shell_always_quoting_style as i32 as u32
                                {
                                    let mut j: size_t = 0;
                                    j = 1 as i32 as size_t;
                                    while j < bytes {
                                        match *arg
                                            .offset(i.wrapping_add(m).wrapping_add(j) as isize) as i32
                                        {
                                            91 | 92 | 94 | 96 | 124 => {
                                                current_block = 3069965560318088815;
                                                break '_process_input;
                                            }
                                            _ => {}
                                        }
                                        j = j.wrapping_add(1);
                                        j;
                                    }
                                }
                                if iswprint(w as wint_t) == 0 {
                                    printable = 0 as i32 != 0;
                                }
                                m = (m as u64).wrapping_add(bytes) as size_t as size_t;
                                if !(mbsinit(&mut mbstate) == 0) {
                                    break;
                                }
                            }
                        }
                    }
                    c_and_shell_quote_compat = printable;
                    if (1 as i32 as u64) < m
                        || backslash_escapes as i32 != 0 && !printable
                    {
                        let mut ilim: size_t = i.wrapping_add(m);
                        loop {
                            if backslash_escapes as i32 != 0 && !printable {
                                if elide_outer_quotes {
                                    current_block = 3069965560318088815;
                                    break '_process_input;
                                }
                                escaping = 1 as i32 != 0;
                                if quoting_style as u32
                                    == quoting_style::shell_always_quoting_style as i32 as u32
                                    && !pending_shell_escape_end
                                {
                                    if len < buffersize {
                                        *buffer.offset(len as isize) = '\'' as i32 as i8;
                                    }
                                    len = len.wrapping_add(1);
                                    len;
                                    if len < buffersize {
                                        *buffer.offset(len as isize) = '$' as i32 as i8;
                                    }
                                    len = len.wrapping_add(1);
                                    len;
                                    if len < buffersize {
                                        *buffer.offset(len as isize) = '\'' as i32 as i8;
                                    }
                                    len = len.wrapping_add(1);
                                    len;
                                    pending_shell_escape_end = 1 as i32 != 0;
                                }
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\\' as i32 as i8;
                                }
                                len = len.wrapping_add(1);
                                len;
                                if len < buffersize {
                                    *buffer.offset(len as isize) = ('0' as i32
                                        + (c as i32 >> 6 as i32)) as i8;
                                }
                                len = len.wrapping_add(1);
                                len;
                                if len < buffersize {
                                    *buffer.offset(len as isize) = ('0' as i32
                                        + (c as i32 >> 3 as i32 & 7 as i32)) as i8;
                                }
                                len = len.wrapping_add(1);
                                len;
                                c = ('0' as i32 + (c as i32 & 7 as i32)) as u8;
                            } else if is_right_quote {
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\\' as i32 as i8;
                                }
                                len = len.wrapping_add(1);
                                len;
                                is_right_quote = 0 as i32 != 0;
                            }
                            if ilim <= i.wrapping_add(1 as i32 as u64) {
                                break;
                            }
                            if pending_shell_escape_end as i32 != 0 && !escaping {
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\'' as i32 as i8;
                                }
                                len = len.wrapping_add(1);
                                len;
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\'' as i32 as i8;
                                }
                                len = len.wrapping_add(1);
                                len;
                                pending_shell_escape_end = 0 as i32 != 0;
                            }
                            if len < buffersize {
                                *buffer.offset(len as isize) = c as i8;
                            }
                            len = len.wrapping_add(1);
                            len;
                            i = i.wrapping_add(1);
                            c = *arg.offset(i as isize) as u8;
                        }
                        current_block = 3935519630188155739;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
            }
            match current_block {
                9100190840740794623 => {
                    if i != 0 as i32 as u64 {
                        current_block = 12544326035781039657;
                    } else {
                        current_block = 15150018934656932653;
                    }
                }
                18092799362847696549 => {
                    if quoting_style as u32
                        == quoting_style::shell_always_quoting_style as i32 as u32
                        && elide_outer_quotes as i32 != 0
                    {
                        current_block = 3069965560318088815;
                        break '_process_input;
                    }
                    current_block = 16808685925051915651;
                }
                _ => {}
            }
            match current_block {
                16808685925051915651 => {
                    if backslash_escapes {
                        c = esc;
                        current_block = 7121273192085788486;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
                15150018934656932653 => {
                    c_and_shell_quote_compat = 1 as i32 != 0;
                    current_block = 5951553903778974260;
                }
                _ => {}
            }
            match current_block {
                5951553903778974260 => {
                    current_block = 7342189930505727164;
                }
                _ => {}
            }
            match current_block {
                7342189930505727164 => {
                    current_block = 7122979066272050138;
                }
                _ => {}
            }
            match current_block {
                7122979066272050138 => {
                    current_block = 6178868810057640957;
                }
                _ => {}
            }
            match current_block {
                6178868810057640957 => {
                    if quoting_style as u32
                        == quoting_style::shell_always_quoting_style as i32 as u32
                        && elide_outer_quotes as i32 != 0
                    {
                        current_block = 3069965560318088815;
                        break '_process_input;
                    }
                    current_block = 12544326035781039657;
                }
                _ => {}
            }
            match current_block {
                12544326035781039657 => {
                    if !((backslash_escapes as i32 != 0
                        && quoting_style as u32
                            != quoting_style::shell_always_quoting_style as i32 as u32
                        || elide_outer_quotes as i32 != 0) && !quote_these_too.is_null()
                        && *quote_these_too
                            .offset(
                                (c as u64)
                                    .wrapping_div(
                                        (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64),
                                    ) as isize,
                            )
                            >> (c as u64)
                                .wrapping_rem(
                                    (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64),
                                ) & 1 as i32 as u32 != 0) && !is_right_quote
                    {
                        current_block = 3935519630188155739;
                    } else {
                        current_block = 7121273192085788486;
                    }
                }
                _ => {}
            }
            match current_block {
                7121273192085788486 => {
                    if elide_outer_quotes {
                        current_block = 3069965560318088815;
                        break '_process_input;
                    }
                    escaping = 1 as i32 != 0;
                    if quoting_style as u32
                        == quoting_style::shell_always_quoting_style as i32 as u32
                        && !pending_shell_escape_end
                    {
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '$' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 1 as i32 != 0;
                    }
                    if len < buffersize {
                        *buffer.offset(len as isize) = '\\' as i32 as i8;
                    }
                    len = len.wrapping_add(1);
                    len;
                    current_block = 3935519630188155739;
                }
                _ => {}
            }
            match current_block {
                3935519630188155739 => {
                    if pending_shell_escape_end as i32 != 0 && !escaping {
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as i8;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 0 as i32 != 0;
                    }
                    if len < buffersize {
                        *buffer.offset(len as isize) = c as i8;
                    }
                    len = len.wrapping_add(1);
                    len;
                    if !c_and_shell_quote_compat {
                        all_c_and_shell_quote_compat = 0 as i32 != 0;
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        if len == 0 as i32 as u64
            && quoting_style as u32
                == quoting_style::shell_always_quoting_style as i32 as u32
            && elide_outer_quotes as i32 != 0
        {
            current_block = 3069965560318088815;
            break;
        }
        if !(quoting_style as u32
            == quoting_style::shell_always_quoting_style as i32 as u32
            && !elide_outer_quotes && encountered_single_quote as i32 != 0)
        {
            current_block = 5102396516157810314;
            break;
        }
        if all_c_and_shell_quote_compat {
            return quotearg_buffer_restyled(
                buffer,
                orig_buffersize,
                arg,
                argsize,
                quoting_style::c_quoting_style,
                flags,
                quote_these_too,
                left_quote,
                right_quote,
            )
        } else {
            if !(buffersize == 0 && orig_buffersize != 0) {
                current_block = 5102396516157810314;
                break;
            }
            buffersize = orig_buffersize;
            len = 0 as i32 as size_t;
        }
    }
    match current_block {
        3069965560318088815 => {
            if quoting_style as u32
                == quoting_style::shell_always_quoting_style as i32 as u32
                && backslash_escapes as i32 != 0
            {
                quoting_style = quoting_style::shell_escape_always_quoting_style;
            }
            return quotearg_buffer_restyled(
                buffer,
                buffersize,
                arg,
                argsize,
                quoting_style,
                flags & !(quoting_flags::QA_ELIDE_OUTER_QUOTES as i32),
                0 as *const u32,
                left_quote,
                right_quote,
            );
        }
        _ => {
            if !quote_string.is_null() && !elide_outer_quotes {
                while *quote_string != 0 {
                    if len < buffersize {
                        *buffer.offset(len as isize) = *quote_string;
                    }
                    len = len.wrapping_add(1);
                    len;
                    quote_string = quote_string.offset(1);
                    quote_string;
                }
            }
            if len < buffersize {
                *buffer.offset(len as isize) = '\0' as i32 as i8;
            }
            return len;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_buffer(
    mut buffer: *mut i8,
    mut buffersize: size_t,
    mut arg: *const i8,
    mut argsize: size_t,
    mut o: *const quoting_options,
) -> size_t {
    let mut p: *const quoting_options = if !o.is_null() {
        o
    } else {
        &mut default_quoting_options
    };
    let mut e: i32 = *__errno_location();
    let mut r: size_t = quotearg_buffer_restyled(
        buffer,
        buffersize,
        arg,
        argsize,
        (*p).style,
        (*p).flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    );
    *__errno_location() = e;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_alloc(
    mut arg: *const i8,
    mut argsize: size_t,
    mut o: *const quoting_options,
) -> *mut i8 {
    return quotearg_alloc_mem(arg, argsize, 0 as *mut size_t, o);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_alloc_mem(
    mut arg: *const i8,
    mut argsize: size_t,
    mut size: *mut size_t,
    mut o: *const quoting_options,
) -> *mut i8 {
    let mut p: *const quoting_options = if !o.is_null() {
        o
    } else {
        &mut default_quoting_options
    };
    let mut e: i32 = *__errno_location();
    let mut flags: i32 = (*p).flags
        | (if !size.is_null() {
            0 as i32
        } else {
            quoting_flags::QA_ELIDE_NULL_BYTES as i32
        });
    let mut bufsize: size_t = (quotearg_buffer_restyled(
        0 as *mut i8,
        0 as i32 as size_t,
        arg,
        argsize,
        (*p).style,
        flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    ))
        .wrapping_add(1 as i32 as u64);
    let mut buf: *mut i8 = xcharalloc(bufsize);
    quotearg_buffer_restyled(
        buf,
        bufsize,
        arg,
        argsize,
        (*p).style,
        flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    );
    *__errno_location() = e;
    if !size.is_null() {
        *size = bufsize.wrapping_sub(1 as i32 as u64);
    }
    return buf;
}
static mut slot0: [i8; 256] = [0; 256];
static mut nslots: i32 = 1 as i32;
static mut slotvec0: slotvec = unsafe {
    {
        let mut init = slotvec {
            size: ::core::mem::size_of::<[i8; 256]>() as u64,
            val: slot0.as_ptr() as *mut _,
        };
        init
    }
};
static mut slotvec: *mut slotvec = unsafe {
    &slotvec0 as *const slotvec as *mut slotvec
};
#[no_mangle]
pub unsafe extern "C" fn quotearg_free() {
    let mut sv: *mut slotvec = slotvec;
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < nslots {
        rpl_free((*sv.offset(i as isize)).val as *mut libc::c_void);
        i += 1;
        i;
    }
    if (*sv.offset(0 as i32 as isize)).val != slot0.as_mut_ptr() {
        rpl_free((*sv.offset(0 as i32 as isize)).val as *mut libc::c_void);
        slotvec0.size = ::core::mem::size_of::<[i8; 256]>() as u64;
        slotvec0.val = slot0.as_mut_ptr();
    }
    if sv != &mut slotvec0 as *mut slotvec {
        rpl_free(sv as *mut libc::c_void);
        slotvec = &mut slotvec0;
    }
    nslots = 1 as i32;
}
unsafe extern "C" fn quotearg_n_options(
    mut n: i32,
    mut arg: *const i8,
    mut argsize: size_t,
    mut options: *const quoting_options,
) -> *mut i8 {
    let mut e: i32 = *__errno_location();
    let mut sv: *mut slotvec = slotvec;
    let mut nslots_max: i32 = (if (2147483647 as i32 as i64) < 9223372036854775807 as i64
    {
        2147483647 as i32 as i64
    } else {
        9223372036854775807 as i64
    }) as i32;
    if !(0 as i32 <= n && n < nslots_max) {
        abort();
    }
    if nslots <= n {
        let mut preallocated: bool = sv == &mut slotvec0 as *mut slotvec;
        let mut new_nslots: idx_t = nslots as idx_t;
        sv = xpalloc(
            (if preallocated as i32 != 0 { 0 as *mut slotvec } else { sv })
                as *mut libc::c_void,
            &mut new_nslots,
            (n - nslots + 1 as i32) as idx_t,
            nslots_max as ptrdiff_t,
            ::core::mem::size_of::<slotvec>() as u64 as idx_t,
        ) as *mut slotvec;
        slotvec = sv;
        if preallocated {
            *sv = slotvec0;
        }
        memset(
            sv.offset(nslots as isize) as *mut libc::c_void,
            0 as i32,
            ((new_nslots - nslots as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<slotvec>() as u64),
        );
        nslots = new_nslots as i32;
    }
    let mut size: size_t = (*sv.offset(n as isize)).size;
    let mut val: *mut i8 = (*sv.offset(n as isize)).val;
    let mut flags: i32 = (*options).flags | quoting_flags::QA_ELIDE_NULL_BYTES as i32;
    let mut qsize: size_t = quotearg_buffer_restyled(
        val,
        size,
        arg,
        argsize,
        (*options).style,
        flags,
        ((*options).quote_these_too).as_ptr(),
        (*options).left_quote,
        (*options).right_quote,
    );
    if size <= qsize {
        size = qsize.wrapping_add(1 as i32 as u64);
        (*sv.offset(n as isize)).size = size;
        if val != slot0.as_mut_ptr() {
            rpl_free(val as *mut libc::c_void);
        }
        val = xcharalloc(size);
        let ref mut fresh0 = (*sv.offset(n as isize)).val;
        *fresh0 = val;
        quotearg_buffer_restyled(
            val,
            size,
            arg,
            argsize,
            (*options).style,
            flags,
            ((*options).quote_these_too).as_ptr(),
            (*options).left_quote,
            (*options).right_quote,
        );
    }
    *__errno_location() = e;
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n(mut n: i32, mut arg: *const i8) -> *mut i8 {
    return quotearg_n_options(
        n,
        arg,
        18446744073709551615 as u64,
        &mut default_quoting_options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_mem(
    mut n: i32,
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    return quotearg_n_options(n, arg, argsize, &mut default_quoting_options);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg(mut arg: *const i8) -> *mut i8 {
    return quotearg_n(0 as i32, arg);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_mem(
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    return quotearg_n_mem(0 as i32, arg, argsize);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_style(
    mut n: i32,
    mut s: quoting_style,
    mut arg: *const i8,
) -> *mut i8 {
    let o: quoting_options = quoting_options_from_style(s);
    return quotearg_n_options(n, arg, 18446744073709551615 as u64, &o);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_style_mem(
    mut n: i32,
    mut s: quoting_style,
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    let o: quoting_options = quoting_options_from_style(s);
    return quotearg_n_options(n, arg, argsize, &o);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_style(
    mut s: quoting_style,
    mut arg: *const i8,
) -> *mut i8 {
    return quotearg_n_style(0 as i32, s, arg);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_style_mem(
    mut s: quoting_style,
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    return quotearg_n_style_mem(0 as i32, s, arg, argsize);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_char_mem(
    mut arg: *const i8,
    mut argsize: size_t,
    mut ch: i8,
) -> *mut i8 {
    let mut options: quoting_options = quoting_options {
        style: quoting_style::literal_quoting_style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: 0 as *const i8,
        right_quote: 0 as *const i8,
    };
    options = default_quoting_options;
    set_char_quoting(&mut options, ch, 1 as i32);
    return quotearg_n_options(0 as i32, arg, argsize, &mut options);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_char(mut arg: *const i8, mut ch: i8) -> *mut i8 {
    return quotearg_char_mem(arg, 18446744073709551615 as u64, ch);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_colon(mut arg: *const i8) -> *mut i8 {
    return quotearg_char(arg, ':' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_colon_mem(
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    return quotearg_char_mem(arg, argsize, ':' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_style_colon(
    mut n: i32,
    mut s: quoting_style,
    mut arg: *const i8,
) -> *mut i8 {
    let mut options: quoting_options = quoting_options {
        style: quoting_style::literal_quoting_style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: 0 as *const i8,
        right_quote: 0 as *const i8,
    };
    options = quoting_options_from_style(s);
    set_char_quoting(&mut options, ':' as i32 as i8, 1 as i32);
    return quotearg_n_options(n, arg, 18446744073709551615 as u64, &mut options);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_custom(
    mut n: i32,
    mut left_quote: *const i8,
    mut right_quote: *const i8,
    mut arg: *const i8,
) -> *mut i8 {
    return quotearg_n_custom_mem(
        n,
        left_quote,
        right_quote,
        arg,
        18446744073709551615 as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_custom_mem(
    mut n: i32,
    mut left_quote: *const i8,
    mut right_quote: *const i8,
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    let mut o: quoting_options = default_quoting_options;
    set_custom_quoting(&mut o, left_quote, right_quote);
    return quotearg_n_options(n, arg, argsize, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_custom(
    mut left_quote: *const i8,
    mut right_quote: *const i8,
    mut arg: *const i8,
) -> *mut i8 {
    return quotearg_n_custom(0 as i32, left_quote, right_quote, arg);
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_custom_mem(
    mut left_quote: *const i8,
    mut right_quote: *const i8,
    mut arg: *const i8,
    mut argsize: size_t,
) -> *mut i8 {
    return quotearg_n_custom_mem(0 as i32, left_quote, right_quote, arg, argsize);
}
#[no_mangle]
pub static mut quote_quoting_options: quoting_options = {
    let mut init = quoting_options {
        style: quoting_style::locale_quoting_style,
        flags: 0 as i32,
        quote_these_too: [0 as i32 as u32, 0, 0, 0, 0, 0, 0, 0],
        left_quote: 0 as *const i8,
        right_quote: 0 as *const i8,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn quote_n_mem(
    mut n: i32,
    mut arg: *const i8,
    mut argsize: size_t,
) -> *const i8 {
    return quotearg_n_options(n, arg, argsize, &mut quote_quoting_options);
}
#[no_mangle]
pub unsafe extern "C" fn quote_mem(
    mut arg: *const i8,
    mut argsize: size_t,
) -> *const i8 {
    return quote_n_mem(0 as i32, arg, argsize);
}
#[no_mangle]
pub unsafe extern "C" fn quote_n(mut n: i32, mut arg: *const i8) -> *const i8 {
    return quote_n_mem(n, arg, 18446744073709551615 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn quote(mut arg: *const i8) -> *const i8 {
    return quote_n(0 as i32, arg);
}