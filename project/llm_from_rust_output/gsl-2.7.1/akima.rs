use std::f64;
use std::ops::{Add, Sub, Mul, Div};
use std::ptr;

#[derive(Debug, Clone, Copy)]
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
            self.cache = Self::bsearch(xa, x, 0, x_index);
        } else if x >= xa[x_index + 1] {
            self.miss_count += 1;
            self.cache = Self::bsearch(xa, x, x_index, xa.len() - 1);
        } else {
            self.hit_count += 1;
        }
        self.cache
    }

    fn bsearch(xa: &[f64], x: f64, index_lo: usize, index_hi: usize) -> usize {
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
pub struct AkimaState {
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
    m: Vec<f64>,
}

impl AkimaState {
    pub fn new(size: usize) -> Result<Self, GslError> {
        Ok(AkimaState {
            b: vec![0.0; size],
            c: vec![0.0; size],
            d: vec![0.0; size],
            m: vec![0.0; size + 4],
        })
    }

    fn calc(&mut self, x_array: &[f64], size: usize) {
        let m = &mut self.m[2..size + 2];
        for i in 0..size - 1 {
            let ne = (m[i + 1] - m[i]).abs() + (m[i - 1] - m[i - 2]).abs();
            if ne == 0.0 {
                self.b[i] = m[i];
                self.c[i] = 0.0;
                self.d[i] = 0.0;
            } else {
                let h_i = x_array[i + 1] - x_array[i];
                let ne_next = (m[i + 2] - m[i + 1]).abs() + (m[i] - m[i - 1]).abs();
                let alpha_i = (m[i - 1] - m[i - 2]).abs() / ne;
                let (t_l_ip1, alpha_ip1) = if ne_next == 0.0 {
                    (m[i], 0.0)
                } else {
                    let alpha_ip1 = (m[i] - m[i - 1]).abs() / ne_next;
                    let t_l_ip1 = (1.0 - alpha_ip1) * m[i] + alpha_ip1 * m[i + 1];
                    (t_l_ip1, alpha_ip1)
                };
                self.b[i] = (1.0 - alpha_i) * m[i - 1] + alpha_i * m[i];
                self.c[i] = (3.0 * m[i] - 2.0 * self.b[i] - t_l_ip1) / h_i;
                self.d[i] = (self.b[i] + t_l_ip1 - 2.0 * m[i]) / (h_i * h_i);
            }
        }
    }

    pub fn init(&mut self, x_array: &[f64], y_array: &[f64], size: usize) -> Result<(), GslError> {
        let m = &mut self.m[2..size + 2];
        for i in 0..size - 1 {
            m[i] = (y_array[i + 1] - y_array[i]) / (x_array[i + 1] - x_array[i]);
        }
        self.m[0] = 3.0 * m[0] - 2.0 * m[1];
        self.m[1] = 2.0 * m[0] - m[1];
        self.m[size] = 2.0 * m[size - 2] - m[size - 3];
        self.m[size + 1] = 3.0 * m[size - 2] - 2.0 * m[size - 3];
        self.calc(x_array, size);
        Ok(())
    }

    pub fn init_periodic(&mut self, x_array: &[f64], y_array: &[f64], size: usize) -> Result<(), GslError> {
        let m = &mut self.m[2..size + 2];
        for i in 0..size - 1 {
            m[i] = (y_array[i + 1] - y_array[i]) / (x_array[i + 1] - x_array[i]);
        }
        self.m[0] = m[size - 3];
        self.m[1] = m[size - 2];
        self.m[size] = m[0];
        self.m[size + 1] = m[1];
        self.calc(x_array, size);
        Ok(())
    }

    pub fn eval(&self, x_array: &[f64], y_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        let index = match acc {
            Some(a) => a.find(x_array, x),
            None => GslInterpAccel::bsearch(x_array, x, 0, x_array.len() - 1),
        };
        let x_lo = x_array[index];
        let delx = x - x_lo;
        let y = y_array[index] + delx * (self.b[index] + delx * (self.c[index] + self.d[index] * delx));
        Ok(y)
    }

    pub fn eval_deriv(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        let index = match acc {
            Some(a) => a.find(x_array, x),
            None => GslInterpAccel::bsearch(x_array, x, 0, x_array.len() - 1),
        };
        let x_lo = x_array[index];
        let delx = x - x_lo;
        let dydx = self.b[index] + delx * (2.0 * self.c[index] + 3.0 * self.d[index] * delx);
        Ok(dydx)
    }

    pub fn eval_deriv2(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        let index = match acc {
            Some(a) => a.find(x_array, x),
            None => GslInterpAccel::bsearch(x_array, x, 0, x_array.len() - 1),
        };
        let x_lo = x_array[index];
        let delx = x - x_lo;
        let y_pp = 2.0 * self.c[index] + 6.0 * self.d[index] * delx;
        Ok(y_pp)
    }

    pub fn eval_integ(
        &self,
        x_array: &[f64],
        y_array: &[f64],
        a: f64,
        b: f64,
        acc: Option<&mut GslInterpAccel>,
    ) -> Result<f64, GslError> {
        let size = x_array.len();
        let index_a = match acc {
            Some(a_acc) => a_acc.find(x_array, a),
            None => GslInterpAccel::bsearch(x_array, a, 0, size - 1),
        };
        let index_b = match acc {
            Some(b_acc) => b_acc.find(x_array, b),
            None => GslInterpAccel::bsearch(x_array, b, 0, size - 1),
        };

        let mut result = 0.0;
        for i in index_a..=index_b {
            let x_hi = x_array[i + 1];
            let x_lo = x_array[i];
            let y_lo = y_array[i];
            let dx = x_hi - x_lo;
            if dx == 0.0 {
                return Err(GslError::Invalid);
            }

            if i == index_a || i == index_b {
                let x1 = if i == index_a { a } else { x_lo };
                let x2 = if i == index_b { b } else { x_hi };
                result += Self::integ_eval(
                    y_lo,
                    self.b[i],
                    self.c[i],
                    self.d[i],
                    x_lo,
                    x1,
                    x2,
                );
            } else {
                result += dx * (y_lo + dx * (0.5 * self.b[i] + dx * (self.c[i] / 3.0 + 0.25 * self.d[i] * dx)));
            }
        }
        Ok(result)
    }

    fn integ_eval(
        ai: f64,
        bi: f64,
        ci: f64,
        di: f64,
        xi: f64,
        a: f64,
        b: f64,
    ) -> f64 {
        let r1 = a - xi;
        let r2 = b - xi;
        let r12 = r1 + r2;
        let bterm = 0.5 * bi * r12;
        let cterm = (1.0 / 3.0) * ci * (r1 * r1 + r2 * r2 + r1 * r2);
        let dterm = 0.25 * di * r12 * (r1 * r1 + r2 * r2);
        (b - a) * (ai + bterm + cterm + dterm)
    }
}

pub struct AkimaInterp {
    state: AkimaState,
    min_size: usize,
}

impl AkimaInterp {
    pub fn new(size: usize) -> Result<Self, GslError> {
        Ok(AkimaInterp {
            state: AkimaState::new(size)?,
            min_size: 5,
        })
    }

    pub fn init(&mut self, x_array: &[f64], y_array: &[f64]) -> Result<(), GslError> {
        if x_array.len() < self.min_size {
            return Err(GslError::Invalid);
        }
        self.state.init(x_array, y_array, x_array.len())
    }

    pub fn eval(&self, x_array: &[f64], y_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        self.state.eval(x_array, y_array, x, acc)
    }

    pub fn eval_deriv(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        self.state.eval_deriv(x_array, x, acc)
    }

    pub fn eval_deriv2(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        self.state.eval_deriv2(x_array, x, acc)
    }

    pub fn eval_integ(
        &self,
        x_array: &[f64],
        y_array: &[f64],
        a: f64,
        b: f64,
        acc: Option<&mut GslInterpAccel>,
    ) -> Result<f64, GslError> {
        self.state.eval_integ(x_array, y_array, a, b, acc)
    }
}

pub struct AkimaPeriodicInterp {
    state: AkimaState,
    min_size: usize,
}

impl AkimaPeriodicInterp {
    pub fn new(size: usize) -> Result<Self, GslError> {
        Ok(AkimaPeriodicInterp {
            state: AkimaState::new(size)?,
            min_size: 5,
        })
    }

    pub fn init(&mut self, x_array: &[f64], y_array: &[f64]) -> Result<(), GslError> {
        if x_array.len() < self.min_size {
            return Err(GslError::Invalid);
        }
        self.state.init_periodic(x_array, y_array, x_array.len())
    }

    pub fn eval(&self, x_array: &[f64], y_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        self.state.eval(x_array, y_array, x, acc)
    }

    pub fn eval_deriv(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        self.state.eval_deriv(x_array, x, acc)
    }

    pub fn eval_deriv2(&self, x_array: &[f64], x: f64, acc: Option<&mut GslInterpAccel>) -> Result<f64, GslError> {
        self.state.eval_deriv2(x_array, x, acc)
    }

    pub fn eval_integ(
        &self,
        x_array: &[f64],
        y_array: &[f64],
        a: f64,
        b: f64,
        acc: Option<&mut GslInterpAccel>,
    ) -> Result<f64, GslError> {
        self.state.eval_integ(x_array, y_array, a, b, acc)
    }
}