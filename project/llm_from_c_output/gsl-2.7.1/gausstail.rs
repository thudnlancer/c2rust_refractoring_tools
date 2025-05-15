use rand::Rng;
use std::f64::consts::{PI, SQRT_2};
use std::f64;

/// Returns a gaussian random variable larger than a
/// This implementation does one-sided upper-tailed deviates.
pub fn gaussian_tail<R: Rng>(rng: &mut R, a: f64, sigma: f64) -> f64 {
    let s = a / sigma;

    if s < 1.0 {
        // For small s, use a direct rejection method. The limit s < 1
        // can be adjusted to optimise the overall efficiency
        loop {
            let x = rng.sample(rand_distr::StandardNormal);
            if x >= s {
                return x * sigma;
            }
        }
    } else {
        // Use the "supertail" deviates from the last two steps
        // of Marsaglia's rectangle-wedge-tail method, as described
        // in Knuth, v2, 3rd ed, pp 123-128.  (See also exercise 11, p139,
        // and the solution, p586.)
        loop {
            let u = rng.gen::<f64>();
            let v = loop {
                let v = rng.gen::<f64>();
                if v != 0.0 {
                    break v;
                }
            };
            let x = (s * s - 2.0 * v.ln()).sqrt();
            if x * u <= s {
                return x * sigma;
            }
        }
    }
}

pub fn gaussian_tail_pdf(x: f64, a: f64, sigma: f64) -> f64 {
    if x < a {
        0.0
    } else {
        let u = x / sigma;
        let f = erfc(a / (SQRT_2 * sigma));
        let n = 0.5 * f;
        (1.0 / (n * (2.0 * PI).sqrt() * sigma)) * (-u * u / 2.0).exp()
    }
}

pub fn ugaussian_tail<R: Rng>(rng: &mut R, a: f64) -> f64 {
    gaussian_tail(rng, a, 1.0)
}

pub fn ugaussian_tail_pdf(x: f64, a: f64) -> f64 {
    gaussian_tail_pdf(x, a, 1.0)
}

// Complementary error function approximation
fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t * (-z * z - 1.26551223 + t * (1.00002368 + t * (0.37409196 +
             t * (0.09678418 + t * (-0.18628806 + t * (0.27886807 +
             t * (-1.13520398 + t * (1.48851587 + t * (-0.82215223 +
             t * 0.17087277))))))))).exp();
    if x < 0.0 { 2.0 - r } else { r }
}