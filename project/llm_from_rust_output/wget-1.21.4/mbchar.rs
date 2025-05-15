use std::mem;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

pub type size_t = usize;
pub type wchar_t = c_int;
pub type wint_t = c_uint;

#[derive(Clone)]
pub struct MbChar {
    ptr: *const c_char,
    bytes: size_t,
    wc_valid: bool,
    wc: wchar_t,
    buf: [c_char; 24],
}

impl MbChar {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null(),
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        }
    }

    pub fn copy_from(&mut self, other: &MbChar) {
        if other.ptr == other.buf.as_ptr() {
            self.buf[..other.bytes].copy_from_slice(&other.buf[..other.bytes]);
            self.ptr = self.buf.as_ptr();
        } else {
            self.ptr = other.ptr;
        }
        self.bytes = other.bytes;
        self.wc_valid = other.wc_valid;
        if self.wc_valid {
            self.wc = other.wc;
        }
    }
}

pub fn is_basic(c: c_char) -> bool {
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

    let idx = (c as u8 as usize) >> 5;
    let bit = (c as u8 as u32) & 31;
    (IS_BASIC_TABLE.get(idx).unwrap_or(&0) >> bit) & 1 != 0
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