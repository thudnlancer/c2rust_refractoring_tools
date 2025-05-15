use std::f64::consts;

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

#[derive(Debug, Copy, Clone)]
pub struct IntegrationFixedParams {
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

pub trait IntegrationFixedType {
    fn check(&self, n: usize, params: &IntegrationFixedParams) -> Result<(), GslError>;
    fn init(
        &self,
        n: usize,
        diag: &mut [f64],
        subdiag: &mut [f64],
        params: &mut IntegrationFixedParams,
    ) -> Result<(), GslError>;
}

pub struct ExponentialType;

impl IntegrationFixedType for ExponentialType {
    fn check(&self, _n: usize, params: &IntegrationFixedParams) -> Result<(), GslError> {
        if (params.b - params.a).abs() <= 2.2204460492503131e-16 {
            return Err(GslError::Domain);
        } else if params.a >= params.b {
            return Err(GslError::Domain);
        } else if params.alpha <= -1.0 {
            return Err(GslError::Domain);
        }
        Ok(())
    }

    fn init(
        &self,
        n: usize,
        diag: &mut [f64],
        subdiag: &mut [f64],
        params: &mut IntegrationFixedParams,
    ) -> Result<(), GslError> {
        let mut a2i = params.alpha;
        for i in 1..=n {
            diag[i - 1] = 0.0;
            a2i += 2.0;
            subdiag[i - 1] = (i as f64 + params.alpha * ((i % 2) as f64)) / (a2i * a2i - 1.0).sqrt();
        }

        params.zemu = 2.0 / (params.alpha + 1.0);
        params.shft = 0.5 * (params.b + params.a);
        params.slp = 0.5 * (params.b - params.a);
        params.al = params.alpha;
        params.be = 0.0;

        Ok(())
    }
}

pub static GSL_INTEGRATION_FIXED_EXPONENTIAL: ExponentialType = ExponentialType;