use std::cmp::min;
use std::ptr;

const CFB_BUFFER_LIMIT: usize = 512;
const NETTLE_MAX_CIPHER_BLOCK_SIZE: usize = 32;

pub type NettleCipherFunc = fn(ctx: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

pub fn cfb_encrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(block_size <= NETTLE_MAX_CIPHER_BLOCK_SIZE);
    assert!(iv.len() >= block_size);
    assert!(dst.len() >= length);
    assert!(src.len() >= length);

    let mut buffer = vec![0u8; block_size];
    let mut p = iv;

    if !ptr::eq(src, dst) {
        let mut remaining = length;
        let mut offset = 0;
        while remaining >= block_size {
            f(ctx, block_size, &mut dst[offset..], p);
            xor_bytes(&mut dst[offset..], &src[offset..], block_size);
            p = &mut dst[offset..];
            offset += block_size;
            remaining -= block_size;
        }
    } else {
        let mut remaining = length;
        let mut offset = 0;
        while remaining >= block_size {
            f(ctx, block_size, &mut buffer, p);
            xor_bytes(&mut dst[offset..], &buffer, block_size);
            p = &mut dst[offset..];
            offset += block_size;
            remaining -= block_size;
        }
    }

    if !ptr::eq(p, iv) {
        iv[..block_size].copy_from_slice(&p[..block_size]);
    }

    if length % block_size != 0 {
        f(ctx, block_size, &mut buffer, iv);
        xor_bytes3(
            &mut dst[length - (length % block_size)..],
            &buffer,
            &src[length - (length % block_size)..],
            length % block_size,
        );
    }
}

pub fn cfb_decrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(block_size <= NETTLE_MAX_CIPHER_BLOCK_SIZE);
    assert!(iv.len() >= block_size);
    assert!(dst.len() >= length);
    assert!(src.len() >= length);

    if !ptr::eq(src, dst) {
        let left = length % block_size;
        let full_blocks = length - left;

        if full_blocks > 0 {
            f(ctx, block_size, dst, iv);
            f(ctx, full_blocks - block_size, &mut dst[block_size..], &src[block_size..]);
            iv.copy_from_slice(&src[full_blocks - block_size..full_blocks]);
            xor_bytes(dst, src, full_blocks);
        }

        if left > 0 {
            let mut buffer = vec![0u8; block_size];
            f(ctx, block_size, &mut buffer, iv);
            xor_bytes3(&mut dst[full_blocks..], &src[full_blocks..], &buffer, left);
        }
    } else {
        let mut buffer = vec![0u8; CFB_BUFFER_LIMIT];
        let mut initial_iv = vec![0u8; block_size];
        let buffer_size = CFB_BUFFER_LIMIT - (CFB_BUFFER_LIMIT % block_size);

        let left = length % block_size;
        let mut remaining = length - left;
        let mut offset = 0;

        while remaining > 0 {
            let part = min(remaining, buffer_size);
            f(ctx, block_size, &mut buffer, iv);
            f(ctx, part - block_size, &mut buffer[block_size..], &dst[offset + block_size..]);
            iv.copy_from_slice(&dst[offset + part - block_size..offset + part]);
            xor_bytes(&mut dst[offset..], &buffer, part);

            remaining -= part;
            offset += part;
        }

        if left > 0 {
            f(ctx, block_size, &mut buffer, iv);
            xor_bytes(&mut dst[offset..], &buffer, left);
        }
    }
}

pub fn cfb8_encrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(block_size <= NETTLE_MAX_CIPHER_BLOCK_SIZE);
    assert!(iv.len() >= block_size);
    assert!(dst.len() >= length);
    assert!(src.len() >= length);

    let mut buffer = vec![0u8; block_size * 2];
    let mut outbuf = vec![0u8; block_size];
    buffer[..block_size].copy_from_slice(iv);
    let mut pos = 0;
    let mut remaining = length;
    let mut offset = 0;

    while remaining > 0 {
        if pos == block_size {
            buffer[..block_size].copy_from_slice(&buffer[block_size..]);
            pos = 0;
        }

        f(ctx, block_size, &mut outbuf, &buffer[pos..]);
        let t = src[offset] ^ outbuf[0];
        dst[offset] = t;
        buffer[pos + block_size] = t;
        remaining -= 1;
        pos += 1;
        offset += 1;
    }

    iv.copy_from_slice(&buffer[pos..pos + block_size]);
}

pub fn cfb8_decrypt(
    ctx: &[u8],
    f: NettleCipherFunc,
    block_size: usize,
    iv: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(block_size <= NETTLE_MAX_CIPHER_BLOCK_SIZE);
    assert!(iv.len() >= block_size);
    assert!(dst.len() >= length);
    assert!(src.len() >= length);

    let mut buffer = vec![0u8; block_size * 2];
    let mut outbuf = vec![0u8; block_size * 2];
    buffer[..block_size].copy_from_slice(iv);
    buffer[block_size..block_size + min(length, block_size)].copy_from_slice(&src[..min(length, block_size)]);

    let mut remaining = length;
    let mut offset = 0;

    while remaining > 0 {
        let i = min(remaining, block_size);
        for j in 0..i {
            f(ctx, block_size, &mut outbuf[j..], &buffer[j..]);
        }

        xor_bytes3(&mut dst[offset..], &src[offset..], &outbuf, i);

        remaining -= i;
        offset += i;

        if i == block_size {
            buffer[..block_size].copy_from_slice(&buffer[block_size..]);
            buffer[block_size..block_size + min(remaining, block_size)]
                .copy_from_slice(&src[offset..offset + min(remaining, block_size)]);
        }
    }

    iv.copy_from_slice(&buffer[offset..offset + block_size]);
}

#[inline]
fn xor_bytes(dst: &mut [u8], src: &[u8], len: usize) {
    for i in 0..len {
        dst[i] ^= src[i];
    }
}

#[inline]
fn xor_bytes3(dst: &mut [u8], src1: &[u8], src2: &[u8], len: usize) {
    for i in 0..len {
        dst[i] = src1[i] ^ src2[i];
    }
}

pub struct CfbCtx<T> {
    ctx: T,
    iv: Vec<u8>,
}

impl<T> CfbCtx<T> {
    pub fn new(ctx: T, block_size: usize) -> Self {
        Self {
            ctx,
            iv: vec![0; block_size],
        }
    }

    pub fn set_iv(&mut self, data: &[u8]) {
        self.iv.copy_from_slice(data);
    }

    pub fn encrypt(&self, f: NettleCipherFunc, length: usize, dst: &mut [u8], src: &[u8]) {
        cfb_encrypt(
            &self.ctx as *const _ as *const u8,
            f,
            self.iv.len(),
            &mut self.iv.clone(),
            length,
            dst,
            src,
        );
    }

    pub fn decrypt(&self, f: NettleCipherFunc, length: usize, dst: &mut [u8], src: &[u8]) {
        cfb_decrypt(
            &self.ctx as *const _ as *const u8,
            f,
            self.iv.len(),
            &mut self.iv.clone(),
            length,
            dst,
            src,
        );
    }
}

pub type Cfb8Ctx<T> = CfbCtx<T>;

impl<T> Cfb8Ctx<T> {
    pub fn encrypt8(&self, f: NettleCipherFunc, length: usize, dst: &mut [u8], src: &[u8]) {
        cfb8_encrypt(
            &self.ctx as *const _ as *const u8,
            f,
            self.iv.len(),
            &mut self.iv.clone(),
            length,
            dst,
            src,
        );
    }

    pub fn decrypt8(&self, f: NettleCipherFunc, length: usize, dst: &mut [u8], src: &[u8]) {
        cfb8_decrypt(
            &self.ctx as *const _ as *const u8,
            f,
            self.iv.len(),
            &mut self.iv.clone(),
            length,
            dst,
            src,
        );
    }
}