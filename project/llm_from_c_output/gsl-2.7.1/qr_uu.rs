use std::f64;
use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, ArrayViewMut2, ArrayView2, s};
use ndarray_linalg::{Solve, Norm, Lapack, Scalar};
use num_traits::{Float, Zero, One};

/// Compute the QR decomposition of the "triangle on top of triangle" matrix
/// 
/// [ U ] = Q [ R ]
/// [ S ]     [ 0 ]
/// 
/// where U,S are N-by-N upper triangular
/// 
/// # Arguments
/// 
/// * `u` - on input, upper triangular N-by-N matrix
///         on output, R factor in upper triangle
/// * `s` - on input, upper triangular N-by-N matrix
///         on output, upper triangular Householder matrix V
/// * `t` - (output) block reflector matrix, N-by-N
/// 
/// # Returns
/// 
/// * `Result<(), &'static str>` - Ok on success, Err on failure
pub fn qr_uu_decomp(
    u: &mut Array2<f64>,
    s: &mut Array2<f64>,
    t: &mut Array2<f64>,
) -> Result<(), &'static str> {
    let n = u.nrows();

    if n != u.ncols() {
        return Err("U matrix must be square");
    } else if s.nrows() != s.ncols() {
        return Err("S matrix must be square");
    } else if n != s.nrows() {
        return Err("S and U must have same dimensions");
    } else if t.nrows() != n || t.ncols() != n {
        return Err("T matrix has wrong dimensions");
    } else if n == 1 {
        // base case, compute Householder transform for single column matrix
        let tau = qrtt_householder_transform(&mut u[[0, 0]], &mut s[[0, 0]]);
        t[[0, 0]] = tau;
        Ok(())
    } else {
        let n1 = n / 2;
        let n2 = n - n1;

        let (mut u11, mut u12, mut u22) = (
            u.slice_mut(s![..n1, ..n1]),
            u.slice_mut(s![..n1, n1..]),
            u.slice_mut(s![n1.., n1..]),
        );

        let (mut s11, mut s12) = (
            s.slice_mut(s![..n1, ..n1]),
            s.slice_mut(s![..n1, n1..]),
        );

        let (mut t11, mut t12, mut t22) = (
            t.slice_mut(s![..n1, ..n1]),
            t.slice_mut(s![..n1, n1..]),
            t.slice_mut(s![n1.., n1..]),
        );

        // Recursively factor [U11; S11]
        qr_uu_decomp(&mut u11.to_owned(), &mut s11.to_owned(), &mut t11.to_owned())?;

        // Compute W = T11^T (U12 + V31^T S12), using T12 as temporary storage
        let mut w = t12.to_owned();
        w.assign(&s12);
        w = s11.t().dot(&w);
        w += &u12;
        w = t11.t().dot(&w);
        u12 -= &w;
        w = s11.dot(&w);
        s12 -= &w;

        // Factor [U22; S12~; S22]
        let mut s_sub = s.slice_mut(s![.., n1..]);
        qr_uz_decomp(&mut u22.to_owned(), &mut s_sub.to_owned(), &mut t22.to_owned())?;

        // Update T12 := -T11 * V1^T * V2 * T22
        w.assign(&s12);
        w = s11.t().dot(&w);
        w = (-1.0) * t11.dot(&w);
        w = w.dot(&t22);
        t12.assign(&w);

        Ok(())
    }
}

/// Find the least squares solution to the overdetermined system
/// 
/// [ U ] x = b
/// [ S ]
/// 
/// using the QR factorization [ U; S ] = Q R.
/// 
/// # Arguments
/// 
/// * `r` - upper triangular R matrix, N-by-N
/// * `y` - upper triangular Y matrix, N-by-N
/// * `t` - upper triangular block reflector, N-by-N
/// * `b` - right hand side, size 2*N
/// * `x` - (output) solution, size 2*N
///          x[0..N] = least squares solution vector
///          x[N..2*N] = vector whose norm equals ||b - Ax||
/// * `work` - workspace, size N
/// 
/// # Returns
/// 
/// * `Result<(), &'static str>` - Ok on success, Err on failure
pub fn qr_uu_lssolve(
    r: &Array2<f64>,
    y: &Array2<f64>,
    t: &Array2<f64>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
    work: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let n = r.nrows();
    let m = 2 * n;

    if r.ncols() != n {
        return Err("R matrix must be square");
    } else if y.nrows() != y.ncols() {
        return Err("Y matrix must be square");
    } else if y.nrows() != n {
        return Err("Y and R must have same dimensions");
    } else if t.nrows() != n || t.ncols() != n {
        return Err("T matrix must be N-by-N");
    } else if m != b.len() {
        return Err("matrix size must match b size");
    } else if m != x.len() {
        return Err("matrix size must match solution size");
    } else if n != work.len() {
        return Err("workspace must be length N");
    }

    // Compute x = Q^T b
    x.assign(b);
    qr_uu_qtvec(y, t, x, work)?;

    // Solve R x = Q^T b
    let mut x1 = x.slice_mut(s![..n]);
    x1.assign(&r.solve(&x1).map_err(|_| "Failed to solve triangular system")?);

    Ok(())
}

/// Apply 2N-by-2N Q^T to the 2N-by-1 vector b
/// 
/// # Arguments
/// 
/// * `y` - upper triangular Y matrix encoded by qr_uu_decomp, N-by-N
/// * `t` - block reflector matrix, N-by-N
/// * `b` - 2N-by-1 vector replaced by Q^T b on output
/// * `work` - workspace, length N
/// 
/// # Returns
/// 
/// * `Result<(), &'static str>` - Ok on success, Err on failure
pub fn qr_uu_qtvec(
    y: &Array2<f64>,
    t: &Array2<f64>,
    b: &mut Array1<f64>,
    work: &mut Array1<f64>,
) -> Result<(), &'static str> {
    let n = y.nrows();
    let m = 2 * n;

    if y.ncols() != n {
        return Err("Y matrix must be square");
    } else if t.nrows() != n || t.ncols() != n {
        return Err("T matrix must be N-by-N");
    } else if b.len() != m {
        return Err("b vector must have length M");
    } else if work.len() != n {
        return Err("workspace must be length N");
    }

    let (mut b1, mut b2) = (b.slice_mut(s![..n]), b.slice_mut(s![n..]);

    // work := Y^T b2
    work.assign(&b2);
    work.assign(&y.t().dot(work));

    // work = b1 + Y^T b2
    work += &b1;

    // work = T^T * work
    work.assign(&t.t().dot(work));

    // b1 := b1 - work
    b1 -= &work;

    // b2 := b2 - Y w
    let yw = y.dot(work);
    b2 -= &yw;

    Ok(())
}

/// Optimized householder transform for QR decomposition of [U; S] matrices
/// 
/// # Arguments
/// 
/// * `v0` - pointer to diagonal element of U (input/output)
/// * `v1` - pointer to diagonal element of S (input/output)
/// 
/// # Returns
/// 
/// Householder coefficient tau
fn qrtt_householder_transform(v0: &mut f64, v1: &mut f64) -> f64 {
    let xnorm = v1.abs();

    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha * alpha + xnorm * xnorm).sqrt();
    let tau = (beta - alpha) / beta;

    let s = alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        *v1 /= s;
        *v0 = beta;
    } else {
        *v1 *= f64::EPSILON / s;
        *v1 /= f64::EPSILON;
        *v0 = beta;
    }

    tau
}

/// QR decomposition for "triangle on top of trapezoidal" matrix
/// 
/// (Internal helper function)
fn qr_uz_decomp(
    u: &mut Array2<f64>,
    s: &mut Array2<f64>,
    t: &mut Array2<f64>,
) -> Result<(), &'static str> {
    // Implementation would be similar to qr_uu_decomp but specialized for trapezoidal case
    // Omitted for brevity - would need to be implemented for full functionality
    unimplemented!()
}