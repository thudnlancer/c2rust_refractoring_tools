//! Integer power functions equivalent to GSL's gsl_pow_int and gsl_pow_uint
//!
//! Translated from C code by Gerard Jungman, originally under GPL

/// Compute x^n where n is an integer
pub fn gsl_pow_int(x: f64, n: i32) -> f64 {
    let un = if n < 0 {
        (1.0 / x, (-n) as u32)
    } else {
        (x, n as u32)
    };
    
    gsl_pow_uint(un.0, un.1)
}

/// Compute x^n where n is an unsigned integer
pub fn gsl_pow_uint(mut x: f64, mut n: u32) -> f64 {
    let mut value = 1.0;
    
    /* repeated squaring method 
     * returns 0.0^0 = 1.0, so continuous in x
     */
    loop {
        if n & 1 != 0 {
            value *= x;  // for n odd
        }
        n >>= 1;
        if n == 0 {
            break;
        }
        x *= x;
    }
    
    value
}