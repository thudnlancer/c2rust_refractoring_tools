use std::f64;

#[derive(Debug, Clone)]
pub struct ChebSeries {
    c: Vec<f64>,
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
    f: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl ChebSeries {
    pub fn eval(&self, x: f64) -> f64 {
        let y = (2.0 * x - self.a - self.b) / (self.b - self.a);
        let y2 = 2.0 * y;
        
        let (d1, d2) = (1..=self.order).rev().fold((0.0, 0.0), |(d1, d2), i| {
            (y2 * d1 - d2 + self.c[i], d1)
        });
        
        y * d1 - d2 + 0.5 * self.c[0]
    }

    pub fn eval_n(&self, n: usize, x: f64) -> f64 {
        let eval_order = n.min(self.order);
        let y = (2.0 * x - self.a - self.b) / (self.b - self.a);
        let y2 = 2.0 * y;
        
        let (d1, d2) = (1..=eval_order).rev().fold((0.0, 0.0), |(d1, d2), i| {
            (y2 * d1 - d2 + self.c[i], d1)
        });
        
        y * d1 - d2 + 0.5 * self.c[0]
    }

    pub fn eval_err(&self, x: f64) -> (f64, f64, GslError) {
        let result = self.eval(x);
        let absc = self.c.iter().map(|&ci| ci.abs()).sum::<f64>();
        let abserr = self.c[self.order].abs() + absc * f64::EPSILON;
        (result, abserr, GslError::Success)
    }

    pub fn eval_n_err(&self, n: usize, x: f64) -> (f64, f64, GslError) {
        let eval_order = n.min(self.order);
        let result = self.eval_n(eval_order, x);
        let absc = self.c[..=eval_order].iter().map(|&ci| ci.abs()).sum::<f64>();
        let abserr = self.c[eval_order].abs() + absc * f64::EPSILON;
        (result, abserr, GslError::Success)
    }

    pub fn eval_mode_e(&self, x: f64, mode: u32) -> (f64, f64, GslError) {
        let eval_order = if mode & 7 == 0 { self.order } else { self.order_sp };
        let result = self.eval_n(eval_order, x);
        let absc = self.c[..=eval_order].iter().map(|&ci| ci.abs()).sum::<f64>();
        let abserr = self.c[eval_order].abs() + absc * f64::EPSILON;
        (result, abserr, GslError::Success)
    }

    pub fn eval_mode(&self, x: f64, mode: u32) -> Result<f64, GslError> {
        let (result, _, status) = self.eval_mode_e(x, mode);
        if status == GslError::Success {
            Ok(result)
        } else {
            Err(status)
        }
    }
}