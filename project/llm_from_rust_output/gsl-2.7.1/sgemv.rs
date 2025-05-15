use num_traits::Zero;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug)]
pub enum BlasError {
    InvalidOrder,
    InvalidTranspose,
    InvalidM,
    InvalidN,
    InvalidLda,
    InvalidIncX,
    InvalidIncY,
    UnrecognizedOperation,
}

impl fmt::Display for BlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlasError::InvalidOrder => write!(f, "Invalid order parameter"),
            BlasError::InvalidTranspose => write!(f, "Invalid transpose parameter"),
            BlasError::InvalidM => write!(f, "Invalid M parameter"),
            BlasError::InvalidN => write!(f, "Invalid N parameter"),
            BlasError::InvalidLda => write!(f, "Invalid lda parameter"),
            BlasError::InvalidIncX => write!(f, "Invalid incX parameter"),
            BlasError::InvalidIncY => write!(f, "Invalid incY parameter"),
            BlasError::UnrecognizedOperation => write!(f, "Unrecognized operation"),
        }
    }
}

pub fn cblas_sgemv(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    x: &[f32],
    inc_x: i32,
    beta: f32,
    y: &mut [f32],
    inc_y: i32,
) -> Result<(), BlasError> {
    // Parameter validation
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        return Err(BlasError::InvalidOrder);
    }
    if trans_a != CBLAS_TRANSPOSE::NoTrans
        && trans_a != CBLAS_TRANSPOSE::Trans
        && trans_a != CBLAS_TRANSPOSE::ConjTrans
    {
        return Err(BlasError::InvalidTranspose);
    }
    if m < 0 {
        return Err(BlasError::InvalidM);
    }
    if n < 0 {
        return Err(BlasError::InvalidN);
    }
    if order == CBLAS_ORDER::RowMajor {
        if lda < 1.max(n) {
            return Err(BlasError::InvalidLda);
        }
    } else if lda < 1.max(m) {
        return Err(BlasError::InvalidLda);
    }
    if inc_x == 0 {
        return Err(BlasError::InvalidIncX);
    }
    if inc_y == 0 {
        return Err(BlasError::InvalidIncY);
    }

    // Early returns for edge cases
    if m == 0 || n == 0 {
        return Ok(());
    }
    if alpha.is_zero() && beta == 1.0 {
        return Ok(());
    }

    let trans = if trans_a == CBLAS_TRANSPOSE::ConjTrans {
        CBLAS_TRANSPOSE::Trans
    } else {
        trans_a
    };

    let (len_x, len_y) = if trans == CBLAS_TRANSPOSE::NoTrans {
        (n, m)
    } else {
        (m, n)
    };

    // Handle beta scaling
    if beta.is_zero() {
        let mut iy = if inc_y > 0 { 0 } else { (len_y - 1) * -inc_y };
        for _ in 0..len_y {
            y[iy as usize] = 0.0;
            iy += inc_y;
        }
    } else if beta != 1.0 {
        let mut iy = if inc_y > 0 { 0 } else { (len_y - 1) * -inc_y };
        for _ in 0..len_y {
            y[iy as usize] *= beta;
            iy += inc_y;
        }
    }

    if alpha.is_zero() {
        return Ok(());
    }

    match (order, trans) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans) => {
            let mut iy = if inc_y > 0 { 0 } else { (len_y - 1) * -inc_y };
            for i in 0..len_y {
                let mut temp = 0.0;
                let mut ix = if inc_x > 0 { 0 } else { (len_x - 1) * -inc_x };
                for j in 0..len_x {
                    temp += x[ix as usize] * a[(lda * i + j) as usize];
                    ix += inc_x;
                }
                y[iy as usize] += alpha * temp;
                iy += inc_y;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => {
            let mut ix = if inc_x > 0 { 0 } else { (len_x - 1) * -inc_x };
            for j in 0..len_x {
                let temp = alpha * x[ix as usize];
                if !temp.is_zero() {
                    let mut iy = if inc_y > 0 { 0 } else { (len_y - 1) * -inc_y };
                    for i in 0..len_y {
                        y[iy as usize] += temp * a[(lda * j + i) as usize];
                        iy += inc_y;
                    }
                }
                ix += inc_x;
            }
        }
        _ => return Err(BlasError::UnrecognizedOperation),
    }

    Ok(())
}