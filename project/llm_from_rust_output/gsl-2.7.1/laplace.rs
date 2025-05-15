use std::f64::consts::E;

pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub get_double: fn() -> f64,
}

pub struct GslRng {
    pub rng_type: &'static GslRngType,
}

impl GslRng {
    pub fn uniform(&self) -> f64 {
        (self.rng_type.get_double)()
    }

    pub fn laplace(&self, a: f64) -> f64 {
        let mut u;
        loop {
            u = 2.0 * self.uniform() - 1.0;
            if u != 0.0 {
                break;
            }
        }
        if u < 0.0 {
            a * (-u).ln()
        } else {
            -a * u.ln()
        }
    }
}

pub fn laplace_pdf(x: f64, a: f64) -> f64 {
    1.0 / (2.0 * a) * (-x.abs() / a).exp()
}