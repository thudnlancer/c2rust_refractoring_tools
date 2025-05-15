use gsl::{
    blas::{dgemv, dnrm2, dtrsv, dsyrk, CblasNoTrans, CblasTrans, CblasUpper, CblasNonUnit},
    error::{Error, Result},
    linalg::{QR_decomp, QR_QTmat, QR_QTvec, QR_matQ, QR_Qvec, cholesky_decomp},
    matrix::{Matrix, MatrixView, MatrixViewMut},
    vector::{Vector, VectorView, VectorViewMut},
    Value,
};
use libc::{c_double, c_int, size_t};
use std::f64::consts::E;

const GSL_EBADLEN: c_int = 19;
const GSL_EINVAL: c_int = 4;
const GSL_EDOM: c_int = 1;
const GSL_ENOTSQR: c_int = 20;
const GSL_SUCCESS: c_int = 0;

pub struct MultifitLinearWorkspace {
    nmax: size_t,
    pmax: size_t,
    n: size_t,
    p: size_t,
    a: Matrix,
    q: Matrix,
    qsi: Matrix,
    s: Vector,
    t: Vector,
    xt: Vector,
    d: Vector,
    rcond: c_double,
}

pub fn multifit_linear_solve(
    x: &Matrix,
    y: &Vector,
    tol: c_double,
    lambda: c_double,
    rank: &mut size_t,
    c: &mut Vector,
    rnorm: &mut c_double,
    snorm: &mut c_double,
    work: &mut MultifitLinearWorkspace,
) -> Result<()> {
    let n = x.size1();
    let p = x.size2();

    if n != work.n || p != work.p {
        return Err(Error::new(GSL_EBADLEN, "observation matrix does not match workspace"));
    }
    if n != y.size() {
        return Err(Error::new(GSL_EBADLEN, "number of observations in y does not match matrix"));
    }
    if p != c.size() {
        return Err(Error::new(GSL_EBADLEN, "number of parameters c does not match matrix"));
    }
    if tol <= 0.0 {
        return Err(Error::new(GSL_EINVAL, "tolerance must be positive"));
    }

    let lambda_sq = lambda * lambda;
    let mut rho_ls = 0.0;
    let mut p_eff = 0;

    let mut a = work.a.submatrix(0, 0, n, p);
    let mut q = work.q.submatrix(0, 0, p, p);
    let mut s = work.s.subvector(0, p);
    let mut qsi = work.qsi.submatrix(0, 0, p, p);
    let mut xt = work.xt.subvector(0, p);
    let mut d = work.d.subvector(0, p);
    let mut t = work.t.subvector(0, n);

    dgemv(CblasTrans, 1.0, &a, y, 0.0, &mut xt)?;

    if n > p {
        t.copy(y)?;
        dgemv(CblasNoTrans, -1.0, &a, &xt, 1.0, &mut t)?;
        rho_ls = dnrm2(&t);
    }

    if lambda > 0.0 {
        for j in 0..p {
            let sj = s.get(j);
            let f = sj * sj / (sj * sj + lambda_sq);
            let ptr = xt.get_mut(j);
            d.set(j, (1.0 - f) * *ptr);
            *ptr *= sj / (sj * sj + lambda_sq);
        }

        dgemv(CblasNoTrans, 1.0, &q, &xt, 0.0, c)?;
        *snorm = dnrm2(c);
        *rnorm = dnrm2(&d);

        if n > p {
            *rnorm = (*rnorm * *rnorm + rho_ls * rho_ls).sqrt();
        }

        d.set_all(1.0);
    } else {
        qsi.copy(&q)?;
        let s0 = s.get(0);
        p_eff = 0;

        for j in 0..p {
            let mut column = qsi.column(j);
            let sj = s.get(j);
            let alpha = if sj <= tol * s0 { 0.0 } else { 1.0 / sj };
            if alpha != 0.0 {
                p_eff += 1;
            }
            column.scale(alpha);
        }

        *rank = p_eff;
        dgemv(CblasNoTrans, 1.0, &qsi, &xt, 0.0, c)?;
        c.div_elements(&d)?;
        *snorm = dnrm2(c);
        *rnorm = rho_ls;
    }

    Ok(())
}

pub fn multifit_linear_solve_lambda(
    lambda: c_double,
    x: &Matrix,
    y: &Vector,
    c: &mut Vector,
    rnorm: &mut c_double,
    snorm: &mut c_double,
    work: &mut MultifitLinearWorkspace,
) -> Result<()> {
    let mut rank = 0;
    multifit_linear_solve(
        x,
        y,
        2.2204460492503131e-16,
        lambda,
        &mut rank,
        c,
        rnorm,
        snorm,
        work,
    )
}

pub fn multifit_linear_apply_w(
    x: &Matrix,
    w: Option<&Vector>,
    y: &Vector,
    wx: &mut Matrix,
    wy: &mut Vector,
) -> Result<()> {
    let n = x.size1();
    let p = x.size2();

    if n != y.size() {
        return Err(Error::new(GSL_EBADLEN, "y vector does not match X"));
    }
    if let Some(w) = w {
        if n != w.size() {
            return Err(Error::new(GSL_EBADLEN, "weight vector does not match X"));
        }
    }
    if n != wx.size1() || p != wx.size2() {
        return Err(Error::new(GSL_EBADLEN, "WX matrix dimensions do not match X"));
    }
    if n != wy.size() {
        return Err(Error::new(GSL_EBADLEN, "Wy vector must be length n"));
    }

    if wx.as_ptr() != x.as_ptr() {
        wx.copy(x)?;
    }
    if wy.as_ptr() != y.as_ptr() {
        wy.copy(y)?;
    }

    if let Some(w) = w {
        for i in 0..n {
            let mut wi = w.get(i);
            if wi < 0.0 {
                wi = 0.0;
            }
            let swi = wi.sqrt();
            let mut row = wx.row(i);
            row.scale(swi);
            *wy.get_mut(i) *= swi;
        }
    }

    Ok(())
}

// ... 其他函数的类似转换 ...

pub fn multifit_linear_lreg(
    smin: c_double,
    smax: c_double,
    reg_param: &mut Vector,
) -> Result<()> {
    if smax <= 0.0 {
        return Err(Error::new(GSL_EINVAL, "smax must be positive"));
    }

    let n = reg_param.size();
    let smin_ratio = 16.0 * 2.2204460492503131e-16;
    let new_smin = if smin > smax * smin_ratio {
        smin
    } else {
        smax * smin_ratio
    };

    reg_param.set(n - 1, new_smin);
    let ratio = (smax / new_smin).powf(1.0 / (n as c_double - 1.0));

    for i in (1..n).rev() {
        let rp1 = reg_param.get(i);
        reg_param.set(i - 1, ratio * rp1);
    }

    Ok(())
}

// ... 继续转换其他函数 ...

pub fn multifit_linear_lk(
    p: size_t,
    k: size_t,
    l: &mut Matrix,
) -> Result<()> {
    if p <= k {
        return Err(Error::new(GSL_EBADLEN, "p must be larger than derivative order"));
    }
    if k >= (100 - 1) as size_t {
        return Err(Error::new(GSL_EBADLEN, "derivative order k too large"));
    }
    if p - k != l.size1() || p != l.size2() {
        return Err(Error::new(GSL_EBADLEN, "L matrix must be (p-k)-by-p"));
    }

    if k == 0 {
        l.set_identity();
        return Ok(());
    }

    l.set_zero();
    let mut c = vec![0.0; k + 1];
    c[0] = -1.0;
    c[1] = 1.0;

    for i in 1..k {
        let mut cjm1 = 0.0;
        for j in 0..=k {
            let cj = c[j];
            c[j] = cjm1 - cj;
            cjm1 = cj;
        }
    }

    for i in 0..=k {
        let mut v = l.superdiagonal(i);
        v.set_all(c[i]);
    }

    Ok(())
}