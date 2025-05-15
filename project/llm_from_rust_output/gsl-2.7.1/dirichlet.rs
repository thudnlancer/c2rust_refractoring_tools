use std::f64;
use rand::Rng;
use rand_distr::{Gamma, Distribution};

pub struct GslRng {
    // Assuming this is a wrapper around a safe RNG
    rng: Box<dyn RngCore>,
}

impl GslRng {
    pub fn uniform_pos(&mut self) -> f64 {
        loop {
            let x = self.rng.gen::<f64>();
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn gamma(&mut self, a: f64, b: f64) -> f64 {
        let gamma = Gamma::new(a, b).unwrap();
        gamma.sample(&mut self.rng)
    }
}

pub fn dirichlet(rng: &mut GslRng, alpha: &[f64], theta: &mut [f64]) {
    assert_eq!(alpha.len(), theta.len());
    let k = alpha.len();
    let mut norm = 0.0;

    // Generate gamma-distributed samples
    for i in 0..k {
        theta[i] = rng.gamma(alpha[i], 1.0);
        norm += theta[i];
    }

    if norm < 1.4916681462400413e-154 {
        dirichlet_small(rng, alpha, theta);
        return;
    }

    // Normalize
    for val in theta.iter_mut() {
        *val /= norm;
    }
}

fn dirichlet_small(rng: &mut GslRng, alpha: &[f64], theta: &mut [f64]) {
    let k = alpha.len();
    let mut norm = 0.0;
    let mut umax = f64::NEG_INFINITY;

    // First pass: compute log-transformed values and find max
    for i in 0..k {
        let u = rng.uniform_pos().ln() / alpha[i];
        theta[i] = u;
        if u > umax {
            umax = u;
        }
    }

    // Second pass: exponentiate and scale
    for i in 0..k {
        theta[i] = (theta[i] - umax).exp();
        theta[i] *= rng.gamma(alpha[i] + 1.0, 1.0);
        norm += theta[i];
    }

    // Normalize
    for val in theta.iter_mut() {
        *val /= norm;
    }
}

pub fn dirichlet_pdf(alpha: &[f64], theta: &[f64]) -> f64 {
    dirichlet_lnpdf(alpha, theta).exp()
}

pub fn dirichlet_lnpdf(alpha: &[f64], theta: &[f64]) -> f64 {
    assert_eq!(alpha.len(), theta.len());
    let mut log_p = 0.0;
    let mut sum_alpha = 0.0;

    for (&a, &t) in alpha.iter().zip(theta.iter()) {
        log_p += (a - 1.0) * t.ln();
        sum_alpha += a;
    }

    log_p += ln_gamma(sum_alpha);
    
    for &a in alpha.iter() {
        log_p -= ln_gamma(a);
    }

    log_p
}

// Placeholder for gsl_sf_lngamma
fn ln_gamma(x: f64) -> f64 {
    // In a real implementation, this would call a proper gamma function
    // For now, we'll use Rust's built-in gamma function
    x.ln_gamma().0
}