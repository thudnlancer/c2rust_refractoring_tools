use std::cmp::{min, max};
use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, Axis};
use ndarray_linalg::Scalar;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinAlgError {
    #[error("lower bandwidth must be less than M")]
    LowerBandwidth,
    #[error("upper bandwidth must be less than N")]
    UpperBandwidth,
    #[error("matrix size inconsistent with bandwidths")]
    BadMatrixSize,
    #[error("pivot vector must have length MIN(M,N)")]
    BadPivotSize,
    #[error("matrix size must match solution size")]
    SolutionSizeMismatch,
    #[error("matrix size must match rhs size")]
    RhsSizeMismatch,
    #[error("L matrix has wrong dimensions")]
    BadLMatrixSize,
    #[error("U matrix has wrong dimensions")]
    BadUMatrixSize,
}

pub fn lu_band_decomp(
    m: usize,
    lb: usize,
    ub: usize,
    ab: &mut Array2<f64>,
    ipiv: &mut Array1<usize>,
) -> Result<(), LinAlgError> {
    let n = ab.nrows();
    let min_mn = min(m, n);

    if lb >= m {
        return Err(LinAlgError::LowerBandwidth);
    } else if ub >= n {
        return Err(LinAlgError::UpperBandwidth);
    } else if ab.ncols() != 2 * lb + ub + 1 {
        return Err(LinAlgError::BadMatrixSize);
    } else if ipiv.len() != min_mn {
        return Err(LinAlgError::BadPivotSize);
    }

    lu_band_decomp_l2(m, lb, ub, ab, ipiv)
}

pub fn lu_band_solve(
    lb: usize,
    ub: usize,
    lub: &Array2<f64>,
    piv: &Array1<usize>,
    b: &Array1<f64>,
    x: &mut Array1<f64>,
) -> Result<(), LinAlgError> {
    let n = lub.nrows();

    if n != x.len() {
        return Err(LinAlgError::SolutionSizeMismatch);
    } else if n != b.len() {
        return Err(LinAlgError::RhsSizeMismatch);
    } else if lb >= n {
        return Err(LinAlgError::LowerBandwidth);
    } else if ub >= n {
        return Err(LinAlgError::UpperBandwidth);
    } else if lub.ncols() != 2 * lb + ub + 1 {
        return Err(LinAlgError::BadMatrixSize);
    } else if piv.len() != n {
        return Err(LinAlgError::BadPivotSize);
    }

    x.assign(b);
    lu_band_svx(lb, ub, lub, piv, x)
}

pub fn lu_band_svx(
    lb: usize,
    ub: usize,
    lub: &Array2<f64>,
    piv: &Array1<usize>,
    x: &mut Array1<f64>,
) -> Result<(), LinAlgError> {
    let n = lub.nrows();

    if n != x.len() {
        return Err(LinAlgError::SolutionSizeMismatch);
    } else if lb >= n {
        return Err(LinAlgError::LowerBandwidth);
    } else if ub >= n {
        return Err(LinAlgError::UpperBandwidth);
    } else if lub.ncols() != 2 * lb + ub + 1 {
        return Err(LinAlgError::BadMatrixSize);
    } else if piv.len() != n {
        return Err(LinAlgError::BadPivotSize);
    }

    if lb > 0 {
        for j in 0..n - 1 {
            let pj = piv[j];
            let xj = x[j];
            let lm = min(lb, n - j - 1);
            
            if j != pj {
                let xl = x[pj];
                x[pj] = xj;
                x[j] = xl;
            }

            let yv = lub.slice(s![j, (ub + lb + 1)..(ub + lb + 1 + lm)]);
            let mut xv = x.slice_mut(s![(j + 1)..(j + 1 + lm)]);
            xv.scaled_add(-xj, &yv);
        }
    }

    // Solve U x = b using banded triangular solve
    // Note: This would require a banded triangular solver implementation
    // For now, we'll leave this as a placeholder
    // cblas_dtbsv(CblasColMajor, CblasUpper, CblasNoTrans, CblasNonUnit,
    //             (int) N, (int) (lb + ub), LUB->data, LUB->tda,
    //             x->data, x->stride);

    Ok(())
}

pub fn lu_band_unpack(
    m: usize,
    lb: usize,
    ub: usize,
    lub: &Array2<f64>,
    piv: &Array1<usize>,
    l: &mut Array2<f64>,
    u: &mut Array2<f64>,
) -> Result<(), LinAlgError> {
    let n = lub.nrows();
    let min_mn = min(m, n);

    if ub >= n {
        return Err(LinAlgError::UpperBandwidth);
    } else if lb >= m {
        return Err(LinAlgError::LowerBandwidth);
    } else if lub.ncols() != 2 * lb + ub + 1 {
        return Err(LinAlgError::BadMatrixSize);
    } else if piv.len() != min_mn {
        return Err(LinAlgError::BadPivotSize);
    } else if l.nrows() != m || l.ncols() != min_mn {
        return Err(LinAlgError::BadLMatrixSize);
    } else if u.nrows() != min_mn || u.ncols() != n {
        return Err(LinAlgError::BadUMatrixSize);
    }

    let ub_u = lb + ub;

    l.fill(0.0);
    u.fill(0.0);
    l.diag_mut().fill(1.0);

    if lb > 0 {
        let jstart = if m > n { min_mn } else { min_mn - 1 };
        
        for j in (0..jstart).rev() {
            let pj = piv[j];
            let lm = min(lb, m - j - 1);
            
            let xv = lub.slice(s![j, (lb + ub + 1)..(lb + ub + 1 + lm)]);
            let yv = l.slice(s![j, ..min_mn]);
            
            let mut m = l.slice_mut(s![(j + 1)..(j + 1 + lm), ..min_mn]);
            m += &xv.insert_axis(Axis(1)) * &yv.insert_axis(Axis(0));

            if j != pj {
                let mut lj = l.row_mut(j);
                let mut lpj = l.row_mut(pj);
                std::mem::swap(&mut lj, &mut lpj);
            }
        }
    }

    for j in 0..=min(ub_u, n - 1) {
        let src = lub.slice(s![..min(m, n - j), ub_u - j]);
        let mut dest = u.slice_mut(s![.., j]);
        dest.assign(&src);
    }

    Ok(())
}

fn lu_band_decomp_l2(
    m: usize,
    lb: usize,
    ub: usize,
    ab: &mut Array2<f64>,
    ipiv: &mut Array1<usize>,
) -> Result<(), LinAlgError> {
    let n = ab.nrows();
    let min_mn = min(m, n);

    if ipiv.len() != min_mn {
        return Err(LinAlgError::BadPivotSize);
    } else if lb >= m {
        return Err(LinAlgError::LowerBandwidth);
    } else if ub >= n {
        return Err(LinAlgError::UpperBandwidth);
    } else if ab.ncols() != 2 * lb + ub + 1 {
        return Err(LinAlgError::BadMatrixSize);
    }

    let ub_u = lb + ub;
    let ldab = ab.ncols();
    let mut ju = 0;

    if lb > 0 {
        let mut m = ab.slice_mut(s![.., ..lb]);
        m.fill(0.0);
    }

    for j in 0..min_mn {
        let lbj = min(lb, m - j - 1);
        let x = ab.slice(s![j, ub_u..(ub_u + lbj + 1)]);
        
        let j_pivot = x.argmax().unwrap();
        ipiv[j] = j + j_pivot;

        let pivot_val = ab[(j, ub_u + j_pivot)];
        if pivot_val != 0.0 {
            ju = max(ju, min(j + ub + j_pivot, n - 1));
        }

        if j_pivot != 0 {
            // Swap columns
            let mut x = ab.slice_mut(s![j, ub_u..(ju - j + 1 + ub_u)]);
            let mut y = ab.slice_mut(s![j, (ub_u + j_pivot)..(ju - j + 1 + ub_u + j_pivot)]);
            std::mem::swap(&mut x, &mut y);
        }

        if lbj > 0 {
            let tmp = ab[(j, ub_u)];
            let mut x = ab.slice_mut(s![j, (ub_u + 1)..(ub_u + 1 + lbj)]);
            x /= tmp;

            if ju > j {
                let mut m = ab.slice_mut(s![(j + 1)..(ju + 1), ub_u..(ub_u + lbj)]);
                let y = ab.slice(s![(j + 1)..(ju + 1), ub_u - 1]);
                
                // Perform rank-1 update
                for i in 0..m.nrows() {
                    let yi = y[i];
                    for k in 0..m.ncols() {
                        m[(i, k)] -= yi * x[k];
                    }
                }
            }
        }
    }

    Ok(())
}