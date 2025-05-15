use std::mem;
use std::ptr;
use std::cmp;

const OCB_BLOCK_SIZE: usize = 16;
const OCB_DIGEST_SIZE: usize = 16;
const OCB_MAX_NONCE_SIZE: usize = 15;
const OCB_MAX_BLOCKS: usize = 16;

#[derive(Clone, Copy)]
struct Block16 {
    b: [u8; OCB_BLOCK_SIZE],
}

impl Block16 {
    fn new() -> Self {
        Block16 { b: [0; OCB_BLOCK_SIZE] }
    }

    fn xor(&mut self, other: &Block16) {
        for i in 0..OCB_BLOCK_SIZE {
            self.b[i] ^= other.b[i];
        }
    }

    fn xor3(&mut self, a: &Block16, b: &Block16) {
        for i in 0..OCB_BLOCK_SIZE {
            self.b[i] = a.b[i] ^ b.b[i];
        }
    }

    fn set(&mut self, other: &Block16) {
        self.b.copy_from_slice(&other.b);
    }
}

struct OcbKey {
    l: [Block16; 4],
}

impl OcbKey {
    fn new() -> Self {
        OcbKey { l: [Block16::new(); 4] }
    }
}

struct OcbCtx {
    initial: Block16,
    offset: Block16,
    sum: Block16,
    checksum: Block16,
    data_count: usize,
    message_count: usize,
}

impl OcbCtx {
    fn new() -> Self {
        OcbCtx {
            initial: Block16::new(),
            offset: Block16::new(),
            sum: Block16::new(),
            checksum: Block16::new(),
            data_count: 0,
            message_count: 0,
        }
    }
}

fn block16_mulx_be(dst: &mut Block16, src: &Block16) {
    let mut carry = (src.b[0] & 0x80) != 0;
    for i in 0..OCB_BLOCK_SIZE {
        let next_carry = (src.b[i] & 0x80) != 0;
        dst.b[i] = (src.b[i] << 1) | if carry { 1 } else { 0 };
        carry = next_carry;
    }
    if carry {
        dst.b[OCB_BLOCK_SIZE - 1] ^= 0x87;
    }
}

fn ocb_set_key(key: &mut OcbKey, cipher: &dyn Fn(&[u8], &mut [u8])) {
    let zero_block = Block16::new();
    cipher(&zero_block.b, &mut key.l[0].b);
    block16_mulx_be(&mut key.l[1], &key.l[0]);
    block16_mulx_be(&mut key.l[2], &key.l[1]);
}

fn extract(u0: u64, u1: u64, offset: usize) -> u64 {
    if offset == 0 {
        u0
    } else {
        let u0 = u0.to_be();
        let u1 = u1.to_be();
        ((u0 << offset) | (u1 >> (64 - offset))).to_be()
    }
}

fn ocb_set_nonce(
    ctx: &mut OcbCtx,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    tag_length: usize,
    nonce_length: usize,
    nonce: &[u8],
) {
    assert!(nonce_length < 16);
    assert!(tag_length > 0 && tag_length <= 16);

    let mut top = Block16::new();
    top.b[0] = (tag_length & 15) << 4;
    top.b[15 - nonce_length] |= 1;
    top.b[16 - nonce_length..].copy_from_slice(nonce);

    let bottom = top.b[15] & 0x3f;
    top.b[15] &= 0xc0;

    let mut encrypted = Block16::new();
    cipher(&top.b, &mut encrypted.b);

    let stretch = u64::from_be_bytes(encrypted.b[0..8].try_into().unwrap());
    let stretch = stretch
        ^ ((u64::from_be_bytes(encrypted.b[0..8].try_into().unwrap()) >> 8)
        | ((u64::from_be_bytes(encrypted.b[8..16].try_into().unwrap()) << 56);

    ctx.initial.b[0..8].copy_from_slice(&extract(
        u64::from_be_bytes(encrypted.b[0..8].try_into().unwrap()),
        u64::from_be_bytes(encrypted.b[8..16].try_into().unwrap()),
        bottom as usize,
    ).to_be_bytes());
    ctx.initial.b[8..16].copy_from_slice(&extract(
        u64::from_be_bytes(encrypted.b[8..16].try_into().unwrap()),
        stretch,
        bottom as usize,
    ).to_be_bytes());

    ctx.sum = Block16::new();
    ctx.checksum = Block16::new();
    ctx.data_count = 0;
    ctx.message_count = 0;
}

fn pad_block(block: &mut Block16, length: usize, data: &[u8]) {
    block.b[..length].copy_from_slice(data);
    block.b[length] = 0x80;
    block.b[length + 1..].fill(0);
}

fn update_offset(key: &OcbKey, offset: &mut Block16, i: usize) {
    if i & 1 != 0 {
        offset.xor(&key.l[2]);
    } else {
        assert!(i > 0);
        let mut diff = Block16::new();
        block16_mulx_be(&mut diff, &key.l[2]);
        let mut i = i >> 1;
        while i & 1 == 0 {
            block16_mulx_be(&mut diff, &diff);
            i >>= 1;
        }
        offset.xor(&diff);
    }
}

fn ocb_fill_n(
    key: &OcbKey,
    offset: &mut Block16,
    count: usize,
    n: usize,
    o: &mut [Block16],
) {
    assert!(n > 0);
    let mut prev = if count & 1 != 0 {
        offset
    } else {
        count += 1;
        offset.xor(&key.l[2]);
        o[0].set(offset);
        &mut o[0]
    };

    let mut i = 0;
    while i + 2 <= n {
        let count = count + 2;
        block16_mulx_be(&mut o[i], &key.l[2]);
        let mut j = count >> 1;
        while j & 1 == 0 {
            block16_mulx_be(&mut o[i], &o[i]);
            j >>= 1;
        }
        o[i].xor(prev);
        o[i + 1].xor3(&o[i], &key.l[2]);
        prev = &mut o[i + 1];
        i += 2;
    }

    offset.set(prev);

    if i < n {
        update_offset(key, offset, count + 1);
        o[i].set(offset);
    }
}

fn ocb_update(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    length: usize,
    data: &[u8],
) {
    assert!(ctx.message_count == 0);

    if ctx.data_count == 0 {
        ctx.offset = Block16::new();
    }

    let mut n = length / OCB_BLOCK_SIZE;
    let mut data = data;

    while n > 0 {
        let blocks = if n <= OCB_MAX_BLOCKS {
            n
        } else {
            OCB_MAX_BLOCKS - 1 + (ctx.data_count & 1)
        };

        let mut block = vec![Block16::new(); blocks];
        ocb_fill_n(key, &mut ctx.offset, ctx.data_count, blocks, &mut block);
        ctx.data_count += blocks;

        let size = blocks * OCB_BLOCK_SIZE;
        xor_blocks(&mut block[0].b, data, size);
        cipher(&block[0].b, &mut block[0].b);
        for b in &block {
            ctx.sum.xor(b);
        }

        n -= blocks;
        data = &data[size..];
    }

    let remaining = length % OCB_BLOCK_SIZE;
    if remaining > 0 {
        let mut block = Block16::new();
        pad_block(&mut block, remaining, data);
        ctx.offset.xor(&key.l[0]);
        block.xor(&ctx.offset);
        cipher(&block.b, &mut block.b);
        ctx.sum.xor(&block);
    }
}

fn xor_blocks(dst: &mut [u8], src: &[u8], len: usize) {
    for i in 0..len {
        dst[i] ^= src[i];
    }
}

fn memxor3(dst: &mut [u8], a: &[u8], b: &[u8], len: usize) {
    for i in 0..len {
        dst[i] = a[i] ^ b[i];
    }
}

fn ocb_crypt_n(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    n: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut src = src;
    let mut dst = dst;
    let mut remaining = n;

    while remaining > 0 {
        let blocks = if remaining <= OCB_MAX_BLOCKS {
            remaining
        } else {
            OCB_MAX_BLOCKS - 1 + (ctx.message_count & 1)
        };

        let mut o = vec![Block16::new(); blocks];
        let mut block = vec![Block16::new(); blocks];
        ocb_fill_n(key, &mut ctx.offset, ctx.message_count, blocks, &mut o);
        ctx.message_count += blocks;

        let size = blocks * OCB_BLOCK_SIZE;
        memxor3(&mut block[0].b, &o[0].b, src, size);
        cipher(&block[0].b, &mut block[0].b);
        memxor3(dst, &block[0].b, &o[0].b, size);

        remaining -= blocks;
        src = &src[size..];
        dst = &mut dst[size..];
    }
}

fn ocb_checksum_n(checksum: &mut Block16, n: usize, src: &[u8]) {
    if n == 1 {
        xor_blocks(&mut checksum.b, src, OCB_BLOCK_SIZE);
        return;
    }

    let mut s0 = 0u64;
    let mut s1 = 0u64;
    let mut edge_word = 0u64;

    let initial = (!(src.as_ptr() as usize) & 7) as usize;
    let mut src = src;

    if initial > 0 {
        for i in 0..initial {
            edge_word = (edge_word << 8) | src[i] as u64;
        }
        src = &src[initial..];
    }

    for _ in 0..n {
        s0 ^= u64::from_be_bytes(src[0..8].try_into().unwrap());
        s1 ^= u64::from_be_bytes(src[8..16].try_into().unwrap());
        src = &src[OCB_BLOCK_SIZE..];
    }

    if initial > 0 {
        s0 ^= u64::from_be_bytes(src[0..8].try_into().unwrap());
        for i in 0..8 - initial {
            edge_word = (edge_word << 8) | src[8 + i] as u64;
        }

        let rotate = initial * 8;
        s0 = (s0 >> rotate) | (s1 << (64 - rotate));
        s1 = (s1 >> rotate) | (s0 << (64 - rotate));

        let mask = (1u64 << (initial * 8)) - 1;
        edge_word = edge_word.to_be();
        s0 ^= edge_word & mask;
        s1 ^= edge_word & !mask;
    }

    checksum.b[0..8].copy_from_slice(&(checksum.u64[0] ^ s0).to_be_bytes());
    checksum.b[8..16].copy_from_slice(&(checksum.u64[1] ^ s1).to_be_bytes());
}

fn ocb_encrypt(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let n = length / OCB_BLOCK_SIZE;

    if ctx.message_count == 0 {
        ctx.offset = ctx.initial;
    }

    if n > 0 {
        ocb_checksum_n(&mut ctx.checksum, n, src);
        ocb_crypt_n(ctx, key, cipher, n, dst, src);
    }

    let remaining = length % OCB_BLOCK_SIZE;
    if remaining > 0 {
        let mut block = Block16::new();
        pad_block(&mut block, remaining, &src[n * OCB_BLOCK_SIZE..]);
        ctx.checksum.xor(&block);

        ctx.offset.xor(&key.l[0]);
        cipher(&ctx.offset.b, &mut block.b);
        memxor3(
            &mut dst[n * OCB_BLOCK_SIZE..],
            &block.b,
            &src[n * OCB_BLOCK_SIZE..],
            remaining,
        );
        ctx.message_count += 1;
    }
}

fn ocb_decrypt(
    ctx: &mut OcbCtx,
    key: &OcbKey,
    encrypt_cipher: &dyn Fn(&[u8], &mut [u8]),
    decrypt_cipher: &dyn Fn(&[u8], &mut [u8]),
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let n = length / OCB_BLOCK_SIZE;

    if ctx.message_count == 0 {
        ctx.offset = ctx.initial;
    }

    if n > 0 {
        ocb_crypt_n(ctx, key, decrypt_cipher, n, dst, src);
        ocb_checksum_n(&mut ctx.checksum, n, dst);
    }

    let remaining = length % OCB_BLOCK_SIZE;
    if remaining > 0 {
        let mut block = Block16::new();
        ctx.offset.xor(&key.l[0]);
        encrypt_cipher(&ctx.offset.b, &mut block.b);
        memxor3(
            &mut dst[n * OCB_BLOCK_SIZE..],
            &block.b,
            &src[n * OCB_BLOCK_SIZE..],
            remaining,
        );

        pad_block(&mut block, remaining, &dst[n * OCB_BLOCK_SIZE..]);
        ctx.checksum.xor(&block);
        ctx.message_count += 1;
    }
}

fn ocb_digest(
    ctx: &OcbCtx,
    key: &OcbKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    length: usize,
    digest: &mut [u8],
) {
    assert!(length <= OCB_DIGEST_SIZE);
    let mut block = Block16::new();
    block.xor3(
        &key.l[1],
        if ctx.message_count > 0 {
            &ctx.offset
        } else {
            &ctx.initial
        },
    );
    block.xor(&ctx.checksum);
    cipher(&block.b, &mut block.b);
    memxor3(digest, &block.b, &ctx.sum.b, length);
}

fn ocb_encrypt_message(
    key: &OcbKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    tlength: usize,
    clength: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(clength >= tlength);
    let mut ctx = OcbCtx::new();
    ocb_set_nonce(&mut ctx, cipher, tlength, nlength, nonce);
    ocb_update(&mut ctx, key, cipher, alength, adata);
    ocb_encrypt(
        &mut ctx,
        key,
        cipher,
        clength - tlength,
        dst,
        src,
    );
    ocb_digest(
        &ctx,
        key,
        cipher,
        tlength,
        &mut dst[clength - tlength..],
    );
}

fn ocb_decrypt_message(
    key: &OcbKey,
    encrypt_cipher: &dyn Fn(&[u8], &mut [u8]),
    decrypt_cipher: &dyn Fn(&[u8], &mut [u8]),
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    tlength: usize,
    mlength: usize,
    dst: &mut [u8],
    src: &[u8],
) -> bool {
    let mut ctx = OcbCtx::new();
    ocb_set_nonce(&mut ctx, encrypt_cipher, tlength, nlength, nonce);
    ocb_update(&mut ctx, key, encrypt_cipher, alength, adata);
    ocb_decrypt(
        &mut ctx,
        key,
        encrypt_cipher,
        decrypt_cipher,
        mlength,
        dst,
        src,
    );
    let mut digest = Block16::new();
    ocb_digest(&ctx, key, encrypt_cipher, tlength, &mut digest.b);
    digest.b[..t