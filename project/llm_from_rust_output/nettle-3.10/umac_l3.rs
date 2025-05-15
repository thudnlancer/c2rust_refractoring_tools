use std::num::Wrapping;

pub type Uint32 = u32;
pub type Uint64 = u64;

pub fn nettle_umac_l3_init(size: usize, k: &mut [Uint64]) {
    for i in 0..size {
        let mut w = k[i].swap_bytes();
        w = (Wrapping(w) % Wrapping(0xffffffffb)).0;
        k[i] = w;
    }
}

fn umac_l3_word(k: &[Uint64], mut w: Uint64) -> Uint64 {
    let mut y = Wrapping(0u64);
    for i in 0..4 {
        y += Wrapping((w & 0xffff) * k[3 - i]);
        w >>= 16;
    }
    y.0
}

pub fn nettle_umac_l3(key: &[Uint64], m: &[Uint64]) -> Uint32 {
    let y = (Wrapping(umac_l3_word(&key[0..4], m[0])) + 
             Wrapping(umac_l3_word(&key[4..8], m[1]))) % 
             Wrapping(0xffffffffb);
    y.0.swap_bytes() as Uint32
}