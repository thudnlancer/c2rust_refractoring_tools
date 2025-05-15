use std::f64::consts::PI;

pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub get_double: fn(&mut dyn RngState) -> f64,
}

pub trait RngState {}

pub struct GslRng<'a> {
    pub rng_type: &'a GslRngType,
    pub state: &'a mut dyn RngState,
}

impl<'a> GslRng<'a> {
    fn uniform_pos(&mut self) -> f64 {
        loop {
            let x = (self.rng_type.get_double)(self.state);
            if x != 0.0 {
                return x;
            }
        }
    }

    fn uniform(&mut self) -> f64 {
        (self.rng_type.get_double)(self.state)
    }

    pub fn gaussian(&mut self, sigma: f64) -> f64 {
        let mut x;
        let mut y;
        let mut r2;

        loop {
            x = -1.0 + 2.0 * self.uniform_pos();
            y = -1.0 + 2.0 * self.uniform_pos();
            r2 = x * x + y * y;
            if r2 <= 1.0 && r2 != 0.0 {
                break;
            }
        }

        sigma * y * (-2.0 * r2.ln() / r2).sqrt()
    }

    pub fn gaussian_ratio_method(&mut self, sigma: f64) -> f64 {
        let s = 0.449871;
        let t = -0.386595;
        let a = 0.19600;
        let b = 0.25472;
        let r1 = 0.27597;
        let r2 = 0.27846;

        loop {
            let u = 1.0 - self.uniform();
            let mut v = self.uniform() - 0.5;
            v *= 1.7156;

            let x = u - s;
            let y = v.abs() - t;
            let q = x * x + y * (a * y - b * x);

            if q < r1 || (q <= r2 && v * v <= -4.0 * u * u * u.ln()) {
                return sigma * (v / u);
            }
        }
    }

    pub fn ugaussian(&mut self) -> f64 {
        self.gaussian(1.0)
    }

    pub fn ugaussian_ratio_method(&mut self) -> f64 {
        self.gaussian_ratio_method(1.0)
    }
}

pub fn gaussian_pdf(x: f64, sigma: f64) -> f64 {
    let u = x / sigma.abs();
    1.0 / ((2.0 * PI).sqrt() * sigma.abs()) * (-u * u / 2.0).exp()
}

pub fn ugaussian_pdf(x: f64) -> f64 {
    gaussian_pdf(x, 1.0)
}