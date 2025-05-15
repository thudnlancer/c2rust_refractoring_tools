use ndarray::{Array1, Array2, Axis, s};
use ndarray_linalg::{Norm, Solve, Scalar};
use num_traits::{Float, Signed, Zero, One};
use std::cmp::Ordering;

/// Estimate the 1-norm of ||A^{-1}|| for a square matrix A
pub fn invnorm1<F: Float + Signed + Scalar>(
    n: usize,
    ainvx: impl Fn(Transpose, &mut Array1<F>, &Array2<F>) -> Result<(), String>,
    a: &Array2<F>,
    work: &mut Array1<F>,
) -> Result<F, String> {
    if work.len() != 3 * n {
        return Err("work vector must have length 3*N".to_string());
    }

    let maxit = 5;
    let (x, rest) = work.view_mut().split_at(Axis(0), n);
    let (v, xi) = rest.split_at(Axis(0), n);

    // Initialize x with 1/N
    x.fill(F::one() / F::from(n).unwrap());

    // v = A^{-1} x
    v.assign(&x);
    ainvx(Transpose::NoTrans, v, a)?;

    // gamma = ||v||_1
    let mut gamma = v.norm_l1();

    // xi = sign(v)
    for (xi_i, &v_i) in xi.iter_mut().zip(v.iter()) {
        *xi_i = v_i.signum();
    }

    // x = A^{-t} xi
    x.assign(&xi);
    ainvx(Transpose::Trans, x, a)?;

    for _ in 0..maxit {
        // Find index of maximum absolute value in x
        let j = x.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap_or(Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap();

        // v := A^{-1} e_j
        v.fill(F::zero());
        v[j] = F::one();
        ainvx(Transpose::NoTrans, v, a)?;

        let gamma_old = gamma;
        gamma = v.norm_l1();

        // Check for convergence
        if same_sign(v, xi) || gamma < gamma_old {
            break;
        }

        // xi = sign(v)
        for (xi_i, &v_i) in xi.iter_mut().zip(v.iter()) {
            *xi_i = v_i.signum();
        }

        // x = A^{-t} xi
        x.assign(&xi);
        ainvx(Transpose::Trans, x, a)?;
    }

    // Alternate estimate
    let mut temp = F::one();
    for (i, x_i) in x.iter_mut().enumerate() {
        let term = F::one() + F::from(i).unwrap() / F::from(n - 1).unwrap();
        *x_i = temp * term;
        temp = -temp;
    }

    // x := A^{-1} x
    ainvx(Transpose::NoTrans, x, a)?;

    let temp_est = F::from(2).unwrap() * x.norm_l1() / (F::from(3).unwrap() * F::from(n).unwrap());
    if temp_est > gamma {
        gamma = temp_est;
    }

    Ok(gamma)
}

/// Compute reciprocal condition number for triangular matrix
pub fn tri_rcond<F: Float + Signed + Scalar>(
    uplo: Uplo,
    a: &Array2<F>,
    work: &mut Array1<F>,
) -> Result<F, String> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err("matrix must be square".to_string());
    }
    if work.len() != 3 * n {
        return Err("work vector must have length 3*N".to_string());
    }

    let anorm = tri_norm1(uplo, a);
    if anorm.is_zero() {
        return Ok(F::zero());
    }

    let ainvnorm = invnorm1(
        n,
        match uplo {
            Uplo::Upper => invtriu,
            Uplo::Lower => invtril,
        },
        a,
        work,
    )?;

    if ainvnorm.is_zero() {
        Ok(F::zero())
    } else {
        Ok((F::one() / anorm) / ainvnorm)
    }
}

/// Calculate 1-norm of triangular matrix
fn tri_norm1<F: Float>(uplo: Uplo, a: &Array2<F>) -> F {
    let n = a.ncols();
    let mut max = F::zero();

    match uplo {
        Uplo::Upper => {
            for j in 0..n {
                let mut sum = F::zero();
                for i in 0..=j {
                    sum = sum + a[(i, j)].abs();
                }
                max = max.max(sum);
            }
        }
        Uplo::Lower => {
            for j in 0..n {
                let mut sum = F::zero();
                for i in j..n {
                    sum = sum + a[(i, j)].abs();
                }
                max = max.max(sum);
            }
        }
    }

    max
}

/// Check if two vectors have same signs
fn same_sign<F: Float + Signed>(x: &Array1<F>, y: &Array1<F>) -> bool {
    x.iter()
        .zip(y.iter())
        .all(|(&xi, &yi)| xi.signum() == yi.signum())
}

/// Solve upper triangular system
fn invtriu<F: Float + Scalar>(
    trans: Transpose,
    x: &mut Array1<F>,
    a: &Array2<F>,
) -> Result<(), String> {
    a.solve_triangular_inplace(Uplo::Upper, trans, x)
        .map_err(|e| e.to_string())
}

/// Solve lower triangular system
fn invtril<F: Float + Scalar>(
    trans: Transpose,
    x: &mut Array1<F>,
    a: &Array2<F>,
) -> Result<(), String> {
    a.solve_triangular_inplace(Uplo::Lower, trans, x)
        .map_err(|e| e.to_string())
}

/// Matrix uplo type
#[derive(Copy, Clone)]
pub enum Uplo {
    Upper,
    Lower,
}

/// Transpose operation
#[derive(Copy, Clone)]
pub enum Transpose {
    NoTrans,
    Trans,
}