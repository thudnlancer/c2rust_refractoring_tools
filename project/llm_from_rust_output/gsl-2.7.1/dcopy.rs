pub fn cblas_dcopy(
    n: i32,
    x: &[f64],
    inc_x: i32,
    y: &mut [f64],
    inc_y: i32,
) {
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        let xi = ix as usize;
        let yi = iy as usize;
        
        if xi < x.len() && yi < y.len() {
            y[yi] = x[xi];
        }

        ix += inc_x;
        iy += inc_y;
    }
}