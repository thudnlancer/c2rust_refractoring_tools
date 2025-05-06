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
pub struct ran1_state_t {
    pub x: u64,
    pub n: u64,
    pub shuffle: [u64; 32],
}
static mut m: i64 = 2147483647 as i32 as i64;
static mut a: i64 = 16807 as i32 as i64;
static mut q: i64 = 127773 as i32 as i64;
static mut r: i64 = 2836 as i32 as i64;
#[inline]
unsafe extern "C" fn ran1_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran1_state_t = vstate as *mut ran1_state_t;
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
    let mut j: u64 = ((*state).n)
        .wrapping_div((1 as i32 + 2147483646 as i32 / 32 as i32) as u64);
    (*state).n = (*state).shuffle[j as usize];
    (*state).shuffle[j as usize] = (*state).x;
    return (*state).n;
}
unsafe extern "C" fn ran1_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut x_max: libc::c_float = 1 as i32 as libc::c_float - 1.2e-7f32;
    let mut x: libc::c_float = ran1_get(vstate) as libc::c_float / 2147483647.0f32;
    if x > x_max {
        return x_max as libc::c_double;
    }
    return x as libc::c_double;
}
unsafe extern "C" fn ran1_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran1_state_t = vstate as *mut ran1_state_t;
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    i = 0 as i32;
    while i < 8 as i32 {
        let mut h: i64 = s.wrapping_div(q as u64) as i64;
        let mut t: i64 = (a as u64)
            .wrapping_mul(s.wrapping_sub((h * q) as u64))
            .wrapping_sub((h * r) as u64) as i64;
        if t < 0 as i32 as i64 {
            t += m;
        }
        s = t as u64;
        i += 1;
        i;
    }
    i = 32 as i32 - 1 as i32;
    while i >= 0 as i32 {
        let mut h_0: i64 = s.wrapping_div(q as u64) as i64;
        let mut t_0: i64 = (a as u64)
            .wrapping_mul(s.wrapping_sub((h_0 * q) as u64))
            .wrapping_sub((h_0 * r) as u64) as i64;
        if t_0 < 0 as i32 as i64 {
            t_0 += m;
        }
        s = t_0 as u64;
        (*state).shuffle[i as usize] = s;
        i -= 1;
        i;
    }
    (*state).x = s;
    (*state).n = s;
}
static mut ran1_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran1\0" as *const u8 as *const i8,
        max: 2147483646 as i32 as u64,
        min: 1 as i32 as u64,
        size: ::core::mem::size_of::<ran1_state_t>() as u64,
        set: Some(ran1_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ran1_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ran1_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ran1: *const gsl_rng_type = unsafe {
    &ran1_type as *const gsl_rng_type
};