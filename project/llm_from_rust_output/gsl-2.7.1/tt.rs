use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: fn(&mut State, u64),
    pub get: fn(&mut State) -> u64,
    pub get_double: fn(&mut State) -> f64,
}

#[derive(Clone, Copy)]
pub struct State {
    pub n: i32,
    pub x: [u64; 25],
}

impl State {
    pub fn new() -> Self {
        State {
            n: 0,
            x: [
                0x95f24dab,
                0x0b685215,
                0xe76ccae7,
                0xaf3ec239,
                0x715fad23,
                0x24a590ad,
                0x69e4b5ef,
                0xbf456141,
                0x96bc1b7b,
                0xa7bdf825,
                0xc1de75b7,
                0x8858a9c9,
                0x2da87693,
                0xb657f9dd,
                0xffdc8a9f,
                0x8121da71,
                0x8b823ecb,
                0x885d05f5,
                0x4e20cd47,
                0x5a9ad5d9,
                0x512c0c03,
                0xea857ccd,
                0x4cc1d30f,
                0x8891a8a1,
                0xa6b7aadb,
            ],
        }
    }
}

fn tt_get(state: &mut State) -> u64 {
    let mag01 = [0u64, 0x8ebfd028];
    let mut y;

    if state.n >= 25 {
        let mut i = 0;
        while i < 25 - 7 {
            state.x[i as usize] = state.x[(i + 7) as usize] 
                ^ (state.x[i as usize] >> 1) 
                ^ mag01[(state.x[i as usize] % 2) as usize];
            i += 1;
        }
        while i < 25 {
            state.x[i as usize] = state.x[(i + (7 - 25)) as usize] 
                ^ (state.x[i as usize] >> 1) 
                ^ mag01[(state.x[i as usize] % 2) as usize];
            i += 1;
        }
        state.n = 0;
    }

    y = state.x[state.n as usize];
    y ^= (y << 7) & 0x2b5b2500;
    y ^= (y << 15) & 0xdb8b0000;
    y &= 0xffffffff;
    y ^= y >> 16;
    state.n += 1;
    y
}

fn tt_get_double(state: &mut State) -> f64 {
    tt_get(state) as f64 / 4294967296.0
}

fn tt_set(state: &mut State, s: u64) {
    if s == 0 {
        *state = State::new();
    } else {
        state.n = 0;
        state.x[0] = s & 0xffffffff;
        for i in 1..25 {
            state.x[i as usize] = (69069u64)
                .wrapping_mul(state.x[(i - 1) as usize])
                & 0xffffffff;
        }
    }
}

pub static GSL_RNG_TT800: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"tt800\0") },
    max: 0xffffffff,
    min: 0,
    size: std::mem::size_of::<State>(),
    set: tt_set,
    get: tt_get,
    get_double: tt_get_double,
};