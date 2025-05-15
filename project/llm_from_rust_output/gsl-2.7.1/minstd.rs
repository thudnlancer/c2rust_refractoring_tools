use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut MinstdState, u64),
    pub get: fn(&mut MinstdState) -> u64,
    pub get_double: fn(&mut MinstdState) -> f64,
}

#[derive(Clone)]
pub struct MinstdState {
    x: u64,
}

const M: i64 = 2147483647;
const A: i64 = 16807;
const Q: i64 = 127773;
const R: i64 = 2836;

fn minstd_get(state: &mut MinstdState) -> u64 {
    let x = state.x;
    let h = (x / Q as u64) as i64;
    let t = (A as u64)
        .wrapping_mul(x.wrapping_sub((h * Q) as u64))
        .wrapping_sub((h * R) as u64) as i64;
    
    state.x = if t < 0 {
        (t + M) as u64
    } else {
        t as u64
    };
    
    state.x
}

fn minstd_get_double(state: &mut MinstdState) -> f64 {
    minstd_get(state) as f64 / 2147483647.0
}

fn minstd_set(state: &mut MinstdState, s: u64) {
    state.x = if s == 0 { 1 } else { s };
}

pub static MINSTD_TYPE: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"minstd\0") },
    max: 2147483646,
    min: 1,
    size: std::mem::size_of::<MinstdState>(),
    set: minstd_set,
    get: minstd_get,
    get_double: minstd_get_double,
};

pub static GSL_RNG_MINSTD: &'static GslRngType = &MINSTD_TYPE;