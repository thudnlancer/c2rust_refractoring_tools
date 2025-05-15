//! Solves quadratic equation a x^2 + b x + c = 0 for complex roots

use num_complex::Complex64;
use std::f64;

/// Solves the quadratic equation a x^2 + b x + c = 0 for complex roots.
/// Returns the number of distinct roots found (0, 1, or 2).
/// The roots are stored in z0 and z1 (for 1 root, it's stored in z0).
pub fn gsl_poly_complex_solve_quadratic(
    a: f64,
    b: f64,
    c: f64,
    z0: &mut Complex64,
    z1: &mut Complex64,
) -> i32 {
    let disc = b * b - 4.0 * a * c;

    if a == 0.0 {
        // Handle linear case
        if b == 0.0 {
            return 0;
        } else {
            *z0 = Complex64::new(-c / b, 0.0);
            return 1;
        }
    }

    if disc > 0.0 {
        if b == 0.0 {
            let s = f64::abs(0.5 * f64::sqrt(disc) / a);
            *z0 = Complex64::new(-s, 0.0);
            *z1 = Complex64::new(s, 0.0);
        } else {
            let sgnb = if b > 0.0 { 1.0 } else { -1.0 };
            let temp = -0.5 * (b + sgnb * f64::sqrt(disc));
            let r1 = temp / a;
            let r2 = c / temp;

            if r1 < r2 {
                *z0 = Complex64::new(r1, 0.0);
                *z1 = Complex64::new(r2, 0.0);
            } else {
                *z0 = Complex64::new(r2, 0.0);
                *z1 = Complex64::new(r1, 0.0);
            }
        }
        return 2;
    } else if disc == 0.0 {
        let root = -0.5 * b / a;
        *z0 = Complex64::new(root, 0.0);
        *z1 = Complex64::new(root, 0.0);
        return 2;
    } else {
        let s = f64::abs(0.5 * f64::sqrt(-disc) / a);
        let real_part = -0.5 * b / a;
        *z0 = Complex64::new(real_part, -s);
        *z1 = Complex64::new(real_part, s);
        return 2;
    }
}