use std::f64::consts::PI;

pub fn gsl_poly_solve_cubic(
    a: f64,
    b: f64,
    c: f64,
) -> (i32, Vec<f64>) {
    let q = a * a - 3.0 * b;
    let r = 2.0 * a * a * a - 9.0 * a * b + 27.0 * c;
    let q_ratio = q / 9.0;
    let r_ratio = r / 54.0;
    let q3 = q_ratio * q_ratio * q_ratio;
    let r2 = r_ratio * r_ratio;
    let cr2 = 729.0 * r * r;
    let cq3 = 2916.0 * q * q * q;

    if r == 0.0 && q == 0.0 {
        let root = -a / 3.0;
        return (3, vec![root, root, root]);
    } else if cr2 == cq3 {
        let sqrt_q = q_ratio.sqrt();
        if r > 0.0 {
            let x0 = -2.0 * sqrt_q - a / 3.0;
            let x1 = sqrt_q - a / 3.0;
            return (3, vec![x0, x1, x1]);
        } else {
            let x0 = -sqrt_q - a / 3.0;
            let x2 = 2.0 * sqrt_q - a / 3.0;
            return (3, vec![x0, x0, x2]);
        }
    } else if r2 < q3 {
        let sgn_r = if r >= 0.0 { 1.0 } else { -1.0 };
        let ratio = sgn_r * (r2 / q3).sqrt();
        let theta = ratio.acos();
        let norm = -2.0 * q_ratio.sqrt();
        
        let mut roots = vec![
            norm * (theta / 3.0).cos() - a / 3.0,
            norm * ((theta + 2.0 * PI) / 3.0).cos() - a / 3.0,
            norm * ((theta - 2.0 * PI) / 3.0).cos() - a / 3.0,
        ];
        
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (3, roots)
    } else {
        let sgn_r = if r >= 0.0 { 1.0 } else { -1.0 };
        let a_val = -sgn_r * (r.abs() + (r2 - q3).sqrt()).powf(1.0 / 3.0);
        let b_val = q_ratio / a_val;
        let x0 = a_val + b_val - a / 3.0;
        (1, vec![x0])
    }
}