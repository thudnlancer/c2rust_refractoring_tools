use ndarray::{ArrayView1, ArrayView2, Axis};
use num_traits::Zero;

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasUplo {
    Upper,
    Lower,
}

pub fn cblas_dsymv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f64,
    a: ArrayView2<f64>,
    lda: usize,
    x: ArrayView1<f64>,
    inc_x: usize,
    beta: f64,
    y: &mut ArrayViewMut1<f64>,
    inc_y: usize,
) -> Result<(), String> {
    if n == 0 {
        return Ok(());
    }

    if a.shape()[0] != n || a.shape()[1] != n {
        return Err("Matrix dimensions must match n".to_string());
    }

    if lda < n {
        return Err("Leading dimension must be >= n".to_string());
    }

    if x.len() < 1 + (n - 1) * inc_x {
        return Err("X vector length too small".to_string());
    }

    if y.len() < 1 + (n - 1) * inc_y {
        return Err("Y vector length too small".to_string());
    }

    // Scale Y by beta
    if !beta.is_one() {
        if beta.is_zero() {
            y.fill(0.0);
        } else {
            *y *= beta;
        }
    }

    // Perform SYMV operation
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = 0.0;
                for j in i + 1..n {
                    y[j * inc_y] += alpha * a[[i, j]] * x[i * inc_x];
                    temp2 += a[[i, j]] * x[j * inc_x];
                }
                y[i * inc_y] += temp1 * a[[i, i]] + alpha * temp2;
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = 0.0;
                for j in 0..i {
                    y[j * inc_y] += alpha * a[[i, j]] * x[i * inc_x];
                    temp2 += a[[i, j]] * x[j * inc_x];
                }
                y[i * inc_y] += temp1 * a[[i, i]] + alpha * temp2;
            }
        }
    }

    Ok(())
}