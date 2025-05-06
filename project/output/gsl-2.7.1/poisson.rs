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
    fn gsl_sf_lnfact(n: u32) -> libc::c_double;
    fn gsl_ran_binomial(r: *const gsl_rng, p: libc::c_double, n: u32) -> u32;
    fn gsl_ran_gamma_int(r: *const gsl_rng, a: u32) -> libc::c_double;
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
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_poisson(
    mut r: *const gsl_rng,
    mut mu: libc::c_double,
) -> u32 {
    let mut emu: libc::c_double = 0.;
    let mut prod: libc::c_double = 1.0f64;
    let mut k: u32 = 0 as i32 as u32;
    while mu > 10 as i32 as libc::c_double {
        let mut m: u32 = (mu * (7.0f64 / 8.0f64)) as u32;
        let mut X: libc::c_double = gsl_ran_gamma_int(r, m);
        if X >= mu {
            return k
                .wrapping_add(
                    gsl_ran_binomial(r, mu / X, m.wrapping_sub(1 as i32 as u32)),
                )
        } else {
            k = k.wrapping_add(m);
            mu -= X;
        }
    }
    emu = exp(-mu);
    loop {
        prod *= gsl_rng_uniform(r);
        k = k.wrapping_add(1);
        k;
        if !(prod > emu) {
            break;
        }
    }
    return k.wrapping_sub(1 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_poisson_array(
    mut r: *const gsl_rng,
    mut n: size_t,
    mut array: *mut u32,
    mut mu: libc::c_double,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        *array.offset(i as isize) = gsl_ran_poisson(r, mu);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_poisson_pdf(
    k: u32,
    mu: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut lf: libc::c_double = gsl_sf_lnfact(k);
    p = exp(log(mu) * k as libc::c_double - lf - mu);
    return p;
}