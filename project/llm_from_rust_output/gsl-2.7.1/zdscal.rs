pub fn cblas_zdscal(
    n: i32,
    alpha: f64,
    x: &mut [f64],
    inc_x: i32,
) {
    if inc_x <= 0 || n <= 0 {
        return;
    }

    let mut ix = 0;
    for _ in 0..n {
        if ix * 2 + 1 >= x.len() {
            break;
        }
        x[ix * 2] *= alpha;
        x[ix * 2 + 1] *= alpha;
        ix += inc_x as usize;
    }
}