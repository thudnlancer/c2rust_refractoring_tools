use std::f64::consts::{PI, FRAC_PI_2};
use rand::Rng;
use rand::distributions::{Uniform, Exponential};

/// The stable Levy probability distributions have the form
///
/// p(x) dx = (1/(2 pi)) \int dt exp(- it x - |c t|^alpha)
///
/// with 0 < alpha <= 2.
///
/// For alpha = 1, we get the Cauchy distribution
/// For alpha = 2, we get the Gaussian distribution with sigma = sqrt(2) c.
///
/// From Chapter 5 of Bratley, Fox and Schrage "A Guide to
/// Simulation". The original reference given there is,
///
/// J.M. Chambers, C.L. Mallows and B. W. Stuck. "A method for
/// simulating stable random variates". Journal of the American
/// Statistical Association, JASA 71 340-344 (1976).
pub fn levy(rng: &mut impl Rng, c: f64, alpha: f64) -> f64 {
    let u = PI * (rng.sample(Uniform::new(0.0, 1.0)) - 0.5);

    if alpha == 1.0 {
        // cauchy case
        let t = u.tan();
        return c * t;
    }

    let v = loop {
        let v = rng.sample(Exponential::new(1.0));
        if v != 0.0 {
            break v;
        }
    };

    if alpha == 2.0 {
        // gaussian case
        let t = 2.0 * u.sin() * v.sqrt();
        return c * t;
    }

    // general case
    let t = (alpha * u).sin() / u.cos().powf(1.0 / alpha);
    let s = ((1.0 - alpha) * u).cos() / v;
    let s = s.powf((1.0 - alpha) / alpha);

    c * t * s
}

/// The stable Levy probability distributions have the form
///
/// 2*pi* p(x) dx
///
///   = \int dt exp(mu*i*t-|sigma*t|^alpha*(1-i*beta*sign(t)*tan(pi*alpha/2))) for alpha!=1
///   = \int dt exp(mu*i*t-|sigma*t|^alpha*(1+i*beta*sign(t)*2/pi*log(|t|)))   for alpha==1
///
/// with 0<alpha<=2, -1<=beta<=1, sigma>0.
///
/// For beta=0, sigma=c, mu=0, we get gsl_ran_levy above.
///
/// For alpha = 1, beta=0, we get the Lorentz distribution
/// For alpha = 2, beta=0, we get the Gaussian distribution
///
/// See A. Weron and R. Weron: Computer simulation of Lvy alpha-stable 
/// variables and processes, preprint Technical University of Wroclaw.
/// http://www.im.pwr.wroc.pl/~hugo/Publications.html
pub fn levy_skew(rng: &mut impl Rng, c: f64, alpha: f64, beta: f64) -> f64 {
    if beta == 0.0 {
        // symmetric case
        return levy(rng, c, alpha);
    }

    let v = PI * (rng.sample(Uniform::new(0.0, 1.0)) - 0.5);

    let w = loop {
        let w = rng.sample(Exponential::new(1.0));
        if w != 0.0 {
            break w;
        }
    };

    if alpha == 1.0 {
        let x = ((FRAC_PI_2 + beta * v) * v.tan() 
               - beta * (FRAC_PI_2 * w * v.cos() / (FRAC_PI_2 + beta * v)).ln()) / FRAC_PI_2;
        return c * (x + beta * c.ln() / FRAC_PI_2);
    } else {
        let t = beta * (FRAC_PI_2 * alpha).tan();
        let b = t.atan() / alpha;
        let s = (1.0 + t * t).powf(1.0 / (2.0 * alpha));

        let x = s * (alpha * (v + b)).sin() / v.cos().powf(1.0 / alpha)
              * ((v - alpha * (v + b)).cos() / w).powf((1.0 - alpha) / alpha);
        c * x
    }
}