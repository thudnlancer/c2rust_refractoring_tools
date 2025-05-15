use std::ffi::CStr;
use std::marker::PhantomData;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    name: &'static CStr,
    max: u64,
    min: u64,
    size: size_t,
    set: fn(&mut RngState, u64),
    get: fn(&mut RngState) -> u64,
    get_double: fn(&mut RngState) -> f64,
}

#[derive(Clone)]
pub struct RngState {
    x: u64,
}

impl RngState {
    fn get(&mut self) -> u64 {
        self.x = self.x.wrapping_mul(self.x.wrapping_add(1)) & 0xffffffff;
        self.x
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }

    fn set(&mut self, s: u64) {
        let diff = s
            .wrapping_rem(4)
            .wrapping_sub(2)
            .wrapping_rem(0xffffffff);
        if diff != 0 {
            self.x = s.wrapping_sub(diff) & 0xffffffff;
        } else {
            self.x = s & 0xffffffff;
        }
    }
}

pub static GSL_RNG_COVEYOU: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"coveyou\0") },
    max: 0xffffffff - 1,
    min: 2,
    size: std::mem::size_of::<RngState>(),
    set: RngState::set,
    get: RngState::get,
    get_double: RngState::get_double,
};