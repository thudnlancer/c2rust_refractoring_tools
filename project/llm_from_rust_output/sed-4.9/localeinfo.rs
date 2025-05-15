use std::char;
use std::cmp::Ordering;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_schar, c_short};
use std::str;

type size_t = c_ulong;
type wchar_t = c_int;
type wint_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    __count: c_int,
    __value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    __wch: c_uint,
    __wchb: [c_char; 4],
}

type mbstate_t = __mbstate_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    multibyte: bool,
    simple: bool,
    using_utf8: bool,
    sbclen: [c_schar; 256],
    sbctowc: [wint_t; 256],
}

const native_c_charset: u32 = 1;

fn is_using_utf8() -> bool {
    let mut wc: wchar_t = 0;
    let mut mbs = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };

    let s = b"\xC4\x80";
    let result = unsafe {
        rpl_mbrtowc(
            &mut wc,
            s.as_ptr() as *const c_char,
            2,
            &mut mbs,
        )
    };

    result == 2 && wc == 0x100
}

fn using_simple_locale(multibyte: bool) -> bool {
    if native_c_charset == 0 || multibyte {
        return false;
    }

    for i in 0..(127 * 2 + 1) {
        let s1 = [i as c_char, 0];
        let s2 = [(i + 1) as c_char, 0];
        let result = unsafe { strcoll(s1.as_ptr(), s2.as_ptr()) };
        if result >= 0 {
            return false;
        }
    }
    true
}

#[no_mangle]
pub fn init_localeinfo(localeinfo: &mut localeinfo) {
    localeinfo.multibyte = unsafe { __ctype_get_mb_cur_max() } > 1;
    localeinfo.simple = using_simple_locale(localeinfo.multibyte);
    localeinfo.using_utf8 = is_using_utf8();

    for i in (-128)..=127 {
        let c = i as c_char;
        let uc = i as u8;
        let mut s = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        let mut wc: wchar_t = 0;
        let len = unsafe { rpl_mbrtowc(&mut wc, &c, 1, &mut s) };

        localeinfo.sbclen[uc as usize] = if len <= 1 {
            1
        } else {
            -(len as c_int)
        } as c_schar;

        localeinfo.sbctowc[uc as usize] = if len <= 1 {
            wc as wint_t
        } else {
            0xffffffff
        };
    }
}

static LONESOME_LOWER: [c_short; 19] = [
    0xb5,
    0x131,
    0x17f,
    0x1c5,
    0x1c8,
    0x1cb,
    0x1f2,
    0x345,
    0x3c2,
    0x3d0,
    0x3d1,
    0x3d5,
    0x3d6,
    0x3f0,
    0x3f1,
    0x3f2,
    0x3f5,
    0x1e9b,
    0x1fbe,
];

#[no_mangle]
pub fn case_folded_counterparts(c: wint_t, folded: &mut [wchar_t]) -> c_int {
    let mut n = 0;
    let uc = unsafe { towupper(c) };
    let lc = unsafe { towlower(uc) };

    if uc != c && n < folded.len() {
        folded[n] = uc as wchar_t;
        n += 1;
    }

    if lc != uc && lc != c && unsafe { towupper(lc) } == uc && n < folded.len() {
        folded[n] = lc as wchar_t;
        n += 1;
    }

    for &li in &LONESOME_LOWER {
        let li = li as wint_t;
        if li != lc && li != uc && li != c && unsafe { towupper(li) } == uc && n < folded.len() {
            folded[n] = li as wchar_t;
            n += 1;
        }
    }

    n as c_int
}

extern "C" {
    fn rpl_mbrtowc(pwc: *mut wchar_t, s: *const c_char, n: size_t, ps: *mut mbstate_t) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strcoll(__s1: *const c_char, __s2: *const c_char) -> c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
}