use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
        if !(x == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_weibull(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = gsl_rng_uniform_pos(r);
    let mut z: libc::c_double = pow(-log(x), 1 as libc::c_int as libc::c_double / b);
    return a * z;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_weibull_pdf(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else if x == 0 as libc::c_int as libc::c_double {
        if b == 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int as libc::c_double / a
        } else {
            return 0 as libc::c_int as libc::c_double
        }
    } else if b == 1 as libc::c_int as libc::c_double {
        return exp(-x / a) / a
    } else {
        let mut p: libc::c_double = b / a
            * exp(
                -pow(x / a, b) + (b - 1 as libc::c_int as libc::c_double) * log(x / a),
            );
        return p;
    };
}
