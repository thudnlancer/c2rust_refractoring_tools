use std::ffi::CStr;

pub type SizeT = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: SizeT,
    pub set: Option<fn(&mut UniState, u64)>,
    pub get: Option<fn(&mut UniState) -> u64>,
    pub get_double: Option<fn(&mut UniState) -> f64>,
}

#[derive(Clone)]
pub struct UniState {
    pub i: i32,
    pub j: i32,
    pub m: [u64; 17],
}

const M1: u32 = 32767;
const M2: u32 = 256;

fn uni_get(state: &mut UniState) -> u64 {
    let i = state.i;
    let j = state.j;
    let mut k = state.m[i as usize].wrapping_sub(state.m[j as usize]) as i64;

    if k < 0 {
        k += M1 as i64;
    }

    state.m[j as usize] = k as u64;

    state.i = if i == 0 { 16 } else { i - 1 };
    state.j = if j == 0 { 16 } else { j - 1 };

    k as u64
}

fn uni_get_double(state: &mut UniState) -> f64 {
    uni_get(state) as f64 / 32767.0
}

fn uni_set(state: &mut UniState, mut s: u64) {
    s = s.wrapping_mul(2).wrapping_add(1);
    let mut seed = if s < M1 as u64 { s } else { M1 as u64 } as u32;

    let k0 = 9069 % M2;
    let k1 = 9069 / M2;
    let mut j0 = seed % M2;
    let mut j1 = seed / M2;

    for i in 0..17 {
        seed = j0.wrapping_mul(k0);
        j1 = (seed / M2)
            .wrapping_add(j0.wrapping_mul(k1))
            .wrapping_add(j1.wrapping_mul(k0))
            .wrapping_rem(M2 / 2);
        j0 = seed % M2;
        state.m[i as usize] = (j0 + M2 * j1) as u64;
    }

    state.i = 4;
    state.j = 16;
}

pub static GSL_RNG_UNI: GslRngType = GslRngType {
    name: "uni",
    max: 32766,
    min: 0,
    size: std::mem::size_of::<UniState>(),
    set: Some(uni_set),
    get: Some(uni_get),
    get_double: Some(uni_get_double),
};