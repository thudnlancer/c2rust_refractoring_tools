use std::f64::consts::TAU;

pub type SizeT = usize;

pub struct GslRng {
    // Assuming this is a safe wrapper around the C GSL RNG
    // In a real implementation, this would properly encapsulate the unsafe code
    inner: Box<dyn GslRngImpl>,
}

trait GslRngImpl {
    fn uniform(&self) -> f64;
    fn gaussian(&self, sigma: f64) -> f64;
}

impl GslRng {
    pub fn uniform(&self) -> f64 {
        self.inner.uniform()
    }

    pub fn gaussian(&self, sigma: f64) -> f64 {
        self.inner.gaussian(sigma)
    }

    pub fn dir_2d(&self) -> (f64, f64) {
        let (u, v, s) = loop {
            let u = -1.0 + 2.0 * self.uniform();
            let v = -1.0 + 2.0 * self.uniform();
            let s = u * u + v * v;
            if s <= 1.0 && s != 0.0 {
                break (u, v, s);
            }
        };
        let x = (u * u - v * v) / s;
        let y = 2.0 * u * v / s;
        (x, y)
    }

    pub fn dir_2d_trig_method(&self) -> (f64, f64) {
        let t = TAU * self.uniform();
        (t.cos(), t.sin())
    }

    pub fn dir_3d(&self) -> (f64, f64, f64) {
        let (x, y, s) = loop {
            let x = -1.0 + 2.0 * self.uniform();
            let y = -1.0 + 2.0 * self.uniform();
            let s = x * x + y * y;
            if s <= 1.0 {
                break (x, y, s);
            }
        };
        let z = -1.0 + 2.0 * s;
        let a = 2.0 * (1.0 - s).sqrt();
        (x * a, y * a, z)
    }

    pub fn dir_nd(&self, n: SizeT) -> Vec<f64> {
        let mut x = Vec::with_capacity(n);
        let mut d;
        
        loop {
            x.clear();
            d = 0.0;
            
            for _ in 0..n {
                let val = self.gaussian(1.0);
                x.push(val);
                d += val * val;
            }
            
            if d != 0.0 {
                break;
            }
        }
        
        d = d.sqrt();
        x.iter_mut().for_each(|val| *val /= d);
        x
    }
}