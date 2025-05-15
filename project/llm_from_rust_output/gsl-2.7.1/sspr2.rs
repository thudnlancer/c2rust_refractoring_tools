use num_traits::Zero;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug)]
pub enum CBLASError {
    InvalidOrder,
    InvalidUplo,
    InvalidN,
    InvalidIncX,
    InvalidIncY,
    UnrecognizedOperation,
}

impl fmt::Display for CBLASError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBLASError::InvalidOrder => write!(f, "Invalid order parameter"),
            CBLASError::InvalidUplo => write!(f, "Invalid uplo parameter"),
            CBLASError::InvalidN => write!(f, "Invalid N parameter"),
            CBLASError::InvalidIncX => write!(f, "Invalid incX parameter"),
            CBLASError::InvalidIncY => write!(f, "Invalid incY parameter"),
            CBLASError::UnrecognizedOperation => write!(f, "Unrecognized operation"),
        }
    }
}

pub fn cblas_sspr2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: f32,
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
    ap: &mut [f32],
) -> Result<(), CBLASError> {
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        return Err(CBLASError::InvalidOrder);
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        return Err(CBLASError::InvalidUplo);
    }
    if n < 0 {
        return Err(CBLASError::InvalidN);
    }
    if inc_x == 0 {
        return Err(CBLASError::InvalidIncX);
    }
    if inc_y == 0 {
        return Err(CBLASError::InvalidIncY);
    }
    if n == 0 || alpha.is_zero() {
        return Ok(());
    }

    let n = n as usize;
    let expected_x_len = if inc_x > 0 {
        n
    } else {
        1 + (n - 1) * (-inc_x) as usize
    };
    let expected_y_len = if inc_y > 0 {
        n
    } else {
        1 + (n - 1) * (-inc_y) as usize
    };

    if x.len() < expected_x_len || y.len() < expected_y_len || ap.len() < n * (n + 1) / 2 {
        return Err(CBLASError::UnrecognizedOperation);
    }

    match (order, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * (-inc_x) as usize };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * (-inc_y) as usize };

            for i in 0..n {
                let tmp1 = (alpha * x[ix]) as f64;
                let tmp2 = (alpha * y[iy]) as f64;
                let mut jx = ix;
                let mut jy = iy;

                for j in i..n {
                    let pos = (i * (2 * n - i - 1)) / 2 + j - i;
                    ap[pos] = (ap[pos] as f64 + tmp1 * y[jy] as f64 + tmp2 * x[jx] as f64) as f32;
                    jx = (jx as i32 + inc_x) as usize;
                    jy = (jy as i32 + inc_y) as usize;
                }

                ix = (ix as i32 + inc_x) as usize;
                iy = (iy as i32 + inc_y) as usize;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * (-inc_x) as usize };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * (-inc_y) as usize };

            for i in 0..n {
                let tmp1 = (alpha * x[ix]) as f64;
                let tmp2 = (alpha * y[iy]) as f64;
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * (-inc_x) as usize };
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * (-inc_y) as usize };

                for j in 0..=i {
                    let pos = i * (i + 1) / 2 + j;
                    ap[pos] = (ap[pos] as f64 + tmp1 * y[jy] as f64 + tmp2 * x[jx] as f64) as f32;
                    jx = (jx as i32 + inc_x) as usize;
                    jy = (jy as i32 + inc_y) as usize;
                }

                ix = (ix as i32 + inc_x) as usize;
                iy = (iy as i32 + inc_y) as usize;
            }
        }
        _ => return Err(CBLASError::UnrecognizedOperation),
    }

    Ok(())
}