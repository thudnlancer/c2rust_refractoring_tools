use std::ffi::CStr;

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut RanlxdState, u64),
    pub get: fn(&mut RanlxdState) -> u64,
    pub get_double: fn(&mut RanlxdState) -> f64,
}

#[derive(Copy, Clone)]
pub struct RanlxdState {
    pub xdbl: [f64; 12],
    pub carry: f64,
    pub ir: u32,
    pub jr: u32,
    pub ir_old: u32,
    pub pr: u32,
}

const NEXT: [usize; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0];
const ONE_BIT: f64 = 1.0 / 281474976710656.0;

fn increment_state(state: &mut RanlxdState) {
    let mut k = 0;
    let mut carry = state.carry;
    let mut ir = state.ir;
    let mut jr = state.jr;
    
    while ir > 0 {
        let y1 = state.xdbl[jr as usize] - state.xdbl[ir as usize];
        let mut y2 = y1 - carry;
        if y2 < 0.0 {
            carry = ONE_BIT;
            y2 += 1.0;
        } else {
            carry = 0.0;
        }
        state.xdbl[ir as usize] = y2;
        ir = NEXT[ir as usize] as u32;
        jr = NEXT[jr as usize] as u32;
        k += 1;
    }

    let kmax = (state.pr.wrapping_sub(12)) as i32;
    while k <= kmax {
        let mut y1 = state.xdbl[7] - state.xdbl[0] - carry;
        let mut y2 = state.xdbl[8] - state.xdbl[1];
        if y1 < 0.0 {
            y2 -= ONE_BIT;
            y1 += 1.0;
        }
        state.xdbl[0] = y1;
        
        let mut y3 = state.xdbl[9] - state.xdbl[2];
        if y2 < 0.0 {
            y3 -= ONE_BIT;
            y2 += 1.0;
        }
        state.xdbl[1] = y2;
        
        y1 = state.xdbl[10] - state.xdbl[3];
        if y3 < 0.0 {
            y1 -= ONE_BIT;
            y3 += 1.0;
        }
        state.xdbl[2] = y3;
        
        y2 = state.xdbl[11] - state.xdbl[4];
        if y1 < 0.0 {
            y2 -= ONE_BIT;
            y1 += 1.0;
        }
        state.xdbl[3] = y1;
        
        y3 = state.xdbl[0] - state.xdbl[5];
        if y2 < 0.0 {
            y3 -= ONE_BIT;
            y2 += 1.0;
        }
        state.xdbl[4] = y2;
        
        y1 = state.xdbl[1] - state.xdbl[6];
        if y3 < 0.0 {
            y1 -= ONE_BIT;
            y3 += 1.0;
        }
        state.xdbl[5] = y3;
        
        y2 = state.xdbl[2] - state.xdbl[7];
        if y1 < 0.0 {
            y2 -= ONE_BIT;
            y1 += 1.0;
        }
        state.xdbl[6] = y1;
        
        y3 = state.xdbl[3] - state.xdbl[8];
        if y2 < 0.0 {
            y3 -= ONE_BIT;
            y2 += 1.0;
        }
        state.xdbl[7] = y2;
        
        y1 = state.xdbl[4] - state.xdbl[9];
        if y3 < 0.0 {
            y1 -= ONE_BIT;
            y3 += 1.0;
        }
        state.xdbl[8] = y3;
        
        y2 = state.xdbl[5] - state.xdbl[10];
        if y1 < 0.0 {
            y2 -= ONE_BIT;
            y1 += 1.0;
        }
        state.xdbl[9] = y1;
        
        y3 = state.xdbl[6] - state.xdbl[11];
        if y2 < 0.0 {
            y3 -= ONE_BIT;
            y2 += 1.0;
        }
        state.xdbl[10] = y2;
        
        if y3 < 0.0 {
            carry = ONE_BIT;
            y3 += 1.0;
        } else {
            carry = 0.0;
        }
        state.xdbl[11] = y3;
        k += 12;
    }

    let kmax = state.pr as i32;
    while k < kmax {
        let y1 = state.xdbl[jr as usize] - state.xdbl[ir as usize];
        let mut y2 = y1 - carry;
        if y2 < 0.0 {
            carry = ONE_BIT;
            y2 += 1.0;
        } else {
            carry = 0.0;
        }
        state.xdbl[ir as usize] = y2;
        ir = NEXT[ir as usize] as u32;
        jr = NEXT[jr as usize] as u32;
        k += 1;
    }

    state.ir = ir;
    state.ir_old = ir;
    state.jr = jr;
    state.carry = carry;
}

fn ranlxd_get(state: &mut RanlxdState) -> u64 {
    (ranlxd_get_double(state) * 4294967296.0) as u64
}

fn ranlxd_get_double(state: &mut RanlxdState) -> f64 {
    let ir = state.ir as usize;
    state.ir = NEXT[ir] as u32;
    if state.ir == state.ir_old {
        increment_state(state);
    }
    state.xdbl[state.ir as usize]
}

fn ranlxd_set_lux(state: &mut RanlxdState, s: u64, luxury: u32) {
    let mut xbit = [0; 31];
    let mut seed = if s == 0 { 1 } else { s } as i64;
    
    let mut i = (seed as u64 & 0xFFFF_FFFF) as i32;
    for k in 0..31 {
        xbit[k] = i % 2;
        i /= 2;
    }

    let mut ibit = 0;
    let mut jbit = 18;
    
    for k in 0..12 {
        let mut x = 0.0;
        for _ in 1..=48 {
            let y = ((xbit[ibit] + 1) % 2) as f64;
            x += x + y;
            xbit[ibit] = (xbit[ibit] + xbit[jbit]) % 2;
            ibit = (ibit + 1) % 31;
            jbit = (jbit + 1) % 31;
        }
        state.xdbl[k] = ONE_BIT * x;
    }

    state.carry = 0.0;
    state.ir = 11;
    state.jr = 7;
    state.ir_old = 0;
    state.pr = luxury;
}

fn ranlxd1_set(state: &mut RanlxdState, s: u64) {
    ranlxd_set_lux(state, s, 202);
}

fn ranlxd2_set(state: &mut RanlxdState, s: u64) {
    ranlxd_set_lux(state, s, 397);
}

const RANLXD1_TYPE: GslRngType = GslRngType {
    name: "ranlxd1",
    max: 0xFFFF_FFFF,
    min: 0,
    size: std::mem::size_of::<RanlxdState>(),
    set: ranlxd1_set,
    get: ranlxd_get,
    get_double: ranlxd_get_double,
};

const RANLXD2_TYPE: GslRngType = GslRngType {
    name: "ranlxd2",
    max: 0xFFFF_FFFF,
    min: 0,
    size: std::mem::size_of::<RanlxdState>(),
    set: ranlxd2_set,
    get: ranlxd_get,
    get_double: ranlxd_get_double,
};

pub static GSL_RNG_RANLXD1: &GslRngType = &RANLXD1_TYPE;
pub static GSL_RNG_RANLXD2: &GslRngType = &RANLXD2_TYPE;