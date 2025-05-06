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
pub struct ran2_state_t {
    pub x: u64,
    pub y: u64,
    pub n: u64,
    pub shuffle: [u64; 32],
}
static mut m1: i64 = 2147483563 as i32 as i64;
static mut a1: i64 = 40014 as i32 as i64;
static mut q1: i64 = 53668 as i32 as i64;
static mut r1: i64 = 12211 as i32 as i64;
static mut m2: i64 = 2147483399 as i32 as i64;
static mut a2: i64 = 40692 as i32 as i64;
static mut q2: i64 = 52774 as i32 as i64;
static mut r2: i64 = 3791 as i32 as i64;
#[inline]
unsafe extern "C" fn ran2_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran2_state_t = vstate as *mut ran2_state_t;
    let x: u64 = (*state).x;
    let y: u64 = (*state).y;
    let mut h1: i64 = x.wrapping_div(q1 as u64) as i64;
    let mut t1: i64 = (a1 as u64)
        .wrapping_mul(x.wrapping_sub((h1 * q1) as u64))
        .wrapping_sub((h1 * r1) as u64) as i64;
    let mut h2: i64 = y.wrapping_div(q2 as u64) as i64;
    let mut t2: i64 = (a2 as u64)
        .wrapping_mul(y.wrapping_sub((h2 * q2) as u64))
        .wrapping_sub((h2 * r2) as u64) as i64;
    if t1 < 0 as i32 as i64 {
        t1 += m1;
    }
    if t2 < 0 as i32 as i64 {
        t2 += m2;
    }
    (*state).x = t1 as u64;
    (*state).y = t2 as u64;
    let mut j: u64 = ((*state).n)
        .wrapping_div((1 as i32 + 2147483562 as i32 / 32 as i32) as u64);
    let mut delta: i64 = ((*state).shuffle[j as usize]).wrapping_sub(t2 as u64) as i64;
    if delta < 1 as i32 as i64 {
        delta += m1 - 1 as i32 as i64;
    }
    (*state).n = delta as u64;
    (*state).shuffle[j as usize] = t1 as u64;
    return (*state).n;
}
unsafe extern "C" fn ran2_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut x_max: libc::c_float = 1 as i32 as libc::c_float - 1.2e-7f32;
    let mut x: libc::c_float = ran2_get(vstate) as libc::c_float / 2147483563.0f32;
    if x > x_max {
        return x_max as libc::c_double;
    }
    return x as libc::c_double;
}
unsafe extern "C" fn ran2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran2_state_t = vstate as *mut ran2_state_t;
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).y = s;
    i = 0 as i32;
    while i < 8 as i32 {
        let mut h: i64 = s.wrapping_div(q1 as u64) as i64;
        let mut t: i64 = (a1 as u64)
            .wrapping_mul(s.wrapping_sub((h * q1) as u64))
            .wrapping_sub((h * r1) as u64) as i64;
        if t < 0 as i32 as i64 {
            t += m1;
        }
        s = t as u64;
        i += 1;
        i;
    }
    i = 32 as i32 - 1 as i32;
    while i >= 0 as i32 {
        let mut h_0: i64 = s.wrapping_div(q1 as u64) as i64;
        let mut t_0: i64 = (a1 as u64)
            .wrapping_mul(s.wrapping_sub((h_0 * q1) as u64))
            .wrapping_sub((h_0 * r1) as u64) as i64;
        if t_0 < 0 as i32 as i64 {
            t_0 += m1;
        }
        s = t_0 as u64;
        (*state).shuffle[i as usize] = s;
        i -= 1;
        i;
    }
    (*state).x = s;
    (*state).n = s;
}
static mut ran2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran2\0" as *const u8 as *const i8,
        max: 2147483562 as i32 as u64,
        min: 1 as i32 as u64,
        size: ::core::mem::size_of::<ran2_state_t>() as u64,
        set: Some(ran2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ran2_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ran2_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ran2: *const gsl_rng_type = unsafe {
    &ran2_type as *const gsl_rng_type
};