use std::f64::consts::PI;

pub struct GslRng {
    // Assuming this is a safe wrapper around the C gsl_rng
    // In a real implementation, this would properly encapsulate the unsafe operations
    inner: Box<dyn GslRngImpl>,
}

trait GslRngImpl {
    fn uniform(&mut self) -> f64;
    fn gaussian(&mut self, sigma: f64) -> f64;
}

impl GslRng {
    pub fn uniform(&mut self) -> f64 {
        self.inner.uniform()
    }

    pub fn gaussian(&mut self, sigma: f64) -> f64 {
        self.inner.gaussian(sigma)
    }

    pub fn gaussian_tail(&mut self, a: f64, sigma: f64) -> f64 {
        let s = a / sigma;
        
        if s < 1.0 {
            loop {
                let x = self.gaussian(1.0);
                if x >= s {
                    return x * sigma;
                }
            }
        } else {
            loop {
                let u = self.uniform();
                let v = loop {
                    let v = self.uniform();
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

    pub fn ugaussian_tail(&mut self, a: f64) -> f64 {
        self.gaussian_tail(a, 1.0)
    }
}

pub fn gaussian_tail_pdf(x: f64, a: f64, sigma: f64) -> f64 {
    if x < a {
        0.0
    } else {
        let u = x / sigma;
        let f = erfc(a / (2.0f64.sqrt() * sigma));
        let n = 0.5 * f;
        let coefficient = 1.0 / (n * (2.0 * PI).sqrt() * sigma);
        coefficient * (-u * u / 2.0).exp()
    }
}

pub fn ugaussian_tail_pdf(x: f64, a: f64) -> f64 {
    gaussian_tail_pdf(x, a, 1.0)
}

// Safe wrapper around erfc function
fn erfc(x: f64) -> f64 {
    unsafe { libm::erfc(x) }
}