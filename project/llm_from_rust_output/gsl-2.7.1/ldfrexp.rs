use std::f64;

#[no_mangle]
pub extern "C" fn gsl_ldexp(x: f64, e: i32) -> f64 {
    if x == 0.0 {
        return x;
    }

    let (mut y, mut ex) = gsl_frexp(x);
    let mut e2 = (e + ex) as f64;

    if e2 >= 1024.0 {
        y *= f64::powf(2.0, e2 - 1024.0 + 1.0);
        e2 = (1024 - 1) as f64;
    } else if e2 <= -1021.0 {
        y *= f64::powf(2.0, e2 + 1021.0 - 1.0);
        e2 = (-1021 + 1) as f64;
    }

    let p2 = f64::powf(2.0, e2);
    y * p2
}

#[no_mangle]
pub extern "C" fn gsl_frexp(x: f64, e: &mut i32) -> f64 {
    if x == 0.0 {
        *e = 0;
        return 0.0;
    } else if !x.is_finite() {
        *e = 0;
        return x;
    } else if x.abs() >= 0.5 && x.abs() < 1.0 {
        *e = 0;
        return x;
    } else {
        let mut ex = f64::ceil(f64::ln(x.abs()) / 0.69314718055994530942);
        let mut ei = ex as i32;
        let mut f;

        if ei < -1021 {
            ei = -1021;
        }
        if ei > 1021 {
            ei = 1021;
        }

        f = x * f64::powf(2.0, -ei as f64);
        if !f.is_finite() {
            *e = 0;
            return f;
        }

        while f.abs() >= 1.0 {
            ei += 1;
            f /= 2.0;
        }

        while f.abs() > 0.0 && f.abs() < 0.5 {
            ei -= 1;
            f *= 2.0;
        }

        *e = ei;
        f
    }
}