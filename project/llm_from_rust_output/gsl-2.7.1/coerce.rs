#[no_mangle]
pub extern "C" fn gsl_coerce_double(x: f64) -> f64 {
    x
}

#[no_mangle]
pub extern "C" fn gsl_coerce_float(x: f32) -> f32 {
    x
}

#[no_mangle]
pub extern "C" fn gsl_coerce_long_double(x: f128::f128) -> f128::f128 {
    x
}