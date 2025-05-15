use ndarray::{Array1, Array2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid matrix order")]
    InvalidOrder,
    #[error("invalid transpose operation")]
    InvalidTranspose,
    #[error("invalid matrix dimensions")]
    InvalidDimensions,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("invalid increment value")]
    InvalidIncrement,
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

pub fn cblas_dgemv(
    order: CblasOrder,
    trans_a: CblasTranspose,
    m: usize,
    n: usize,
    alpha: f64,
    a: &Array2<f64>,
    lda: usize,
    x: &Array1<f64>,
    inc_x: usize,
    beta: f64,
    y: &mut Array1<f64>,
    inc_y: usize,
) -> Result<(), BlasError> {
    // Validate input parameters
    if m == 0 || n == 0 {
        return Ok(());
    }

    if a.shape()[0] < m || a.shape()[1] < n {
        return Err(BlasError::InvalidDimensions);
    }

    if lda < if matches!(order, CblasOrder::RowMajor) { n } else { m } {
        return Err(BlasError::InvalidLeadingDimension);
    }

    let x_len = if matches!(trans_a, CblasTranspose::NoTrans) { n } else { m };
    if x.len() < x_len * inc_x {
        return Err(BlasError::InvalidDimensions);
    }

    if y.len() < if matches!(trans_a, CblasTranspose::NoTrans) { m } else { n } * inc_y {
        return Err(BlasError::InvalidDimensions);
    }

    if inc_x == 0 || inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    // Perform matrix-vector multiplication
    match order {
        CblasOrder::RowMajor => {
            match trans_a {
                CblasTranspose::NoTrans => {
                    for i in 0..m {
                        let mut temp = 0.0;
                        for j in 0..n {
                            temp += a[[i, j]] * x[j * inc_x];
                        }
                        y[i * inc_y] = alpha * temp + beta * y[i * inc_y];
                    }
                }
                CblasTranspose::Trans | CblasTranspose::ConjTrans => {
                    for j in 0..n {
                        let mut temp = 0.0;
                        for i in 0..m {
                            temp += a[[i, j]] * x[i * inc_x];
                        }
                        y[j * inc_y] = alpha * temp + beta * y[j * inc_y];
                    }
                }
            }
        }
        CblasOrder::ColMajor => {
            match trans_a {
                CblasTranspose::NoTrans => {
                    for i in 0..m {
                        let mut temp = 0.0;
                        for j in 0..n {
                            temp += a[[j, i]] * x[j * inc_x];
                        }
                        y[i * inc_y] = alpha * temp + beta * y[i * inc_y];
                    }
                }
                CblasTranspose::Trans | CblasTranspose::ConjTrans => {
                    for j in 0..n {
                        let mut temp = 0.0;
                        for i in 0..m {
                            temp += a[[j, i]] * x[i * inc_x];
                        }
                        y[j * inc_y] = alpha * temp + beta * y[j * inc_y];
                    }
                }
            }
        }
    }

    Ok(())
}