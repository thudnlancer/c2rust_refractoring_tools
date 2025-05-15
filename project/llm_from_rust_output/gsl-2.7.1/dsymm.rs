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
pub enum BlasError {
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

fn check_parameters(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    m: i32,
    n: i32,
    lda: i32,
    ldb: i32,
    ldc: i32,
) -> Result<(), BlasError> {
    let dim_a = if side == CBLAS_SIDE::Left { m } else { n };

    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        return Err(BlasError::InvalidOrder);
    }
    if side != CBLAS_SIDE::Left && side != CBLAS_SIDE::Right {
        return Err(BlasError::InvalidSide);
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        return Err(BlasError::InvalidUplo);
    }
    if m < 0 {
        return Err(BlasError::InvalidM);
    }
    if n < 0 {
        return Err(BlasError::InvalidN);
    }
    if lda < 1.max(dim_a) {
        return Err(BlasError::InvalidLda);
    }

    match order {
        CBLAS_ORDER::RowMajor => {
            if ldb < 1.max(n) {
                return Err(BlasError::InvalidLdb);
            }
            if ldc < 1.max(n) {
                return Err(BlasError::InvalidLdc);
            }
        }
        CBLAS_ORDER::ColMajor => {
            if ldb < 1.max(m) {
                return Err(BlasError::InvalidLdb);
            }
            if ldc < 1.max(m) {
                return Err(BlasError::InvalidLdc);
            }
        }
    }

    Ok(())
}

pub fn cblas_dsymm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    m: i32,
    n: i32,
    alpha: f64,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    beta: f64,
    c: &mut [f64],
    ldc: i32,
) -> Result<(), BlasError> {
    check_parameters(order, side, uplo, m, n, lda, ldb, ldc)?;

    if alpha == 0.0 && beta == 1.0 {
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

    let n1 = usize::try_from(n1).unwrap();
    let n2 = usize::try_from(n2).unwrap();
    let lda = usize::try_from(lda).unwrap();
    let ldb = usize::try_from(ldb).unwrap();
    let ldc = usize::try_from(ldc).unwrap();

    if beta == 0.0 {
        for i in 0..n1 {
            for j in 0..n2 {
                c[i * ldc + j] = 0.0;
            }
        }
    } else if beta != 1.0 {
        for i in 0..n1 {
            for j in 0..n2 {
                c[i * ldc + j] *= beta;
            }
        }
    }

    if alpha == 0.0 {
        return Ok(());
    }

    match (side, uplo) {
        (CBLAS_SIDE::Left, CBLAS_UPLO::Upper) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let temp1 = alpha * b[i * ldb + j];
                    let mut temp2 = 0.0;
                    c[i * ldc + j] += temp1 * a[i * lda + i];
                    for k in (i + 1)..n1 {
                        let aik = a[i * lda + k];
                        c[k * ldc + j] += aik * temp1;
                        temp2 += aik * b[k * ldb + j];
                    }
                    c[i * ldc + j] += alpha * temp2;
                }
            }
        }
        (CBLAS_SIDE::Left, CBLAS_UPLO::Lower) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let temp1 = alpha * b[i * ldb + j];
                    let mut temp2 = 0.0;
                    for k in 0..i {
                        let aik = a[i * lda + k];
                        c[k * ldc + j] += aik * temp1;
                        temp2 += aik * b[k * ldb + j];
                    }
                    c[i * ldc + j] += temp1 * a[i * lda + i] + alpha * temp2;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Upper) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let temp1 = alpha * b[i * ldb + j];
                    let mut temp2 = 0.0;
                    c[i * ldc + j] += temp1 * a[j * lda + j];
                    for k in (j + 1)..n2 {
                        let ajk = a[j * lda + k];
                        c[i * ldc + k] += temp1 * ajk;
                        temp2 += b[i * ldb + k] * ajk;
                    }
                    c[i * ldc + j] += alpha * temp2;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Lower) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let temp1 = alpha * b[i * ldb + j];
                    let mut temp2 = 0.0;
                    for k in 0..j {
                        let ajk = a[j * lda + k];
                        c[i * ldc + k] += temp1 * ajk;
                        temp2 += b[i * ldb + k] * ajk;
                    }
                    c[i * ldc + j] += temp1 * a[j * lda + j] + alpha * temp2;
                }
            }
        }
        _ => return Err(BlasError::UnrecognizedOperation),
    }

    Ok(())
}