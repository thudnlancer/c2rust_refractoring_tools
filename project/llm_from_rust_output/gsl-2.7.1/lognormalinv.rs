use libc::c_double;
use std::f64;

extern "C" {
    fn gsl_cdf_ugaussian_Pinv(P: c_double) -> c_double;
    fn gsl_cdf_ugaussian_Qinv(Q: c_double) -> c_double;
}

pub fn gsl_cdf_lognormal_Pinv(P: c_double, zeta: c_double, sigma: c_double) -> c_double {
    if P == 1.0 {
        return f64::INFINITY;
    } else if P == 0.0 {
        return 0.0;
    }
    let u = unsafe { gsl_cdf_ugaussian_Pinv(P) };
    let x = (zeta + sigma * u).exp();
    x
}

pub fn gsl_cdf_lognormal_Qinv(Q: c_double, zeta: c_double, sigma: c_double) -> c_double {
    if Q == 0.0 {
        return f64::INFINITY;
    } else if Q == 1.0 {
        return 0.0;
    }
    let u = unsafe { gsl_cdf_ugaussian_Qinv(Q) };
    let x = (zeta + sigma * u).exp();
    x
}