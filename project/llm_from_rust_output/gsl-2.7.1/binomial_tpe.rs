use std::f64::consts::PI;
use std::f64;

pub struct GslRng {
    rng_type: GslRngType,
    state: Vec<u8>,
}

pub struct GslRngType {
    name: String,
    max: u64,
    min: u64,
    size: usize,
    get_double: fn(&mut Vec<u8>) -> f64,
}

impl GslRng {
    pub fn uniform(&mut self) -> f64 {
        (self.rng_type.get_double)(&mut self.state)
    }
}

fn stirling(y1: f64) -> f64 {
    let y2 = y1 * y1;
    (13860.0 - (462.0 - (132.0 - (99.0 - 140.0 / y2) / y2) / y2) / y2) / y1 / 166320.0
}

pub fn gsl_ran_binomial_tpe(rng: &mut GslRng, p: f64, n: u32) -> u32 {
    gsl_ran_binomial(rng, p, n)
}

pub fn gsl_ran_binomial(rng: &mut GslRng, mut p: f64, n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut flipped = false;
    if p > 0.5 {
        p = 1.0 - p;
        flipped = true;
    }

    let q = 1.0 - p;
    let s = p / q;
    let np = n as f64 * p;

    let ix = if np < 14.0 {
        let mut f0 = q.powi(n as i32);
        let mut ix = 0;
        loop {
            let mut f = f0;
            let u = rng.uniform();
            ix = 0;
            while ix <= 110 {
                if u < f {
                    break;
                }
                let new_f = f * s * (n - ix as u32) as f64 / (ix + 1) as f64;
                if new_f.is_infinite() || new_f.is_nan() {
                    break;
                }
                f = new_f;
                ix += 1;
            }
            if u < f {
                break ix;
            }
        }
    } else {
        let ffm = np + p;
        let m = ffm as i32;
        let fm = m as f64;
        let xm = fm + 0.5;
        let npq = np * q;
        let p1 = (2.195 * npq.sqrt() - 4.6 * q).floor() + 0.5;
        let xl = xm - p1;
        let xr = xm + p1;
        let c = 0.134 + 20.5 / (15.3 + fm);
        let p2 = p1 * (1.0 + c + c);
        let al = (ffm - xl) / (ffm - xl * p);
        let lambda_l = al * (1.0 + 0.5 * al);
        let ar = (xr - ffm) / (xr * q);
        let lambda_r = ar * (1.0 + 0.5 * ar);
        let p3 = p2 + c / lambda_l;
        let p4 = p3 + c / lambda_r;

        loop {
            let u = rng.uniform() * p4;
            let v = rng.uniform();
            let ix = if u <= p1 {
                (xm - p1 * v + u) as i32
            } else if u <= p2 {
                let x = xl + (u - p1) / c;
                let v = v * c + 1.0 - (x - xm).abs() / p1;
                if v > 1.0 || v <= 0.0 {
                    continue;
                }
                x as i32
            } else if u <= p3 {
                let ix = (xl + v.ln() / lambda_l) as i32;
                if ix < 0 {
                    continue;
                }
                let v = v * (u - p2) * lambda_l;
                ix
            } else {
                let ix = (xr - v.ln() / lambda_r) as i32;
                if ix as f64 > n as f64 {
                    continue;
                }
                let v = v * (u - p3) * lambda_r;
                ix
            };

            let k = (ix - m).abs();
            if k <= 20 {
                let g = (n + 1) as f64 * s;
                let mut f = 1.0;
                if m < ix {
                    for i in (m + 1)..=ix {
                        f *= g / i as f64 - s;
                    }
                } else if m > ix {
                    for i in (ix + 1)..=m {
                        f /= g / i as f64 - s;
                    }
                }
                if v <= f {
                    break ix;
                }
            } else {
                let var = v.ln();
                if (k as f64) < npq / 2.0 - 1.0 {
                    let amaxp = k as f64 / npq
                        * ((k as f64 * (k as f64 / 3.0 + 0.625) + 1.0 / 6.0) / npq
                        + 0.5;
                    let ynorm = -((k * k) as f64 / (2.0 * npq));
                    if var < ynorm - amaxp {
                        break ix;
                    }
                    if var > ynorm + amaxp {
                        continue;
                    }
                }

                let x1 = ix as f64 + 1.0;
                let w1 = (n - ix as u32) as f64 + 1.0;
                let f1 = fm + 1.0;
                let z1 = n as f64 + 1.0 - fm;
                let accept = xm * (f1 / x1).ln()
                    + (n as f64 - m as f64 + 0.5) * (z1 / w1).ln()
                    + (ix - m) as f64 * (w1 * p / (x1 * q)).ln()
                    + stirling(f1)
                    + stirling(z1)
                    - stirling(x1)
                    - stirling(w1);

                if var <= accept {
                    break ix;
                }
            }
        }
    };

    if flipped {
        n - ix as u32
    } else {
        ix as u32
    }
}