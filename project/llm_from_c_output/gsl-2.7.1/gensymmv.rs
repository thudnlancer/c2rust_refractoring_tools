/*
 * Translated from C to Rust
 * Original source: eigen/gensymmv.c
 */

use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};
use libc::{c_int, c_double};
use gsl::{
    eigen::{symmv_alloc, symmv_free, symmv, SymmvWorkspace},
    linalg::{cholesky_decomp1},
    blas::{dtrsm, dnrm2, dscal},
    matrix::{Matrix, MatrixView, MatrixViewMut},
    vector::{Vector, VectorView, VectorViewMut},
    types::{GslResult, GslError},
};

/// Workspace for generalized symmetric-definite eigensystem
pub struct GensymmvWorkspace {
    size: usize,
    symmv_workspace: SymmvWorkspace,
}

impl GensymmvWorkspace {
    /// Allocate a workspace for solving the generalized symmetric-definite eigenvalue problem
    pub fn new(n: usize) -> Option<Self> {
        if n == 0 {
            return None;
        }

        let symmv_ws = symmv_alloc(n)?;
        
        Some(Self {
            size: n,
            symmv_workspace: symmv_ws,
        })
    }

    /// Free the workspace
    pub fn free(self) {
        // SymmvWorkspace is dropped automatically
    }
}

/// Solve the generalized symmetric-definite eigenvalue problem A x = λ B x
pub fn gensymmv(
    a: &mut Matrix,
    b: &mut Matrix,
    eval: &mut Vector,
    evec: &mut Matrix,
    w: &mut GensymmvWorkspace,
) -> GslResult<()> {
    let n = a.size1();

    // Check matrix and vector sizes
    if n != a.size2() {
        return Err(GslError::NotSquare);
    }
    if n != b.size1() || n != b.size2() {
        return Err(GslError::BadLength);
    }
    if eval.size() != n {
        return Err(GslError::BadLength);
    }
    if evec.size1() != evec.size2() {
        return Err(GslError::NotSquare);
    }
    if evec.size1() != n {
        return Err(GslError::BadLength);
    }
    if w.size != n {
        return Err(GslError::BadLength);
    }

    // Compute Cholesky factorization of B
    cholesky_decomp1(b)?;

    // Transform to standard symmetric eigenvalue problem
    gensymm_standardize(a, b);

    // Compute eigenvalues and eigenvectors
    symmv(a, eval, evec, &mut w.symmv_workspace)?;

    // Backtransform eigenvectors: evec -> L^{-T} evec
    dtrsm(
        CblasLeft,
        CblasLower,
        CblasTrans,
        CblasNonUnit,
        1.0,
        b,
        evec,
    );

    // Renormalize eigenvectors
    normalize_eigenvectors(evec);

    Ok(())
}

/// Normalize eigenvectors so their Euclidean norm is 1
fn normalize_eigenvectors(evec: &mut Matrix) {
    let n = evec.size1();
    
    for i in 0..n {
        let mut vi = evec.column(i);
        let norm = dnrm2(&vi);
        let scale = 1.0 / norm;
        
        dscal(scale, &mut vi);
    }
}

/// Transform generalized eigenproblem to standard form
fn gensymm_standardize(a: &mut Matrix, b: &Matrix) {
    // Implementation would mirror the original C version
    // This is a placeholder for the actual transformation logic
}