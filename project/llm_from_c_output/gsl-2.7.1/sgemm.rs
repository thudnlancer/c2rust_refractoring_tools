use ndarray::{Array2, ArrayView2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid matrix dimensions")]
    InvalidDimensions,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("invalid transpose parameter")]
    InvalidTranspose,
    #[error("invalid order parameter")]
    InvalidOrder,
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

pub fn cblas_sgemm(
    order: CblasOrder,
    trans_a: CblasTranspose,
    trans_b: CblasTranspose,
    m: usize,
    n: usize,
    k: usize,
    alpha: f32,
    a: ArrayView2<f32>,
    lda: usize,
    b: ArrayView2<f32>,
    ldb: usize,
    beta: f32,
    c: &mut Array2<f32>,
    ldc: usize,
) -> Result<(), BlasError> {
    // Validate dimensions
    if m == 0 || n == 0 || k == 0 {
        return Ok(());
    }

    // Validate leading dimensions
    let (a_rows, a_cols) = match trans_a {
        CblasTranspose::NoTrans => (m, k),
        _ => (k, m),
    };
    let (b_rows, b_cols) = match trans_b {
        CblasTranspose::NoTrans => (k, n),
        _ => (n, k),
    };

    if a.shape()[0] != a_rows || a.shape()[1] != a_cols {
        return Err(BlasError::InvalidDimensions);
    }
    if b.shape()[0] != b_rows || b.shape()[1] != b_cols {
        return Err(BlasError::InvalidDimensions);
    }
    if c.shape()[0] != m || c.shape()[1] != n {
        return Err(BlasError::InvalidDimensions);
    }

    // Perform matrix multiplication
    match order {
        CblasOrder::RowMajor => {
            let a_t = match trans_a {
                CblasTranspose::NoTrans => a.view(),
                CblasTranspose::Trans => a.t(),
                CblasTranspose::ConjTrans => a.t(), // Assuming real data
            };
            let b_t = match trans_b {
                CblasTranspose::NoTrans => b.view(),
                CblasTranspose::Trans => b.t(),
                CblasTranspose::ConjTrans => b.t(), // Assuming real data
            };

            *c = alpha * a_t.dot(&b_t) + beta * c.view();
        }
        CblasOrder::ColMajor => {
            // For column-major, we can transpose everything and use row-major operation
            let a_t = match trans_a {
                CblasTranspose::NoTrans => a.t(),
                CblasTranspose::Trans => a.view(),
                CblasTranspose::ConjTrans => a.view(), // Assuming real data
            };
            let b_t = match trans_b {
                CblasTranspose::NoTrans => b.t(),
                CblasTranspose::Trans => b.view(),
                CblasTranspose::ConjTrans => b.view(), // Assuming real data
            };
            let mut c_t = c.t().to_owned();

            c_t = alpha * a_t.dot(&b_t) + beta * c_t.view();
            *c = c_t.t().to_owned();
        }
    }

    Ok(())
}