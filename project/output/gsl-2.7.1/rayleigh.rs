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
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
#[inline]
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
        if !(x == 0 as i32 as libc::c_double) {
            break;
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_rayleigh(
    mut r: *const gsl_rng,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = gsl_rng_uniform_pos(r);
    return sigma * sqrt(-2.0f64 * log(u));
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_rayleigh_pdf(
    x: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    if x < 0 as i32 as libc::c_double {
        return 0 as i32 as libc::c_double
    } else {
        let mut u: libc::c_double = x / sigma;
        let mut p: libc::c_double = u / sigma * exp(-u * u / 2.0f64);
        return p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_rayleigh_tail(
    mut r: *const gsl_rng,
    a: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = gsl_rng_uniform_pos(r);
    return sqrt(a * a - 2.0f64 * sigma * sigma * log(u));
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_rayleigh_tail_pdf(
    x: libc::c_double,
    a: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    if x < a {
        return 0 as i32 as libc::c_double
    } else {
        let mut u: libc::c_double = x / sigma;
        let mut v: libc::c_double = a / sigma;
        let mut p: libc::c_double = u / sigma * exp((v + u) * (v - u) / 2.0f64);
        return p;
    };
}