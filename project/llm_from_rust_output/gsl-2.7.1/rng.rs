use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
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
pub struct GslRngType {
    pub name: CString,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: Box<dyn Fn(&mut [u8], u64)>,
    pub get: Box<dyn Fn(&[u8]) -> u64>,
    pub get_double: Box<dyn Fn(&[u8]) -> f64>,
}

#[derive(Debug, Clone)]
pub struct GslRng {
    pub type_: &'static GslRngType,
    pub state: Vec<u8>,
}

impl GslRng {
    pub fn new(rng_type: &'static GslRngType) -> Result<Self, GslError> {
        let mut state = vec![0; rng_type.size];
        (rng_type.set)(&mut state, 0); // Default seed will be set later
        let mut rng = Self {
            type_: rng_type,
            state,
        };
        rng.set(0); // Set default seed
        Ok(rng)
    }

    pub fn clone(&self) -> Result<Self, GslError> {
        Ok(Self {
            type_: self.type_,
            state: self.state.clone(),
        })
    }

    pub fn set(&mut self, seed: u64) {
        (self.type_.set)(&mut self.state, seed);
    }

    pub fn max(&self) -> u64 {
        self.type_.max
    }

    pub fn min(&self) -> u64 {
        self.type_.min
    }

    pub fn name(&self) -> &CString {
        &self.type_.name
    }

    pub fn size(&self) -> usize {
        self.type_.size
    }

    pub fn state(&self) -> &[u8] {
        &self.state
    }

    pub fn print_state(&self) {
        for byte in &self.state {
            print!("{:02x}", byte);
        }
        println!();
    }
}

pub fn gsl_rng_memcpy(dest: &mut GslRng, src: &GslRng) -> Result<(), GslError> {
    if dest.type_ != src.type_ {
        return Err(GslError::Invalid);
    }
    dest.state.copy_from_slice(&src.state);
    Ok(())
}