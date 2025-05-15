use gsl::{
    blas::{CBLAS_TRANSPOSE, CblasNoTrans, CblasTrans},
    error::{GSL_EBADLEN, GSL_EINVAL, GSL_SUCCESS},
    matrix::{Matrix, MatrixView},
    multifit::linear::{multifit_linear_solve, Workspace},
    vector::{Vector, VectorView},
};

pub fn gsl_multifit_wlinear(
    x: &Matrix,
    w: &Vector,
    y: &Vector,
    c: &mut Vector,
    cov: &mut Matrix,
    chisq: &mut f64,
    work: &mut Workspace,
) -> i32 {
    let mut rank = 0;
    gsl_multifit_wlinear_tsvd(
        x,
        w,
        y,
        2.2204460492503131e-16,
        c,
        cov,
        chisq,
        &mut rank,
        work,
    )
}

pub fn gsl_multifit_wlinear_tsvd(
    x: &Matrix,
    w: &Vector,
    y: &Vector,
    tol: f64,
    c: &mut Vector,
    cov: &mut Matrix,
    chisq: &mut f64,
    rank: &mut usize,
    work: &mut Workspace,
) -> i32 {
    let n = x.size1();
    let p = x.size2();

    if y.size() != n {
        return GSL_EBADLEN;
    }
    if w.size() != n {
        return GSL_EBADLEN;
    }
    if p != c.size() {
        return GSL_EBADLEN;
    }
    if tol <= 0.0 {
        return GSL_EINVAL;
    }

    let mut rnorm = 0.0;
    let mut snorm = 0.0;
    let mut a = work.a.submatrix(0, 0, n, p);
    let mut b = work.t.subvector(0, n);

    let status = work.apply_w(x, w, y, &mut a, &mut b);
    if status != GSL_SUCCESS {
        return status;
    }

    let status = work.bsvd(&a);
    if status != GSL_SUCCESS {
        return status;
    }

    let status = multifit_linear_solve(
        x,
        &b,
        tol,
        0.0,
        rank,
        c,
        &mut rnorm,
        &mut snorm,
        work,
    );
    if status != GSL_SUCCESS {
        return status;
    }

    *chisq = rnorm * rnorm;

    let p = x.size2();
    let mut qsi = work.qsi.submatrix(0, 0, p, p);
    let mut d = work.d.subvector(0, p);

    for i in 0..p {
        let row_i = qsi.row(i);
        let d_i = d.get(i);

        for j in i..p {
            let row_j = qsi.row(j);
            let d_j = d.get(j);
            let mut s = 0.0;

            row_i.dot(&row_j, &mut s);
            cov.set(i, j, s / (d_i * d_j));
            cov.set(j, i, s / (d_i * d_j));
        }
    }

    GSL_SUCCESS
}

pub fn gsl_multifit_wlinear_svd(
    x: &Matrix,
    w: &Vector,
    y: &Vector,
    tol: f64,
    rank: &mut usize,
    c: &mut Vector,
    cov: &mut Matrix,
    chisq: &mut f64,
    work: &mut Workspace,
) -> i32 {
    gsl_multifit_wlinear_tsvd(x, w, y, tol, c, cov, chisq, rank, work)
}

pub fn gsl_multifit_wlinear_usvd(
    x: &Matrix,
    w: &Vector,
    y: &Vector,
    tol: f64,
    rank: &mut usize,
    c: &mut Vector,
    cov: &mut Matrix,
    chisq: &mut f64,
    work: &mut Workspace,
) -> i32 {
    gsl_multifit_wlinear_tsvd(x, w, y, tol, c, cov, chisq, rank, work)
}