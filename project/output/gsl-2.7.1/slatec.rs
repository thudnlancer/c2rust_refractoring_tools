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
pub struct slatec_state_t {
    pub x0: i64,
    pub x1: i64,
}
static mut P: i64 = 4194304 as i32 as i64;
static mut a1: i64 = 1536 as i32 as i64;
static mut a0: i64 = 1029 as i32 as i64;
static mut a1ma0: i64 = 507 as i32 as i64;
static mut c: i64 = 1731 as i32 as i64;
#[inline]
unsafe extern "C" fn slatec_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut y0: i64 = 0;
    let mut y1: i64 = 0;
    let mut state: *mut slatec_state_t = vstate as *mut slatec_state_t;
    y0 = a0 * (*state).x0;
    y1 = a1 * (*state).x1 + a1ma0 * ((*state).x0 - (*state).x1) + y0;
    y0 = y0 + c;
    (*state).x0 = y0 % 2048 as i32 as i64;
    y1 = y1 + (y0 - (*state).x0) / 2048 as i32 as i64;
    (*state).x1 = y1 % 2048 as i32 as i64;
    return ((*state).x1 * 2048 as i32 as i64 + (*state).x0) as u64;
}
unsafe extern "C" fn slatec_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return slatec_get(vstate) as libc::c_double / 4194304.0f64;
}
unsafe extern "C" fn slatec_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut slatec_state_t = vstate as *mut slatec_state_t;
    s = s.wrapping_rem(8 as i32 as u64);
    s = s.wrapping_mul((P / 8 as i32 as i64) as u64);
    (*state).x0 = s.wrapping_rem(2048 as i32 as u64) as i64;
    (*state).x1 = s.wrapping_sub((*state).x0 as u64).wrapping_div(2048 as i32 as u64)
        as i64;
}
static mut slatec_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"slatec\0" as *const u8 as *const i8,
        max: 4194303 as i32 as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<slatec_state_t>() as u64,
        set: Some(slatec_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(slatec_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            slatec_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_slatec: *const gsl_rng_type = unsafe {
    &slatec_type as *const gsl_rng_type
};