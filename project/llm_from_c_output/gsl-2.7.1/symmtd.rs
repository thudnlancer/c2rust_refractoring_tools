use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, ArrayViewMut2, Axis};
use ndarray_linalg::{Householder, Norm, Lapack};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymmtdError {
    #[error("symmetric tridiagonal decomposition requires square matrix")]
    NotSquare,
    #[error("size of tau must be N-1")]
    BadTauSize,
    #[error("size of Q must match size of A")]
    BadQSize,
    #[error("size of diagonal must match size of A")]
    BadDiagSize,
    #[error("size of subdiagonal must be (matrix size - 1)")]
    BadSubdiagSize,
}

/// Factorize a symmetric matrix A into A = Q T Q'
/// where Q is orthogonal and T is symmetric tridiagonal.
pub fn symmtd_decomp(
    a: &mut Array2<f64>,
    tau: &mut Array1<f64>,
) -> Result<(), SymmtdError> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(SymmtdError::NotSquare);
    }
    if tau.len() + 1 != n {
        return Err(SymmtdError::BadTauSize);
    }

    for i in 0..n - 2 {
        let mut v = a.slice_mut(s![i + 1.., i]);
        let tau_i = householder_transform(&mut v);
        
        if tau_i != 0.0 {
            let ei = v[0];
            v[0] = 1.0;

            let mut m = a.slice_mut(s![i + 1.., i + 1..]);
            let mut x = tau.slice_mut(s![i..]);

            // x = tau * A * v
            x.fill(0.0);
            symmetric_matrix_vector_product(&m, &v, tau_i, &mut x);

            // w = x - (1/2) tau * (x' * v) * v
            let xv = x.dot(&v);
            let alpha = -0.5 * tau_i * xv;
            x.scaled_add(alpha, &v);

            // A = A - v w' - w v'
            rank2_update(&mut m, -1.0, &v, &x);

            v[0] = ei;
        }

        tau[i] = tau_i;
    }

    Ok(())
}

/// Form the orthogonal matrix Q from the packed QR matrix
pub fn symmtd_unpack(
    a: &Array2<f64>,
    tau: &Array1<f64>,
    q: &mut Array2<f64>,
    diag: &mut Array1<f64>,
    sdiag: &mut Array1<f64>,
) -> Result<(), SymmtdError> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(SymmtdError::NotSquare);
    }
    if tau.len() + 1 != n {
        return Err(SymmtdError::BadTauSize);
    }
    if q.nrows() != n || q.ncols() != n {
        return Err(SymmtdError::BadQSize);
    }
    if diag.len() != n {
        return Err(SymmtdError::BadDiagSize);
    }
    if sdiag.len() + 1 != n {
        return Err(SymmtdError::BadSubdiagSize);
    }

    // Initialize Q to identity
    q.fill(0.0);
    for i in 0..n {
        q[(i, i)] = 1.0;
    }

    for i in (0..n - 2).rev() {
        let h = a.slice(s![i + 1.., i]);
        let ti = tau[i];
        let mut m = q.slice_mut(s![i + 1.., i + 1..]);
        let mut work = diag.slice_mut(s![0..n - i - 1]);
        
        let mut h_copy = h.to_owned();
        let tmp = h_copy[0];
        h_copy[0] = 1.0;
        
        apply_householder_left(&h_copy.view(), ti, &mut m, &mut work);
        h_copy[0] = tmp;
    }

    // Copy diagonal and subdiagonal
    diag.assign(&a.diag());
    for i in 0..n - 1 {
        sdiag[i] = a[(i + 1, i)];
    }

    Ok(())
}

/// Unpack the tridiagonal matrix T from packed symmetric matrix
pub fn symmtd_unpack_t(
    a: &Array2<f64>,
    diag: &mut Array1<f64>,
    sdiag: &mut Array1<f64>,
) -> Result<(), SymmtdError> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(SymmtdError::NotSquare);
    }
    if diag.len() != n {
        return Err(SymmtdError::BadDiagSize);
    }
    if sdiag.len() + 1 != n {
        return Err(SymmtdError::BadSubdiagSize);
    }

    diag.assign(&a.diag());
    for i in 0..n - 1 {
        sdiag[i] = a[(i + 1, i)];
    }

    Ok(())
}

// Helper functions

fn householder_transform(v: &mut ArrayViewMut1<f64>) -> f64 {
    let norm = v.norm_l2();
    if norm == 0.0 {
        return 0.0;
    }

    let alpha = -v[0].signum() * norm;
    let beta = v[0] - alpha;
    v[0] = beta;
    
    if beta != 0.0 {
        v.mapv_inplace(|x| x / beta);
    }

    -2.0 * alpha * beta / (alpha.powi(2) + norm.powi(2) - v[0].powi(2))
}

fn symmetric_matrix_vector_product(
    a: &ArrayView2<f64>,
    x: &ArrayView1<f64>,
    alpha: f64,
    y: &mut ArrayViewMut1<f64>,
) {
    for i in 0..a.nrows() {
        let mut sum = 0.0;
        for j in 0..a.ncols() {
            sum += a[(i, j)] * x[j];
        }
        y[i] += alpha * sum;
    }
}

fn rank2_update(
    a: &mut ArrayViewMut2<f64>,
    alpha: f64,
    x: &ArrayView1<f64>,
    y: &ArrayView1<f64>,
) {
    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            a[(i, j)] += alpha * (x[i] * y[j] + y[i] * x[j]);
        }
    }
}

fn apply_householder_left(
    h: &ArrayView1<f64>,
    tau: f64,
    a: &mut ArrayViewMut2<f64>,
    work: &mut ArrayViewMut1<f64>,
) {
    let n = a.nrows();
    let m = a.ncols();
    
    // work = A' * h
    for j in 0..m {
        work[j] = 0.0;
        for i in 0..n {
            work[j] += a[(i, j)] * h[i];
        }
    }
    
    // A = A - tau * h * work'
    for i in 0..n {
        for j in 0..m {
            a[(i, j)] -= tau * h[i] * work[j];
        }
    }
}