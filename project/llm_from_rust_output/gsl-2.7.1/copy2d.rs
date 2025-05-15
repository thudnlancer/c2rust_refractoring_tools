use std::error::Error;
use std::ffi::CString;
use std::ptr;

pub type SizeT = usize;

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
pub struct GslHistogram2d {
    pub nx: SizeT,
    pub ny: SizeT,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

impl GslHistogram2d {
    pub fn new(nx: SizeT, ny: SizeT, xrange: &[f64], yrange: &[f64]) -> Result<Self, Box<dyn Error>> {
        if xrange.len() != nx + 1 || yrange.len() != ny + 1 {
            return Err("Invalid range length".into());
        }

        Ok(Self {
            nx,
            ny,
            xrange: xrange.to_vec(),
            yrange: yrange.to_vec(),
            bin: vec![0.0; nx * ny],
        })
    }

    pub fn memcpy(&mut self, src: &Self) -> Result<(), GslError> {
        if self.nx != src.nx || self.ny != src.ny {
            return Err(GslError::Invalid);
        }

        self.xrange.copy_from_slice(&src.xrange);
        self.yrange.copy_from_slice(&src.yrange);
        self.bin.copy_from_slice(&src.bin);

        Ok(())
    }

    pub fn clone(&self) -> Result<Self, Box<dyn Error>> {
        Self::new(self.nx, self.ny, &self.xrange, &self.yrange).map(|mut h| {
            h.bin.copy_from_slice(&self.bin);
            h
        })
    }
}

fn gsl_error(reason: &str, file: &str, line: i32, errno: GslError) {
    eprintln!("GSL error: {} at {}:{} (code {})", reason, file, line, errno as i32);
}