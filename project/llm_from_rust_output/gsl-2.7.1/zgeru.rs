use std::ffi::CString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug)]
pub enum BlasError {
    InvalidOrder,
    InvalidM,
    InvalidN,
    InvalidIncX,
    InvalidIncY,
    InvalidLda,
    UnrecognizedOperation,
}

impl std::fmt::Display for BlasError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlasError::InvalidOrder => write!(f, "Invalid order parameter"),
            BlasError::InvalidM => write!(f, "Invalid M parameter"),
            BlasError::InvalidN => write!(f, "Invalid N parameter"),
            BlasError::InvalidIncX => write!(f, "Invalid incX parameter"),
            BlasError::InvalidIncY => write!(f, "Invalid incY parameter"),
            BlasError::InvalidLda => write!(f, "Invalid lda parameter"),
            BlasError::UnrecognizedOperation => write!(f, "Unrecognized operation"),
        }
    }
}

impl std::error::Error for BlasError {}

pub fn cblas_zgeru(
    order: CBLAS_ORDER,
    m: i32,
    n: i32,
    alpha: (f64, f64),
    x: &[(f64, f64)],
    inc_x: i32,
    y: &[(f64, f64)],
    inc_y: i32,
    a: &mut [(f64, f64)],
    lda: i32,
) -> Result<(), BlasError> {
    // Parameter validation
    if m < 0 {
        return Err(BlasError::InvalidM);
    }
    if n < 0 {
        return Err(BlasError::InvalidN);
    }
    if inc_x == 0 {
        return Err(BlasError::InvalidIncX);
    }
    if inc_y == 0 {
        return Err(BlasError::InvalidIncY);
    }

    let min_lda = match order {
        CBLAS_ORDER::RowMajor => n.max(1),
        CBLAS_ORDER::ColMajor => m.max(1),
    };
    if lda < min_lda {
        return Err(BlasError::InvalidLda);
    }

    let (alpha_real, alpha_imag) = alpha;

    match order {
        CBLAS_ORDER::RowMajor => {
            let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
            for i in 0..m {
                let (x_real, x_imag) = x[(ix as usize)];
                let tmp_real = alpha_real * x_real - alpha_imag * x_imag;
                let tmp_imag = alpha_imag * x_real + alpha_real * x_imag;

                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
                for j in 0..n {
                    let (y_real, y_imag) = y[(jy as usize)];
                    let idx = (i * lda + j) as usize;
                    a[idx].0 += y_real * tmp_real - y_imag * tmp_imag;
                    a[idx].1 += y_imag * tmp_real + y_real * tmp_imag;
                    jy += inc_y;
                }
                ix += inc_x;
            }
        }
        CBLAS_ORDER::ColMajor => {
            let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            for j in 0..n {
                let (y_real, y_imag) = y[(jy as usize)];
                let tmp_real = alpha_real * y_real - alpha_imag * y_imag;
                let tmp_imag = alpha_imag * y_real + alpha_real * y_imag;

                let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
                for i in 0..m {
                    let (x_real, x_imag) = x[(ix as usize)];
                    let idx = (i + lda * j) as usize;
                    a[idx].0 += x_real * tmp_real - x_imag * tmp_imag;
                    a[idx].1 += x_imag * tmp_real + x_real * tmp_imag;
                    ix += inc_x;
                }
                jy += inc_y;
            }
        }
    }

    Ok(())
}