use ndarray::{Array1, Array2};
use num_complex::Complex64;
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

pub fn cblas_zgbmv(
    order: CblasOrder,
    trans_a: CblasTranspose,
    m: usize,
    n: usize,
    kl: usize,
    ku: usize,
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

    if kl >= m || ku >= n {
        return Err(BlasError::InvalidDimensions);
    }

    if lda < kl + ku + 1 {
        return Err(BlasError::InvalidLeadingDimension);
    }

    if inc_x == 0 || inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    let x_len = if trans_a == CblasTranspose::NoTrans {
        n
    } else {
        m
    };

    let y_len = if trans_a == CblasTranspose::NoTrans {
        m
    } else {
        n
    };

    if x.len() * inc_x < x_len || y.len() * inc_y < y_len {
        return Err(BlasError::InvalidDimensions);
    }

    // Scale Y by beta
    if beta != Complex64::new(1.0, 0.0) {
        for i in 0..y_len {
            y[i * inc_y] = y[i * inc_y] * beta;
        }
    }

    // Perform matrix-vector multiplication
    match trans_a {
        CblasTranspose::NoTrans => {
            for i in 0..m {
                let mut temp = Complex64::new(0.0, 0.0);
                let start = if i > ku { i - ku } else { 0 };
                let end = if i + kl < n { i + kl } else { n - 1 };

                for j in start..=end {
                    let a_index = match order {
                        CblasOrder::RowMajor => (i, j),
                        CblasOrder::ColMajor => (j, i),
                    };
                    temp += a[a_index] * x[j * inc_x];
                }
                y[i * inc_y] += alpha * temp;
            }
        }
        CblasTranspose::Trans | CblasTranspose::ConjTrans => {
            for j in 0..n {
                let mut temp = Complex64::new(0.0, 0.0);
                let start = if j > kl { j - kl } else { 0 };
                let end = if j + ku < m { j + ku } else { m - 1 };

                for i in start..=end {
                    let a_index = match order {
                        CblasOrder::RowMajor => (i, j),
                        CblasOrder::ColMajor => (j, i),
                    };
                    let a_val = match trans_a {
                        CblasTranspose::Trans => a[a_index],
                        CblasTranspose::ConjTrans => a[a_index].conj(),
                        _ => unreachable!(),
                    };
                    temp += a_val * x[i * inc_x];
                }
                y[j * inc_y] += alpha * temp;
            }
        }
    }

    Ok(())
}