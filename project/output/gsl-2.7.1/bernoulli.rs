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
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_bernoulli(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
) -> u32 {
    let mut u: libc::c_double = gsl_rng_uniform(r);
    if u < p { return 1 as i32 as u32 } else { return 0 as i32 as u32 };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_bernoulli_pdf(
    k: u32,
    mut p: libc::c_double,
) -> libc::c_double {
    if k == 0 as i32 as u32 {
        return 1 as i32 as libc::c_double - p
    } else if k == 1 as i32 as u32 {
        return p
    } else {
        return 0 as i32 as libc::c_double
    };
}