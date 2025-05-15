use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: fn(&mut RanduState, u64),
    pub get: fn(&mut RanduState) -> u64,
    pub get_double: fn(&mut RanduState) -> f64,
}

#[derive(Clone, Copy)]
pub struct RanduState {
    pub x: u64,
}

const A: u64 = 65539;

fn randu_get(state: &mut RanduState) -> u64 {
    state.x = (A.wrapping_mul(state.x)) & 0x7fffffff;
    state.x
}

fn randu_get_double(state: &mut RanduState) -> f64 {
    randu_get(state) as f64 / 2147483648.0
}

fn randu_set(state: &mut RanduState, s: u64) {
    let seed = if s == 0 { 1 } else { s };
    state.x = seed;
}

pub const RANDU_TYPE: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"randu\0") },
    max: 0x7fffffff,
    min: 1,
    size: std::mem::size_of::<RanduState>(),
    set: randu_set,
    get: randu_get,
    get_double: randu_get_double,
};

pub static GSL_RNG_RANDU: &GslRngType = &RANDU_TYPE;