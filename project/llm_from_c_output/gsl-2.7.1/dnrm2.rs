pub fn cblas_dnrm2(n: i32, x: &[f64], inc_x: i32) -> f64 {
    if n <= 0 || inc_x <= 0 {
        return 0.0;
    }

    let mut scale = 0.0;
    let mut ssq = 1.0;

    for i in (0..n as usize).step_by(inc_x as usize) {
        if x[i] != 0.0 {
            let abs_xi = x[i].abs();
            if scale < abs_xi {
                ssq = 1.0 + ssq * (scale / abs_xi).powi(2);
                scale = abs_xi;
            } else {
                ssq += (abs_xi / scale).powi(2);
            }
        }
    }

    scale * ssq.sqrt()
}