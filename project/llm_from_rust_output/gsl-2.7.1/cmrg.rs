use std::ffi::CStr;
use std::marker::PhantomData;

pub type SizeT = usize;

#[derive(Debug, Clone)]
pub struct GslRngType {
    name: &'static str,
    max: u64,
    min: u64,
    size: SizeT,
    set: fn(&mut RngState, u64),
    get: fn(&mut RngState) -> u64,
    get_double: fn(&mut RngState) -> f64,
}

#[derive(Debug, Clone)]
pub struct CmrgState {
    x1: i64,
    x2: i64,
    x3: i64,
    y1: i64,
    y2: i64,
    y3: i64,
}

#[derive(Debug, Clone)]
pub struct RngState {
    state: CmrgState,
    _marker: PhantomData<*mut ()>,
}

const M1: i64 = 2147483647;
const M2: i64 = 2145483479;
const A2: i64 = 63308;
const QA2: i64 = 33921;
const RA2: i64 = 12979;
const A3: i64 = -183326;
const QA3: i64 = 11714;
const RA3: i64 = 2883;
const B1: i64 = 86098;
const QB1: i64 = 24919;
const RB1: i64 = 7417;
const B3: i64 = -539608;
const QB3: i64 = 3976;
const RB3: i64 = 2071;

fn cmrg_get(state: &mut RngState) -> u64 {
    let h3 = state.state.x3 / QA3;
    let mut p3 = -A3 * (state.state.x3 - h3 * QA3) - h3 * RA3;
    let h2 = state.state.x2 / QA2;
    let mut p2 = A2 * (state.state.x2 - h2 * QA2) - h2 * RA2;

    if p3 < 0 {
        p3 += M1;
    }
    if p2 < 0 {
        p2 += M1;
    }

    state.state.x3 = state.state.x2;
    state.state.x2 = state.state.x1;
    state.state.x1 = p2 - p3;
    if state.state.x1 < 0 {
        state.state.x1 += M1;
    }

    let h3 = state.state.y3 / QB3;
    let mut p3 = -B3 * (state.state.y3 - h3 * QB3) - h3 * RB3;
    let h1 = state.state.y1 / QB1;
    let mut p1 = B1 * (state.state.y1 - h1 * QB1) - h1 * RB1;

    if p3 < 0 {
        p3 += M2;
    }
    if p1 < 0 {
        p1 += M2;
    }

    state.state.y3 = state.state.y2;
    state.state.y2 = state.state.y1;
    state.state.y1 = p1 - p3;
    if state.state.y1 < 0 {
        state.state.y1 += M2;
    }

    if state.state.x1 < state.state.y1 {
        (state.state.x1 - state.state.y1 + M1) as u64
    } else {
        (state.state.x1 - state.state.y1) as u64
    }
}

fn cmrg_get_double(state: &mut RngState) -> f64 {
    cmrg_get(state) as f64 / 2147483647.0
}

fn cmrg_set(state: &mut RngState, mut s: u64) {
    if s == 0 {
        s = 1;
    }

    s = 69069u64.wrapping_mul(s) & 0xffffffff;
    state.state.x1 = (s % M1 as u64) as i64;
    
    s = 69069u64.wrapping_mul(s) & 0xffffffff;
    state.state.x2 = (s % M1 as u64) as i64;
    
    s = 69069u64.wrapping_mul(s) & 0xffffffff;
    state.state.x3 = (s % M1 as u64) as i64;
    
    s = 69069u64.wrapping_mul(s) & 0xffffffff;
    state.state.y1 = (s % M2 as u64) as i64;
    
    s = 69069u64.wrapping_mul(s) & 0xffffffff;
    state.state.y2 = (s % M2 as u64) as i64;
    
    s = 69069u64.wrapping_mul(s) & 0xffffffff;
    state.state.y3 = (s % M2 as u64) as i64;

    for _ in 0..7 {
        cmrg_get(state);
    }
}

pub static GSL_RNG_CMRG: GslRngType = GslRngType {
    name: "cmrg",
    max: 2147483646,
    min: 0,
    size: std::mem::size_of::<CmrgState>(),
    set: cmrg_set,
    get: cmrg_get,
    get_double: cmrg_get_double,
};