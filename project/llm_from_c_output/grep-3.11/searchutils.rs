use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::slice;
use std::str;
use std::char;
use std::cmp;
use std::iter;

const NCHAR: usize = 256;

struct LocaleInfo {
    sbctowc: [wchar_t; NCHAR],
    sbclen: [i8; NCHAR],
    multibyte: bool,
    using_utf8: bool,
}

static mut LOCALEINFO: LocaleInfo = LocaleInfo {
    sbctowc: [0; NCHAR],
    sbclen: [0; NCHAR],
    multibyte: false,
    using_utf8: false,
};

type wchar_t = u32;
type idx_t = isize;
type kwset_t = *mut ();

static mut sbwordchar: [bool; NCHAR] = [false; NCHAR];

fn wordchar(wc: wchar_t) -> bool {
    wc == '_' as wchar_t || wc.is_alphanumeric()
}

fn wordinit() {
    unsafe {
        for i in 0..NCHAR {
            sbwordchar[i] = wordchar(LOCALEINFO.sbctowc[i]);
        }
    }
}

fn kwsinit(mb_trans: bool) -> kwset_t {
    let trans = if match_icase && (MB_CUR_MAX == 1 || mb_trans) {
        let mut trans = Vec::with_capacity(NCHAR);
        for i in 0..NCHAR {
            trans.push(i as u8);
        }
        Some(trans)
    } else {
        None
    };

    unsafe { kwsalloc(trans.as_ref().map(|v| v.as_ptr())) }
}

fn mb_goback(
    mb_start: &mut *const c_char,
    mbclen: Option<&mut idx_t>,
    cur: *const c_char,
    end: *const c_char,
) -> idx_t {
    let mut p = *mb_start;
    let mut p0 = p;

    if cur <= p {
        return cur as idx_t - p as idx_t;
    }

    unsafe {
        if LOCALEINFO.using_utf8 {
            p = cur;

            if (*cur & 0xc0) == 0x80 {
                for i in 1..=3 {
                    if (*cur.offset(-i) & 0xc0) != 0x80 {
                        let long_enough = (!*cur.offset(-i) >> (7 - i) == 0;

                        if long_enough {
                            let mut mbs = mem::zeroed();
                            let clen = imbrlen(
                                cur.offset(-i),
                                end.offset_from(cur.offset(-i)) as usize,
                                &mut mbs,
                            );
                            if clen >= 0 {
                                p0 = cur.offset(-i);
                                p = p0.offset(clen as isize);
                            }
                        }
                        break;
                    }
                }
            }
        } else {
            let mut mbs = mem::zeroed();
            let mut clen;

            loop {
                clen = mb_clen(p, end.offset_from(p) as usize, &mut mbs);

                if clen < 0 {
                    clen = 1;
                    mbs = mem::zeroed();
                }
                p0 = p;
                p = p.offset(clen as isize);
                if p >= cur {
                    break;
                }
            }

            if let Some(mbclen) = mbclen {
                *mbclen = clen as idx_t;
            }
        }

        *mb_start = p;
        if p == cur {
            0
        } else {
            cur as idx_t - p0 as idx_t
        }
    }
}

fn wordchars_count(buf: *const c_char, end: *const c_char, countall: bool) -> idx_t {
    let mut n = 0;
    let mut mbs = mem::zeroed();

    unsafe {
        while n < end.offset_from(buf) {
            let b = *buf.offset(n) as u8;
            if sbwordchar[b as usize] {
                n += 1;
            } else if LOCALEINFO.sbclen[b as usize] != -2 {
                break;
            } else {
                let mut wc = 0;
                let wcbytes = mbrtowc(
                    &mut wc,
                    buf.offset(n),
                    end.offset_from(buf.offset(n)) as usize,
                    &mut mbs,
                );
                if !wordchar(wc) {
                    break;
                }
                n += if wcbytes == 0 { 1 } else { wcbytes as isize };
            }
            if !countall {
                break;
            }
        }
    }
    n
}

fn wordchars_size(buf: *const c_char, end: *const c_char) -> idx_t {
    wordchars_count(buf, end, true)
}

fn wordchar_next(buf: *const c_char, end: *const c_char) -> idx_t {
    wordchars_count(buf, end, false)
}

fn wordchar_prev(buf: *const c_char, cur: *const c_char, end: *const c_char) -> idx_t {
    if buf == cur {
        return 0;
    }
    unsafe {
        let mut cur = cur.offset(-1);
        let b = *cur;
        if !LOCALEINFO.multibyte || LOCALEINFO.using_utf8 && (b >> 7) == 0 {
            return sbwordchar[b as usize] as idx_t;
        }
        let mut p = buf;
        cur = cur.offset(-mb_goback(&mut p, None, cur, end));
        wordchar_next(cur, end)
    }
}

// Placeholder for external C functions
extern "C" {
    fn kwsalloc(trans: *const u8) -> kwset_t;
    fn imbrlen(s: *const c_char, n: usize, ps: *mut mbstate_t) -> isize;
    fn mb_clen(s: *const c_char, n: usize, ps: *mut mbstate_t) -> isize;
    fn mbrtowc(pwc: *mut wchar_t, s: *const c_char, n: usize, ps: *mut mbstate_t) -> usize;
}

static MB_CUR_MAX: usize = 1;
static match_icase: bool = false;

#[repr(C)]
struct mbstate_t {
    _data: [u8; 128],
}