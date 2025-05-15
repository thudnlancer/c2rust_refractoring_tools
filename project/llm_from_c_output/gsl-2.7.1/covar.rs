use ndarray::{Array2, Array1, Axis};
use ndarray_linalg::{QR, Solve, Inverse};
use permutation::Permutation;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultifitError {
    #[error("Jacobian must be rectangular M x N with M >= N")]
    BadJacobianSize,
    #[error("Covariance matrix must be square and match second dimension of jacobian")]
    BadCovarianceSize,
    #[error("Numerical error in QR decomposition")]
    QRError,
}

pub fn multifit_covar(
    j: &Array2<f64>,
    epsrel: f64,
    covar: &mut Array2<f64>,
) -> Result<(), MultifitError> {
    let (m, n) = j.dim();
    
    if m < n {
        return Err(MultifitError::BadJacobianSize);
    }
    
    if covar.dim().0 != covar.dim().1 || covar.dim().0 != n {
        return Err(MultifitError::BadCovarianceSize);
    }

    let mut r = j.clone();
    let qr = r.qr().map_err(|_| MultifitError::QRError)?;
    let (q, r) = qr.into_qr();

    multifit_covar_qrpt(&r, &Permutation::identity(n), epsrel, covar)
}

pub fn multifit_covar_qrpt(
    r: &Array2<f64>,
    perm: &Permutation<usize>,
    epsrel: f64,
    covar: &mut Array2<f64>,
) -> Result<(), MultifitError> {
    let n = r.shape()[1];
    let mut r_inv = r.clone();
    let tolr = epsrel * r[[0, 0]].abs();
    let mut kmax = 0;

    // Form the inverse of R in the full upper triangle of R
    for k in 0..n {
        let rkk = r_inv[[k, k]];
        if rkk.abs() <= tolr {
            break;
        }

        r_inv[[k, k]] = 1.0 / rkk;

        for j in 0..k {
            let t = r_inv[[j, k]] / rkk;
            r_inv[[j, k]] = 0.0;

            for i in 0..=j {
                let rik = r_inv[[i, k]];
                let rij = r_inv[[i, j]];
                r_inv[[i, k]] = rik - t * rij;
            }
        }
        kmax = k;
    }

    // Form the full upper triangle of the inverse of R^T R
    for k in 0..=kmax {
        for j in 0..k {
            let rjk = r_inv[[j, k]];

            for i in 0..=j {
                let rij = r_inv[[i, j]];
                let rik = r_inv[[i, k]];
                r_inv[[i, j]] = rij + rjk * rik;
            }
        }

        let t = r_inv[[k, k]];
        for i in 0..=k {
            let rik = r_inv[[i, k]];
            r_inv[[i, k]] = t * rik;
        }
    }

    // Form the covariance matrix with permutation
    for j in 0..n {
        let pj = perm.apply_inv(j);

        for i in 0..=j {
            let pi = perm.apply_inv(i);
            let rij = if j > kmax { 0.0 } else { r_inv[[i, j]] };

            if pi > pj {
                covar[[pi, pj]] = rij;
            } else if pi < pj {
                covar[[pj, pi]] = rij;
            }
        }

        let rjj = r_inv[[j, j]];
        let pj = perm.apply_inv(j);
        covar[[pj, pj]] = rjj;
    }

    // Symmetrize the covariance matrix
    for j in 0..n {
        for i in 0..j {
            let rji = covar[[j, i]];
            covar[[i, j]] = rji;
        }
    }

    Ok(())
}