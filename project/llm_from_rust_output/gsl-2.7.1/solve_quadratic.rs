use std::f64;

pub fn gsl_poly_solve_quadratic(
    a: f64,
    b: f64,
    c: f64,
) -> (Option<(f64, f64)>, usize) {
    if a == 0.0 {
        if b == 0.0 {
            return (None, 0);
        } else {
            return (Some((-c / b, 0.0)), 1);
        }
    }

    let disc = b * b - 4.0 * a * c;

    if disc > 0.0 {
        let roots = if b == 0.0 {
            let r = (-c / a).sqrt();
            (-r, r)
        } else {
            let sgnb = if b > 0.0 { 1.0 } else { -1.0 };
            let temp = -0.5 * (b + sgnb * disc.sqrt());
            let r1 = temp / a;
            let r2 = c / temp;
            if r1 < r2 {
                (r1, r2)
            } else {
                (r2, r1)
            }
        };
        (Some(roots), 2)
    } else if disc == 0.0 {
        let root = -0.5 * b / a;
        (Some((root, root)), 2)
    } else {
        (None, 0)
    }
}