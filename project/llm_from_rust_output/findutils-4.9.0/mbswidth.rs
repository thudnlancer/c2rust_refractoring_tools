use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};
use std::mem;

type WcharT = c_int;
type WintT = c_uint;
type SizeT = usize;

#[derive(Debug, Clone, Copy)]
struct MbState {
    count: c_int,
    value: MbStateValue,
}

#[derive(Debug, Clone, Copy)]
union MbStateValue {
    wch: c_uint,
    wchb: [c_char; 4],
}

const IS_CNTRL: c_uint = 2;
const IS_PRINT: c_uint = 16384;

fn gnu_mbswidth(string: &CStr, flags: c_int) -> c_int {
    let bytes = string.to_bytes();
    mbsnwidth(bytes, flags)
}

fn mbsnwidth(string: &[u8], flags: c_int) -> c_int {
    let mut width = 0;
    let mut pos = 0;

    if unsafe { libc::__ctype_get_mb_cur_max() } > 1 {
        while pos < string.len() {
            let c = string[pos] as c_char;
            match c {
                // Printable ASCII characters
                32..=126 => {
                    pos += 1;
                    width += 1;
                }
                _ => {
                    let mut mbstate = MbState {
                        count: 0,
                        value: MbStateValue { wch: 0 },
                    };
                    unsafe {
                        libc::memset(
                            &mut mbstate as *mut _ as *mut libc::c_void,
                            0,
                            mem::size_of::<MbState>(),
                        );
                    }

                    loop {
                        let mut wc: WcharT = 0;
                        let remaining = &string[pos..];
                        let bytes = unsafe {
                            libc::rpl_mbrtowc(
                                &mut wc,
                                remaining.as_ptr() as *const c_char,
                                remaining.len(),
                                &mut mbstate as *mut _ as *mut libc::mbstate_t,
                            )
                        };

                        if bytes == (-1isize) as SizeT {
                            if flags & 1 == 0 {
                                pos += 1;
                                width += 1;
                                break;
                            } else {
                                return -1;
                            }
                        } else if bytes == (-2isize) as SizeT {
                            if flags & 1 == 0 {
                                pos = string.len();
                                width += 1;
                                break;
                            } else {
                                return -1;
                            }
                        } else {
                            let bytes = if bytes == 0 { 1 } else { bytes };
                            let w = unsafe { libc::wcwidth(wc) };

                            if w >= 0 {
                                width = match width.checked_add(w) {
                                    Some(w) => w,
                                    None => return c_int::MAX,
                                };
                            } else if flags & 2 == 0 {
                                if unsafe { libc::iswcntrl(wc as WintT) } == 0 {
                                    width = match width.checked_add(1) {
                                        Some(w) => w,
                                        None => return c_int::MAX,
                                    };
                                }
                            } else {
                                return -1;
                            }

                            pos += bytes;
                            if unsafe { libc::mbsinit(&mbstate as *const _ as *const libc::mbstate_t) } != 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else {
        while pos < string.len() {
            let c = string[pos] as c_char;
            let c_uchar = c as u8;
            let ctype_b = unsafe { *libc::__ctype_b_loc() };

            if unsafe { *ctype_b.offset(c_uchar as isize) } as c_int & IS_PRINT as c_int != 0 {
                width = match width.checked_add(1) {
                    Some(w) => w,
                    None => return c_int::MAX,
                };
            } else if flags & 2 == 0 {
                if unsafe { *ctype_b.offset(c_uchar as isize) } as c_int & IS_CNTRL as c_int == 0 {
                    width = match width.checked_add(1) {
                        Some(w) => w,
                        None => return c_int::MAX,
                    };
                }
            } else {
                return -1;
            }
            pos += 1;
        }
    }

    width
}