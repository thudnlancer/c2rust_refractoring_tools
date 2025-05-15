use std::mem;
use std::ptr;

const XTS_BLOCK_SIZE: usize = 16;

#[derive(Clone)]
struct Block16 {
    bytes: [u8; XTS_BLOCK_SIZE],
}

impl Block16 {
    fn new() -> Self {
        Block16 {
            bytes: [0; XTS_BLOCK_SIZE],
        }
    }

    fn mulx_le(&mut self) {
        let mut carry = 0;
        for i in (0..XTS_BLOCK_SIZE).rev() {
            let next_carry = (self.bytes[i] & 0x80) != 0;
            self.bytes[i] = (self.bytes[i] << 1) | carry;
            carry = if next_carry { 1 } else { 0 };
        }
        if carry != 0 {
            self.bytes[XTS_BLOCK_SIZE - 1] ^= 0x87;
        }
    }
}

fn memxor(dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len());
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d ^= *s;
    }
}

fn memxor3(dst: &mut [u8], a: &[u8], b: &[u8]) {
    assert_eq!(dst.len(), a.len());
    assert_eq!(dst.len(), b.len());
    for ((d, a), b) in dst.iter_mut().zip(a.iter()).zip(b.iter()) {
        *d = *a ^ *b;
    }
}

fn check_length(length: usize, dst: &mut [u8]) {
    assert!(length >= XTS_BLOCK_SIZE);
    if length < XTS_BLOCK_SIZE {
        dst.iter_mut().for_each(|x| *x = 0);
    }
}

pub fn xts_encrypt_message(
    enc_ctx: &dyn Fn(&[u8], &mut [u8]),
    twk_ctx: &dyn Fn(&[u8], &mut [u8]),
    tweak: &[u8],
    src: &[u8],
    dst: &mut [u8],
) {
    let length = src.len();
    check_length(length, dst);

    let mut T = Block16::new();
    twk_ctx(tweak, &mut T.bytes);

    let mut pos = 0;
    while length - pos >= 2 * XTS_BLOCK_SIZE || length - pos == XTS_BLOCK_SIZE {
        let mut P = Block16::new();
        memxor3(&mut P.bytes, &src[pos..], &T.bytes);
        enc_ctx(&P.bytes, &mut dst[pos..]);
        memxor(&mut dst[pos..], &T.bytes);

        pos += XTS_BLOCK_SIZE;
        if length - pos > XTS_BLOCK_SIZE {
            T.mulx_le();
        }
    }

    if length - pos > 0 {
        let mut S = Block16::new();
        let mut P = Block16::new();
        memxor3(&mut P.bytes, &src[pos..], &T.bytes);
        enc_ctx(&P.bytes, &mut S.bytes);
        memxor(&mut S.bytes, &T.bytes);

        T.mulx_le();
        pos += XTS_BLOCK_SIZE;

        let remaining = length - pos;
        memxor3(&mut P.bytes, &src[pos..], &T.bytes);
        memxor3(&mut P.bytes[remaining..], &S.bytes[remaining..], &T.bytes[remaining..]);

        enc_ctx(&P.bytes, &mut dst[pos - XTS_BLOCK_SIZE..pos]);
        memxor(&mut dst[pos - XTS_BLOCK_SIZE..pos], &T.bytes);

        dst[pos..pos + remaining].copy_from_slice(&S.bytes[..remaining]);
    }
}

pub fn xts_decrypt_message(
    dec_ctx: &dyn Fn(&[u8], &mut [u8]),
    twk_ctx: &dyn Fn(&[u8], &mut [u8]),
    encf: &dyn Fn(&[u8], &mut [u8]),
    tweak: &[u8],
    src: &[u8],
    dst: &mut [u8],
) {
    let length = src.len();
    check_length(length, dst);

    let mut T = Block16::new();
    encf(tweak, &mut T.bytes);

    let mut pos = 0;
    while length - pos >= 2 * XTS_BLOCK_SIZE || length - pos == XTS_BLOCK_SIZE {
        let mut C = Block16::new();
        memxor3(&mut C.bytes, &src[pos..], &T.bytes);
        dec_ctx(&C.bytes, &mut dst[pos..]);
        memxor(&mut dst[pos..], &T.bytes);

        pos += XTS_BLOCK_SIZE;
        if length - pos > XTS_BLOCK_SIZE {
            T.mulx_le();
        }
    }

    if length - pos > 0 {
        let mut T1 = T.clone();
        T1.mulx_le();

        let mut S = Block16::new();
        let mut C = Block16::new();
        memxor3(&mut C.bytes, &src[pos..], &T1.bytes);
        dec_ctx(&C.bytes, &mut S.bytes);
        memxor(&mut S.bytes, &T1.bytes);

        let remaining = length - pos;
        pos += XTS_BLOCK_SIZE;

        memxor3(&mut C.bytes, &src[pos..], &T.bytes);
        memxor3(&mut C.bytes[remaining..], &S.bytes[remaining..], &T.bytes[remaining..]);

        dec_ctx(&C.bytes, &mut dst[pos - XTS_BLOCK_SIZE..pos]);
        memxor(&mut dst[pos - XTS_BLOCK_SIZE..pos], &T.bytes);

        dst[pos..pos + remaining].copy_from_slice(&S.bytes[..remaining]);
    }
}

// AES-128 and AES-256 implementations would need to be provided separately
// as they would depend on specific cipher implementations