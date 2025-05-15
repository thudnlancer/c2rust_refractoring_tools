use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HistogramError {
    DomainError(String),
    MemoryAllocationError(String),
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistogramError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            HistogramError::MemoryAllocationError(msg) => write!(f, "Memory allocation error: {}", msg),
        }
    }
}

impl Error for HistogramError {}

pub struct Histogram {
    range: Vec<f64>,
    bin: Vec<f64>,
    n: usize,
}

impl Histogram {
    pub fn calloc_range(n: usize, range: &[f64]) -> Result<Self, HistogramError> {
        // check arguments
        if n == 0 {
            return Err(HistogramError::DomainError(
                "histogram length n must be positive integer".to_string(),
            ));
        }

        // check ranges
        for i in 0..n {
            if range[i] >= range[i + 1] {
                return Err(HistogramError::DomainError(
                    "histogram bin extremes must be in increasing order".to_string(),
                ));
            }
        }

        // Allocate histogram
        let mut h_range = Vec::with_capacity(n + 1);
        let mut h_bin = Vec::with_capacity(n);

        // initialize ranges
        h_range.extend_from_slice(&range[..=n]);

        // clear contents
        h_bin.resize(n, 0.0);

        Ok(Histogram {
            range: h_range,
            bin: h_bin,
            n,
        })
    }
}