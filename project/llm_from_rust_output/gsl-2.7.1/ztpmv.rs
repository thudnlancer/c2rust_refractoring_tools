use num_complex::Complex64;

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

pub fn cblas_ztpmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    ap: &[Complex64],
    x: &mut [Complex64],
    inc_x: i32,
) {
    // Parameter validation
    if n < 0 {
        panic!("Invalid n: {}", n);
    }
    if inc_x == 0 {
        panic!("Invalid inc_x: {}", inc_x);
    }

    let conj = if trans == CBLAS_TRANSPOSE::ConjTrans {
        -1.0
    } else {
        1.0
    };

    let trans_simple = if trans != CBLAS_TRANSPOSE::ConjTrans {
        trans
    } else {
        CBLAS_TRANSPOSE::Trans
    };

    let nonunit = diag == CBLAS_DIAG::NonUnit;

    match (order, trans_simple, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            for i in 0..n {
                let a_offset = (i * (2 * n - i + 1)) / 2;
                let aii = ap[a_offset as usize];
                let aii = if trans == CBLAS_TRANSPOSE::ConjTrans {
                    Complex64::new(aii.re, conj * aii.im)
                } else {
                    aii
                };

                let mut temp = if nonunit {
                    x[ix as usize] * aii
                } else {
                    x[ix as usize]
                };

                let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + (i + 1) * inc_x;
                for j in (i + 1)..n {
                    let aij = ap[(a_offset + j - i) as usize];
                    let aij = if trans == CBLAS_TRANSPOSE::ConjTrans {
                        Complex64::new(aij.re, conj * aij.im)
                    } else {
                        aij
                    };
                    temp += x[jx as usize] * aij;
                    jx += inc_x;
                }
                x[ix as usize] = temp;
                ix += inc_x;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) => {
            let mut ix = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + inc_x * (n - 1);
            for i in (0..n).rev() {
                let a_offset = (i * (i + 1)) / 2;
                let aii = ap[(a_offset + i) as usize];
                let aii = if trans == CBLAS_TRANSPOSE::ConjTrans {
                    Complex64::new(aii.re, conj * aii.im)
                } else {
                    aii
                };

                let mut temp = if nonunit {
                    x[ix as usize] * aii
                } else {
                    x[ix as usize]
                };

                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
                for j in 0..i {
                    let aij = ap[(a_offset + j) as usize];
                    let aij = if trans == CBLAS_TRANSPOSE::ConjTrans {
                        Complex64::new(aij.re, conj * aij.im)
                    } else {
                        aij
                    };
                    temp += x[jx as usize] * aij;
                    jx += inc_x;
                }
                x[ix as usize] = temp;
                ix -= inc_x;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) => {
            let mut ix = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + inc_x * (n - 1);
            for i in (0..n).rev() {
                let a_offset = (i * (2 * n - i + 1)) / 2;
                let aii = ap[a_offset as usize];
                let aii = if trans == CBLAS_TRANSPOSE::ConjTrans {
                    Complex64::new(aii.re, conj * aii.im)
                } else {
                    aii
                };

                let mut temp = if nonunit {
                    x[ix as usize] * aii
                } else {
                    x[ix as usize]
                };

                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
                for j in 0..i {
                    let aji_offset = (j * (2 * n - j + 1)) / 2;
                    let aji = ap[(aji_offset + i - j) as usize];
                    let aji = if trans == CBLAS_TRANSPOSE::ConjTrans {
                        Complex64::new(aji.re, conj * aji.im)
                    } else {
                        aji
                    };
                    temp += x[jx as usize] * aji;
                    jx += inc_x;
                }
                x[ix as usize] = temp;
                ix -= inc_x;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            for i in 0..n {
                let a_offset = (i * (i + 1)) / 2;
                let aii = ap[(a_offset + i) as usize];
                let aii = if trans == CBLAS_TRANSPOSE::ConjTrans {
                    Complex64::new(aii.re, conj * aii.im)
                } else {
                    aii
                };

                let mut temp = if nonunit {
                    x[ix as usize] * aii
                } else {
                    x[ix as usize]
                };

                let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + (i + 1) * inc_x;
                for j in (i + 1)..n {
                    let aji_offset = (j * (j + 1)) / 2;
                    let aji = ap[(aji_offset + i) as usize];
                    let aji = if trans == CBLAS_TRANSPOSE::ConjTrans {
                        Complex64::new(aji.re, conj * aji.im)
                    } else {
                        aji
                    };
                    temp += x[jx as usize] * aji;
                    jx += inc_x;
                }
                x[ix as usize] = temp;
                ix += inc_x;
            }
        }
        _ => panic!("Unrecognized operation"),
    }
}