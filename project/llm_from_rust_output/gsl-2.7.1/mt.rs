use std::ffi::CString;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut MtState, u64),
    pub get: fn(&mut MtState) -> u64,
    pub get_double: fn(&mut MtState) -> f64,
}

#[derive(Clone)]
pub struct MtState {
    pub mt: [u64; 624],
    pub mti: i32,
}

const UPPER_MASK: u64 = 0x80000000;
const LOWER_MASK: u64 = 0x7fffffff;

impl MtState {
    fn get(&mut self) -> u64 {
        if self.mti >= 624 {
            self.generate_numbers();
        }

        let mut k = self.mt[self.mti as usize];
        self.mti += 1;

        k ^= k >> 11;
        k ^= (k << 7) & 0x9d2c5680;
        k ^= (k << 15) & 0xefc60000;
        k ^= k >> 18;

        k
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }

    fn generate_numbers(&mut self) {
        for kk in 0..(624 - 397) {
            let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
            self.mt[kk] = self.mt[kk + 397] ^ (y >> 1) ^ if y & 1 != 0 { 0x9908b0df } else { 0 };
        }

        for kk in (624 - 397)..(624 - 1) {
            let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
            self.mt[kk] = self.mt[kk + 397 - 624] ^ (y >> 1) ^ if y & 1 != 0 { 0x9908b0df } else { 0 };
        }

        let y = (self.mt[624 - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
        self.mt[624 - 1] = self.mt[397 - 1] ^ (y >> 1) ^ if y & 1 != 0 { 0x9908b0df } else { 0 };

        self.mti = 0;
    }

    fn set(&mut self, mut s: u64) {
        if s == 0 {
            s = 4357;
        }

        self.mt[0] = s & 0xffffffff;
        for i in 1..624 {
            self.mt[i] = (1812433253u64)
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u64);
            self.mt[i] &= 0xffffffff;
        }
        self.mti = 624;
    }

    fn set_1999(&mut self, mut s: u64) {
        if s == 0 {
            s = 4357;
        }

        for i in 0..624 {
            self.mt[i] = s & 0xffff0000;
            s = 69069u64.wrapping_mul(s).wrapping_add(1) & 0xffffffff;
            self.mt[i] |= (s & 0xffff0000) >> 16;
            s = 69069u64.wrapping_mul(s).wrapping_add(1) & 0xffffffff;
        }
        self.mti = 624;
    }

    fn set_1998(&mut self, mut s: u64) {
        if s == 0 {
            s = 4357;
        }

        self.mt[0] = s & 0xffffffff;
        for i in 1..624 {
            self.mt[i] = (69069u64).wrapping_mul(self.mt[i - 1]) & 0xffffffff;
        }
        self.mti = 624;
    }
}

pub fn create_rng_type(name: &str, set_fn: fn(&mut MtState, u64)) -> GslRngType {
    GslRngType {
        name: CString::new(name).unwrap(),
        max: 0xffffffff,
        min: 0,
        size: std::mem::size_of::<MtState>(),
        set: set_fn,
        get: MtState::get,
        get_double: MtState::get_double,
    }
}

lazy_static! {
    pub static ref GSL_RNG_MT19937: GslRngType = create_rng_type("mt19937", MtState::set);
    pub static ref GSL_RNG_MT19937_1999: GslRngType = create_rng_type("mt19937_1999", MtState::set_1999);
    pub static ref GSL_RNG_MT19937_1998: GslRngType = create_rng_type("mt19937_1998", MtState::set_1998);
    pub static ref GSL_RNG_DEFAULT: GslRngType = GSL_RNG_MT19937.clone();
}

pub static GSL_RNG_DEFAULT_SEED: u64 = 0;