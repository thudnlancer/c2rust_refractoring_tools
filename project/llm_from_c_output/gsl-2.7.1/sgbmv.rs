use ndarray::{Array1, Array2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid transpose parameter")]
    InvalidTranspose,
    #[error("Invalid dimension")]
    InvalidDimension,
    #[error("Invalid increment")]
    InvalidIncrement,
    #[error("Invalid leading dimension")]
    InvalidLeadingDimension,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

pub fn cblas_sgbmv(
    order: CblasOrder,
    trans: CblasTranspose,
    m: usize,
    n: usize,
    kl: usize,
    ku: usize,
    alpha: f32,
    a: &Array2<f32>,
    lda: usize,
    x: &Array1<f32>,
    inc_x: usize,
    beta: f32,
    y: &mut Array1<f32>,
    inc_y: usize,
) -> Result<(), BlasError> {
    // Validate parameters
    if m == 0 || n == 0 {
        return Err(BlasError::InvalidDimension);
    }
    if kl >= m || ku >= n {
        return Err(BlasError::InvalidDimension);
    }
    if inc_x == 0 || inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }
    if lda < kl + ku + 1 {
        return Err(BlasError::InvalidLeadingDimension);
    }

    let (n_rows, n_cols) = match trans {
        CblasTranspose::NoTrans => (m, n),
        _ => (n, m),
    };

    if x.len() < 1 + (n_cols - 1) * inc_x {
        return Err(BlasError::InvalidDimension);
    }
    if y.len() < 1 + (n_rows - 1) * inc_y {
        return Err(BlasError::InvalidDimension);
    }

    // Scale Y by beta if needed
    if beta != 1.0 {
        if beta == 0.0 {
            for i in (0..n_rows).step_by(inc_y) {
                y[i] = 0.0;
            }
        } else {
            for i in (0..n_rows).step_by(inc_y) {
                y[i] *= beta;
            }
        }
    }

    // Perform matrix-vector multiplication
    match trans {
        CblasTranspose::NoTrans => {
            for i in 0..m {
                let mut temp = 0.0;
                let start = if i > ku { i - ku } else { 0 };
                let end = std::cmp::min(i + kl + 1, n);
                
                for j in start..end {
                    let a_index = match order {
                        CblasOrder::RowMajor => (ku + i - j, j),
                        CblasOrder::ColMajor => (j, ku + j - i),
                    };
                    temp += a[a_index] * x[j * inc_x];
                }
                y[i * inc_y] += alpha * temp;
            }
        }
        _ => {
            for j in 0..n {
                let mut temp = 0.0;
                let start = if j > kl { j - kl } else { 0 };
                let end = std::cmp::min(j + ku + 1, m);
                
                for i in start..end {
                    let a_index = match order {
                        CblasOrder::RowMajor => (ku + i - j, j),
                        CblasOrder::ColMajor => (j, ku + j - i),
                    };
                    temp += a[a_index] * x[i * inc_x];
                }
                y[j * inc_y] += alpha * temp;
            }
        }
    }

    Ok(())
}