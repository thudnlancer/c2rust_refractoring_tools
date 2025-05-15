use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use ndarray_linalg::{cholesky::Cholesky, Dot, Solve};
use rand::distributions::{Distribution, StandardNormal};
use rand::Rng;
use std::f64::consts::PI;

/// Generate a random vector from a multivariate Gaussian distribution using
/// the Cholesky decomposition of the variance-covariance matrix.
///
/// # Arguments
/// * `rng` - Random number generator
/// * `mu` - Mean vector (dimension d)
/// * `l` - Lower triangular matrix from Cholesky decomposition of variance-covariance matrix Sigma = L L^T (dimension d x d)
///
/// # Returns
/// Result containing the random vector or an error string
pub fn multivariate_gaussian<R: Rng>(
    rng: &mut R,
    mu: &Array1<f64>,
    l: &Array2<f64>,
) -> Result<Array1<f64>, String> {
    let m = l.nrows();
    let n = l.ncols();

    if m != n {
        return Err("requires square matrix".to_string());
    } else if mu.len() != m {
        return Err("incompatible dimension of mean vector with variance-covariance matrix".to_string());
    }

    // Generate standard normal random variables
    let z: Array1<f64> = Array1::from_iter(
        (0..m).map(|_| StandardNormal.sample(rng))
    );

    // Compute L * z + mu
    Ok(l.dot(&z) + mu)
}

/// Compute the log of the probability density function at a given quantile
/// vector for a multivariate Gaussian distribution.
///
/// # Arguments
/// * `x` - Vector of quantiles (dimension d)
/// * `mu` - Mean vector (dimension d)
/// * `l` - Lower triangular matrix from Cholesky decomposition of variance-covariance matrix Sigma = L L^T (dimension d x d)
///
/// # Returns
/// Result containing the log PDF or an error string
pub fn multivariate_gaussian_log_pdf(
    x: &Array1<f64>,
    mu: &Array1<f64>,
    l: &Array2<f64>,
) -> Result<f64, String> {
    let m = l.nrows();
    let n = l.ncols();

    if m != n {
        return Err("requires square matrix".to_string());
    } else if mu.len() != m {
        return Err("incompatible dimension of mean vector with variance-covariance matrix".to_string());
    } else if x.len() != m {
        return Err("incompatible dimension of quantile vector".to_string());
    }

    // Compute x - mu
    let diff = x - mu;

    // Solve L * y = (x - mu) for y
    let y = l.solve(&diff).map_err(|e| e.to_string())?;

    // Compute quadratic form: (x - mu)^T Sigma^{-1} (x - mu) = y^T y
    let quad_form = y.dot(&y);

    // Compute log(sqrt(|Sigma|)) = sum(log(L_ii))
    let log_sqrt_det_sigma = l.diag().iter().map(|&x| x.ln()).sum::<f64>();

    Ok(-0.5 * quad_form - log_sqrt_det_sigma - 0.5 * (m as f64) * (2.0 * PI).ln())
}

/// Compute the probability density function at a given quantile vector
pub fn multivariate_gaussian_pdf(
    x: &Array1<f64>,
    mu: &Array1<f64>,
    l: &Array2<f64>,
) -> Result<f64, String> {
    multivariate_gaussian_log_pdf(x, mu, l).map(|logpdf| logpdf.exp())
}

/// Compute the maximum-likelihood estimate of the mean vector
pub fn multivariate_gaussian_mean(x: &Array2<f64>) -> Array1<f64> {
    x.mean_axis(ndarray::Axis(0)).unwrap()
}

/// Compute the maximum-likelihood estimate of the variance-covariance matrix
pub fn multivariate_gaussian_vcov(x: &Array2<f64>) -> Array2<f64> {
    let n_samples = x.nrows() as f64;
    let centered = x - &x.mean_axis(ndarray::Axis(0)).unwrap();
    centered.t().dot(&centered) / (n_samples - 1.0)
}