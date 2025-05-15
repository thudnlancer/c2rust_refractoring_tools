use std::convert::TryInto;
use std::mem;

type Uint32 = u32;
type Uint64 = u64;

fn umac_poly64(kh: Uint32, kl: Uint32, y: Uint64, m: Uint64) -> Uint64;
fn umac_poly128(k: &[Uint32], y: &mut [Uint64], mh: Uint64, ml: Uint64);

pub fn umac_l2_init(size: usize, k: &mut [Uint32]) {
    for word in k.iter_mut().take(size) {
        *word = word.swap_bytes() & 0x1ffffff;
    }
}

pub fn umac_l2(
    key: &[Uint32],
    state: &mut [Uint64],
    n: usize,
    count: Uint64,
    m: &[Uint64],
) {
    let prev = &mut state[2 * n..];
    
    match count {
        0 => {
            prev[..n].copy_from_slice(&m[..n]);
        }
        1 => {
            for i in 0..n {
                let y = umac_poly64(key[0], key[1], 1, prev[i]);
                state[2 * i + 1] = umac_poly64(key[0], key[1], y, m[i]);
            }
        }
        c if c < 16384 => {
            for i in 0..n {
                state[2 * i + 1] = umac_poly64(key[0], key[1], state[2 * i + 1], m[i]);
            }
        }
        c if c % 2 == 0 => {
            if c == 16384 {
                for i in 0..n {
                    let mut y = state[2 * i + 1];
                    if y >= (!59 + 1) {
                        y -= !59 + 1;
                    }
                    state[2 * i] = 0;
                    state[2 * i + 1] = 1;
                    umac_poly128(&key[2..], &mut state[2 * i..2 * i + 2], 0, y);
                }
            }
            prev[..n].copy_from_slice(&m[..n]);
        }
        _ => {
            for i in 0..n {
                umac_poly128(&key[2..], &mut state[2 * i..2 * i + 2], prev[i], m[i]);
            }
        }
    }
}

pub fn umac_l2_final(key: &[Uint32], state: &mut [Uint64], n: usize, count: Uint64) {
    assert!(count > 0);
    let prev = &mut state[2 * n..];

    match count {
        1 => {
            for i in 0..n {
                state[2 * i] = 0;
                state[2 * i + 1] = prev[i];
            }
        }
        c if c <= 16384 => {
            for i in 0..n {
                state[2 * i] = 0;
                let mut y = state[2 * i + 1];
                if y >= (!59 + 1) {
                    y -= !59 + 1;
                }
                state[2 * i + 1] = y;
            }
        }
        _ => {
            let pad = 1 << 63;
            if count % 2 == 1 {
                for i in 0..n {
                    umac_poly128(&key[2..], &mut state[2 * i..2 * i + 2], prev[i], pad);
                }
            } else {
                for i in 0..n {
                    umac_poly128(&key[2..], &mut state[2 * i..2 * i + 2], pad, 0);
                }
            }
            
            for i in 0..n {
                let yh = state[2 * i];
                let yl = state[2 * i + 1];
                if yh == !0 && yl >= (!159 + 1) {
                    state[2 * i] = 0;
                    state[2 * i + 1] = yl - (!159 + 1);
                }
            }
        }
    }
}