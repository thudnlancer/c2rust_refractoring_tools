use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, ArrayViewMut2, Axis};
use ndarray_linalg::{Householder, Norm};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BidiagError {
    #[error("matrix dimensions must satisfy M >= N")]
    BadMatrixDimensions,
    #[error("tau_U size must be N")]
    BadTauUSize,
    #[error("tau_V size must be N - 1")]
    BadTauVSize,
    #[error("U matrix dimensions must be M x N")]
    BadUDimensions,
    #[error("V matrix dimensions must be N x N")]
    BadVDimensions,
    #[error("diagonal size must match matrix size")]
    BadDiagonalSize,
    #[error("superdiagonal size must be (diagonal size - 1)")]
    BadSuperdiagonalSize,
}

pub fn bidiag_decomp(
    a: &mut Array2<f64>,
    tau_u: &mut Array1<f64>,
    tau_v: &mut Array1<f64>,
) -> Result<(), BidiagError> {
    let (m, n) = a.dim();
    
    if m < n {
        return Err(BidiagError::BadMatrixDimensions);
    } else if tau_u.len() != n {
        return Err(BidiagError::BadTauUSize);
    } else if tau_v.len() + 1 != n {
        return Err(BidiagError::BadTauVSize);
    }

    let mut tmp = Array1::zeros(m);

    for j in 0..n {
        // Apply Householder transformation to current column
        let mut v = a.slice_mut(s![j.., j]);
        let tau_j = householder_transform(&mut v);
        
        // Apply the transformation to the remaining columns
        if j + 1 < n {
            let mut m = a.slice_mut(s![j.., j+1..]);
            let mut work = tau_u.slice_mut(s![j..]);
            let tmp_val = v[0];
            v[0] = 1.0;
            
            householder_left(tau_j, &v.view(), &mut m, &mut work);
            v[0] = tmp_val;
        }

        tau_u[j] = tau_j;

        // Apply Householder transformation to current row
        if j + 1 < n {
            let mut v = a.slice_mut(s![j, j+1..]);
            let tau_j = householder_transform(&mut v);
            
            // Apply the transformation to the remaining rows
            if j + 1 < m {
                let mut m = a.slice_mut(s![j+1.., j+1..]);
                let mut work = tmp.slice_mut(s![..m-j-1]);
                householder_right(tau_j, &v.view(), &mut m, &mut work);
            }

            tau_v[j] = tau_j;
        }
    }

    Ok(())
}

pub fn bidiag_unpack(
    a: &Array2<f64>,
    tau_u: &Array1<f64>,
    tau_v: &Array1<f64>,
    u: &mut Array2<f64>,
    v: &mut Array2<f64>,
    diag: &mut Array1<f64>,
    superdiag: &mut Array1<f64>,
) -> Result<(), BidiagError> {
    let (m, n) = a.dim();
    let k = m.min(n);

    if m < n {
        return Err(BidiagError::BadMatrixDimensions);
    } else if tau_u.len() != k {
        return Err(BidiagError::BadTauUSize);
    } else if tau_v.len() + 1 != k {
        return Err(BidiagError::BadTauVSize);
    } else if u.dim() != (m, n) {
        return Err(BidiagError::BadUDimensions);
    } else if v.dim() != (n, n) {
        return Err(BidiagError::BadVDimensions);
    } else if diag.len() != k {
        return Err(BidiagError::BadDiagonalSize);
    } else if superdiag.len() + 1 != k {
        return Err(BidiagError::BadSuperdiagonalSize);
    }

    // Copy diagonal into diag
    for i in 0..n {
        diag[i] = a[(i, i)];
    }

    // Copy superdiagonal into superdiag
    for i in 0..n - 1 {
        superdiag[i] = a[(i, i + 1)];
    }

    // Initialize V to identity
    v.fill(0.0);
    for i in 0..n {
        v[(i, i)] = 1.0;
    }

    for i in (0..n - 1).rev() {
        let h = a.slice(s![i, i+1..]);
        let ti = tau_v[i];
        let mut m = v.slice_mut(s![i+1.., i+1..]);
        let mut work = u.slice_mut(s![0, ..n-i-1]);
        
        let mut h_copy = h.to_owned();
        let tmp = h_copy[0];
        h_copy[0] = 1.0;
        
        householder_left(ti, &h_copy.view(), &mut m, &mut work);
        h_copy[0] = tmp;
    }

    // Initialize U to identity
    u.fill(0.0);
    for i in 0..m {
        u[(i, i)] = 1.0;
    }

    for j in (0..n).rev() {
        let h = a.slice(s![j.., j]);
        let tj = tau_u[j];
        let mut m = u.slice_mut(s![j.., j..]);
        
        householder_hm(tj, &h.view(), &mut m);
    }

    Ok(())
}

pub fn bidiag_unpack2(
    a: &mut Array2<f64>,
    tau_u: &mut Array1<f64>,
    tau_v: &mut Array1<f64>,
    v: &mut Array2<f64>,
) -> Result<(), BidiagError> {
    let (m, n) = a.dim();
    let k = m.min(n);

    if m < n {
        return Err(BidiagError::BadMatrixDimensions);
    } else if tau_u.len() != k {
        return Err(BidiagError::BadTauUSize);
    } else if tau_v.len() + 1 != k {
        return Err(BidiagError::BadTauVSize);
    } else if v.dim() != (n, n) {
        return Err(BidiagError::BadVDimensions);
    }

    // Initialize V to identity
    v.fill(0.0);
    for i in 0..n {
        v[(i, i)] = 1.0;
    }

    for i in (0..n - 1).rev() {
        let r = a.slice(s![i, ..]);
        let h = r.slice(s![i+1..]);
        let ti = tau_v[i];
        let mut m = v.slice_mut(s![i+1.., i+1..]);
        
        householder_hm(ti, &h.view(), &mut m);
    }

    // Copy superdiagonal into tau_v
    for i in 0..n - 1 {
        tau_v[i] = a[(i, i + 1)];
    }

    // Allow U to be unpacked into the same memory as A
    for j in (0..n).rev() {
        let tj = tau_u[j];
        let ajj = a[(j, j)];
        let mut m = a.slice_mut(s![j.., j..]);
        
        tau_u[j] = ajj;
        householder_hm1(tj, &mut m);
    }

    Ok(())
}

pub fn bidiag_unpack_b(
    a: &Array2<f64>,
    diag: &mut Array1<f64>,
    superdiag: &mut Array1<f64>,
) -> Result<(), BidiagError> {
    let (m, n) = a.dim();
    let k = m.min(n);

    if diag.len() != k {
        return Err(BidiagError::BadDiagonalSize);
    } else if superdiag.len() + 1 != k {
        return Err(BidiagError::BadSuperdiagonalSize);
    }

    // Copy diagonal into diag
    for i in 0..k {
        diag[i] = a[(i, i)];
    }

    // Copy superdiagonal into superdiag
    for i in 0..k - 1 {
        superdiag[i] = a[(i, i + 1)];
    }

    Ok(())
}

// Helper functions
fn householder_transform(v: &mut ArrayViewMut1<f64>) -> f64 {
    let norm = v.norm();
    if norm == 0.0 {
        return 0.0;
    }
    
    let alpha = -v[0].signum() * norm;
    let beta = 1.0 / (v[0] - alpha);
    v[0] = alpha;
    
    for i in 1..v.len() {
        v[i] *= beta;
    }
    
    1.0 / beta
}

fn householder_left(
    tau: f64,
    v: &ArrayView1<f64>,
    a: &mut ArrayViewMut2<f64>,
    work: &mut ArrayViewMut1<f64>,
) {
    let n = a.shape()[1];
    
    for j in 0..n {
        let mut aj = a.slice_mut(s![.., j]);
        let dot = v.dot(&aj);
        work[j] = tau * dot;
    }
    
    for j in 0..n {
        let mut aj = a.slice_mut(s![.., j]);
        let wj = work[j];
        azip!((a in &mut aj, &v in v) *a -= v * wj);
    }
}

fn householder_right(
    tau: f64,
    v: &ArrayView1<f64>,
    a: &mut ArrayViewMut2<f64>,
    work: &mut ArrayViewMut1<f64>,
) {
    let m = a.shape()[0];
    
    for i in 0..m {
        let mut ai = a.slice_mut(s![i, ..]);
        let dot = v.dot(&ai);
        work[i] = tau * dot;
    }
    
    for i in 0..m {
        let mut ai = a.slice_mut(s![i, ..]);
        let wi = work[i];
        azip!((a in &mut ai, &v in v) *a -= v * wi);
    }
}

fn householder_hm(tau: f64, v: &ArrayView1<f64>, a: &mut ArrayViewMut2<f64>) {
    let mut work = Array1::zeros(a.shape()[1]);
    householder_left(tau, v, a, &mut work.view_mut());
}

fn householder_hm1(tau: f64, a: &mut ArrayViewMut2<f64>) {
    let v = a.slice(s![.., 0]);
    let mut work = Array1::zeros(a.shape()[1]);
    householder_left(tau, &v.view(), a, &mut work.view_mut());
}