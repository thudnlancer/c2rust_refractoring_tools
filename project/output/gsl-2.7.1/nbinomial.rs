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
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_poisson(r: *const gsl_rng, mu: libc::c_double) -> u32;
    fn gsl_ran_gamma(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_negative_binomial(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
    mut n: libc::c_double,
) -> u32 {
    let mut X: libc::c_double = gsl_ran_gamma(r, n, 1.0f64);
    let mut k: u32 = gsl_ran_poisson(r, X * (1 as i32 as libc::c_double - p) / p);
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_negative_binomial_pdf(
    k: u32,
    p: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut f: libc::c_double = gsl_sf_lngamma(k as libc::c_double + n);
    let mut a: libc::c_double = gsl_sf_lngamma(n);
    let mut b: libc::c_double = gsl_sf_lngamma(k as libc::c_double + 1.0f64);
    P = exp(f - a - b + n * log(p) + k as libc::c_double * log1p(-p));
    return P;
}