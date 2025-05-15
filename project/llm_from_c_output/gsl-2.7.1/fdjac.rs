use std::f64;
use std::cmp::max;

/// Compute approximate Jacobian using forward differences
///
/// # Arguments
///
/// * `x` - parameter vector
/// * `wts` - data weights
/// * `fdf` - fdf struct
/// * `f` - (input) vector of function values f_i(x)
/// * `j` - (output) Jacobian matrix
///
/// # Returns
///
/// * `Result<(), &'static str>` - success or error
fn fdjac(
    x: &mut [f64],
    wts: Option<&[f64]>,
    fdf: &mut dyn FnMut(&[f64], Option<&[f64]>, &mut [f64]) -> Result<(), &'static str>,
    f: &[f64],
    j: &mut [Vec<f64>],
) -> Result<(), &'static str> {
    let epsfcn = 0.0;
    let eps = (max(epsfcn, f64::EPSILON) as f64).sqrt();
    let n = f.len();
    let p = x.len();

    for j_col in 0..p {
        let xj = x[j_col];
        let h = eps * xj.abs();
        let h = if h == 0.0 { eps } else { h };

        // perturb x_j to compute forward difference
        x[j_col] = xj + h;

        // use column j_col of J as temporary storage for f(x + dx)
        let mut fnext = vec![0.0; n];
        fdf(x, wts, &mut fnext)?;

        // restore x_j
        x[j_col] = xj;

        let h_inv = 1.0 / h;
        for i in 0..n {
            j[i][j_col] = (fnext[i] - f[i]) * h_inv;
        }
    }

    Ok(())
}

/// Compute approximate Jacobian using finite differences
///
/// # Arguments
///
/// * `x` - parameter vector
/// * `wts` - data weights (set to None if not needed)
/// * `fdf` - fdf
/// * `f` - (input) function values f_i(x)
/// * `j` - (output) approximate Jacobian matrix
///
/// # Returns
///
/// * `Result<(), &'static str>` - success or error
pub fn multifit_fdfsolver_dif_df(
    x: &mut [f64],
    wts: Option<&[f64]>,
    fdf: &mut dyn FnMut(&[f64], Option<&[f64]>, &mut [f64]) -> Result<(), &'static str>,
    f: &[f64],
    j: &mut [Vec<f64>],
) -> Result<(), &'static str> {
    fdjac(x, wts, fdf, f, j)
}

/// Compute function values (analytic) and approximate Jacobian using finite differences
///
/// # Arguments
///
/// * `x` - parameter vector
/// * `fdf` - fdf
/// * `f` - (output) function values f_i(x)
/// * `j` - (output) approximate Jacobian matrix
///
/// # Returns
///
/// * `Result<(), &'static str>` - success or error
#[allow(deprecated)]
pub fn multifit_fdfsolver_dif_fdf(
    x: &mut [f64],
    fdf: &mut dyn FnMut(&[f64], Option<&[f64]>, &mut [f64]) -> Result<(), &'static str>,
    f: &mut [f64],
    j: &mut [Vec<f64>],
) -> Result<(), &'static str> {
    fdf(x, None, f)?;
    fdjac(x, None, fdf, f, j)
}