use std::ffi::CStr;

pub type SizeT = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: SizeT,
    pub set: fn(&mut R250State, u64),
    pub get: fn(&mut R250State) -> u64,
    pub get_double: fn(&mut R250State) -> f64,
}

#[derive(Clone)]
pub struct R250State {
    pub i: i32,
    pub x: [u64; 250],
}

impl R250State {
    fn get(&mut self) -> u64 {
        let j = if self.i >= 147 {
            self.i - 147
        } else {
            self.i + 103
        };
        
        let k = self.x[self.i as usize] ^ self.x[j as usize];
        self.x[self.i as usize] = k;
        
        if self.i >= 249 {
            self.i = 0;
        } else {
            self.i += 1;
        }
        
        k
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4_294_967_296.0
    }

    fn set(&mut self, mut s: u64) {
        if s == 0 {
            s = 1;
        }
        
        self.i = 0;
        for i in 0..250 {
            s = 69069_u64.wrapping_mul(s) & 0xFFFF_FFFF;
            self.x[i] = s;
        }
        
        let mut msb = 0x8000_0000;
        let mut mask = 0xFFFF_FFFF;
        
        for i in 0..32 {
            let k = 7 * i + 3;
            self.x[k as usize] &= mask;
            self.x[k as usize] |= msb;
            mask >>= 1;
            msb >>= 1;
        }
    }
}

pub static GSL_RNG_R250: GslRngType = GslRngType {
    name: "r250",
    max: 0xFFFF_FFFF,
    min: 0,
    size: std::mem::size_of::<R250State>(),
    set: R250State::set,
    get: R250State::get,
    get_double: R250State::get_double,
};