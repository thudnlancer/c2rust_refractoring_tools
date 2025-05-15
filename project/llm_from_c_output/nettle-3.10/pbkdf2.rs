// pbkdf2.rs

// PKCS #5 password-based key derivation function PBKDF2, see RFC 2898.

// Original Copyright (C) 2012 Simon Josefsson, Niels Möller
// Rust translation maintains original functionality while following Rust safety practices

use std::cmp::min;
use std::convert::TryInto;

type HashUpdateFn = fn(&mut dyn MacContext, &[u8]);
type HashDigestFn = fn(&mut dyn MacContext, &mut [u8]);

pub trait MacContext {
    fn update(&mut self, data: &[u8]);
    fn digest(&mut self, output: &mut [u8]);
}

pub fn pbkdf2(
    mac_ctx: &mut dyn MacContext,
    update: HashUpdateFn,
    digest: HashDigestFn,
    digest_size: usize,
    iterations: u32,
    salt: &[u8],
    length: usize,
    dst: &mut [u8],
) {
    assert!(iterations > 0);

    if length == 0 {
        return;
    }

    let mut i = 1u32;
    let mut remaining_length = length;
    let mut dst_pos = 0;

    while remaining_length > 0 {
        let mut t = vec![0u8; digest_size];
        let mut u = vec![0u8; digest_size];

        // First iteration
        let counter_bytes = i.to_be_bytes();
        mac_ctx.update(salt);
        mac_ctx.update(&counter_bytes);
        digest(mac_ctx, &mut t);

        // Subsequent iterations
        for _ in 1..iterations {
            mac_ctx.update(&t);
            digest(mac_ctx, &mut u);

            xor_bytes(&mut t, &u);
        }

        let copy_size = min(remaining_length, digest_size);
        dst[dst_pos..dst_pos + copy_size].copy_from_slice(&t[..copy_size]);
        
        dst_pos += copy_size;
        remaining_length -= copy_size;
        i += 1;
    }
}

fn xor_bytes(a: &mut [u8], b: &[u8]) {
    assert_eq!(a.len(), b.len());
    for (a_byte, b_byte) in a.iter_mut().zip(b) {
        *a_byte ^= *b_byte;
    }
}

// HMAC variants would be implemented as separate functions using specific hash implementations
// For example:
/*
pub fn pbkdf2_hmac_sha1(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    length: usize,
    dst: &mut [u8],
) {
    let mut hmac = HmacSha1::new_from_slice(key).unwrap();
    pbkdf2(
        &mut hmac,
        HmacSha1::update,
        HmacSha1::digest,
        sha1::DIGEST_LENGTH,
        iterations,
        salt,
        length,
        dst,
    );
}
*/