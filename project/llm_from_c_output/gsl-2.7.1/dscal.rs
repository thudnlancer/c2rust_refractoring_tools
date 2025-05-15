use ndarray::{ArrayViewMut, Array1};
use num_traits::Float;

pub fn cblas_dscal<N: Float>(n: usize, alpha: N, x: &mut ArrayViewMut<'_, N>, inc_x: usize) {
    if n == 0 || inc_x == 0 {
        return;
    }

    let len = x.len();
    if inc_x == 1 {
        for i in 0..len {
            x[i] = x[i] * alpha;
        }
    } else {
        let mut ix = 0;
        for _ in 0..n {
            if ix >= len {
                break;
            }
            x[ix] = x[ix] * alpha;
            ix += inc_x;
        }
    }
}