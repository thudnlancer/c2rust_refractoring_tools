use std::f64;

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut Rand48State, u64),
    pub get: fn(&mut Rand48State) -> u64,
    pub get_double: fn(&mut Rand48State) -> f64,
}

#[derive(Copy, Clone)]
pub struct Rand48State {
    pub x0: u16,
    pub x1: u16,
    pub x2: u16,
}

const A0: u16 = 0xe66d;
const A1: u16 = 0xdeec;
const A2: u16 = 0x5;
const C0: u16 = 0xb;

fn rand48_advance(state: &mut Rand48State) {
    let x0 = state.x0 as u64;
    let x1 = state.x1 as u64;
    let x2 = state.x2 as u64;
    
    let mut a = (A0 as u64).wrapping_mul(x0).wrapping_add(C0 as u64);
    state.x0 = (a & 0xffff) as u16;
    a >>= 16;
    
    a = a.wrapping_add((A0 as u64).wrapping_mul(x1).wrapping_add((A1 as u64).wrapping_mul(x0)));
    state.x1 = (a & 0xffff) as u16;
    a >>= 16;
    
    a = a.wrapping_add(
        (A0 as u64).wrapping_mul(x2)
        .wrapping_add((A1 as u64).wrapping_mul(x1))
        .wrapping_add((A2 as u64).wrapping_mul(x0));
    state.x2 = (a & 0xffff) as u16;
}

fn rand48_get(state: &mut Rand48State) -> u64 {
    rand48_advance(state);
    let x1 = state.x1 as u64;
    let x2 = state.x2 as u64;
    (x2 << 16).wrapping_add(x1)
}

fn rand48_get_double(state: &mut Rand48State) -> f64 {
    rand48_advance(state);
    f64::ldexp(state.x2 as f64, -16) 
        + f64::ldexp(state.x1 as f64, -32) 
        + f64::ldexp(state.x0 as f64, -48)
}

fn rand48_set(state: &mut Rand48State, s: u64) {
    if s == 0 {
        state.x0 = 0x330e;
        state.x1 = 0xabcd;
        state.x2 = 0x1234;
    } else {
        state.x0 = 0x330e;
        state.x1 = (s & 0xffff) as u16;
        state.x2 = ((s >> 16) & 0xffff) as u16;
    }
}

pub static GSL_RNG_RAND48: GslRngType = GslRngType {
    name: "rand48",
    max: 0xffffffff,
    min: 0,
    size: std::mem::size_of::<Rand48State>(),
    set: rand48_set,
    get: rand48_get,
    get_double: rand48_get_double,
};