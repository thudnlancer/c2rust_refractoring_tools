use std::f64;

#[no_mangle]
pub extern "C" fn gsl_nan() -> f64 {
    f64::NAN
}

#[no_mangle]
pub extern "C" fn gsl_posinf() -> f64 {
    f64::INFINITY
}

#[no_mangle]
pub extern "C" fn gsl_neginf() -> f64 {
    f64::NEG_INFINITY
}

#[no_mangle]
pub extern "C" fn gsl_finite(x: f64) -> i32 {
    x.is_finite() as i32
}

#[no_mangle]
pub extern "C" fn gsl_isnan(x: f64) -> i32 {
    x.is_nan() as i32
}

#[no_mangle]
pub extern "C" fn gsl_isinf(x: f64) -> i32 {
    if x.is_infinite() {
        if x > 0.0 {
            1
        } else {
            -1
        }
    } else {
        0
    }
}