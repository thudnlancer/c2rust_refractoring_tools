use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::Binomial;
use std::f64::consts::E;

/// The multinomial distribution has the form
///
///                                      N!           n_1  n_2      n_K
///    prob(n_1, n_2, ... n_K) = -------------------- p_1  p_2  ... p_K
///                              (n_1! n_2! ... n_K!) 
///
/// where n_1, n_2, ... n_K are nonnegative integers, sum_{k=1,K} n_k = N,
/// and p = (p_1, p_2, ..., p_K) is a probability distribution. 
///
/// Random variates are generated using the conditional binomial method.
/// This scales well with N and does not require a setup step.
///
/// Ref: 
/// C.S. David, The computer generation of multinomial random variates,
/// Comp. Stat. Data Anal. 16 (1993) 205-217
pub fn multinomial<R: Rng>(
    rng: &mut R,
    k: usize,
    n: u32,
    p: &[f64],
) -> Vec<u32> {
    let mut norm = 0.0;
    let mut sum_p = 0.0;
    let mut sum_n = 0;
    let mut result = vec![0; k];

    // p[k] may contain non-negative weights that do not sum to 1.0
    // Even a probability distribution will not exactly sum to 1.0
    // due to rounding errors.
    for &prob in p {
        norm += prob;
    }

    for i in 0..k {
        if p[i] > 0.0 {
            let remaining = n - sum_n;
            let prob = p[i] / (norm - sum_p);
            let binom = Binomial::new(remaining as u64, prob).unwrap();
            result[i] = binom.sample(rng) as u32;
        } else {
            result[i] = 0;
        }

        sum_p += p[i];
        sum_n += result[i];
    }

    result
}

pub fn multinomial_pdf(k: usize, p: &[f64], n: &[u32]) -> f64 {
    E.powf(multinomial_lnpdf(k, p, n))
}

pub fn multinomial_lnpdf(k: usize, p: &[f64], n: &[u32]) -> f64 {
    let mut total_n = 0;
    let mut norm = 0.0;
    let mut log_pdf = 0.0;

    for &count in n {
        total_n += count;
    }

    for &prob in p {
        norm += prob;
    }

    log_pdf = ln_factorial(total_n);

    for i in 0..k {
        // Handle case where n[k]==0 and p[k]==0
        if n[i] > 0 {
            log_pdf += (p[i] / norm).ln() * f64::from(n[i]) - ln_factorial(n[i]);
        }
    }

    log_pdf
}

fn ln_factorial(n: u32) -> f64 {
    if n <= 1 {
        0.0
    } else {
        let n = n as f64;
        n * n.ln() - n + (2.0 * std::f64::consts::PI * n).ln() / 2.0 
            + 1.0 / (12.0 * n) - 1.0 / (360.0 * n.powi(3))
    }
}