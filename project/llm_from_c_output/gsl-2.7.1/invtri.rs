use ndarray::{Array2, Axis};
use ndarray_linalg::{error::LinalgError, Solve, Scalar};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriangularError {
    #[error("matrix must be square")]
    NotSquare,
    #[error("matrix is singular")]
    Singular,
}

const CROSSOVER_INVTRI: usize = 64;

pub fn tri_invert(uplo: Uplo, diag: Diag, t: &mut Array2<f64>) -> Result<(), TriangularError> {
    let n = t.nrows();
    if n != t.ncols() {
        return Err(TriangularError::NotSquare);
    }

    if triangular_singular(t)? {
        return Err(TriangularError::Singular);
    }

    triangular_inverse_l3(uplo, diag, t)
}

#[derive(Copy, Clone, Debug)]
pub enum Uplo {
    Upper,
    Lower,
}

#[derive(Copy, Clone, Debug)]
pub enum Diag {
    NonUnit,
    Unit,
}

fn triangular_inverse_l2(uplo: Uplo, diag: Diag, t: &mut Array2<f64>) -> Result<(), TriangularError> {
    let n = t.nrows();
    if n != t.ncols() {
        return Err(TriangularError::NotSquare);
    }

    match uplo {
        Uplo::Upper => {
            for i in 0..n {
                let aii = if diag == Diag::NonUnit {
                    let tii = t[(i, i)];
                    if tii == 0.0 {
                        return Err(TriangularError::Singular);
                    }
                    t[(i, i)] = 1.0 / tii;
                    -t[(i, i)]
                } else {
                    -1.0
                };

                if i > 0 {
                    let mut v = t.slice_mut(s![..i, i]);
                    let m = t.slice(s![..i, ..i]);
                    
                    // Compute v = m * v
                    let mv = m.dot(&v);
                    v.assign(&mv);
                    
                    // Scale v by aii
                    v *= aii;
                }
            }
        }
        Uplo::Lower => {
            for i in 0..n {
                let j = n - i - 1;
                let ajj = if diag == Diag::NonUnit {
                    let tjj = t[(j, j)];
                    if tjj == 0.0 {
                        return Err(TriangularError::Singular);
                    }
                    t[(j, j)] = 1.0 / tjj;
                    -t[(j, j)]
                } else {
                    -1.0
                };

                if j < n - 1 {
                    let mut v = t.slice_mut(s![j+1.., j]);
                    let m = t.slice(s![j+1.., j+1..]);
                    
                    // Compute v = m * v
                    let mv = m.dot(&v);
                    v.assign(&mv);
                    
                    // Scale v by ajj
                    v *= ajj;
                }
            }
        }
    }

    Ok(())
}

fn triangular_inverse_l3(uplo: Uplo, diag: Diag, t: &mut Array2<f64>) -> Result<(), TriangularError> {
    let n = t.nrows();
    if n != t.ncols() {
        return Err(TriangularError::NotSquare);
    } else if n <= CROSSOVER_INVTRI {
        return triangular_inverse_l2(uplo, diag, t);
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let (mut t11, mut rest) = t.view_mut().split_at(Axis(0), n1);
    let (mut t12, mut t22) = rest.split_at(Axis(1), n1);
    let mut t21 = t.slice_mut(s![n1.., ..n1]);

    // Recursion on t11
    triangular_inverse_l3(uplo, diag, &mut t11)?;

    match uplo {
        Uplo::Lower => {
            // t21 = -t21 * t11
            t21 = -t21.dot(&t11);
            
            // t21 = t22^{-1} * t21
            let t22_view = t22.view();
            t21 = t22_view.solve_into(t21).map_err(|_| TriangularError::Singular)?;
        }
        Uplo::Upper => {
            // t12 = -t11 * t12
            t12 = -t11.dot(&t12);
            
            // t12 = t12 * t22^{-1}
            let t22_view = t22.view();
            t12 = t12.dot(&t22_view.inv().map_err(|_| TriangularError::Singular)?;
        }
    }

    // Recursion on t22
    triangular_inverse_l3(uplo, diag, &mut t22)?;

    Ok(())
}

fn triangular_singular(t: &Array2<f64>) -> Result<bool, TriangularError> {
    for i in 0..t.nrows() {
        if t[(i, i)] == 0.0 {
            return Ok(true);
        }
    }
    Ok(false)
}

// Deprecated functions kept for compatibility
pub fn tri_upper_invert(t: &mut Array2<f64>) -> Result<(), TriangularError> {
    if triangular_singular(t)? {
        return Err(TriangularError::Singular);
    }
    triangular_inverse_l3(Uplo::Upper, Diag::NonUnit, t)
}

pub fn tri_lower_invert(t: &mut Array2<f64>) -> Result<(), TriangularError> {
    if triangular_singular(t)? {
        return Err(TriangularError::Singular);
    }
    triangular_inverse_l3(Uplo::Lower, Diag::NonUnit, t)
}

pub fn tri_upper_unit_invert(t: &mut Array2<f64>) -> Result<(), TriangularError> {
    if triangular_singular(t)? {
        return Err(TriangularError::Singular);
    }
    triangular_inverse_l3(Uplo::Upper, Diag::Unit, t)
}

pub fn tri_lower_unit_invert(t: &mut Array2<f64>) -> Result<(), TriangularError> {
    if triangular_singular(t)? {
        return Err(TriangularError::Singular);
    }
    triangular_inverse_l3(Uplo::Lower, Diag::Unit, t)
}