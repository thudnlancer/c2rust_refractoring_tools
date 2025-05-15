use rand::distributions::{Distribution, Exponential, StandardNormal};
use rand::Rng;
use std::f64::consts::PI;

/// The t-distribution has the form
///
/// p(x) dx = (Gamma((nu + 1)/2)/(sqrt(pi nu) Gamma(nu/2))
/// * (1 + (x^2)/nu)^-((nu + 1)/2) dx
///
/// The method used here is the one described in Knuth
pub fn t_distribution<R: Rng>(rng: &mut R, nu: f64) -> f64 {
    if nu <= 2.0 {
        let y1: f64 = StandardNormal.sample(rng);
        let y2 = chi_squared(rng, nu);
        y1 / (y2 / nu).sqrt()
    } else {
        loop {
            let y1: f64 = StandardNormal.sample(rng);
            let exp = Exponential::new(1.0 / (nu / 2.0 - 1.0));
            let y2 = exp.sample(rng);
            let z = y1 * y1 / (nu - 2.0);
            
            if 1.0 - z >= 0.0 && (-y2 - z).exp() <= (1.0 - z) {
                // Note that there is a typo in Knuth's formula, the line below
                // is taken from the original paper of Marsaglia, Mathematics of
                // Computation, 34 (1980), p 234-256
                return y1 / ((1.0 - 2.0 / nu) * (1.0 - z)).sqrt();
            }
        }
    }
}

pub fn t_distribution_pdf(x: f64, nu: f64) -> f64 {
    let lg1 = ln_gamma(nu / 2.0);
    let lg2 = ln_gamma((nu + 1.0) / 2.0);
    ((lg2 - lg1).exp() / (PI * nu).sqrt()) * (1.0 + x * x / nu).powf(-(nu + 1.0) / 2.0)
}

// Helper functions to replace GSL functions
fn chi_squared<R: Rng>(rng: &mut R, nu: f64) -> f64 {
    let normal = StandardNormal;
    (0..nu as usize).map(|_| normal.sample(rng).powi(2)).sum()
}

fn ln_gamma(x: f64) -> f64 {
    // Approximation of ln(Gamma(x)) using Lanczos approximation
    // This is a simplified version - for production use consider a more accurate implementation
    let x = x - 1.0;
    let tmp = x + 5.5;
    let log = (2.0 * PI).sqrt().ln() 
        + (x + 0.5) * tmp.ln() 
        - tmp 
        + (1.0 + 76.18009173 / (x + 1.0) 
           - 86.50532033 / (x + 2.0) 
           + 24.01409822 / (x + 3.0) 
           - 1.231739516 / (x + 4.0) 
           + 0.00120858003 / (x + 5.0) 
           - 0.00000536382 / (x + 6.0)).ln();
    log
}