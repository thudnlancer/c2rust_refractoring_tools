use std::f64::consts::E;
use std::ops::SubAssign;

pub struct GslRng {
    // Assuming this is a safe wrapper around the C GSL RNG
    // Implementation details would depend on the actual GSL bindings
}

impl GslRng {
    pub fn uniform(&self) -> f64 {
        // Safe wrapper around gsl_rng_uniform
        unimplemented!()
    }

    pub fn gamma_int(&self, a: u32) -> f64 {
        // Safe wrapper around gsl_ran_gamma_int
        unimplemented!()
    }

    pub fn binomial(&self, p: f64, n: u32) -> u32 {
        // Safe wrapper around gsl_ran_binomial
        unimplemented!()
    }
}

pub fn poisson(rng: &GslRng, mu: f64) -> u32 {
    let mut emu;
    let mut prod = 1.0;
    let mut k = 0;
    let mut mu = mu;

    while mu > 10.0 {
        let m = (mu * (7.0 / 8.0)) as u32;
        let x = rng.gamma_int(m);
        
        if x >= mu {
            return k + rng.binomial(mu / x, m - 1);
        } else {
            k += m;
            mu -= x;
        }
    }

    emu = (-mu).exp();
    loop {
        prod *= rng.uniform();
        k += 1;
        if prod <= emu {
            break;
        }
    }
    k - 1
}

pub fn poisson_array(rng: &GslRng, n: usize, array: &mut [u32], mu: f64) {
    for item in array.iter_mut().take(n) {
        *item = poisson(rng, mu);
    }
}

pub fn poisson_pdf(k: u32, mu: f64) -> f64 {
    let lf = ln_fact(k);
    (mu.ln() * k as f64 - lf - mu).exp()
}

fn ln_fact(n: u32) -> f64 {
    // Safe implementation of gsl_sf_lnfact
    // Could use a lookup table or approximation for better performance
    let mut res = 0.0;
    for i in 1..=n {
        res += (i as f64).ln();
    }
    res
}