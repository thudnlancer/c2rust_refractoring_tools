use gsl::{
    blas::{daxpy, ddot, dgemv, dger, dnrm2, dscal, Layout, Transpose},
    error::{Error, Result},
    types::{Matrix, Vector, VectorView},
};
use libc::c_double;
use std::f64;

pub fn householder_transform(v: &mut Vector) -> c_double {
    let n = v.size();
    if n == 1 {
        return 0.0;
    }

    let (alpha, x_view) = {
        let alpha = v.get(0);
        let x = v.subvector(1, n - 1);
        (alpha, x)
    };

    let xnorm = dnrm2(&x_view);
    if xnorm == 0.0 {
        return 0.0;
    }

    let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
    let tau = (beta - alpha) / beta;
    let s = alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        dscal(1.0 / s, &mut x_view);
        v.set(0, beta);
    } else {
        let scale = f64::EPSILON / s;
        dscal(scale, &mut x_view);
        dscal(1.0 / f64::EPSILON, &mut x_view);
        v.set(0, beta);
    }

    tau
}

pub fn householder_transform2(alpha: &mut c_double, v: &mut Vector) -> c_double {
    let n = v.size();
    if n == 1 {
        return 0.0;
    }

    let x_view = v.subvector(0, n - 1);
    let xnorm = dnrm2(&x_view);
    if xnorm == 0.0 {
        return 0.0;
    }

    let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
    let tau = (beta - *alpha) / beta;
    let s = *alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        dscal(1.0 / s, &mut x_view);
        *alpha = beta;
    } else {
        let scale = f64::EPSILON / s;
        dscal(scale, &mut x_view);
        dscal(1.0 / f64::EPSILON, &mut x_view);
        *alpha = beta;
    }

    tau
}

pub fn householder_hm(tau: c_double, v: &Vector, a: &mut Matrix) -> Result<()> {
    if tau == 0.0 {
        return Ok(());
    }

    for j in 0..a.size2() {
        let mut wj = a.get(0, j);
        for i in 1..a.size1() {
            wj += a.get(i, j) * v.get(i);
        }

        let a0j = a.get(0, j);
        a.set(0, j, a0j - tau * wj);

        for i in 1..a.size1() {
            let aij = a.get(i, j);
            let vi = v.get(i);
            a.set(i, j, aij - tau * vi * wj);
        }
    }

    Ok(())
}

pub fn householder_mh(tau: c_double, v: &Vector, a: &mut Matrix) -> Result<()> {
    if tau == 0.0 {
        return Ok(());
    }

    for i in 0..a.size1() {
        let mut wi = a.get(i, 0);
        for j in 1..a.size2() {
            wi += a.get(i, j) * v.get(j);
        }

        let ai0 = a.get(i, 0);
        a.set(i, 0, ai0 - tau * wi);

        for j in 1..a.size2() {
            let vj = v.get(j);
            let aij = a.get(i, j);
            a.set(i, j, aij - tau * wi * vj);
        }
    }

    Ok(())
}

pub fn householder_hv(tau: c_double, v: &Vector, w: &mut Vector) -> Result<()> {
    let n = v.size();
    if tau == 0.0 {
        return Ok(());
    }

    let w0 = w.get(0);
    let v1 = v.subvector(1, n - 1);
    let mut w1 = w.subvector(1, n - 1);

    let d1 = ddot(&v1, &w1)?;
    let d = w0 + d1;

    w.set(0, w0 - tau * d);
    daxpy(-tau * d, &v1, &mut w1)?;

    Ok(())
}

pub fn householder_left(
    tau: c_double,
    v: &Vector,
    a: &mut Matrix,
    work: &mut Vector,
) -> Result<()> {
    let m = a.size1();
    let n = a.size2();

    if v.size() != m {
        return Err(Error::BadLen);
    }
    if work.size() != n {
        return Err(Error::BadLen);
    }

    if tau == 0.0 {
        return Ok(());
    }

    dgemv(
        Layout::RowMajor,
        Transpose::Trans,
        1.0,
        a,
        v,
        0.0,
        work,
    )?;
    dger(Layout::RowMajor, -tau, v, work, a)?;

    Ok(())
}

pub fn householder_right(
    tau: c_double,
    v: &Vector,
    a: &mut Matrix,
    work: &mut Vector,
) -> Result<()> {
    let m = a.size1();
    let n = a.size2();

    if v.size() != n {
        return Err(Error::BadLen);
    }
    if work.size() != m {
        return Err(Error::BadLen);
    }

    if tau == 0.0 {
        return Ok(());
    }

    let mut v = v.clone();
    let v0 = v.get(0);
    v.set(0, 1.0);

    dgemv(
        Layout::RowMajor,
        Transpose::NoTrans,
        1.0,
        a,
        &v,
        0.0,
        work,
    )?;
    dger(Layout::RowMajor, -tau, work, &v, a)?;

    v.set(0, v0);

    Ok(())
}

pub fn householder_hm1(tau: c_double, a: &mut Matrix) -> Result<()> {
    if tau == 0.0 {
        a.set(0, 0, 1.0);
        for j in 1..a.size2() {
            a.set(0, j, 0.0);
        }
        for i in 1..a.size1() {
            a.set(i, 0, 0.0);
        }
        return Ok(());
    }

    for j in 1..a.size2() {
        let mut wj = 0.0;
        for i in 1..a.size1() {
            let vi = a.get(i, 0);
            wj += a.get(i, j) * vi;
        }

        a.set(0, j, -tau * wj);

        for i in 1..a.size1() {
            let vi = a.get(i, 0);
            let aij = a.get(i, j);
            a.set(i, j, aij - tau * vi * wj);
        }
    }

    for i in 1..a.size1() {
        let vi = a.get(i, 0);
        a.set(i, 0, -tau * vi);
    }

    a.set(0, 0, 1.0 - tau);

    Ok(())
}