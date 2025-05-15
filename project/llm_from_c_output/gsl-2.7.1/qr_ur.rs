use ndarray::{Array2, ArrayViewMut2, ArrayViewMut1, s};
use ndarray_linalg::{Scalar, Norm, Lapack};
use num_traits::{Float, Zero, One};
use std::cmp::Ordering;

/// Compute the QR decomposition of the "triangle on top of rectangle" matrix
/// [ S ] = Q [ R ]
/// [ A ]     [ 0 ]
/// where S is N-by-N upper triangular and A is M-by-N dense.
pub fn qr_ur_decomp(
    s: &mut Array2<f64>,
    a: &mut Array2<f64>,
    t: &mut Array2<f64>,
) -> Result<(), &'static str> {
    let m = a.nrows();
    let n = s.nrows();

    if n != s.ncols() {
        return Err("S matrix must be square");
    } else if n != a.ncols() {
        return Err("S and A have different number of columns");
    } else if t.nrows() != n || t.ncols() != n {
        return Err("T matrix has wrong dimensions");
    } else if n == 1 {
        // base case, compute Householder transform for single column matrix
        let t00 = t[(0, 0)];
        let s00 = s[(0, 0)];
        let mut v = a.column_mut(0);
        *t00 = qrtr_householder_transform(s00, &mut v);
        Ok(())
    } else {
        // partition matrices
        let n1 = n / 2;
        let n2 = n - n1;

        let mut s11 = s.slice_mut(s![..n1, ..n1]);
        let mut s12 = s.slice_mut(s![..n1, n1..]);
        let mut s22 = s.slice_mut(s![n1.., n1..]);

        let mut a1 = a.slice_mut(s![.., ..n1]);
        let mut a2 = a.slice_mut(s![.., n1..]);

        let mut t11 = t.slice_mut(s![..n1, ..n1]);
        let mut t12 = t.slice_mut(s![..n1, n1..]);
        let mut t22 = t.slice_mut(s![n1.., n1..]);

        // recursively factor [S11; A1]
        qr_ur_decomp(&mut s11, &mut a1, &mut t11)?;

        // compute W = T11^T (S12 + A1^T A2)
        let mut w = s12.to_owned();
        w += &a1.t().dot(&a2);
        w = t11.t().dot(&w);
        
        // update A2 and S12
        a2 -= &a1.dot(&w);
        s12 -= &w;

        // recursively factor [S22~; A2~]
        qr_ur_decomp(&mut s22, &mut a2, &mut t22)?;

        // update T12 := -T11 * A1^T * A2 * T22
        let mut t12_temp = a1.t().dot(&a2);
        t12_temp = -t11.dot(&t12_temp);
        t12_temp = t12_temp.dot(&t22);
        t12.assign(&t12_temp);

        Ok(())
    }
}

/// Optimized Householder transform for QR decomposition of [S; A] matrices
fn qrtr_householder_transform(v0: &mut f64, v: &mut ArrayViewMut1<f64>) -> f64 {
    let xnorm = v.norm_l2();
    
    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
    let tau = (beta - alpha) / beta;

    let s = alpha - beta;
    
    if s.abs() > f64::MIN_POSITIVE {
        *v /= s;
        *v0 = beta;
    } else {
        *v *= f64::EPSILON / s;
        *v /= f64::EPSILON;
        *v0 = beta;
    }

    tau
}