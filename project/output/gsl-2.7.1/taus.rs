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
pub struct taus_state_t {
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
}
#[inline]
unsafe extern "C" fn taus_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut taus_state_t = vstate as *mut taus_state_t;
    (*state).s1 = ((*state).s1 & 4294967294 as u64) << 12 as i32 & 0xffffffff as u64
        ^ ((*state).s1 << 13 as i32 & 0xffffffff as u64 ^ (*state).s1) >> 19 as i32;
    (*state).s2 = ((*state).s2 & 4294967288 as u64) << 4 as i32 & 0xffffffff as u64
        ^ ((*state).s2 << 2 as i32 & 0xffffffff as u64 ^ (*state).s2) >> 25 as i32;
    (*state).s3 = ((*state).s3 & 4294967280 as u64) << 17 as i32 & 0xffffffff as u64
        ^ ((*state).s3 << 3 as i32 & 0xffffffff as u64 ^ (*state).s3) >> 11 as i32;
    return (*state).s1 ^ (*state).s2 ^ (*state).s3;
}
unsafe extern "C" fn taus_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return taus_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn taus_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut taus_state_t = vstate as *mut taus_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).s1 = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).s2 = (69069 as i32 as u64).wrapping_mul((*state).s1) & 0xffffffff as u64;
    (*state).s3 = (69069 as i32 as u64).wrapping_mul((*state).s2) & 0xffffffff as u64;
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
}
unsafe extern "C" fn taus2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut taus_state_t = vstate as *mut taus_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).s1 = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    if (*state).s1 < 2 as i32 as u64 {
        (*state).s1 = ((*state).s1).wrapping_add(2 as u64);
    }
    (*state).s2 = (69069 as i32 as u64).wrapping_mul((*state).s1) & 0xffffffff as u64;
    if (*state).s2 < 8 as i32 as u64 {
        (*state).s2 = ((*state).s2).wrapping_add(8 as u64);
    }
    (*state).s3 = (69069 as i32 as u64).wrapping_mul((*state).s2) & 0xffffffff as u64;
    if (*state).s3 < 16 as i32 as u64 {
        (*state).s3 = ((*state).s3).wrapping_add(16 as u64);
    }
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
}
static mut taus_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"taus\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<taus_state_t>() as u64,
        set: Some(taus_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(taus_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            taus_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_taus: *const gsl_rng_type = unsafe {
    &taus_type as *const gsl_rng_type
};
static mut taus2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"taus2\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<taus_state_t>() as u64,
        set: Some(taus2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(taus_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            taus_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_taus2: *const gsl_rng_type = unsafe {
    &taus2_type as *const gsl_rng_type
};