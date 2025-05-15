use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut Gfsr4State, u64)>,
    pub get: Option<fn(&mut Gfsr4State) -> u64>,
    pub get_double: Option<fn(&mut Gfsr4State) -> f64>,
}

#[derive(Clone)]
pub struct Gfsr4State {
    pub nd: i32,
    pub ra: [u64; 16384],
}

impl Gfsr4State {
    fn get(&mut self) -> u64 {
        self.nd = (self.nd + 1) & 16383;
        self.ra[self.nd as usize] = self.ra[((self.nd + (16383 + 1 - 471)) & 16383) as usize]
            ^ self.ra[((self.nd + (16383 + 1 - 1586)) & 16383 as usize]
            ^ self.ra[((self.nd + (16383 + 1 - 6988)) & 16383) as usize]
            ^ self.ra[((self.nd + (16383 + 1 - 9689)) & 16383) as usize];
        self.ra[self.nd as usize]
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }

    fn set(&mut self, mut s: u64) {
        let mut msb: u64 = 0x80000000;
        let mut mask: u64 = 0xffffffff;

        if s == 0 {
            s = 4357;
        }

        for i in 0..=16383 {
            let mut t: u64 = 0;
            let mut bit = msb;
            for _ in 0..32 {
                s = (69069 as u64).wrapping_mul(s) & 0xffffffff;
                if s & msb != 0 {
                    t |= bit;
                }
                bit >>= 1;
            }
            self.ra[i as usize] = t;
        }

        for i in 0..32 {
            let k = 7 + i * 3;
            self.ra[k as usize] &= mask;
            self.ra[k as usize] |= msb;
            mask >>= 1;
            msb >>= 1;
        }
        self.nd = 32;
    }
}

pub static GSL_RNG_GFSR4: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"gfsr4\0") },
    max: 0xffffffff,
    min: 0,
    size: std::mem::size_of::<Gfsr4State>(),
    set: Some(Gfsr4State::set),
    get: Some(Gfsr4State::get),
    get_double: Some(Gfsr4State::get_double),
};