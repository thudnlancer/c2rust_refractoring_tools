#[no_mangle]
pub extern "C" fn gsl_cdf_exponential_Pinv(P: libc::c_double, mu: libc::c_double) -> libc::c_double {
    -mu * libm::log1p(-P)
}

#[no_mangle]
pub extern "C" fn gsl_cdf_exponential_Qinv(Q: libc::c_double, mu: libc::c_double) -> libc::c_double {
    -mu * libm::log(Q)
}