use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug, Clone)]
pub struct Histogram2D {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

impl Histogram2D {
    pub fn equal_bins(&self, other: &Self) -> bool {
        if self.nx != other.nx || self.ny != other.ny {
            return false;
        }
        
        self.xrange.iter().zip(other.xrange.iter()).all(|(a, b)| a == b) &&
        self.yrange.iter().zip(other.yrange.iter()).all(|(a, b)| a == b)
    }

    pub fn add(&mut self, other: &Self) -> Result<(), GslError> {
        if !self.equal_bins(other) {
            return Err(GslError::Inval);
        }
        
        self.bin.iter_mut()
            .zip(other.bin.iter())
            .for_each(|(a, b)| *a += b);
        
        Ok(())
    }

    pub fn sub(&mut self, other: &Self) -> Result<(), GslError> {
        if !self.equal_bins(other) {
            return Err(GslError::Inval);
        }
        
        self.bin.iter_mut()
            .zip(other.bin.iter())
            .for_each(|(a, b)| *a -= b);
        
        Ok(())
    }

    pub fn mul(&mut self, other: &Self) -> Result<(), GslError> {
        if !self.equal_bins(other) {
            return Err(GslError::Inval);
        }
        
        self.bin.iter_mut()
            .zip(other.bin.iter())
            .for_each(|(a, b)| *a *= b);
        
        Ok(())
    }

    pub fn div(&mut self, other: &Self) -> Result<(), GslError> {
        if !self.equal_bins(other) {
            return Err(GslError::Inval);
        }
        
        self.bin.iter_mut()
            .zip(other.bin.iter())
            .for_each(|(a, b)| *a /= b);
        
        Ok(())
    }

    pub fn scale(&mut self, scale: f64) -> Result<(), GslError> {
        self.bin.iter_mut().for_each(|a| *a *= scale);
        Ok(())
    }

    pub fn shift(&mut self, shift: f64) -> Result<(), GslError> {
        self.bin.iter_mut().for_each(|a| *a += shift);
        Ok(())
    }
}