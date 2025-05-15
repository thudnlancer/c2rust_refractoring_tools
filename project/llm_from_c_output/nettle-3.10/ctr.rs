use std::cmp::min;
use std::mem;

const NETTLE_MAX_CIPHER_BLOCK_SIZE: usize = 32;
const CTR_BUFFER_LIMIT: usize = 1024;

pub type NettleCipherFunc = fn(ctx: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

#[derive(Debug)]
pub struct CtrContext<C> {
    pub ctx: C,
    pub ctr: Vec<u8>,
}

impl<C> CtrContext<C> {
    pub fn new(block_size: usize) -> Self 
    where
        C: Default,
    {
        CtrContext {
            ctx: C::default(),
            ctr: vec![0; block_size],
        }
    }

    pub fn set_counter(&mut self, data: &[u8]) {
        self.ctr.copy_from_slice(data);
    }
}

pub fn ctr_crypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    ctr: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    if src.len() != dst.len() {
        panic!("Source and destination buffers must have equal length");
    }

    if block_size == 16 {
        ctr_crypt16(ctx, f, ctr, length, dst, src);
        return;
    }

    if !src.is_empty() && !dst.is_empty() && src.as_ptr() != dst.as_ptr() {
        let filled = ctr_fill(block_size, ctr, length, dst);

        let (dst_block, remaining_dst) = dst.split_at_mut(filled);
        let (src_block, remaining_src) = src.split_at(filled);

        f(ctx, filled, dst_block, dst_block);
        memxor(dst_block, src_block);

        if filled < length {
            let mut block = vec![0; block_size];
            f(ctx, block_size, &mut block, ctr);
            increment(block_size, ctr);
            memxor3(
                &mut remaining_dst[0..length - filled],
                &remaining_src[0..length - filled],
                &block[0..length - filled],
            );
        }
    } else {
        let buffer_size = if length < block_size {
            block_size
        } else if length <= CTR_BUFFER_LIMIT {
            length
        } else {
            CTR_BUFFER_LIMIT
        };

        let mut buffer = vec![0; buffer_size];
        let mut remaining_length = length;
        let mut dst_offset = 0;

        while remaining_length >= block_size {
            let filled = ctr_fill(
                block_size,
                ctr,
                min(buffer_size, remaining_length),
                &mut buffer,
            );
            assert!(filled > 0);
            f(ctx, filled, &mut buffer[..filled], &buffer[..filled]);
            memxor(&mut dst[dst_offset..dst_offset + filled], &buffer[..filled]);
            remaining_length -= filled;
            dst_offset += filled;
        }

        if remaining_length > 0 {
            f(ctx, block_size, &mut buffer, ctr);
            increment(block_size, ctr);
            memxor(
                &mut dst[dst_offset..dst_offset + remaining_length],
                &buffer[..remaining_length],
            );
        }
    }
}

fn ctr_crypt16(
    ctx: &[u8],
    f: NettleCipherFunc,
    ctr: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    if cfg!(target_endian = "big") {
        ctr_fill16_big_endian(ctr, length, dst, src);
    } else if cfg!(all(target_endian = "little", feature = "bswap64")) {
        ctr_fill16_little_endian_bswap(ctr, length, dst, src);
    } else {
        // Fallback to generic implementation
        ctr_fill_generic(16, ctr, length, dst, src);
    }

    f(ctx, length, dst, dst);
    memxor(dst, src);
}

fn ctr_fill(
    block_size: usize,
    ctr: &mut [u8],
    length: usize,
    buffer: &mut [u8],
) -> usize {
    let mut i = 0;
    while i + block_size <= length {
        buffer[i..i + block_size].copy_from_slice(ctr);
        increment(block_size, ctr);
        i += block_size;
    }
    i
}

fn ctr_fill_generic(
    block_size: usize,
    ctr: &mut [u8],
    length: usize,
    buffer: &mut [u8],
    src: &[u8],
) {
    let mut i = 0;
    while i + block_size <= length {
        buffer[i..i + block_size].copy_from_slice(ctr);
        increment(block_size, ctr);
        i += block_size;
    }
}

#[cfg(target_endian = "big")]
fn ctr_fill16_big_endian(
    ctr: &mut [u8],
    length: usize,
    buffer: &mut [u8],
    src: &[u8],
) {
    let mut hi = u64::from_be_bytes(ctr[0..8].try_into().unwrap());
    let mut lo = u64::from_be_bytes(ctr[8..16].try_into().unwrap());

    let blocks = length / 16;
    for i in 0..blocks {
        buffer[i*16..i*16+8].copy_from_slice(&hi.to_be_bytes());
        buffer[i*16+8..i*16+16].copy_from_slice(&lo.to_be_bytes());
        hi = hi.wrapping_add(!lo.checked_add(1).unwrap_or(0) as u64);
        lo = lo.wrapping_add(1);
    }

    ctr[0..8].copy_from_slice(&hi.to_be_bytes());
    ctr[8..16].copy_from_slice(&lo.to_be_bytes());
}

#[cfg(all(target_endian = "little", feature = "bswap64"))]
fn ctr_fill16_little_endian_bswap(
    ctr: &mut [u8],
    length: usize,
    buffer: &mut [u8],
    src: &[u8],
) {
    let mut hi = u64::from_le_bytes(ctr[0..8].try_into().unwrap());
    let mut lo = u64::from_be_bytes(ctr[8..16].try_into().unwrap());

    let blocks = length / 16;
    for i in 0..blocks {
        buffer[i*16..i*16+8].copy_from_slice(&hi.to_le_bytes());
        buffer[i*16+8..i*16+16].copy_from_slice(&lo.to_be_bytes());
        if lo == u64::MAX {
            hi = hi.swap_bytes().wrapping_add(1).swap_bytes();
        }
        lo = lo.wrapping_add(1);
    }

    ctr[0..8].copy_from_slice(&hi.to_le_bytes());
    ctr[8..16].copy_from_slice(&lo.to_be_bytes());
}

fn increment(block_size: usize, ctr: &mut [u8]) {
    for i in (0..block_size).rev() {
        ctr[i] = ctr[i].wrapping_add(1);
        if ctr[i] != 0 {
            break;
        }
    }
}

fn memxor(dst: &mut [u8], src: &[u8]) {
    for (d, s) in dst.iter_mut().zip(src) {
        *d ^= *s;
    }
}

fn memxor3(dst: &mut [u8], src1: &[u8], src2: &[u8]) {
    for ((d, s1), s2) in dst.iter_mut().zip(src1).zip(src2) {
        *d = *s1 ^ *s2;
    }
}