use ndarray::{Array2, Array1, ArrayViewMut1, ArrayViewMut2, Axis};
use ndarray_linalg::{Householder, Norm};

pub enum HessenbergError {
    NotSquare,
    BadLength,
}

/// Compute the Householder reduction to Hessenberg form of a square N-by-N matrix A.
/// H = U^t A U
///
/// # Arguments
/// * `a` - Matrix to reduce (mutated in place)
/// * `tau` - Vector to store scalar factors in Householder matrices (must be length N)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(HessenbergError)` on error
pub fn hessenberg_decomp(a: &mut Array2<f64>, tau: &mut Array1<f64>) -> Result<(), HessenbergError> {
    let n = a.nrows();
    
    if n != a.ncols() {
        return Err(HessenbergError::NotSquare);
    } else if n != tau.len() {
        return Err(HessenbergError::BadLength);
    } else if n < 3 {
        return Ok(());
    }

    for i in 0..n-2 {
        // Get column i from row i+1 to end
        let mut col = a.slice_mut(s![i+1.., i]);
        
        // Store column in tau vector
        tau.slice_mut(s![i+1..n]).assign(&col);
        
        // Compute Householder transformation
        let tau_i = householder_transform(&mut col);
        
        // Apply left Householder matrix to A[i+1:, i:]
        {
            let mut m = a.slice_mut(s![i+1.., i..]);
            apply_householder_left(tau_i, &col, &mut m);
        }
        
        // Apply right Householder matrix to A[0:, i+1:]
        {
            let mut m = a.slice_mut(s![.., i+1..]);
            apply_householder_right(tau_i, &col, &mut m);
        }
        
        // Save Householder coefficient
        tau[i] = tau_i;
        
        // Store Householder vector below subdiagonal
        if col.len() > 1 {
            a.slice_mut(s![i+2.., i]).assign(&col.slice(s![1..]));
        }
    }

    Ok(())
}

/// Construct the matrix U which transforms a matrix A into its upper Hessenberg form
/// H = U^t A U by unpacking the information stored in H from hessenberg_decomp.
pub fn hessenberg_unpack(
    h: &Array2<f64>,
    tau: &Array1<f64>,
    u: &mut Array2<f64>,
) -> Result<(), HessenbergError> {
    u.fill(0.0);
    for i in 0..u.nrows() {
        u[(i, i)] = 1.0;
    }
    hessenberg_unpack_accum(h, tau, u)
}

/// Accumulate Householder transformations into matrix V
pub fn hessenberg_unpack_accum(
    h: &Array2<f64>,
    tau: &Array1<f64>,
    v: &mut Array2<f64>,
) -> Result<(), HessenbergError> {
    let n = h.nrows();
    
    if n != h.ncols() {
        return Err(HessenbergError::NotSquare);
    } else if n != tau.len() {
        return Err(HessenbergError::BadLength);
    } else if n != v.ncols() {
        return Err(HessenbergError::BadLength);
    } else if n < 3 {
        return Ok(());
    }

    for j in 0..n-2 {
        let tau_j = tau[j];
        let hv = h.slice(s![j+1.., j]);
        
        // Apply right Householder matrix to V
        let mut m = v.slice_mut(s![.., j+1..]);
        apply_householder_right(tau_j, &hv, &mut m);
    }

    Ok(())
}

/// Zero out the lower triangular portion of the Hessenberg matrix H
pub fn hessenberg_set_zero(h: &mut Array2<f64>) -> Result<(), HessenbergError> {
    let n = h.nrows();
    
    if n != h.ncols() {
        return Err(HessenbergError::NotSquare);
    } else if n < 3 {
        return Ok(());
    }

    for j in 0..n-2 {
        for i in j+2..n {
            h[(i, j)] = 0.0;
        }
    }

    Ok(())
}

/// Perform Householder reduction on a submatrix of a larger matrix
pub fn hessenberg_submatrix(
    m: &mut Array2<f64>,
    a: &mut Array2<f64>,
    top: usize,
    tau: &mut Array1<f64>,
) -> Result<(), HessenbergError> {
    let n = a.nrows();
    let n_m = m.nrows();
    
    if n != a.ncols() {
        return Err(HessenbergError::NotSquare);
    } else if n != tau.len() {
        return Err(HessenbergError::BadLength);
    } else if n < 3 {
        return Ok(());
    }

    for i in 0..n-2 {
        // Get column i from row i+1 to end
        let mut col = a.slice_mut(s![i+1.., i]);
        
        // Store column in tau vector
        tau.slice_mut(s![i+1..n]).assign(&col);
        
        // Compute Householder transformation
        let tau_i = householder_transform(&mut col);
        
        // Apply left Householder matrix to M[top+i+1:, top+i:]
        {
            let mut sub_m = m.slice_mut(s![top+i+1.., top+i..]);
            apply_householder_left(tau_i, &col, &mut sub_m);
        }
        
        // Apply right Householder matrix to M[.., top+i+1:]
        {
            let mut sub_m = m.slice_mut(s![.., top+i+1..top+n]);
            apply_householder_right(tau_i, &col, &mut sub_m);
        }
        
        // Save Householder coefficient
        tau[i] = tau_i;
        
        // Store Householder vector below subdiagonal
        if col.len() > 1 {
            a.slice_mut(s![i+2.., i]).assign(&col.slice(s![1..]));
        }
    }

    Ok(())
}

// Helper functions

fn householder_transform(v: &mut ArrayViewMut1<f64>) -> f64 {
    let norm = v.norm();
    if norm == 0.0 {
        return 0.0;
    }
    
    let beta = if v[0] > 0.0 {
        -norm
    } else {
        norm
    };
    
    let scale = 1.0 / (v[0] - beta);
    v[0] = beta;
    *v *= scale;
    
    beta
}

fn apply_householder_left(tau: f64, v: &ArrayViewMut1<f64>, m: &mut ArrayViewMut2<f64>) {
    let n = v.len();
    if n == 0 {
        return;
    }
    
    let w = tau * v.dot(&m.t().dot(v));
    for j in 0..m.ncols() {
        let mut col = m.column_mut(j);
        col -= &(w * v);
    }
}

fn apply_householder_right(tau: f64, v: &ArrayViewMut1<f64>, m: &mut ArrayViewMut2<f64>) {
    let n = v.len();
    if n == 0 {
        return;
    }
    
    let w = tau * m.dot(v);
    for i in 0..m.nrows() {
        let mut row = m.row_mut(i);
        row -= &(w * v);
    }
}