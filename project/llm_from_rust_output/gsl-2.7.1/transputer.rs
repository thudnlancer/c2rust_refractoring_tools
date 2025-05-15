use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut TransputerState, u64),
    pub get: fn(&mut TransputerState) -> u64,
    pub get_double: fn(&mut TransputerState) -> f64,
}

#[derive(Clone)]
pub struct TransputerState {
    pub x: u64,
}

impl TransputerState {
    pub fn get(&mut self) -> u64 {
        self.x = (1664525u64).wrapping_mul(self.x) & 0xffffffffu64;
        self.x
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0f64
    }

    pub fn set(&mut self, s: u64) {
        let seed = if s == 0 { 1 } else { s };
        self.x = seed;
    }
}

pub fn transputer_set(state: &mut TransputerState, s: u64) {
    state.set(s);
}

pub fn transputer_get(state: &mut TransputerState) -> u64 {
    state.get()
}

pub fn transputer_get_double(state: &mut TransputerState) -> f64 {
    state.get_double()
}

pub static GSL_RNG_TRANSPUTER: GslRngType = GslRngType {
    name: "transputer",
    max: 0xffffffffu64,
    min: 1u64,
    size: std::mem::size_of::<TransputerState>(),
    set: transputer_set,
    get: transputer_get,
    get_double: transputer_get_double,
};