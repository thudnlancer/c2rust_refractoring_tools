/* 
 * Camellia-256 key setup in Rust
 * Translated from C code with equivalent functionality
 */

use std::convert::TryInto;

// Constants
const SIGMA1: u64 = 0xA09E667F3BCC908B;
const SIGMA2: u64 = 0xB67AE8584CAA73B2;
const SIGMA3: u64 = 0xC6EF372FE94F82BE;
const SIGMA4: u64 = 0x54FF53A5F1D36F1C;
const SIGMA5: u64 = 0x10E527FADE682D1D;
const SIGMA6: u64 = 0xB05688C2B3E6C1FD;

const _CAMELLIA256_NKEYS: usize = 34;

/// Camellia 256-bit context
pub struct Camellia256Ctx {
    keys: [u64; _CAMELLIA256_NKEYS],
}

impl Camellia256Ctx {
    pub fn new() -> Self {
        Self {
            keys: [0; _CAMELLIA256_NKEYS],
        }
    }
}

/// Rotate left 128-bit value (pair of u64) by n bits
fn rotl128(n: u32, x0: &mut u64, x1: &mut u64) {
    let n = n % 128;
    if n == 0 {
        return;
    } else if n < 64 {
        let t0 = (*x0 << n) | (*x1 >> (64 - n));
        let t1 = (*x1 << n) | (*x0 >> (64 - n));
        *x0 = t0;
        *x1 = t1;
    } else {
        let m = n - 64;
        let t0 = (*x1 << m) | (*x0 >> (64 - m));
        let t1 = (*x0 << m) | (*x1 >> (64 - m));
        *x0 = t0;
        *x1 = t1;
    }
}

/// Camellia F function
fn camellia_f(x: u64, k: u64) -> u64 {
    let z = x ^ k;
    let y1 = z.rotate_left(1);
    let y2 = z.rotate_left(7);
    let y3 = z >> 63;
    let y4 = z >> 56;
    let y5 = (y3 & z) ^ y1;
    let y6 = (y4 & y2) ^ z;
    y5 ^ y6
}

/// Absorb subkeys into context
fn camellia_absorb(nkeys: usize, ctx_keys: &mut [u64], subkey: &[u64]) {
    for i in 0..nkeys {
        ctx_keys[i] = subkey[i];
    }
}

fn camellia256_set_encrypt_key_internal(ctx: &mut Camellia256Ctx, k0: u64, k1: u64, k2: u64, k3: u64) {
    let mut subkey = [0u64; _CAMELLIA256_NKEYS + 2];
    let mut w;
    
    /* generate KL dependent subkeys */
    subkey[0] = k0; subkey[1] = k1;
    {
        let mut k0 = k0;
        let mut k1 = k1;
        rotl128(45, &mut k0, &mut k1);
        subkey[12] = k0; subkey[13] = k1;
        rotl128(15, &mut k0, &mut k1);
        subkey[16] = k0; subkey[17] = k1;
        rotl128(17, &mut k0, &mut k1);
        subkey[22] = k0; subkey[23] = k1;
        rotl128(34, &mut k0, &mut k1);
        subkey[30] = k0; subkey[31] = k1;
    }

    /* generate KR dependent subkeys */
    {
        let mut k2 = k2;
        let mut k3 = k3;
        rotl128(15, &mut k2, &mut k3);
        subkey[4] = k2; subkey[5] = k3;
        rotl128(15, &mut k2, &mut k3);
        subkey[8] = k2; subkey[9] = k3;
        rotl128(30, &mut k2, &mut k3);
        subkey[18] = k2; subkey[19] = k3;
        rotl128(34, &mut k2, &mut k3);
        subkey[26] = k2; subkey[27] = k3;
        rotl128(34, &mut k2, &mut k3);
    }

    /* generate KA */
    let mut k0 = subkey[0] ^ k2;
    let mut k1 = subkey[1] ^ k3;

    w = camellia_f(k0, SIGMA1);
    k1 ^= w;

    k0 = camellia_f(k1, SIGMA2) ^ k2;

    w = camellia_f(k0, SIGMA3);
    k1 ^= w ^ k3;

    w = camellia_f(k1, SIGMA4);
    k0 ^= w;

    /* generate KB */
    let mut k2 = k2 ^ k0;
    let mut k3 = k3 ^ k1;
    w = camellia_f(k2, SIGMA5);
    k3 ^= w;
    w = camellia_f(k3, SIGMA6);
    k2 ^= w;

    /* generate KA dependent subkeys */
    {
        rotl128(15, &mut k0, &mut k1);
        subkey[6] = k0; subkey[7] = k1;
        rotl128(30, &mut k0, &mut k1);
        subkey[14] = k0; subkey[15] = k1;
        rotl128(32, &mut k0, &mut k1);
        subkey[24] = k0; subkey[25] = k1;
        rotl128(17, &mut k0, &mut k1);
        subkey[28] = k0; subkey[29] = k1;
    }

    /* generate KB dependent subkeys */
    subkey[2] = k2; subkey[3] = k3;
    {
        rotl128(30, &mut k2, &mut k3);
        subkey[10] = k2; subkey[11] = k3;
        rotl128(30, &mut k2, &mut k3);
        subkey[20] = k2; subkey[21] = k3;
        rotl128(51, &mut k2, &mut k3);
        subkey[32] = k2; subkey[33] = k3;
    }

    /* Common final processing */
    camellia_absorb(_CAMELLIA256_NKEYS, &mut ctx.keys, &subkey);
}

/// Set encryption key from 256-bit key
pub fn camellia256_set_encrypt_key(ctx: &mut Camellia256Ctx, key: &[u8; 32]) {
    let k0 = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let k1 = u64::from_be_bytes(key[8..16].try_into().unwrap());
    let k2 = u64::from_be_bytes(key[16..24].try_into().unwrap());
    let k3 = u64::from_be_bytes(key[24..32].try_into().unwrap());

    camellia256_set_encrypt_key_internal(ctx, k0, k1, k2, k3);
}

/// Set encryption key from 192-bit key
pub fn camellia192_set_encrypt_key(ctx: &mut Camellia256Ctx, key: &[u8; 24]) {
    let k0 = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let k1 = u64::from_be_bytes(key[8..16].try_into().unwrap());
    let k2 = u64::from_be_bytes(key[16..24].try_into().unwrap());

    camellia256_set_encrypt_key_internal(ctx, k0, k1, k2, !k2);
}