use ndarray::{Array2, ArrayView2, ArrayViewMut2};
use num_complex::Complex64;
use thiserror::Error;

const CROSSOVER_INVTRI: usize = 64;
const GSL_COMPLEX_NEGONE: Complex64 = Complex64::new(-1.0, 0.0);
const GSL_COMPLEX_ONE: Complex64 = Complex64::new(1.0, 0.0);

#[derive(Error, Debug)]
pub enum LinAlgError {
    #[error("matrix must be square")]
    NotSquare,
    #[error("matrix is singular")]
    Singular,
}

#[derive(Copy, Clone, Debug)]
pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

#[derive(Copy, Clone, Debug)]
pub enum CBLAS_DIAG {
    NonUnit,
    Unit,
}

pub fn complex_tri_invert(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    t: &mut Array2<Complex64>,
) -> Result<(), LinAlgError> {
    let n = t.nrows();

    if n != t.ncols() {
        return Err(LinAlgError::NotSquare);
    }

    if triangular_singular(t)? {
        return Err(LinAlgError::Singular);
    }

    complex_tri_invert_l3(uplo, diag, t)
}

fn complex_tri_invert_l2(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    t: &mut Array2<Complex64>,
) -> Result<(), LinAlgError> {
    let n = t.nrows();

    if n != t.ncols() {
        return Err(LinAlgError::NotSquare);
    }

    match uplo {
        CBLAS_UPLO::Upper => {
            for i in 0..n {
                let mut aii = if diag == CBLAS_DIAG::NonUnit {
                    let tii = t[(i, i)];
                    let inv_tii = tii.inv();
                    t[(i, i)] = inv_tii;
                    -inv_tii
                } else {
                    GSL_COMPLEX_NEGONE
                };

                if i > 0 {
                    let m = t.slice(s![0..i, 0..i]);
                    let mut v = t.slice_mut(s![0..i, i]);
                    ztrmv(CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans, diag, &m, &mut v);
                    zscal(aii, &mut v);
                }
            }
        }
        CBLAS_UPLO::Lower => {
            for i in 0..n {
                let j = n - i - 1;
                let mut ajj = if diag == CBLAS_DIAG::NonUnit {
                    let tjj = t[(j, j)];
                    let inv_tjj = tjj.inv();
                    t[(j, j)] = inv_tjj;
                    -inv_tjj
                } else {
                    GSL_COMPLEX_NEGONE
                };

                if j < n - 1 {
                    let m = t.slice(s![j+1..n, j+1..n]);
                    let mut v = t.slice_mut(s![j+1..n, j]);
                    ztrmv(CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans, diag, &m, &mut v);
                    zscal(ajj, &mut v);
                }
            }
        }
    }

    Ok(())
}

fn complex_tri_invert_l3(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    t: &mut Array2<Complex64>,
) -> Result<(), LinAlgError> {
    let n = t.nrows();

    if n != t.ncols() {
        return Err(LinAlgError::NotSquare);
    } else if n <= CROSSOVER_INVTRI {
        return complex_tri_invert_l2(uplo, diag, t);
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let (mut t11, rest) = t.view_mut().split_at((n1, n1));
    let (t12, t21_t22) = rest.split_at((n1, n2));
    let (t21, mut t22) = t21_t22.split_at((n2, n1));

    complex_tri_invert_l3(uplo, diag, &mut t11.to_owned())?;

    match uplo {
        CBLAS_UPLO::Lower => {
            ztrmm(
                CBLAS_SIDE::Right,
                uplo,
                CBLAS_TRANSPOSE::NoTrans,
                diag,
                GSL_COMPLEX_NEGONE,
                &t11.to_owned(),
                &mut t21.to_owned(),
            );
            ztrsm(
                CBLAS_SIDE::Left,
                uplo,
                CBLAS_TRANSPOSE::NoTrans,
                diag,
                GSL_COMPLEX_ONE,
                &t22.to_owned(),
                &mut t21.to_owned(),
            );
        }
        CBLAS_UPLO::Upper => {
            ztrmm(
                CBLAS_SIDE::Left,
                uplo,
                CBLAS_TRANSPOSE::NoTrans,
                diag,
                GSL_COMPLEX_NEGONE,
                &t11.to_owned(),
                &mut t12.to_owned(),
            );
            ztrsm(
                CBLAS_SIDE::Right,
                uplo,
                CBLAS_TRANSPOSE::NoTrans,
                diag,
                GSL_COMPLEX_ONE,
                &t22.to_owned(),
                &mut t12.to_owned(),
            );
        }
    }

    complex_tri_invert_l3(uplo, diag, &mut t22.to_owned())?;

    Ok(())
}

fn triangular_singular(t: &Array2<Complex64>) -> Result<bool, LinAlgError> {
    for i in 0..t.nrows() {
        let z = t[(i, i)];
        if z.re == 0.0 && z.im == 0.0 {
            return Ok(true);
        }
    }
    Ok(false)
}

// BLAS-like operations implemented in Rust
#[derive(Copy, Clone, Debug)]
enum CBLAS_TRANSPOSE {
    NoTrans,
    Trans,
    ConjTrans,
}

#[derive(Copy, Clone, Debug)]
enum CBLAS_SIDE {
    Left,
    Right,
}

fn ztrmv(
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    a: &ArrayView2<Complex64>,
    x: &mut ArrayViewMut1<Complex64>,
) {
    // Implementation of ztrmv
    unimplemented!()
}

fn zscal(a: Complex64, x: &mut ArrayViewMut1<Complex64>) {
    *x *= a;
}

fn ztrmm(
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    transa: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    alpha: Complex64,
    a: &Array2<Complex64>,
    b: &mut Array2<Complex64>,
) {
    // Implementation of ztrmm
    unimplemented!()
}

fn ztrsm(
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    transa: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    alpha: Complex64,
    a: &Array2<Complex64>,
    b: &mut Array2<Complex64>,
) {
    // Implementation of ztrsm
    unimplemented!()
}