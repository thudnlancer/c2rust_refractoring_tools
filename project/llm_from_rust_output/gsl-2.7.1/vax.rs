use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct GslRngType {
    name: &'static CStr,
    max: u64,
    min: u64,
    size: usize,
    set: fn(&mut VaxState, u64),
    get: fn(&mut VaxState) -> u64,
    get_double: fn(&mut VaxState) -> f64,
}

#[derive(Clone, Copy)]
pub struct VaxState {
    x: u64,
}

impl VaxState {
    fn get(&mut self) -> u64 {
        self.x = 69069u64.wrapping_mul(self.x).wrapping_add(1) & 0xFFFF_FFFF;
        self.x
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4_294_967_296.0
    }

    fn set(&mut self, s: u64) {
        self.x = s;
    }
}

static VAX_TYPE: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"vax\0") },
    max: 0xFFFF_FFFF,
    min: 0,
    size: std::mem::size_of::<VaxState>(),
    set: VaxState::set,
    get: VaxState::get,
    get_double: VaxState::get_double,
};

pub static GSL_RNG_VAX: &GslRngType = &VAX_TYPE;