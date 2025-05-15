use ndarray::{Array2, ArrayView2};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CBlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid side")]
    InvalidSide,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimension")]
    InvalidDimension,
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_ORDER {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_SIDE {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

pub fn cblas_csymm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    m: usize,
    n: usize,
    alpha: Complex32,
    a: ArrayView2<Complex32>,
    lda: usize,
    b: ArrayView2<Complex32>,
    ldb: usize,
    beta: Complex32,
    c: &mut Array2<Complex32>,
    ldc: usize,
) -> Result<(), CBlasError> {
    // Validate dimensions
    if m == 0 || n == 0 {
        return Ok(());
    }

    let (a_rows, a_cols) = match side {
        CBLAS_SIDE::Left => (m, m),
        CBLAS_SIDE::Right => (n, n),
    };

    if a.nrows() != a_rows || a.ncols() != a_cols {
        return Err(CBlasError::InvalidDimension);
    }

    if b.nrows() != m || b.ncols() != n {
        return Err(CBlasError::InvalidDimension);
    }

    if c.nrows() != m || c.ncols() != n {
        return Err(CBlasError::InvalidDimension);
    }

    // Perform the symmetric matrix multiplication
    match side {
        CBLAS_SIDE::Left => {
            // C = alpha * A * B + beta * C
            for i in 0..m {
                for j in 0..n {
                    let mut sum = Complex32::new(0.0, 0.0);
                    for k in 0..m {
                        let a_val = match uplo {
                            CBLAS_UPLO::Upper if k <= i => a[(i, k)],
                            CBLAS_UPLO::Lower if k >= i => a[(i, k)],
                            _ => a[(k, i)].conj(),
                        };
                        sum += a_val * b[(k, j)];
                    }
                    c[(i, j)] = alpha * sum + beta * c[(i, j)];
                }
            }
        }
        CBLAS_SIDE::Right => {
            // C = alpha * B * A + beta * C
            for i in 0..m {
                for j in 0..n {
                    let mut sum = Complex32::new(0.0, 0.0);
                    for k in 0..n {
                        let a_val = match uplo {
                            CBLAS_UPLO::Upper if k <= j => a[(j, k)],
                            CBLAS_UPLO::Lower if k >= j => a[(j, k)],
                            _ => a[(k, j)].conj(),
                        };
                        sum += b[(i, k)] * a_val;
                    }
                    c[(i, j)] = alpha * sum + beta * c[(i, j)];
                }
            }
        }
    }

    Ok(())
}