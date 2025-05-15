use std::mem;
use std::ptr;
use std::slice;

type size_t = usize;
type uint8_t = u8;
type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

pub type NettleCipherFunc = fn(key: &[u8], length: size_t, dst: &mut [u8], src: &[u8]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcbKey {
    pub L: [NettleBlock16; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcbCtx {
    pub initial: NettleBlock16,
    pub offset: NettleBlock16,
    pub sum: NettleBlock16,
    pub checksum: NettleBlock16,
    pub data_count: size_t,
    pub message_count: size_t,
}

impl NettleBlock16 {
    #[inline]
    fn mulx_be(&mut self, src: &NettleBlock16) {
        unsafe {
            let carry = (src.u64_0[0] & 0x80) >> 7;
            self.u64_0[0] = ((src.u64_0[0] & 0x7f7f7f7f7f7f7f7f) << 1)
                | ((src.u64_0[0] & 0x8080808080808080) >> 15)
                | ((src.u64_0[1] & 0x80) << 49);
            self.u64_0[1] = ((src.u64_0[1] & 0x7f7f7f7f7f7f7f7f) << 1)
                | ((src.u64_0[1] & 0x8080808080808080) >> 15)
                ^ (0x87 << 56 & carry.wrapping_neg());
        }
    }

    #[inline]
    fn xor(&mut self, x: &NettleBlock16) {
        unsafe {
            self.u64_0[0] ^= x.u64_0[0];
            self.u64_0[1] ^= x.u64_0[1];
        }
    }

    #[inline]
    fn set(&mut self, x: &NettleBlock16) {
        unsafe {
            self.u64_0[0] = x.u64_0[0];
            self.u64_0[1] = x.u64_0[1];
        }
    }

    #[inline]
    fn xor3(&mut self, x: &NettleBlock16, y: &NettleBlock16) {
        unsafe {
            self.u64_0[0] = x.u64_0[0] ^ y.u64_0[0];
            self.u64_0[1] = x.u64_0[1] ^ y.u64_0[1];
        }
    }
}

fn extract(u0: uint64_t, u1: uint64_t, offset: u32) -> uint64_t {
    if offset == 0 {
        return u0;
    }
    let u0 = u0.swap_bytes();
    let u1 = u1.swap_bytes();
    ((u0 << offset | u1 >> (64 - offset)) as u64).swap_bytes()
}

pub fn nettle_ocb_set_key(key: &mut OcbKey, cipher: &[u8], f: NettleCipherFunc) {
    let zero_block = NettleBlock16 { b: [0; 16] };
    let mut temp = zero_block;
    f(cipher, 16, &mut temp.b, &zero_block.b);
    key.L[0] = temp;

    let mut temp = key.L[0];
    temp.mulx_be(&key.L[0]);
    key.L[1] = temp;

    let mut temp = key.L[1];
    temp.mulx_be(&key.L[1]);
    key.L[2] = temp;
}

fn update_offset(key: &OcbKey, offset: &mut NettleBlock16, i: size_t) {
    if i & 1 != 0 {
        offset.xor(&key.L[2]);
    } else {
        assert!(i > 0);
        let mut diff = key.L[2];
        diff.mulx_be(&key.L[2]);
        let mut i = i >> 1;
        while i & 1 == 0 {
            diff.mulx_be(&diff);
            i >>= 1;
        }
        offset.xor(&diff);
    }
}

fn pad_block(block: &mut NettleBlock16, length: size_t, data: &[u8]) {
    unsafe {
        ptr::copy_nonoverlapping(data.as_ptr(), block.b.as_mut_ptr(), length);
        block.b[length] = 0x80;
        ptr::write_bytes(block.b.as_mut_ptr().add(length + 1), 0, 15 - length);
    }
}

pub fn nettle_ocb_set_nonce(
    ctx: &mut OcbCtx,
    cipher: &[u8],
    f: NettleCipherFunc,
    tag_length: size_t,
    nonce_length: size_t,
    nonce: &[u8],
) {
    assert!(nonce_length < 16);
    assert!(tag_length > 0 && tag_length <= 16);

    let mut top = NettleBlock16 { b: [0; 16] };
    top.b[0] = ((tag_length & 15) << 4) as u8;
    top.b[15 - nonce_length] |= 1;
    top.b[15 - nonce_length..15].copy_from_slice(&nonce[..nonce_length]);

    f(cipher, 16, &mut top.b, &top.b);

    let bottom = (top.b[15] & 0x3f) as u32;
    top.b[15] &= 0xc0;

    let stretch = top.u64_0[0] ^ ((top.u64_0[0] >> 8) | (top.u64_0[1] << 56));

    ctx.initial.u64_0[0] = extract(top.u64_0[0], top.u64_0[1], bottom);
    ctx.initial.u64_0[1] = extract(top.u64_0[1], stretch, bottom);
    ctx.sum = NettleBlock16 { u64_0: [0; 2] };
    ctx.checksum = NettleBlock16 { u64_0: [0; 2] };
    ctx.message_count = 0;
    ctx.data_count = 0;
}

fn ocb_fill_n(
    key: &OcbKey,
    offset: &mut NettleBlock16,
    count: size_t,
    n: size_t,
    o: &mut [NettleBlock16],
) {
    assert!(n > 0);
    let mut prev = if count & 1 != 0 {
        offset
    } else {
        let count = count + 1;
        offset.xor(&key.L[2]);
        o[0].set(offset);
        n -= 1;
        &mut o[0]
    };

    let mut i = 0;
    while n >= 2 {
        let count = count + 2;
        o[i].mulx_be(&key.L[2]);
        let mut j = count >> 1;
        while j & 1 == 0 {
            o[i].mulx_be(&o[i]);
            j >>= 1;
        }
        o[i].xor(prev);
        o[i + 1].xor3(&o[i], &key.L[2]);
        prev = &mut o[i + 1];
        n -= 2;
        i += 2;
    }

    if n > 0 {
        let count = count + 1;
        update_offset(key, offset, count);
        o[i].set(offset);
    }
}

pub fn nettle_ocb_update(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: size_t,
    data: &[u8],
) {
    assert!(ctx.message_count == 0);

    if ctx.data_count == 0 {
        ctx.offset = NettleBlock16 { u64_0: [0; 2] };
    }

    let mut n = length / 16;
    let mut data_ptr = data.as_ptr();
    let mut block = [NettleBlock16 { b: [0; 16] }; 16];

    while n > 0 {
        let blocks = if n <= 16 { n } else { 15 + (ctx.data_count & 1) };
        ocb_fill_n(key, &mut ctx.offset, ctx.data_count, blocks, &mut block);
        ctx.data_count += blocks;

        let size = blocks * 16;
        xor_blocks(&mut block[0].b, data_ptr, size);
        f(cipher, size, &mut block[0].b, &block[0].b);

        for i in 0..blocks {
            ctx.sum.xor(&block[i]);
        }

        n -= blocks;
        data_ptr = unsafe { data_ptr.add(size) };
    }

    let remaining = length % 16;
    if remaining > 0 {
        let mut pad_block = NettleBlock16 { b: [0; 16] };
        pad_block(&mut pad_block, remaining, unsafe {
            slice::from_raw_parts(data_ptr, remaining)
        });
        ctx.offset.xor(&key.L[0]);
        pad_block.xor(&ctx.offset);
        f(cipher, 16, &mut pad_block.b, &pad_block.b);
        ctx.sum.xor(&pad_block);
    }
}

fn xor_blocks(dst: &mut [u8], src: *const u8, n: size_t) {
    unsafe {
        for i in 0..n {
            dst[i] ^= *src.add(i);
        }
    }
}

fn ocb_crypt_n(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    n: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut o = [NettleBlock16 { b: [0; 16] }; 16];
    let mut block = [NettleBlock16 { b: [0; 16] }; 16];
    let mut src_ptr = src.as_ptr();
    let mut dst_ptr = dst.as_mut_ptr();
    let mut remaining = n;

    while remaining > 0 {
        let blocks = if remaining <= 16 {
            remaining
        } else {
            15 + (ctx.message_count & 1)
        };
        ocb_fill_n(key, &mut ctx.offset, ctx.message_count, blocks, &mut o);
        ctx.message_count += blocks;

        let size = blocks * 16;
        xor_blocks3(&mut block[0].b, &o[0].b, src_ptr, size);
        f(cipher, size, &mut block[0].b, &block[0].b);
        xor_blocks3(dst_ptr, &block[0].b, &o[0].b, size);

        remaining -= blocks;
        src_ptr = unsafe { src_ptr.add(size) };
        dst_ptr = unsafe { dst_ptr.add(size) };
    }
}

fn xor_blocks3(dst: *mut u8, a: &[u8], b: *const u8, n: size_t) {
    unsafe {
        for i in 0..n {
            *dst.add(i) = a[i] ^ *b.add(i);
        }
    }
}

fn ocb_checksum_n(checksum: &mut NettleBlock16, n: size_t, src: &[u8]) {
    if n == 1 {
        xor_blocks(&mut checksum.b, src.as_ptr(), 16);
        return;
    }

    let mut initial = (!(src.as_ptr() as usize) & 7) as u32;
    let mut edge_word = 0u64;
    let mut src_ptr = src.as_ptr();

    if initial > 0 {
        for _ in 0..initial {
            edge_word = (edge_word << 8) | unsafe { *src_ptr } as u64;
            src_ptr = unsafe { src_ptr.add(1) };
        }
    }

    let mut s0 = 0u64;
    let mut s1 = 0u64;
    let mut remaining = n;

    while remaining > 0 {
        s0 ^= unsafe { *(src_ptr as *const u64) };
        s1 ^= unsafe { *(src_ptr.add(8) as *const u64) };
        remaining -= 1;
        src_ptr = unsafe { src_ptr.add(16) };
    }

    if initial > 0 {
        s0 ^= unsafe { *(src_ptr as *const u64) };
        src_ptr = unsafe { src_ptr.add(8) };
        for _ in 0..(8 - initial) {
            edge_word = (edge_word << 8) | unsafe { *src_ptr } as u64;
            src_ptr = unsafe { src_ptr.add(1) };
        }

        edge_word = edge_word.swap_bytes();
        let mask = (1u64 << (8 * initial)) - 1;
        s0 ^= edge_word & mask;
        s1 ^= edge_word & !mask;
    }

    checksum.u64_0[0] ^= s0;
    checksum.u64_0[1] ^= s1;
}

pub fn nettle_ocb_encrypt(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    if ctx.message_count == 0 {
        ctx.offset = ctx.initial;
    }

    let n = length / 16;
    if n > 0 {
        ocb_checksum_n(&mut ctx.checksum, n, src);
        ocb_crypt_n(ctx, key, cipher, f, n, dst, src);
    }

    let remaining = length % 16;
    if remaining > 0 {
        let offset = n * 16;
        let mut block = NettleBlock16 { b: [0; 16] };
        pad_block(&mut block, remaining, &src[offset..]);
        ctx.checksum.xor(&block);
        ctx.offset.xor(&key.L[0]);
        f(cipher, 16, &mut block.b, &ctx.offset.b);
        xor_blocks3(
            unsafe { dst.as_mut_ptr().add(offset) },
            &block.b,
            unsafe { src.as_ptr().add(offset) },
            remaining,
        );
        ctx.message_count += 1;
    }
}

pub fn nettle_ocb_decrypt(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    encrypt_ctx: &[u8],
    encrypt: NettleCipherFunc,
    decrypt_ctx: &[u8],
    decrypt: NettleCipherFunc,
    length: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    if ctx.message_count == 0 {
        ctx.offset = ctx.initial;
    }

    let n = length / 16;
    if n > 0 {
        ocb_crypt_n(ctx, key, decrypt_ctx, decrypt, n, dst, src);
        ocb_checksum_n(&mut ctx.checksum, n, dst);
    }

    let remaining = length % 16;
    if remaining > 0 {
        let offset = n * 16;
        ctx.offset.xor(&key.L[0]);
        let mut block = NettleBlock16 { b: [0; 16] };
        encrypt(encrypt_ctx, 16, &mut block.b, &ctx.offset.b);
        xor_blocks3(
            unsafe { dst.as_mut_ptr().add(offset) },
            &block.b,
            unsafe { src.as_ptr().add(offset) },
            remaining,
        );
        pad_block(&mut block, remaining, &dst[offset..]);
        ctx.checksum.xor(&block);
        ctx.message_count += 1;
    }
}

pub fn nettle_ocb_digest(
    ctx: &OcbCtx,
    key: &OcbKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: size_t,
    digest: &mut [u8],
) {
    assert!(length <= 16);

    let mut block = if ctx.message_count > 0 {
        ctx.offset
    } else {
        ctx.initial
    };
    block.xor3(&key.L[1], &ctx.checksum);
    f(cipher, 16, &mut block.b, &block.b);
    xor_blocks3(digest.as_mut_ptr(), &block.b, &ctx.sum.b, length);
}

pub fn nettle_ocb_encrypt_message(
    key: &OcbKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    nlength: size_t,
    nonce: &[u8],
    alength: size_t,
    adata: &[u8],
    tlength: size_t,
    clength: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(clength >= tlength);

    let mut ctx = OcbCtx {
        initial: NettleBlock16 { b: [0; 16] },
        offset: NettleBlock16 { b: [0; 16] },
        sum: NettleBlock16 { b: [0; 16] },
        checksum: NettleBlock16 { b: [0; 16] },
        data_count: 0,
        message_count: 0,
    };

    nettle_ocb_set_nonce(&mut ctx, cipher, f, tlength, nlength, non