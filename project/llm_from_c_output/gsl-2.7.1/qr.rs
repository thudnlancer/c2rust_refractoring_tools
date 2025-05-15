use std::f64;
use std::f64::consts::EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
}

pub fn qr_companion(h: &mut [f64], nc: usize, zroot: &mut [Complex]) -> Result<(), &'static str> {
    let mut t = 0.0;
    let mut n = nc;

    loop {
        if n == 0 {
            return Ok(());
        }

        let mut iterations = 0;

        loop {
            let e = (1..=n).rev().find(|&e| {
                if e < 2 {
                    return false;
                }
                let a1 = fmat(h, e, e - 1, nc).abs();
                let a2 = fmat(h, e - 1, e - 1, nc).abs();
                let a3 = fmat(h, e, e, nc).abs();
                a1 <= f64::EPSILON * (a2 + a3)
            }).unwrap_or(0);

            let x = fmat(h, n, n, nc);

            if e == n {
                zroot[n - 1] = Complex::new(x + t, 0.0);
                n -= 1;
                break;
            }

            let y = fmat(h, n - 1, n - 1, nc);
            let w = fmat(h, n - 1, n, nc) * fmat(h, n, n - 1, nc);

            if e == n - 1 {
                let p = (y - x) / 2.0;
                let q = p * p + w;
                let y = q.abs().sqrt();
                let x = x + t;

                if q > 0.0 {
                    let y = if p < 0.0 { -y } else { y } + p;
                    zroot[n - 1] = Complex::new(x - w / y, 0.0);
                    zroot[n - 2] = Complex::new(x + y, 0.0);
                } else {
                    zroot[n - 1] = Complex::new(x + p, -y);
                    zroot[n - 2] = Complex::new(x + p, y);
                }
                n -= 2;
                break;
            }

            if iterations == 120 {
                return Err("Too many iterations");
            }

            if iterations % 10 == 0 && iterations > 0 {
                t += x;
                for i in 1..=n {
                    let idx = (i - 1) * nc + (i - 1);
                    h[idx] -= x;
                }

                let s = fmat(h, n, n - 1, nc).abs() + fmat(h, n - 1, n - 2, nc).abs();
                let y = 0.75 * s;
                let x = y;
                let w = -0.4375 * s * s;
            }

            iterations += 1;

            let m = (e..=n - 2).rev().find(|&m| {
                let z = fmat(h, m, m, nc);
                let r = x - z;
                let s = y - z;
                let p = fmat(h, m, m + 1, nc) + (r * s - w) / fmat(h, m + 1, m, nc);
                let q = fmat(h, m + 1, m + 1, nc) - z - r - s;
                let r_val = fmat(h, m + 2, m + 1, nc);
                let s_val = p.abs() + q.abs() + r_val.abs();
                let p = p / s_val;
                let q = q / s_val;
                let r_val = r_val / s_val;

                if m == e {
                    return true;
                }

                let a1 = fmat(h, m, m - 1, nc).abs();
                let a2 = fmat(h, m - 1, m - 1, nc).abs();
                let a3 = fmat(h, m + 1, m + 1, nc).abs();

                a1 * (q.abs() + r_val.abs()) <= f64::EPSILON * p.abs() * (a2 + a3)
            }).unwrap_or(e);

            for i in m + 2..=n {
                set_fmat(h, i, i - 2, nc, 0.0);
            }

            for i in m + 3..=n {
                set_fmat(h, i, i - 3, nc, 0.0);
            }

            for k in m..=n - 1 {
                let notlast = k != n - 1;

                let (mut p, mut q, mut r_val) = if k != m {
                    (
                        fmat(h, k, k - 1, nc),
                        fmat(h, k + 1, k - 1, nc),
                        if notlast { fmat(h, k + 2, k - 1, nc) } else { 0.0 },
                    )
                } else {
                    (0.0, 0.0, 0.0)
                };

                if k != m {
                    let x = p.abs() + q.abs() + r_val.abs();
                    if x == 0.0 {
                        continue;
                    }
                    p /= x;
                    q /= x;
                    r_val /= x;
                }

                let mut s = (p * p + q * q + r_val * r_val).sqrt();
                if p < 0.0 {
                    s = -s;
                }

                if k != m {
                    set_fmat(h, k, k - 1, nc, -s * (p.abs() + q.abs() + r_val.abs()));
                } else if e != m {
                    let val = fmat(h, k, k - 1, nc);
                    set_fmat(h, k, k - 1, nc, -val);
                }

                p += s;
                let x = p / s;
                let y = q / s;
                let z = r_val / s;
                q /= p;
                r_val /= p;

                for j in k..=n {
                    let mut p = fmat(h, k, j, nc) + q * fmat(h, k + 1, j, nc);
                    if notlast {
                        p += r_val * fmat(h, k + 2, j, nc);
                        let val = fmat(h, k + 2, j, nc) - p * z;
                        set_fmat(h, k + 2, j, nc, val);
                    }
                    let val = fmat(h, k + 1, j, nc) - p * y;
                    set_fmat(h, k + 1, j, nc, val);
                    let val = fmat(h, k, j, nc) - p * x;
                    set_fmat(h, k, j, nc, val);
                }

                let j = if k + 3 < n { k + 3 } else { n };

                for i in e..=j {
                    let mut p = x * fmat(h, i, k, nc) + y * fmat(h, i, k + 1, nc);
                    if notlast {
                        p += z * fmat(h, i, k + 2, nc);
                        let val = fmat(h, i, k + 2, nc) - p * r_val;
                        set_fmat(h, i, k + 2, nc, val);
                    }
                    let val = fmat(h, i, k + 1, nc) - p * q;
                    set_fmat(h, i, k + 1, nc, val);
                    let val = fmat(h, i, k, nc) - p;
                    set_fmat(h, i, k, nc, val);
                }
            }
        }
    }
}

fn fmat(matrix: &[f64], row: usize, col: usize, nc: usize) -> f64 {
    matrix[(row - 1) * nc + (col - 1)]
}

fn set_fmat(matrix: &mut [f64], row: usize, col: usize, nc: usize, value: f64) {
    matrix[(row - 1) * nc + (col - 1)] = value;
}