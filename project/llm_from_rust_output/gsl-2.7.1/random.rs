use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_double, c_void};

pub type size_t = c_ulong;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: c_ulong,
    pub min: c_ulong,
    pub size: size_t,
    pub set: Option<fn(&mut dyn RngState, c_ulong)>,
    pub get: Option<fn(&mut dyn RngState) -> c_ulong>,
    pub get_double: Option<fn(&mut dyn RngState) -> c_double>,
}

pub trait RngState {
    fn as_mut_ptr(&mut self) -> *mut c_void;
}

#[derive(Clone)]
pub struct Random8State {
    pub x: c_long,
}

impl RngState for Random8State {
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

#[derive(Clone)]
pub struct Random32State {
    pub i: c_int,
    pub j: c_int,
    pub x: [c_long; 7],
}

impl RngState for Random32State {
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

#[derive(Clone)]
pub struct Random64State {
    pub i: c_int,
    pub j: c_int,
    pub x: [c_long; 15],
}

impl RngState for Random64State {
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

#[derive(Clone)]
pub struct Random128State {
    pub i: c_int,
    pub j: c_int,
    pub x: [c_long; 31],
}

impl RngState for Random128State {
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

#[derive(Clone)]
pub struct Random256State {
    pub i: c_int,
    pub j: c_int,
    pub x: [c_long; 63],
}

impl RngState for Random256State {
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

fn random8_get(state: &mut Random8State) -> c_ulong {
    state.x = ((1103515245 as c_long * state.x + 12345) as c_ulong & 0x7fffffff) as c_long;
    state.x as c_ulong
}

fn random_get(i: &mut c_int, j: &mut c_int, n: c_int, x: &mut [c_long]) -> c_long {
    let k;
    x[*i as usize] += x[*j as usize];
    k = (x[*i as usize] >> 1) & 0x7fffffff;
    
    *i += 1;
    if *i == n {
        *i = 0;
    }
    
    *j += 1;
    if *j == n {
        *j = 0;
    }
    
    k
}

fn random32_get(state: &mut Random32State) -> c_ulong {
    random_get(&mut state.i, &mut state.j, 7, &mut state.x) as c_ulong
}

fn random64_get(state: &mut Random64State) -> c_ulong {
    random_get(&mut state.i, &mut state.j, 15, &mut state.x) as c_ulong
}

fn random128_get(state: &mut Random128State) -> c_ulong {
    random_get(&mut state.i, &mut state.j, 31, &mut state.x) as c_ulong
}

fn random256_get(state: &mut Random256State) -> c_ulong {
    random_get(&mut state.i, &mut state.j, 63, &mut state.x) as c_ulong
}

fn random8_get_double(state: &mut Random8State) -> c_double {
    random8_get(state) as c_double / 2147483648.0
}

fn random32_get_double(state: &mut Random32State) -> c_double {
    random32_get(state) as c_double / 2147483648.0
}

fn random64_get_double(state: &mut Random64State) -> c_double {
    random64_get(state) as c_double / 2147483648.0
}

fn random128_get_double(state: &mut Random128State) -> c_double {
    random128_get(state) as c_double / 2147483648.0
}

fn random256_get_double(state: &mut Random256State) -> c_double {
    random256_get(state) as c_double / 2147483648.0
}

fn random8_bsd_set(state: &mut Random8State, s: c_ulong) {
    let mut seed = s;
    if seed == 0 {
        seed = 1;
    }
    state.x = seed as c_long;
}

fn bsd_initialize(x: &mut [c_long], s: c_ulong) {
    let mut seed = s;
    if seed == 0 {
        seed = 1;
    }
    
    x[0] = seed as c_long;
    for i in 1..x.len() {
        x[i] = 1103515245 * x[i-1] + 12345;
    }
}

fn random32_bsd_set(state: &mut Random32State, s: c_ulong) {
    bsd_initialize(&mut state.x, s);
    state.i = 3;
    state.j = 0;
    
    for _ in 0..10*7 {
        random32_get(state);
    }
}

fn random64_bsd_set(state: &mut Random64State, s: c_ulong) {
    bsd_initialize(&mut state.x, s);
    state.i = 1;
    state.j = 0;
    
    for _ in 0..10*15 {
        random64_get(state);
    }
}

fn random128_bsd_set(state: &mut Random128State, s: c_ulong) {
    bsd_initialize(&mut state.x, s);
    state.i = 3;
    state.j = 0;
    
    for _ in 0..10*31 {
        random128_get(state);
    }
}

fn random256_bsd_set(state: &mut Random256State, s: c_ulong) {
    bsd_initialize(&mut state.x, s);
    state.i = 1;
    state.j = 0;
    
    for _ in 0..10*63 {
        random256_get(state);
    }
}

// Similar implementations for glibc2 and libc5 variants would follow
// with their respective initialization functions

// Static instances of GslRngType would be defined here
// following the same pattern as the original code but using the safe Rust types