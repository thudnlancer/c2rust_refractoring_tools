use std::ffi::CString;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut RanState, u64)>,
    pub get: Option<fn(&mut RanState) -> u64>,
    pub get_double: Option<fn(&mut RanState) -> f64>,
}

#[derive(Clone)]
pub struct RanState {
    pub i: u32,
    pub aa: [u64; 2009],
    pub ran_x: [u64; 100],
}

fn ran_array(aa: &mut [u64; 2009], n: u32, ran_x: &mut [u64; 100]) {
    let mut j = 0;
    while j < 100 {
        aa[j] = ran_x[j];
        j += 1;
    }
    
    while j < n as usize {
        aa[j] = (aa[j.wrapping_sub(100)]).wrapping_sub(aa[j.wrapping_sub(37)]) & ((1 << 30) - 1);
        j += 1;
    }
    
    let mut i = 0;
    while i < 37 {
        ran_x[i] = (aa[j.wrapping_sub(100)]).wrapping_sub(aa[j.wrapping_sub(37)]) & ((1 << 30) - 1);
        i += 1;
        j += 1;
    }
    
    while i < 100 {
        ran_x[i] = (aa[j.wrapping_sub(100)]).wrapping_sub(ran_x[i.wrapping_sub(37)]) & ((1 << 30) - 1);
        i += 1;
        j += 1;
    }
}

fn ran_get(state: &mut RanState) -> u64 {
    let i = state.i;
    if i == 0 {
        ran_array(&mut state.aa, 2009, &mut state.ran_x);
    }
    state.i = i.wrapping_add(1).wrapping_rem(2009);
    state.aa[i as usize]
}

fn ran_get_double(state: &mut RanState) -> f64 {
    ran_get(state) as f64 / 1073741824.0
}

fn ran_set(state: &mut RanState, s: u64) {
    let mut x = [0i64; 199];
    let mut j = 0;
    let mut ss = (s.wrapping_add(2) & ((1 << 30) - 2)) as i64;
    
    while j < 100 {
        x[j] = ss;
        ss <<= 1;
        if ss >= 1 << 30 {
            ss -= (1 << 30) - 2;
        }
        j += 1;
    }
    
    while j < 199 {
        x[j] = 0;
        j += 1;
    }
    
    x[1] += 1;
    ss = (s & ((1 << 30) - 1)) as i64;
    let mut t = 69;
    
    while t != 0 {
        j = 99;
        while j > 0 {
            x[j * 2] = x[j];
            j -= 1;
        }
        
        j = 198;
        while j > 63 {
            x[199 - j - 1] = x[j] & ((1 << 30) - 2);
            j -= 2;
        }
        
        j = 198;
        while j >= 100 {
            if x[j] & 1 != 0 {
                x[j - 63] = (x[j - 63] - x[j]) & ((1 << 30) - 1);
                x[j - 100] = (x[j - 100] - x[j]) & ((1 << 30) - 1);
            }
            j -= 1;
        }
        
        if ss & 1 != 0 {
            j = 100;
            while j > 0 {
                x[j] = x[j - 1];
                j -= 1;
            }
            x[0] = x[100];
            if x[100] & 1 != 0 {
                x[37] = (x[37] - x[100]) & ((1 << 30) - 1);
            }
        }
        
        if ss != 0 {
            ss >>= 1;
        } else {
            t -= 1;
        }
    }
    
    state.i = 0;
    j = 0;
    while j < 37 {
        state.ran_x[j + 63] = x[j] as u64;
        j += 1;
    }
    
    while j < 100 {
        state.ran_x[j - 37] = x[j] as u64;
        j += 1;
    }
}

pub static GSL_RNG_KNUTHRAN: GslRngType = GslRngType {
    name: CString::new("knuthran").unwrap(),
    max: 0x3fffffff,
    min: 0,
    size: std::mem::size_of::<RanState>(),
    set: Some(ran_set),
    get: Some(ran_get),
    get_double: Some(ran_get_double),
};