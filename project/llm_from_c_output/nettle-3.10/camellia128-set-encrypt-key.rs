/* 
 * Key setup for the camellia block cipher.
 * 
 * Translated from C to Rust while maintaining functionality and safety.
 */

use std::convert::TryInto;

// Constants
const SIGMA1: u64 = 0xA09E667F_3BCC908B;
const SIGMA2: u64 = 0xB67AE858_4CAA73B2;
const SIGMA3: u64 = 0xC6EF372F_E94F82BE;
const SIGMA4: u64 = 0x54FF53A5_F1D36F1C;
const _CAMELLIA128_NKEYS: usize = 26;

/// Rotates left a 128-bit value stored in two u64 registers
#[inline(always)]
fn rotl128(shift: u32, hi: &mut u64, lo: &mut u64) {
    if shift == 0 {
        return;
    }
    let tmp = *hi >> (64 - shift);
    *hi = (*hi << shift) | (*lo >> (64 - shift));
    *lo = (*lo << shift) | tmp;
}

/// Camellia F function
#[inline(always)]
fn camellia_f(x: u64, k: u64, out: &mut u64) {
    let y = x ^ k;
    let z = (y >> 32) ^ (y & 0xFFFFFFFF);
    *out = (y << 32) ^ (z << 16) ^ (z << 1) ^ (z >> 15);
}

/// Context structure for Camellia-128
pub struct Camellia128Ctx {
    pub keys: [u64; _CAMELLIA128_NKEYS],
}

/// Absorbs subkeys into the context
fn camellia_absorb(nkeys: usize, ctx_keys: &mut [u64], subkey: &[u64]) {
    assert!(nkeys <= ctx_keys.len());
    assert!(subkey.len() >= nkeys);
    ctx_keys[..nkeys].copy_from_slice(&subkey[..nkeys]);
}

/// Sets the encryption key for Camellia-128
pub fn camellia128_set_encrypt_key(ctx: &mut Camellia128Ctx, key: &[u8]) {
    assert!(key.len() >= 16);

    let k0 = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let mut k1 = u64::from_be_bytes(key[8..16].try_into().unwrap());

    let mut subkey = [0u64; _CAMELLIA128_NKEYS + 2];
    let mut w;

    // Generate KL dependent subkeys
    subkey[0] = k0;
    subkey[1] = k1;
    let mut k0 = k0;
    let mut k1 = k1;
    rotl128(15, &mut k0, &mut k1);
    subkey[4] = k0;
    subkey[5] = k1;
    rotl128(30, &mut k0, &mut k1);
    subkey[10] = k0;
    subkey[11] = k1;
    rotl128(15, &mut k0, &mut k1);
    subkey[13] = k1;
    rotl128(17, &mut k0, &mut k1);
    subkey[16] = k0;
    subkey[17] = k1;
    rotl128(17, &mut k0, &mut k1);
    subkey[18] = k0;
    subkey[19] = k1;
    rotl128(17, &mut k0, &mut k1);
    subkey[22] = k0;
    subkey[23] = k1;

    // Generate KA
    k0 = subkey[0];
    w = subkey[1];
    let mut temp;
    camellia_f(k0, SIGMA1, &mut temp);
    k1 = temp;
    w ^= k1;
    camellia_f(w, SIGMA2, &mut temp);
    k0 = temp;
    camellia_f(k0, SIGMA3, &mut temp);
    w = temp;
    k1 ^= w;
    camellia_f(k1, SIGMA4, &mut temp);
    w = temp;
    k0 ^= w;

    // Generate KA dependent subkeys
    subkey[2] = k0;
    subkey[3] = k1;
    rotl128(15, &mut k0, &mut k1);
    subkey[6] = k0;
    subkey[7] = k1;
    rotl128(15, &mut k0, &mut k1);
    subkey[8] = k0;
    subkey[9] = k1;
    rotl128(15, &mut k0, &mut k1);
    subkey[12] = k0;
    rotl128(15, &mut k0, &mut k1);
    subkey[14] = k0;
    subkey[15] = k1;
    rotl128(34, &mut k0, &mut k1);
    subkey[20] = k0;
    subkey[21] = k1;
    rotl128(17, &mut k0, &mut k1);
    subkey[24] = k0;
    subkey[25] = k1;

    // Common final processing
    camellia_absorb(_CAMELLIA128_NKEYS, &mut ctx.keys, &subkey);
}