use ndarray::{Array1, Array2};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HbmvError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimensions")]
    InvalidDimensions,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("invalid increment")]
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

pub fn cblas_zhbmv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    k: usize,
    alpha: Complex64,
    a: &Array2<Complex64>,
    lda: usize,
    x: &Array1<Complex64>,
    inc_x: usize,
    beta: Complex64,
    y: &mut Array1<Complex64>,
    inc_y: usize,
) -> Result<(), HbmvError> {
    if n == 0 {
        return Ok(());
    }

    if x.len() != ((n - 1) * inc_x + 1) {
        return Err(HbmvError::InvalidDimensions);
    }

    if y.len() != ((n - 1) * inc_y + 1) {
        return Err(HbmvError::InvalidDimensions);
    }

    if lda < k + 1 {
        return Err(HbmvError::InvalidLeadingDimension);
    }

    if inc_x <= 0 {
        return Err(HbmvError::InvalidIncrement);
    }

    if inc_y <= 0 {
        return Err(HbmvError::InvalidIncrement);
    }

    match uplo {
        CblasUplo::Upper | CblasUplo::Lower => {}
        _ => return Err(HbmvError::InvalidUplo),
    }

    // Scale Y by beta
    if beta != Complex64::new(1.0, 0.0) {
        for i in (0..n).step_by(inc_y) {
            y[i] = beta * y[i];
        }
    }

    // Perform the matrix-vector multiplication
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = Complex64::new(0.0, 0.0);
                let k_min = if k > i { 0 } else { i - k };
                
                for j in k_min..i {
                    y[j * inc_y] += temp1 * a[(i - j, j)].conj();
                    temp2 += a[(i - j, j)] * x[j * inc_x];
                }
                
                y[i * inc_y] += temp1 * a[(0, i)].re + alpha * temp2;
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = Complex64::new(0.0, 0.0);
                let k_max = if (i + k + 1) > n { n } else { i + k + 1 };
                
                for j in (i + 1)..k_max {
                    y[j * inc_y] += temp1 * a[(j - i, i)];
                    temp2 += a[(j - i, i)].conj() * x[j * inc_x];
                }
                
                y[i * inc_y] += temp1 * a[(0, i)].re + alpha * temp2;
            }
        }
    }

    Ok(())
}