use ndarray::{Array2, Array1, ArrayView1, ArrayView2, ArrayViewMut1, ArrayViewMut2};
use ndarray_linalg::{Householder, Solve, Scalar};
use num_traits::Float;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QRDecompError {
    #[error("M must be >= N")]
    BadDimension,
    #[error("T matrix must be square")]
    NotSquare,
    #[error("T matrix dimensions don't match A")]
    DimensionMismatch,
    #[error("Matrix size doesn't match vector size")]
    SizeMismatch,
    #[error("Workspace size mismatch")]
    WorkSizeMismatch,
}

pub fn qr_decomp_r<T: Float + Scalar>(
    a: &mut Array2<T>,
    t: &mut Array2<T>,
) -> Result<(), QRDecompError> {
    let (m, n) = a.dim();
    let (t_n, t_m) = t.dim();

    if m < n {
        return Err(QRDecompError::BadDimension);
    } else if t_n != t_m {
        return Err(QRDecompError::NotSquare);
    } else if t_n != n {
        return Err(QRDecompError::DimensionMismatch);
    }

    if n == 1 {
        // Base case: compute Householder transform for single column
        let mut v = a.column_mut(0);
        let tau = Householder::transform(&mut v);
        t[(0, 0)] = tau;
        Ok(())
    } else {
        let n1 = n / 2;
        let n2 = n - n1;
        let m2 = m - n1;

        let mut a11 = a.slice_mut(s![..n1, ..n1]);
        let mut a12 = a.slice_mut(s![..n1, n1..]);
        let mut a21 = a.slice_mut(s![n1.., ..n1]);
        let mut a22 = a.slice_mut(s![n1.., n1..]);

        let mut t11 = t.slice_mut(s![..n1, ..n1]);
        let mut t12 = t.slice_mut(s![..n1, n1..]);
        let mut t22 = t.slice_mut(s![n1.., n1..]);

        // Recursively factor [A11; A21] = Q1 [R11; 0]
        let mut a_sub = a.slice_mut(s![.., ..n1]);
        qr_decomp_r(&mut a_sub, &mut t11)?;

        // Compute [R12; A22] := Q1^T [A12; A22]
        let mut w = t12.to_owned();
        w.assign(&a12);
        ndarray_linalg::trmm(1.0, &a11.t(), 'L', 'L', 'T', 'U', &mut w);
        ndarray_linalg::gemm(1.0, &a21.t(), &a22, 1.0, &mut w);
        ndarray_linalg::trmm(1.0, &t11.t(), 'L', 'U', 'T', 'N', &mut w);
        ndarray_linalg::gemm(-1.0, &a21, &w, 1.0, &mut a22);
        ndarray_linalg::trmm(1.0, &a11, 'L', 'L', 'N', 'U', &mut w);
        a12 -= &w;

        // Recursively factor A22 = Q2~ R22
        qr_decomp_r(&mut a22, &mut t22)?;

        // Update T12 := -T11 * V1^T * V2 * T22
        let v21 = a21.slice(s![..n2, ..]);
        t12.assign(&v21.t());
        let v22 = a22.slice(s![..n2, ..]);
        ndarray_linalg::trmm(1.0, &v22, 'R', 'L', 'N', 'U', &mut t12);

        if m > n {
            let v31 = a.slice(s![n.., ..n1]);
            let v32 = a.slice(s![n.., n1..]);
            ndarray_linalg::gemm(1.0, &v31.t(), &v32, 1.0, &mut t12);
        }

        ndarray_linalg::trmm(-1.0, &t11, 'L', 'U', 'N', 'N', &mut t12);
        ndarray_linalg::trmm(1.0, &t22, 'R', 'U', 'N', 'N', &mut t12);

        Ok(())
    }
}

pub fn qr_solve_r<T: Float + Scalar>(
    qr: &Array2<T>,
    t: &Array2<T>,
    b: &Array1<T>,
    x: &mut Array1<T>,
) -> Result<(), QRDecompError> {
    let n = qr.ncols();
    if qr.nrows() != n {
        return Err(QRDecompError::NotSquare);
    } else if t.dim() != (n, n) {
        return Err(QRDecompError::DimensionMismatch);
    } else if b.len() != n {
        return Err(QRDecompError::SizeMismatch);
    } else if x.len() != n {
        return Err(QRDecompError::SizeMismatch);
    }

    // Compute x = V^T b
    x.assign(b);
    ndarray_linalg::trmv('L', 'T', 'U', qr, x);

    // x = T^T x
    ndarray_linalg::trmv('U', 'T', 'N', t, x);

    // x = V x
    ndarray_linalg::trmv('L', 'N', 'U', qr, x);

    // x = b - x
    x.zip_mut_with(b, |xi, &bi| *xi = bi - *xi);

    // Solve R x = Q^T b
    ndarray_linalg::trsv('U', 'N', 'N', qr, x)?;

    Ok(())
}

pub fn qr_lssolve_r<T: Float + Scalar>(
    qr: &Array2<T>,
    t: &Array2<T>,
    b: &Array1<T>,
    x: &mut Array1<T>,
    work: &mut Array1<T>,
) -> Result<(), QRDecompError> {
    let (m, n) = qr.dim();
    if m < n {
        return Err(QRDecompError::BadDimension);
    } else if t.dim() != (n, n) {
        return Err(QRDecompError::DimensionMismatch);
    } else if b.len() != m {
        return Err(QRDecompError::SizeMismatch);
    } else if x.len() != m {
        return Err(QRDecompError::SizeMismatch);
    } else if work.len() != n {
        return Err(QRDecompError::WorkSizeMismatch);
    }

    let r = qr.slice(s![..n, ..]);
    let mut x1 = x.slice_mut(s![..n]);

    // Compute x = Q^T b
    x.assign(b);
    qr_qtvec_r(qr, t, x, work)?;

    // Solve R x1 = Q^T b
    ndarray_linalg::trsv('U', 'N', 'N', &r, &mut x1)?;

    Ok(())
}

pub fn qr_qtvec_r<T: Float + Scalar>(
    qr: &Array2<T>,
    t: &Array2<T>,
    b: &mut Array1<T>,
    work: &mut Array1<T>,
) -> Result<(), QRDecompError> {
    let (m, n) = qr.dim();
    if m < n {
        return Err(QRDecompError::BadDimension);
    } else if t.dim() != (n, n) {
        return Err(QRDecompError::DimensionMismatch);
    } else if b.len() != m {
        return Err(QRDecompError::SizeMismatch);
    } else if work.len() != n {
        return Err(QRDecompError::WorkSizeMismatch);
    }

    let v1 = qr.slice(s![..n, ..]);
    let mut b1 = b.slice_mut(s![..n]);

    // work := V1^T b1
    work.assign(&b1);
    ndarray_linalg::trmv('L', 'T', 'U', &v1, work);

    if m > n {
        let v2 = qr.slice(s![n.., ..]);
        let b2 = b.slice(s![n..]);

        // work = work + V2^T b2
        ndarray_linalg::gemv(1.0, &v2.t(), &b2, 1.0, work);
    }

    // work = T^T work
    ndarray_linalg::trmv('U', 'T', 'N', t, work);

    if m > n {
        let v2 = qr.slice(s![n.., ..]);
        let mut b2 = b.slice_mut(s![n..]);

        // b2 = b2 - V2 work
        ndarray_linalg::gemv(-1.0, &v2, work, 1.0, &mut b2);
    }

    // b1 = b1 - V1 work
    ndarray_linalg::trmv('L', 'N', 'U', &v1, work);
    b1 -= &work;

    Ok(())
}

// Additional helper functions would follow similar patterns...