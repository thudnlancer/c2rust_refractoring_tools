use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: fn(&mut RandState, u64),
    pub get: fn(&mut RandState) -> u64,
    pub get_double: fn(&mut RandState) -> f64,
}

#[derive(Default, Clone, Copy)]
pub struct RandState {
    pub x: u64,
}

impl RandState {
    fn get(&mut self) -> u64 {
        self.x = (1103515245u64)
            .wrapping_mul(self.x)
            .wrapping_add(12345)
            & 0x7fffffff;
        self.x
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483648.0
    }

    fn set(&mut self, s: u64) {
        self.x = s;
    }
}

pub static GSL_RNG_RAND: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"rand\0") },
    max: 0x7fffffff,
    min: 0,
    size: std::mem::size_of::<RandState>(),
    set: RandState::set,
    get: RandState::get,
    get_double: RandState::get_double,
};