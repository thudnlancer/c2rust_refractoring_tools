use std::ffi::CString;

#[derive(Clone)]
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: Option<fn(&mut Ran1State, u64)>,
    pub get: Option<fn(&mut Ran1State) -> u64>,
    pub get_double: Option<fn(&mut Ran1State) -> f64>,
}

#[derive(Clone)]
pub struct Ran1State {
    x: u64,
    n: u64,
    shuffle: [u64; 32],
}

const M: i64 = 2147483647;
const A: i64 = 16807;
const Q: i64 = 127773;
const R: i64 = 2836;

fn ran1_get(state: &mut Ran1State) -> u64 {
    let x = state.x;
    let h = (x / Q as u64) as i64;
    let t = (A as u64)
        .wrapping_mul(x.wrapping_sub((h * Q) as u64))
        .wrapping_sub((h * R) as u64) as i64;
    
    state.x = if t < 0 { (t + M) as u64 } else { t as u64 };
    
    let j = state.n / (1 + 2147483646 / 32) as u64;
    state.n = state.shuffle[j as usize];
    state.shuffle[j as usize] = state.x;
    state.n
}

fn ran1_get_double(state: &mut Ran1State) -> f64 {
    const X_MAX: f32 = 1.0 - 1.2e-7;
    let x = ran1_get(state) as f32 / 2147483647.0;
    if x > X_MAX { X_MAX as f64 } else { x as f64 }
}

fn ran1_set(state: &mut Ran1State, mut s: u64) {
    if s == 0 {
        s = 1;
    }

    for _ in 0..8 {
        let h = (s / Q as u64) as i64;
        let t = (A as u64)
            .wrapping_mul(s.wrapping_sub((h * Q) as u64))
            .wrapping_sub((h * R) as u64) as i64;
        s = if t < 0 { (t + M) as u64 } else { t as u64 };
    }

    for i in (0..32).rev() {
        let h = (s / Q as u64) as i64;
        let t = (A as u64)
            .wrapping_mul(s.wrapping_sub((h * Q) as u64))
            .wrapping_sub((h * R) as u64) as i64;
        s = if t < 0 { (t + M) as u64 } else { t as u64 };
        state.shuffle[i] = s;
    }

    state.x = s;
    state.n = s;
}

lazy_static! {
    pub static ref GSL_RNG_RAN1: GslRngType = GslRngType {
        name: CString::new("ran1").unwrap(),
        max: 2147483646,
        min: 1,
        size: std::mem::size_of::<Ran1State>(),
        set: Some(ran1_set),
        get: Some(ran1_get),
        get_double: Some(ran1_get_double),
    };
}