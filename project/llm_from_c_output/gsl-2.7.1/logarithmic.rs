use rand::Rng;
use std::f64::consts::LN_2;

/// Logarithmic distribution
/// 
/// prob(n) =   p^n / (n log(1/(1-p)) for n = 1, 2, 3, ...
/// 
/// We use Kemp's second accelerated generator, from Luc Devroye's book
/// on "Non-Uniform Random Variate Generation", Springer
pub fn logarithmic(rng: &mut impl Rng, p: f64) -> u32 {
    let c = (1.0 - p).ln();
    let v = rng.gen_range(0.0..1.0);
    
    if v >= p {
        1
    } else {
        let u = rng.gen_range(0.0..1.0);
        let q = 1.0 - (c * u).exp();
        
        if v <= q * q {
            let x = 1.0 + v.ln() / q.ln();
            x as u32
        } else if v <= q {
            2
        } else {
            1
        }
    }
}

pub fn logarithmic_pdf(k: u32, p: f64) -> f64 {
    if k == 0 {
        0.0
    } else {
        p.powi(k as i32) / (k as f64) / (1.0 / (1.0 - p)).ln()
    }
}