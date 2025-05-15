use ndarray::{Array1, Array2};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CblasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimension")]
    InvalidDimension,
    #[error("invalid stride")]
    InvalidStride,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasUplo {
    Upper,
    Lower,
}

pub fn cblas_chbmv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    k: usize,
    alpha: Complex32,
    a: &Array2<Complex32>,
    lda: usize,
    x: &Array1<Complex32>,
    inc_x: usize,
    beta: Complex32,
    y: &mut Array1<Complex32>,
    inc_y: usize,
) -> Result<(), CblasError> {
    if n == 0 {
        return Ok(());
    }

    if k >= n {
        return Err(CblasError::InvalidDimension);
    }

    if inc_x == 0 || inc_y == 0 {
        return Err(CblasError::InvalidStride);
    }

    if x.len() < 1 + (n - 1) * inc_x || y.len() < 1 + (n - 1) * inc_y {
        return Err(CblasError::InvalidDimension);
    }

    if a.shape()[0] < lda || a.shape()[1] < n {
        return Err(CblasError::InvalidDimension);
    }

    // Scale Y by beta
    if beta != Complex32::new(1.0, 0.0) {
        for i in (0..n).step_by(inc_y) {
            y[i] = y[i] * beta;
        }
    }

    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = Complex32::new(0.0, 0.0);
                let kd = k.min(n - i - 1);
                for j in 1..=kd {
                    let idx = match order {
                        CblasOrder::RowMajor => (i, i + j),
                        CblasOrder::ColMajor => (i + j, i),
                    };
                    y[(i + j) * inc_y] += temp1 * a[idx].conj();
                    temp2 += a[idx] * x[(i + j) * inc_x];
                }
                y[i * inc_y] += temp1 * a[(i, i)].re + alpha * temp2;
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = Complex32::new(0.0, 0.0);
                let kd = k.min(i);
                for j in 1..=kd {
                    let idx = match order {
                        CblasOrder::RowMajor => (i, i - j),
                        CblasOrder::ColMajor => (i - j, i),
                    };
                    y[(i - j) * inc_y] += temp1 * a[idx].conj();
                    temp2 += a[idx] * x[(i - j) * inc_x];
                }
                y[i * inc_y] += temp1 * a[(i, i)].re + alpha * temp2;
            }
        }
    }

    Ok(())
}