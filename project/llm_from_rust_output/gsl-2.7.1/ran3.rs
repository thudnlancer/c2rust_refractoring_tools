use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut Ran3State, u64),
    pub get: fn(&mut Ran3State) -> u64,
    pub get_double: fn(&mut Ran3State) -> f64,
}

#[derive(Clone)]
pub struct Ran3State {
    pub x: u32,
    pub y: u32,
    pub buffer: [u64; 56],
}

impl Ran3State {
    fn new() -> Self {
        Self {
            x: 0,
            y: 31,
            buffer: [0; 56],
        }
    }
}

fn ran3_get(state: &mut Ran3State) -> u64 {
    state.x = state.x.wrapping_add(1);
    if state.x == 56 {
        state.x = 1;
    }

    state.y = state.y.wrapping_add(1);
    if state.y == 56 {
        state.y = 1;
    }

    let mut j = state.buffer[state.x as usize].wrapping_sub(state.buffer[state.y as usize]) as i64;
    if j < 0 {
        j += 1_000_000_000;
    }

    state.buffer[state.x as usize] = j as u64;
    j as u64
}

fn ran3_get_double(state: &mut Ran3State) -> f64 {
    ran3_get(state) as f64 / 1_000_000_000.0
}

fn ran3_set(state: &mut Ran3State, s: u64) {
    let mut s = s;
    if s == 0 {
        s = 1;
    }

    let mut j = (161_803_398u64.wrapping_sub(s) % 1_000_000_000) as i64;
    if j < 0 {
        j += 1_000_000_000;
    }

    state.buffer[0] = 0;
    state.buffer[55] = j as u64;

    let mut k = 1i64;
    for i in 1..55 {
        let n = (21 * i) % 55;
        state.buffer[n] = k as u64;
        k = j - k;
        if k < 0 {
            k += 1_000_000_000;
        }
        j = state.buffer[n] as i64;
    }

    for _ in 0..4 {
        for i in 1..56 {
            let t = (state.buffer[i].wrapping_sub(
                state.buffer[(1 + (i + 30) % 55) as usize]
            )) as i64;
            let t = if t < 0 { t + 1_000_000_000 } else { t };
            state.buffer[i] = t as u64;
        }
    }

    state.x = 0;
    state.y = 31;
}

pub static GSL_RNG_RAN3: GslRngType = GslRngType {
    name: "ran3",
    max: 1_000_000_000,
    min: 0,
    size: std::mem::size_of::<Ran3State>(),
    set: ran3_set,
    get: ran3_get,
    get_double: ran3_get_double,
};