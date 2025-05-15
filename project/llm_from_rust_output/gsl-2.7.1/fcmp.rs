#[no_mangle]
pub extern "C" fn gsl_fcmp(x1: f64, x2: f64, epsilon: f64) -> i32 {
    let max = if x1.abs() > x2.abs() { x1 } else { x2 };
    let exponent = max.log2().floor() as i32 + 1;
    let delta = epsilon * 2f64.powi(exponent);
    let difference = x1 - x2;
    
    if difference > delta {
        1
    } else if difference < -delta {
        -1
    } else {
        0
    }
}