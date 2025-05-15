use ndarray::{Array2, Array1};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CBlasError {
    #[error("invalid order parameter")]
    InvalidOrder,
    #[error("invalid uplo parameter")]
    InvalidUplo,
    #[error("invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("invalid increment value: {0}")]
    InvalidIncrement(i32),
    #[error("invalid leading dimension: {0}")]
    InvalidLeadingDimension(i32),
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

pub fn cblas_zher2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex64,
    x: &Array1<Complex64>,
    inc_x: i32,
    y: &Array1<Complex64>,
    inc_y: i32,
    a: &mut Array2<Complex64>,
    lda: i32,
) -> Result<(), CBlasError> {
    if n < 0 {
        return Err(CBlasError::InvalidDimension("n must be non-negative".to_string()));
    }
    if inc_x == 0 {
        return Err(CBlasError::InvalidIncrement(inc_x));
    }
    if inc_y == 0 {
        return Err(CBlasError::InvalidIncrement(inc_y));
    }
    if lda < n.max(1) {
        return Err(CBlasError::InvalidLeadingDimension(lda));
    }

    let n_usize = n as usize;
    if x.len() < ((n_usize - 1) * inc_x.abs() as usize + 1) {
        return Err(CBlasError::InvalidDimension("x length too small".to_string()));
    }
    if y.len() < ((n_usize - 1) * inc_y.abs() as usize + 1) {
        return Err(CBlasError::InvalidDimension("y length too small".to_string()));
    }
    if a.shape() != [n_usize, n_usize] {
        return Err(CBlasError::InvalidDimension("a dimensions mismatch".to_string()));
    }

    match uplo {
        CBLAS_UPLO::Upper => {
            for i in 0..n_usize {
                for j in i..n_usize {
                    let xi = x[i * inc_x as usize];
                    let yi = y[i * inc_y as usize];
                    let xj = x[j * inc_x as usize];
                    let yj = y[j * inc_y as usize];
                    
                    a[(i, j)] += alpha * xi * yj.conj() + alpha.conj() * yi * xj.conj();
                }
            }
        }
        CBLAS_UPLO::Lower => {
            for i in 0..n_usize {
                for j in 0..=i {
                    let xi = x[i * inc_x as usize];
                    let yi = y[i * inc_y as usize];
                    let xj = x[j * inc_x as usize];
                    let yj = y[j * inc_y as usize];
                    
                    a[(i, j)] += alpha * xi * yj.conj() + alpha.conj() * yi * xj.conj();
                }
            }
        }
    }

    Ok(())
}