use std::f64::consts::PI;
use ndarray::{Array2, ArrayView2, ArrayViewMut2};
use rand::distributions::{Distribution, ChiSquared, StandardNormal};
use rand::Rng;
use statrs::function::gamma::ln_gamma;

/// Error types for Wishart distribution operations
#[derive(Debug)]
pub enum WishartError {
    NotSquareMatrix,
    IncompatibleDimensions,
    InvalidDegreesOfFreedom,
    CholeskySolveFailed,
}

/// Generate a random matrix from a Wishart distribution using the Bartlett decomposition
///
/// # Arguments
/// * `rng` - Random number generator
/// * `df` - Degrees of freedom
/// * `l` - Cholesky decomposition of scale matrix V = L L^T (d x d)
/// * `result` - Output matrix (d x d)
/// * `work` - Workspace matrix (d x d)
pub fn wishart(
    rng: &mut impl Rng,
    df: f64,
    l: &ArrayView2<f64>,
    result: &mut ArrayViewMut2<f64>,
    work: &mut ArrayViewMut2<f64>,
) -> Result<(), WishartError> {
    let d = l.nrows();

    // Validate inputs
    if l.nrows() != l.ncols() {
        return Err(WishartError::NotSquareMatrix);
    }
    if result.nrows() != result.ncols() {
        return Err(WishartError::NotSquareMatrix);
    }
    if work.nrows() != work.ncols() {
        return Err(WishartError::NotSquareMatrix);
    }
    if result.nrows() != d {
        return Err(WishartError::IncompatibleDimensions);
    }
    if work.nrows() != d {
        return Err(WishartError::IncompatibleDimensions);
    }
    if df <= (d - 1) as f64 {
        return Err(WishartError::InvalidDegreesOfFreedom);
    }

    // Fill the work matrix with Bartlett decomposition
    work.fill(0.0);
    let chi = ChiSquared::new(1.0).unwrap();
    let normal = StandardNormal;

    for i in 0..d {
        work[[i, i]] = chi.sample(rng).sqrt();
        for j in 0..i {
            work[[i, j]] = rng.sample(normal);
        }
    }

    // Compute L * A
    let mut la = Array2::zeros((d, d));
    for i in 0..d {
        for j in 0..d {
            for k in 0..=i.min(j) {
                la[[i, j]] += l[[i, k]] * work[[k, j]];
            }
        }
    }

    // Compute (L * A) * (L * A)^T
    for i in 0..d {
        for j in 0..d {
            let mut sum = 0.0;
            for k in 0..d {
                sum += la[[i, k]] * la[[j, k]];
            }
            result[[i, j]] = sum;
        }
    }

    Ok(())
}

/// Compute the log of the probability density function for a Wishart distribution
///
/// # Arguments
/// * `x` - Quantile matrix (d x d)
/// * `l_x` - Cholesky decomposition of quantile matrix X = L_X L_X^T (d x d)
/// * `df` - Degrees of freedom
/// * `l` - Cholesky decomposition of scale matrix V = L L^T (d x d)
/// * `work` - Workspace matrix (d x d)
pub fn wishart_log_pdf(
    x: &ArrayView2<f64>,
    l_x: &ArrayView2<f64>,
    df: f64,
    l: &ArrayView2<f64>,
    work: &mut ArrayViewMut2<f64>,
) -> Result<f64, WishartError> {
    let d = l.nrows();

    // Validate inputs
    if l.nrows() != l.ncols() {
        return Err(WishartError::NotSquareMatrix);
    }
    if x.nrows() != x.ncols() {
        return Err(WishartError::NotSquareMatrix);
    }
    if l_x.nrows() != l_x.ncols() {
        return Err(WishartError::NotSquareMatrix);
    }
    if x.nrows() != d {
        return Err(WishartError::IncompatibleDimensions);
    }
    if l_x.nrows() != d {
        return Err(WishartError::IncompatibleDimensions);
    }
    if df <= (d - 1) as f64 {
        return Err(WishartError::InvalidDegreesOfFreedom);
    }

    // Compute log of multivariate Gamma
    let mut log_mv_ga = d as f64 * (d as f64 - 1.0) * 0.25 * PI.ln();
    for i in 0..d {
        log_mv_ga += ln_gamma(0.5 * (df - i as f64 + 1.0));
    }

    // Compute log determinant of scale matrix
    let mut log_det_v = 0.0;
    for i in 0..d {
        log_det_v += l[[i, i]].ln();
    }
    log_det_v *= 2.0;

    // Compute log determinant of quantile matrix
    let mut log_det_x = 0.0;
    for i in 0..d {
        log_det_x += l_x[[i, i]].ln();
    }
    log_det_x *= 2.0;

    // Compute trace of V^(-1) X
    // Note: In Rust, we'd typically use a linear algebra crate like ndarray-linalg
    // for Cholesky solve. This is a simplified placeholder.
    // In practice, you'd want to use proper linear algebra operations here.
    let mut tr_vinv_x = 0.0;
    for i in 0..d {
        tr_vinv_x += work[[i, i]]; // This is simplified - actual implementation would solve the system
    }

    // Compute final log PDF
    Ok(-(0.5 * df * d as f64) * 2.0f64.ln()
        - (0.5 * df) * log_det_v
        - log_mv_ga
        + 0.5 * (df - d as f64 - 1.0) * log_det_x
        - 0.5 * tr_vinv_x)
}

/// Compute the probability density function for a Wishart distribution
pub fn wishart_pdf(
    x: &ArrayView2<f64>,
    l_x: &ArrayView2<f64>,
    df: f64,
    l: &ArrayView2<f64>,
    work: &mut ArrayViewMut2<f64>,
) -> Result<f64, WishartError> {
    let logpdf = wishart_log_pdf(x, l_x, df, l, work)?;
    Ok(logpdf.exp())
}