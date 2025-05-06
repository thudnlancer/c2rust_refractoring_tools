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
pub struct r250_state_t {
    pub i: i32,
    pub x: [u64; 250],
}
#[inline]
unsafe extern "C" fn r250_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut r250_state_t = vstate as *mut r250_state_t;
    let mut k: u64 = 0;
    let mut j: i32 = 0;
    let mut i: i32 = (*state).i;
    if i >= 147 as i32 {
        j = i - 147 as i32;
    } else {
        j = i + 103 as i32;
    }
    k = (*state).x[i as usize] ^ (*state).x[j as usize];
    (*state).x[i as usize] = k;
    if i >= 249 as i32 {
        (*state).i = 0 as i32;
    } else {
        (*state).i = i + 1 as i32;
    }
    return k;
}
unsafe extern "C" fn r250_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return r250_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn r250_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut r250_state_t = vstate as *mut r250_state_t;
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).i = 0 as i32;
    i = 0 as i32;
    while i < 250 as i32 {
        s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
        (*state).x[i as usize] = s;
        i += 1;
        i;
    }
    let mut msb: u64 = 0x80000000 as u64;
    let mut mask: u64 = 0xffffffff as u64;
    i = 0 as i32;
    while i < 32 as i32 {
        let mut k: i32 = 7 as i32 * i + 3 as i32;
        (*state).x[k as usize] &= mask;
        (*state).x[k as usize] |= msb;
        mask >>= 1 as i32;
        msb >>= 1 as i32;
        i += 1;
        i;
    }
}
static mut r250_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"r250\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<r250_state_t>() as u64,
        set: Some(r250_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(r250_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            r250_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_r250: *const gsl_rng_type = unsafe {
    &r250_type as *const gsl_rng_type
};