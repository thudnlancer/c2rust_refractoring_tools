use rand::Rng;
use rand_distr::{Distribution, Gamma, StandardNormal, Uniform};
use std::f64::consts::{E, PI};

/// Gamma distribution implementation based on Knuth's algorithm
pub struct GammaKnuth;

impl GammaKnuth {
    /// Generate a gamma-distributed random number with shape `a` and scale `b`
    pub fn gamma_knuth<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
        assert!(a > 0.0, "Shape parameter a must be positive");
        
        let na = a.floor();
        
        if a >= u32::MAX as f64 {
            b * (Self::gamma_large(rng, a.floor()) + Self::gamma_frac(rng, a - a.floor()))
        } else if a == na {
            b * Self::gamma_int(rng, na as u32)
        } else if na == 0.0 {
            b * Self::gamma_frac(rng, a)
        } else {
            b * (Self::gamma_int(rng, na as u32) + Self::gamma_frac(rng, a - na))
        }
    }

    /// Generate gamma-distributed random number for integer shape parameter
    fn gamma_int<R: Rng>(rng: &mut R, a: u32) -> f64 {
        if a < 12 {
            let mut prod = 1.0;
            let uniform = Uniform::new(0.0, 1.0);
            
            for _ in 0..a {
                prod *= uniform.sample(rng);
            }
            
            -prod.ln()
        } else {
            Self::gamma_large(rng, a as f64)
        }
    }

    /// Generate gamma-distributed random number for large shape parameter (a > 1)
    fn gamma_large<R: Rng>(rng: &mut R, a: f64) -> f64 {
        assert!(a > 1.0, "Shape parameter a must be > 1 for gamma_large");
        
        let sqa = (2.0 * a - 1.0).sqrt();
        let uniform = Uniform::new(0.0, 1.0);
        
        loop {
            let y;
            let x;
            
            loop {
                y = (PI * uniform.sample(rng)).tan();
                x = sqa * y + a - 1.0;
                if x > 0.0 {
                    break;
                }
            }
            
            let v = uniform.sample(rng);
            if v <= (1.0 + y * y) * ((a - 1.0) * (x / (a - 1.0)).ln() - sqa * y).exp() {
                return x;
            }
        }
    }

    /// Generate gamma-distributed random number for fractional shape parameter (0 < a < 1)
    fn gamma_frac<R: Rng>(rng: &mut R, a: f64) -> f64 {
        assert!(a > 0.0 && a < 1.0, "Shape parameter a must be 0 < a < 1 for gamma_frac");
        
        if a == 0.0 {
            return 0.0;
        }
        
        let p = E / (a + E);
        let uniform = Uniform::new(0.0, 1.0);
        let uniform_pos = Uniform::new(0.0, 1.0);
        
        loop {
            let u = uniform.sample(rng);
            let v = uniform_pos.sample(rng);
            
            let (x, q) = if u < p {
                let x = v.powf(1.0 / a);
                let q = (-x).exp();
                (x, q)
            } else {
                let x = 1.0 - v.ln();
                let q = x.powf(a - 1.0);
                (x, q)
            };
            
            if uniform.sample(rng) < q {
                return x;
            }
        }
    }

    /// Calculate the PDF of the gamma distribution
    pub fn gamma_pdf(x: f64, a: f64, b: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else if x == 0.0 {
            if a == 1.0 {
                1.0 / b
            } else {
                0.0
            }
        } else if a == 1.0 {
            (-x / b).exp() / b
        } else {
            let lngamma = a.ln_gamma().0;
            ((a - 1.0) * (x / b).ln() - x / b - lngamma).exp() / b
        }
    }
}

/// Gamma distribution implementation based on Marsaglia and Tsang's algorithm
pub struct GammaMT;

impl GammaMT {
    /// Generate a gamma-distributed random number with shape `a` and scale `b`
    pub fn gamma_mt<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
        Self::gamma(rng, a, b)
    }

    /// Generate a gamma-distributed random number with shape `a` and scale `b`
    pub fn gamma<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
        assert!(a > 0.0, "Shape parameter a must be positive");
        
        if a < 1.0 {
            let u = rng.gen_range(0.0..1.0);
            return Self::gamma(rng, 1.0 + a, b) * u.powf(1.0 / a);
        }
        
        let d = a - 1.0 / 3.0;
        let c = (1.0 / 3.0) / d.sqrt();
        let normal = StandardNormal;
        
        loop {
            let x;
            let v;
            
            loop {
                x = normal.sample(rng);
                v = 1.0 + c * x;
                if v > 0.0 {
                    break;
                }
            }
            
            let v = v * v * v;
            let u = rng.gen_range(0.0..1.0);
            
            if u < 1.0 - 0.0331 * x * x * x * x {
                return b * d * v;
            }
            
            if u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln()) {
                return b * d * v;
            }
        }
    }
}