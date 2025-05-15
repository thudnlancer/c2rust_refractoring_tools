use std::ffi::CString;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut Ran2State, u64)>,
    pub get: Option<fn(&mut Ran2State) -> u64>,
    pub get_double: Option<fn(&mut Ran2State) -> f64>,
}

#[derive(Clone)]
pub struct Ran2State {
    pub x: u64,
    pub y: u64,
    pub n: u64,
    pub shuffle: [u64; 32],
}

const M1: i64 = 2147483563;
const A1: i64 = 40014;
const Q1: i64 = 53668;
const R1: i64 = 12211;
const M2: i64 = 2147483399;
const A2: i64 = 40692;
const Q2: i64 = 52774;
const R2: i64 = 3791;

fn ran2_get(state: &mut Ran2State) -> u64 {
    let x = state.x;
    let y = state.y;
    
    let h1 = (x / Q1 as u64) as i64;
    let mut t1 = (A1 as u64)
        .wrapping_mul(x.wrapping_sub((h1 * Q1) as u64))
        .wrapping_sub((h1 * R1) as u64) as i64;
    
    let h2 = (y / Q2 as u64) as i64;
    let mut t2 = (A2 as u64)
        .wrapping_mul(y.wrapping_sub((h2 * Q2) as u64))
        .wrapping_sub((h2 * R2) as u64) as i64;
    
    if t1 < 0 {
        t1 += M1;
    }
    if t2 < 0 {
        t2 += M2;
    }
    
    state.x = t1 as u64;
    state.y = t2 as u64;
    
    let j = state.n / (1 + 2147483562 / 32) as u64;
    let mut delta = (state.shuffle[j as usize] as i64).wrapping_sub(t2);
    
    if delta < 1 {
        delta += M1 - 1;
    }
    
    state.n = delta as u64;
    state.shuffle[j as usize] = t1 as u64;
    
    state.n
}

fn ran2_get_double(state: &mut Ran2State) -> f64 {
    const X_MAX: f32 = 1.0 - 1.2e-7;
    let x = ran2_get(state) as f32 / 2147483563.0;
    
    if x > X_MAX {
        X_MAX as f64
    } else {
        x as f64
    }
}

fn ran2_set(state: &mut Ran2State, mut s: u64) {
    if s == 0 {
        s = 1;
    }
    
    state.y = s;
    
    for _ in 0..8 {
        let h = (s / Q1 as u64) as i64;
        let mut t = (A1 as u64)
            .wrapping_mul(s.wrapping_sub((h * Q1) as u64))
            .wrapping_sub((h * R1) as u64) as i64;
        
        if t < 0 {
            t += M1;
        }
        s = t as u64;
    }
    
    for i in (0..32).rev() {
        let h = (s / Q1 as u64) as i64;
        let mut t = (A1 as u64)
            .wrapping_mul(s.wrapping_sub((h * Q1) as u64))
            .wrapping_sub((h * R1) as u64) as i64;
        
        if t < 0 {
            t += M1;
        }
        s = t as u64;
        state.shuffle[i] = s;
    }
    
    state.x = s;
    state.n = s;
}

lazy_static! {
    pub static ref GSL_RNG_RAN2: GslRngType = GslRngType {
        name: CString::new("ran2").unwrap(),
        max: 2147483562,
        min: 1,
        size: std::mem::size_of::<Ran2State>(),
        set: Some(ran2_set),
        get: Some(ran2_get),
        get_double: Some(ran2_get_double),
    };
}