use rand::Rng;
use std::f64::consts::PI;

const SMALL_MEAN: f64 = 14.0;
const BINV_CUTOFF: i32 = 110;
const FAR_FROM_MEAN: i32 = 20;

fn stirling(y1: f64) -> f64 {
    let y2 = y1 * y1;
    (13860.0 - (462.0 - (132.0 - (99.0 - 140.0 / y2) / y2) / y2) / y2) / y1 / 166320.0
}

pub fn gsl_ran_binomial_tpe<R: Rng>(rng: &mut R, p: f64, n: u32) -> u32 {
    gsl_ran_binomial(rng, p, n)
}

pub fn gsl_ran_binomial<R: Rng>(rng: &mut R, mut p: f64, n: u32) -> u32 {
    let mut ix: i32;
    let mut flipped = false;

    if n == 0 {
        return 0;
    }

    if p > 0.5 {
        p = 1.0 - p;
        flipped = true;
    }

    let q = 1.0 - p;
    let s = p / q;
    let np = n as f64 * p;

    if np < SMALL_MEAN {
        let mut f0 = q.powi(n as i32);

        loop {
            let mut f = f0;
            let u = rng.gen::<f64>();

            for i in 0..=BINV_CUTOFF {
                if u < f {
                    ix = i;
                    return if flipped { n - ix as u32 } else { ix as u32 };
                }
                let i_f64 = i as f64;
                f *= s * (n as f64 - i_f64) / (i_f64 + 1.0);
            }
        }
    } else {
        let ffm = np + p;
        let m = ffm.floor() as i32;
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
            let u = rng.gen::<f64>() * p4;
            let v = rng.gen::<f64>();

            ix = if u <= p1 {
                (xm - p1 * v + u) as i32
            } else if u <= p2 {
                let x = xl + (u - p1) / c;
                let v_new = v * c + 1.0 - (x - xm).abs() / p1;
                if v_new > 1.0 || v_new <= 0.0 {
                    continue;
                }
                x as i32
            } else if u <= p3 {
                let ix_val = (xl + v.ln() / lambda_l) as i32;
                if ix_val < 0 {
                    continue;
                }
                ix_val
            } else {
                let ix_val = (xr - v.ln() / lambda_r) as i32;
                if ix_val > n as i32 {
                    continue;
                }
                ix_val
            };

            let k = (ix - m).abs();
            let accept = if k <= FAR_FROM_MEAN {
                let g = (n as f64 + 1.0) * s;
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
                v <= f
            } else {
                let var = v.ln();
                if k < (npq / 2.0 - 1.0) as i32 {
                    let amaxp = (k as f64 / npq)
                        * ((k as f64 * (k as f64 / 3.0 + 0.625) + (1.0 / 6.0)) / npq + 0.5);
                    let ynorm = -(k as f64 * k as f64 / (2.0 * npq));
                    if var < ynorm - amaxp {
                        break;
                    }
                    if var > ynorm + amaxp {
                        continue;
                    }
                }

                let x1 = ix as f64 + 1.0;
                let w1 = n as f64 - ix as f64 + 1.0;
                let f1 = fm + 1.0;
                let z1 = n as f64 + 1.0 - fm;

                let accept_val = xm * (f1 / x1).ln()
                    + (n as f64 - m as f64 + 0.5) * (z1 / w1).ln()
                    + (ix as f64 - m as f64) * (w1 * p / (x1 * q)).ln()
                    + stirling(f1)
                    + stirling(z1)
                    - stirling(x1)
                    - stirling(w1);

                var <= accept_val
            };

            if accept {
                break;
            }
        }
    }

    if flipped {
        n - ix as u32
    } else {
        ix as u32
    }
}