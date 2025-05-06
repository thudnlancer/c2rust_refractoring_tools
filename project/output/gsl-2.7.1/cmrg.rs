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
pub struct cmrg_state_t {
    pub x1: i64,
    pub x2: i64,
    pub x3: i64,
    pub y1: i64,
    pub y2: i64,
    pub y3: i64,
}
static mut m1: i64 = 2147483647 as i32 as i64;
static mut m2: i64 = 2145483479 as i32 as i64;
static mut a2: i64 = 63308 as i32 as i64;
static mut qa2: i64 = 33921 as i32 as i64;
static mut ra2: i64 = 12979 as i32 as i64;
static mut a3: i64 = -(183326 as i32) as i64;
static mut qa3: i64 = 11714 as i32 as i64;
static mut ra3: i64 = 2883 as i32 as i64;
static mut b1: i64 = 86098 as i32 as i64;
static mut qb1: i64 = 24919 as i32 as i64;
static mut rb1: i64 = 7417 as i32 as i64;
static mut b3: i64 = -(539608 as i32) as i64;
static mut qb3: i64 = 3976 as i32 as i64;
static mut rb3: i64 = 2071 as i32 as i64;
#[inline]
unsafe extern "C" fn cmrg_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut cmrg_state_t = vstate as *mut cmrg_state_t;
    let mut h3: i64 = (*state).x3 / qa3;
    let mut p3: i64 = -a3 * ((*state).x3 - h3 * qa3) - h3 * ra3;
    let mut h2: i64 = (*state).x2 / qa2;
    let mut p2: i64 = a2 * ((*state).x2 - h2 * qa2) - h2 * ra2;
    if p3 < 0 as i32 as i64 {
        p3 += m1;
    }
    if p2 < 0 as i32 as i64 {
        p2 += m1;
    }
    (*state).x3 = (*state).x2;
    (*state).x2 = (*state).x1;
    (*state).x1 = p2 - p3;
    if (*state).x1 < 0 as i32 as i64 {
        (*state).x1 += m1;
    }
    let mut h3_0: i64 = (*state).y3 / qb3;
    let mut p3_0: i64 = -b3 * ((*state).y3 - h3_0 * qb3) - h3_0 * rb3;
    let mut h1: i64 = (*state).y1 / qb1;
    let mut p1: i64 = b1 * ((*state).y1 - h1 * qb1) - h1 * rb1;
    if p3_0 < 0 as i32 as i64 {
        p3_0 += m2;
    }
    if p1 < 0 as i32 as i64 {
        p1 += m2;
    }
    (*state).y3 = (*state).y2;
    (*state).y2 = (*state).y1;
    (*state).y1 = p1 - p3_0;
    if (*state).y1 < 0 as i32 as i64 {
        (*state).y1 += m2;
    }
    if (*state).x1 < (*state).y1 {
        return ((*state).x1 - (*state).y1 + m1) as u64
    } else {
        return ((*state).x1 - (*state).y1) as u64
    };
}
unsafe extern "C" fn cmrg_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return cmrg_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn cmrg_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut cmrg_state_t = vstate as *mut cmrg_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x1 = s.wrapping_rem(m1 as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x2 = s.wrapping_rem(m1 as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).x3 = s.wrapping_rem(m1 as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).y1 = s.wrapping_rem(m2 as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).y2 = s.wrapping_rem(m2 as u64) as i64;
    s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
    (*state).y3 = s.wrapping_rem(m2 as u64) as i64;
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
}
static mut cmrg_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"cmrg\0" as *const u8 as *const i8,
        max: 2147483646 as i32 as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<cmrg_state_t>() as u64,
        set: Some(cmrg_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(cmrg_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            cmrg_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_cmrg: *const gsl_rng_type = unsafe {
    &cmrg_type as *const gsl_rng_type
};