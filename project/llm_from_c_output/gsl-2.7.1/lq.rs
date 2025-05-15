use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayViewMut1, ArrayViewMut2, s};
use ndarray_linalg::{Householder, Solve, LeastSquares};
use num_traits::Float;

/// Factorise a general N x M matrix A into A = L Q
/// where Q is orthogonal (M x M) and L is lower triangular (N x M).
pub fn lq_decomp(a: &mut Array2<f64>, tau: &mut Array1<f64>) -> Result<(), &'static str> {
    let (n, m) = a.dim();
    let k = n.min(m);

    if tau.len() != k {
        return Err("size of tau must be MIN(M,N)");
    }

    for i in 0..k {
        let mut c = a.slice_mut(s![i, i..]);
        let tau_i = householder_transform(&mut c);
        tau[i] = tau_i;

        if i + 1 < n {
            let mut m = a.slice_mut(s![i+1.., i..]);
            apply_householder(&c.view(), tau_i, &mut m);
        }
    }

    Ok(())
}

/// Solve the system x^T A = b^T using the LQ factorization
pub fn lq_solve_t(
    lq: &Array2<f64>,
    tau: &Array1<f64>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let n = lq.nrows();
    if n != lq.ncols() {
        return Err("LQ matrix must be square");
    }
    if lq.ncols() != b.len() {
        return Err("matrix size must match b size");
    }
    if lq.nrows() != x.len() {
        return Err("matrix size must match solution size");
    }

    x.assign(b);
    lq_svx_t(lq, tau, x)
}

/// Solve the system x^T A = b^T in place using LQ factorization
pub fn lq_svx_t(lq: &Array2<f64>, tau: &Array1<f64>, x: &mut Array1<f64>) -> Result<(), &'static str> {
    let n = lq.nrows();
    if n != lq.ncols() {
        return Err("LQ matrix must be square");
    }
    if lq.nrows() != x.len() {
        return Err("matrix size must match x/rhs size");
    }

    lq_vec_qt(lq, tau, x)?;
    x.solve_triangular_into(ndarray_linalg::UPLO::Lower, lq)
        .map_err(|_| "failed to solve triangular system")
}

/// Find least squares solution to overdetermined system x^T A = b^T
pub fn lq_lssolve_t(
    lq: &Array2<f64>,
    tau: &Array1<f64>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
    residual: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let (n, m) = lq.dim();
    if m < n {
        return Err("LQ matrix must have M>=N");
    }
    if m != b.len() {
        return Err("matrix size must match b size");
    }
    if n != x.len() {
        return Err("matrix size must match solution size");
    }
    if m != residual.len() {
        return Err("matrix size must match residual size");
    }

    let l = lq.slice(s![..n, ..n]);
    residual.assign(b);

    lq_vec_qt(lq, tau, residual)?;

    x.assign(&residual.slice(s![..n]));
    x.solve_triangular_into(ndarray_linalg::UPLO::Lower, &l)
        .map_err(|_| "failed to solve triangular system")?;

    residual.slice_mut(s![..n]).fill(0.0);
    lq_vec_q(lq, tau, residual)?;

    Ok(())
}

/// Form the orthogonal matrix Q from packed LQ matrix
pub fn lq_unpack(
    lq: &Array2<f64>,
    tau: &Array1<f64>,
    q: &mut Array2<f64>,
    l: &mut Array2<f64>,
) -> Result<(), &'static str> {
    let (m, n) = lq.dim();
    let k = m.min(n);

    if q.dim() != (n, n) {
        return Err("Q matrix must be N x N");
    }
    if l.dim() != (m, n) {
        return Err("L matrix must be M x N");
    }
    if tau.len() != k {
        return Err("size of tau must be MIN(M,N)");
    }

    q.fill(0.0);
    for i in 0..n {
        q[(i, i)] = 1.0;
    }

    for i in (0..k).rev() {
        let h = lq.slice(s![i, i..]);
        let mut m = q.slice_mut(s![i.., i..]);
        apply_householder(&h, tau[i], &mut m);
    }

    for i in 0..m {
        let l_border = i.min(n - 1);
        for j in 0..=l_border {
            l[(i, j)] = lq[(i, j)];
        }
        for j in l_border + 1..n {
            l[(i, j)] = 0.0;
        }
    }

    Ok(())
}

/// Compute v Q^T where Q is from LQ factorization
pub fn lq_vec_qt(lq: &Array2<f64>, tau: &Array1<f64>, v: &mut Array1<f64>) -> Result<(), &'static str> {
    let (n, m) = lq.dim();
    let k = n.min(m);

    if tau.len() != k {
        return Err("size of tau must be MIN(M,N)");
    }
    if v.len() != m {
        return Err("vector size must be M");
    }

    for i in 0..k {
        let h = lq.slice(s![i, i..]);
        let mut w = v.slice_mut(s![i..]);
        apply_householder_vec(&h, tau[i], &mut w);
    }

    Ok(())
}

/// Compute v Q where Q is from LQ factorization
pub fn lq_vec_q(lq: &Array2<f64>, tau: &Array1<f64>, v: &mut Array1<f64>) -> Result<(), &'static str> {
    let (n, m) = lq.dim();
    let k = n.min(m);

    if tau.len() != k {
        return Err("size of tau must be MIN(M,N)");
    }
    if v.len() != m {
        return Err("vector size must be M");
    }

    for i in (0..k).rev() {
        let h = lq.slice(s![i, i..]);
        let mut w = v.slice_mut(s![i..]);
        apply_householder_vec(&h, tau[i], &mut w);
    }

    Ok(())
}

/// Helper: Householder transformation
fn householder_transform(v: &mut ArrayViewMut1<f64>) -> f64 {
    let norm = v.mapv(|x| x.powi(2)).sum().sqrt();
    if norm == 0.0 {
        return 0.0;
    }

    let alpha = -v[0].signum() * norm;
    let beta = 1.0 / (v[0] - alpha);
    v[0] = alpha;

    let tau = (alpha - v[0]) / alpha;
    v[1..] *= beta;

    tau
}

/// Helper: Apply Householder transformation to matrix
fn apply_householder(h: &ArrayView1<f64>, tau: f64, m: &mut ArrayViewMut2<f64>) {
    for mut col in m.columns_mut() {
        let dot = h.dot(&col);
        col.scaled_add(-tau * dot, h);
    }
}

/// Helper: Apply Householder transformation to vector
fn apply_householder_vec(h: &ArrayView1<f64>, tau: f64, v: &mut ArrayViewMut1<f64>) {
    let dot = h.dot(v);
    v.scaled_add(-tau * dot, h);
}