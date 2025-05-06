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
pub struct ranlux_state_t {
    pub i: u32,
    pub j: u32,
    pub n: u32,
    pub skip: u32,
    pub carry: u32,
    pub u: [u64; 24],
}
static mut mask_lo: u64 = 0xffffff as u64;
static mut mask_hi: u64 = !(0xffffff as u64);
static mut two24: u64 = 16777216 as i32 as u64;
#[inline]
unsafe extern "C" fn increment_state(mut state: *mut ranlux_state_t) -> u64 {
    let mut i: u32 = (*state).i;
    let mut j: u32 = (*state).j;
    let mut delta: i64 = ((*state).u[j as usize])
        .wrapping_sub((*state).u[i as usize])
        .wrapping_sub((*state).carry as u64) as i64;
    if delta as u64 & mask_hi != 0 {
        (*state).carry = 1 as i32 as u32;
        delta = (delta as u64 & mask_lo) as i64;
    } else {
        (*state).carry = 0 as i32 as u32;
    }
    (*state).u[i as usize] = delta as u64;
    if i == 0 as i32 as u32 {
        i = 23 as i32 as u32;
    } else {
        i = i.wrapping_sub(1);
        i;
    }
    (*state).i = i;
    if j == 0 as i32 as u32 {
        j = 23 as i32 as u32;
    } else {
        j = j.wrapping_sub(1);
        j;
    }
    (*state).j = j;
    return delta as u64;
}
#[inline]
unsafe extern "C" fn ranlux_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ranlux_state_t = vstate as *mut ranlux_state_t;
    let skip: u32 = (*state).skip;
    let mut r: u64 = increment_state(state);
    (*state).n = ((*state).n).wrapping_add(1);
    (*state).n;
    if (*state).n == 24 as i32 as u32 {
        let mut i: u32 = 0;
        (*state).n = 0 as i32 as u32;
        i = 0 as i32 as u32;
        while i < skip {
            increment_state(state);
            i = i.wrapping_add(1);
            i;
        }
    }
    return r;
}
unsafe extern "C" fn ranlux_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ranlux_get(vstate) as libc::c_double / 16777216.0f64;
}
unsafe extern "C" fn ranlux_set_lux(
    mut vstate: *mut libc::c_void,
    mut s: u64,
    mut luxury: u32,
) {
    let mut state: *mut ranlux_state_t = vstate as *mut ranlux_state_t;
    let mut i: i32 = 0;
    let mut seed: i64 = 0;
    if s == 0 as i32 as u64 {
        s = 314159265 as i32 as u64;
    }
    seed = s as i64;
    i = 0 as i32;
    while i < 24 as i32 {
        let mut k: u64 = (seed / 53668 as i32 as i64) as u64;
        seed = (40014 as i32 as u64)
            .wrapping_mul(
                (seed as u64).wrapping_sub(k.wrapping_mul(53668 as i32 as u64)),
            )
            .wrapping_sub(k.wrapping_mul(12211 as i32 as u64)) as i64;
        if seed < 0 as i32 as i64 {
            seed += 2147483563 as i32 as i64;
        }
        (*state).u[i as usize] = (seed as u64).wrapping_rem(two24);
        i += 1;
        i;
    }
    (*state).i = 23 as i32 as u32;
    (*state).j = 9 as i32 as u32;
    (*state).n = 0 as i32 as u32;
    (*state).skip = luxury.wrapping_sub(24 as i32 as u32);
    if (*state).u[23 as i32 as usize] & mask_hi != 0 {
        (*state).carry = 1 as i32 as u32;
    } else {
        (*state).carry = 0 as i32 as u32;
    };
}
unsafe extern "C" fn ranlux_set(mut vstate: *mut libc::c_void, mut s: u64) {
    ranlux_set_lux(vstate, s, 223 as i32 as u32);
}
unsafe extern "C" fn ranlux389_set(mut vstate: *mut libc::c_void, mut s: u64) {
    ranlux_set_lux(vstate, s, 389 as i32 as u32);
}
static mut ranlux_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlux\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranlux_state_t>() as u64,
        set: Some(ranlux_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranlux_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranlux_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut ranlux389_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlux389\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranlux_state_t>() as u64,
        set: Some(ranlux389_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranlux_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranlux_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ranlux: *const gsl_rng_type = unsafe {
    &ranlux_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_ranlux389: *const gsl_rng_type = unsafe {
    &ranlux389_type as *const gsl_rng_type
};