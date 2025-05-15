use std::error::Error;
use std::fmt;
use std::ptr;

use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayViewMut1, ArrayViewMut2};
use ndarray_linalg::{cholesky::*, eig::*, norm::Norm, scal::*, types::*};
use num_complex::Complex64;

#[derive(Debug)]
pub enum GenHermError {
    InvalidMatrix,
    DimensionMismatch,
    NotPositiveDefinite,
    AllocationFailed,
}

impl fmt::Display for GenHermError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenHermError::InvalidMatrix => write!(f, "matrix must be square"),
            GenHermError::DimensionMismatch => write!(f, "matrix dimensions do not match"),
            GenHermError::NotPositiveDefinite => write!(f, "matrix is not positive definite"),
            GenHermError::AllocationFailed => write!(f, "failed to allocate memory"),
        }
    }
}

impl Error for GenHermError {}

pub struct GenHermWorkspace {
    size: usize,
    herm_workspace: HermWorkspace,
}

impl GenHermWorkspace {
    pub fn new(n: usize) -> Result<Self, GenHermError> {
        if n == 0 {
            return Err(GenHermError::InvalidMatrix);
        }

        let herm_workspace = HermWorkspace::new(n)
            .map_err(|_| GenHermError::AllocationFailed)?;

        Ok(Self {
            size: n,
            herm_workspace,
        })
    }
}

pub fn genhermv(
    a: &mut Array2<Complex64>,
    b: &mut Array2<Complex64>,
    eval: &mut Array1<f64>,
    evec: &mut Array2<Complex64>,
    workspace: &mut GenHermWorkspace,
) -> Result<(), GenHermError> {
    let n = a.shape()[0];

    if a.shape()[0] != a.shape()[1] {
        return Err(GenHermError::InvalidMatrix);
    } else if b.shape()[0] != n || b.shape()[1] != n {
        return Err(GenHermError::DimensionMismatch);
    } else if eval.len() != n {
        return Err(GenHermError::DimensionMismatch);
    } else if evec.shape()[0] != evec.shape()[1] {
        return Err(GenHermError::InvalidMatrix);
    } else if evec.shape()[0] != n {
        return Err(GenHermError::DimensionMismatch);
    } else if workspace.size != n {
        return Err(GenHermError::DimensionMismatch);
    }

    // Compute Cholesky factorization of B
    let cholesky = b.cholesky(UPLO::Lower)
        .map_err(|_| GenHermError::NotPositiveDefinite)?;

    // Transform to standard Hermitian eigenvalue problem
    genherm_standardize(a, &cholesky);

    // Compute eigenvalues and eigenvectors
    let (eigenvalues, eigenvectors) = a.eigh(UPLO::Lower)
        .map_err(|_| GenHermError::InvalidMatrix)?;
    eval.assign(&eigenvalues);
    evec.assign(&eigenvectors);

    // Backtransform eigenvectors: evec -> L^{-H} evec
    let l_inv_h = cholesky.invh().unwrap();
    *evec = l_inv_h.t().dot(evec);

    // Renormalize eigenvectors
    genhermv_normalize_eigenvectors(evec);

    Ok(())
}

fn genherm_standardize(a: &mut Array2<Complex64>, cholesky: &Array2<Complex64>) {
    let l_inv = cholesky.inv().unwrap();
    *a = l_inv.t().dot(a).dot(&l_inv);
}

fn genhermv_normalize_eigenvectors(evec: &mut Array2<Complex64>) {
    for mut col in evec.columns_mut() {
        let norm = col.norm();
        col.scale(Complex64::new(1.0 / norm, 0.0));
    }
}

struct HermWorkspace {
    size: usize,
}

impl HermWorkspace {
    fn new(n: usize) -> Result<Self, GenHermError> {
        if n == 0 {
            return Err(GenHermError::InvalidMatrix);
        }
        Ok(Self { size: n })
    }
}