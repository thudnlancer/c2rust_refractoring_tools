use std::f64::consts::PI;

pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub get_double: fn(&mut ()) -> f64,
}

pub struct GslRng<'a> {
    pub type_: &'a GslRngType,
    pub state: &'a mut (),
}

impl<'a> GslRng<'a> {
    pub fn uniform(&mut self) -> f64 {
        (self.type_.get_double)(self.state)
    }
}

pub fn bivariate_gaussian(
    rng: &mut GslRng,
    sigma_x: f64,
    sigma_y: f64,
    rho: f64,
) -> (f64, f64) {
    let (u, v, r2) = loop {
        let u = -1.0 + 2.0 * rng.uniform();
        let v = -1.0 + 2.0 * rng.uniform();
        let r2 = u * u + v * v;
        if r2 < 1.0 && r2 != 0.0 {
            break (u, v, r2);
        }
    };

    let scale = (-2.0 * r2.ln() / r2).sqrt();
    let x = sigma_x * u * scale;
    let y = sigma_y * (rho * u + (1.0 - rho * rho).sqrt() * v) * scale;

    (x, y)
}

pub fn bivariate_gaussian_pdf(
    x: f64,
    y: f64,
    sigma_x: f64,
    sigma_y: f64,
    rho: f64,
) -> f64 {
    let u = x / sigma_x;
    let v = y / sigma_y;
    let c = 1.0 - rho * rho;
    let exponent = -(u * u - 2.0 * rho * u * v + v * v) / (2.0 * c);
    
    1.0 / (2.0 * PI * sigma_x * sigma_y * c.sqrt()) * exponent.exp()
}