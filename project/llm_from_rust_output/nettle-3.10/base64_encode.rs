use std::convert::TryInto;

const BASE64_ENCODE_TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Clone, Copy)]
pub struct Base64EncodeCtx {
    alphabet: &'static [u8; 64],
    word: u16,
    bits: u8,
}

impl Default for Base64EncodeCtx {
    fn default() -> Self {
        Self {
            alphabet: &BASE64_ENCODE_TABLE,
            word: 0,
            bits: 0,
        }
    }
}

fn encode_raw(alphabet: &[u8; 64], dst: &mut [u8], src: &[u8]) {
    let length = src.len();
    let mut in_pos = length;
    let mut out_pos = ((length + 2) / 3) * 4;
    let left_over = length % 3;

    if left_over != 0 {
        in_pos -= left_over;
        out_pos -= 1;
        dst[out_pos] = b'=';
        
        match left_over {
            1 => {
                out_pos -= 1;
                dst[out_pos] = b'=';
                out_pos -= 1;
                dst[out_pos] = alphabet[(0x3f & (src[in_pos] as usize) << 4];
            }
            2 => {
                out_pos -= 1;
                dst[out_pos] = alphabet[(0x3f & (src[in_pos + 1] as usize) << 2];
                out_pos -= 1;
                dst[out_pos] = alphabet[(0x3f & ((src[in_pos] as usize) << 4 | (src[in_pos + 1] as usize) >> 4)];
            }
            _ => unreachable!(),
        }
        
        out_pos -= 1;
        dst[out_pos] = alphabet[(0x3f & (src[in_pos] as usize) >> 2];
    }

    while in_pos > 0 {
        in_pos -= 3;
        out_pos -= 1;
        dst[out_pos] = alphabet[(0x3f & src[in_pos + 2] as usize)];
        out_pos -= 1;
        dst[out_pos] = alphabet[(0x3f & ((src[in_pos + 1] as usize) << 2 | (src[in_pos + 2] as usize) >> 6))];
        out_pos -= 1;
        dst[out_pos] = alphabet[(0x3f & ((src[in_pos] as usize) << 4 | (src[in_pos + 1] as usize) >> 4))];
        out_pos -= 1;
        dst[out_pos] = alphabet[(0x3f & (src[in_pos] as usize) >> 2)];
    }

    assert_eq!(in_pos, 0);
    assert_eq!(out_pos, 0);
}

pub fn base64_encode_raw(dst: &mut [u8], src: &[u8]) {
    encode_raw(&BASE64_ENCODE_TABLE, dst, src);
}

pub fn base64_encode_group(dst: &mut [u8], group: u32) {
    dst[0] = BASE64_ENCODE_TABLE[(0x3f & (group >> 18)) as usize];
    dst[1] = BASE64_ENCODE_TABLE[(0x3f & (group >> 12)) as usize];
    dst[2] = BASE64_ENCODE_TABLE[(0x3f & (group >> 6)) as usize];
    dst[3] = BASE64_ENCODE_TABLE[(0x3f & group) as usize];
}

pub fn base64_encode_init(ctx: &mut Base64EncodeCtx) {
    ctx.bits = 0;
    ctx.word = 0;
    ctx.alphabet = &BASE64_ENCODE_TABLE;
}

pub fn base64_encode_single(ctx: &mut Base64EncodeCtx, dst: &mut [u8], src: u8) -> usize {
    let mut done = 0;
    let mut word = ((ctx.word as u32) << 8 | src as u32);
    let mut bits = ctx.bits as u32 + 8;

    while bits >= 6 {
        bits -= 6;
        dst[done] = ctx.alphabet[(0x3f & (word >> bits)) as usize];
        done += 1;
    }

    ctx.bits = bits as u8;
    ctx.word = word as u16;
    assert!(done <= 2);
    done
}

pub fn base64_encode_update(ctx: &mut Base64EncodeCtx, dst: &mut [u8], src: &[u8]) -> usize {
    let mut done = 0;
    let mut left = src.len();
    let mut src_pos = 0;

    while ctx.bits != 0 && left != 0 {
        left -= 1;
        done += base64_encode_single(ctx, &mut dst[done..], src[src_pos]);
        src_pos += 1;
    }

    let left_over = left % 3;
    let bulk = left - left_over;

    if bulk != 0 {
        assert!(bulk % 3 == 0);
        encode_raw(ctx.alphabet, &mut dst[done..], &src[src_pos..src_pos + bulk]);
        done += (bulk + 2) / 3 * 4;
        src_pos += bulk;
        left = left_over;
    }

    while left != 0 {
        left -= 1;
        done += base64_encode_single(ctx, &mut dst[done..], src[src_pos]);
        src_pos += 1;
    }

    assert!(done <= (src.len() * 8 + 4) / 6);
    done
}

pub fn base64_encode_final(ctx: &mut Base64EncodeCtx, dst: &mut [u8]) -> usize {
    let mut done = 0;
    let bits = ctx.bits as u32;

    if bits != 0 {
        dst[done] = ctx.alphabet[(0x3f & ((ctx.word as u32) << (6 - bits))) as usize];
        done += 1;
        
        let mut padding_bits = bits;
        while padding_bits < 6 {
            dst[done] = b'=';
            done += 1;
            padding_bits += 2;
        }

        ctx.bits = 0;
    }

    assert!(done <= 3);
    done
}