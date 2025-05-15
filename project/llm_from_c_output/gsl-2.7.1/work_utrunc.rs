use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SumLevinError {
    DomainError,
    MemoryAllocationError,
}

impl fmt::Display for SumLevinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SumLevinError::DomainError => write!(f, "length n must be positive integer"),
            SumLevinError::MemoryAllocationError => write!(f, "memory allocation failed"),
        }
    }
}

impl Error for SumLevinError {}

pub struct SumLevinUtruncWorkspace {
    q_num: Vec<f64>,
    q_den: Vec<f64>,
    dsum: Vec<f64>,
    size: usize,
    terms_used: usize,
    sum_plain: f64,
}

impl SumLevinUtruncWorkspace {
    pub fn new(n: usize) -> Result<Self, SumLevinError> {
        if n == 0 {
            return Err(SumLevinError::DomainError);
        }

        let q_num = vec![0.0; n];
        let q_den = vec![0.0; n];
        let dsum = vec![0.0; n];

        Ok(Self {
            q_num,
            q_den,
            dsum,
            size: n,
            terms_used: 0,
            sum_plain: 0.0,
        })
    }
}

impl Drop for SumLevinUtruncWorkspace {
    fn drop(&mut self) {
        // Rust's Vec handles deallocation automatically
    }
}