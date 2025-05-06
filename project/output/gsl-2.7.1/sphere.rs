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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_gaussian(r: *const gsl_rng, sigma: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_dir_2d(
    mut r: *const gsl_rng,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    loop {
        u = -(1 as i32) as libc::c_double
            + 2 as i32 as libc::c_double * gsl_rng_uniform(r);
        v = -(1 as i32) as libc::c_double
            + 2 as i32 as libc::c_double * gsl_rng_uniform(r);
        s = u * u + v * v;
        if !(s > 1.0f64 || s == 0.0f64) {
            break;
        }
    }
    *x = (u * u - v * v) / s;
    *y = 2 as i32 as libc::c_double * u * v / s;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_dir_2d_trig_method(
    mut r: *const gsl_rng,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    let mut t: libc::c_double = 6.2831853071795864f64 * gsl_rng_uniform(r);
    *x = cos(t);
    *y = sin(t);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_dir_3d(
    mut r: *const gsl_rng,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut z: *mut libc::c_double,
) {
    let mut s: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    loop {
        *x = -(1 as i32) as libc::c_double
            + 2 as i32 as libc::c_double * gsl_rng_uniform(r);
        *y = -(1 as i32) as libc::c_double
            + 2 as i32 as libc::c_double * gsl_rng_uniform(r);
        s = *x * *x + *y * *y;
        if !(s > 1.0f64) {
            break;
        }
    }
    *z = -(1 as i32) as libc::c_double + 2 as i32 as libc::c_double * s;
    a = 2 as i32 as libc::c_double * sqrt(1 as i32 as libc::c_double - s);
    *x *= a;
    *y *= a;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_dir_nd(
    mut r: *const gsl_rng,
    mut n: size_t,
    mut x: *mut libc::c_double,
) {
    let mut d: libc::c_double = 0.;
    let mut i: size_t = 0;
    d = 0 as i32 as libc::c_double;
    loop {
        i = 0 as i32 as size_t;
        while i < n {
            *x.offset(i as isize) = gsl_ran_gaussian(r, 1.0f64);
            d += *x.offset(i as isize) * *x.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        if !(d == 0 as i32 as libc::c_double) {
            break;
        }
    }
    d = sqrt(d);
    i = 0 as i32 as size_t;
    while i < n {
        *x.offset(i as isize) /= d;
        i = i.wrapping_add(1);
        i;
    }
}