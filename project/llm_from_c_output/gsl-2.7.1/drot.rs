use ndarray::{ArrayViewMut1, ArrayView1};

/// Applies a plane rotation to vectors X and Y.
///
/// # Arguments
/// * `n` - Number of elements in the vectors X and Y.
/// * `x` - Mutable view of vector X.
/// * `inc_x` - Stride between consecutive elements of X.
/// * `y` - Mutable view of vector Y.
/// * `inc_y` - Stride between consecutive elements of Y.
/// * `c` - Cosine of the rotation angle.
/// * `s` - Sine of the rotation angle.
pub fn cblas_drot(
    n: usize,
    x: ArrayViewMut1<f64>,
    inc_x: usize,
    y: ArrayViewMut1<f64>,
    inc_y: usize,
    c: f64,
    s: f64,
) {
    if n == 0 {
        return;
    }

    if inc_x == 1 && inc_y == 1 {
        for i in 0..n {
            let temp = c * x[i] + s * y[i];
            y[i] = c * y[i] - s * x[i];
            x[i] = temp;
        }
    } else {
        let mut ix = 0;
        let mut iy = 0;
        for _ in 0..n {
            let temp = c * x[ix] + s * y[iy];
            y[iy] = c * y[iy] - s * x[ix];
            x[ix] = temp;
            ix += inc_x;
            iy += inc_y;
        }
    }
}