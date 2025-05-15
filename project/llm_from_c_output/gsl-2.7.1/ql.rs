use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, ArrayView2, ArrayViewMut2, Axis};
use ndarray_linalg::{Householder, QR, Solve, Determinant, Inverse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinAlgError {
    #[error("size of tau must be N")]
    BadTauSize,
    #[error("Q matrix must be M x M")]
    NotSquareQ,
    #[error("L matrix must be M x N")]
    NotSquareL,
}

/// Performs QL decomposition of a matrix A
/// 
/// # Arguments
/// 
/// * `a` - M x N matrix to decompose (will be modified)
/// * `tau` - Output vector for Householder coefficients (length N)
/// 
/// # Returns
/// 
/// * `Result<(), LinAlgError>` - Ok if successful, error otherwise
pub fn ql_decomp(a: &mut Array2<f64>, tau: &mut Array1<f64>) -> Result<(), LinAlgError> {
    let (m, n) = a.dim();
    
    if tau.len() != n {
        return Err(LinAlgError::BadTauSize);
    }

    let k = m.min(n);
    
    for i in 0..k {
        // Compute the Householder transformation
        let mut c = a.slice_mut(s![..m-i, n-i-1..n-i]);
        let alpha = c[[m-i-1, 0]];
        let (tau_j, norm) = householder_transform(&mut c.column_mut(0));
        
        // Apply the transformation to the left of A[..m-i, ..n-i-1]
        if i + 1 < n {
            let mut work = tau.slice_mut(s![..n-i-1]);
            let mut m_sub = a.slice_mut(s![..m-i, ..n-i-1]);
            
            // Temporarily set alpha to 1.0 for the transformation
            c[[m-i-1, 0]] = 1.0;
            apply_householder_left(tau_j, &c.column(0), &mut m_sub, &mut work)?;
            c[[m-i-1, 0]] = alpha;
        }
        
        tau[n-i-1] = tau_j;
    }

    Ok(())
}

/// Forms the orthogonal matrix Q and lower triangular matrix L from packed QL decomposition
/// 
/// # Arguments
/// 
/// * `ql` - Packed QL matrix from decomposition
/// * `tau` - Householder coefficients from decomposition
/// * `q` - Output M x M orthogonal matrix
/// * `l` - Output M x N lower triangular matrix
/// 
/// # Returns
/// 
/// * `Result<(), LinAlgError>` - Ok if successful, error otherwise
pub fn ql_unpack(
    ql: &Array2<f64>,
    tau: &Array1<f64>,
    q: &mut Array2<f64>,
    l: &mut Array2<f64>,
) -> Result<(), LinAlgError> {
    let (m, n) = ql.dim();
    
    if q.dim() != (m, m) {
        return Err(LinAlgError::NotSquareQ);
    } else if l.dim() != (m, n) {
        return Err(LinAlgError::NotSquareL);
    } else if tau.len() != n {
        return Err(LinAlgError::BadTauSize);
    }

    let k = m.min(n);
    
    // Initialize Q to identity
    q.fill(0.0);
    for i in 0..m {
        q[[i, i]] = 1.0;
    }

    for i in 0..k {
        let h = ql.slice(s![..m-k+i+1, n-k+i..n-k+i+1]);
        let mut q_sub = q.slice_mut(s![..m-k+i+1, ..m-k+i+1]);
        let mut work = l.slice_mut(s![..m-k+i+1, 0..1]);
        let ti = tau[n-k+i];
        
        // Temporarily set ptr to 1.0
        let mut ql_mut = ql.to_owned();
        ql_mut[[m-k+i, n-k+i]] = 1.0;
        let h_mut = ql_mut.slice(s![..m-k+i+1, n-k+i..n-k+i+1]);
        
        apply_householder_left(ti, &h_mut.column(0), &mut q_sub, &mut work)?;
    }

    // Form lower triangular matrix L
    l.fill(0.0);
    
    if m >= n {
        let src = ql.slice(s![m-n.., ..n]);
        let mut dest = l.slice_mut(s![m-n.., ..n]);
        copy_lower_triangular(&src, &mut dest);
    } else {
        let src1 = ql.slice(s![.., ..n-m]);
        let mut dest1 = l.slice_mut(s![.., ..n-m]);
        dest1.assign(&src1);
        
        let src2 = ql.slice(s![.., n-m..]);
        let mut dest2 = l.slice_mut(s![.., n-m..]);
        copy_lower_triangular(&src2, &mut dest2);
    }

    Ok(())
}

/// Helper function to perform Householder transformation
fn householder_transform(v: &mut ArrayViewMut1<f64>) -> (f64, f64) {
    let norm = v.norm_l2();
    let alpha = v[0].signum() * norm;
    let beta = -alpha;
    
    if norm == 0.0 {
        return (0.0, 0.0);
    }
    
    v[0] -= alpha;
    let tau = (alpha - beta) / alpha;
    
    (tau, norm)
}

/// Helper function to apply Householder transformation from the left
fn apply_householder_left(
    tau: f64,
    v: &ArrayView1<f64>,
    a: &mut ArrayViewMut2<f64>,
    work: &mut ArrayViewMut1<f64>,
) -> Result<(), LinAlgError> {
    let n = v.len();
    let m = a.ncols();
    
    if work.len() < m {
        return Err(LinAlgError::BadTauSize);
    }
    
    work.fill(0.0);
    
    // work = A' * v
    for j in 0..m {
        for i in 0..n {
            work[j] += a[[i, j]] * v[i];
        }
    }
    
    // A = A - tau * v * work'
    for i in 0..n {
        for j in 0..m {
            a[[i, j]] -= tau * v[i] * work[j];
        }
    }
    
    Ok(())
}

/// Helper function to copy lower triangular part of matrix
fn copy_lower_triangular(src: &ArrayView2<f64>, dest: &mut ArrayViewMut2<f64>) {
    let n = src.nrows().min(src.ncols());
    for i in 0..n {
        for j in 0..=i {
            dest[[i, j]] = src[[i, j]];
        }
    }
}