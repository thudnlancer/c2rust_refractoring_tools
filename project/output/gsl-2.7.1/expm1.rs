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
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_expm1(x: libc::c_double) -> libc::c_double {
    if fabs(x) < 0.69314718055994530942f64 {
        let mut i: libc::c_double = 1.0f64;
        let mut sum: libc::c_double = x;
        let mut term: libc::c_double = x / 1.0f64;
        loop {
            i += 1.;
            i;
            term *= x / i;
            sum += term;
            if !(fabs(term) > fabs(sum) * 2.2204460492503131e-16f64) {
                break;
            }
        }
        return sum;
    } else {
        return exp(x) - 1 as i32 as libc::c_double
    };
}