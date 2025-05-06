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
pub struct mrg_state_t {
    pub x1: i64,
    pub x2: i64,
    pub x3: i64,
    pub x4: i64,
    pub x5: i64,
}
static mut m: i64 = 2147483647 as i32 as i64;
static mut a1: i64 = 107374182 as i32 as i64;
static mut q1: i64 = 20 as i32 as i64;
static mut r1: i64 = 7 as i32 as i64;
static mut a5: i64 = 104480 as i32 as i64;
static mut q5: i64 = 20554 as i32 as i64;
static mut r5: i64 = 1727 as i32 as i64;
#[inline]
unsafe extern "C" fn mrg_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut mrg_state_t = vstate as *mut mrg_state_t;
    let mut p1: i64 = 0;
    let mut h1: i64 = 0;
    let mut p5: i64 = 0;
    let mut h5: i64 = 0;
    h5 = (*state).x5 / q5;
    p5 = a5 * ((*state).x5 - h5 * q5) - h5 * r5;
    if p5 > 0 as i32 as i64 {
        p5 -= m;
    }
    h1 = (*state).x1 / q1;
    p1 = a1 * ((*state).x1 - h1 * q1) - h1 * r1;
    if p1 < 0 as i32 as i64 {
        p1 += m;
    }
    (*state).x5 = (*state).x4;
    (*state).x4 = (*state).x3;
    (*state).x3 = (*state).x2;
    (*state).x2 = (*state).x1;
    (*state).x1 = p1 + p5;
    if (*state).x1 < 0 as i32 as i64 {
        (*state).x1 += m;
    }
    return (*state).x1 as u64;
}
unsafe extern "C" fn mrg_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return mrg_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn mrg_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut mrg_state_t = vstate as *mut mrg_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x1 = s.wrapping_rem(m as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x2 = s.wrapping_rem(m as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x3 = s.wrapping_rem(m as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x4 = s.wrapping_rem(m as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x5 = s.wrapping_rem(m as u64) as i64;
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
}
static mut mrg_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mrg\0" as *const u8 as *const i8,
        max: 2147483646 as i32 as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<mrg_state_t>() as u64,
        set: Some(mrg_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(mrg_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            mrg_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_mrg: *const gsl_rng_type = unsafe {
    &mrg_type as *const gsl_rng_type
};