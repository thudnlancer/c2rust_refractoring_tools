use ndarray::{Array1, Array2, ArrayView1, ArrayViewMut1, ArrayViewMut2, Axis};
use num_traits::{Float, Signed, Zero};
use std::cmp::Ordering;

/// Compute a householder transformation (tau,v) of a vector x so that P x = [ I - tau*v*v' ] x annihilates x(1:n-1)
pub fn householder_transform<T: Float + Signed>(v: &mut Array1<T>) -> T {
    let n = v.len();

    if n == 1 {
        T::zero()
    } else {
        let x = v.slice(s![1..]);
        let xnorm = x.dot(&x).sqrt();

        if xnorm.is_zero() {
            return T::zero();
        }

        let alpha = v[0];
        let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
        let tau = (beta - alpha) / beta;

        let s = alpha - beta;

        if s.abs() > T::epsilon() {
            v.slice_mut(s![1..]).mapv_inplace(|x| x / s);
            v[0] = beta;
        } else {
            v.slice_mut(s![1..]).mapv_inplace(|x| x * (T::epsilon() / s));
            v.slice_mut(s![1..]).mapv_inplace(|x| x / T::epsilon());
            v[0] = beta;
        }

        tau
    }
}

/// Compute a householder transformation P so that P [alpha; x] = [beta; 0]
pub fn householder_transform2<T: Float + Signed>(alpha: &mut T, v: &mut Array1<T>) -> T {
    let n = v.len();

    if n == 1 {
        T::zero()
    } else {
        let x = v.slice(s![..n-1]);
        let xnorm = x.dot(&x).sqrt();

        if xnorm.is_zero() {
            return T::zero();
        }

        let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
        let tau = (beta - *alpha) / beta;

        let s = *alpha - beta;

        if s.abs() > T::epsilon() {
            v.slice_mut(s![..n-1]).mapv_inplace(|x| x / s);
            *alpha = beta;
        } else {
            v.slice_mut(s![..n-1]).mapv_inplace(|x| x * (T::epsilon() / s));
            v.slice_mut(s![..n-1]).mapv_inplace(|x| x / T::epsilon());
            *alpha = beta;
        }

        tau
    }
}

/// Apply a householder transformation H = I - tau v v^T to matrix A from the left
pub fn householder_hm<T: Float>(
    tau: T,
    v: &Array1<T>,
    a: &mut Array2<T>,
) -> Result<(), &'static str> {
    if tau.is_zero() {
        return Ok(());
    }

    let m = a.nrows();
    let n = a.ncols();

    if v.len() != m {
        return Err("matrix must match Householder vector dimensions");
    }

    for j in 0..n {
        let mut wj = a[(0, j)];
        for i in 1..m {
            wj = wj + a[(i, j)] * v[i];
        }

        a[(0, j)] = a[(0, j)] - tau * wj;
        for i in 1..m {
            a[(i, j)] = a[(i, j)] - tau * v[i] * wj;
        }
    }

    Ok(())
}

/// Apply a householder transformation H = I - tau v v^T to matrix A from the right
pub fn householder_mh<T: Float>(
    tau: T,
    v: &Array1<T>,
    a: &mut Array2<T>,
) -> Result<(), &'static str> {
    if tau.is_zero() {
        return Ok(());
    }

    let m = a.nrows();
    let n = a.ncols();

    if v.len() != n {
        return Err("matrix must match Householder vector dimensions");
    }

    for i in 0..m {
        let mut wi = a[(i, 0)];
        for j in 1..n {
            wi = wi + a[(i, j)] * v[j];
        }

        a[(i, 0)] = a[(i, 0)] - tau * wi;
        for j in 1..n {
            a[(i, j)] = a[(i, j)] - tau * wi * v[j];
        }
    }

    Ok(())
}

/// Apply a householder transformation H = I - tau v v^T to vector w
pub fn householder_hv<T: Float>(
    tau: T,
    v: &Array1<T>,
    w: &mut Array1<T>,
) -> Result<(), &'static str> {
    if tau.is_zero() {
        return Ok(());
    }

    let n = v.len();
    if w.len() != n {
        return Err("vector dimensions must match");
    }

    let mut d = w[0];
    for i in 1..n {
        d = d + w[i] * v[i];
    }

    w[0] = w[0] - tau * d;
    for i in 1..n {
        w[i] = w[i] - tau * v[i] * d;
    }

    Ok(())
}

/// Apply a householder transformation from the left with workspace
pub fn householder_left<T: Float>(
    tau: T,
    v: &Array1<T>,
    a: &mut Array2<T>,
    work: &mut Array1<T>,
) -> Result<(), &'static str> {
    let m = a.nrows();
    let n = a.ncols();

    if v.len() != m {
        return Err("matrix must match Householder vector dimensions");
    }
    if work.len() != n {
        return Err("workspace must match matrix");
    }

    if tau.is_zero() {
        return Ok(());
    }

    // work = A^T v
    for j in 0..n {
        work[j] = T::zero();
        for i in 0..m {
            work[j] = work[j] + a[(i, j)] * v[i];
        }
    }

    // A = A - tau v work^T
    for i in 0..m {
        for j in 0..n {
            a[(i, j)] = a[(i, j)] - tau * v[i] * work[j];
        }
    }

    Ok(())
}

/// Apply a householder transformation from the right with workspace
pub fn householder_right<T: Float>(
    tau: T,
    v: &mut Array1<T>,
    a: &mut Array2<T>,
    work: &mut Array1<T>,
) -> Result<(), &'static str> {
    let m = a.nrows();
    let n = a.ncols();

    if v.len() != n {
        return Err("matrix must match Householder vector dimensions");
    }
    if work.len() != m {
        return Err("workspace must match matrix");
    }

    if tau.is_zero() {
        return Ok(());
    }

    let v0 = v[0];
    v[0] = T::one();

    // work = A v
    for i in 0..m {
        work[i] = T::zero();
        for j in 0..n {
            work[i] = work[i] + a[(i, j)] * v[j];
        }
    }

    // A = A - tau work v^T
    for i in 0..m {
        for j in 0..n {
            a[(i, j)] = a[(i, j)] - tau * work[i] * v[j];
        }
    }

    v[0] = v0;

    Ok(())
}

/// Apply a householder transformation to a matrix being built up from the identity
pub fn householder_hm1<T: Float>(tau: T, a: &mut Array2<T>) -> Result<(), &'static str> {
    if tau.is_zero() {
        let m = a.nrows();
        let n = a.ncols();

        a[(0, 0)] = T::one();
        for j in 1..n {
            a[(0, j)] = T::zero();
        }
        for i in 1..m {
            a[(i, 0)] = T::zero();
        }

        return Ok(());
    }

    let m = a.nrows();
    let n = a.ncols();

    for j in 1..n {
        let mut wj = T::zero();
        for i in 1..m {
            wj = wj + a[(i, j)] * a[(i, 0)];
        }

        a[(0, j)] = -tau * wj;
        for i in 1..m {
            a[(i, j)] = a[(i, j)] - tau * a[(i, 0)] * wj;
        }
    }

    for i in 1..m {
        a[(i, 0)] = -tau * a[(i, 0)];
    }

    a[(0, 0)] = T::one() - tau;

    Ok(())
}