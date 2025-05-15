use ndarray::{Array2, Array1, Axis, s};
use ndarray_linalg::{Solve, Norm};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinalgError {
    #[error("matrix must be square")]
    NotSquare,
    #[error("matrix is singular")]
    Singular,
    #[error("dimension mismatch")]
    DimensionMismatch,
    #[error("work vector must have length 3*N")]
    WorkSize,
}

/// Compute the 1-norm of a symmetric matrix stored in lower triangle
fn ldlt_norm1(a: &Array2<f64>) -> f64 {
    let n = a.nrows();
    let mut max = 0.0;

    for j in 0..n {
        let mut sum = a.slice(s![j.., j]).mapv(f64::abs).sum();

        // Add symmetric elements from above diagonal
        for i in 0..j {
            sum += a[(i, j)].abs();
        }

        if sum > max {
            max = sum;
        }
    }

    max
}

/// Perform LDL^T decomposition of a symmetric positive semi-definite matrix
pub fn ldlt_decomp(a: &mut Array2<f64>) -> Result<(), LinalgError> {
    let n = a.nrows();

    if a.nrows() != a.ncols() {
        return Err(LinalgError::NotSquare);
    }

    // Check for quick return
    if n == 1 {
        return Ok(());
    }

    // Compute ||A||_1
    let anorm = ldlt_norm1(a);

    // Special case first column
    let a00 = a[(0, 0)];
    if a00 == 0.0 {
        return Err(LinalgError::Singular);
    }

    // Scale first column
    a.slice_mut(s![1.., 0]).mapv_inplace(|x| x / a00);

    // Use first subrow as temporary workspace
    let mut work = Array1::zeros(n - 1);

    for j in 1..n {
        let ajj = a[(j, j)];
        let mut dval = 0.0;

        // Compute w vector
        for i in 0..j {
            let aii = a[(i, i)];
            let aji = a[(j, i)];
            work[i] = aji * aii;
        }

        // Compute dot product
        let v = a.slice(s![j, ..j]);
        dval = v.dot(&work.slice(s![..j]));

        let ajj = ajj - dval;
        if ajj == 0.0 {
            return Err(LinalgError::Singular);
        }

        a[(j, j)] = ajj;

        if j < n - 1 {
            let ajjinv = 1.0 / ajj;
            let mut v = a.slice_mut(s![j+1.., j]);
            let m = a.slice(s![j+1.., ..j]);
            let w = work.slice(s![..j]);
            let update = -ajjinv * m.dot(&w);
            v += &update;
            v.mapv_inplace(|x| x * ajjinv);
        }
    }

    // Save ||A||_1 in upper right corner
    a[(0, n - 1)] = anorm;

    Ok(())
}

/// Solve LDL^T x = b
pub fn ldlt_solve(ldlt: &Array2<f64>, b: &Array1<f64>, x: &mut Array1<f64>) -> Result<(), LinalgError> {
    if ldlt.nrows() != ldlt.ncols() {
        return Err(LinalgError::NotSquare);
    } else if ldlt.nrows() != b.len() {
        return Err(LinalgError::DimensionMismatch);
    } else if ldlt.ncols() != x.len() {
        return Err(LinalgError::DimensionMismatch);
    }

    x.assign(b);
    ldlt_svx(ldlt, x)
}

/// Solve LDL^T x = b in place
pub fn ldlt_svx(ldlt: &Array2<f64>, x: &mut Array1<f64>) -> Result<(), LinalgError> {
    if ldlt.nrows() != ldlt.ncols() {
        return Err(LinalgError::NotSquare);
    } else if ldlt.ncols() != x.len() {
        return Err(LinalgError::DimensionMismatch);
    }

    // Forward substitution: L z = b
    let mut z = x.clone();
    for i in 0..ldlt.nrows() {
        let mut sum = 0.0;
        for j in 0..i {
            sum += ldlt[(i, j)] * z[j];
        }
        z[i] = (x[i] - sum) / 1.0; // L has unit diagonal
    }

    // Diagonal solve: D y = z
    for i in 0..ldlt.nrows() {
        z[i] /= ldlt[(i, i)];
    }

    // Back substitution: L^T x = y
    for i in (0..ldlt.nrows()).rev() {
        let mut sum = 0.0;
        for j in i+1..ldlt.nrows() {
            sum += ldlt[(j, i)] * x[j];
        }
        x[i] = z[i] - sum;
    }

    Ok(())
}

/// Estimate reciprocal condition number of LDL^T matrix
pub fn ldlt_rcond(ldlt: &Array2<f64>, work: &mut Array1<f64>) -> Result<f64, LinalgError> {
    let n = ldlt.nrows();

    if ldlt.nrows() != ldlt.ncols() {
        return Err(LinalgError::NotSquare);
    } else if work.len() != 3 * n {
        return Err(LinalgError::WorkSize);
    }

    let anorm = if n == 1 {
        ldlt[(0, 0)].abs()
    } else {
        ldlt[(0, n - 1)]
    };

    if anorm == 0.0 {
        return Ok(0.0);
    }

    // TODO: Implement invnorm1 equivalent in Rust
    // For now, we'll return a dummy value
    let ainvnorm = 1.0;
    let rcond = (1.0 / anorm) / ainvnorm;

    Ok(rcond)
}