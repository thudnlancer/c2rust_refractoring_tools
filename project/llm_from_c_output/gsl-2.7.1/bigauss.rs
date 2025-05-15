//! Bivariate Gaussian distribution implementation translated from C to Rust.

use std::f64::consts::PI;
use rand::Rng;

/// Generates two correlated Gaussian random variables using the Box-Muller method.
///
/// # Arguments
/// * `rng` - Random number generator implementing Rng trait
/// * `sigma_x` - Standard deviation in x direction
/// * `sigma_y` - Standard deviation in y direction
/// * `rho` - Correlation coefficient between x and y
///
/// # Returns
/// A tuple of (x, y) values sampled from the bivariate Gaussian distribution
pub fn bivariate_gaussian<R: Rng>(
    rng: &mut R,
    sigma_x: f64,
    sigma_y: f64,
    rho: f64,
) -> (f64, f64) {
    let (u, v) = loop {
        // choose x,y in uniform square (-1,-1) to (+1,+1)
        let u = -1.0 + 2.0 * rng.gen::<f64>();
        let v = -1.0 + 2.0 * rng.gen::<f64>();
        
        // see if it is in the unit circle
        let r2 = u * u + v * v;
        if r2 <= 1.0 && r2 != 0.0 {
            break (u, v);
        }
    };

    let scale = (-2.0 * r2.ln() / r2).sqrt();
    let x = sigma_x * u * scale;
    let y = sigma_y * (rho * u + (1.0 - rho * rho).sqrt() * v) * scale;
    
    (x, y)
}

/// Calculates the probability density function of the bivariate Gaussian distribution.
///
/// # Arguments
/// * `x` - x coordinate
/// * `y` - y coordinate
/// * `sigma_x` - Standard deviation in x direction
/// * `sigma_y` - Standard deviation in y direction
/// * `rho` - Correlation coefficient between x and y
///
/// # Returns
/// The probability density at point (x, y)
pub fn bivariate_gaussian_pdf(
    x: f64,
    y: f64,
    sigma_x: f64,
    sigma_y: f64,
    rho: f64,
) -> f64 {
    let u = x / sigma_x;
    let v = y / sigma_y;
    let c = 1.0 - rho * rho;
    let exponent = -(u * u - 2.0 * rho * u * v + v * v) / (2.0 * c);
    (1.0 / (2.0 * PI * sigma_x * sigma_y * c.sqrt())) * exponent.exp()
}