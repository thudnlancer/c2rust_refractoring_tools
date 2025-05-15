#[no_mangle]
pub extern "C" fn gsl_expm1(x: f64) -> f64 {
    if x.abs() < 0.69314718055994530942f64 {
        let mut i = 1.0f64;
        let mut sum = x;
        let mut term = x / 1.0f64;
        loop {
            i += 1.0;
            term *= x / i;
            sum += term;
            if !(term.abs() > sum.abs() * 2.2204460492503131e-16f64) {
                break;
            }
        }
        sum
    } else {
        x.exp() - 1.0
    }
}