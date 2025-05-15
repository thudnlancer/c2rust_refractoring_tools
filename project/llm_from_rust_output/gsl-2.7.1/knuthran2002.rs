use std::ffi::CString;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut RanState, u64),
    pub get: fn(&mut RanState) -> u64,
    pub get_double: fn(&mut RanState) -> f64,
}

#[derive(Clone)]
pub struct RanState {
    pub i: u32,
    pub aa: [i64; 1009],
    pub ran_x: [i64; 100],
}

fn ran_array(aa: &mut [i64; 1009], n: u32, ran_x: &mut [i64; 100]) {
    let mut j = 0;
    while j < 100 {
        aa[j] = ran_x[j];
        j += 1;
    }

    while j < n as usize {
        aa[j] = (aa[j - 100] - aa[j - 37]) & ((1 << 30) - 1);
        j += 1;
    }

    let mut i = 0;
    while i < 37 {
        ran_x[i] = (aa[j - 100] - aa[j - 37]) & ((1 << 30) - 1);
        i += 1;
        j += 1;
    }

    while i < 100 {
        ran_x[i] = (aa[j - 100] - ran_x[i - 37]) & ((1 << 30) - 1);
        i += 1;
        j += 1;
    }
}

fn ran_get(state: &mut RanState) -> u64 {
    let i = state.i as usize;
    if i == 0 {
        ran_array(&mut state.aa, 1009, &mut state.ran_x);
    }
    let v = state.aa[i] as u64;
    state.i = (i + 1) % 100;
    v
}

fn ran_get_double(state: &mut RanState) -> f64 {
    ran_get(state) as f64 / 1073741824.0
}

fn ran_set(state: &mut RanState, mut s: u64) {
    let mut x = [0i64; 199];
    let mut ss;

    if s == 0 {
        s = 314159;
    }

    ss = (s + 2) & ((1 << 30) - 2) as u64;
    ss = ss as i64;

    for j in 0..100 {
        x[j] = ss;
        ss <<= 1;
        if ss >= 1 << 30 {
            ss -= (1 << 30) - 2;
        }
    }

    x[1] += 1;
    ss = (s & ((1 << 30) - 1) as u64) as i64;

    let mut t = 70 - 1;
    while t != 0 {
        for j in (1..100).rev() {
            x[j * 2] = x[j];
            x[j * 2 - 1] = 0;
        }

        for j in (100..(200 - 1)).rev() {
            x[j - (100 - 37)] = (x[j - (100 - 37)] - x[j]) & ((1 << 30) - 1);
            x[j - 100] = (x[j - 100] - x[j]) & ((1 << 30) - 1);
        }

        if ss & 1 != 0 {
            for j in (1..=100).rev() {
                x[j] = x[j - 1];
            }
            x[0] = x[100];
            x[37] = (x[37] - x[100]) & ((1 << 30) - 1);
        }

        if ss != 0 {
            ss >>= 1;
        } else {
            t -= 1;
        }
    }

    for j in 0..37 {
        state.ran_x[j + 100 - 37] = x[j];
    }

    for j in 37..100 {
        state.ran_x[j - 37] = x[j];
    }

    for _ in 0..10 {
        ran_array(&mut x, (200 - 1) as u32, &mut state.ran_x);
    }

    state.i = 0;
}

pub static GSL_RNG_KNUTHRAN2002: GslRngType = GslRngType {
    name: CString::new("knuthran2002").unwrap(),
    max: 0x3fffffff,
    min: 0,
    size: std::mem::size_of::<RanState>(),
    set: ran_set,
    get: ran_get,
    get_double: ran_get_double,
};