use rand::Rng;
use rand_distr::{StandardNormal, Distribution};
use std::f64::consts::PI;

/// Generates a random 2D direction vector using the Von Neumann method
pub fn dir_2d(rng: &mut impl Rng) -> (f64, f64) {
    let (u, v, s) = loop {
        let u = -1.0 + 2.0 * rng.gen::<f64>();
        let v = -1.0 + 2.0 * rng.gen::<f64>();
        let s = u * u + v * v;
        if s <= 1.0 && s != 0.0 {
            break (u, v, s);
        }
    };

    let x = (u * u - v * v) / s;
    let y = 2.0 * u * v / s;
    (x, y)
}

/// Generates a random 2D direction vector using trigonometric method
pub fn dir_2d_trig_method(rng: &mut impl Rng) -> (f64, f64) {
    let t = 2.0 * PI * rng.gen::<f64>();
    (t.cos(), t.sin())
}

/// Generates a random 3D direction vector
pub fn dir_3d(rng: &mut impl Rng) -> (f64, f64, f64) {
    let (x, y, s) = loop {
        let x = -1.0 + 2.0 * rng.gen::<f64>();
        let y = -1.0 + 2.0 * rng.gen::<f64>();
        let s = x * x + y * y;
        if s <= 1.0 {
            break (x, y, s);
        }
    };

    let z = -1.0 + 2.0 * s;
    let a = 2.0 * (1.0 - s).sqrt();
    (x * a, y * a, z)
}

/// Generates a random N-dimensional direction vector
pub fn dir_nd(rng: &mut impl Rng, n: usize) -> Vec<f64> {
    let standard_normal = StandardNormal;
    let mut x = Vec::with_capacity(n);
    let mut d = 0.0;

    loop {
        x.clear();
        d = 0.0;
        
        for _ in 0..n {
            let val = standard_normal.sample(rng);
            x.push(val);
            d += val * val;
        }

        if d != 0.0 {
            break;
        }
    }

    let d = d.sqrt();
    x.iter_mut().for_each(|val| *val /= d);
    x
}