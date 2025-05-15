use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis, s};
use ndarray_linalg::{QR, Solve, Norm};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use std::cmp::min;
use std::f64::EPSILON;

/// QRPT decomposition for a matrix A, producing Q, R, P, and tau vectors
pub fn qrpt_decomp(
    a: &mut Array2<f64>,
    tau: &mut Array1<f64>,
    p: &mut Array1<usize>,
    signum: &mut i32,
    norm: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let (m, n) = a.dim();
    let k = min(m, n);

    if tau.len() != k {
        return Err("size of tau must be MIN(M,N)");
    }
    if p.len() != n {
        return Err("permutation size must be N");
    }
    if norm.len() != n {
        return Err("norm size must be N");
    }

    *signum = 1;

    // Initialize permutation to identity
    for (i, pi) in p.iter_mut().enumerate() {
        *pi = i;
    }

    // Compute column norms
    for i in 0..n {
        let col = a.column(i);
        norm[i] = col.norm_l2();
    }

    for i in 0..k {
        // Find column with largest norm
        let mut max_norm = norm[i];
        let mut kmax = i;

        for j in i + 1..n {
            let x = norm[j];
            if x > max_norm {
                max_norm = x;
                kmax = j;
            }
        }

        if kmax != i {
            // Swap columns
            a.swap_columns(i, kmax);
            p.swap(i, kmax);
            norm.swap(i, kmax);
            *signum = -*signum;
        }

        // Compute Householder transformation
        let mut c = a.slice_mut(s![i.., i]);
        let tau_i = householder_transform(&mut c);
        tau[i] = tau_i;

        // Apply transformation to remaining columns
        if i + 1 < n {
            let mut m = a.slice_mut(s![i.., i + 1..]);
            householder_hm(tau_i, &c.view(), &mut m);
        }

        // Update norms of remaining columns
        if i + 1 < m {
            for j in i + 1..n {
                let x = norm[j];
                if x > 0.0 {
                    let a_ij = a[(i, j)];
                    let temp = a_ij / x;
                    let y = if temp.abs() >= 1.0 {
                        0.0
                    } else {
                        x * (1.0 - temp * temp).sqrt()
                    };

                    // Recompute norm if accuracy loss detected
                    if (y / x).abs() < (20.0f64).sqrt() * EPSILON.sqrt() {
                        let c_col = a.column(j);
                        let c = c_col.slice(s![i + 1..]);
                        norm[j] = c.norm_l2();
                    } else {
                        norm[j] = y;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Householder transformation
fn householder_transform(v: &mut Array1<f64>) -> f64 {
    let norm = v.norm_l2();
    if norm == 0.0 {
        return 0.0;
    }

    let alpha = -v[0].signum() * norm;
    let beta = 1.0 / (v[0] - alpha);
    v[0] = alpha;

    for i in 1..v.len() {
        v[i] *= beta;
    }

    -alpha
}

/// Apply Householder transformation to matrix
fn householder_hm(tau: f64, v: &ArrayView1<f64>, a: &mut ArrayViewMut2<f64>) {
    // Implementation of Householder matrix multiplication
    // A = (I - tau * v * v^T) * A
    let m = a.nrows();
    let n = a.ncols();

    for j in 0..n {
        let mut dot = 0.0;
        for i in 0..m {
            dot += v[i] * a[(i, j)];
        }
        dot *= tau;

        for i in 0..m {
            a[(i, j)] -= v[i] * dot;
        }
    }
}

/// Solve system using QRPT decomposition
pub fn qrpt_solve(
    qr: &Array2<f64>,
    tau: &Array1<f64>,
    p: &Array1<usize>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
) -> Result<(), &'static str> {
    if qr.nrows() != qr.ncols() {
        return Err("QR matrix must be square");
    }
    if qr.nrows() != p.len() {
        return Err("matrix size must match permutation size");
    }
    if qr.nrows() != b.len() {
        return Err("matrix size must match b size");
    }
    if qr.ncols() != x.len() {
        return Err("matrix size must match solution size");
    }

    x.assign(b);
    qrpt_svx(qr, tau, p, x)
}

/// Solve system in-place using QRPT decomposition
pub fn qrpt_svx(
    qr: &Array2<f64>,
    tau: &Array1<f64>,
    p: &Array1<usize>,
    x: &mut Array1<f64>,
) -> Result<(), &'static str> {
    if qr.nrows() != qr.ncols() {
        return Err("QR matrix must be square");
    }
    if qr.nrows() != p.len() {
        return Err("matrix size must match permutation size");
    }
    if qr.ncols() != x.len() {
        return Err("matrix size must match solution size");
    }

    // Compute Q^T b
    qr_qtvec(qr, tau, x);

    // Solve R x = sol
    let r = qr;
    for i in 0..x.len() {
        for j in 0..i {
            x[i] -= r[(j, i)] * x[j];
        }
        x[i] /= r[(i, i)];
    }

    // Apply inverse permutation
    let mut temp = x.clone();
    for (i, &pi) in p.iter().enumerate() {
        x[pi] = temp[i];
    }

    Ok(())
}

/// Apply Q^T to vector
fn qr_qtvec(qr: &Array2<f64>, tau: &Array1<f64>, x: &mut Array1<f64>) {
    let m = qr.nrows();
    let n = qr.ncols();
    let k = min(m, n);

    for i in 0..k {
        let v = qr.slice(s![i.., i]);
        let tau_i = tau[i];

        // Compute v^T * x[i..]
        let mut dot = 0.0;
        for j in 0..v.len() {
            dot += v[j] * x[i + j];
        }
        dot *= tau_i;

        // x[i..] -= dot * v
        for j in 0..v.len() {
            x[i + j] -= dot * v[j];
        }
    }
}

/// Least squares solution using QRPT decomposition
pub fn qrpt_lssolve(
    qr: &Array2<f64>,
    tau: &Array1<f64>,
    p: &Array1<usize>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
    residual: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let n = qr.ncols();
    qrpt_lssolve2(qr, tau, p, b, n, x, residual)
}

/// Rank-deficient least squares solution using QRPT decomposition
pub fn qrpt_lssolve2(
    qr: &Array2<f64>,
    tau: &Array1<f64>,
    p: &Array1<usize>,
    b: &Array1<f64>,
    rank: usize,
    x: &mut Array1<f64>,
    residual: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let m = qr.nrows();
    let n = qr.ncols();

    if m < n {
        return Err("QR matrix must have M>=N");
    }
    if m != b.len() {
        return Err("matrix size must match b size");
    }
    if rank == 0 || rank > n {
        return Err("rank must have 0 < rank <= N");
    }
    if n != x.len() {
        return Err("matrix size must match solution size");
    }
    if m != residual.len() {
        return Err("matrix size must match residual size");
    }

    residual.assign(b);
    qr_qtvec(qr, tau, residual);

    // Solve R11 x1 = QTb1
    let r11 = qr.slice(s![0..rank, 0..rank]);
    let qtb1 = residual.slice(s![0..rank]);
    let mut x1 = x.slice_mut(s![0..rank]);

    // Forward substitution
    for i in 0..rank {
        let mut sum = qtb1[i];
        for j in 0..i {
            sum -= r11[(j, i)] * x1[j];
        }
        x1[i] = sum / r11[(i, i)];
    }

    // Set x[rank..] to zero
    for i in rank..n {
        x[i] = 0.0;
    }

    // Apply inverse permutation
    let mut temp = x.clone();
    for (i, &pi) in p.iter().enumerate() {
        x[pi] = temp[i];
    }

    // Compute residual = Q * (QTb with first rank elements zeroed)
    let mut qtb = residual.clone();
    for i in 0..rank {
        qtb[i] = 0.0;
    }
    qr_qvec(qr, tau, &qtb, residual);

    Ok(())
}

/// Apply Q to vector
fn qr_qvec(qr: &Array2<f64>, tau: &Array1<f64>, x: &Array1<f64>, out: &mut Array1<f64>) {
    out.assign(x);
    let m = qr.nrows();
    let n = qr.ncols();
    let k = min(m, n);

    for i in (0..k).rev() {
        let v = qr.slice(s![i.., i]);
        let tau_i = tau[i];

        // Compute v^T * out[i..]
        let mut dot = 0.0;
        for j in 0..v.len() {
            dot += v[j] * out[i + j];
        }
        dot *= tau_i;

        // out[i..] -= dot * v
        for j in 0..v.len() {
            out[i + j] -= dot * v[j];
        }
    }
}

/// Estimate rank of QRPT decomposition
pub fn qrpt_rank(qr: &Array2<f64>, tol: f64) -> usize {
    let m = qr.nrows();
    let n = qr.ncols();
    let diag = qr.diag();
    let eps = if tol < 0.0 {
        let min = diag.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = diag.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let absmax = min.abs().max(max.abs());
        let ee = absmax.log2() as i32;
        20.0 * (m + n) as f64 * 2.0f64.powi(ee) * EPSILON
    } else {
        tol
    };

    diag.iter().filter(|&&d| d.abs() > eps).count()
}