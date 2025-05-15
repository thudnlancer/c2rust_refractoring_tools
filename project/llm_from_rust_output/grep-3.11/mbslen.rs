use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use std::mem::MaybeUninit;

pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: c_int,
    pub __value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [c_char; 24],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}

pub type mbui_iterator_t = mbuiter_multi;

static IS_BASIC_TABLE: [c_uint; 0] = [];

fn is_basic(c: c_char) -> bool {
    unsafe {
        *IS_BASIC_TABLE
            .as_ptr()
            .offset((c as u8 as c_int >> 5) as isize)
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
        // Continue processing
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

    let max_bytes = unsafe { libc::__ctype_get_mb_cur_max() };
    let len = unsafe { libc::strnlen(iter.cur.ptr, max_bytes) };
    
    let mut wc = MaybeUninit::uninit();
    let bytes = unsafe {
        libc::mbrtowc(
            wc.as_mut_ptr(),
            iter.cur.ptr,
            len,
            &mut iter.state as *mut _ as *mut _
        )
    };

    if bytes == (-1isize) as size_t {
        iter.cur.bytes = 1;
        iter.cur.wc_valid = false;
    } else if bytes == (-2isize) as size_t {
        iter.cur.bytes = unsafe { libc::strlen(iter.cur.ptr) };
        iter.cur.wc_valid = false;
    } else {
        if bytes == 0 {
            iter.cur.bytes = 1;
            assert_eq!(unsafe { *iter.cur.ptr }, 0, "Null terminator expected");
            assert_eq!(unsafe { wc.assume_init() }, 0, "Wide null expected");
        }
        iter.cur.wc = unsafe { wc.assume_init() };
        iter.cur.wc_valid = true;
        if mbsinit(&iter.state) {
            iter.in_shift = false;
        }
    }
    
    iter.cur.bytes = bytes;
    iter.next_done = true;
}

pub fn mbslen(string: &CStr) -> size_t {
    if unsafe { libc::__ctype_get_mb_cur_max() } > 1 {
        let mut count = 0;
        let mut iter = mbuiter_multi {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: string.as_ptr(),
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };

        unsafe {
            libc::memset(
                &mut iter.state as *mut _ as *mut _,
                0,
                std::mem::size_of::<mbstate_t>(),
            );
        }

        loop {
            mbuiter_multi_next(&mut iter);
            if !(iter.cur.wc_valid && iter.cur.wc == 0) {
                break;
            }
            count += 1;
            iter.cur.ptr = unsafe { iter.cur.ptr.offset(iter.cur.bytes as isize) };
            iter.next_done = false;
        }

        count
    } else {
        unsafe { libc::strlen(string.as_ptr()) }
    }
}