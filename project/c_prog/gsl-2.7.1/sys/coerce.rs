use ::libc;
#[no_mangle]
pub unsafe extern "C" fn gsl_coerce_double(x: libc::c_double) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    ::core::ptr::write_volatile(&mut y as *mut libc::c_double, x);
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_coerce_float(x: libc::c_float) -> libc::c_float {
    let mut y: libc::c_float = 0.;
    ::core::ptr::write_volatile(&mut y as *mut libc::c_float, x);
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_coerce_long_double(x: f128::f128) -> f128::f128 {
    let mut y: f128::f128 = f128::f128::ZERO;
    ::core::ptr::write_volatile(&mut y as *mut f128::f128, x);
    return y;
}
