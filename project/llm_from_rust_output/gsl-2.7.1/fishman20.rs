use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct RanState {
    x: u32,
}

pub struct GslRngType {
    name: &'static str,
    max: u32,
    min: u32,
    size: usize,
    set: fn(&mut RanState, u32),
    get: fn(&RanState) -> u32,
    get_double: fn(&RanState) -> f64,
}

const M: i32 = 2147483647;
const A: i32 = 48271;
const Q: i32 = 44488;
const R: i32 = 3399;

fn ran_get(state: &RanState) -> u32 {
    let x = state.x as i64;
    let h = (x / Q as i64) as i32;
    let t = (A as i64 * (x - (h * Q) as i64) - (h * R) as i64;
    
    if t < 0 {
        (t + M as i64) as u32
    } else {
        t as u32
    }
}

fn ran_get_double(state: &RanState) -> f64 {
    ran_get(state) as f64 / 2147483647.0
}

fn ran_set(state: &mut RanState, s: u32) {
    let mut seed = s;
    if seed % M as u32 == 0 {
        seed = 1;
    }
    state.x = seed & M as u32;
}

pub static GSL_RNG_FISHMAN20: GslRngType = GslRngType {
    name: "fishman20",
    max: 2147483646,
    min: 1,
    size: std::mem::size_of::<RanState>(),
    set: ran_set,
    get: ran_get,
    get_double: ran_get_double,
};