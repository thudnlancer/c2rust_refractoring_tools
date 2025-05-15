use num_complex::Complex64;
use std::convert::TryFrom;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_SIDE {
    Left = 141,
    Right = 142,
}

#[derive(Debug)]
pub enum HemmError {
    InvalidOrder,
    InvalidSide,
    InvalidUplo,
    InvalidM,
    InvalidN,
    InvalidLda,
    InvalidLdb,
    InvalidLdc,
    UnrecognizedOperation,
}

pub fn cblas_zhemm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    m: i32,
    n: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    b: &[Complex64],
    ldb: i32,
    beta: Complex64,
    c: &mut [Complex64],
) -> Result<(), HemmError> {
    // Parameter validation
    let dim_a = if side == CBLAS_SIDE::Left { m } else { n };
    
    if m < 0 {
        return Err(HemmError::InvalidM);
    }
    if n < 0 {
        return Err(HemmError::InvalidN);
    }
    if lda < std::cmp::max(1, dim_a) {
        return Err(HemmError::InvalidLda);
    }
    
    match order {
        CBLAS_ORDER::RowMajor => {
            if ldb < std::cmp::max(1, n) {
                return Err(HemmError::InvalidLdb);
            }
            if ldc < std::cmp::max(1, n) {
                return Err(HemmError::InvalidLdc);
            }
        }
        CBLAS_ORDER::ColMajor => {
            if ldb < std::cmp::max(1, m) {
                return Err(HemmError::InvalidLdb);
            }
            if ldc < std::cmp::max(1, m) {
                return Err(HemmError::InvalidLdc);
            }
        }
    }

    // Early return if alpha is zero and beta is one
    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return Ok(());
    }

    let (n1, n2, uplo, side) = match order {
        CBLAS_ORDER::RowMajor => (m, n, uplo, side),
        CBLAS_ORDER::ColMajor => (
            n,
            m,
            match uplo {
                CBLAS_UPLO::Upper => CBLAS_UPLO::Lower,
                CBLAS_UPLO::Lower => CBLAS_UPLO::Upper,
            },
            match side {
                CBLAS_SIDE::Left => CBLAS_SIDE::Right,
                CBLAS_SIDE::Right => CBLAS_SIDE::Left,
            },
        ),
    };

    // Handle beta scaling
    if beta == Complex64::new(0.0, 0.0) {
        for i in 0..n1 {
            for j in 0..n2 {
                let idx = (i * ldc + j) as usize;
                c[idx] = Complex64::new(0.0, 0.0);
            }
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        for i in 0..n1 {
            for j in 0..n2 {
                let idx = (i * ldc + j) as usize;
                c[idx] = beta * c[idx];
            }
        }
    }

    // Early return if alpha is zero
    if alpha == Complex64::new(0.0, 0.0) {
        return Ok(());
    }

    // Main computation
    match (side, uplo) {
        (CBLAS_SIDE::Left, CBLAS_UPLO::Upper) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b[(i * ldb + j) as usize];
                    let temp1 = alpha * bij;
                    let mut temp2 = Complex64::new(0.0, 0.0);
                    
                    let aii = a[(i * lda + i) as usize].re;
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] += Complex64::new(temp1.re * aii, temp1.im * aii);
                    
                    for k in (i + 1)..n1 {
                        let aik = a[(i * lda + k) as usize];
                        let bkj = b[(k * ldb + j) as usize];
                        let c_idx_k = (k * ldc + j) as usize;
                        
                        c[c_idx_k] += aik * temp1.conj();
                        temp2 += aik * bkj;
                    }
                    
                    c[c_idx] += alpha * temp2;
                }
            }
        }
        (CBLAS_SIDE::Left, CBLAS_UPLO::Lower) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b[(i * ldb + j) as usize];
                    let temp1 = alpha * bij;
                    let mut temp2 = Complex64::new(0.0, 0.0);
                    
                    for k in 0..i {
                        let aik = a[(i * lda + k) as usize];
                        let bkj = b[(k * ldb + j) as usize];
                        let c_idx_k = (k * ldc + j) as usize;
                        
                        c[c_idx_k] += aik * temp1.conj();
                        temp2 += aik * bkj;
                    }
                    
                    let aii = a[(i * lda + i) as usize].re;
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] += Complex64::new(temp1.re * aii, temp1.im * aii);
                    c[c_idx] += alpha * temp2;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Upper) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b[(i * ldb + j) as usize];
                    let temp1 = alpha * bij;
                    let mut temp2 = Complex64::new(0.0, 0.0);
                    
                    let ajj = a[(j * lda + j) as usize].re;
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] += Complex64::new(temp1.re * ajj, temp1.im * ajj);
                    
                    for k in (j + 1)..n2 {
                        let ajk = a[(j * lda + k) as usize];
                        let bik = b[(i * ldb + k) as usize];
                        let c_idx_k = (i * ldc + k) as usize;
                        
                        c[c_idx_k] += temp1 * ajk;
                        temp2 += bik * ajk.conj();
                    }
                    
                    c[c_idx] += alpha * temp2;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Lower) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b[(i * ldb + j) as usize];
                    let temp1 = alpha * bij;
                    let mut temp2 = Complex64::new(0.0, 0.0);
                    
                    for k in 0..j {
                        let ajk = a[(j * lda + k) as usize];
                        let bik = b[(i * ldb + k) as usize];
                        let c_idx_k = (i * ldc + k) as usize;
                        
                        c[c_idx_k] += temp1 * ajk;
                        temp2 += bik * ajk.conj();
                    }
                    
                    let ajj = a[(j * lda + j) as usize].re;
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] += Complex64::new(temp1.re * ajj, temp1.im * ajj);
                    c[c_idx] += alpha * temp2;
                }
            }
        }
        _ => return Err(HemmError::UnrecognizedOperation),
    }

    Ok(())
}