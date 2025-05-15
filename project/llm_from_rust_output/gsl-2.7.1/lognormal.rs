use std::f64::consts::PI;

pub struct GslRngType {
    pub name: String,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub get_double: fn(&mut ()) -> f64,
}

pub struct GslRng {
    pub rng_type: GslRngType,
    pub state: Box<dyn std::any::Any>,
}

impl GslRng {
    pub fn uniform(&mut self) -> f64 {
        let state = self.state.downcast_mut::<()>().unwrap();
        (self.rng_type.get_double)(state)
    }

    pub fn lognormal(&mut self, zeta: f64, sigma: f64) -> f64 {
        let (u, v) = loop {
            let u = -1.0 + 2.0 * self.uniform();
            let v = -1.0 + 2.0 * self.uniform();
            let r2 = u * u + v * v;
            if r2 <= 1.0 && r2 != 0.0 {
                break (u, v);
            }
        };

        let r2 = u * u + v * v;
        let normal = u * (-2.0 * r2.ln() / r2).sqrt();
        (sigma * normal + zeta).exp()
    }
}

pub fn lognormal_pdf(x: f64, zeta: f64, sigma: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        let u = (x.ln() - zeta) / sigma;
        let coefficient = 1.0 / (x * sigma.abs() * (2.0 * PI).sqrt());
        coefficient * (-u * u / 2.0).exp()
    }
}