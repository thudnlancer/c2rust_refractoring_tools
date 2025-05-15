use std::f64::consts::PI;
use rand::Rng;

/// The lognormal distribution has the form 
///
/// p(x) dx = 1/(x * sqrt(2 pi sigma^2)) exp(-(ln(x) - zeta)^2/2 sigma^2) dx
///
/// for x > 0. Lognormal random numbers are the exponentials of
/// gaussian random numbers
pub fn lognormal(rng: &mut impl Rng, zeta: f64, sigma: f64) -> f64 {
    let (u, v, r2) = loop {
        // choose x,y in uniform square (-1,-1) to (+1,+1)
        let u = -1.0 + 2.0 * rng.gen::<f64>();
        let v = -1.0 + 2.0 * rng.gen::<f64>();
        
        // see if it is in the unit circle
        let r2 = u * u + v * v;
        if r2 <= 1.0 && r2 != 0.0 {
            break (u, v, r2);
        }
    };

    let normal = u * (-2.0 * r2.ln() / r2).sqrt();
    (sigma * normal + zeta).exp()
}

pub fn lognormal_pdf(x: f64, zeta: f64, sigma: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        let u = (x.ln() - zeta) / sigma;
        let p = 1.0 / (x * sigma.abs() * (2.0 * PI).sqrt()) * (-(u * u) / 2.0).exp();
        p
    }
}