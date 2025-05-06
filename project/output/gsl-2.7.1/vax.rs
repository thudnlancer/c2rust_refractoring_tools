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
pub struct vax_state_t {
    pub x: u64,
}
#[inline]
unsafe extern "C" fn vax_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut vax_state_t = vstate as *mut vax_state_t;
    (*state).x = (69069 as i32 as u64)
        .wrapping_mul((*state).x)
        .wrapping_add(1 as i32 as u64) & 0xffffffff as u64;
    return (*state).x;
}
unsafe extern "C" fn vax_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return vax_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn vax_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut vax_state_t = vstate as *mut vax_state_t;
    (*state).x = s;
}
static mut vax_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"vax\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<vax_state_t>() as u64,
        set: Some(vax_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(vax_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            vax_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_vax: *const gsl_rng_type = unsafe {
    &vax_type as *const gsl_rng_type
};