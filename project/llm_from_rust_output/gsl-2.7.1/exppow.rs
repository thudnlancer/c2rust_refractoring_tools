use std::f64::consts::SQRT_2;
use libm::{exp, log, pow, sqrt, fabs};

pub struct GslRng {
    // Assuming this is a safe wrapper around the GSL RNG
    // In a real implementation, this would properly encapsulate the unsafe code
    inner: Box<dyn RngCore>,
}

impl GslRng {
    pub fn uniform(&mut self) -> f64 {
        self.inner.next_f64()
    }

    pub fn uniform_pos(&mut self) -> f64 {
        loop {
            let x = self.uniform();
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn gaussian(&mut self, sigma: f64) -> f64 {
        // Implementation of Box-Muller transform
        let u1 = self.uniform_pos();
        let u2 = self.uniform_pos();
        sigma * (-2.0 * log(u1)).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }

    pub fn laplace(&mut self, a: f64) -> f64 {
        let u = self.uniform() - 0.5;
        -a * u.signum() * log(1.0 - 2.0 * u.abs())
    }

    pub fn gamma(&mut self, a: f64, b: f64) -> f64 {
        // Marsaglia and Tsang's method for gamma distribution
        if a < 1.0 {
            let u = self.uniform();
            self.gamma(1.0 + a, b) * pow(u, 1.0 / a)
        } else {
            let d = a - 1.0 / 3.0;
            let c = 1.0 / sqrt(9.0 * d);
            loop {
                let mut x;
                let mut v;
                loop {
                    x = self.gaussian(1.0);
                    v = 1.0 + c * x;
                    if v > 0.0 {
                        break;
                    }
                }
                v = v * v * v;
                let u = self.uniform();
                if u < 1.0 - 0.0331 * (x * x) * (x * x)
                    || log(u) < 0.5 * x * x + d * (1.0 - v + log(v))
                {
                    return b * d * v;
                }
            }
        }
    }
}

pub trait RngCore {
    fn next_f64(&mut self) -> f64;
}

pub fn exppow(rng: &mut GslRng, a: f64, b: f64) -> f64 {
    if !(1.0..=4.0).contains(&b) {
        let u = rng.uniform();
        let v = rng.gamma(1.0 / b, 1.0);
        let z = a * pow(v, 1.0 / b);
        if u > 0.5 { z } else { -z }
    } else if b == 1.0 {
        rng.laplace(a)
    } else if b < 2.0 {
        let b_inv = 1.0 / b;
        let b_pow = pow(b_inv, b_inv);
        loop {
            let x = rng.laplace(b_pow);
            let u = rng.uniform_pos();
            let h = -pow(fabs(x), b) + fabs(x) / b_pow - 1.0 + b_inv;
            if log(u) <= h {
                return a * x;
            }
        }
    } else if b == 2.0 {
        rng.gaussian(a / SQRT_2)
    } else {
        let b_inv = 1.0 / b;
        let b_pow = pow(b_inv, b_inv);
        loop {
            let x = rng.gaussian(b_pow);
            let u = rng.uniform_pos();
            let h = -pow(fabs(x), b) 
                + x * x / (2.0 * b_pow * b_pow) 
                + b_inv - 0.5;
            if log(u) <= h {
                return a * x;
            }
        }
    }
}

pub fn exppow_pdf(x: f64, a: f64, b: f64) -> f64 {
    let lngamma = lngamma(1.0 + 1.0 / b);
    1.0 / (2.0 * a) * exp(-pow(fabs(x / a), b) - lngamma)
}

// Approximation of log gamma function
fn lngamma(x: f64) -> f64 {
    // Lanczos approximation would be better here
    x.ln() - x + (2.0 * std::f64::consts::PI / x).sqrt().ln() / 2.0
        + x * (1.0 + 1.0 / (12.0 * x) + 1.0 / (288.0 * x * x)).ln()
}