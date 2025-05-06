#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const i8,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> ()>,
    pub get: Option<unsafe extern "C" fn(*mut libc::c_void) -> u64>,
    pub get_double: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taus113_state_t {
    pub z1: u64,
    pub z2: u64,
    pub z3: u64,
    pub z4: u64,
}
#[inline]
unsafe extern "C" fn taus113_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut taus113_state_t = vstate as *mut taus113_state_t;
    let mut b1: u64 = 0;
    let mut b2: u64 = 0;
    let mut b3: u64 = 0;
    let mut b4: u64 = 0;
    b1 = ((*state).z1 << 6 as u64 & 0xffffffff as u64 ^ (*state).z1) >> 13 as u64;
    (*state).z1 = ((*state).z1 & 4294967294 as u64) << 18 as u64 & 0xffffffff as u64
        ^ b1;
    b2 = ((*state).z2 << 2 as u64 & 0xffffffff as u64 ^ (*state).z2) >> 27 as u64;
    (*state).z2 = ((*state).z2 & 4294967288 as u64) << 2 as u64 & 0xffffffff as u64 ^ b2;
    b3 = ((*state).z3 << 13 as u64 & 0xffffffff as u64 ^ (*state).z3) >> 21 as u64;
    (*state).z3 = ((*state).z3 & 4294967280 as u64) << 7 as u64 & 0xffffffff as u64 ^ b3;
    b4 = ((*state).z4 << 3 as u64 & 0xffffffff as u64 ^ (*state).z4) >> 12 as u64;
    (*state).z4 = ((*state).z4 & 4294967168 as u64) << 13 as u64 & 0xffffffff as u64
        ^ b4;
    return (*state).z1 ^ (*state).z2 ^ (*state).z3 ^ (*state).z4;
}
unsafe extern "C" fn taus113_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return taus113_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn taus113_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut taus113_state_t = vstate as *mut taus113_state_t;
    if s == 0 {
        s = 1 as u64;
    }
    (*state).z1 = (69069 as u64).wrapping_mul(s) & 0xffffffff as u64;
    if (*state).z1 < 2 as u64 {
        (*state).z1 = ((*state).z1).wrapping_add(2 as u64);
    }
    (*state).z2 = (69069 as u64).wrapping_mul((*state).z1) & 0xffffffff as u64;
    if (*state).z2 < 8 as u64 {
        (*state).z2 = ((*state).z2).wrapping_add(8 as u64);
    }
    (*state).z3 = (69069 as u64).wrapping_mul((*state).z2) & 0xffffffff as u64;
    if (*state).z3 < 16 as u64 {
        (*state).z3 = ((*state).z3).wrapping_add(16 as u64);
    }
    (*state).z4 = (69069 as u64).wrapping_mul((*state).z3) & 0xffffffff as u64;
    if (*state).z4 < 128 as u64 {
        (*state).z4 = ((*state).z4).wrapping_add(128 as u64);
    }
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
}
static mut taus113_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"taus113\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<taus113_state_t>() as u64,
        set: Some(taus113_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(taus113_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            taus113_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_taus113: *const gsl_rng_type = unsafe {
    &taus113_type as *const gsl_rng_type
};