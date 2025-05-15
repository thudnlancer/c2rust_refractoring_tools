use std::error::Error;
use std::fmt;

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

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug)]
pub struct Histogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

#[derive(Debug)]
pub struct HistogramPdf {
    n: usize,
    range: Vec<f64>,
    sum: Vec<f64>,
}

fn find(n: usize, range: &[f64], x: f64) -> Result<usize, GslError> {
    if x < range[0] {
        return Err(GslError::Domain);
    }
    if x >= range[n] {
        return Err(GslError::Range);
    }

    let u = (x - range[0]) / (range[n] - range[0]);
    let i_linear = (u * n as f64) as usize;

    if x >= range[i_linear] && x < range[i_linear + 1] {
        return Ok(i_linear);
    }

    let mut upper = n;
    let mut lower = 0;

    while upper - lower > 1 {
        let mid = (upper + lower) / 2;
        if x >= range[mid] {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    if x < range[lower] || x >= range[lower + 1] {
        return Err(GslError::Sanity);
    }

    Ok(lower)
}

impl HistogramPdf {
    pub fn sample(&self, r: f64) -> Result<f64, GslError> {
        let r = if r == 1.0 { 0.0 } else { r };
        let i = find(self.n, &self.sum, r).map_err(|_| {
            GslError::Domain
        })?;

        let delta = (r - self.sum[i]) / (self.sum[i + 1] - self.sum[i]);
        let x = self.range[i] + delta * (self.range[i + 1] - self.range[i]);

        Ok(x)
    }

    pub fn new(n: usize) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::Domain);
        }

        Ok(Self {
            n,
            range: vec![0.0; n + 1],
            sum: vec![0.0; n + 1],
        })
    }

    pub fn init(&mut self, h: &Histogram) -> Result<(), GslError> {
        if self.n != h.n {
            return Err(GslError::Invalid);
        }

        for &bin in &h.bin {
            if bin < 0.0 {
                return Err(GslError::Domain);
            }
        }

        self.range.copy_from_slice(&h.range);

        let mut mean = 0.0;
        let mut sum = 0.0;

        for (i, &bin) in h.bin.iter().enumerate() {
            mean += (bin - mean) / (i + 1) as f64;
        }

        self.sum[0] = 0.0;
        for (i, &bin) in h.bin.iter().enumerate() {
            sum += bin / mean / self.n as f64;
            self.sum[i + 1] = sum;
        }

        Ok(())
    }
}