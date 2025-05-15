use ndarray::{Array1, Array2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid uplo parameter")]
    InvalidUplo,
    #[error("Invalid trans parameter")]
    InvalidTrans,
    #[error("Invalid diag parameter")]
    InvalidDiag,
    #[error("Invalid dimension")]
    InvalidDimension,
    #[error("Invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("Invalid increment")]
    InvalidIncrement,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasUplo {
    Upper,
    Lower,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasDiag {
    NonUnit,
    Unit,
}

pub fn cblas_dtrmv(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    diag: CblasDiag,
    n: usize,
    a: &Array2<f64>,
    lda: usize,
    x: &mut Array1<f64>,
    incx: usize,
) -> Result<(), BlasError> {
    // Validate parameters
    if n == 0 {
        return Ok(());
    }

    if a.nrows() != n || a.ncols() != n {
        return Err(BlasError::InvalidDimension);
    }

    if lda < n.max(1) {
        return Err(BlasError::InvalidLeadingDimension);
    }

    if incx == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    // Perform matrix-vector multiplication
    match order {
        CblasOrder::RowMajor | CblasOrder::ColMajor => {
            for i in 0..n {
                let mut temp = 0.0;
                let start = match uplo {
                    CblasUplo::Upper => i,
                    CblasUplo::Lower => 0,
                };
                let end = match uplo {
                    CblasUplo::Upper => n,
                    CblasUplo::Lower => i + 1,
                };

                for j in start..end {
                    let a_val = match order {
                        CblasOrder::RowMajor => a[(i, j)],
                        CblasOrder::ColMajor => a[(j, i)],
                    };
                    let x_val = x[j * incx];
                    temp += a_val * x_val;
                }

                if matches!(diag, CblasDiag::Unit) {
                    temp += x[i * incx];
                }

                x[i * incx] = match trans {
                    CblasTranspose::NoTrans => temp,
                    CblasTranspose::Trans | CblasTranspose::ConjTrans => {
                        let mut temp_trans = 0.0;
                        for j in start..end {
                            let a_val = match order {
                                CblasOrder::RowMajor => a[(j, i)],
                                CblasOrder::ColMajor => a[(i, j)],
                            };
                            let x_val = x[j * incx];
                            temp_trans += a_val * x_val;
                        }
                        if matches!(diag, CblasDiag::Unit) {
                            temp_trans += x[i * incx];
                        }
                        temp_trans
                    }
                };
            }
        }
    }

    Ok(())
}