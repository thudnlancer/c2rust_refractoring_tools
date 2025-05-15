pub fn cblas_sscal(n: usize, alpha: f32, x: &mut [f32], inc_x: usize) {
    if n == 0 || inc_x == 0 {
        return;
    }

    if inc_x == 1 {
        for val in x.iter_mut().take(n) {
            *val *= alpha;
        }
    } else {
        let mut idx = 0;
        for _ in 0..n {
            x[idx] *= alpha;
            idx += inc_x;
            if idx >= x.len() {
                break;
            }
        }
    }
}