use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultifitError {
    #[error("number of observations in y does not match matrix")]
    ObservationMismatch,
    #[error("number of weights in w does not match matrix")]
    WeightMismatch,
    #[error("number of parameters c does not match matrix")]
    ParameterMismatch,
    #[error("tolerance must be positive")]
    InvalidTolerance,
    #[error("SVD computation failed")]
    SvdFailure,
    #[error("Linear solve failed")]
    LinearSolveFailure,
    #[error("Workspace error")]
    WorkspaceError,
}

pub struct MultifitLinearWorkspace {
    a: Array2<f64>,
    t: Array1<f64>,
    qsi: Array2<f64>,
    d: Array1<f64>,
}

pub fn gsl_multifit_wlinear(
    x: ArrayView2<f64>,
    w: ArrayView1<f64>,
    y: ArrayView1<f64>,
    c: &mut Array1<f64>,
    cov: &mut Array2<f64>,
    chisq: &mut f64,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let mut rank = 0;
    gsl_multifit_wlinear_tsvd(
        x,
        w,
        y,
        f64::EPSILON,
        c,
        cov,
        chisq,
        &mut rank,
        work,
    )
}

pub fn gsl_multifit_wlinear_tsvd(
    x: ArrayView2<f64>,
    w: ArrayView1<f64>,
    y: ArrayView1<f64>,
    tol: f64,
    c: &mut Array1<f64>,
    cov: &mut Array2<f64>,
    chisq: &mut f64,
    rank: &mut usize,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    let n = x.shape()[0];
    let p = x.shape()[1];

    if y.shape()[0] != n {
        return Err(MultifitError::ObservationMismatch);
    } else if w.shape()[0] != n {
        return Err(MultifitError::WeightMismatch);
    } else if p != c.shape()[0] {
        return Err(MultifitError::ParameterMismatch);
    } else if tol <= 0.0 {
        return Err(MultifitError::InvalidTolerance);
    }

    // Compute A = sqrt(W) X, b = sqrt(W) y
    let mut a = work.a.slice_mut(s![..n, ..p]);
    let mut b = work.t.slice_mut(s![..n]);
    multifit_linear_apply_w(x, w, y, &mut a, &mut b)?;

    // Compute SVD of A
    multifit_linear_bsvd(&a.view(), work)?;

    let (rnorm, snorm) = multifit_linear_solve(
        &x.view(),
        &b.view(),
        tol,
        0.0,
        rank,
        c,
        work,
    )?;

    *chisq = rnorm * rnorm;

    // variance-covariance matrix cov = s2 * (Q S^-1) (Q S^-1)^T
    let qsi = work.qsi.slice(s![..p, ..p]);
    let d = work.d.slice(s![..p]);

    for i in 0..p {
        let row_i = qsi.row(i);
        let d_i = d[i];

        for j in i..p {
            let row_j = qsi.row(j);
            let d_j = d[j];
            let s = row_i.dot(&row_j);

            cov[[i, j]] = s / (d_i * d_j);
            cov[[j, i]] = s / (d_i * d_j);
        }
    }

    Ok(())
}

pub fn gsl_multifit_wlinear_svd(
    x: ArrayView2<f64>,
    w: ArrayView1<f64>,
    y: ArrayView1<f64>,
    tol: f64,
    rank: &mut usize,
    c: &mut Array1<f64>,
    cov: &mut Array2<f64>,
    chisq: &mut f64,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    gsl_multifit_wlinear_tsvd(x, w, y, tol, c, cov, chisq, rank, work)
}

pub fn gsl_multifit_wlinear_usvd(
    x: ArrayView2<f64>,
    w: ArrayView1<f64>,
    y: ArrayView1<f64>,
    tol: f64,
    rank: &mut usize,
    c: &mut Array1<f64>,
    cov: &mut Array2<f64>,
    chisq: &mut f64,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    gsl_multifit_wlinear_tsvd(x, w, y, tol, c, cov, chisq, rank, work)
}

// Helper functions would be implemented separately
fn multifit_linear_apply_w(
    x: ArrayView2<f64>,
    w: ArrayView1<f64>,
    y: ArrayView1<f64>,
    a: &mut Array2<f64>,
    b: &mut Array1<f64>,
) -> Result<(), MultifitError> {
    // Implementation would multiply by sqrt(w)
    unimplemented!()
}

fn multifit_linear_bsvd(
    a: &Array2<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(), MultifitError> {
    // Implementation would compute SVD
    unimplemented!()
}

fn multifit_linear_solve(
    x: &ArrayView2<f64>,
    b: &ArrayView1<f64>,
    tol: f64,
    lambda: f64,
    rank: &mut usize,
    c: &mut Array1<f64>,
    work: &mut MultifitLinearWorkspace,
) -> Result<(f64, f64), MultifitError> {
    // Implementation would solve linear system
    unimplemented!()
}