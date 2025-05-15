use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use ndarray_linalg::{cholesky::Cholesky, LeastSquaresSvd, Scalar};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultifitError {
    #[error("invalid matrix dimensions")]
    BadLen,
    #[error("singular matrix")]
    SingularMatrix,
    #[error("invalid input")]
    InvalidInput,
    #[error("failed to find minimum radius")]
    MinRadiusNotFound,
}

pub struct MultifitLinearWorkspace {
    nmax: usize,
    pmax: usize,
    a: Array2<f64>,
    q: Array2<f64>,
    s: Array1<f64>,
    t: Array1<f64>,
    xt: Array1<f64>,
    d: Array1<f64>,
    qsi: Array2<f64>,
}

impl MultifitLinearWorkspace {
    pub fn new(nmax: usize, pmax: usize) -> Self {
        Self {
            nmax,
            pmax,
            a: Array2::zeros((nmax, pmax)),
            q: Array2::zeros((pmax, pmax)),
            s: Array1::zeros(pmax),
            t: Array1::zeros(nmax),
            xt: Array1::zeros(pmax),
            d: Array1::ones(pmax),
            qsi: Array2::zeros((pmax, 1)),
        }
    }
}

pub fn multifit_linear_solve(
    lambda: f64,
    x: &Array2<f64>,
    y: &Array1<f64>,
    c: &mut Array1<f64>,
    rnorm: &mut f64,
    snorm: &mut f64,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let mut rank = 0;
    multifit_linear_solve_internal(
        x,
        y,
        f64::EPSILON,
        lambda,
        &mut rank,
        c,
        rnorm,
        snorm,
        work,
    )
}

fn multifit_linear_solve_internal(
    x: &Array2<f64>,
    y: &Array1<f64>,
    eps: f64,
    lambda: f64,
    rank: &mut usize,
    c: &mut Array1<f64>,
    rnorm: &mut f64,
    snorm: &mut f64,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    // Implementation of the actual solver would go here
    // This is a placeholder for the actual implementation
    Ok(())
}

pub fn multifit_linear_apply_w(
    x: &Array2<f64>,
    w: Option<&Array1<f64>>,
    y: &Array1<f64>,
    wx: &mut Array2<f64>,
    wy: &mut Array1<f64>,
) -> Result<(), MultifitError> {
    let n = x.shape()[0];
    let p = x.shape()[1];

    if n != y.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if let Some(w_vec) = w {
        if n != w_vec.shape()[0] {
            return Err(MultifitError::BadLen);
        }
    }

    if n != wx.shape()[0] || p != wx.shape()[1] {
        return Err(MultifitError::BadLen);
    }

    if n != wy.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    // Copy data if pointers are different
    if !wx.as_ptr().eq(&x.as_ptr()) {
        wx.assign(x);
    }
    if !wy.as_ptr().eq(&y.as_ptr()) {
        wy.assign(y);
    }

    if let Some(w_vec) = w {
        for i in 0..n {
            let mut wi = w_vec[i];
            if wi < 0.0 {
                wi = 0.0;
            }
            let swi = wi.sqrt();
            wx.row_mut(i).mapv_inplace(|x| x * swi);
            wy[i] *= swi;
        }
    }

    Ok(())
}

pub fn multifit_linear_wstdform1(
    l: Option<&Array1<f64>>,
    x: &Array2<f64>,
    w: Option<&Array1<f64>>,
    y: &Array1<f64>,
    xs: &mut Array2<f64>,
    ys: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let n = x.shape()[0];
    let p = x.shape()[1];

    if n > work.nmax || p > work.pmax {
        return Err(MultifitError::BadLen);
    }

    if let Some(l_vec) = l {
        if p != l_vec.shape()[0] {
            return Err(MultifitError::BadLen);
        }
    }

    if n != y.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if let Some(w_vec) = w {
        if n != w_vec.shape()[0] {
            return Err(MultifitError::BadLen);
        }
    }

    if n != xs.shape()[0] || p != xs.shape()[1] {
        return Err(MultifitError::BadLen);
    }

    if n != ys.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    // Compute Xs = sqrt(W) X and ys = sqrt(W) y
    multifit_linear_apply_w(x, w, y, xs, ys)?;

    if let Some(l_vec) = l {
        for j in 0..p {
            let lj = l_vec[j];
            if lj == 0.0 {
                return Err(MultifitError::SingularMatrix);
            }
            xs.column_mut(j).mapv_inplace(|x| x / lj);
        }
    }

    Ok(())
}

pub fn multifit_linear_stdform1(
    l: Option<&Array1<f64>>,
    x: &Array2<f64>,
    y: &Array1<f64>,
    xs: &mut Array2<f64>,
    ys: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    multifit_linear_wstdform1(l, x, None, y, xs, ys, work)
}

pub fn multifit_linear_l_decomp(l: &mut Array2<f64>, tau: &mut Array1<f64>) -> Result<(), MultifitError> {
    let m = l.shape()[0];
    let p = l.shape()[1];

    if tau.shape()[0] != m.min(p) {
        return Err(MultifitError::BadLen);
    }

    if m >= p {
        // Square or tall L matrix - perform QR decomposition
        let (q, r) = l.qr()?;
        l.assign(&r);
        tau.assign(&q.diag());
    } else {
        // More columns than rows - compute QR decomposition of L^T
        let lt = l.t();
        let (q, r) = lt.qr()?;
        l.assign(&r.t());
        tau.assign(&q.diag());
    }

    Ok(())
}

pub fn multifit_linear_wstdform2(
    lqr: &Array2<f64>,
    ltau: &Array1<f64>,
    x: &Array2<f64>,
    w: Option<&Array1<f64>>,
    y: &Array1<f64>,
    xs: &mut Array2<f64>,
    ys: &mut Array1<f64>,
    m: &mut Array2<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let m_rows = lqr.shape()[0];
    let n = x.shape()[0];
    let p = x.shape()[1];

    if n > work.nmax || p > work.pmax {
        return Err(MultifitError::BadLen);
    }

    if p != lqr.shape()[1] {
        return Err(MultifitError::BadLen);
    }

    if n != y.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if let Some(w_vec) = w {
        if n != w_vec.shape()[0] {
            return Err(MultifitError::BadLen);
        }
    }

    if m_rows >= p {
        // Case 1: m >= p
        if n != xs.shape()[0] || p != xs.shape()[1] {
            return Err(MultifitError::BadLen);
        }

        if n != ys.shape()[0] {
            return Err(MultifitError::BadLen);
        }

        // Compute Xs = sqrt(W) X and ys = sqrt(W) y
        multifit_linear_apply_w(x, w, y, xs, ys)?;

        // Compute X~ = X R^{-1} using QR decomposition of L
        let r = lqr.slice(s![..p, ..p]);
        for i in 0..n {
            let mut row = xs.row_mut(i);
            let r_t = r.t();
            row = r_t.solve(&row)?;
        }
    } else {
        // Case 2: m < p
        let pm = p - m_rows;
        let npm = n - pm;

        if npm != xs.shape()[0] || m_rows != xs.shape()[1] {
            return Err(MultifitError::BadLen);
        }

        if npm != ys.shape()[0] {
            return Err(MultifitError::BadLen);
        }

        if n != m.shape()[0] || p != m.shape()[1] {
            return Err(MultifitError::BadLen);
        }

        // Implementation of the rectangular case would go here
        // This is a placeholder for the actual implementation
    }

    Ok(())
}

pub fn multifit_linear_stdform2(
    lqr: &Array2<f64>,
    ltau: &Array1<f64>,
    x: &Array2<f64>,
    y: &Array1<f64>,
    xs: &mut Array2<f64>,
    ys: &mut Array1<f64>,
    m: &mut Array2<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    multifit_linear_wstdform2(lqr, ltau, x, None, y, xs, ys, m, work)
}

pub fn multifit_linear_genform1(
    l: &Array1<f64>,
    cs: &Array1<f64>,
    c: &mut Array1<f64>,
    work: &MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    if l.shape()[0] > work.pmax {
        return Err(MultifitError::BadLen);
    }

    if l.shape()[0] != cs.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if l.shape()[0] != c.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    // Compute true solution vector c = L^{-1} c~
    c.assign(&(cs / l));
    Ok(())
}

pub fn multifit_linear_wgenform2(
    lqr: &Array2<f64>,
    ltau: &Array1<f64>,
    x: &Array2<f64>,
    w: Option<&Array1<f64>>,
    y: &Array1<f64>,
    cs: &Array1<f64>,
    m: &Array2<f64>,
    c: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let m_rows = lqr.shape()[0];
    let n = x.shape()[0];
    let p = x.shape()[1];

    if n > work.nmax || p > work.pmax {
        return Err(MultifitError::BadLen);
    }

    if p != lqr.shape()[1] {
        return Err(MultifitError::BadLen);
    }

    if p != c.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if let Some(w_vec) = w {
        if n != w_vec.shape()[0] {
            return Err(MultifitError::BadLen);
        }
    }

    if n != y.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if m_rows >= p {
        // Case 1: m >= p
        if p != cs.shape()[0] {
            return Err(MultifitError::BadLen);
        }

        // Solve R c = cs for true solution c
        let r = lqr.slice(s![..p, ..p]);
        c.assign(&r.solve(&cs)?);
    } else {
        // Case 2: m < p
        if m_rows != cs.shape()[0] {
            return Err(MultifitError::BadLen);
        }

        if n != m.shape()[0] || p != m.shape()[1] {
            return Err(MultifitError::BadLen);
        }

        // Implementation of the rectangular case would go here
        // This is a placeholder for the actual implementation
    }

    Ok(())
}

pub fn multifit_linear_genform2(
    lqr: &Array2<f64>,
    ltau: &Array1<f64>,
    x: &Array2<f64>,
    y: &Array1<f64>,
    cs: &Array1<f64>,
    m: &Array2<f64>,
    c: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    multifit_linear_wgenform2(lqr, ltau, x, None, y, cs, m, c, work)
}

pub fn multifit_linear_lreg(
    smin: f64,
    smax: f64,
    reg_param: &mut Array1<f64>,
) -> Result<(), MultifitError> {
    if smax <= 0.0 {
        return Err(MultifitError::InvalidInput);
    }

    let n = reg_param.shape()[0];
    let smin_ratio = 16.0 * f64::EPSILON;
    let new_smin = smin.max(smax * smin_ratio);
    reg_param[n - 1] = new_smin;

    let ratio = (smax / new_smin).powf(1.0 / (n as f64 - 1.0));

    for i in (0..n - 1).rev() {
        let rp1 = reg_param[i + 1];
        reg_param[i] = ratio * rp1;
    }

    Ok(())
}

pub fn multifit_linear_lcurve(
    y: &Array1<f64>,
    reg_param: &mut Array1<f64>,
    rho: &mut Array1<f64>,
    eta: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let n = y.shape()[0];
    let n_points = rho.shape()[0];

    if n != work.nmax {
        return Err(MultifitError::BadLen);
    }

    if n_points < 3 {
        return Err(MultifitError::BadLen);
    }

    if n_points != eta.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    if reg_param.shape()[0] != eta.shape()[0] {
        return Err(MultifitError::BadLen);
    }

    let p = work.pmax;
    let s = work.s.view();
    let smax = s[0];
    let smin = s[p - 1];

    // Compute projection xt = U^T y
    let u = work.a.slice(s![..n, ..p]);
    let mut xt = work.xt.slice_mut(s![..p]);
    xt.assign(&u.t().dot(y));

    let norm_uty = xt.norm_l2();
    let norm_y = y.norm_l2();
    let dr = norm_y.powi(2) - norm_uty.powi(2);

    // Calculate regularization parameters
    multifit_linear_lreg(smin, smax, reg_param)?;

    for i in 0..n_points {
        let lambda = reg_param[i];
        let lambda_sq = lambda * lambda;

        let mut workp = work.xt.slice_mut(s![..p]);
        let mut workp2 = work.d.slice_mut(s![..p]);

        for j in 0..p {
            let sj = s[j];
            let xtj = xt[j];
            let f = sj / (sj.powi(2) + lambda_sq);

            workp[j] = f * xtj;
            workp2[j] = (1.0 - sj * f) * xtj;
        }

        eta[i] = workp.norm_l2();
        rho[i] = workp2.norm_l2();
    }

    if n > p && dr > 0.0 {
        for i in 0..n_points {
            rho[i] = (rho[i].powi(2) + dr).sqrt();
        }
    }

    // Restore D to identity matrix
    work.d.fill(1.0);

    Ok(())
}

pub fn multifit_linear_lcurvature(
    y: &Array1<f64>,
    reg_param: &Array1<f64>,
    rho: &Array1<f64>,
    eta: &Array1<f64>,
    kappa: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let n = y.shape()[0];
    let n_points = rho.shape()[0];

    if n != work.nmax