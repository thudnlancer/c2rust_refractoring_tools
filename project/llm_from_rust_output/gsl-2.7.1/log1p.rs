#[no_mangle]
pub extern "C" fn gsl_log1p(x: f64) -> f64 {
    let y = 1.0 + x;
    let z = y - 1.0;
    y.ln() - (z - x) / y
}