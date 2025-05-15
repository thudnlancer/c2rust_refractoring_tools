pub fn gsl_cdf_flat_Pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 { 
        b 
    } else if P == 0.0 { 
        a 
    } else {
        (1.0 - P) * a + P * b
    }
}

pub fn gsl_cdf_flat_Qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 { 
        b 
    } else if Q == 1.0 { 
        a 
    } else {
        Q * a + (1.0 - Q) * b
    }
}