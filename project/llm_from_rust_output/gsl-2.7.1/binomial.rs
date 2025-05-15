use std::f64::consts::{E, LN_2};
use rand::Rng;
use rand_distr::{Beta, Distribution};

pub struct GslRng<R: Rng>(R);

impl<R: Rng> GslRng<R> {
    pub fn new(rng: R) -> Self {
        GslRng(rng)
    }

    pub fn uniform(&mut self) -> f64 {
        self.0.gen()
    }

    pub fn beta(&mut self, a: f64, b: f64) -> f64 {
        Beta::new(a, b).unwrap().sample(&mut self.0)
    }
}

pub fn binomial_knuth(rng: &mut impl Rng, p: f64, n: u32) -> u32 {
    let mut r = GslRng(rng);
    let mut k = 0;
    let mut p = p;
    let mut n = n;

    while n > 10 {
        let a = 1 + n / 2;
        let b = 1 + n - a;
        let x = r.beta(a as f64, b as f64);

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
        let u = r.uniform();
        if u < p {
            k += 1;
        }
    }

    k
}

pub fn binomial_pdf(k: u32, p: f64, n: u32) -> f64 {
    if k > n {
        0.0
    } else if p == 0.0 {
        if k == 0 { 1.0 } else { 0.0 }
    } else if p == 1.0 {
        if k == n { 1.0 } else { 0.0 }
    } else {
        let ln_cnk = ln_choose(n, k);
        let term = ln_cnk + k as f64 * p.ln() + (n - k) as f64 * (1.0 - p).ln();
        term.exp()
    }
}

fn ln_choose(n: u32, k: u32) -> f64 {
    if k > n {
        0.0
    } else if k == 0 || k == n {
        0.0
    } else {
        let k = std::cmp::min(k, n - k);
        let mut r = 0.0;
        for i in 1..=k {
            r += ((n - k + i) as f64).ln() - (i as f64).ln();
        }
        r
    }
}