use std::ffi::CStr;
use std::mem;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: fn(&mut RanState, u64),
    pub get: fn(&mut RanState) -> u64,
    pub get_double: fn(&mut RanState) -> f64,
}

#[derive(Clone, Copy)]
pub struct RanState {
    pub x: u64,
}

impl RanState {
    fn new(seed: u64) -> Self {
        let mut state = RanState { x: 0 };
        (GSL_RNG_FISHMAN18.set)(&mut state, seed);
        state
    }
}

fn schrage(a: u64, b: u64, m: u64) -> u64 {
    if a == 0 {
        return 0;
    }
    let q = m / a;
    let mut t = (2 * m) - (m % a) * (b / q);
    if t >= m {
        t -= m;
    }
    t += a * (b % q);
    if t >= m { t - m } else { t }
}

fn schrage_mult(a: u64, b: u64, m: u64, sqrtm: u64) -> u64 {
    let t0 = schrage(sqrtm, b, m);
    let t1 = schrage(a / sqrtm, t0, m);
    let t2 = schrage(a % sqrtm, b, m);
    let t = t1 + t2;
    if t >= m { t - m } else { t }
}

fn ran_get(state: &mut RanState) -> u64 {
    state.x = schrage_mult(62089911, state.x, 0x7fffffff, 46341);
    state.x
}

fn ran_get_double(state: &mut RanState) -> f64 {
    ran_get(state) as f64 / 2147483647.0
}

fn ran_set(state: &mut RanState, s: u64) {
    let mut seed = s;
    if seed % 0x7fffffff == 0 {
        seed = 1;
    }
    state.x = seed % 0x7fffffff;
}

pub const GSL_RNG_FISHMAN18: GslRngType = GslRngType {
    name: "fishman18",
    max: 0x7fffffff - 1,
    min: 1,
    size: mem::size_of::<RanState>(),
    set: ran_set,
    get: ran_get,
    get_double: ran_get_double,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schrage() {
        assert_eq!(schrage(5, 3, 7), 1);
    }

    #[test]
    fn test_ran_set_get() {
        let mut state = RanState::new(12345);
        let val = (GSL_RNG_FISHMAN18.get)(&mut state);
        assert!(val >= GSL_RNG_FISHMAN18.min && val <= GSL_RNG_FISHMAN18.max);
    }
}