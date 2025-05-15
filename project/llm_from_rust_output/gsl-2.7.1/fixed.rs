use std::f64;
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
pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

#[derive(Debug, Clone)]
pub struct GslIntegrationFixedParams {
    pub alpha: f64,
    pub beta: f64,
    pub a: f64,
    pub b: f64,
    pub zemu: f64,
    pub shft: f64,
    pub slp: f64,
    pub al: f64,
    pub be: f64,
}

pub trait GslIntegrationFixedType {
    fn check(&self, n: usize, params: &GslIntegrationFixedParams) -> Result<(), GslError>;
    fn init(
        &self,
        n: usize,
        diag: &mut [f64],
        subdiag: &mut [f64],
        params: &mut GslIntegrationFixedParams,
    ) -> Result<(), GslError>;
}

#[derive(Debug, Clone)]
pub struct GslIntegrationFixedWorkspace<T: GslIntegrationFixedType> {
    n: usize,
    weights: Vec<f64>,
    x: Vec<f64>,
    diag: Vec<f64>,
    subdiag: Vec<f64>,
    type_: T,
}

impl<T: GslIntegrationFixedType> GslIntegrationFixedWorkspace<T> {
    pub fn new(
        type_: T,
        n: usize,
        a: f64,
        b: f64,
        alpha: f64,
        beta: f64,
    ) -> Result<Self, GslError> {
        if n < 1 {
            return Err(GslError::Domain);
        }

        let mut workspace = Self {
            n,
            weights: vec![0.0; n],
            x: vec![0.0; n],
            diag: vec![0.0; n],
            subdiag: vec![0.0; n],
            type_,
        };

        workspace.compute(a, b, alpha, beta)?;
        Ok(workspace)
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn nodes(&self) -> &[f64] {
        &self.x
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn integrate(&self, func: &GslFunction) -> f64 {
        self.weights
            .iter()
            .zip(self.x.iter())
            .map(|(w, x)| w * (func.function)(*x))
            .sum()
    }

    fn compute(&mut self, a: f64, b: f64, alpha: f64, beta: f64) -> Result<(), GslError> {
        let mut params = GslIntegrationFixedParams {
            alpha,
            beta,
            a,
            b,
            zemu: 0.0,
            shft: 0.0,
            slp: 0.0,
            al: 0.0,
            be: 0.0,
        };

        self.type_.check(self.n, &params)?;
        self.type_
            .init(self.n, &mut self.diag, &mut self.subdiag, &mut params)?;

        if params.zemu <= 0.0 {
            return Err(GslError::Invalid);
        }

        self.x.copy_from_slice(&self.diag);
        self.weights[0] = params.zemu.sqrt();
        for w in self.weights.iter_mut().skip(1) {
            *w = 0.0;
        }

        imtqlx(&mut self.x, &mut self.subdiag, &mut self.weights)?;

        for w in self.weights.iter_mut() {
            *w *= *w;
        }

        let p = params.slp.powf(params.al + params.be + 1.0);
        for (x, w) in self.x.iter_mut().zip(self.weights.iter_mut()) {
            *x = params.shft + params.slp * *x;
            *w *= p;
        }

        Ok(())
    }
}

fn imtqlx(d: &mut [f64], e: &mut [f64], z: &mut [f64]) -> Result<(), GslError> {
    let n = d.len();
    if n == 1 {
        return Ok(());
    }

    e[n - 1] = 0.0;
    let mut l = 0;
    let itn = 30;

    while l < n {
        let mut j = 0;
        let mut m = l;
        while m < n {
            if m == n - 1 {
                break;
            }
            if e[m].abs() <= f64::EPSILON * (d[m].abs() + d[m + 1].abs()) {
                break;
            }
            m += 1;
        }

        if m == l {
            l += 1;
            continue;
        }

        if j >= itn {
            return Err(GslError::MaxIter);
        }
        j += 1;

        let mut g = (d[l + 1] - d[l]) / (2.0 * e[l]);
        let mut r = (g * g + 1.0).sqrt();
        g = d[m] - d[l] + e[l] / (g + r * g.signum());
        let mut s = 1.0;
        let mut c = 1.0;
        let mut p = 0.0;

        for i in (l..m).rev() {
            let f = s * e[i];
            let b = c * e[i];
            if g.abs() <= f.abs() {
                c = g / f;
                r = (c * c + 1.0).sqrt();
                e[i + 1] = f * r;
                s = 1.0 / r;
                c *= s;
            } else {
                s = f / g;
                r = (s * s + 1.0).sqrt();
                e[i + 1] = g * r;
                c = 1.0 / r;
                s *= c;
            }

            g = d[i + 1] - p;
            r = (d[i] - g) * s + 2.0 * c * b;
            p = s * r;
            d[i + 1] = g + p;
            g = c * r - b;

            let f = z[i + 1];
            z[i + 1] = s * z[i] + c * f;
            z[i] = c * z[i] - s * f;
        }

        d[l] -= p;
        e[l] = g;
        e[m] = 0.0;
    }

    for i in 1..n {
        let mut k = i;
        let mut p = d[i];
        for j in i + 1..=n {
            if d[j - 1] < p {
                k = j;
                p = d[j - 1];
            }
        }
        if k != i {
            d.swap(k - 1, i);
            z.swap(k - 1, i);
        }
    }

    Ok(())
}