use num_complex::Complex32;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

pub fn cblas_ctpmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    ap: &[Complex32],
    x: &mut [Complex32],
    incx: i32,
) {
    if n < 0 {
        panic!("Invalid dimension: N must be non-negative");
    }
    if incx == 0 {
        panic!("Invalid increment: incX must not be zero");
    }

    let conj = if trans == CBLAS_TRANSPOSE::ConjTrans {
        -1.0
    } else {
        1.0
    };

    let trans_simplified = if trans != CBLAS_TRANSPOSE::ConjTrans {
        trans
    } else {
        CBLAS_TRANSPOSE::Trans
    };

    let nonunit = diag == CBLAS_DIAG::NonUnit;

    match (order, trans_simplified, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
            for i in 0..n {
                let pos = (i * (2 * n - i + 1)) / 2;
                let aii = ap[pos as usize];
                let aii_conj = Complex32::new(aii.re, conj * aii.im);

                let mut temp = if nonunit {
                    aii_conj * x[(ix * 2) as usize]
                } else {
                    x[(ix * 2) as usize]
                };

                let mut jx = (if incx > 0 { 0 } else { (n - 1) * -incx }) + (i + 1) * incx;
                for j in (i + 1)..n {
                    let pos = (i * (2 * n - i + 1)) / 2 + (j - i);
                    let aij = ap[pos as usize];
                    let aij_conj = Complex32::new(aij.re, conj * aij.im);
                    temp += aij_conj * x[(jx * 2) as usize];
                    jx += incx;
                }

                x[(ix * 2) as usize] = temp;
                ix += incx;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) => {
            let mut ix = (if incx > 0 { 0 } else { (n - 1) * -incx }) + incx * (n - 1);
            for i in (0..n).rev() {
                let pos = (i * (i + 1)) / 2 + i;
                let aii = ap[pos as usize];
                let aii_conj = Complex32::new(aii.re, conj * aii.im);

                let mut temp = if nonunit {
                    aii_conj * x[(ix * 2) as usize]
                } else {
                    x[(ix * 2) as usize]
                };

                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx };
                for j in 0..i {
                    let pos = (i * (i + 1)) / 2 + j;
                    let aij = ap[pos as usize];
                    let aij_conj = Complex32::new(aij.re, conj * aij.im);
                    temp += aij_conj * x[(jx * 2) as usize];
                    jx += incx;
                }

                x[(ix * 2) as usize] = temp;
                ix -= incx;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) => {
            let mut ix = (if incx > 0 { 0 } else { (n - 1) * -incx }) + incx * (n - 1);
            for i in (0..n).rev() {
                let pos = (i * (2 * n - i + 1)) / 2;
                let aii = ap[pos as usize];
                let aii_conj = Complex32::new(aii.re, conj * aii.im);

                let mut temp = if nonunit {
                    aii_conj * x[(ix * 2) as usize]
                } else {
                    x[(ix * 2) as usize]
                };

                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx };
                for j in 0..i {
                    let pos = (j * (2 * n - j + 1)) / 2 + (i - j);
                    let aji = ap[pos as usize];
                    let aji_conj = Complex32::new(aji.re, conj * aji.im);
                    temp += aji_conj * x[(jx * 2) as usize];
                    jx += incx;
                }

                x[(ix * 2) as usize] = temp;
                ix -= incx;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
            for i in 0..n {
                let pos = (i * (i + 1)) / 2 + i;
                let aii = ap[pos as usize];
                let aii_conj = Complex32::new(aii.re, conj * aii.im);

                let mut temp = if nonunit {
                    aii_conj * x[(ix * 2) as usize]
                } else {
                    x[(ix * 2) as usize]
                };

                let mut jx = (if incx > 0 { 0 } else { (n - 1) * -incx }) + (i + 1) * incx;
                for j in (i + 1)..n {
                    let pos = (j * (j + 1)) / 2 + i;
                    let aji = ap[pos as usize];
                    let aji_conj = Complex32::new(aji.re, conj * aji.im);
                    temp += aji_conj * x[(jx * 2) as usize];
                    jx += incx;
                }

                x[(ix * 2) as usize] = temp;
                ix += incx;
            }
        }
        _ => panic!("Unrecognized operation combination"),
    }
}