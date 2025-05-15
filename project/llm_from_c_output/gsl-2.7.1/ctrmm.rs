use blas_sys::{cblas_ctrmm, CBLAS_DIAG, CBLAS_ORDER, CBLAS_SIDE, CBLAS_TRANSPOSE, CBLAS_UPLO};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BlasError {
    InvalidOrder,
    InvalidSide,
    InvalidUplo,
    InvalidTranspose,
    InvalidDiag,
    InvalidDimension,
}

impl fmt::Display for BlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlasError::InvalidOrder => write!(f, "Invalid CBLAS_ORDER value"),
            BlasError::InvalidSide => write!(f, "Invalid CBLAS_SIDE value"),
            BlasError::InvalidUplo => write!(f, "Invalid CBLAS_UPLO value"),
            BlasError::InvalidTranspose => write!(f, "Invalid CBLAS_TRANSPOSE value"),
            BlasError::InvalidDiag => write!(f, "Invalid CBLAS_DIAG value"),
            BlasError::InvalidDimension => write!(f, "Invalid matrix dimension"),
        }
    }
}

impl Error for BlasError {}

pub fn cblas_ctrmm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    m: i32,
    n: i32,
    alpha: &[f32; 2],
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> Result<(), BlasError> {
    if m <= 0 || n <= 0 {
        return Err(BlasError::InvalidDimension);
    }

    unsafe {
        cblas_ctrmm(
            order,
            side,
            uplo,
            trans_a,
            diag,
            m,
            n,
            alpha.as_ptr() as *const _,
            a.as_ptr(),
            lda,
            b.as_mut_ptr(),
            ldb,
        );
    }

    Ok(())
}