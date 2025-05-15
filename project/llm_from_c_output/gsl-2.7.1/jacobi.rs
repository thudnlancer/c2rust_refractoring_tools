use std::f64;
use ndarray::{Array2, Array1, ArrayView2, ArrayView1, Axis};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JacobiError {
    #[error("eigenproblem requires square matrix")]
    NotSquare,
    #[error("eigenvector matrix must match input matrix")]
    BadEigenvectorLen,
    #[error("eigenvalue vector must match input matrix")]
    BadEigenvalueLen,
    #[error("maximum iterations reached")]
    MaxIterations,
}

fn symschur2(a: &Array2<f64>, p: usize, q: usize) -> (f64, f64, f64) {
    let apq = a[[p, q]];

    if apq != 0.0 {
        let app = a[[p, p]];
        let aqq = a[[q, q]];
        let tau = (aqq - app) / (2.0 * apq);
        let t = if tau >= 0.0 {
            1.0 / (tau + (1.0 + tau * tau).sqrt())
        } else {
            -1.0 / (-tau + (1.0 + tau * tau).sqrt())
        };

        let c1 = 1.0 / (1.0 + t * t).sqrt();
        (c1, t * c1, apq.abs())
    } else {
        (1.0, 0.0, 0.0)
    }
}

fn apply_jacobi_l(a: &mut Array2<f64>, p: usize, q: usize, c: f64, s: f64) {
    for j in 0..a.ncols() {
        let apj = a[[p, j]];
        let aqj = a[[q, j]];
        a[[p, j]] = apj * c - aqj * s;
        a[[q, j]] = apj * s + aqj * c;
    }
}

fn apply_jacobi_r(a: &mut Array2<f64>, p: usize, q: usize, c: f64, s: f64) {
    for i in 0..a.nrows() {
        let aip = a[[i, p]];
        let aiq = a[[i, q]];
        a[[i, p]] = aip * c - aiq * s;
        a[[i, q]] = aip * s + aiq * c;
    }
}

fn norm(a: &Array2<f64>) -> f64 {
    let mut scale = 0.0;
    let mut ssq = 1.0;

    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            if i == j {
                continue;
            }

            let aij = a[[i, j]];
            if aij != 0.0 {
                let ax = aij.abs();
                if scale < ax {
                    ssq = 1.0 + ssq * (scale / ax).powi(2);
                    scale = ax;
                } else {
                    ssq += (ax / scale).powi(2);
                }
            }
        }
    }

    scale * ssq.sqrt()
}

pub fn gsl_eigen_jacobi(
    a: &mut Array2<f64>,
    eval: &mut Array1<f64>,
    evec: &mut Array2<f64>,
    max_rot: usize,
) -> Result<usize, JacobiError> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(JacobiError::NotSquare);
    } else if n != evec.nrows() || n != evec.ncols() {
        return Err(JacobiError::BadEigenvectorLen);
    } else if n != eval.len() {
        return Err(JacobiError::BadEigenvalueLen);
    }

    eval.fill(0.0);
    evec.fill(0.0);
    for i in 0..n {
        evec[[i, i]] = 1.0;
    }

    let mut nrot = 0;
    for _ in 0..max_rot {
        let nrm = norm(a);
        if nrm == 0.0 {
            break;
        }

        for p in 0..n {
            for q in p + 1..n {
                let (c, s, _) = symschur2(a, p, q);
                apply_jacobi_l(a, p, q, c, s);
                apply_jacobi_r(a, p, q, c, s);
                apply_jacobi_r(evec, p, q, c, s);
            }
        }
        nrot += 1;
    }

    for p in 0..n {
        eval[p] = a[[p, p]];
    }

    if nrot == max_rot {
        Err(JacobiError::MaxIterations)
    } else {
        Ok(nrot)
    }
}

pub fn gsl_eigen_invert_jacobi(
    a: &Array2<f64>,
    ainv: &mut Array2<f64>,
    max_rot: usize,
) -> Result<(), JacobiError> {
    let n = a.nrows();
    if n != a.ncols() || ainv.nrows() != ainv.ncols() {
        return Err(JacobiError::NotSquare);
    } else if n != ainv.nrows() {
        return Err(JacobiError::BadEigenvectorLen);
    }

    let mut tmp = a.clone();
    let mut eval = Array1::zeros(n);
    let mut evec = Array2::zeros((n, n));

    let nrot = gsl_eigen_jacobi(&mut tmp, &mut eval, &mut evec, max_rot)?;

    for i in 0..n {
        for j in 0..n {
            let mut ainv_ij = 0.0;
            for k in 0..n {
                let f = 1.0 / eval[k];
                let vik = evec[[i, k]];
                let vjk = evec[[j, k]];
                ainv_ij += vik * vjk * f;
            }
            ainv[[i, j]] = ainv_ij;
        }
    }

    Ok(())
}