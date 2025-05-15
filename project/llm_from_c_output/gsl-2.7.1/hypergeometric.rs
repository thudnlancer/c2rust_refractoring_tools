//! Cumulative distribution function for a hypergeometric random variable.

use std::f64;

/// Computes the lower tail probability Pr(X <= k)
fn lower_tail(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    let mut i = k as i32;
    let mut s = hypergeometric_pdf(i as u32, n1, n2, t);
    let mut p = s;
    
    while i > 0 {
        let factor = (i as f64 / (n1 as f64 - i as f64 + 1.0)) * 
                    ((n2 as f64 + i as f64 - t as f64) / (t as f64 - i as f64 + 1.0));
        s *= factor;
        p += s;
        let relerr = s / p;
        if relerr < f64::EPSILON {
            break;
        }
        i -= 1;
    }

    p
}

/// Computes the upper tail probability Pr(X > k)
fn upper_tail(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    let mut i = k + 1;
    let mut s = hypergeometric_pdf(i, n1, n2, t);
    let mut q = s;
    
    while i < t {
        let factor = ((n1 as f64 - i as f64) / (i as f64 + 1.0)) * 
                    ((t as f64 - i as f64) / (n2 as f64 + i as f64 + 1.0 - t as f64));
        s *= factor;
        q += s;
        let relerr = s / q;
        if relerr < f64::EPSILON {
            break;
        }
        i += 1;
    }

    q
}

/// Computes the hypergeometric probability density function
fn hypergeometric_pdf(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    // Implementation of hypergeometric PDF would go here
    // This is a placeholder - actual implementation would use exact calculations
    // with logarithms and exponentiation to avoid overflow
    unimplemented!()
}

/// Computes Pr(X <= k)
pub fn hypergeometric_cdf_p(k: u32, n1: u32, n2: u32, t: u32) -> Result<f64, &'static str> {
    if t > (n1 + n2) {
        Err("t larger than population size")
    } else if k >= n1 || k >= t {
        Ok(1.0)
    } else if k == 0 {
        Ok(0.0)
    } else {
        let midpoint = (t as f64 * n1 as f64) / (n1 as f64 + n2 as f64);

        if k as f64 >= midpoint {
            Ok(1.0 - upper_tail(k, n1, n2, t))
        } else {
            Ok(lower_tail(k, n1, n2, t))
        }
    }
}

/// Computes Pr(X > k)
pub fn hypergeometric_cdf_q(k: u32, n1: u32, n2: u32, t: u32) -> Result<f64, &'static str> {
    if t > (n1 + n2) {
        Err("t larger than population size")
    } else if k >= n1 || k >= t {
        Ok(0.0)
    } else if k == 0 {
        Ok(1.0)
    } else {
        let midpoint = (t as f64 * n1 as f64) / (n1 as f64 + n2 as f64);

        if (k as f64) < midpoint {
            Ok(1.0 - lower_tail(k, n1, n2, t))
        } else {
            Ok(upper_tail(k, n1, n2, t))
        }
    }
}