use gsl::{
    blas::{CblasDiag, CblasTranspose, CblasUpperLower},
    error::{GslError, GslResult},
    matrix::Matrix,
    rng::Rng,
    stats::{covariance, mean, variance},
    vector::Vector,
};
use libc::c_double;
use std::f64::consts::PI;

pub fn multivariate_gaussian(
    rng: &mut Rng,
    mu: &Vector,
    l: &Matrix,
    result: &mut Vector,
) -> GslResult<()> {
    let m = l.size1();
    let n = l.size2();

    if m != n {
        return Err(GslError::NotSquare);
    }
    if mu.len() != m {
        return Err(GslError::BadLength);
    }
    if result.len() != m {
        return Err(GslError::BadLength);
    }

    for i in 0..m {
        result.set(i, rng.gaussian(0.0, 1.0)?);
    }

    l.dtrmv(
        CblasUpperLower::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        result,
    )?;
    result.add(mu)?;

    Ok(())
}

pub fn multivariate_gaussian_log_pdf(
    x: &Vector,
    mu: &Vector,
    l: &Matrix,
    work: &mut Vector,
) -> GslResult<f64> {
    let m = l.size1();
    let n = l.size2();

    if m != n {
        return Err(GslError::NotSquare);
    }
    if mu.len() != m {
        return Err(GslError::BadLength);
    }
    if x.len() != m {
        return Err(GslError::BadLength);
    }
    if work.len() != m {
        return Err(GslError::BadLength);
    }

    for i in 0..m {
        work.set(i, x.get(i) - mu.get(i));
    }

    l.dtrsv(
        CblasUpperLower::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        work,
    )?;

    let quad_form = work.dot(work)?;
    let mut log_sqrt_det_sigma = 0.0;

    for i in 0..m {
        log_sqrt_det_sigma += l.get(i, i).ln();
    }

    Ok(-0.5 * quad_form - log_sqrt_det_sigma - 0.5 * (m as f64) * (2.0 * PI).ln())
}

pub fn multivariate_gaussian_pdf(
    x: &Vector,
    mu: &Vector,
    l: &Matrix,
    work: &mut Vector,
) -> GslResult<f64> {
    let logpdf = multivariate_gaussian_log_pdf(x, mu, l, work)?;
    Ok(logpdf.exp())
}

pub fn multivariate_gaussian_mean(x: &Matrix, mu_hat: &mut Vector) -> GslResult<()> {
    let n = x.size2();

    if n != mu_hat.len() {
        return Err(GslError::BadLength);
    }

    for j in 0..n {
        let column = x.column(j);
        let mean = mean(column.data(), column.stride(), x.size1())?;
        mu_hat.set(j, mean);
    }

    Ok(())
}

pub fn multivariate_gaussian_vcov(x: &Matrix, sigma_hat: &mut Matrix) -> GslResult<()> {
    let d = x.size2();

    if sigma_hat.size1() != sigma_hat.size2() {
        return Err(GslError::NotSquare);
    }
    if d != sigma_hat.size1() {
        return Err(GslError::BadLength);
    }

    for j1 in 0..d {
        sigma_hat.set(
            j1,
            j1,
            variance(&x.column(j1).data(), x.column(j1).stride(), x.size1())?,
        );

        for j2 in (j1 + 1)..d {
            let cov = covariance(
                &x.column(j1).data(),
                x.column(j1).stride(),
                &x.column(j2).data(),
                x.column(j2).stride(),
                x.size1(),
            )?;

            sigma_hat.set(j1, j2, cov);
            sigma_hat.set(j2, j1, cov);
        }
    }

    Ok(())
}