use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis, s};
use ndarray_linalg::{cholesky::*, solve::*, Inverse, Norm};
use ndarray_rand::rand_distr::num_traits::Zero;
use num_traits::{Float, One};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CholeskyError {
    #[error("matrix must be square")]
    NotSquare,
    #[error("dimensions do not match")]
    BadLength,
    #[error("matrix is singular")]
    Singular,
}

pub struct PCholeskyParams<'a> {
    ldl: &'a Array2<f64>,
    perm: &'a Array1<usize>,
}

pub fn pcholesky_decomp(
    copy_uplo: bool,
    a: &mut Array2<f64>,
    p: &mut Array1<usize>,
) -> Result<(), CholeskyError> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(CholeskyError::NotSquare);
    }
    if p.len() != n {
        return Err(CholeskyError::BadLength);
    }

    if copy_uplo {
        // Save a copy of A in upper triangle
        for i in 0..n {
            for j in i+1..n {
                a[[i, j]] = a[[j, i]];
            }
        }
    }

    // Initialize permutation
    for i in 0..n {
        p[i] = i;
    }

    for k in 0..n {
        // Find pivot
        let mut max_idx = k;
        let mut max_val = a[[k, k]].abs();
        for j in k+1..n {
            let val = a[[j, j]].abs();
            if val > max_val {
                max_val = val;
                max_idx = j;
            }
        }

        // Swap rows and columns
        if max_idx != k {
            p.swap(k, max_idx);
            a.swap_rows(k, max_idx);
            a.swap_columns(k, max_idx);
        }

        if k < n - 1 {
            let alpha = a[[k, k]];
            if alpha == 0.0 {
                return Err(CholeskyError::Singular);
            }
            let alphainv = 1.0 / alpha;

            // v = A(k+1:n, k)
            let mut v = a.slice_mut(s![k+1.., k]);

            // m = A(k+1:n, k+1:n)
            let mut m = a.slice_mut(s![k+1.., k+1..]);

            // m = m - v v^T / alpha
            for i in 0..m.nrows() {
                for j in 0..=i {
                    m[[i, j]] -= v[i] * v[j] * alphainv;
                }
            }

            // v = v / alpha
            v *= alphainv;
        }
    }

    Ok(())
}

pub fn pcholesky_solve(
    ldl: &Array2<f64>,
    p: &Array1<usize>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
) -> Result<(), CholeskyError> {
    let n = ldl.nrows();
    if n != ldl.ncols() {
        return Err(CholeskyError::NotSquare);
    }
    if p.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if b.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if x.len() != n {
        return Err(CholeskyError::BadLength);
    }

    x.assign(b);
    pcholesky_svx(ldl, p, x)
}

pub fn pcholesky_svx(
    ldl: &Array2<f64>,
    p: &Array1<usize>,
    x: &mut Array1<f64>,
) -> Result<(), CholeskyError> {
    let n = ldl.nrows();
    if n != ldl.ncols() {
        return Err(CholeskyError::NotSquare);
    }
    if p.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if x.len() != n {
        return Err(CholeskyError::BadLength);
    }

    // x := P b
    permute_vector(p, x);

    // Solve L w = P b
    let l = ldl.triangular_lower().unwrap();
    solve_triangular(&l, x, false)?;

    // Solve D y = w
    let d = ldl.diag();
    x.zip_mut_with(&d, |xi, &di| *xi /= di);

    // Solve L^T z = y
    solve_triangular(&l.t(), x, true)?;

    // x = P^T z
    permute_vector_inverse(p, x);

    Ok(())
}

pub fn pcholesky_decomp2(
    a: &mut Array2<f64>,
    p: &mut Array1<usize>,
    s: &mut Array1<f64>,
) -> Result<(), CholeskyError> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(CholeskyError::NotSquare);
    }
    if p.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if s.len() != n {
        return Err(CholeskyError::BadLength);
    }

    // Save a copy of A in upper triangle
    for i in 0..n {
        for j in i+1..n {
            a[[i, j]] = a[[j, i]];
        }
    }

    // Compute scaling factors
    for i in 0..n {
        s[i] = 1.0 / a[[i, i]].sqrt();
    }

    // Apply scaling
    for i in 0..n {
        for j in 0..=i {
            a[[i, j]] *= s[i] * s[j];
        }
    }

    // Compute Cholesky decomposition
    pcholesky_decomp(false, a, p)
}

pub fn pcholesky_solve2(
    ldl: &Array2<f64>,
    p: &Array1<usize>,
    s: &Array1<f64>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
) -> Result<(), CholeskyError> {
    let n = ldl.nrows();
    if n != ldl.ncols() {
        return Err(CholeskyError::NotSquare);
    }
    if p.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if s.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if b.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if x.len() != n {
        return Err(CholeskyError::BadLength);
    }

    x.assign(b);
    x *= s;
    pcholesky_svx(ldl, p, x)?;
    x *= s;
    Ok(())
}

pub fn pcholesky_invert(
    ldl: &Array2<f64>,
    p: &Array1<usize>,
    ainv: &mut Array2<f64>,
) -> Result<(), CholeskyError> {
    let n = ldl.nrows();
    if n != ldl.ncols() {
        return Err(CholeskyError::NotSquare);
    }
    if p.len() != n {
        return Err(CholeskyError::BadLength);
    }
    if ainv.nrows() != n || ainv.ncols() != n {
        return Err(CholeskyError::BadLength);
    }

    // Invert lower triangle
    let mut l_inv = ldl.triangular_lower().unwrap().inv()?;

    // Compute sqrt(D^{-1}) L^{-1}
    for i in 0..n {
        let di = ldl[[i, i]];
        let invsqrt_di = 1.0 / di.sqrt();
        
        if i > 0 {
            l_inv.slice_mut(s![i, ..i]) *= invsqrt_di;
        }
        
        l_inv[[i, i]] = invsqrt_di;
    }

    // Compute A^{-1} = L^{-T} D^{-1} L^{-1}
    *ainv = l_inv.t().dot(&l_inv);

    // Copy lower triangle to upper
    for i in 0..n {
        for j in i+1..n {
            ainv[[i, j]] = ainv[[j, i]];
        }
    }

    // Apply permutation
    for i in 0..n {
        permute_vector_inverse(p, &mut ainv.row_mut(i));
    }
    for i in 0..n {
        permute_vector_inverse(p, &mut ainv.column_mut(i));
    }

    Ok(())
}

fn permute_vector(p: &Array1<usize>, x: &mut Array1<f64>) {
    let mut temp = x.to_owned();
    for (i, &pi) in p.iter().enumerate() {
        x[i] = temp[pi];
    }
}

fn permute_vector_inverse(p: &Array1<usize>, x: &mut Array1<f64>) {
    let mut temp = x.to_owned();
    for (i, &pi) in p.iter().enumerate() {
        temp[pi] = x[i];
    }
    x.assign(&temp);
}

fn solve_triangular(
    l: &Array2<f64>,
    x: &mut Array1<f64>,
    transpose: bool,
) -> Result<(), CholeskyError> {
    // Forward or backward substitution depending on transpose
    let n = l.nrows();
    if transpose {
        for i in (0..n).rev() {
            let mut sum = 0.0;
            for j in i+1..n {
                sum += l[[j, i]] * x[j];
            }
            x[i] = (x[i] - sum) / l[[i, i]];
        }
    } else {
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..i {
                sum += l[[i, j]] * x[j];
            }
            x[i] -= sum;
        }
    }
    Ok(())
}