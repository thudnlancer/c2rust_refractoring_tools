use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut SlatecState, u64)>,
    pub get: Option<fn(&mut SlatecState) -> u64>,
    pub get_double: Option<fn(&mut SlatecState) -> f64>,
}

#[derive(Clone, Copy)]
pub struct SlatecState {
    pub x0: i64,
    pub x1: i64,
}

const P: i64 = 4194304;
const A1: i64 = 1536;
const A0: i64 = 1029;
const A1MA0: i64 = 507;
const C: i64 = 1731;

fn slatec_get(state: &mut SlatecState) -> u64 {
    let mut y0 = A0 * state.x0;
    let mut y1 = A1 * state.x1 + A1MA0 * (state.x0 - state.x1) + y0;
    
    y0 += C;
    state.x0 = y0 % 2048;
    y1 += (y0 - state.x0) / 2048;
    state.x1 = y1 % 2048;
    
    (state.x1 * 2048 + state.x0) as u64
}

fn slatec_get_double(state: &mut SlatecState) -> f64 {
    slatec_get(state) as f64 / 4194304.0
}

fn slatec_set(state: &mut SlatecState, s: u64) {
    let mut s = s % 8;
    s *= (P / 8) as u64;
    
    state.x0 = (s % 2048) as i64;
    state.x1 = ((s - state.x0 as u64) / 2048) as i64;
}

static SLATEC_TYPE: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"slatec\0") },
    max: 4194303,
    min: 0,
    size: std::mem::size_of::<SlatecState>(),
    set: Some(slatec_set),
    get: Some(slatec_get),
    get_double: Some(slatec_get_double),
};

pub static GSL_RNG_SLATEC: &GslRngType = &SLATEC_TYPE;