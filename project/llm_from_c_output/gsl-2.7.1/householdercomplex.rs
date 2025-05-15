use num_complex::Complex64;
use std::f64::consts;
use std::ops::{Add, Mul, Neg, Sub};
use ndarray::{Array1, Array2, ArrayView1, ArrayViewMut1, ArrayViewMut2, Axis};
use ndarray_linalg::{Norm, Scalar, Dot};

const GSL_SUCCESS: i32 = 0;
const GSL_EBADLEN: i32 = 1;

/// Computes a householder transformation matrix H such that
/// H' v = -/+ |v| e_1
/// where e_1 is the first unit vector.
pub fn complex_householder_transform(v: &mut Array1<Complex64>) -> Complex64 {
    let n = v.len();
    
    if n == 1 {
        let alpha = v[0];
        let absa = alpha.norm();
        let beta_r = -if alpha.re >= 0.0 { 1.0 } else { -1.0 } * absa;

        if beta_r == 0.0 {
            Complex64::new(0.0, 0.0)
        } else {
            let tau = Complex64::new(
                (beta_r - alpha.re) / beta_r,
                -alpha.im / beta_r,
            );
            v[0] = Complex64::new(beta_r, 0.0);
            tau
        }
    } else {
        let alpha = v[0];
        let absa = alpha.norm();
        let xnorm = v.slice(s![1..]).norm();
        
        if xnorm == 0.0 && alpha.im == 0.0 {
            return Complex64::new(0.0, 0.0);
        }
        
        let beta_r = -if alpha.re >= 0.0 { 1.0 } else { -1.0 } * (absa.hypot(xnorm));
        let tau = Complex64::new(
            (beta_r - alpha.re) / beta_r,
            -alpha.im / beta_r,
        );

        let amb = alpha - beta_r;
        let s = Complex64::new(1.0, 0.0) / amb;
        v.slice_mut(s![1..]).mapv_inplace(|x| x * s);

        v[0] = Complex64::new(beta_r, 0.0);
        tau
    }
}

/// Applies a Householder transformation to a vector
pub fn complex_householder_hv(
    tau: Complex64,
    v: &Array1<Complex64>,
    w: &mut Array1<Complex64>,
) -> Result<(), i32> {
    let n = v.len();

    if tau.re == 0.0 && tau.im == 0.0 {
        return Ok(());
    }

    if n == 1 {
        let w0 = w[0];
        let a = Complex64::new(1.0 - tau.re, -tau.im);
        let b = a * w0;
        w[0] = b;
    } else {
        let z0 = w[0];
        let v1 = v.slice(s![1..]);
        let mut w1 = w.slice_mut(s![1..]);

        let z1 = v1.dot(&w1);
        let z = z0 + z1;

        let tz = tau * z;
        let ntz = -tz;

        w[0] = w[0] + ntz;
        w1.scaled_add(ntz, &v1);
    }

    Ok(())
}

/// Applies a Householder transformation from the left to a matrix
pub fn complex_householder_left(
    tau: Complex64,
    v: &mut Array1<Complex64>,
    a: &mut Array2<Complex64>,
    work: &mut Array1<Complex64>,
) -> Result<(), i32> {
    let m = a.nrows();
    let n = a.ncols();

    if v.len() != m {
        return Err(GSL_EBADLEN);
    } else if work.len() != n {
        return Err(GSL_EBADLEN);
    }

    if tau.re == 0.0 && tau.im == 0.0 {
        return Ok(());
    }

    let v0 = v[0];
    v[0] = Complex64::new(1.0, 0.0);

    // work = A^H v
    work.fill(Complex64::new(0.0, 0.0));
    for j in 0..n {
        for i in 0..m {
            work[j] += a[(i, j)].conj() * v[i];
        }
    }

    // A = A - tau v work^H
    let mtau = -tau;
    for i in 0..m {
        for j in 0..n {
            a[(i, j)] += mtau * v[i] * work[j].conj();
        }
    }

    v[0] = v0;
    Ok(())
}

/// Applies a Householder transformation to a matrix (deprecated)
#[allow(deprecated)]
pub fn complex_householder_hm(
    tau: Complex64,
    v: &Array1<Complex64>,
    a: &mut Array2<Complex64>,
) -> Result<(), i32> {
    let m = a.nrows();
    let n = a.ncols();

    if tau.re == 0.0 && tau.im == 0.0 {
        return Ok(());
    }

    for j in 0..n {
        let mut wj = a[(0, j)];
        for i in 1..m {
            wj += a[(i, j)] * v[i].conj();
        }

        let tauwj = tau * wj;

        a[(0, j)] -= tauwj;
        for i in 1..m {
            a[(i, j)] -= v[i] * tauwj;
        }
    }

    Ok(())
}

/// Applies a Householder transformation to a matrix on the right (deprecated)
#[allow(deprecated)]
pub fn complex_householder_mh(
    tau: Complex64,
    v: &Array1<Complex64>,
    a: &mut Array2<Complex64>,
) -> Result<(), i32> {
    let m = a.nrows();
    let n = a.ncols();

    if tau.re == 0.0 && tau.im == 0.0 {
        return Ok(());
    }

    for i in 0..m {
        let mut wi = a[(i, 0)];
        for j in 1..n {
            wi += a[(i, j)] * v[j];
        }

        let tauwi = tau * wi;

        a[(i, 0)] -= tauwi;
        for j in 1..n {
            a[(i, j)] -= tauwi * v[j].conj();
        }
    }

    Ok(())
}