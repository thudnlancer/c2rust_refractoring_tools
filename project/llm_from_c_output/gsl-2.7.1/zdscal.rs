use num_complex::Complex64;

pub fn cblas_zdscal(n: usize, alpha: f64, x: &mut [Complex64], inc_x: usize) {
    if n == 0 || inc_x == 0 {
        return;
    }

    if inc_x == 1 {
        for elem in x.iter_mut().take(n) {
            *elem *= alpha;
        }
    } else {
        let mut ix = 0;
        for _ in 0..n {
            x[ix] *= alpha;
            ix += inc_x;
            if ix >= x.len() {
                break;
            }
        }
    }
}