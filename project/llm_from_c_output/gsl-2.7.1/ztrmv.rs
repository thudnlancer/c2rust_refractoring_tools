use blas_sys::cblas::{CblasOrder, CblasUplo, CblasTranspose, CblasDiag};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BlasError {
    InvalidOrder,
    InvalidUplo,
    InvalidTranspose,
    InvalidDiag,
    InvalidDimension,
    InvalidIncrement,
}

impl fmt::Display for BlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlasError::InvalidOrder => write!(f, "Invalid order parameter"),
            BlasError::InvalidUplo => write!(f, "Invalid uplo parameter"),
            BlasError::InvalidTranspose => write!(f, "Invalid transpose parameter"),
            BlasError::InvalidDiag => write!(f, "Invalid diag parameter"),
            BlasError::InvalidDimension => write!(f, "Invalid dimension parameter"),
            BlasError::InvalidIncrement => write!(f, "Invalid increment parameter"),
        }
    }
}

impl Error for BlasError {}

pub fn cblas_ztrmv(
    order: CblasOrder,
    uplo: CblasUplo,
    trans_a: CblasTranspose,
    diag: CblasDiag,
    n: i32,
    a: &[f64],
    lda: i32,
    x: &mut [f64],
    inc_x: i32,
) -> Result<(), BlasError> {
    if n < 0 {
        return Err(BlasError::InvalidDimension);
    }
    if lda < n.max(1) {
        return Err(BlasError::InvalidDimension);
    }
    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    // Parameter validation
    match order {
        CblasOrder::RowMajor | CblasOrder::ColMajor => (),
        _ => return Err(BlasError::InvalidOrder),
    }

    match uplo {
        CblasUplo::Upper | CblasUplo::Lower => (),
        _ => return Err(BlasError::InvalidUplo),
    }

    match trans_a {
        CblasTranspose::NoTrans | CblasTranspose::Trans | CblasTranspose::ConjTrans => (),
        _ => return Err(BlasError::InvalidTranspose),
    }

    match diag {
        CblasDiag::NonUnit | CblasDiag::Unit => (),
        _ => return Err(BlasError::InvalidDiag),
    }

    // Call the BLAS implementation
    unsafe {
        blas_sys::cblas_ztrmv(
            order,
            uplo,
            trans_a,
            diag,
            n,
            a.as_ptr() as *const _,
            lda,
            x.as_mut_ptr() as *mut _,
            inc_x,
        );
    }

    Ok(())
}