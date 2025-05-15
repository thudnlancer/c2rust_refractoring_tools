use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct RanState {
    x: u64,
    y: u64,
    z: u64,
}

impl RanState {
    pub fn new(seed: u64) -> Self {
        let mut s = seed;
        if s % 0x7fffffff == 0 || s % 0x7fffff07 == 0 {
            s = 1;
        }

        let x = s % 0x7fffffff;
        let y = s % 0x7fffff07;
        let z = if x > y {
            x - y
        } else {
            0x7fffffff + x - y
        };

        Self { x, y, z }
    }

    pub fn get(&mut self) -> u64 {
        let r = 3399u64.wrapping_mul(self.x / 44488) as i64;
        let mut y = 48271u64.wrapping_mul(self.x % 44488).wrapping_sub(r as u64) as i64;
        if y < 0 {
            y = (y as u64).wrapping_add(0x7fffffff) as i64;
        }
        self.x = y as u64;

        let r = 3791u64.wrapping_mul(self.y / 52774) as i64;
        y = 40692u64.wrapping_mul(self.y % 52774).wrapping_sub(r as u64) as i64;
        if y < 0 {
            y = (y as u64).wrapping_add(0x7fffff07) as i64;
        }
        self.y = y as u64;

        self.z = if self.x > self.y {
            self.x - self.y
        } else {
            0x7fffffff + self.x - self.y
        };

        self.z
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }
}

pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: usize,
}

pub static GSL_RNG_FISHMAN2X: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"fishman2x\0") },
    max: 0x7fffffff - 1,
    min: 0,
    size: std::mem::size_of::<RanState>(),
};