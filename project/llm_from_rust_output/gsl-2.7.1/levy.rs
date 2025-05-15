use std::f64::consts::PI;
use libm::{atan, cos, sin, tan, log, pow, sqrt};

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut [u8], u64)>,
    pub get: Option<fn(&[u8]) -> u64>,
    pub get_double: Option<fn(&[u8]) -> f64>,
}

#[derive(Copy, Clone)]
pub struct GslRng<'a> {
    pub type_: &'a GslRngType,
    pub state: &'a mut [u8],
}

impl<'a> GslRng<'a> {
    fn uniform_pos(&mut self) -> f64 {
        let get_double = self.type_.get_double.expect("non-null function pointer");
        loop {
            let x = get_double(self.state);
            if x != 0.0 {
                return x;
            }
        }
    }

    fn exponential(&mut self, mu: f64) -> f64 {
        // Assuming this is implemented elsewhere as a safe wrapper
        // For now, we'll just use the Rust rand crate's exponential distribution
        use rand_distr::Exp;
        let exp = Exp::new(1.0 / mu).unwrap();
        rand::thread_rng().sample(exp)
    }

    pub fn levy(&mut self, c: f64, alpha: f64) -> f64 {
        let u = PI * (self.uniform_pos() - 0.5);

        if alpha == 1.0 {
            return c * tan(u);
        }

        let v = loop {
            let v = self.exponential(1.0);
            if v != 0.0 {
                break v;
            }
        };

        if alpha == 2.0 {
            return c * 2.0 * sin(u) * sqrt(v);
        }

        let t = sin(alpha * u) / pow(cos(u), 1.0 / alpha);
        let s = pow(
            cos((1.0 - alpha) * u) / v,
            (1.0 - alpha) / alpha,
        );
        c * t * s
    }

    pub fn levy_skew(&mut self, c: f64, alpha: f64, beta: f64) -> f64 {
        if beta == 0.0 {
            return self.levy(c, alpha);
        }

        let v = PI * (self.uniform_pos() - 0.5);
        let w = loop {
            let w = self.exponential(1.0);
            if w != 0.0 {
                break w;
            }
        };

        if alpha == 1.0 {
            let x = ((PI/2.0 + beta * v) * tan(v)
                - beta * log(
                    (PI/2.0) * w * cos(v) / (PI/2.0 + beta * v
                )) / (PI/2.0);
            return c * (x + beta * log(c) / (PI/2.0));
        }

        let t = beta * tan(PI/2.0 * alpha);
        let b = atan(t) / alpha;
        let s = pow(1.0 + t * t, 1.0 / (2.0 * alpha));
        let x = s * sin(alpha * (v + b)) / pow(cos(v), 1.0 / alpha)
            * pow(
                cos(v - alpha * (v + b)) / w,
                (1.0 - alpha) / alpha,
            );
        c * x
    }
}