use ndarray::{Array2, Array1};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CBlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid uplo parameter")]
    InvalidUplo,
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("Invalid increment value")]
    InvalidIncrement,
    #[error("Invalid leading dimension")]
    InvalidLeadingDimension,
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

pub fn cblas_cher2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: usize,
    alpha: Complex32,
    x: &Array1<Complex32>,
    inc_x: usize,
    y: &Array1<Complex32>,
    inc_y: usize,
    a: &mut Array2<Complex32>,
    lda: usize,
) -> Result<(), CBlasError> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err(CBlasError::InvalidIncrement);
    }

    if inc_y == 0 {
        return Err(CBlasError::InvalidIncrement);
    }

    if lda < n.max(1) {
        return Err(CBlasError::InvalidLeadingDimension);
    }

    let x_len = (n - 1) * inc_x + 1;
    let y_len = (n - 1) * inc_y + 1;

    if x.len() < x_len || y.len() < y_len || a.dim().0 < n || a.dim().1 < n {
        return Err(CBlasError::InvalidDimension(
            "Input array dimensions are too small".to_string(),
        ));
    }

    match uplo {
        CBLAS_UPLO::Upper => {
            for i in 0..n {
                for j in i..n {
                    let xi = x[i * inc_x];
                    let yi = y[i * inc_y];
                    let xj = x[j * inc_x];
                    let yj = y[j * inc_y];
                    
                    let value = alpha * xi * yj.conj() + alpha.conj() * yi * xj.conj();
                    a[[i, j]] += value;
                }
            }
        }
        CBLAS_UPLO::Lower => {
            for i in 0..n {
                for j in 0..=i {
                    let xi = x[i * inc_x];
                    let yi = y[i * inc_y];
                    let xj = x[j * inc_x];
                    let yj = y[j * inc_y];
                    
                    let value = alpha * xi * yj.conj() + alpha.conj() * yi * xj.conj();
                    a[[i, j]] += value;
                }
            }
        }
    }

    Ok(())
}