// umac-l2.rs

use std::mem;

const KEY_MASK: u32 = 0x01ffffff;
const UMAC_P64: u64 = 0xffffffffffffffc5;
const UMAC_P128_HI: u64 = 0xffffffffffffffff;
const UMAC_P128_LO: u64 = 0xffffffffffffffc5;

pub fn bswap32_if_le(w: u32) -> u32 {
    if cfg!(target_endian = "little") {
        w.swap_bytes()
    } else {
        w
    }
}

pub fn _nettle_umac_l2_init(size: usize, k: &mut [u32]) {
    for item in k.iter_mut().take(size) {
        let w = bswap32_if_le(*item);
        *item = w & KEY_MASK;
    }
}

pub fn _nettle_umac_l2(
    key: &[u32],
    state: &mut [u64],
    n: usize,
    count: u64,
    m: &[u64],
) {
    let state_len = state.len();
    let prev = &mut state[2 * n..state_len];
    
    if count == 0 {
        prev[..n].copy_from_slice(&m[..n]);
    } else if count == 1 {
        for i in 0..n {
            let key_idx = i * 6;
            let y = _nettle_umac_poly64(key[key_idx], key[key_idx + 1], 1, prev[i]);
            state[2 * i + 1] = _nettle_umac_poly64(key[key_idx], key[key_idx + 1], y, m[i]);
        }
    } else if count < UMAC_POLY64_BLOCKS {
        for i in 0..n {
            let key_idx = i * 6;
            state[2 * i + 1] = _nettle_umac_poly64(key[key_idx], key[key_idx + 1], state[2 * i + 1], m[i]);
        }
    } else if count % 2 == 0 {
        if count == UMAC_POLY64_BLOCKS {
            for i in 0..n {
                let key_idx = 2 + i * 6;
                let y = state[2 * i + 1];
                let y = if y >= UMAC_P64 { y - UMAC_P64 } else { y };
                state[2 * i] = 0;
                state[2 * i + 1] = 1;
                
                _nettle_umac_poly128(&key[key_idx..], &mut state[2 * i..2 * i + 2], 0, y);
            }
        }
        prev[..n].copy_from_slice(&m[..n]);
    } else {
        for i in 0..n {
            let key_idx = 2 + i * 6;
            _nettle_umac_poly128(&key[key_idx..], &mut state[2 * i..2 * i + 2], prev[i], m[i]);
        }
    }
}

pub fn _nettle_umac_l2_final(
    key: &[u32],
    state: &mut [u64],
    n: usize,
    count: u64,
) {
    let state_len = state.len();
    let prev = &mut state[2 * n..state_len];
    
    assert!(count > 0);
    if count == 1 {
        for i in 0..n {
            state[2 * i] = 0;
            state[2 * i + 1] = prev[i];
        }
    } else if count <= UMAC_POLY64_BLOCKS {
        for i in 0..n {
            state[2 * i] = 0;
            let y = state[2 * i + 1];
            state[2 * i + 1] = if y >= UMAC_P64 { y - UMAC_P64 } else { y };
        }
    } else {
        let pad = 1u64 << 63;
        if count % 2 == 1 {
            for i in 0..n {
                let key_idx = 2 + i * 6;
                _nettle_umac_poly128(&key[key_idx..], &mut state[2 * i..2 * i + 2], prev[i], pad);
            }
        } else {
            for i in 0..n {
                let key_idx = 2 + i * 6;
                _nettle_umac_poly128(&key[key_idx..], &mut state[2 * i..2 * i + 2], pad, 0);
            }
        }
        
        for i in 0..n {
            let yh = state[2 * i];
            let yl = state[2 * i + 1];
            if yh == UMAC_P128_HI && yl >= UMAC_P128_LO {
                state[2 * i] = 0;
                state[2 * i + 1] = yl - UMAC_P128_LO;
            }
        }
    }
}

// Note: The following functions are assumed to be defined elsewhere in the Rust codebase
fn _nettle_umac_poly64(k0: u32, k1: u32, y: u64, m: u64) -> u64 {
    unimplemented!()
}

fn _nettle_umac_poly128(key: &[u32], state: &mut [u64], prev: u64, m: u64) {
    unimplemented!()
}

const UMAC_POLY64_BLOCKS: u64 = 1024;  // This should match the C constant