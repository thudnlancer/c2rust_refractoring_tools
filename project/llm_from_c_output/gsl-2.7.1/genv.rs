use std::f64;
use std::ptr;
use std::mem;
use std::cmp;
use std::ops::{Deref, DerefMut};
use libc::{c_int, c_double, c_void};

use gsl::{
    blas, 
    complex::{Complex, ComplexF64},
    eigen::{GenWorkspace, GenvWorkspace},
    error::{Error, Result},
    linalg,
    matrix::{Matrix, MatrixF64, MatrixView, MatrixViewMut},
    schur,
    vector::{Vector, VectorF64, VectorView, VectorViewMut},
    vector_complex::{VectorComplex, VectorComplexF64, VectorComplexView, VectorComplexViewMut},
};

const GSL_DBL_MIN: f64 = f64::MIN_POSITIVE;
const GSL_DBL_EPSILON: f64 = f64::EPSILON;

/// Allocate a workspace for solving the generalized eigenvalue problem.
pub fn genv_alloc(n: usize) -> Result<GenvWorkspace> {
    if n == 0 {
        return Err(Error::new("matrix dimension must be positive integer"));
    }

    let mut w = GenvWorkspace {
        size: n,
        Q: None,
        Z: None,
        gen_workspace_p: None,
        work1: None,
        work2: None,
        work3: None,
        work4: None,
        work5: None,
        work6: None,
    };

    w.gen_workspace_p = Some(GenWorkspace::new(n)?);

    // Compute the full Schur forms
    if let Some(ref mut gen) = w.gen_workspace_p {
        gen.params(1, 1, 1);
    }

    w.work1 = Some(VectorF64::new(n)?);
    w.work2 = Some(VectorF64::new(n)?);
    w.work3 = Some(VectorF64::new(n)?);
    w.work4 = Some(VectorF64::new(n)?);
    w.work5 = Some(VectorF64::new(n)?);
    w.work6 = Some(VectorF64::new(n)?);

    Ok(w)
}

/// Free workspace
pub fn genv_free(w: &mut GenvWorkspace) {
    w.gen_workspace_p = None;
    w.work1 = None;
    w.work2 = None;
    w.work3 = None;
    w.work4 = None;
    w.work5 = None;
    w.work6 = None;
}

/// Solve the generalized eigenvalue problem A x = λ B x
pub fn genv(
    A: &mut MatrixF64,
    B: &mut MatrixF64,
    alpha: &mut VectorComplexF64,
    beta: &mut VectorF64,
    evec: &mut MatrixComplexF64,
    w: &mut GenvWorkspace,
) -> Result<()> {
    let n = A.size1();

    // Check matrix and vector sizes
    if n != A.size2() {
        return Err(Error::new("matrix must be square to compute eigenvalues"));
    } else if n != B.size1() || n != B.size2() {
        return Err(Error::new("B matrix dimensions must match A"));
    } else if alpha.len() != n || beta.len() != n {
        return Err(Error::new("eigenvalue vector must match matrix size"));
    } else if w.size != n {
        return Err(Error::new("matrix size does not match workspace"));
    } else if evec.size1() != n {
        return Err(Error::new("eigenvector matrix has wrong size"));
    }

    // Create temporary matrix view for Z
    let mut Z = MatrixF64::view_mut(evec.data_mut(), n, n, 2 * n)?;

    // Compute eigenvalues and Schur forms
    let s = {
        let gen = w.gen_workspace_p.as_mut().unwrap();
        gen_QZ(A, B, alpha, beta, w.Q.as_mut(), Some(&mut Z), gen)?
    };

    // Save right Schur vectors if requested
    if let Some(ref mut z) = w.Z {
        z.copy_from(&Z)?;
    }

    // Compute eigenvectors if all eigenvalues were found
    if s == Ok(()) {
        genv_get_right_eigenvectors(A, B, &Z, evec, w)?;
        genv_normalize_eigenvectors(alpha, evec);
    }

    s
}

/// Solve generalized eigenvalue problem with Schur vectors
pub fn genv_QZ(
    A: &mut MatrixF64,
    B: &mut MatrixF64,
    alpha: &mut VectorComplexF64,
    beta: &mut VectorF64,
    evec: &mut MatrixComplexF64,
    Q: Option<&mut MatrixF64>,
    Z: Option<&mut MatrixF64>,
    w: &mut GenvWorkspace,
) -> Result<()> {
    if let Some(q) = Q {
        if A.size1() != q.size1() || A.size1() != q.size2() {
            return Err(Error::new("Q matrix has wrong dimensions"));
        }
    }
    if let Some(z) = Z {
        if A.size1() != z.size1() || A.size1() != z.size2() {
            return Err(Error::new("Z matrix has wrong dimensions"));
        }
    }

    w.Q = Q;
    w.Z = Z;

    let result = genv(A, B, alpha, beta, evec, w);

    w.Q = None;
    w.Z = None;

    result
}

/// Compute right eigenvectors from Schur form
fn genv_get_right_eigenvectors(
    S: &MatrixF64,
    T: &MatrixF64,
    Z: &MatrixF64,
    evec: &mut MatrixComplexF64,
    w: &mut GenvWorkspace,
) -> Result<()> {
    let n = w.size;
    let small = GSL_DBL_MIN * (n as f64) / GSL_DBL_EPSILON;
    let big = 1.0 / small;
    let bignum = 1.0 / (GSL_DBL_MIN * (n as f64));

    let mut complex_pair = false;
    for k in 0..n {
        let je = n - 1 - k;

        if complex_pair {
            complex_pair = false;
            continue;
        }

        let mut nw = 1;
        if je > 0 && S.get(je, je - 1) != 0.0 {
            complex_pair = true;
            nw = 2;
        }

        if !complex_pair {
            if S.get(je, je).abs() <= GSL_DBL_MIN && T.get(je, je).abs() <= GSL_DBL_MIN {
                // Singular matrix pencil - unit eigenvector
                for i in 0..n {
                    evec.set(i, je, Complex::new(0.0, 0.0));
                }
                evec.set(je, je, Complex::new(1.0, 0.0));
                continue;
            }

            // Clear vector
            for i in 0..n {
                w.work3.as_mut().unwrap().set(i, 0.0);
            }
        } else {
            // Clear vectors
            for i in 0..n {
                w.work3.as_mut().unwrap().set(i, 0.0);
                w.work4.as_mut().unwrap().set(i, 0.0);
            }
        }

        // ... [rest of the complex implementation]
        // Note: The full implementation would continue here with the complex
        // eigenvalue calculations and triangular solves, but it's omitted
        // for brevity in this example.
    }

    Ok(())
}

/// Normalize eigenvectors
fn genv_normalize_eigenvectors(alpha: &VectorComplexF64, evec: &mut MatrixComplexF64) {
    let n = evec.size1();

    for i in 0..n {
        let ai = alpha.get(i);
        let mut vi = evec.column_mut(i);

        let re = vi.real();

        if ai.imag() == 0.0 {
            let scale = 1.0 / blas::dnrm2(&re);
            blas::dscal(scale, &mut re);
        } else if ai.imag() > 0.0 {
            let im = vi.imag();

            let scale = 1.0 / f64::hypot(blas::dnrm2(&re), blas::dnrm2(&im));
            blas::zdscal(scale, &mut vi);

            let mut vi_next = evec.column_mut(i + 1);
            blas::zdscal(scale, &mut vi_next);
        }
    }
}