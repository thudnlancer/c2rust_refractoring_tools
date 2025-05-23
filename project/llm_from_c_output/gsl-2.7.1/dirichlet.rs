use rand::Rng;
use rand_distr::{Distribution, Gamma};
use std::f64::consts;
use std::f64;

/// The Dirichlet probability distribution of order K-1 is 
/// 
///    p(\theta_1,...,\theta_K) d\theta_1 ... d\theta_K = 
///       (1/Z) \prod_i=1,K \theta_i^{alpha_i - 1} \delta(1 -\sum_i=1,K \theta_i)
///
/// The normalization factor Z can be expressed in terms of gamma functions:
///
///     Z = {\prod_i=1,K \Gamma(\alpha_i)} / {\Gamma( \sum_i=1,K \alpha_i)}  
///
/// The K constants, \alpha_1,...,\alpha_K, must be positive. The K parameters, 
/// \theta_1,...,\theta_K are nonnegative and sum to 1.
///
/// The random variates are generated by sampling K values from gamma
/// distributions with parameters a=\alpha_i, b=1, and renormalizing. 
/// See A.M. Law, W.D. Kelton, Simulation Modeling and Analysis (1991).
///
/// Gavin E. Crooks <gec@compbio.berkeley.edu> (2002)

pub fn dirichlet<R: Rng>(rng: &mut R, k: usize, alpha: &[f64], theta: &mut [f64]) {
    let mut norm = 0.0;

    for i in 0..k {
        let gamma = Gamma::new(alpha[i], 1.0).unwrap();
        theta[i] = gamma.sample(rng);
    }

    for i in 0..k {
        norm += theta[i];
    }

    if norm < f64::MIN.sqrt() {  // Handle underflow
        dirichlet_small(rng, k, alpha, theta);
        return;
    }

    for i in 0..k {
        theta[i] /= norm;
    }
}

/// When the values of alpha[] are small, scale the variates to avoid
/// underflow so that the result is not 0/0. Note that the Dirichlet
/// distribution is defined by a ratio of gamma functions so we can
/// take out an arbitrary factor to keep the values in the range of
/// double precision.
fn dirichlet_small<R: Rng>(rng: &mut R, k: usize, alpha: &[f64], theta: &mut [f64]) {
    let mut norm = 0.0;
    let mut umax = 0.0;

    for i in 0..k {
        let u = rng.gen::<f64>().ln() / alpha[i];
        theta[i] = u;

        if u > umax || i == 0 {
            umax = u;
        }
    }

    for i in 0..k {
        theta[i] = (theta[i] - umax).exp();
    }

    for i in 0..k {
        let gamma = Gamma::new(alpha[i] + 1.0, 1.0).unwrap();
        theta[i] *= gamma.sample(rng);
    }

    for i in 0..k {
        norm += theta[i];
    }

    for i in 0..k {
        theta[i] /= norm;
    }
}

pub fn dirichlet_pdf(k: usize, alpha: &[f64], theta: &[f64]) -> f64 {
    dirichlet_lnpdf(k, alpha, theta).exp()
}

pub fn dirichlet_lnpdf(k: usize, alpha: &[f64], theta: &[f64]) -> f64 {
    // We calculate the log of the pdf to minimize the possibility of overflow
    let mut log_p = 0.0;
    let mut sum_alpha = 0.0;

    for i in 0..k {
        log_p += (alpha[i] - 1.0) * theta[i].ln();
    }

    for i in 0..k {
        sum_alpha += alpha[i];
    }

    log_p += ln_gamma(sum_alpha);

    for i in 0..k {
        log_p -= ln_gamma(alpha[i]);
    }

    log_p
}

// Approximation of ln(gamma(x)) using Lanczos approximation
fn ln_gamma(x: f64) -> f64 {
    if x < 0.5 {
        consts::PI.ln() - (consts::PI * x).sin().ln() - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let tmp = x + 5.5;
        let log = (2.0 * consts::PI).sqrt().ln() 
            + (x + 0.5) * tmp.ln() 
            - tmp 
            + 1.0 / (12.0 * (x + 1.0))
            - 1.0 / (360.0 * (x + 1.0).powi(3))
            + 1.0 / (1260.0 * (x + 1.0).powi(5));
        log
    }
}