use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut RanState, u64),
    pub get: fn(&mut RanState) -> u64,
    pub get_double: fn(&mut RanState) -> f64,
}

#[derive(Clone)]
pub struct RanState {
    pub x: u64,
}

impl RanState {
    fn new(s: u64) -> Self {
        let seed = if s == 0 { 1 } else { s };
        Self {
            x: seed & 0xffffffff,
        }
    }

    fn get(&mut self) -> u64 {
        self.x = (1812433253u64).wrapping_mul(self.x) & 0xffffffff;
        self.x
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

pub static GSL_RNG_BOROSH13: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"borosh13\0") },
    max: 0xffffffff,
    min: 1,
    size: std::mem::size_of::<RanState>(),
    set: |state, s| *state = RanState::new(s),
    get: |state| state.get(),
    get_double: |state| state.get_double(),
};