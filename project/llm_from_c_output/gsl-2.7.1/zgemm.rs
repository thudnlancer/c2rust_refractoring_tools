use ndarray::prelude::*;
use num_complex::Complex64;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("invalid matrix dimensions")]
    InvalidDimension,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("invalid transpose parameter")]
    InvalidTranspose,
    #[error("invalid order parameter")]
    InvalidOrder,
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

pub fn cblas_zgemm(
    order: CblasOrder,
    trans_a: CblasTranspose,
    trans_b: CblasTranspose,
    m: usize,
    n: usize,
    k: usize,
    alpha: Complex64,
    a: &Array2<Complex64>,
    lda: usize,
    b: &Array2<Complex64>,
    ldb: usize,
    beta: Complex64,
    c: &mut Array2<Complex64>,
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

    if a.dim() != (a_rows, a_cols) || b.dim() != (b_rows, b_cols) || c.dim() != (m, n) {
        return Err(BlasError::InvalidDimension);
    }

    if lda < a_rows || ldb < b_rows || ldc < m {
        return Err(BlasError::InvalidLeadingDimension);
    }

    // Perform matrix multiplication
    let a_view = match trans_a {
        CblasTranspose::NoTrans => a.view(),
        CblasTranspose::Trans => a.t(),
        CblasTranspose::ConjTrans => a.t().mapv(|x| x.conj()),
    };

    let b_view = match trans_b {
        CblasTranspose::NoTrans => b.view(),
        CblasTranspose::Trans => b.t(),
        CblasTranspose::ConjTrans => b.t().mapv(|x| x.conj()),
    };

    let mut result = a_view.dot(&b_view) * alpha;
    if beta != Complex64::new(0.0, 0.0) {
        result += c * beta;
    }

    *c = result;

    Ok(())
}