use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: fn(&mut RanState, u64),
    pub get: fn(&mut RanState) -> u64,
    pub get_double: fn(&mut RanState) -> f64,
}

#[derive(Clone, Copy)]
pub struct RanState {
    pub x: u64,
}

impl RanState {
    fn get(&mut self) -> u64 {
        self.x = (1566083941u64).wrapping_mul(self.x) & 0xffffffffu64;
        self.x
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }

    fn set(&mut self, s: u64) {
        let seed = if s == 0 { 1 } else { s };
        self.x = seed & 0xffffffffu64;
    }
}

pub static GSL_RNG_WATERMAN14: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"waterman14\0") },
    max: 0xffffffffu64,
    min: 1u64,
    size: std::mem::size_of::<RanState>(),
    set: RanState::set,
    get: RanState::get,
    get_double: RanState::get_double,
};