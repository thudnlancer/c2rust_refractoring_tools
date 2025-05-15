use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SumLevinError {
    DomainError(String),
    MemoryAllocationError(String),
}

impl fmt::Display for SumLevinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SumLevinError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            SumLevinError::MemoryAllocationError(msg) => write!(f, "Memory allocation error: {}", msg),
        }
    }
}

impl Error for SumLevinError {}

pub struct SumLevinUWorkspace {
    q_num: Vec<f64>,
    q_den: Vec<f64>,
    dq_num: Vec<f64>,
    dq_den: Vec<f64>,
    dsum: Vec<f64>,
    size: usize,
    terms_used: usize,
    sum_plain: f64,
}

impl SumLevinUWorkspace {
    pub fn new(n: usize) -> Result<Self, SumLevinError> {
        if n == 0 {
            return Err(SumLevinError::DomainError(
                "length n must be positive integer".to_string(),
            ));
        }

        let q_num = vec![0.0; n];
        let q_den = vec![0.0; n];
        let dq_num = vec![0.0; n * n];
        let dq_den = vec![0.0; n * n];
        let dsum = vec![0.0; n];

        Ok(Self {
            q_num,
            q_den,
            dq_num,
            dq_den,
            dsum,
            size: n,
            terms_used: 0,
            sum_plain: 0.0,
        })
    }
}

impl Drop for SumLevinUWorkspace {
    fn drop(&mut self) {
        // All memory is automatically managed by Vec
    }
}