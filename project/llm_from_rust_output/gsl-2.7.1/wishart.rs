use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    error::{Error, Result},
    linalg::cholesky::Cholesky,
    matrix::{Matrix, MatrixView},
    rng::Rng,
    stats::distributions::{chi_squared::ChiSquared, normal::StandardNormal},
};
use std::f64::consts::{LN_2, LN_PI};

pub fn wishart(
    rng: &mut Rng,
    df: f64,
    l: &MatrixView,
    result: &mut Matrix,
    work: &mut Matrix,
) -> Result<()> {
    let d = l.size1();
    if l.size1() != l.size2() {
        return Err(Error::NotSquare);
    }
    if result.size1() != result.size2() {
        return Err(Error::NotSquare);
    }
    if work.size1() != work.size2() {
        return Err(Error::NotSquare);
    }
    if result.size1() != d {
        return Err(Error::BadLength);
    }
    if work.size1() != d {
        return Err(Error::BadLength);
    }
    if df <= (d - 1) as f64 {
        return Err(Error::Domain);
    }

    work.set_zero();
    let chi_sq = ChiSquared::new(df);
    let normal = StandardNormal;

    for i in 0..d {
        work.set(i, i, rng.sample(&chi_sq.with_df(df - i as f64)).sqrt());
        for j in 0..i {
            work.set(i, j, rng.sample(&normal));
        }
    }

    work.trmm(
        CblasSide::Left,
        CblasUplo::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        1.0,
        l,
    )?;

    result.syrk(
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        1.0,
        work,
        0.0,
    )?;

    for i in 0..d {
        for j in 0..i {
            result.set(i, j, result.get(j, i));
        }
    }

    Ok(())
}

pub fn wishart_log_pdf(
    x: &MatrixView,
    l_x: &MatrixView,
    df: f64,
    l: &MatrixView,
    result: &mut f64,
    work: &mut Matrix,
) -> Result<()> {
    let d = l.size1();
    if l.size1() != l.size2() {
        return Err(Error::NotSquare);
    }
    if x.size1() != x.size2() {
        return Err(Error::NotSquare);
    }
    if l_x.size1() != l_x.size2() {
        return Err(Error::NotSquare);
    }
    if x.size1() != d {
        return Err(Error::BadLength);
    }
    if l_x.size1() != d {
        return Err(Error::BadLength);
    }
    if df <= (d - 1) as f64 {
        return Err(Error::Domain);
    }

    let mut log_mv_ga = (d * (d - 1)) as f64 * 0.25 * LN_PI;
    for i in 0..d {
        log_mv_ga += gsl::sf::lngamma((df - i as f64 + 1.0) * 0.5);
    }

    let mut log_det_v = l.get(0, 0).ln();
    for i in 1..d {
        log_det_v += l.get(i, i).ln();
    }
    log_det_v *= 2.0;

    let mut log_det_x = l_x.get(0, 0).ln();
    for i in 1..d {
        log_det_x += l_x.get(i, i).ln();
    }
    log_det_x *= 2.0;

    Cholesky::solve_mat(l, x, work)?;

    let mut tr_vinv_x = work.get(0, 0);
    for i in 1..d {
        tr_vinv_x += work.get(i, i);
    }

    *result = -0.5 * df * d as f64 * LN_2
        - 0.5 * df * log_det_v
        - log_mv_ga
        + 0.5 * (df - d as f64 - 1.0) * log_det_x
        - 0.5 * tr_vinv_x;

    Ok(())
}

pub fn wishart_pdf(
    x: &MatrixView,
    l_x: &MatrixView,
    df: f64,
    l: &MatrixView,
    result: &mut f64,
    work: &mut Matrix,
) -> Result<()> {
    let mut logpdf = 0.0;
    wishart_log_pdf(x, l_x, df, l, &mut logpdf, work)?;
    *result = logpdf.exp();
    Ok(())
}