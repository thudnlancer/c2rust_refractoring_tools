use ndarray::{Array2, ArrayView2};
use num_traits::Zero;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("invalid order parameter")]
    InvalidOrder,
    #[error("invalid side parameter")]
    InvalidSide,
    #[error("invalid uplo parameter")]
    InvalidUplo,
    #[error("invalid transa parameter")]
    InvalidTransA,
    #[error("invalid diag parameter")]
    InvalidDiag,
    #[error("invalid matrix dimensions")]
    InvalidDimensions,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasUplo {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasDiag {
    NonUnit,
    Unit,
}

pub fn cblas_dtrmm(
    order: CblasOrder,
    side: CblasSide,
    uplo: CblasUplo,
    transa: CblasTranspose,
    diag: CblasDiag,
    m: usize,
    n: usize,
    alpha: f64,
    a: ArrayView2<f64>,
    b: &mut Array2<f64>,
) -> Result<(), BlasError> {
    // Validate matrix dimensions
    if a.is_empty() || b.is_empty() {
        return Err(BlasError::InvalidDimensions);
    }

    // Perform the triangular matrix multiplication
    match side {
        CblasSide::Left => {
            if a.shape()[0] < m || a.shape()[1] < m {
                return Err(BlasError::InvalidDimensions);
            }
            if b.shape()[0] < m || b.shape()[1] < n {
                return Err(BlasError::InvalidDimensions);
            }

            for i in 0..m {
                for j in 0..n {
                    let mut temp = alpha * b[(i, j)];
                    match uplo {
                        CblasUplo::Upper => {
                            for k in 0..i {
                                temp += a[(k, i)] * b[(k, j)];
                            }
                        }
                        CblasUplo::Lower => {
                            for k in i + 1..m {
                                temp += a[(k, i)] * b[(k, j)];
                            }
                        }
                    }
                    if diag == CblasDiag::NonUnit {
                        temp *= a[(i, i)];
                    }
                    b[(i, j)] = temp;
                }
            }
        }
        CblasSide::Right => {
            if a.shape()[0] < n || a.shape()[1] < n {
                return Err(BlasError::InvalidDimensions);
            }
            if b.shape()[0] < m || b.shape()[1] < n {
                return Err(BlasError::InvalidDimensions);
            }

            for j in 0..n {
                for i in 0..m {
                    let mut temp = alpha * b[(i, j)];
                    match uplo {
                        CblasUplo::Upper => {
                            for k in 0..j {
                                temp += b[(i, k)] * a[(k, j)];
                            }
                        }
                        CblasUplo::Lower => {
                            for k in j + 1..n {
                                temp += b[(i, k)] * a[(k, j)];
                            }
                        }
                    }
                    if diag == CblasDiag::NonUnit {
                        temp *= a[(j, j)];
                    }
                    b[(i, j)] = temp;
                }
            }
        }
    }

    Ok(())
}