use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum WaveletError {
    AllocationFailed,
    InvalidWavelet,
    InvalidLength,
}

impl fmt::Display for WaveletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WaveletError::AllocationFailed => write!(f, "failed to allocate space"),
            WaveletError::InvalidWavelet => write!(f, "invalid wavelet member"),
            WaveletError::InvalidLength => write!(f, "length n must be positive integer"),
        }
    }
}

impl Error for WaveletError {}

pub struct Wavelet<'a> {
    type_: &'a WaveletType,
    h1: Vec<f64>,
    g1: Vec<f64>,
    h2: Vec<f64>,
    g2: Vec<f64>,
    nc: usize,
    offset: usize,
}

impl<'a> Wavelet<'a> {
    pub fn new(
        wavelet_type: &'a WaveletType,
        k: usize,
    ) -> Result<Self, WaveletError> {
        let mut h1 = Vec::new();
        let mut g1 = Vec::new();
        let mut h2 = Vec::new();
        let mut g2 = Vec::new();
        let mut nc = 0;
        let mut offset = 0;

        if let Err(_) = (wavelet_type.init)(&mut h1, &mut g1, &mut h2, &mut g2, &mut nc, &mut offset, k) {
            return Err(WaveletError::InvalidWavelet);
        }

        Ok(Self {
            type_: wavelet_type,
            h1,
            g1,
            h2,
            g2,
            nc,
            offset,
        })
    }

    pub fn name(&self) -> &str {
        self.type_.name
    }
}

pub struct WaveletType {
    pub name: &'static str,
    pub init: fn(&mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut usize, &mut usize, usize) -> Result<(), ()>,
}

pub struct WaveletWorkspace {
    n: usize,
    scratch: Vec<f64>,
}

impl WaveletWorkspace {
    pub fn new(n: usize) -> Result<Self, WaveletError> {
        if n == 0 {
            return Err(WaveletError::InvalidLength);
        }

        let scratch = vec![0.0; n];
        
        Ok(Self {
            n,
            scratch,
        })
    }
}