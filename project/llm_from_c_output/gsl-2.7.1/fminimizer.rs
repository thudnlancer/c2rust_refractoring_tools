use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub enum MinimizerError {
    AllocationFailed,
    InitializationFailed,
    IncompatibleSize,
}

impl fmt::Display for MinimizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinimizerError::AllocationFailed => write!(f, "failed to allocate space"),
            MinimizerError::InitializationFailed => write!(f, "failed to initialize"),
            MinimizerError::IncompatibleSize => write!(f, "incompatible sizes"),
        }
    }
}

impl Error for MinimizerError {}

pub struct MinimizerType {
    size: usize,
    name: &'static str,
    alloc: fn(&mut [u8], usize) -> Result<(), MinimizerError>,
    set: fn(&mut [u8], &dyn Fn(&[f64]) -> f64, &mut [f64], &mut f64, &[f64]) -> Result<(), MinimizerError>,
    iterate: fn(&mut [u8], &dyn Fn(&[f64]) -> f64, &mut [f64], &mut f64, &mut f64) -> Result<(), MinimizerError>,
    free: fn(&mut [u8]),
}

pub struct Minimizer {
    typ: &'static MinimizerType,
    state: Vec<u8>,
    x: Vec<f64>,
    size: f64,
    fval: f64,
    f: Option<Box<dyn Fn(&[f64]) -> f64>>,
}

impl Minimizer {
    pub fn new(typ: &'static MinimizerType, n: usize) -> Result<Self, MinimizerError> {
        let mut state = vec![0u8; typ.size];
        let x = vec![0.0; n];
        
        (typ.alloc)(&mut state, n)?;
        
        Ok(Minimizer {
            typ,
            state,
            x,
            size: 0.0,
            fval: 0.0,
            f: None,
        })
    }

    pub fn set(&mut self, f: Box<dyn Fn(&[f64]) -> f64>, x: &[f64], step_size: &[f64]) -> Result<(), MinimizerError> {
        if self.x.len() != x.len() || self.x.len() != step_size.len() {
            return Err(MinimizerError::IncompatibleSize);
        }

        self.f = Some(f);
        self.x.copy_from_slice(x);

        (self.typ.set)(&mut self.state, self.f.as_ref().unwrap(), &mut self.x, &mut self.size, step_size)
    }

    pub fn iterate(&mut self) -> Result<(), MinimizerError> {
        (self.typ.iterate)(&mut self.state, self.f.as_ref().unwrap(), &mut self.x, &mut self.size, &mut self.fval)
    }

    pub fn name(&self) -> &'static str {
        self.typ.name
    }

    pub fn x(&self) -> &[f64] {
        &self.x
    }

    pub fn minimum(&self) -> f64 {
        self.fval
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}

impl Drop for Minimizer {
    fn drop(&mut self) {
        (self.typ.free)(&mut self.state);
    }
}