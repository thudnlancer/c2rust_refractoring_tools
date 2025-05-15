use std::mem;
use std::convert::TryInto;
use std::num::Wrapping;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_block16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_block8 {
    pub b: [u8; 8],
    pub u64_0: u64,
}

#[derive(Copy, Clone)]
pub struct aes128_ctx {
    pub keys: [u32; 44],
}

#[derive(Copy, Clone)]
pub struct aes192_ctx {
    pub keys: [u32; 52],
}

#[derive(Copy, Clone)]
pub struct aes256_ctx {
    pub keys: [u32; 60],
}

pub type nettle_cipher_func = fn(&[u32], usize, &mut [u8], &[u8]);

fn nettle_nist_keywrap16(
    ctx: &[u32],
    encrypt: nettle_cipher_func,
    iv: &[u8],
    ciphertext: &mut [u8],
    cleartext: &[u8],
) {
    assert!(ciphertext.len() >= 16);
    assert!(ciphertext.len() % 8 == 0);

    let n = (ciphertext.len() - 8) / 8;
    let (a_bytes, r_bytes) = ciphertext.split_at_mut(8);
    
    r_bytes.copy_from_slice(&cleartext[..r_bytes.len()]);
    a_bytes.copy_from_slice(&iv[..8]);

    let mut a = nettle_block8 { b: [0; 8] };
    a.b.copy_from_slice(a_bytes);

    for j in 0..6 {
        for i in 0..n {
            let mut i_block = nettle_block16 { b: [0; 16] };
            i_block.u64_0[0] = a.u64_0;
            i_block.b[8..].copy_from_slice(&r_bytes[i*8..(i+1)*8]);

            let mut b_block = nettle_block16 { b: [0; 16] };
            encrypt(ctx, 16, &mut b_block.b, &i_block.b);

            a.u64_0 = b_block.u64_0[0] ^ ((n * j + i + 1) as u64).swap_bytes();
            r_bytes[i*8..(i+1)*8].copy_from_slice(&b_block.b[8..]);
        }
    }

    a_bytes.copy_from_slice(&a.b);
}

fn nettle_nist_keyunwrap16(
    ctx: &[u32],
    decrypt: nettle_cipher_func,
    iv: &[u8],
    cleartext: &mut [u8],
    ciphertext: &[u8],
) -> bool {
    assert!(cleartext.len() >= 8);
    assert!(cleartext.len() % 8 == 0);

    let n = cleartext.len() / 8;
    let (a_bytes, r_bytes) = ciphertext.split_at(8);
    
    let mut a = nettle_block8 { b: [0; 8] };
    a.b.copy_from_slice(a_bytes);
    cleartext.copy_from_slice(r_bytes);

    for j in (0..6).rev() {
        for i in (0..n).rev() {
            let mut i_block = nettle_block16 { b: [0; 16] };
            i_block.u64_0[0] = a.u64_0 ^ ((n * j + i + 1) as u64).swap_bytes();
            i_block.b[8..].copy_from_slice(&cleartext[i*8..(i+1)*8]);

            let mut b_block = nettle_block16 { b: [0; 16] };
            decrypt(ctx, 16, &mut b_block.b, &i_block.b);

            a.u64_0 = b_block.u64_0[0];
            cleartext[i*8..(i+1)*8].copy_from_slice(&b_block.b[8..]);
        }
    }

    a.b == iv[..8]
}

pub fn nettle_aes128_keywrap(
    ctx: &aes128_ctx,
    iv: &[u8],
    ciphertext: &mut [u8],
    cleartext: &[u8],
) {
    nettle_nist_keywrap16(
        &ctx.keys,
        |keys, len, dst, src| {
            let mut ctx = aes128_ctx { keys: [0; 44] };
            ctx.keys.copy_from_slice(keys);
            unsafe { nettle_aes128_encrypt(&ctx, len, dst.as_mut_ptr(), src.as_ptr()) }
        },
        iv,
        ciphertext,
        cleartext,
    );
}

pub fn nettle_aes192_keywrap(
    ctx: &aes192_ctx,
    iv: &[u8],
    ciphertext: &mut [u8],
    cleartext: &[u8],
) {
    nettle_nist_keywrap16(
        &ctx.keys,
        |keys, len, dst, src| {
            let mut ctx = aes192_ctx { keys: [0; 52] };
            ctx.keys.copy_from_slice(keys);
            unsafe { nettle_aes192_encrypt(&ctx, len, dst.as_mut_ptr(), src.as_ptr()) }
        },
        iv,
        ciphertext,
        cleartext,
    );
}

pub fn nettle_aes256_keywrap(
    ctx: &aes256_ctx,
    iv: &[u8],
    ciphertext: &mut [u8],
    cleartext: &[u8],
) {
    nettle_nist_keywrap16(
        &ctx.keys,
        |keys, len, dst, src| {
            let mut ctx = aes256_ctx { keys: [0; 60] };
            ctx.keys.copy_from_slice(keys);
            unsafe { nettle_aes256_encrypt(&ctx, len, dst.as_mut_ptr(), src.as_ptr()) }
        },
        iv,
        ciphertext,
        cleartext,
    );
}

pub fn nettle_aes128_keyunwrap(
    ctx: &aes128_ctx,
    iv: &[u8],
    cleartext: &mut [u8],
    ciphertext: &[u8],
) -> bool {
    nettle_nist_keyunwrap16(
        &ctx.keys,
        |keys, len, dst, src| {
            let mut ctx = aes128_ctx { keys: [0; 44] };
            ctx.keys.copy_from_slice(keys);
            unsafe { nettle_aes128_decrypt(&ctx, len, dst.as_mut_ptr(), src.as_ptr()) }
        },
        iv,
        cleartext,
        ciphertext,
    )
}

pub fn nettle_aes192_keyunwrap(
    ctx: &aes192_ctx,
    iv: &[u8],
    cleartext: &mut [u8],
    ciphertext: &[u8],
) -> bool {
    nettle_nist_keyunwrap16(
        &ctx.keys,
        |keys, len, dst, src| {
            let mut ctx = aes192_ctx { keys: [0; 52] };
            ctx.keys.copy_from_slice(keys);
            unsafe { nettle_aes192_decrypt(&ctx, len, dst.as_mut_ptr(), src.as_ptr()) }
        },
        iv,
        cleartext,
        ciphertext,
    )
}

pub fn nettle_aes256_keyunwrap(
    ctx: &aes256_ctx,
    iv: &[u8],
    cleartext: &mut [u8],
    ciphertext: &[u8],
) -> bool {
    nettle_nist_keyunwrap16(
        &ctx.keys,
        |keys, len, dst, src| {
            let mut ctx = aes256_ctx { keys: [0; 60] };
            ctx.keys.copy_from_slice(keys);
            unsafe { nettle_aes256_decrypt(&ctx, len, dst.as_mut_ptr(), src.as_ptr()) }
        },
        iv,
        cleartext,
        ciphertext,
    )
}