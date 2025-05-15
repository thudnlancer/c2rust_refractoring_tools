use ndarray::{Array1, Array2};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CblasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid transpose parameter")]
    InvalidTranspose,
    #[error("Invalid dimension parameters")]
    InvalidDimensions,
    #[error("Invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("Invalid increment parameters")]
    InvalidIncrement,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

pub fn cblas_cgbmv(
    order: CblasOrder,
    trans_a: CblasTranspose,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: Complex32,
    a: &Array2<Complex32>,
    lda: i32,
    x: &Array1<Complex32>,
    inc_x: i32,
    beta: Complex32,
    y: &mut Array1<Complex32>,
    inc_y: i32,
) -> Result<(), CblasError> {
    // Validate parameters
    if m < 0 || n < 0 || kl < 0 || ku < 0 {
        return Err(CblasError::InvalidDimensions);
    }
    if lda < kl + ku + 1 {
        return Err(CblasError::InvalidLeadingDimension);
    }
    if inc_x == 0 || inc_y == 0 {
        return Err(CblasError::InvalidIncrement);
    }

    let len_x = if trans_a == CblasTranspose::NoTrans {
        n
    } else {
        m
    };
    let len_y = if trans_a == CblasTranspose::NoTrans {
        m
    } else {
        n
    };

    if x.len() != (len_x as usize).max(1) || y.len() != (len_y as usize).max(1) {
        return Err(CblasError::InvalidDimensions);
    }

    // Scale Y by beta
    if beta != Complex32::new(1.0, 0.0) {
        for elem in y.iter_mut() {
            *elem = *elem * beta;
        }
    }

    // Perform matrix-vector multiplication
    match trans_a {
        CblasTranspose::NoTrans => {
            for i in 0..m {
                let mut temp = Complex32::new(0.0, 0.0);
                let start = 0.max(i - kl) as usize;
                let end = (n as usize).min(i as usize + ku + 1);

                for j in start..end {
                    let a_index = match order {
                        CblasOrder::RowMajor => (i as usize, j as usize),
                        CblasOrder::ColMajor => (j as usize, i as usize),
                    };
                    temp += a[a_index] * x[j];
                }
                y[i as usize] += alpha * temp;
            }
        }
        CblasTranspose::Trans | CblasTranspose::ConjTrans => {
            for j in 0..n {
                let mut temp = Complex32::new(0.0, 0.0);
                let start = 0.max(j - ku) as usize;
                let end = (m as usize).min(j as usize + kl + 1);

                for i in start..end {
                    let a_index = match order {
                        CblasOrder::RowMajor => (i as usize, j as usize),
                        CblasOrder::ColMajor => (j as usize, i as usize),
                    };
                    let a_val = if trans_a == CblasTranspose::Trans {
                        a[a_index]
                    } else {
                        a[a_index].conj()
                    };
                    temp += a_val * x[i];
                }
                y[j as usize] += alpha * temp;
            }
        }
    }

    Ok(())
}