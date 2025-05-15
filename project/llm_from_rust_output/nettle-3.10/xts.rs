use std::mem::MaybeUninit;

const XTS_BLOCK_SIZE: usize = 16;

#[derive(Copy, Clone)]
#[repr(C)]
pub union Block16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

impl Block16 {
    pub fn new() -> Self {
        Block16 { b: [0; 16] }
    }
}

pub type CipherFunc = fn(ctx: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

fn block16_mulx_le(dst: &mut Block16, src: &Block16) {
    unsafe {
        let carry = src.u64_0[1] >> 63;
        dst.u64_0[1] = (src.u64_0[1] << 1) | (src.u64_0[0] >> 63);
        dst.u64_0[0] = (src.u64_0[0] << 1) ^ (0x87 & carry.wrapping_neg());
    }
}

fn check_length(length: usize, dst: &mut [u8]) {
    assert!(length >= XTS_BLOCK_SIZE, "length >= XTS_BLOCK_SIZE");
    if length < XTS_BLOCK_SIZE {
        dst[..length].fill(0);
    }
}

pub fn xts_encrypt_message(
    enc_ctx: &[u8],
    twk_ctx: &[u8],
    encf: CipherFunc,
    tweak: &[u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(tweak.len() >= XTS_BLOCK_SIZE);
    assert!(dst.len() >= length);
    assert!(src.len() >= length);

    let mut T = Block16::new();
    let mut P = Block16::new();
    check_length(length, dst);

    encf(twk_ctx, XTS_BLOCK_SIZE, unsafe { T.b.as_mut() }, tweak);

    let mut remaining = length;
    let mut src_pos = 0;
    let mut dst_pos = 0;

    while remaining >= 2 * XTS_BLOCK_SIZE || remaining == XTS_BLOCK_SIZE {
        unsafe {
            xor3(
                P.b.as_mut_ptr(),
                src.as_ptr().add(src_pos),
                T.b.as_ptr(),
                XTS_BLOCK_SIZE,
            );
        }

        encf(
            enc_ctx,
            XTS_BLOCK_SIZE,
            &mut dst[dst_pos..dst_pos + XTS_BLOCK_SIZE],
            unsafe { P.b.as_ref() },
        );

        unsafe {
            xor(
                dst.as_mut_ptr().add(dst_pos),
                T.b.as_ptr(),
                XTS_BLOCK_SIZE,
            );
        }

        if remaining > XTS_BLOCK_SIZE {
            block16_mulx_le(&mut T, &T);
        }

        remaining -= XTS_BLOCK_SIZE;
        src_pos += XTS_BLOCK_SIZE;
        dst_pos += XTS_BLOCK_SIZE;
    }

    if remaining != 0 {
        let mut S = Block16::new();
        unsafe {
            xor3(
                P.b.as_mut_ptr(),
                src.as_ptr().add(src_pos),
                T.b.as_ptr(),
                XTS_BLOCK_SIZE,
            );
        }

        encf(enc_ctx, XTS_BLOCK_SIZE, unsafe { S.b.as_mut() }, unsafe {
            P.b.as_ref()
        });

        unsafe {
            xor(S.b.as_mut_ptr(), T.b.as_ptr(), XTS_BLOCK_SIZE);
            block16_mulx_le(&mut T, &T);
        }

        let partial_len = remaining - XTS_BLOCK_SIZE;
        src_pos += XTS_BLOCK_SIZE;

        unsafe {
            xor3(
                P.b.as_mut_ptr(),
                src.as_ptr().add(src_pos),
                T.b.as_ptr(),
                partial_len,
            );
            xor3(
                P.b.as_mut_ptr().add(partial_len),
                S.b.as_ptr().add(partial_len),
                T.b.as_ptr().add(partial_len),
                XTS_BLOCK_SIZE - partial_len,
            );
        }

        encf(
            enc_ctx,
            XTS_BLOCK_SIZE,
            &mut dst[dst_pos..dst_pos + XTS_BLOCK_SIZE],
            unsafe { P.b.as_ref() },
        );

        unsafe {
            xor(
                dst.as_mut_ptr().add(dst_pos),
                T.b.as_ptr(),
                XTS_BLOCK_SIZE,
            );
        }

        dst_pos += XTS_BLOCK_SIZE;
        dst[dst_pos..dst_pos + partial_len]
            .copy_from_slice(&unsafe { S.b.as_ref() }[..partial_len]);
    }
}

// Similar implementation for xts_decrypt_message would follow
// Helper functions for XOR operations would be implemented safely

#[inline]
fn xor(dst: *mut u8, src: *const u8, len: usize) {
    for i in 0..len {
        unsafe {
            *dst.add(i) ^= *src.add(i);
        }
    }
}

#[inline]
fn xor3(dst: *mut u8, a: *const u8, b: *const u8, len: usize) {
    for i in 0..len {
        unsafe {
            *dst.add(i) = *a.add(i) ^ *b.add(i);
        }
    }
}