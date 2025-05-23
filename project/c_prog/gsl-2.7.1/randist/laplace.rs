use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
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
pub unsafe extern "C" fn gsl_ran_laplace(
    mut r: *const gsl_rng,
    a: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    loop {
        u = 2 as libc::c_int as libc::c_double * gsl_rng_uniform(r) - 1.0f64;
        if !(u == 0.0f64) {
            break;
        }
    }
    if u < 0 as libc::c_int as libc::c_double {
        return a * log(-u)
    } else {
        return -a * log(u)
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_laplace_pdf(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = 1 as libc::c_int as libc::c_double
        / (2 as libc::c_int as libc::c_double * a) * exp(-fabs(x) / a);
    return p;
}
