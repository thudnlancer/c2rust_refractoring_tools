//! Solves the real roots of x^3 + a x^2 + b x + c = 0

use std::f64::consts::PI;

/// Solves the cubic equation x^3 + a x^2 + b x + c = 0
/// Returns the number of real roots found and stores them in the provided references
pub fn gsl_poly_solve_cubic(
    a: f64,
    b: f64,
    c: f64,
    x0: &mut f64,
    x1: &mut f64,
    x2: &mut f64,
) -> i32 {
    let q = a * a - 3.0 * b;
    let r = 2.0 * a * a * a - 9.0 * a * b + 27.0 * c;

    let q_9 = q / 9.0;
    let r_54 = r / 54.0;

    let q3 = q_9 * q_9 * q_9;
    let r2 = r_54 * r_54;

    let cr2 = 729.0 * r * r;
    let cq3 = 2916.0 * q * q * q;

    if r_54 == 0.0 && q_9 == 0.0 {
        *x0 = -a / 3.0;
        *x1 = -a / 3.0;
        *x2 = -a / 3.0;
        3
    } else if cr2 == cq3 {
        // this test is actually r2 == q3, written in a form suitable
        // for exact computation with integers

        // Due to finite precision some double roots may be missed, and
        // considered to be a pair of complex roots z = x +/- epsilon i
        // close to the real axis.

        let sqrt_q = q_9.sqrt();

        if r_54 > 0.0 {
            *x0 = -2.0 * sqrt_q - a / 3.0;
            *x1 = sqrt_q - a / 3.0;
            *x2 = sqrt_q - a / 3.0;
        } else {
            *x0 = -sqrt_q - a / 3.0;
            *x1 = -sqrt_q - a / 3.0;
            *x2 = 2.0 * sqrt_q - a / 3.0;
        }
        3
    } else if r2 < q3 {
        let sgn_r = if r_54 >= 0.0 { 1.0 } else { -1.0 };
        let ratio = sgn_r * (r2 / q3).sqrt();
        let theta = ratio.acos();
        let norm = -2.0 * q_9.sqrt();
        *x0 = norm * (theta / 3.0).cos() - a / 3.0;
        *x1 = norm * ((theta + 2.0 * PI) / 3.0).cos() - a / 3.0;
        *x2 = norm * ((theta - 2.0 * PI) / 3.0).cos() - a / 3.0;

        // Sort x0, x1, x2 into increasing order
        if *x0 > *x1 {
            std::mem::swap(x0, x1);
        }

        if *x1 > *x2 {
            std::mem::swap(x1, x2);
            if *x0 > *x1 {
                std::mem::swap(x0, x1);
            }
        }

        3
    } else {
        let sgn_r = if r_54 >= 0.0 { 1.0 } else { -1.0 };
        let abs_r = r_54.abs();
        let a_term = -sgn_r * (abs_r + (r2 - q3).sqrt()).powf(1.0 / 3.0);
        let b_term = q_9 / a_term;
        *x0 = a_term + b_term - a / 3.0;
        1
    }
}