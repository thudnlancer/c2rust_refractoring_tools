use blas_sys::c::{CBLAS_ORDER, CBLAS_UPLO};
use num_complex::Complex32;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BlasError {
    InvalidOrder,
    InvalidUplo,
    InvalidDimension,
    InvalidIncrement,
    NullPointer,
}

impl fmt::Display for BlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlasError::InvalidOrder => write!(f, "Invalid order parameter"),
            BlasError::InvalidUplo => write!(f, "Invalid uplo parameter"),
            BlasError::InvalidDimension => write!(f, "Invalid dimension parameter"),
            BlasError::InvalidIncrement => write!(f, "Invalid increment parameter"),
            BlasError::NullPointer => write!(f, "Null pointer encountered"),
        }
    }
}

impl Error for BlasError {}

pub fn cblas_chpmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: &Complex32,
    ap: &[Complex32],
    x: &[Complex32],
    inc_x: i32,
    beta: &Complex32,
    y: &mut [Complex32],
    inc_y: i32,
) -> Result<(), BlasError> {
    if n < 0 {
        return Err(BlasError::InvalidDimension);
    }
    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement);
    }
    if inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    let expected_ap_len = (n * (n + 1)) / 2;
    if ap.len() < expected_ap_len as usize {
        return Err(BlasError::InvalidDimension);
    }

    let expected_x_len = if inc_x > 0 {
        1 + (n - 1) * inc_x
    } else {
        1 - (n - 1) * inc_x
    };
    if x.len() < expected_x_len as usize {
        return Err(BlasError::InvalidDimension);
    }

    let expected_y_len = if inc_y > 0 {
        1 + (n - 1) * inc_y
    } else {
        1 - (n - 1) * inc_y
    };
    if y.len() < expected_y_len as usize {
        return Err(BlasError::InvalidDimension);
    }

    unsafe {
        blas_sys::c::cblas_chpmv(
            order,
            uplo,
            n,
            alpha as *const Complex32 as *const _,
            ap.as_ptr() as *const _,
            x.as_ptr() as *const _,
            inc_x,
            beta as *const Complex32 as *const _,
            y.as_mut_ptr() as *mut _,
            inc_y,
        );
    }

    Ok(())
}