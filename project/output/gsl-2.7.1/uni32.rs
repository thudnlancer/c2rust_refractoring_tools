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
pub struct uni32_state_t {
    pub i: i32,
    pub j: i32,
    pub m: [u64; 17],
}
static mut m1: u64 = 2147483647 as i32 as u64;
static mut m2: u64 = 65536 as i32 as u64;
#[inline]
unsafe extern "C" fn uni32_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut uni32_state_t = vstate as *mut uni32_state_t;
    let i: i64 = (*state).i as i64;
    let j: i64 = (*state).j as i64;
    let mut k: i64 = ((*state).m[i as usize]).wrapping_sub((*state).m[j as usize])
        as i64;
    if k < 0 as i32 as i64 {
        k = (k as u64).wrapping_add(m1) as i64 as i64;
    }
    (*state).m[j as usize] = k as u64;
    if i == 0 as i32 as i64 {
        (*state).i = 16 as i32;
    } else {
        (*state).i -= 1;
        (*state).i;
    }
    if j == 0 as i32 as i64 {
        (*state).j = 16 as i32;
    } else {
        (*state).j -= 1;
        (*state).j;
    }
    return k as u64;
}
unsafe extern "C" fn uni32_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return uni32_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn uni32_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut seed: i64 = 0;
    let mut k0: i64 = 0;
    let mut k1: i64 = 0;
    let mut j0: i64 = 0;
    let mut j1: i64 = 0;
    let mut i: i32 = 0;
    let mut state: *mut uni32_state_t = vstate as *mut uni32_state_t;
    seed = (if s < m1 { s } else { m1 }) as i64;
    seed
        -= (if seed % 2 as i32 as i64 == 0 as i32 as i64 { 1 as i32 } else { 0 as i32 })
            as i64;
    k0 = (9069 as i32 as u64).wrapping_rem(m2) as i64;
    k1 = (9069 as i32 as u64).wrapping_div(m2) as i64;
    j0 = (seed as u64).wrapping_rem(m2) as i64;
    j1 = (seed as u64).wrapping_div(m2) as i64;
    i = 0 as i32;
    while i < 17 as i32 {
        seed = j0 * k0;
        j1 = (seed as u64)
            .wrapping_div(m2)
            .wrapping_add((j0 * k1) as u64)
            .wrapping_add((j1 * k0) as u64)
            .wrapping_rem(m2.wrapping_div(2 as i32 as u64)) as i64;
        j0 = (seed as u64).wrapping_rem(m2) as i64;
        (*state).m[i as usize] = (j0 as u64).wrapping_add(m2.wrapping_mul(j1 as u64));
        i += 1;
        i;
    }
    (*state).i = 4 as i32;
    (*state).j = 16 as i32;
}
static mut uni32_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"uni32\0" as *const u8 as *const i8,
        max: 2147483646 as i32 as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<uni32_state_t>() as u64,
        set: Some(uni32_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(uni32_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            uni32_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_uni32: *const gsl_rng_type = unsafe {
    &uni32_type as *const gsl_rng_type
};