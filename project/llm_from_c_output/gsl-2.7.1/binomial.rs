use rand::Rng;
use rand_distr::{Beta, Distribution};
use std::f64::consts::E;

/// The binomial distribution has the form,
///
/// prob(k) =  n!/(k!(n-k)!) *  p^k (1-p)^(n-k) for k = 0, 1, ..., n
///
/// This is the algorithm from Knuth
pub fn binomial_knuth<R: Rng>(rng: &mut R, p: f64, n: u32) -> u32 {
    let mut k = 0;
    let mut n = n;
    let mut p = p;

    while n > 10 {
        let a = 1 + (n / 2);
        let b = 1 + n - a;

        let beta = Beta::new(a as f64, b as f64).unwrap();
        let x = beta.sample(rng);

        if x >= p {
            n = a - 1;
            p /= x;
        } else {
            k += a;
            n = b - 1;
            p = (p - x) / (1.0 - x);
        }
    }

    for _ in 0..n {
        let u = rng.gen::<f64>();
        if u < p {
            k += 1;
        }
    }

    k
}

pub fn binomial_pdf(k: u32, p: f64, n: u32) -> f64 {
    if k > n {
        0.0
    } else {
        if p == 0.0 {
            if k == 0 { 1.0 } else { 0.0 }
        } else if p == 1.0 {
            if k == n { 1.0 } else { 0.0 }
        } else {
            let ln_cnk = ln_choose(n as f64, k as f64);
            let log_p = p.ln();
            let log_one_minus_p = (1.0 - p).ln();
            let p = ln_cnk + (k as f64) * log_p + (n - k) as f64 * log_one_minus_p;
            E.powf(p)
        }
    }
}

fn ln_choose(n: f64, k: f64) -> f64 {
    (n + 1.0).ln_gamma().0 - (k + 1.0).ln_gamma().0 - (n - k + 1.0).ln_gamma().0
}