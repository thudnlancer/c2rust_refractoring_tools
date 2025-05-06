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
    fn gsl_ran_binomial(r: *const gsl_rng, p: libc::c_double, n: u32) -> u32;
    fn gsl_sf_lnfact(n: u32) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_multinomial(
    mut r: *const gsl_rng,
    K: size_t,
    N: u32,
    mut p: *const libc::c_double,
    mut n: *mut u32,
) {
    let mut k: size_t = 0;
    let mut norm: libc::c_double = 0.0f64;
    let mut sum_p: libc::c_double = 0.0f64;
    let mut sum_n: u32 = 0 as i32 as u32;
    k = 0 as i32 as size_t;
    while k < K {
        norm += *p.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    k = 0 as i32 as size_t;
    while k < K {
        if *p.offset(k as isize) > 0.0f64 {
            *n.offset(k as isize) = gsl_ran_binomial(
                r,
                *p.offset(k as isize) / (norm - sum_p),
                N.wrapping_sub(sum_n),
            );
        } else {
            *n.offset(k as isize) = 0 as i32 as u32;
        }
        sum_p += *p.offset(k as isize);
        sum_n = sum_n.wrapping_add(*n.offset(k as isize));
        k = k.wrapping_add(1);
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_multinomial_pdf(
    K: size_t,
    mut p: *const libc::c_double,
    mut n: *const u32,
) -> libc::c_double {
    return exp(gsl_ran_multinomial_lnpdf(K, p, n));
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_multinomial_lnpdf(
    K: size_t,
    mut p: *const libc::c_double,
    mut n: *const u32,
) -> libc::c_double {
    let mut k: size_t = 0;
    let mut N: u32 = 0 as i32 as u32;
    let mut log_pdf: libc::c_double = 0.0f64;
    let mut norm: libc::c_double = 0.0f64;
    k = 0 as i32 as size_t;
    while k < K {
        N = N.wrapping_add(*n.offset(k as isize));
        k = k.wrapping_add(1);
        k;
    }
    k = 0 as i32 as size_t;
    while k < K {
        norm += *p.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    log_pdf = gsl_sf_lnfact(N);
    k = 0 as i32 as size_t;
    while k < K {
        if *n.offset(k as isize) > 0 as i32 as u32 {
            log_pdf
                += log(*p.offset(k as isize) / norm)
                    * *n.offset(k as isize) as libc::c_double
                    - gsl_sf_lnfact(*n.offset(k as isize));
        }
        k = k.wrapping_add(1);
        k;
    }
    return log_pdf;
}