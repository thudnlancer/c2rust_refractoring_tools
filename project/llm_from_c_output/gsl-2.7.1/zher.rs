use ndarray::{Array2, Array1};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CBlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid uplo parameter")]
    InvalidUplo,
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("Invalid increment: {0}")]
    InvalidIncrement(String),
    #[error("Invalid leading dimension: {0}")]
    InvalidLeadingDimension(String),
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_ORDER {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

pub fn cblas_zher(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: usize,
    alpha: f64,
    x: &Array1<Complex64>,
    inc_x: isize,
    a: &mut Array2<Complex64>,
    lda: usize,
) -> Result<(), CBlasError> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err(CBlasError::InvalidIncrement("inc_x cannot be zero".to_string()));
    }

    if lda < n.max(1) {
        return Err(CBlasError::InvalidLeadingDimension(
            "lda must be at least max(1, n)".to_string(),
        ));
    }

    if x.len() < ((n - 1) / inc_x.unsigned_abs() + 1) {
        return Err(CBlasError::InvalidDimension(
            "x length insufficient for n and inc_x".to_string(),
        ));
    }

    let alpha_complex = Complex64::new(alpha, 0.0);

    match uplo {
        CBLAS_UPLO::Upper => {
            for j in 0..n {
                let x_j = x[(j * inc_x.unsigned_abs())];
                let temp = alpha_complex * x_j.conj();

                for i in 0..j {
                    let x_i = x[(i * inc_x.unsigned_abs())];
                    a[(i, j)] += x_i * temp;
                }

                let a_jj = a[(j, j)].re + (x_j * temp).re;
                a[(j, j)] = Complex64::new(a_jj, 0.0);
            }
        }
        CBLAS_UPLO::Lower => {
            for j in 0..n {
                let x_j = x[(j * inc_x.unsigned_abs())];
                let temp = alpha_complex * x_j.conj();

                let a_jj = a[(j, j)].re + (x_j * temp).re;
                a[(j, j)] = Complex64::new(a_jj, 0.0);

                for i in j + 1..n {
                    let x_i = x[(i * inc_x.unsigned_abs())];
                    a[(i, j)] += x_i * temp;
                }
            }
        }
    }

    Ok(())
}