// camellia-absorb.rs

/* Final key setup processing for the camellia block cipher.

   Based on the original C code:
   Copyright (C) 2006,2007 NTT
   (Nippon Telegraph and Telephone Corporation).
   Copyright (C) 2010 Niels Möller

   Translated to Rust following the same licensing terms:
     * GNU Lesser General Public License (LGPL) version 3 or later
     * GNU General Public License (GPL) version 2 or later
     Or both in parallel
*/

use std::num::Wrapping;

/// Rotates a 32-bit value left by specified number of bits
fn rotl32(bits: u32, value: u32) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

/// Performs the Camellia key absorption as described in the specification
pub fn camellia_absorb(nkeys: usize, dst: &mut [u64], subkey: &[u64]) {
    let mut kw2;
    let mut kw4;
    let mut dw;
    let mut tl;
    let mut tr;
    let mut i;

    /* At this point, the subkey array contains the subkeys as described
       in the spec, 26 for short keys and 34 for large keys. */

    /* absorb kw2 to other subkeys */
    kw2 = subkey[1];

    let mut subkey = subkey.to_vec();
    subkey[3] ^= kw2;
    subkey[5] ^= kw2;
    subkey[7] ^= kw2;
    
    i = 8;
    while i < nkeys {
        /* FIXME: gcc for x86_32 is smart enough to fetch the 32 low bits
           and xor the result into the 32 high bits, but it still generates
           worse code than for explicit 32-bit operations. */
        kw2 ^= (kw2 & !subkey[i+1]) << 32;
        dw = ((kw2 & subkey[i+1]) >> 32) as u32;
        kw2 ^= u64::from(rotl32(1, dw));

        subkey[i+3] ^= kw2;
        subkey[i+5] ^= kw2;
        subkey[i+7] ^= kw2;
        i += 8;
    }
    subkey[i] ^= kw2;
    
    /* absorb kw4 to other subkeys */  
    kw4 = subkey[nkeys + 1];

    i = nkeys - 8;
    while i > 0 {
        subkey[i+6] ^= kw4;
        subkey[i+4] ^= kw4;
        subkey[i+2] ^= kw4;
        kw4 ^= (kw4 & !subkey[i]) << 32;
        dw = ((kw4 & subkey[i]) >> 32) as u32;
        kw4 ^= u64::from(rotl32(1, dw));
        i -= 8;
    }

    subkey[6] ^= kw4;
    subkey[4] ^= kw4;
    subkey[2] ^= kw4;
    subkey[0] ^= kw4;

    /* key XOR is end of F-function */
    dst[0] = subkey[0] ^ subkey[2];
    dst[1] = subkey[3];

    dst[2] = subkey[2] ^ subkey[4];
    dst[3] = subkey[3] ^ subkey[5];
    dst[4] = subkey[4] ^ subkey[6];
    dst[5] = subkey[5] ^ subkey[7];

    i = 8;
    while i < nkeys {
        tl = ((subkey[i+2] >> 32) ^ (subkey[i+2] & !subkey[i])) as u32;
        dw = (tl & (subkey[i] >> 32) as u32) as u32;
        tr = (subkey[i+2] ^ u64::from(rotl32(1, dw))) as u32;
        dst[i-2] = subkey[i-2] ^ ((u64::from(tl) << 32) | u64::from(tr);

        dst[i-1] = subkey[i];
        dst[i] = subkey[i+1];

        tl = ((subkey[i-1] >> 32) ^ (subkey[i-1] & !subkey[i+1])) as u32;
        dw = (tl & (subkey[i+1] >> 32) as u32) as u32;
        tr = (subkey[i-1] ^ u64::from(rotl32(1, dw))) as u32;
        dst[i+1] = subkey[i+3] ^ ((u64::from(tl) << 32) | u64::from(tr));

        dst[i+2] = subkey[i+2] ^ subkey[i+4];
        dst[i+3] = subkey[i+3] ^ subkey[i+5];
        dst[i+4] = subkey[i+4] ^ subkey[i+6];
        dst[i+5] = subkey[i+5] ^ subkey[i+7];
        i += 8;
    }
    dst[i-2] = subkey[i-2];
    dst[i-1] = subkey[i] ^ subkey[i-1];

    // 32-bit implementation would go here
    // Note: The CAMELLIA_F_HALF_INV macros are not translated as they
    // were conditionally compiled for !HAVE_NATIVE_64_BIT systems
    // and their implementation isn't shown in the original code
}