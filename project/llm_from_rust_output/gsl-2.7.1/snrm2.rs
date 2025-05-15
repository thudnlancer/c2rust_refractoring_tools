pub fn cblas_snrm2(n: i32, x: &[f32], inc_x: i32) -> f32 {
    if n <= 0 || inc_x <= 0 {
        return 0.0;
    } else if n == 1 {
        return x[0].abs();
    }

    let mut scale = 0.0;
    let mut ssq = 1.0;
    let mut ix = 0;

    for _ in 0..n {
        let x_val = x[ix as usize];
        if x_val != 0.0 {
            let ax = x_val.abs();
            if scale < ax {
                ssq = 1.0 + ssq * (scale / ax).powi(2);
                scale = ax;
            } else {
                ssq += (ax / scale).powi(2);
            }
        }
        ix += inc_x;
    }

    (scale * ssq.sqrt()) as f32
}