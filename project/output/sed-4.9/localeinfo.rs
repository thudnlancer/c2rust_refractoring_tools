use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strcoll(__s1: *const i8, __s2: *const i8) -> i32;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type wint_t = u32;
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
pub enum C2RustUnnamed_0 {
    native_c_charset = 1,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::native_c_charset => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            1 => C2RustUnnamed_0::native_c_charset,
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
unsafe extern "C" fn is_using_utf8() -> bool {
    let mut wc: wchar_t = 0;
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    return rpl_mbrtowc(
        &mut wc,
        b"\xC4\x80\0" as *const u8 as *const i8,
        2 as i32 as size_t,
        &mut mbs,
    ) == 2 as i32 as u64 && wc == 0x100 as i32;
}
unsafe extern "C" fn using_simple_locale(mut multibyte: bool) -> bool {
    if C2RustUnnamed_0::native_c_charset as i32 == 0 || multibyte as i32 != 0 {
        return 0 as i32 != 0;
    }
    let mut i: i32 = 0 as i32;
    while i < 127 as i32 * 2 as i32 + 1 as i32 {
        if 0 as i32
            <= strcoll(
                [i as i8, 0 as i32 as i8].as_mut_ptr(),
                [(i + 1 as i32) as i8, 0 as i32 as i8].as_mut_ptr(),
            )
        {
            return 0 as i32 != 0;
        }
        i += 1;
        i;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn init_localeinfo(mut localeinfo: *mut localeinfo) {
    (*localeinfo).multibyte = __ctype_get_mb_cur_max() > 1 as i32 as u64;
    (*localeinfo).simple = using_simple_locale((*localeinfo).multibyte);
    (*localeinfo).using_utf8 = is_using_utf8();
    let mut i: i32 = -(127 as i32) - 1 as i32;
    while i <= 127 as i32 {
        let mut c: i8 = i as i8;
        let mut uc: u8 = i as u8;
        let mut s: mbstate_t = {
            let mut init = __mbstate_t {
                __count: 0 as i32,
                __value: C2RustUnnamed { __wch: 0 },
            };
            init
        };
        let mut wc: wchar_t = 0;
        let mut len: size_t = rpl_mbrtowc(&mut wc, &mut c, 1 as i32 as size_t, &mut s);
        (*localeinfo).sbclen[uc as usize] = (if len <= 1 as i32 as u64 {
            1 as i32
        } else {
            -(len.wrapping_neg() as i32)
        }) as libc::c_schar;
        (*localeinfo).sbctowc[uc as usize] = if len <= 1 as i32 as u64 {
            wc as u32
        } else {
            0xffffffff as u32
        };
        i += 1;
        i;
    }
}
static mut lonesome_lower: [libc::c_short; 19] = [
    0xb5 as i32 as libc::c_short,
    0x131 as i32 as libc::c_short,
    0x17f as i32 as libc::c_short,
    0x1c5 as i32 as libc::c_short,
    0x1c8 as i32 as libc::c_short,
    0x1cb as i32 as libc::c_short,
    0x1f2 as i32 as libc::c_short,
    0x345 as i32 as libc::c_short,
    0x3c2 as i32 as libc::c_short,
    0x3d0 as i32 as libc::c_short,
    0x3d1 as i32 as libc::c_short,
    0x3d5 as i32 as libc::c_short,
    0x3d6 as i32 as libc::c_short,
    0x3f0 as i32 as libc::c_short,
    0x3f1 as i32 as libc::c_short,
    0x3f2 as i32 as libc::c_short,
    0x3f5 as i32 as libc::c_short,
    0x1e9b as i32 as libc::c_short,
    0x1fbe as i32 as libc::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn case_folded_counterparts(
    mut c: wint_t,
    mut folded: *mut wchar_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0 as i32;
    let mut uc: wint_t = towupper(c);
    let mut lc: wint_t = towlower(uc);
    if uc != c {
        let fresh0 = n;
        n = n + 1;
        *folded.offset(fresh0 as isize) = uc as wchar_t;
    }
    if lc != uc && lc != c && towupper(lc) == uc {
        let fresh1 = n;
        n = n + 1;
        *folded.offset(fresh1 as isize) = lc as wchar_t;
    }
    i = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[libc::c_short; 19]>() as u64)
            .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
    {
        let mut li: wint_t = lonesome_lower[i as usize] as wint_t;
        if li != lc && li != uc && li != c && towupper(li) == uc {
            let fresh2 = n;
            n = n + 1;
            *folded.offset(fresh2 as isize) = li as wchar_t;
        }
        i += 1;
        i;
    }
    return n;
}