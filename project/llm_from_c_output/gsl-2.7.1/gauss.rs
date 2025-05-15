use std::f64::consts::PI;
use rand::Rng;

/// Polar (Box-Mueller) method; See Knuth v2, 3rd ed, p122
pub fn gaussian(rng: &mut impl Rng, sigma: f64) -> f64 {
    let mut x;
    let mut y;
    let mut r2;

    loop {
        // choose x,y in uniform square (-1,-1) to (+1,+1)
        x = -1.0 + 2.0 * rng.gen::<f64>();
        y = -1.0 + 2.0 * rng.gen::<f64>();

        // see if it is in the unit circle
        r2 = x * x + y * y;
        if r2 <= 1.0 && r2 != 0.0 {
            break;
        }
    }

    // Box-Muller transform
    sigma * y * (-2.0 * r2.ln() / r2).sqrt()
}

/// Ratio method (Kinderman-Monahan); see Knuth v2, 3rd ed, p130.
/// K+M, ACM Trans Math Software 3 (1977) 257-260.
///
/// [Added by Charles Karney] This is an implementation of Leva's
/// modifications to the original K+M method; see:
/// J. L. Leva, ACM Trans Math Software 18 (1992) 449-453 and 454-455.
pub fn gaussian_ratio_method(rng: &mut impl Rng, sigma: f64) -> f64 {
    const S: f64 = 0.449871;    // Constants from Leva
    const T: f64 = -0.386595;
    const A: f64 = 0.19600;
    const B: f64 = 0.25472;
    const R1: f64 = 0.27597;
    const R2: f64 = 0.27846;

    loop {
        // Generate a point P = (u, v) uniform in a rectangle enclosing
        // the K+M region v^2 <= - 4 u^2 log(u).

        // u in (0, 1] to avoid singularity at u = 0
        let u = 1.0 - rng.gen::<f64>();

        // v is in the asymmetric interval [-0.5, 0.5). However v = -0.5
        // is rejected in the last part of the while clause. The
        // resulting normal deviate is strictly symmetric about 0
        // (provided that v is symmetric once v = -0.5 is excluded).
        let mut v = rng.gen::<f64>() - 0.5;

        // Constant 1.7156 > sqrt(8/e) (for accuracy); but not by too
        // much (for efficiency).
        v *= 1.7156;

        // Compute Leva's quadratic form Q
        let x = u - S;
        let y = v.abs() - T;
        let q = x * x + y * (A * y - B * x);

        // Accept P if Q < r1 (Leva)
        // Reject P if Q > r2 (Leva)
        // Accept if v^2 <= -4 u^2 log(u) (K+M)
        // This final test is executed 0.012 times on average.
        if q < R1 || (q <= R2 && v * v <= -4.0 * u * u * u.ln()) {
            return sigma * (v / u); // Return slope
        }
    }
}

pub fn gaussian_pdf(x: f64, sigma: f64) -> f64 {
    let u = x / sigma.abs();
    (1.0 / (2.0 * PI).sqrt() / sigma.abs()) * (-u * u / 2.0).exp()
}

pub fn ugaussian(rng: &mut impl Rng) -> f64 {
    gaussian(rng, 1.0)
}

pub fn ugaussian_ratio_method(rng: &mut impl Rng) -> f64 {
    gaussian_ratio_method(rng, 1.0)
}

pub fn ugaussian_pdf(x: f64) -> f64 {
    gaussian_pdf(x, 1.0)
}