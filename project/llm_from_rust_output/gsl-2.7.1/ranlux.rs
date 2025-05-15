use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut RanluxState, u64),
    pub get: fn(&mut RanluxState) -> u64,
    pub get_double: fn(&mut RanluxState) -> f64,
}

#[derive(Clone)]
pub struct RanluxState {
    pub i: u32,
    pub j: u32,
    pub n: u32,
    pub skip: u32,
    pub carry: u32,
    pub u: [u64; 24],
}

const MASK_LO: u64 = 0xffffff;
const MASK_HI: u64 = !0xffffff;
const TWO24: u64 = 16777216;

fn increment_state(state: &mut RanluxState) -> u64 {
    let i = state.i;
    let j = state.j;
    let delta = state.u[j as usize].wrapping_sub(state.u[i as usize]).wrapping_sub(state.carry as u64) as i64;

    let (carry, delta) = if (delta as u64 & MASK_HI) != 0 {
        (1, (delta as u64 & MASK_LO) as i64)
    } else {
        (0, delta)
    };

    state.u[i as usize] = delta as u64;
    state.carry = carry;
    state.i = if i == 0 { 23 } else { i - 1 };
    state.j = if j == 0 { 23 } else { j - 1 };

    delta as u64
}

fn ranlux_get(state: &mut RanluxState) -> u64 {
    let r = increment_state(state);
    state.n = state.n.wrapping_add(1);

    if state.n == 24 {
        state.n = 0;
        for _ in 0..state.skip {
            increment_state(state);
        }
    }

    r
}

fn ranlux_get_double(state: &mut RanluxState) -> f64 {
    ranlux_get(state) as f64 / 16777216.0
}

fn ranlux_set_lux(state: &mut RanluxState, mut s: u64, luxury: u32) {
    if s == 0 {
        s = 314159265;
    }

    let mut seed = s as i64;
    for i in 0..24 {
        let k = (seed / 53668) as u64;
        seed = (40014u64.wrapping_mul((seed as u64).wrapping_sub(k.wrapping_mul(53668)))
            .wrapping_sub(k.wrapping_mul(12211)) as i64;
        if seed < 0 {
            seed += 2147483563;
        }
        state.u[i as usize] = (seed as u64) % TWO24;
    }

    state.i = 23;
    state.j = 9;
    state.n = 0;
    state.skip = luxury - 24;
    state.carry = if state.u[23] & MASK_HI != 0 { 1 } else { 0 };
}

fn ranlux_set(state: &mut RanluxState, s: u64) {
    ranlux_set_lux(state, s, 223);
}

fn ranlux389_set(state: &mut RanluxState, s: u64) {
    ranlux_set_lux(state, s, 389);
}

pub const GSL_RNG_RANLUX: GslRngType = GslRngType {
    name: "ranlux",
    max: 0xffffff,
    min: 0,
    size: std::mem::size_of::<RanluxState>(),
    set: ranlux_set,
    get: ranlux_get,
    get_double: ranlux_get_double,
};

pub const GSL_RNG_RANLUX389: GslRngType = GslRngType {
    name: "ranlux389",
    max: 0xffffff,
    min: 0,
    size: std::mem::size_of::<RanluxState>(),
    set: ranlux389_set,
    get: ranlux_get,
    get_double: ranlux_get_double,
};