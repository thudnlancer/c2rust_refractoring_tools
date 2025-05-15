use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayViewMut1, ArrayViewMut2};
use ndarray_linalg::{c32, c64, error::LinalgError, lapack::UPLO, norm::Norm, Scalar};
use num_traits::{Float, Num, One, Zero};
use std::cmp;
use std::f64::consts;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Compute the QR decomposition of the "triangle on top of diagonal" matrix
/// [ U ] = Q [ R ]
/// [ D ]     [ 0 ]
/// where U is N-by-N upper triangular, D is N-by-N diagonal
pub fn qr_ud_decomp(
    u: &mut Array2<f64>,
    d: &Array1<f64>,
    y: &mut Array2<f64>,
    t: &mut Array2<f64>,
) -> Result<(), LinalgError> {
    let n = u.nrows();
    assert_eq!(u.ncols(), n, "U matrix must be square");
    assert_eq!(d.len(), n, "D matrix does not match dimensions of U");
    assert_eq!(y.nrows(), n, "Y matrix must be square");
    assert_eq!(y.ncols(), n, "Y matrix does not match dimensions of U");
    assert_eq!(t.nrows(), n, "T matrix has wrong dimensions");
    assert_eq!(t.ncols(), n, "T matrix has wrong dimensions");

    if n == 1 {
        // base case, compute Householder transform for single column matrix
        let t00 = t[(0, 0)];
        let u00 = u[(0, 0)];
        let y00 = y[(0, 0)];
        y[(0, 0)] = d[0];
        t[(0, 0)] = qrtd_householder_transform(&mut u[(0, 0)], &mut y[(0, 0)]);
        Ok(())
    } else {
        let n1 = n / 2;
        let n2 = n - n1;

        let mut u11 = u.slice_mut(s![..n1, ..n1]);
        let mut u12 = u.slice_mut(s![..n1, n1..]);
        let mut u22 = u.slice_mut(s![n1.., n1..]);

        let d1 = d.slice(s![..n1]);
        let d2 = d.slice(s![n1..]);

        let mut y11 = y.slice_mut(s![..n1, ..n1]);
        let mut y12 = y.slice_mut(s![..n1, n1..]);

        let mut t11 = t.slice_mut(s![..n1, ..n1]);
        let mut t12 = t.slice_mut(s![..n1, n1..]);
        let mut t22 = t.slice_mut(s![n1.., n1..]);

        // Recursively factor [U11; D11]
        qr_ud_decomp(&mut u11, &d1, &mut y11, &mut t11)?;

        // Compute W = T11^T U12
        let mut w = u12.to_owned();
        w = t11.t().dot(&w);
        u12 -= &w;

        // Compute Y12 = -V31 W
        y12.assign(&(-1.0 * &y11.dot(&w)));

        // Factor [U22; Y12; D22]
        let mut m = y.slice_mut(s![.., n1..]);
        aux_qr_trd_decomp(&mut u22, &mut m, &d2, &mut t22)?;

        // Update T12 := -T11 * V1^T * V2 * T22
        t12.assign(&y12);
        t12 = y11.t().dot(&t12);
        t12 = (-1.0 * &t11).dot(&t12);
        t12 = t12.dot(&t22);

        Ok(())
    }
}

/// Find the least squares solution to the overdetermined system
/// [ U ] x = b
/// [ D ]
pub fn qr_ud_lssolve(
    r: &Array2<f64>,
    y: &Array2<f64>,
    t: &Array2<f64>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
    work: &mut Array1<f64>,
) -> Result<(), LinalgError> {
    let n = r.nrows();
    assert_eq!(r.ncols(), n, "R matrix must be square");
    assert_eq!(y.nrows(), n, "Y matrix must be square");
    assert_eq!(y.ncols(), n, "Y and R must have same dimensions");
    assert_eq!(t.nrows(), n, "T matrix must be N-by-N");
    assert_eq!(t.ncols(), n, "T matrix must be N-by-N");
    assert_eq!(b.len(), 2 * n, "matrix size must match b size");
    assert_eq!(x.len(), 2 * n, "matrix size must match solution size");
    assert_eq!(work.len(), n, "workspace must be length N");

    qr_uu_lssolve(r, y, t, b, x, work)
}

fn aux_qr_trd_decomp(
    u: &mut Array2<f64>,
    a: &mut Array2<f64>,
    d: &Array1<f64>,
    t: &mut Array2<f64>,
) -> Result<(), LinalgError> {
    let n = u.nrows();
    assert_eq!(u.ncols(), n, "U matrix must be square");
    assert!(a.nrows() > n, "A matrix must have > N rows");
    assert_eq!(d.len(), n, "D matrix does not match dimensions of U");
    assert_eq!(t.nrows(), n, "T matrix has wrong dimensions");
    assert_eq!(t.ncols(), n, "T matrix has wrong dimensions");

    if n == 1 {
        let m = a.nrows() - n;
        let t00 = t[(0, 0)];
        let u00 = u[(0, 0)];
        let mut v = a.slice_mut(s![..m, 0]);
        let d00 = a[(m, 0)];
        a[(m, 0)] = d[0];
        t[(0, 0)] = qrtrd_householder_transform(&mut u00, &mut v, &mut a[(m, 0)]);
        Ok(())
    } else {
        let m = a.nrows() - n;
        let n1 = n / 2;
        let n2 = n - n1;

        let mut u11 = u.slice_mut(s![..n1, ..n1]);
        let mut u12 = u.slice_mut(s![..n1, n1..]);
        let mut u22 = u.slice_mut(s![n1.., n1..]);

        let mut a1 = a.slice_mut(s![..m, ..n1]);
        let mut a2 = a.slice_mut(s![..m, n1..]);

        let d1 = d.slice(s![..n1]);
        let d2 = d.slice(s![n1..]);

        let mut t11 = t.slice_mut(s![..n1, ..n1]);
        let mut t12 = t.slice_mut(s![..n1, n1..]);
        let mut t22 = t.slice_mut(s![n1.., n1..]);

        let mut y41 = a.slice_mut(s![m..m + n1, ..n1]);
        let mut y42 = a.slice_mut(s![m..m + n1, n1..]);

        // Recursively factor [U11; A1; D11]
        let mut m_sub = a.slice_mut(s![..m + n1, ..n1]);
        aux_qr_trd_decomp(&mut u11, &mut m_sub, &d1, &mut t11)?;

        // Compute W = T11^T (U12 + V31^T A2)
        let mut w = u12.to_owned();
        w += &a1.t().dot(&a2);
        w = t11.t().dot(&w);
        u12 -= &w;

        // Compute A2 := A2 - V31 W
        a2 -= &a1.dot(&w);

        // Compute Y42 := -V41 W
        y42.assign(&(-1.0 * &y41.dot(&w)));

        // Factor [U22; A2; Y42; D22]
        let mut m_sub = a.slice_mut(s![.., n1..]);
        aux_qr_trd_decomp(&mut u22, &mut m_sub, &d2, &mut t22)?;

        // Update T12 := -T11 * (V31~^T V32~ + V41~^T V42~) * T22
        t12.assign(&y42);
        t12 = y41.t().dot(&t12);
        t12 += &a1.t().dot(&a2);
        t12 = (-1.0 * &t11).dot(&t12);
        t12 = t12.dot(&t22);

        Ok(())
    }
}

fn qrtd_householder_transform(v0: &mut f64, v1: &mut f64) -> f64 {
    let xnorm = v1.abs();
    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.hypot(xnorm));
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

fn qrtrd_householder_transform(
    v0: &mut f64,
    v: &mut ArrayViewMut1<f64>,
    d: &mut f64,
) -> f64 {
    let xnorm = v.norm_l2().hypot(*d);
    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.hypot(xnorm));
    let tau = (beta - alpha) / beta;

    let s = alpha - beta;
    if s.abs() > f64::MIN_POSITIVE {
        *v /= s;
        *d /= s;
        *v0 = beta;
    } else {
        *v *= f64::EPSILON / s;
        *v /= f64::EPSILON;
        *d *= f64::EPSILON / s;
        *d /= f64::EPSILON;
        *v0 = beta;
    }

    tau
}

// Placeholder for qr_uu_lssolve which would need to be implemented separately
fn qr_uu_lssolve(
    _r: &Array2<f64>,
    _y: &Array2<f64>,
    _t: &Array2<f64>,
    _b: &Array1<f64>,
    _x: &mut Array1<f64>,
    _work: &mut Array1<f64>,
) -> Result<(), LinalgError> {
    unimplemented!()
}