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
pub struct uni_state_t {
    pub i: i32,
    pub j: i32,
    pub m: [u64; 17],
}
static mut m1: u32 = 32767 as i32 as u32;
static mut m2: u32 = 256 as i32 as u32;
#[inline]
unsafe extern "C" fn uni_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut uni_state_t = vstate as *mut uni_state_t;
    let i: i32 = (*state).i;
    let j: i32 = (*state).j;
    let mut k: i64 = ((*state).m[i as usize]).wrapping_sub((*state).m[j as usize])
        as i64;
    if k < 0 as i32 as i64 {
        k += m1 as i64;
    }
    (*state).m[j as usize] = k as u64;
    if i == 0 as i32 {
        (*state).i = 16 as i32;
    } else {
        (*state).i -= 1;
        (*state).i;
    }
    if j == 0 as i32 {
        (*state).j = 16 as i32;
    } else {
        (*state).j -= 1;
        (*state).j;
    }
    return k as u64;
}
unsafe extern "C" fn uni_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return uni_get(vstate) as libc::c_double / 32767.0f64;
}
unsafe extern "C" fn uni_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut i: u32 = 0;
    let mut seed: u32 = 0;
    let mut k0: u32 = 0;
    let mut k1: u32 = 0;
    let mut j0: u32 = 0;
    let mut j1: u32 = 0;
    let mut state: *mut uni_state_t = vstate as *mut uni_state_t;
    s = (2 as i32 as u64).wrapping_mul(s).wrapping_add(1 as i32 as u64);
    seed = (if s < m1 as u64 { s } else { m1 as u64 }) as u32;
    k0 = (9069 as i32 as u32).wrapping_rem(m2);
    k1 = (9069 as i32 as u32).wrapping_div(m2);
    j0 = seed.wrapping_rem(m2);
    j1 = seed.wrapping_div(m2);
    i = 0 as i32 as u32;
    while i < 17 as i32 as u32 {
        seed = j0.wrapping_mul(k0);
        j1 = seed
            .wrapping_div(m2)
            .wrapping_add(j0.wrapping_mul(k1))
            .wrapping_add(j1.wrapping_mul(k0))
            .wrapping_rem(m2.wrapping_div(2 as i32 as u32));
        j0 = seed.wrapping_rem(m2);
        (*state).m[i as usize] = j0.wrapping_add(m2.wrapping_mul(j1)) as u64;
        i = i.wrapping_add(1);
        i;
    }
    (*state).i = 4 as i32;
    (*state).j = 16 as i32;
}
static mut uni_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"uni\0" as *const u8 as *const i8,
        max: 32766 as i32 as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<uni_state_t>() as u64,
        set: Some(uni_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(uni_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            uni_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_uni: *const gsl_rng_type = unsafe {
    &uni_type as *const gsl_rng_type
};