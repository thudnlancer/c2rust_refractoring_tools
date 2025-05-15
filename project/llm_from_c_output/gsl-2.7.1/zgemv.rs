use ndarray::{Array2, Array1};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("invalid order parameter")]
    InvalidOrder,
    #[error("invalid transpose parameter")]
    InvalidTranspose,
    #[error("invalid matrix dimensions")]
    InvalidDimensions,
    #[error("invalid increment value")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

pub fn cblas_zgemv(
    order: CblasOrder,
    trans_a: CblasTranspose,
    m: usize,
    n: usize,
    alpha: Complex64,
    a: &Array2<Complex64>,
    lda: usize,
    x: &Array1<Complex64>,
    inc_x: usize,
    beta: Complex64,
    y: &mut Array1<Complex64>,
    inc_y: usize,
) -> Result<(), BlasError> {
    // Validate parameters
    if m == 0 || n == 0 {
        return Ok(());
    }

    if a.shape()[0] < m || a.shape()[1] < n {
        return Err(BlasError::InvalidDimensions);
    }

    if lda < if order == CblasOrder::RowMajor { n } else { m } {
        return Err(BlasError::InvalidLeadingDimension);
    }

    if inc_x <= 0 || inc_y <= 0 {
        return Err(BlasError::InvalidIncrement);
    }

    let x_len = if trans_a == CblasTranspose::NoTrans { n } else { m };
    if x.len() < 1 + (x_len - 1) * inc_x {
        return Err(BlasError::InvalidDimensions);
    }

    if y.len() < 1 + (m - 1) * inc_y {
        return Err(BlasError::InvalidDimensions);
    }

    // Perform the matrix-vector multiplication
    match (order, trans_a) {
        (CblasOrder::RowMajor, CblasTranspose::NoTrans) => {
            for i in 0..m {
                let mut temp = Complex64::new(0.0, 0.0);
                for j in 0..n {
                    temp += a[(i, j)] * x[j * inc_x];
                }
                y[i * inc_y] = alpha * temp + beta * y[i * inc_y];
            }
        }
        (CblasOrder::RowMajor, CblasTranspose::Trans) => {
            for j in 0..n {
                let mut temp = Complex64::new(0.0, 0.0);
                for i in 0..m {
                    temp += a[(i, j)] * x[i * inc_x];
                }
                y[j * inc_y] = alpha * temp + beta * y[j * inc_y];
            }
        }
        (CblasOrder::RowMajor, CblasTranspose::ConjTrans) => {
            for j in 0..n {
                let mut temp = Complex64::new(0.0, 0.0);
                for i in 0..m {
                    temp += a[(i, j)].conj() * x[i * inc_x];
                }
                y[j * inc_y] = alpha * temp + beta * y[j * inc_y];
            }
        }
        (CblasOrder::ColMajor, CblasTranspose::NoTrans) => {
            for j in 0..n {
                let x_j = x[j * inc_x];
                for i in 0..m {
                    y[i * inc_y] = alpha * a[(i, j)] * x_j + beta * y[i * inc_y];
                }
            }
        }
        (CblasOrder::ColMajor, CblasTranspose::Trans) => {
            for i in 0..m {
                let mut temp = Complex64::new(0.0, 0.0);
                for j in 0..n {
                    temp += a[(i, j)] * x[j * inc_x];
                }
                y[i * inc_y] = alpha * temp + beta * y[i * inc_y];
            }
        }
        (CblasOrder::ColMajor, CblasTranspose::ConjTrans) => {
            for i in 0..m {
                let mut temp = Complex64::new(0.0, 0.0);
                for j in 0..n {
                    temp += a[(i, j)].conj() * x[j * inc_x];
                }
                y[i * inc_y] = alpha * temp + beta * y[i * inc_y];
            }
        }
    }

    Ok(())
}