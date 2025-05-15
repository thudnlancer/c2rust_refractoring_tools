use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut RanmarState, u64),
    pub get: fn(&mut RanmarState) -> u64,
    pub get_double: fn(&mut RanmarState) -> f64,
}

#[derive(Clone)]
pub struct RanmarState {
    pub i: u32,
    pub j: u32,
    pub carry: i64,
    pub u: [u64; 97],
}

const TWO24: u64 = 16777216;

fn ranmar_get(state: &mut RanmarState) -> u64 {
    let i = state.i as usize;
    let j = state.j as usize;
    let mut carry = state.carry;
    
    let mut delta = state.u[i].wrapping_sub(state.u[j]) as i64;
    if delta < 0 {
        delta = (delta as u64).wrapping_add(TWO24) as i64;
    }
    
    state.u[i] = delta as u64;
    state.i = if state.i == 0 { 96 } else { state.i - 1 };
    state.j = if state.j == 0 { 96 } else { state.j - 1 };
    
    carry += -7654321;
    if carry < 0 {
        carry = (carry as u64).wrapping_add(TWO24.wrapping_sub(3)) as i64;
    }
    state.carry = carry;
    
    delta -= carry;
    if delta < 0 {
        delta = (delta as u64).wrapping_add(TWO24) as i64;
    }
    
    delta as u64
}

fn ranmar_get_double(state: &mut RanmarState) -> f64 {
    ranmar_get(state) as f64 / 16777216.0
}

fn ranmar_set(state: &mut RanmarState, s: u64) {
    let ij = s / 30082;
    let kl = s % 30082;
    
    let mut i = (ij / 177 % 177 + 2) as i32;
    let mut j = (ij % 177 + 2) as i32;
    let mut k = (kl / 169 % 178 + 1) as i32;
    let mut l = (kl % 169) as i32;
    
    for a in 0..97 {
        let mut sum = 0u64;
        let mut t = TWO24;
        
        for _ in 0..24 {
            let m = (i * j % 179 * k % 179) as u64;
            i = j;
            j = k;
            k = m as i32;
            l = (53 * l + 1) % 169;
            t >>= 1;
            
            if (l as u64).wrapping_mul(m) % 64 >= 32 {
                sum = sum.wrapping_add(t);
            }
        }
        
        state.u[a] = sum;
    }
    
    state.i = 96;
    state.j = 32;
    state.carry = 362436;
}

pub static GSL_RNG_RANMAR: GslRngType = GslRngType {
    name: "ranmar",
    max: 0xffffff,
    min: 0,
    size: std::mem::size_of::<RanmarState>(),
    set: ranmar_set,
    get: ranmar_get,
    get_double: ranmar_get_double,
};