use std::f64;

pub enum GslError {
    Dom,
    Range,
    // Other GSL error variants omitted for brevity
    // ...
}

fn gsl_error(reason: &str, file: &str, line: u32, errno: GslError) {
    // Implementation would log or handle the error appropriately
    eprintln!("GSL Error ({}:{}): {} - {:?}", file, line, reason, errno);
}

fn gsl_ran_hypergeometric_pdf(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    // Implementation would call the actual GSL function
    // This is a placeholder for the actual implementation
    0.0
}

fn lower_tail(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    let mut relerr = 0.0;
    let mut i = k as i32;
    let mut s = gsl_ran_hypergeometric_pdf(i as u32, n1, n2, t);
    let mut p = s;

    while i > 0 {
        let factor = i as f64 / (n1 - i as u32) as f64 + 1.0
            * (n2 + i as u32 - t) as f64 / (t - i as u32) as f64 + 1.0;
        s *= factor;
        p += s;
        relerr = s / p;
        if relerr < 2.2204460492503131e-16 {
            break;
        }
        i -= 1;
    }
    p
}

fn upper_tail(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    let mut relerr = 0.0;
    let mut i = k + 1;
    let mut s = gsl_ran_hypergeometric_pdf(i, n1, n2, t);
    let mut q = s;

    while i < t {
        let factor = (n1 - i) as f64 / (i as f64 + 1.0)
            * (t - i) as f64 / (n2 + i) as f64 + 1.0 - t as f64;
        s *= factor;
        q += s;
        relerr = s / q;
        if relerr < 2.2204460492503131e-16 {
            break;
        }
        i += 1;
    }
    q
}

pub fn gsl_cdf_hypergeometric_p(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    if t > n1 + n2 {
        gsl_error(
            "t larger than population size",
            "hypergeometric.c",
            119,
            GslError::Dom,
        );
        return f64::NAN;
    } else if k >= n1 || k >= t {
        1.0
    } else if k as f64 < 0.0 {
        0.0
    } else {
        let midpoint = t as f64 * n1 as f64 / (n1 as f64 + n2 as f64);
        if k as f64 >= midpoint {
            1.0 - upper_tail(k, n1, n2, t)
        } else {
            lower_tail(k, n1, n2, t)
        }
    }
}

pub fn gsl_cdf_hypergeometric_q(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    if t > n1 + n2 {
        gsl_error(
            "t larger than population size",
            "hypergeometric.c",
            158,
            GslError::Dom,
        );
        return f64::NAN;
    } else if k >= n1 || k >= t {
        0.0
    } else if k as f64 < 0.0 {
        1.0
    } else {
        let midpoint = t as f64 * n1 as f64 / (n1 as f64 + n2 as f64);
        if k as f64 < midpoint {
            1.0 - lower_tail(k, n1, n2, t)
        } else {
            upper_tail(k, n1, n2, t)
        }
    }
}