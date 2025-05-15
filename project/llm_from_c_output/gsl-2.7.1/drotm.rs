use ndarray::{Array1, ArrayView1, ArrayViewMut1};

pub fn cblas_drotm(
    n: usize,
    x: &mut ArrayViewMut1<f64>,
    inc_x: usize,
    y: &mut ArrayViewMut1<f64>,
    inc_y: usize,
    p: &ArrayView1<f64>,
) {
    if n == 0 {
        return;
    }

    let h11 = p[0];
    let h21 = p[1];
    let h12 = p[2];
    let h22 = p[3];

    if h12 == 0.0 && h21 == 0.0 {
        return;
    }

    for i in 0..n {
        let xi = x[i * inc_x];
        let yi = y[i * inc_y];
        
        x[i * inc_x] = h11 * xi + h12 * yi;
        y[i * inc_y] = h21 * xi + h22 * yi;
    }
}