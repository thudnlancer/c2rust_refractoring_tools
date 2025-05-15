use ndarray::{Array1, ArrayViewMut1};
use num_complex::Complex64;

pub fn cblas_zswap(
    n: usize,
    x: &mut ArrayViewMut1<Complex64>,
    inc_x: usize,
    y: &mut ArrayViewMut1<Complex64>,
    inc_y: usize,
) {
    if n == 0 {
        return;
    }

    let x_slice = x.as_slice_mut().unwrap();
    let y_slice = y.as_slice_mut().unwrap();

    for i in 0..n {
        let x_idx = i * inc_x;
        let y_idx = i * inc_y;
        
        if x_idx >= x_slice.len() || y_idx >= y_slice.len() {
            break;
        }
        
        x_slice.swap(x_idx, y_idx, y_slice[y_idx]);
    }
}