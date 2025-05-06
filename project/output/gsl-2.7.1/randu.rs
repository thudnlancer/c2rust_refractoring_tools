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
pub struct randu_state_t {
    pub x: u64,
}
static mut a: i64 = 65539 as i32 as i64;
#[inline]
unsafe extern "C" fn randu_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut randu_state_t = vstate as *mut randu_state_t;
    (*state).x = (a as u64).wrapping_mul((*state).x) & 0x7fffffff as u64;
    return (*state).x;
}
unsafe extern "C" fn randu_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return randu_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn randu_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut randu_state_t = vstate as *mut randu_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).x = s;
}
static mut randu_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"randu\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 1 as i32 as u64,
        size: ::core::mem::size_of::<randu_state_t>() as u64,
        set: Some(randu_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(randu_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            randu_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_randu: *const gsl_rng_type = unsafe {
    &randu_type as *const gsl_rng_type
};