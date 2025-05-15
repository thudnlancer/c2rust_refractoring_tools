use std::mem::{self, MaybeUninit};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}

pub type mbstate_t = __mbstate_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: usize,
    pub wc_valid: bool,
    pub wc: i32,
    pub buf: [i8; 24],
}

pub type mbchar_t = mbchar;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbiter_multi {
    pub limit: *const i8,
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}

static IS_BASIC_TABLE: [u32; 0] = [];

pub fn mbiter_multi_copy(new_iter: &mut mbiter_multi, old_iter: &mbiter_multi) {
    new_iter.limit = old_iter.limit;
    new_iter.in_shift = old_iter.in_shift;
    
    if new_iter.in_shift {
        new_iter.state = old_iter.state;
    } else {
        new_iter.state = unsafe { mem::zeroed() };
    }
    
    new_iter.next_done = old_iter.next_done;
    mb_copy(&mut new_iter.cur, &old_iter.cur);
}

pub fn mbiter_multi_reloc(iter: &mut mbiter_multi, ptrdiff: isize) {
    unsafe {
        iter.cur.ptr = iter.cur.ptr.offset(ptrdiff);
        iter.limit = iter.limit.offset(ptrdiff);
    }
}

pub fn mbiter_multi_next(iter: &mut mbiter_multi) {
    if iter.next_done {
        return;
    }

    if iter.in_shift {
        handle_shift_state(iter);
    } else if is_basic(unsafe { *iter.cur.ptr }) {
        iter.cur.bytes = 1;
        iter.cur.wc = unsafe { *iter.cur.ptr } as i32;
        iter.cur.wc_valid = true;
    } else {
        iter.in_shift = true;
        handle_shift_state(iter);
    }

    iter.next_done = true;
}

fn handle_shift_state(iter: &mut mbiter_multi) {
    let bytes = unsafe {
        let distance = iter.limit.offset_from(iter.cur.ptr);
        rpl_mbrtowc(
            &mut iter.cur.wc,
            iter.cur.ptr,
            distance as usize,
            &mut iter.state,
        )
    };

    match bytes {
        usize::MAX => {
            iter.cur.bytes = 1;
            iter.cur.wc_valid = false;
        }
        usize::MAX - 1 => {
            iter.cur.bytes = unsafe { iter.limit.offset_from(iter.cur.ptr) as usize };
            iter.cur.wc_valid = false;
        }
        _ => {
            iter.cur.bytes = if bytes == 0 { 1 } else { bytes };
            iter.cur.wc_valid = true;
            if unsafe { mbsinit(&mut iter.state) } != 0 {
                iter.in_shift = false;
            }
        }
    }
}

fn is_basic(c: i8) -> bool {
    unsafe {
        (*IS_BASIC_TABLE
            .as_ptr()
            .offset((c as u8 as i32 >> 5) as isize)
            >> (c as u8 as i32 & 31))
            & 1u32
            != 0
    }
}

fn mb_copy(new_mbc: &mut mbchar_t, old_mbc: &mbchar_t) {
    if old_mbc.ptr == old_mbc.buf.as_ptr() as *const i8 {
        new_mbc.buf[..old_mbc.bytes].copy_from_slice(unsafe {
            std::slice::from_raw_parts(old_mbc.buf.as_ptr(), old_mbc.bytes)
        });
        new_mbc.ptr = new_mbc.buf.as_ptr() as *const i8;
    } else {
        new_mbc.ptr = old_mbc.ptr;
    }
    
    new_mbc.bytes = old_mbc.bytes;
    new_mbc.wc_valid = old_mbc.wc_valid;
    if new_mbc.wc_valid {
        new_mbc.wc = old_mbc.wc;
    }
}

extern "C" {
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn rpl_mbrtowc(pwc: *mut i32, s: *const i8, n: usize, ps: *mut mbstate_t) -> usize;
}