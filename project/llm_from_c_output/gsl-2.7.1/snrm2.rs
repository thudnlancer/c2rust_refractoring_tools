pub fn cblas_snrm2(n: i32, x: &[f32], inc_x: i32) -> f32 {
    if n <= 0 || inc_x <= 0 {
        return 0.0;
    }

    let mut scale = 0.0;
    let mut ssq = 1.0;
    let mut abs_xi;

    let mut ix = 0;
    for _ in 0..n {
        if ix >= x.len() {
            break;
        }
        abs_xi = x[ix].abs();
        if abs_xi != 0.0 {
            if scale < abs_xi {
                ssq = 1.0 + ssq * (scale / abs_xi).powi(2);
                scale = abs_xi;
            } else {
                ssq += (abs_xi / scale).powi(2);
            }
        }
        ix = (ix as i32 + inc_x) as usize;
    }

    scale * ssq.sqrt()
}