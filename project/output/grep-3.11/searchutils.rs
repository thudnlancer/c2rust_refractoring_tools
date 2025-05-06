#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, linkage)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type kwset;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn rpl_mbrlen(s: *const i8, n: size_t, ps: *mut mbstate_t) -> size_t;
    fn iswalnum(__wc: wint_t) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut match_icase: bool;
    fn kwsalloc(_: *const i8) -> kwset_t;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    static mut localeinfo: localeinfo;
}
pub type __int32_t = i32;
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
pub type ptrdiff_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    NCHAR = 256,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::NCHAR => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            256 => C2RustUnnamed_0::NCHAR,
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
pub type kwset_t = *mut kwset;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn to_uchar(mut ch: i8) -> u8 {
    return ch as u8;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_clen(
    mut s: *const i8,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: libc::c_schar = localeinfo.sbclen[to_uchar(*s) as usize];
    return if len as i32 == -(2 as i32) { imbrlen(s, n, mbs) } else { len as i64 };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn imbrlen(
    mut s: *const i8,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: size_t = rpl_mbrlen(s, n as size_t, mbs);
    if len <= 16 as i32 as u64 {
        return len as ptrdiff_t;
    }
    let mut neglen: ptrdiff_t = len.wrapping_neg() as ptrdiff_t;
    return -neglen;
}
static mut sbwordchar: [bool; 256] = [false; 256];
unsafe extern "C" fn wordchar(mut wc: wint_t) -> bool {
    return wc == '_' as i32 as u32 || iswalnum(wc) != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wordinit() {
    let mut i: i32 = 0 as i32;
    while i < C2RustUnnamed_0::NCHAR as i32 {
        sbwordchar[i as usize] = wordchar(localeinfo.sbctowc[i as usize]);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn kwsinit(mut mb_trans: bool) -> kwset_t {
    let mut trans: *mut i8 = 0 as *mut i8;
    if match_icase as i32 != 0
        && (__ctype_get_mb_cur_max() == 1 as i32 as u64 || mb_trans as i32 != 0)
    {
        trans = ximalloc(C2RustUnnamed_0::NCHAR as i32 as idx_t) as *mut i8;
        let mut i: i32 = 0 as i32;
        while i < C2RustUnnamed_0::NCHAR as i32 {
            *trans.offset(i as isize) = ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = i;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(i);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(i as isize);
                }
                __res
            }) as i8;
            i += 1;
            i;
        }
    }
    return kwsalloc(trans);
}
#[no_mangle]
pub unsafe extern "C" fn mb_goback(
    mut mb_start: *mut *const i8,
    mut mbclen: *mut idx_t,
    mut cur: *const i8,
    mut end: *const i8,
) -> ptrdiff_t {
    let mut p: *const i8 = *mb_start;
    let mut p0: *const i8 = p;
    if cur <= p {
        return cur.offset_from(p) as i64;
    }
    if localeinfo.using_utf8 {
        p = cur;
        if *cur as i32 & 0xc0 as i32 == 0x80 as i32 {
            let mut i: i32 = 1 as i32;
            while i <= 3 as i32 {
                if *cur.offset(-i as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                    let mut long_enough: bool = (!(*cur.offset(-i as isize) as i32)
                        & 0xff as i32) >> 7 as i32 - i == 0 as i32;
                    if long_enough {
                        let mut mbs: mbstate_t = {
                            let mut init = __mbstate_t {
                                __count: 0 as i32,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            init
                        };
                        let mut clen: ptrdiff_t = imbrlen(
                            cur.offset(-(i as isize)),
                            end.offset_from(cur.offset(-(i as isize))) as i64,
                            &mut mbs,
                        );
                        if 0 as i32 as i64 <= clen {
                            p0 = cur.offset(-(i as isize));
                            p = p0.offset(clen as isize);
                        }
                    }
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
    } else {
        let mut mbs_0: mbstate_t = {
            let mut init = __mbstate_t {
                __count: 0 as i32,
                __value: C2RustUnnamed { __wch: 0 },
            };
            init
        };
        let mut clen_0: ptrdiff_t = 0;
        loop {
            clen_0 = mb_clen(p, end.offset_from(p) as i64, &mut mbs_0);
            if clen_0 < 0 as i32 as i64 {
                clen_0 = 1 as i32 as ptrdiff_t;
                memset(
                    &mut mbs_0 as *mut mbstate_t as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<mbstate_t>() as u64,
                );
            }
            p0 = p;
            p = p.offset(clen_0 as isize);
            if !(p < cur) {
                break;
            }
        }
        if !mbclen.is_null() {
            *mbclen = clen_0;
        }
    }
    *mb_start = p;
    return if p == cur { 0 as i32 as i64 } else { cur.offset_from(p0) as i64 };
}
unsafe extern "C" fn wordchars_count(
    mut buf: *const i8,
    mut end: *const i8,
    mut countall: bool,
) -> idx_t {
    let mut n: idx_t = 0 as i32 as idx_t;
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    while n < end.offset_from(buf) as i64 {
        let mut b: u8 = *buf.offset(n as isize) as u8;
        if sbwordchar[b as usize] {
            n += 1;
            n;
        } else {
            if localeinfo.sbclen[b as usize] as i32 != -(2 as i32) {
                break;
            }
            let mut wc: wchar_t = 0 as i32;
            let mut wcbytes: size_t = rpl_mbrtowc(
                &mut wc,
                buf.offset(n as isize),
                (end.offset_from(buf) as i64 - n) as size_t,
                &mut mbs,
            );
            if !wordchar(wc as wint_t) {
                break;
            }
            n = (n as u64)
                .wrapping_add(wcbytes.wrapping_add((wcbytes == 0) as i32 as u64))
                as idx_t as idx_t;
        }
        if !countall {
            break;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn wordchars_size(
    mut buf: *const i8,
    mut end: *const i8,
) -> idx_t {
    return wordchars_count(buf, end, 1 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn wordchar_next(mut buf: *const i8, mut end: *const i8) -> idx_t {
    return wordchars_count(buf, end, 0 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn wordchar_prev(
    mut buf: *const i8,
    mut cur: *const i8,
    mut end: *const i8,
) -> idx_t {
    if buf == cur {
        return 0 as i32 as idx_t;
    }
    cur = cur.offset(-1);
    let mut b: u8 = *cur as u8;
    if !localeinfo.multibyte
        || localeinfo.using_utf8 as i32 & !(b as i32 >> 7 as i32) != 0
    {
        return sbwordchar[b as usize] as idx_t;
    }
    let mut p: *const i8 = buf;
    cur = cur.offset(-(mb_goback(&mut p, 0 as *mut idx_t, cur, end) as isize));
    return wordchar_next(cur, end);
}