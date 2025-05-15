use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use ndarray_linalg::{norm::Norm, Scalar};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GcvError {
    #[error("vector length mismatch")]
    LengthMismatch,
    #[error("maximum iterations reached")]
    MaxIterations,
}

pub struct GcvParams<'a> {
    s: ArrayView1<'a, f64>,
    uty: ArrayView1<'a, f64>,
    delta0: f64,
    np: usize,
    workp: Array1<f64>,
}

pub fn multifit_linear_gcv_init(
    y: &Array1<f64>,
    reg_param: &mut Array1<f64>,
    uty: &mut Array1<f64>,
    delta0: &mut f64,
    work: &LinearWorkspace,
) -> Result<(), GcvError> {
    let n = y.len();
    let p = work.p;

    if n != work.n {
        return Err(GcvError::LengthMismatch);
    } else if uty.len() != p {
        return Err(GcvError::LengthMismatch);
    }

    let u = work.a.slice(s![.., ..p]);
    let s = work.s.slice(s![..p]);

    let smax = s[0];
    let smin = s[p - 1];

    let normy = y.norm_l2();
    uty.assign(&u.t().dot(y));
    let norm_uty = uty.norm_l2();

    let dr = (normy + norm_uty) * (normy - norm_uty);

    multifit_linear_lreg(smin, smax, reg_param);

    *delta0 = if n > p && dr > 0.0 { dr } else { 0.0 };

    Ok(())
}

pub fn multifit_linear_gcv_curve(
    reg_param: &Array1<f64>,
    uty: &Array1<f64>,
    delta0: f64,
    g: &mut Array1<f64>,
    work: &LinearWorkspace,
) -> Result<(), GcvError> {
    let n = work.n;
    let p = work.p;
    let n_points = reg_param.len();

    if uty.len() != p {
        return Err(GcvError::LengthMismatch);
    } else if g.len() != n_points {
        return Err(GcvError::LengthMismatch);
    }

    let s = work.s.slice(s![..p]);
    let mut workp = work.qsi.slice(s![.., 0]).to_owned();

    let params = GcvParams {
        s: s.view(),
        uty: uty.view(),
        delta0,
        np: n - p,
        workp,
    };

    for (i, &lambda) in reg_param.iter().enumerate() {
        let gi = gcv_func(lambda, &params);
        g[i] = gi;
    }

    Ok(())
}

pub fn multifit_linear_gcv_min(
    reg_param: &Array1<f64>,
    uty: &Array1<f64>,
    g: &Array1<f64>,
    delta0: f64,
    lambda: &mut f64,
    work: &LinearWorkspace,
) -> Result<(), GcvError> {
    let n = work.n;
    let p = work.p;
    let n_points = reg_param.len();

    if uty.len() != p {
        return Err(GcvError::LengthMismatch);
    } else if g.len() != n_points {
        return Err(GcvError::LengthMismatch);
    }

    let s = work.s.slice(s![..p]);
    let workp = work.qsi.slice(s![.., 0]).to_owned();

    let idx_g = g.argmin().unwrap();
    let mut a = reg_param[std::cmp::min(idx_g + 1, n_points - 1)];
    let mut b = reg_param[std::cmp::max(idx_g - 1, 0)];
    let mut m = reg_param[idx_g];

    if idx_g == 0 || idx_g == n_points - 1 {
        *lambda = m;
        return Ok(());
    }

    let params = GcvParams {
        s: s.view(),
        uty: uty.view(),
        delta0,
        np: n - p,
        workp,
    };

    // TODO: Implement Brent's method minimization here
    // This is a placeholder for the actual minimization code
    *lambda = m;
    Ok(())
}

pub fn multifit_linear_gcv_calc(
    lambda: f64,
    uty: &Array1<f64>,
    delta0: f64,
    work: &LinearWorkspace,
) -> Result<f64, GcvError> {
    let n = work.n;
    let p = work.p;

    if uty.len() != p {
        return Err(GcvError::LengthMismatch);
    }

    let s = work.s.slice(s![..p]);
    let workp = work.qsi.slice(s![.., 0]).to_owned();

    let params = GcvParams {
        s: s.view(),
        uty: uty.view(),
        delta0,
        np: n - p,
        workp,
    };

    Ok(gcv_func(lambda, &params))
}

pub fn multifit_linear_gcv(
    y: &Array1<f64>,
    reg_param: &mut Array1<f64>,
    g: &mut Array1<f64>,
    lambda: &mut f64,
    g_lambda: &mut f64,
    work: &LinearWorkspace,
) -> Result<(), GcvError> {
    let n = y.len();
    let n_points = g.len();

    if n != work.n {
        return Err(GcvError::LengthMismatch);
    } else if reg_param.len() != n_points {
        return Err(GcvError::LengthMismatch);
    }

    let p = work.p;
    let mut uty = work.xt.slice(s![..p]).to_owned();
    let mut delta0 = 0.0;

    multifit_linear_gcv_init(y, reg_param, &mut uty, &mut delta0, work)?;
    multifit_linear_gcv_curve(reg_param, &uty, delta0, g, work)?;
    multifit_linear_gcv_min(reg_param, &uty, g, delta0, lambda, work)?;
    *g_lambda = multifit_linear_gcv_calc(*lambda, &uty, delta0, work)?;

    Ok(())
}

fn gcv_func(lambda: f64, params: &GcvParams) -> f64 {
    let s = &params.s;
    let uty = &params.uty;
    let delta0 = params.delta0;
    let np = params.np;
    let mut workp = params.workp.clone();
    let p = s.len();

    let lambda_sq = lambda * lambda;
    let mut sumf = 0.0;

    for i in 0..p {
        let si = s[i];
        let fi = lambda_sq / (si * si + lambda_sq);
        workp[i] = fi;
        sumf += fi;
    }

    let d = np as f64 + sumf;
    workp *= uty;
    let norm = workp.norm_l2();

    (norm * norm + delta0) / (d * d)
}

struct LinearWorkspace {
    a: Array2<f64>,
    s: Array1<f64>,
    qsi: Array2<f64>,
    xt: Array1<f64>,
    n: usize,
    p: usize,
}

fn multifit_linear_lreg(smin: f64, smax: f64, reg_param: &mut Array1<f64>) {
    // Implementation of regularization parameter calculation
    // This is a placeholder for the actual implementation
    for (i, param) in reg_param.iter_mut().enumerate() {
        *param = smin + (smax - smin) * (i as f64) / (reg_param.len() as f64 - 1.0);
    }
}