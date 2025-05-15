pub fn cblas_dscal(
    n: i32,
    alpha: f64,
    x: &mut [f64],
    inc_x: i32,
) {
    if inc_x <= 0 {
        return;
    }

    let mut ix = 0;
    for _ in 0..n {
        if (ix as usize) < x.len() {
            x[ix as usize] *= alpha;
        }
        ix += inc_x;
    }
}