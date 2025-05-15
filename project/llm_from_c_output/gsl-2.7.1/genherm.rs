use ndarray::{Array2, Array1, ArrayView2, ArrayViewMut2, Axis};
use ndarray_linalg::{cholesky::*, eig::*, solve::*, types::*};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenHermError {
    #[error("matrix dimension must be positive integer")]
    InvalidDimension,
    #[error("failed to allocate space for workspace")]
    AllocationFailed,
    #[error("matrix must be square to compute eigenvalues")]
    NotSquare,
    #[error("matrix dimensions must match")]
    DimensionMismatch,
    #[error("matrix size does not match workspace")]
    WorkspaceSizeMismatch,
    #[error("B matrix is not positive definite")]
    NotPositiveDefinite,
    #[error("LAPACK error")]
    LapackError(#[from] ndarray_linalg::error::LinalgError),
}

pub struct GenHermWorkspace {
    size: usize,
    herm_workspace: HermWorkspace,
}

impl GenHermWorkspace {
    pub fn new(n: usize) -> Result<Self, GenHermError> {
        if n == 0 {
            return Err(GenHermError::InvalidDimension);
        }

        let herm_workspace = HermWorkspace::new(n)
            .map_err(|_| GenHermError::AllocationFailed)?;

        Ok(Self {
            size: n,
            herm_workspace,
        })
    }
}

pub fn genherm(
    a: &mut Array2<Complex64>,
    b: &mut Array2<Complex64>,
    eval: &mut Array1<f64>,
    workspace: &GenHermWorkspace,
) -> Result<(), GenHermError> {
    let n = a.shape()[0];

    if a.shape()[1] != n {
        return Err(GenHermError::NotSquare);
    } else if b.shape() != a.shape() {
        return Err(GenHermError::DimensionMismatch);
    } else if eval.len() != n {
        return Err(GenHermError::DimensionMismatch);
    } else if workspace.size != n {
        return Err(GenHermError::WorkspaceSizeMismatch);
    }

    // Compute Cholesky factorization of B
    let b_cholesky = b.cholesky(UPLO::Lower)?;

    // Transform to standard hermitian eigenvalue problem
    genherm_standardize(a, &b_cholesky)?;

    // Solve hermitian eigenvalue problem
    let (eigenvalues, _) = a.eig_into()?;
    *eval = eigenvalues.mapv(|c| c.re);

    Ok(())
}

fn genherm_standardize(
    a: &mut Array2<Complex64>,
    b: &Array2<Complex64>,
) -> Result<(), GenHermError> {
    if a.shape()[0] <= CROSSOVER_GENHERM {
        genherm_standardize_l2(a, b)
    } else {
        genherm_standardize_l3(a, b)
    }
}

fn genherm_standardize_l2(
    a: &mut Array2<Complex64>,
    b: &Array2<Complex64>,
) -> Result<(), GenHermError> {
    let n = a.shape()[0];

    for i in 0..n {
        let a_ii = a[(i, i)];
        let b_ii = b[(i, i)];
        let new_val = a_ii / (b_ii * b_ii.conj());
        a[(i, i)] = new_val;

        if i < n - 1 {
            let mut ai = a.slice_mut(s![i+1.., i]);
            let mut ma = a.slice_mut(s![i+1.., i+1..]);
            let bi = b.slice(s![i+1.., i]);
            let mb = b.slice(s![i+1.., i+1..]);

            ai /= b[(i, i)];

            let z = Complex64::new(-0.5 * new_val.re, 0.0);
            let mut temp = bi.to_owned();
            temp.scaled_add(z, &ai);

            // HER2 operation
            ma.her2(-Complex64::new(1.0, 0.0), &ai, &bi);

            ai.scaled_add(z, &bi);

            // TRSV operation
            let _ = solve_triangular(&mb, UPLO::Lower, Diag::NonUnit, &mut ai)?;
        }
    }

    Ok(())
}

fn genherm_standardize_l3(
    a: &mut Array2<Complex64>,
    b: &Array2<Complex64>,
) -> Result<(), GenHermError> {
    let n = a.shape()[0];
    if n <= CROSSOVER_GENHERM {
        return genherm_standardize_l2(a, b);
    }

    let n1 = split_complex(n);
    let n2 = n - n1;

    let (mut a11, mut a21, mut a22) = (
        a.slice_mut(s![..n1, ..n1]),
        a.slice_mut(s![n1.., ..n1]),
        a.slice_mut(s![n1.., n1..]),
    );

    let (b11, b21, b22) = (
        b.slice(s![..n1, ..n1]),
        b.slice(s![n1.., ..n1]),
        b.slice(s![n1.., n1..]),
    );

    let mhalf = Complex64::new(-0.5, 0.0);

    // Recursion on (A11, B11)
    genherm_standardize_l3(&mut a11, &b11)?;

    // A21 = A21 * B11^{-1}
    let _ = solve_triangular(&b11, UPLO::Lower, Diag::NonUnit, &mut a21.t())?;

    // A21 = A21 - 1/2 B21 A11
    let mut temp = Array2::zeros((n2, n1));
    temp.hemm(Side::Right, &a11, &b21, mhalf, Complex64::new(1.0, 0.0));
    a21 += &temp;

    // A22 = A22 - A21 * B21' - B21 * A21'
    a22.her2k(UPLO::Lower, &a21, &b21, -Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0));

    // A21 = A21 - 1/2 B21 A11
    a21 += &temp;

    // A21 = B22^{-1} * A21
    let _ = solve_triangular(&b22, UPLO::Lower, Diag::NonUnit, &mut a21)?;

    // Recursion on (A22, B22)
    genherm_standardize_l3(&mut a22, &b22)?;

    Ok(())
}

const CROSSOVER_GENHERM: usize = 128;

fn split_complex(n: usize) -> usize {
    n / 2
}

struct HermWorkspace {
    size: usize,
}

impl HermWorkspace {
    fn new(n: usize) -> Result<Self, GenHermError> {
        Ok(Self { size: n })
    }
}