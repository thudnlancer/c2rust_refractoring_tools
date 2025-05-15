use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GslError {
    DomainError(String),
    MemoryError(String),
    InvalidArgument(String),
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GslError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            GslError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            GslError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl Error for GslError {}

pub struct GslHistogram {
    pub n: usize,
    pub range: Vec<f64>,
    pub bin: Vec<f64>,
}

pub struct GslHistogramPdf {
    pub n: usize,
    pub range: Vec<f64>,
    pub sum: Vec<f64>,
}

impl GslHistogramPdf {
    pub fn sample(&self, r: f64) -> Result<f64, GslError> {
        let mut r = r;
        
        if r == 1.0 {
            r = 0.0;
        }

        let i = match find(&self.sum, r) {
            Ok(idx) => idx,
            Err(_) => return Err(GslError::DomainError("cannot find r in cumulative pdf".to_string())),
        };

        let delta = (r - self.sum[i]) / (self.sum[i + 1] - self.sum[i]);
        let x = self.range[i] + delta * (self.range[i + 1] - self.range[i]);
        Ok(x)
    }

    pub fn alloc(n: usize) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::DomainError(
                "histogram pdf length n must be positive integer".to_string(),
            ));
        }

        let range = vec![0.0; n + 1];
        let sum = vec![0.0; n + 1];

        Ok(Self { n, range, sum })
    }

    pub fn init(&mut self, h: &GslHistogram) -> Result<(), GslError> {
        if self.n != h.n {
            return Err(GslError::InvalidArgument(
                "histogram length must match pdf length".to_string(),
            ));
        }

        for &bin in &h.bin {
            if bin < 0.0 {
                return Err(GslError::DomainError(
                    "histogram bins must be non-negative to compute a probability distribution".to_string(),
                ));
            }
        }

        self.range.copy_from_slice(&h.range[..=self.n]);

        let mut mean = 0.0;
        let mut sum = 0.0;

        for i in 0..self.n {
            mean += (h.bin[i] - mean) / (i + 1) as f64;
        }

        self.sum[0] = 0.0;

        for i in 0..self.n {
            sum += (h.bin[i] / mean) / self.n as f64;
            self.sum[i + 1] = sum;
        }

        Ok(())
    }
}

fn find(sum: &[f64], r: f64) -> Result<usize, ()> {
    for (i, &val) in sum.iter().enumerate() {
        if r <= val {
            return Ok(i);
        }
    }
    Err(())
}