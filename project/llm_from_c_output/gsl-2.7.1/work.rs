use std::error::Error;
use std::fmt;
use ndarray::{Array2, Array1};

#[derive(Debug)]
pub enum MultifitError {
    AllocationFailed(String),
}

impl fmt::Display for MultifitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultifitError::AllocationFailed(msg) => write!(f, "failed to allocate space: {}", msg),
        }
    }
}

impl Error for MultifitError {}

pub struct MultifitLinearWorkspace {
    nmax: usize,    // max number of observations
    pmax: usize,    // max number of parameters
    n: usize,
    p: usize,
    rcond: f64,
    a: Array2<f64>,
    q: Array2<f64>,
    qsi: Array2<f64>,
    s: Array1<f64>,
    t: Array1<f64>,
    xt: Array1<f64>,
    d: Array1<f64>,
}

impl MultifitLinearWorkspace {
    pub fn new(nmax: usize, pmax: usize) -> Result<Self, MultifitError> {
        let a = Array2::zeros((nmax, pmax));
        let q = Array2::zeros((pmax, pmax));
        let qsi = Array2::zeros((pmax, pmax));
        let s = Array1::zeros(pmax);
        let t = Array1::zeros(nmax);
        let xt = Array1::zeros(pmax);
        let d = Array1::zeros(pmax);

        Ok(Self {
            nmax,
            pmax,
            n: 0,
            p: 0,
            rcond: 0.0,
            a,
            q,
            qsi,
            s,
            t,
            xt,
            d,
        })
    }
}

impl Drop for MultifitLinearWorkspace {
    fn drop(&mut self) {
        // All resources are automatically dropped by Rust's ownership system
    }
}