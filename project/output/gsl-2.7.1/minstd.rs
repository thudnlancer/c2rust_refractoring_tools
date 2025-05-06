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
pub struct minstd_state_t {
    pub x: u64,
}
static mut m: i64 = 2147483647 as i32 as i64;
static mut a: i64 = 16807 as i32 as i64;
static mut q: i64 = 127773 as i32 as i64;
static mut r: i64 = 2836 as i32 as i64;
#[inline]
unsafe extern "C" fn minstd_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut minstd_state_t = vstate as *mut minstd_state_t;
    let x: u64 = (*state).x;
    let h: i64 = x.wrapping_div(q as u64) as i64;
    let t: i64 = (a as u64)
        .wrapping_mul(x.wrapping_sub((h * q) as u64))
        .wrapping_sub((h * r) as u64) as i64;
    if t < 0 as i32 as i64 {
        (*state).x = (t + m) as u64;
    } else {
        (*state).x = t as u64;
    }
    return (*state).x;
}
unsafe extern "C" fn minstd_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return minstd_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn minstd_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut minstd_state_t = vstate as *mut minstd_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).x = s;
}
static mut minstd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"minstd\0" as *const u8 as *const i8,
        max: 2147483646 as i32 as u64,
        min: 1 as i32 as u64,
        size: ::core::mem::size_of::<minstd_state_t>() as u64,
        set: Some(minstd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(minstd_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            minstd_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_minstd: *const gsl_rng_type = unsafe {
    &minstd_type as *const gsl_rng_type
};