use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use std::mem::MaybeUninit;

pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[derive(Copy, Clone)]
pub struct mbstate_t {
    __count: c_int,
    __value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    __wch: c_uint,
    __wchb: [c_char; 4],
}

#[derive(Copy, Clone)]
pub struct mbchar {
    ptr: *const c_char,
    bytes: size_t,
    wc_valid: bool,
    wc: wchar_t,
    buf: [c_char; 24],
}

#[derive(Copy, Clone)]
pub struct mbuiter_multi {
    in_shift: bool,
    state: mbstate_t,
    next_done: bool,
    cur: mbchar,
}

pub type mbui_iterator_t = mbuiter_multi;

static IS_BASIC_TABLE: [c_uint; 0] = [];

fn is_basic(c: c_char) -> bool {
    unsafe {
        *IS_BASIC_TABLE.as_ptr().offset((c as u8 as c_int >> 5) as isize) 
            >> (c as u8 as c_int & 31) 
            & 1 as c_uint != 0
    }
}

fn mbsinit(ps: &mbstate_t) -> bool {
    unsafe { libc::mbsinit(ps as *const _ as *const _) != 0 }
}

fn mbuiter_multi_next(iter: &mut mbuiter_multi) {
    if iter.next_done {
        return;
    }

    if iter.in_shift {
        // Continue processing multi-byte character
    } else if is_basic(unsafe { *iter.cur.ptr }) {
        iter.cur.bytes = 1;
        iter.cur.wc = unsafe { *iter.cur.ptr } as wchar_t;
        iter.cur.wc_valid = true;
        iter.next_done = true;
        return;
    } else {
        assert!(mbsinit(&iter.state), "mbstate not initialized");
    }

    iter.in_shift = true;
    let max_len = unsafe { libc::__ctype_get_mb_cur_max() };
    let len = unsafe { 
        libc::strnlen(iter.cur.ptr, max_len) 
    };
    
    let bytes = unsafe {
        libc::mbrtowc(
            &mut iter.cur.wc,
            iter.cur.ptr,
            len,
            &mut iter.state as *mut _ as *mut _
        )
    };

    iter.cur.bytes = if bytes == size_t::MAX {
        1
    } else if bytes == size_t::MAX - 1 {
        unsafe { libc::strlen(iter.cur.ptr) }
    } else {
        bytes
    };

    if bytes == 0 {
        iter.cur.bytes = 1;
        assert_eq!(unsafe { *iter.cur.ptr }, 0, "Null terminator expected");
        assert_eq!(iter.cur.wc, 0, "Wide char should be null");
    }

    iter.cur.wc_valid = bytes != size_t::MAX && bytes != size_t::MAX - 1;
    if mbsinit(&iter.state) {
        iter.in_shift = false;
    }
    iter.next_done = true;
}

pub fn mbslen(string: *const c_char) -> size_t {
    unsafe {
        if libc::__ctype_get_mb_cur_max() > 1 {
            let mut count = 0;
            let mut iter = mbuiter_multi {
                in_shift: false,
                state: mbstate_t {
                    __count: 0,
                    __value: C2RustUnnamed { __wch: 0 },
                },
                next_done: false,
                cur: mbchar {
                    ptr: string,
                    bytes: 0,
                    wc_valid: false,
                    wc: 0,
                    buf: [0; 24],
                },
            };

            libc::memset(
                &mut iter.state as *mut _ as *mut _,
                0,
                std::mem::size_of::<mbstate_t>(),
            );

            loop {
                mbuiter_multi_next(&mut iter);
                if !(iter.cur.wc_valid && iter.cur.wc == 0) {
                    break;
                }
                count += 1;
                iter.cur.ptr = iter.cur.ptr.add(iter.cur.bytes as usize);
                iter.next_done = false;
            }
            count
        } else {
            libc::strlen(string)
        }
    }
}