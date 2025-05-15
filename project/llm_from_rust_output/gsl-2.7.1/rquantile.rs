use std::cmp::Ordering;
use std::ptr::NonNull;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslRstatQuantileWorkspace {
    p: f64,
    q: [f64; 5],
    npos: [i32; 5],
    np: [f64; 5],
    dnp: [f64; 5],
    n: usize,
}

impl GslRstatQuantileWorkspace {
    pub fn new(p: f64) -> Result<Self, GslError> {
        if p < 0.0 || p > 1.0 {
            return Err(GslError::Invalid);
        }

        let mut w = Self {
            p,
            q: [0.0; 5],
            npos: [0; 5],
            np: [0.0; 5],
            dnp: [0.0; 5],
            n: 0,
        };

        w.reset();
        Ok(w)
    }

    pub fn reset(&mut self) -> Result<(), GslError> {
        for i in 0..5 {
            self.npos[i] = (i + 1) as i32;
        }

        self.np[0] = 1.0;
        self.np[1] = 1.0 + 2.0 * self.p;
        self.np[2] = 1.0 + 4.0 * self.p;
        self.np[3] = 3.0 + 2.0 * self.p;
        self.np[4] = 5.0;

        self.dnp[0] = 0.0;
        self.dnp[1] = 0.5 * self.p;
        self.dnp[2] = self.p;
        self.dnp[3] = 0.5 * (1.0 + self.p);
        self.dnp[4] = 1.0;

        self.n = 0;
        Ok(())
    }

    pub fn add(&mut self, x: f64) -> Result<(), GslError> {
        if self.n < 5 {
            self.q[self.n] = x;
        } else {
            if self.n == 5 {
                self.q.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            }

            let k = if x < self.q[0] {
                self.q[0] = x;
                0
            } else if x >= self.q[4] {
                self.q[4] = x;
                3
            } else {
                match (0..4).find(|&i| self.q[i] <= x && x < self.q[i + 1]) {
                    Some(k) => k,
                    None => return Err(GslError::Invalid),
                }
            };

            for i in (k + 1)..5 {
                self.npos[i] += 1;
            }

            for i in 0..5 {
                self.np[i] += self.dnp[i];
            }

            for i in 1..4 {
                let ni = self.npos[i] as f64;
                let d = self.np[i] - ni;

                if (d >= 1.0 && self.npos[i + 1] - self.npos[i] > 1)
                    || (d <= -1.0 && self.npos[i - 1] - self.npos[i] < -1)
                {
                    let dsign = if d > 0.0 { 1 } else { -1 };
                    let qp1 = self.q[i + 1];
                    let qi = self.q[i];
                    let qm1 = self.q[i - 1];
                    let np1 = self.npos[i + 1] as f64;
                    let nm1 = self.npos[i - 1] as f64;

                    let qp = calc_psq(qp1, qi, qm1, dsign as f64, np1, ni, nm1);

                    if qm1 < qp && qp < qp1 {
                        self.q[i] = qp;
                    } else {
                        self.q[i] += dsign as f64 * (self.q[i + dsign] - qi)
                            / (self.npos[i + dsign] as f64 - ni);
                    }

                    self.npos[i] += dsign;
                }
            }
        }

        self.n += 1;
        Ok(())
    }

    pub fn get(&mut self) -> f64 {
        if self.n > 5 {
            self.q[2]
        } else {
            self.q.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            quantile_from_sorted_data(&self.q, self.n, self.p)
        }
    }
}

fn quantile_from_sorted_data(sorted_data: &[f64], n: usize, f: f64) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let index = f * (n - 1) as f64;
    let i = index.floor() as usize;
    let delta = index - i as f64;

    if i == n - 1 {
        sorted_data[i]
    } else {
        sorted_data[i] + delta * (sorted_data[i + 1] - sorted_data[i])
    }
}

fn calc_psq(qp1: f64, q: f64, qm1: f64, d: f64, np1: f64, n: f64, nm1: f64) -> f64 {
    let outer = d / (np1 - nm1);
    let inner_left = (n - nm1 + d) * (qp1 - q) / (np1 - n);
    let inner_right = (np1 - n - d) * (q - qm1) / (n - nm1);
    q + outer * (inner_left + inner_right)
}