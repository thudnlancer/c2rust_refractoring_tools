use std::ffi::CString;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut dyn RngState, u64)>,
    pub get: Option<fn(&mut dyn RngState) -> u64>,
    pub get_double: Option<fn(&mut dyn RngState) -> f64>,
}

pub trait RngState {
    fn increment_state(&mut self);
    fn get_double(&mut self) -> f64;
    fn get(&mut self) -> u64;
    fn set_lux(&mut self, s: u64, luxury: u32);
}

#[derive(Clone)]
pub struct RanlxsState {
    xdbl: [f64; 12],
    ydbl: [f64; 12],
    carry: f64,
    xflt: [f32; 24],
    ir: u32,
    jr: u32,
    is: u32,
    is_old: u32,
    pr: u32,
}

impl RanlxsState {
    const NEXT: [i32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0];
    const SNEXT: [i32; 24] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 0,
    ];
    const SBASE: f64 = 16777216.0;
    const SONE_BIT: f64 = 1.0 / 16777216.0;
    const ONE_BIT: f64 = 1.0 / 281474976710656.0;
    const SHIFT: f64 = 268435456.0;

    pub fn new() -> Self {
        Self {
            xdbl: [0.0; 12],
            ydbl: [0.0; 12],
            carry: 0.0,
            xflt: [0.0; 24],
            ir: 0,
            jr: 7,
            is: 23,
            is_old: 0,
            pr: 109,
        }
    }

    fn increment_state(&mut self) {
        let mut k = 0;
        let mut kmax;
        let mut m;
        let mut x;
        let mut y1;
        let mut y2;
        let mut y3;
        let mut carry = self.carry;
        let mut ir = self.ir;
        let mut jr = self.jr;

        while ir > 0 {
            y1 = self.xdbl[jr as usize] - self.xdbl[ir as usize];
            y2 = y1 - carry;
            if y2 < 0.0 {
                carry = Self::ONE_BIT;
                y2 += 1.0;
            } else {
                carry = 0.0;
            }
            self.xdbl[ir as usize] = y2;
            ir = Self::NEXT[ir as usize] as u32;
            jr = Self::NEXT[jr as usize] as u32;
            k += 1;
        }

        kmax = (self.pr - 12) as i32;
        while k <= kmax {
            y1 = self.xdbl[7] - self.xdbl[0] - carry;
            y2 = self.xdbl[8] - self.xdbl[1];
            if y1 < 0.0 {
                y2 -= Self::ONE_BIT;
                y1 += 1.0;
            }
            self.xdbl[0] = y1;
            
            y3 = self.xdbl[9] - self.xdbl[2];
            if y2 < 0.0 {
                y3 -= Self::ONE_BIT;
                y2 += 1.0;
            }
            self.xdbl[1] = y2;
            
            y1 = self.xdbl[10] - self.xdbl[3];
            if y3 < 0.0 {
                y1 -= Self::ONE_BIT;
                y3 += 1.0;
            }
            self.xdbl[2] = y3;
            
            y2 = self.xdbl[11] - self.xdbl[4];
            if y1 < 0.0 {
                y2 -= Self::ONE_BIT;
                y1 += 1.0;
            }
            self.xdbl[3] = y1;
            
            y3 = self.xdbl[0] - self.xdbl[5];
            if y2 < 0.0 {
                y3 -= Self::ONE_BIT;
                y2 += 1.0;
            }
            self.xdbl[4] = y2;
            
            y1 = self.xdbl[1] - self.xdbl[6];
            if y3 < 0.0 {
                y1 -= Self::ONE_BIT;
                y3 += 1.0;
            }
            self.xdbl[5] = y3;
            
            y2 = self.xdbl[2] - self.xdbl[7];
            if y1 < 0.0 {
                y2 -= Self::ONE_BIT;
                y1 += 1.0;
            }
            self.xdbl[6] = y1;
            
            y3 = self.xdbl[3] - self.xdbl[8];
            if y2 < 0.0 {
                y3 -= Self::ONE_BIT;
                y2 += 1.0;
            }
            self.xdbl[7] = y2;
            
            y1 = self.xdbl[4] - self.xdbl[9];
            if y3 < 0.0 {
                y1 -= Self::ONE_BIT;
                y3 += 1.0;
            }
            self.xdbl[8] = y3;
            
            y2 = self.xdbl[5] - self.xdbl[10];
            if y1 < 0.0 {
                y2 -= Self::ONE_BIT;
                y1 += 1.0;
            }
            self.xdbl[9] = y1;
            
            y3 = self.xdbl[6] - self.xdbl[11];
            if y2 < 0.0 {
                y3 -= Self::ONE_BIT;
                y2 += 1.0;
            }
            self.xdbl[10] = y2;
            
            if y3 < 0.0 {
                carry = Self::ONE_BIT;
                y3 += 1.0;
            } else {
                carry = 0.0;
            }
            self.xdbl[11] = y3;
            k += 12;
        }

        kmax = self.pr as i32;
        while k < kmax {
            y1 = self.xdbl[jr as usize] - self.xdbl[ir as usize];
            y2 = y1 - carry;
            if y2 < 0.0 {
                carry = Self::ONE_BIT;
                y2 += 1.0;
            } else {
                carry = 0.0;
            }
            self.xdbl[ir as usize] = y2;
            self.ydbl[ir as usize] = y2 + Self::SHIFT;
            ir = Self::NEXT[ir as usize] as u32;
            jr = Self::NEXT[jr as usize] as u32;
            k += 1;
        }

        self.ydbl[ir as usize] = self.xdbl[ir as usize] + Self::SHIFT;
        let mut k = Self::NEXT[ir as usize];
        while k > 0 {
            self.ydbl[k as usize] = self.xdbl[k as usize] + Self::SHIFT;
            k = Self::NEXT[k as usize];
        }

        k = 0;
        m = 0;
        while k < 12 {
            x = self.xdbl[k as usize];
            y2 = self.ydbl[k as usize] - Self::SHIFT;
            if y2 > x {
                y2 -= Self::SONE_BIT;
            }
            y1 = (x - y2) * Self::SBASE;
            self.xflt[m as usize] = y1 as f32;
            m += 1;
            self.xflt[m as usize] = y2 as f32;
            m += 1;
            k += 1;
        }

        self.ir = ir;
        self.is = 2 * ir;
        self.is_old = 2 * ir;
        self.jr = jr;
        self.carry = carry;
    }
}

impl RngState for RanlxsState {
    fn increment_state(&mut self) {
        self.increment_state()
    }

    fn get_double(&mut self) -> f64 {
        let is = Self::SNEXT[self.is as usize] as u32;
        self.is = is;
        if is == self.is_old {
            self.increment_state();
        }
        self.xflt[self.is as usize] as f64
    }

    fn get(&mut self) -> u64 {
        (self.get_double() * 16777216.0) as u64
    }

    fn set_lux(&mut self, s: u64, luxury: u32) {
        let mut ibit = 0;
        let mut jbit = 18;
        let mut xbit = [0; 31];
        let mut seed = if s == 0 { 1 } else { s } as i64;
        let mut i = (seed as u64 & 0x7FFFFFFF) as i32;

        for k in 0..31 {
            xbit[k] = i % 2;
            i /= 2;
        }

        for k in 0..12 {
            let mut x = 0.0;
            for m in 1..=48 {
                let y = xbit[ibit] as f64;
                x += x + y;
                xbit[ibit] = (xbit[ibit] + xbit[jbit]) % 2;
                ibit = (ibit + 1) % 31;
                jbit = (jbit + 1) % 31;
            }
            self.xdbl[k] = Self::ONE_BIT * x;
        }

        self.carry = 0.0;
        self.ir = 0;
        self.jr = 7;
        self.is = 23;
        self.is_old = 0;
        self.pr = luxury;
    }
}

pub fn ranlxs0_set(state: &mut dyn RngState, s: u64) {
    state.set_lux(s, 109);
}

pub fn ranlxs1_set(state: &mut dyn RngState, s: u64) {
    state.set_lux(s, 202);
}

pub fn ranlxs2_set(state: &mut dyn RngState, s: u64) {
    state.set_lux(s, 397);
}

pub fn ranlxs_get(state: &mut dyn RngState) -> u64 {
    state.get()
}

pub fn ranlxs_get_double(state: &mut dyn RngState) -> f64 {
    state.get_double()
}

pub static RANLXS0_TYPE: GslRngType = GslRngType {
    name: CString::new("ranlxs0").unwrap(),
    max: 0xFFFFFF,
    min: 0,
    size: std::mem::size_of::<RanlxsState>(),
    set: Some(ranlxs0_set),
    get: Some(ranlxs_get),
    get_double: Some(ranlxs_get_double),
};

pub static RANLXS1_TYPE: GslRngType = GslRngType {
    name: CString::new("ranlxs1").unwrap(),
    max: 0xFFFFFF,
    min: 0,
    size: std::mem::size_of::<RanlxsState>(),
    set: Some(ranlxs1_set),
    get: Some(ranlxs_get),
    get_double: Some(ranlxs_get_double),
};

pub static RANLXS2_TYPE: GslRngType = GslRngType {
    name: CString::new("ranlxs2").unwrap(),
    max: 0xFFFFFF,
    min: 0,
    size: std::mem::size_of::<RanlxsState>(),
    set: Some(ranlxs2_set),
    get: Some(ranlxs_get),
    get_double: Some(ranlxs_get_double),
};

pub static GSL_RNG_RANLXS0: &GslRngType = &RANLXS0_TYPE;
pub static GSL_RNG_RANLXS1: &GslRngType = &RANLXS1_TYPE;
pub static GSL_RNG_RANLXS2: &GslRngType = &RANLXS2_TYPE;