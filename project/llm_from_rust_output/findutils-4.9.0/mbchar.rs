use std::mem;

#[derive(Copy, Clone)]
pub struct MbChar {
    ptr: *const u8,
    bytes: usize,
    wc_valid: bool,
    wc: i32,
    buf: [u8; 24],
}

pub type MbCharT = MbChar;

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

pub fn is_basic(c: u8) -> bool {
    (IS_BASIC_TABLE[(c as usize >> 5)] >> (c & 31) & 1 != 0
}

pub fn mb_copy(new_mbc: &mut MbCharT, old_mbc: &MbCharT) {
    let buf_ptr = old_mbc.buf.as_ptr();
    
    if old_mbc.ptr == buf_ptr {
        new_mbc.buf[..old_mbc.bytes].copy_from_slice(&old_mbc.buf[..old_mbc.bytes]);
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

pub fn mb_width_aux(wc: u32) -> i32 {
    let w = unsafe { libc::wcwidth(wc as i32) };
    if w >= 0 {
        w
    } else if unsafe { libc::iswcntrl(wc as _) } != 0 {
        0
    } else {
        1
    }
}