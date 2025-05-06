#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn gsl_ran_negative_binomial(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_double,
    ) -> u32;
    fn gsl_ran_negative_binomial_pdf(
        k: u32,
        p: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_double;
}
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
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_pascal(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
    mut n: u32,
) -> u32 {
    let mut k: u32 = gsl_ran_negative_binomial(r, p, n as libc::c_double);
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_pascal_pdf(
    k: u32,
    p: libc::c_double,
    mut n: u32,
) -> libc::c_double {
    let mut P: libc::c_double = gsl_ran_negative_binomial_pdf(k, p, n as libc::c_double);
    return P;
}