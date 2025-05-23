use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
    fn gsl_ran_gamma(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
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
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_erlang(
    mut r: *const gsl_rng,
    a: libc::c_double,
    n: libc::c_double,
) -> libc::c_double {
    return gsl_ran_gamma(r, n, a);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_erlang_pdf(
    x: libc::c_double,
    a: libc::c_double,
    n: libc::c_double,
) -> libc::c_double {
    if x <= 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut p: libc::c_double = 0.;
        let mut lngamma: libc::c_double = gsl_sf_lngamma(n);
        p = exp((n - 1 as libc::c_int as libc::c_double) * log(x / a) - x / a - lngamma)
            / a;
        return p;
    };
}
