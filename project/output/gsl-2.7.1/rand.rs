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
pub struct rand_state_t {
    pub x: u64,
}
#[inline]
unsafe extern "C" fn rand_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut rand_state_t = vstate as *mut rand_state_t;
    (*state).x = (1103515245 as i32 as u64)
        .wrapping_mul((*state).x)
        .wrapping_add(12345 as i32 as u64) & 0x7fffffff as u64;
    return (*state).x;
}
unsafe extern "C" fn rand_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return rand_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn rand_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut rand_state_t = vstate as *mut rand_state_t;
    (*state).x = s;
}
static mut rand_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"rand\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<rand_state_t>() as u64,
        set: Some(rand_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(rand_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            rand_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_rand: *const gsl_rng_type = unsafe {
    &rand_type as *const gsl_rng_type
};