use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut ZufState, u64),
    pub get: fn(&mut ZufState) -> u64,
    pub get_double: fn(&mut ZufState) -> f64,
}

#[derive(Clone)]
pub struct ZufState {
    pub n: i32,
    pub u: [u64; 607],
}

const ZUF_RANDMAX: u64 = 16777216;

fn zuf_get(state: &mut ZufState) -> u64 {
    let n = state.n;
    let m = (n - 273 + 607) % 607;
    let mut t = state.u[n as usize].wrapping_add(state.u[m as usize]);
    
    while t > ZUF_RANDMAX {
        t = t.wrapping_sub(ZUF_RANDMAX);
    }
    
    state.u[n as usize] = t;
    state.n = if n == 606 { 0 } else { n + 1 };
    t
}

fn zuf_get_double(state: &mut ZufState) -> f64 {
    zuf_get(state) as f64 / 16777216.0
}

fn zuf_set(state: &mut ZufState, s: u64) {
    let mut kl: i64 = 9373;
    let mut ij: i64 = 1802;
    let mut i: i64;
    let mut j: i64;
    let mut k: i64;
    let mut l: i64;
    let mut m: i64;
    let mut x: f64;
    let mut y: f64;
    let mut ii: i64;
    let mut jj: i64;

    state.n = 0;
    let s = if s == 0 { 1802 } else { s } as i64;
    
    ij = s;
    i = (ij / 177 % 177) + 2;
    j = (ij % 177) + 2;
    k = (kl / 169 % 178) + 1;
    l = kl % 169;

    for ii in 0..607 {
        x = 0.0;
        y = 0.5;
        
        for _ in 1..=24 {
            m = (((i * j) % 179) * k) % 179;
            i = j;
            j = k;
            k = m;
            l = (l * 53 + 1) % 169;
            
            if (l * m % 64) >= 32 {
                x += y;
            }
            y *= 0.5;
        }
        
        state.u[ii as usize] = (x * ZUF_RANDMAX as f64) as u64;
    }
}

pub static GSL_RNG_ZUF: GslRngType = GslRngType {
    name: "zuf",
    max: 0xffffff,
    min: 0,
    size: std::mem::size_of::<ZufState>(),
    set: zuf_set,
    get: zuf_get,
    get_double: zuf_get_double,
};