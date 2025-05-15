use std::mem;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

pub type size_t = usize;
pub type wchar_t = c_int;
pub type wint_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbChar {
    ptr: *const c_char,
    bytes: size_t,
    wc_valid: bool,
    wc: wchar_t,
    buf: [c_char; 24],
}

pub type mbchar_t = MbChar;

const IS_BASIC_TABLE: [u32; 8] = [
    0x1a00,
    0xffffffef,
    0xfffffffe,
    0x7ffffffe,
    0,
    0,
    0,
    0,
];

pub fn is_basic(c: c_char) -> bool {
    let idx = (c as u8 as usize) >> 5;
    let bit = (c as u8 as u32) & 31;
    (IS_BASIC_TABLE.get(idx).unwrap_or(&0) >> bit) & 1 != 0
}

pub fn mb_copy(new_mbc: &mut mbchar_t, old_mbc: &mbchar_t) {
    let is_internal = old_mbc.ptr == old_mbc.buf.as_ptr() as *const c_char;
    
    if is_internal {
        new_mbc.buf[..old_mbc.bytes].copy_from_slice(
            &old_mbc.buf[..old_mbc.bytes]
        );
        new_mbc.ptr = new_mbc.buf.as_ptr();
    } else {
        new_mbc.ptr = old_mbc.ptr;
    }
    
    new_mbc.bytes = old_mbc.bytes;
    new_mbc.wc_valid = old_mbc.wc_valid;
    if new_mbc.wc_valid {
        new_mbc.wc = old_mbc.wc;
    }
}

pub fn mb_width_aux(wc: wint_t) -> c_int {
    let w = unsafe { libc::wcwidth(wc as wchar_t) };
    if w >= 0 {
        w
    } else if unsafe { libc::iswcntrl(wc as _) } != 0 {
        0
    } else {
        1
    }
}