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

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

pub fn cblas_zgerc(
    order: CBLAS_ORDER,
    m: i32,
    n: i32,
    alpha: Complex,
    x: &[Complex],
    inc_x: i32,
    y: &[Complex],
    inc_y: i32,
    a: &mut [Complex],
    lda: i32,
) -> Result<(), BlasError> {
    // Parameter validation
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        return Err(BlasError::InvalidOrder);
    }
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

    // Check array lengths
    let x_len = if m == 0 {
        0
    } else {
        1 + (m - 1) * inc_x.abs()
    } as usize;
    let y_len = if n == 0 {
        0
    } else {
        1 + (n - 1) * inc_y.abs()
    } as usize;
    let a_len = match order {
        CBLAS_ORDER::RowMajor => (lda * m) as usize,
        CBLAS_ORDER::ColMajor => (lda * n) as usize,
    };

    if x.len() < x_len || y.len() < y_len || a.len() < a_len {
        return Err(BlasError::UnrecognizedOperation);
    }

    match order {
        CBLAS_ORDER::RowMajor => {
            let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
            for i in 0..m {
                let x_elem = x[(ix * 2) as usize];
                let tmp = Complex::new(
                    alpha.real * x_elem.real - alpha.imag * x_elem.imag,
                    alpha.imag * x_elem.real + alpha.real * x_elem.imag,
                );

                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
                for j in 0..n {
                    let y_elem = y[(jy * 2) as usize];
                    let y_conj = Complex::new(y_elem.real, -y_elem.imag);

                    let a_idx = (i * lda + j) as usize;
                    a[a_idx].real += y_conj.real * tmp.real - y_conj.imag * tmp.imag;
                    a[a_idx].imag += y_conj.imag * tmp.real + y_conj.real * tmp.imag;

                    jy += inc_y;
                }
                ix += inc_x;
            }
        }
        CBLAS_ORDER::ColMajor => {
            let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            for j in 0..n {
                let y_elem = y[(jy * 2) as usize];
                let y_conj = Complex::new(y_elem.real, -y_elem.imag);
                let tmp = Complex::new(
                    alpha.real * y_conj.real - alpha.imag * y_conj.imag,
                    alpha.imag * y_conj.real + alpha.real * y_conj.imag,
                );

                let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
                for i in 0..m {
                    let x_elem = x[(ix * 2) as usize];

                    let a_idx = (i + j * lda) as usize;
                    a[a_idx].real += x_elem.real * tmp.real - x_elem.imag * tmp.imag;
                    a[a_idx].imag += x_elem.imag * tmp.real + x_elem.real * tmp.imag;

                    ix += inc_x;
                }
                jy += inc_y;
            }
        }
    }

    Ok(())
}