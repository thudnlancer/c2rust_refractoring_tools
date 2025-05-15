//! Solves quadratic equations of the form a x^2 + b x + c = 0

/// Finds the real roots of a quadratic equation a x^2 + b x + c = 0
/// Returns the number of real roots found (0, 1, or 2)
/// The roots are stored in x0 and x1 in ascending order
/// For the case of a single root, it will be stored in x0
pub fn gsl_poly_solve_quadratic(
    a: f64,
    b: f64,
    c: f64,
    x0: &mut f64,
    x1: &mut f64,
) -> i32 {
    if a == 0.0 {
        // Handle linear case
        if b == 0.0 {
            return 0;
        } else {
            *x0 = -c / b;
            return 1;
        }
    }

    let disc = b * b - 4.0 * a * c;

    if disc > 0.0 {
        if b == 0.0 {
            let r = (-c / a).sqrt();
            *x0 = -r;
            *x1 = r;
        } else {
            let sgnb = if b > 0.0 { 1.0 } else { -1.0 };
            let temp = -0.5 * (b + sgnb * disc.sqrt());
            let r1 = temp / a;
            let r2 = c / temp;

            if r1 < r2 {
                *x0 = r1;
                *x1 = r2;
            } else {
                *x0 = r2;
                *x1 = r1;
            }
        }
        2
    } else if disc == 0.0 {
        *x0 = -0.5 * b / a;
        *x1 = *x0;
        2
    } else {
        0
    }
}