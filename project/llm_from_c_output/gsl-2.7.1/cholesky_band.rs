use std::cmp::{min, max};
use std::f64;

#[derive(Debug)]
pub enum LinalgError {
    BadLen,
    NotSquare,
    NotPositiveDefinite,
    SingularMatrix,
}

pub fn cholesky_band_decomp(a: &mut [Vec<f64>]) -> Result<(), LinalgError> {
    let n = a.len();
    let ndiag = if n > 0 { a[0].len() } else { 0 };

    if ndiag > n {
        return Err(LinalgError::BadLen);
    }

    let p = ndiag - 1; // lower bandwidth
    let kld = max(1, p) as usize;

    if ndiag > 1 {
        let anorm = cholesky_band_norm1(a);
        a[n - 1][p] = anorm;
    }

    for j in 0..n {
        let ajj = a[j][0];
        if ajj <= 0.0 {
            return Err(LinalgError::NotPositiveDefinite);
        }

        let ajj_sqrt = ajj.sqrt();
        a[j][0] = ajj_sqrt;

        let lenv = min(p, n - j - 1);
        if lenv > 0 {
            // Scale v
            for i in 1..=lenv {
                a[j][i] /= ajj_sqrt;
            }

            // DSYR update
            for k in 0..lenv {
                for l in k..lenv {
                    a[j + 1 + l][k] -= a[j][1 + k] * a[j][1 + l];
                }
            }
        }
    }

    Ok(())
}

pub fn cholesky_band_solve(llt: &[Vec<f64>], b: &[f64], x: &mut [f64]) -> Result<(), LinalgError> {
    if llt.len() != b.len() {
        return Err(LinalgError::BadLen);
    }
    if llt.len() != x.len() {
        return Err(LinalgError::BadLen);
    }

    x.copy_from_slice(b);
    cholesky_band_svx(llt, x)
}

pub fn cholesky_band_svx(llt: &[Vec<f64>], x: &mut [f64]) -> Result<(), LinalgError> {
    let n = llt.len();
    if n != x.len() {
        return Err(LinalgError::BadLen);
    }

    let p = llt[0].len() - 1;

    // Forward substitution: L c = b
    for i in 0..n {
        let mut sum = x[i];
        for k in max(0, i as isize - p as isize) as usize..i {
            sum -= llt[i][i - k] * x[k];
        }
        x[i] = sum / llt[i][0];
    }

    // Back substitution: L^T x = c
    for i in (0..n).rev() {
        let mut sum = x[i];
        for k in i + 1..min(n, i + p + 1) {
            sum -= llt[k][k - i] * x[k];
        }
        x[i] = sum / llt[i][0];
    }

    Ok(())
}

pub fn cholesky_band_solvem(llt: &[Vec<f64>], b: &[Vec<f64>], x: &mut [Vec<f64>]) -> Result<(), LinalgError> {
    if llt.len() != b.len() {
        return Err(LinalgError::BadLen);
    }
    if llt.len() != x.len() {
        return Err(LinalgError::BadLen);
    }
    if b.is_empty() || b[0].len() != x[0].len() {
        return Err(LinalgError::BadLen);
    }

    for (x_col, b_col) in x.iter_mut().zip(b.iter()) {
        x_col.copy_from_slice(b_col);
    }

    cholesky_band_svxm(llt, x)
}

pub fn cholesky_band_svxm(llt: &[Vec<f64>], x: &mut [Vec<f64>]) -> Result<(), LinalgError> {
    let n = llt.len();
    if n != x.len() {
        return Err(LinalgError::BadLen);
    }

    let nrhs = if n > 0 { x[0].len() } else { 0 };
    for j in 0..nrhs {
        let mut x_col: Vec<f64> = x.iter().map(|row| row[j]).collect();
        cholesky_band_svx(llt, &mut x_col)?;
        for (i, &val) in x_col.iter().enumerate() {
            x[i][j] = val;
        }
    }

    Ok(())
}

pub fn cholesky_band_invert(llt: &[Vec<f64>], ainv: &mut [Vec<f64>]) -> Result<(), LinalgError> {
    let n = llt.len();
    if n == 0 || n != ainv.len() || n != ainv[0].len() {
        return Err(LinalgError::BadLen);
    }

    cholesky_band_unpack(llt, ainv)?;

    // Standard Cholesky inversion
    for i in 0..n {
        ainv[i][i] = 1.0 / ainv[i][i];
        for j in (0..i).rev() {
            let mut sum = 0.0;
            for k in j..i {
                sum += ainv[i][k] * ainv[j][k];
            }
            ainv[j][i] = -sum / ainv[j][j];
            ainv[i][j] = ainv[j][i];
        }
    }

    Ok(())
}

pub fn cholesky_band_unpack(llt: &[Vec<f64>], l: &mut [Vec<f64>]) -> Result<(), LinalgError> {
    let n = llt.len();
    if n == 0 || n != l.len() || n != l[0].len() {
        return Err(LinalgError::BadLen);
    }

    let p = llt[0].len() - 1;

    for i in 0..n {
        for j in 0..n {
            l[i][j] = 0.0;
        }
    }

    for i in 0..=p {
        for j in 0..n - i {
            l[j + i][j] = llt[j][i];
        }
    }

    Ok(())
}

pub fn cholesky_band_rcond(llt: &[Vec<f64>], work: &mut [f64]) -> Result<f64, LinalgError> {
    let n = llt.len();
    if work.len() < 3 * n {
        return Err(LinalgError::BadLen);
    }

    let ndiag = if n > 0 { llt[0].len() } else { 0 };
    let anorm = if ndiag == 1 {
        llt.iter().map(|row| row[0]).fold(0.0, |a, b| a.max(b)).powi(2)
    } else {
        llt[n - 1][ndiag - 1]
    };

    if anorm == 0.0 {
        return Ok(0.0);
    }

    // TODO: Implement invnorm1 equivalent
    let ainvnorm = 1.0; // Placeholder

    if ainvnorm == 0.0 {
        Ok(0.0)
    } else {
        Ok((1.0 / anorm) / ainvnorm)
    }
}

pub fn cholesky_band_scale(a: &[Vec<f64>], s: &mut [f64]) -> Result<(), LinalgError> {
    let n = a.len();
    let ndiag = if n > 0 { a[0].len() } else { 0 };

    if ndiag > n {
        return Err(LinalgError::BadLen);
    }
    if n != s.len() {
        return Err(LinalgError::BadLen);
    }

    for i in 0..n {
        let aii = a[i][0];
        s[i] = if aii <= 0.0 { 1.0 } else { 1.0 / aii.sqrt() };
    }

    Ok(())
}

pub fn cholesky_band_scale_apply(a: &mut [Vec<f64>], s: &[f64]) -> Result<(), LinalgError> {
    let n = a.len();
    let ndiag = if n > 0 { a[0].len() } else { 0 };

    if ndiag > n {
        return Err(LinalgError::BadLen);
    }
    if n != s.len() {
        return Err(LinalgError::BadLen);
    }

    for j in 0..n {
        let sj = s[j];
        for i in j..min(n, j + ndiag) {
            let si = s[i];
            a[j][i - j] *= sj * si;
        }
    }

    Ok(())
}

fn cholesky_band_norm1(a: &[Vec<f64>]) -> f64 {
    let n = a.len();
    let ndiag = if n > 0 { a[0].len() } else { 0 };

    if ndiag == 1 {
        a.iter().map(|row| row[0].abs()).fold(0.0, |a, b| a.max(b))
    } else {
        let mut value = 0.0;
        for j in 0..n {
            let ncol = min(ndiag, n - j);
            let mut sum = a[j][0..ncol].iter().map(|&x| x.abs()).sum();
            
            let mut k = j;
            let mut l = 1;
            while k > 0 && l < ndiag {
                k -= 1;
                sum += a[k][l].abs();
                l += 1;
            }

            value = value.max(sum);
        }
        value
    }
}

fn cholesky_band_ainv(transa: bool, x: &mut [f64], llt: &[Vec<f64>]) -> Result<(), LinalgError> {
    // transa is unused in the original C code
    let _ = transa;
    
    cholesky_band_svx(llt, x)?;
    Ok(())
}