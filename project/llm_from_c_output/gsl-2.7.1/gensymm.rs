use ndarray::{Array2, Array1, ArrayView2, ArrayView1, Axis};
use ndarray_linalg::{cholesky::*, solve::*, SymmetricEigen, UPLO};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EigenError {
    #[error("matrix dimension must be positive integer")]
    InvalidDimension,
    #[error("failed to allocate space for workspace")]
    AllocationFailed,
    #[error("matrix must be square to compute eigenvalues")]
    NotSquare,
    #[error("B matrix dimensions must match A")]
    DimensionMismatch,
    #[error("eigenvalue vector must match matrix size")]
    VectorSizeMismatch,
    #[error("matrix size does not match workspace")]
    WorkspaceSizeMismatch,
    #[error("B matrix is not positive definite")]
    NotPositiveDefinite,
    #[error("LAPACK error")]
    LapackError(#[from] ndarray_linalg::error::LinalgError),
}

pub struct GensymmWorkspace {
    size: usize,
    symm_workspace: SymmetricEigen<f64>,
}

impl GensymmWorkspace {
    pub fn new(n: usize) -> Result<Self, EigenError> {
        if n == 0 {
            return Err(EigenError::InvalidDimension);
        }

        let symm_workspace = SymmetricEigen::new(n);
        Ok(Self {
            size: n,
            symm_workspace,
        })
    }
}

pub fn gensymm(
    a: &mut Array2<f64>,
    b: &mut Array2<f64>,
    eval: &mut Array1<f64>,
    workspace: &GensymmWorkspace,
) -> Result<(), EigenError> {
    let n = a.nrows();

    if n != a.ncols() {
        return Err(EigenError::NotSquare);
    } else if n != b.nrows() || n != b.ncols() {
        return Err(EigenError::DimensionMismatch);
    } else if eval.len() != n {
        return Err(EigenError::VectorSizeMismatch);
    } else if workspace.size != n {
        return Err(EigenError::WorkspaceSizeMismatch);
    }

    // Compute Cholesky factorization of B
    let b_cholesky = b.cholesky(UPLO::Lower)?;

    // Transform to standard symmetric eigenvalue problem
    gensymm_standardize(a, &b_cholesky)?;

    // Compute eigenvalues
    let eigs = a.symmetric_eigen()?;
    eval.assign(&eigs.eigenvalues);

    Ok(())
}

fn gensymm_standardize(a: &mut Array2<f64>, b_cholesky: &Array2<f64>) -> Result<(), EigenError> {
    gensymm_standardize_l3(a, b_cholesky)
}

fn gensymm_standardize_l2(a: &mut Array2<f64>, b_cholesky: &Array2<f64>) -> Result<(), EigenError> {
    let n = a.nrows();
    
    for i in 0..n {
        let b_ii = b_cholesky[(i, i)];
        let mut a_i = a.slice_mut(s![i, i..]);
        a_i[0] /= b_ii * b_ii;

        if i < n - 1 {
            let mut a_sub = a.slice_mut(s![i+1.., i]);
            let b_sub = b_cholesky.slice(s![i+1.., i]);
            
            // a_sub /= b_ii
            a_sub /= b_ii;

            let c = -0.5 * a_i[0];
            a_sub.scaled_add(c, &b_sub);

            // Perform symmetric rank-2 update
            let mut a_block = a.slice_mut(s![i+1.., i+1..]);
            let b_block = b_cholesky.slice(s![i+1.., i+1..]);
            ndarray_linalg::ops::syr2(-1.0, &a_sub, &b_sub, &mut a_block)?;

            a_sub.scaled_add(c, &b_sub);

            // Solve triangular system
            let a_sub_vec = a_sub.to_owned();
            let sol = b_block.solve_triangular(UPLO::Lower, ndarray_linalg::Diag::NonUnit, &a_sub_vec)?;
            a_sub.assign(&sol);
        }
    }

    Ok(())
}

const CROSSOVER_GENSYMM: usize = 64;

fn gensymm_standardize_l3(a: &mut Array2<f64>, b_cholesky: &Array2<f64>) -> Result<(), EigenError> {
    let n = a.nrows();
    
    if n <= CROSSOVER_GENSYMM {
        gensymm_standardize_l2(a, b_cholesky)
    } else {
        let n1 = n / 2;
        let n2 = n - n1;

        let (mut a11, mut a21, mut a22) = (
            a.slice_mut(s![..n1, ..n1]),
            a.slice_mut(s![n1.., ..n1]),
            a.slice_mut(s![n1.., n1..]),
        );
        
        let (b11, b21, b22) = (
            b_cholesky.slice(s![..n1, ..n1]),
            b_cholesky.slice(s![n1.., ..n1]),
            b_cholesky.slice(s![n1.., n1..]),
        );

        // Recursion on (A11, B11)
        gensymm_standardize_l3(&mut a11.to_owned(), &b11)?;
        a.slice_mut(s![..n1, ..n1]).assign(&a11);

        // A21 = A21 * B11^{-1}
        let mut a21_owned = a21.to_owned();
        ndarray_linalg::solve_triangular(&b11, UPLO::Lower, ndarray_linalg::Transpose::Trans, &mut a21_owned)?;
        a21.assign(&a21_owned);

        // A21 = A21 - 1/2 B21 A11
        let mut temp = Array2::zeros((n2, n1));
        ndarray_linalg::symm(1.0, &a11, &b21, 0.0, &mut temp)?;
        a21.scaled_add(-0.5, &temp);

        // A22 = A22 - A21 * B21' - B21 * A21'
        let mut a22_owned = a22.to_owned();
        ndarray_linalg::syr2k(-1.0, &a21, &b21, 1.0, &mut a22_owned)?;
        a22.assign(&a22_owned);

        // A21 = A21 - 1/2 B21 A11
        a21.scaled_add(-0.5, &temp);

        // A21 = B22^{-1} * A21
        let mut a21_owned = a21.to_owned();
        ndarray_linalg::solve_triangular(&b22, UPLO::Lower, ndarray_linalg::Transpose::No, &mut a21_owned)?;
        a21.assign(&a21_owned);

        // Recursion on (A22, B22)
        gensymm_standardize_l3(&mut a22.to_owned(), &b22)?;
        a.slice_mut(s![n1.., n1..]).assign(&a22);

        Ok(())
    }
}