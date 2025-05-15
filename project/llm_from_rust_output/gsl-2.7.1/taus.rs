use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut TausState, u64),
    pub get: fn(&mut TausState) -> u64,
    pub get_double: fn(&mut TausState) -> f64,
}

#[derive(Clone, Copy)]
pub struct TausState {
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
}

fn taus_get(state: &mut TausState) -> u64 {
    state.s1 = ((state.s1 & 0xFFFFFFFE) << 12) ^ (((state.s1 << 13) ^ state.s1) >> 19;
    state.s2 = ((state.s2 & 0xFFFFFFF8) << 4) ^ (((state.s2 << 2) ^ state.s2) >> 25;
    state.s3 = ((state.s3 & 0xFFFFFFF0) << 17) ^ (((state.s3 << 3) ^ state.s3) >> 11;
    state.s1 ^ state.s2 ^ state.s3
}

fn taus_get_double(state: &mut TausState) -> f64 {
    taus_get(state) as f64 / 4294967296.0
}

fn taus_set(state: &mut TausState, mut s: u64) {
    if s == 0 {
        s = 1;
    }
    state.s1 = 69069_u64.wrapping_mul(s) & 0xFFFFFFFF;
    state.s2 = 69069_u64.wrapping_mul(state.s1) & 0xFFFFFFFF;
    state.s3 = 69069_u64.wrapping_mul(state.s2) & 0xFFFFFFFF;
    
    for _ in 0..6 {
        taus_get(state);
    }
}

fn taus2_set(state: &mut TausState, mut s: u64) {
    if s == 0 {
        s = 1;
    }
    state.s1 = 69069_u64.wrapping_mul(s) & 0xFFFFFFFF;
    if state.s1 < 2 {
        state.s1 += 2;
    }
    
    state.s2 = 69069_u64.wrapping_mul(state.s1) & 0xFFFFFFFF;
    if state.s2 < 8 {
        state.s2 += 8;
    }
    
    state.s3 = 69069_u64.wrapping_mul(state.s2) & 0xFFFFFFFF;
    if state.s3 < 16 {
        state.s3 += 16;
    }
    
    for _ in 0..6 {
        taus_get(state);
    }
}

pub const GSL_RNG_TAUS: GslRngType = GslRngType {
    name: "taus",
    max: 0xFFFFFFFF,
    min: 0,
    size: std::mem::size_of::<TausState>(),
    set: taus_set,
    get: taus_get,
    get_double: taus_get_double,
};

pub const GSL_RNG_TAUS2: GslRngType = GslRngType {
    name: "taus2",
    max: 0xFFFFFFFF,
    min: 0,
    size: std::mem::size_of::<TausState>(),
    set: taus2_set,
    get: taus_get,
    get_double: taus_get_double,
};