use rand::Rng;
use rand::distributions::{Distribution, Gamma, Uniform};
use std::f64::consts::E;

/// The poisson distribution has the form
/// 
/// p(n) = (mu^n / n!) exp(-mu) 
/// 
/// for n = 0, 1, 2, ... . The method used here is the one from Knuth.
pub fn gsl_ran_poisson<R: Rng>(rng: &mut R, mu: f64) -> u32 {
    let mut emu;
    let mut prod = 1.0;
    let mut k = 0;
    let mut mu = mu;

    while mu > 10.0 {
        let m = (mu * (7.0 / 8.0)) as u32;
        
        let gamma = Gamma::new(m as f64, 1.0);
        let x = gamma.sample(rng);
        
        if x >= mu {
            let p = mu / x;
            let binomial = gsl_ran_binomial(rng, p, m - 1);
            return k + binomial;
        } else {
            k += m;
            mu -= x;
        }
    }

    // This following method works well when mu is small
    emu = (-mu).exp();

    loop {
        let uniform = Uniform::new(0.0, 1.0);
        prod *= uniform.sample(rng);
        k += 1;
        if prod <= emu {
            break;
        }
    }

    k - 1
}

pub fn gsl_ran_poisson_array<R: Rng>(rng: &mut R, n: usize, array: &mut [u32], mu: f64) {
    for item in array.iter_mut().take(n) {
        *item = gsl_ran_poisson(rng, mu);
    }
}

pub fn gsl_ran_poisson_pdf(k: u32, mu: f64) -> f64 {
    let lf = ln_fact(k);
    (mu.ln() * (k as f64) - lf - mu).exp()
}

// Helper functions
fn gsl_ran_binomial<R: Rng>(rng: &mut R, p: f64, n: u32) -> u32 {
    let uniform = Uniform::new(0.0, 1.0);
    let mut successes = 0;
    
    for _ in 0..n {
        if uniform.sample(rng) < p {
            successes += 1;
        }
    }
    
    successes
}

fn ln_fact(k: u32) -> f64 {
    if k == 0 {
        0.0
    } else {
        let mut res = 0.0;
        for i in 1..=k {
            res += (i as f64).ln();
        }
        res
    }
}