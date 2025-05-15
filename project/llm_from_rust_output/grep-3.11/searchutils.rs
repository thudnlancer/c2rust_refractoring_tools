use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_uchar, c_void, c_long, c_ulong};
use std::slice;

const NCHAR: usize = 256;

#[derive(Debug, Clone, Copy)]
struct MbState {
    __count: c_int,
    __value: MbStateValue,
}

#[derive(Debug, Clone, Copy)]
union MbStateValue {
    __wch: c_uint,
    __wchb: [c_char; 4],
}

#[derive(Debug, Clone, Copy)]
struct LocaleInfo {
    multibyte: bool,
    simple: bool,
    using_utf8: bool,
    sbclen: [i8; 256],
    sbctowc: [c_uint; 256],
}

static mut LOCALEINFO: LocaleInfo = LocaleInfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};

static mut MATCH_ICASE: bool = false;
static mut SBWORDCHAR: [bool; 256] = [false; 256];

fn to_uchar(ch: c_char) -> c_uchar {
    ch as c_uchar
}

fn wordchar(wc: c_uint) -> bool {
    wc == '_' as c_uint || unsafe { libc::iswalnum(wc as _) != 0 }
}

fn wordinit() {
    unsafe {
        for i in 0..NCHAR {
            SBWORDCHAR[i] = wordchar(LOCALEINFO.sbctowc[i]);
        }
    }
}

fn kwsinit(mb_trans: bool) -> *mut c_void {
    unsafe {
        let mut trans: *mut c_char = ptr::null_mut();
        if MATCH_ICASE && (libc::__ctype_get_mb_cur_max() == 1 || mb_trans) {
            trans = libc::malloc(NCHAR) as *mut c_char;
            for i in 0..NCHAR {
                let upper = if i < 128 {
                    libc::toupper(i as c_int) as c_char
                } else {
                    i as c_char
                };
                *trans.add(i) = upper;
            }
        }
        libc::kwsalloc(trans)
    }
}

fn mb_clen(s: *const c_char, n: c_long, mbs: *mut MbState) -> c_long {
    unsafe {
        let len = LOCALEINFO.sbclen[to_uchar(*s) as usize];
        if len == -2 {
            imbrlen(s, n, mbs)
        } else {
            len as c_long
        }
    }
}

fn imbrlen(s: *const c_char, n: c_long, mbs: *mut MbState) -> c_long {
    unsafe {
        let len = libc::mbrlen(s, n as usize, mbs as *mut _);
        if len <= 16 {
            len as c_long
        } else {
            -(len as c_long)
        }
    }
}

fn mb_goback(
    mb_start: &mut *const c_char,
    mbclen: &mut c_long,
    cur: *const c_char,
    end: *const c_char,
) -> c_long {
    unsafe {
        let mut p = *mb_start;
        let mut p0 = p;
        if cur <= p {
            return cur as isize - p as isize;
        }

        if LOCALEINFO.using_utf8 {
            p = cur;
            if *cur as c_int & 0xC0 == 0x80 {
                for i in 1..=3 {
                    if *cur.offset(-i) as c_int & 0xC0 != 0x80 {
                        let long_enough = (!(*cur.offset(-i) as c_int) & 0xFF) >> (7 - i) == 0;
                        if long_enough {
                            let mut mbs = MbState {
                                __count: 0,
                                __value: MbStateValue { __wch: 0 },
                            };
                            let clen = imbrlen(
                                cur.offset(-i),
                                end as isize - cur.offset(-i) as isize,
                                &mut mbs,
                            );
                            if clen >= 0 {
                                p0 = cur.offset(-i);
                                p = p0.offset(clen);
                            }
                        }
                        break;
                    }
                }
            }
        } else {
            let mut mbs = MbState {
                __count: 0,
                __value: MbStateValue { __wch: 0 },
            };
            let mut clen = 0;
            loop {
                clen = mb_clen(p, end as isize - p as isize, &mut mbs);
                if clen < 0 {
                    clen = 1;
                    ptr::write_bytes(&mut mbs as *mut _ as *mut c_void, 0, 1);
                }
                p0 = p;
                p = p.offset(clen);
                if p >= cur {
                    break;
                }
            }
            if !mbclen.is_null() {
                *mbclen = clen;
            }
        }
        *mb_start = p;
        if p == cur {
            0
        } else {
            cur as isize - p0 as isize
        }
    }
}

fn wordchars_count(buf: *const c_char, end: *const c_char, countall: bool) -> c_long {
    unsafe {
        let mut n = 0;
        let mut mbs = MbState {
            __count: 0,
            __value: MbStateValue { __wch: 0 },
        };
        while n < end as isize - buf as isize {
            let b = *buf.offset(n) as c_uchar;
            if SBWORDCHAR[b as usize] {
                n += 1;
            } else {
                if LOCALEINFO.sbclen[b as usize] != -2 {
                    break;
                }
                let mut wc = 0;
                let wcbytes = libc::mbrtowc(
                    &mut wc,
                    buf.offset(n),
                    (end as isize - buf as isize - n) as usize,
                    &mut mbs as *mut _ as *mut _,
                );
                if !wordchar(wc as c_uint) {
                    break;
                }
                n += (wcbytes + (wcbytes == 0) as usize) as isize;
            }
            if !countall {
                break;
            }
        }
        n
    }
}

fn wordchars_size(buf: *const c_char, end: *const c_char) -> c_long {
    wordchars_count(buf, end, true)
}

fn wordchar_next(buf: *const c_char, end: *const c_char) -> c_long {
    wordchars_count(buf, end, false)
}

fn wordchar_prev(buf: *const c_char, cur: *const c_char, end: *const c_char) -> c_long {
    if buf == cur {
        return 0;
    }
    unsafe {
        let mut cur = cur.offset(-1);
        let b = *cur as c_uchar;
        if !LOCALEINFO.multibyte || (LOCALEINFO.using_utf8 && (b >> 7) == 0) {
            return SBWORDCHAR[b as usize] as c_long;
        }
        let mut p = buf;
        cur = cur.offset(-mb_goback(&mut p, ptr::null_mut(), cur, end));
        wordchar_next(cur, end)
    }
}