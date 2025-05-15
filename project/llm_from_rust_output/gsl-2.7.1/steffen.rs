use std::f64;
use std::ops::{Add, Sub, Mul, Div};
use std::ptr::null_mut;
use std::sync::Once;

#[derive(Debug, Clone, Copy, PartialEq)]
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
pub struct GslInterpAccel {
    cache: usize,
    miss_count: usize,
    hit_count: usize,
}

impl GslInterpAccel {
    pub fn new() -> Self {
        GslInterpAccel {
            cache: 0,
            miss_count: 0,
            hit_count: 0,
        }
    }

    pub fn find(&mut self, xa: &[f64], x: f64) -> usize {
        let mut x_index = self.cache;
        if x < xa[x_index] {
            self.miss_count += 1;
            self.cache = self.bsearch(xa, x, 0, x_index);
        } else if x >= xa[x_index + 1] {
            self.miss_count += 1;
            self.cache = self.bsearch(xa, x, x_index, xa.len() - 1);
        } else {
            self.hit_count += 1;
        }
        self.cache
    }

    fn bsearch(&self, xa: &[f64], x: f64, index_lo: usize, index_hi: usize) -> usize {
        let mut ilo = index_lo;
        let mut ihi = index_hi;
        while ihi > ilo + 1 {
            let i = (ihi + ilo) / 2;
            if xa[i] > x {
                ihi = i;
            } else {
                ilo = i;
            }
        }
        ilo
    }
}

#[derive(Debug, Clone)]
pub struct SteffenState {
    a: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
    y_prime: Vec<f64>,
}

impl SteffenState {
    pub fn new(size: usize) -> Result<Self, GslError> {
        Ok(SteffenState {
            a: vec![0.0; size],
            b: vec![0.0; size],
            c: vec![0.0; size],
            d: vec![0.0; size],
            y_prime: vec![0.0; size],
        })
    }

    pub fn init(&mut self, x_array: &[f64], y_array: &[f64]) -> Result<(), GslError> {
        let size = x_array.len();
        if size < 3 {
            return Err(GslError::Invalid);
        }

        let h0 = x_array[1] - x_array[0];
        let s0 = (y_array[1] - y_array[0]) / h0;
        self.y_prime[0] = s0;

        for i in 1..size - 1 {
            let hi = x_array[i + 1] - x_array[i];
            let him1 = x_array[i] - x_array[i - 1];
            let si = (y_array[i + 1] - y_array[i]) / hi;
            let sim1 = (y_array[i] - y_array[i - 1]) / him1;
            let pi = (sim1 * hi + si * him1) / (him1 + hi);

            self.y_prime[i] = (Self::copysign(1.0, sim1) + Self::copysign(1.0, si))
                * f64::min(
                    f64::min(f64::abs(sim1), f64::abs(si)),
                    0.5 * f64::abs(pi),
                );
        }

        self.y_prime[size - 1] = (y_array[size - 1] - y_array[size - 2])
            / (x_array[size - 1] - x_array[size - 2]);

        for i in 0..size - 1 {
            let hi = x_array[i + 1] - x_array[i];
            let si = (y_array[i + 1] - y_array[i]) / hi;
            self.a[i] = (self.y_prime[i] + self.y_prime[i + 1] - 2.0 * si) / (hi * hi);
            self.b[i] = (3.0 * si - 2.0 * self.y_prime[i] - self.y_prime[i + 1]) / hi;
            self.c[i] = self.y_prime[i];
            self.d[i] = y_array[i];
        }

        Ok(())
    }

    pub fn eval(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        let index = match acc {
            Some(a) => a.find(x_array, x),
            None => self.bsearch(x_array, x, 0, x_array.len() - 1),
        };

        let x_lo = x_array[index];
        let delx = x - x_lo;
        Ok(self.d[index] + delx * (self.c[index] + delx * (self.b[index] + delx * self.a[index])))
    }

    pub fn eval_deriv(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        let index = match acc {
            Some(a) => a.find(x_array, x),
            None => self.bsearch(x_array, x, 0, x_array.len() - 1),
        };

        let x_lo = x_array[index];
        let delx = x - x_lo;
        Ok(self.c[index] + delx * (2.0 * self.b[index] + delx * 3.0 * self.a[index]))
    }

    pub fn eval_deriv2(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        let index = match acc {
            Some(a) => a.find(x_array, x),
            None => self.bsearch(x_array, x, 0, x_array.len() - 1),
        };

        let x_lo = x_array[index];
        let delx = x - x_lo;
        Ok(6.0 * self.a[index] * delx + 2.0 * self.b[index])
    }

    pub fn eval_integ(
        &self,
        x_array: &[f64],
        a: f64,
        b: f64,
        acc: Option<&mut GslInterpAccel>,
    ) -> Result<f64, GslError> {
        let size = x_array.len();
        let index_a = match acc {
            Some(accel) => accel.find(x_array, a),
            None => self.bsearch(x_array, a, 0, size - 1),
        };
        let index_b = match acc {
            Some(accel) => accel.find(x_array, b),
            None => self.bsearch(x_array, b, 0, size - 1),
        };

        let mut result = 0.0;
        for i in index_a..=index_b {
            let x_hi = x_array[i + 1];
            let x_lo = x_array[i];
            let dx = x_hi - x_lo;
            if dx == 0.0 {
                return Err(GslError::Invalid);
            }

            let x1 = if i == index_a { a - x_lo } else { 0.0 };
            let x2 = if i == index_b { b - x_lo } else { dx };

            result += 0.25 * self.a[i] * (x2.powi(4) - x1.powi(4))
                + (1.0 / 3.0) * self.b[i] * (x2.powi(3) - x1.powi(3))
                + 0.5 * self.c[i] * (x2.powi(2) - x1.powi(2))
                + self.d[i] * (x2 - x1);
        }
        Ok(result)
    }

    fn bsearch(&self, xa: &[f64], x: f64, index_lo: usize, index_hi: usize) -> usize {
        let mut ilo = index_lo;
        let mut ihi = index_hi;
        while ihi > ilo + 1 {
            let i = (ihi + ilo) / 2;
            if xa[i] > x {
                ihi = i;
            } else {
                ilo = i;
            }
        }
        ilo
    }

    fn copysign(x: f64, y: f64) -> f64 {
        if (x < 0.0 && y > 0.0) || (x > 0.0 && y < 0.0) {
            -x
        } else {
            x
        }
    }
}

pub struct GslInterpSteffen;

impl GslInterpSteffen {
    pub fn new(size: usize) -> Result<SteffenState, GslError> {
        if size < 3 {
            return Err(GslError::Invalid);
        }
        SteffenState::new(size)
    }
}

static STEFFEN_INTERP: Once = Once::new();

pub fn gsl_interp_steffen() -> &'static GslInterpSteffen {
    STEFFEN_INTERP.call_once(|| {});
    &GslInterpSteffen
}