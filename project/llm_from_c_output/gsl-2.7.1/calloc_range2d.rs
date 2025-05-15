use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Histogram2D {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

#[derive(Debug)]
pub enum HistogramError {
    DomainError(String),
    AllocationError(String),
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HistogramError::DomainError(ref msg) => write!(f, "Domain error: {}", msg),
            HistogramError::AllocationError(ref msg) => write!(f, "Allocation error: {}", msg),
        }
    }
}

impl Error for HistogramError {}

pub fn histogram2d_calloc_range(
    nx: usize,
    ny: usize,
    xrange: &[f64],
    yrange: &[f64],
) -> Result<Histogram2D, HistogramError> {
    // Check arguments
    if nx == 0 {
        return Err(HistogramError::DomainError(
            "histogram length nx must be positive integer".to_string(),
        ));
    }

    if ny == 0 {
        return Err(HistogramError::DomainError(
            "histogram length ny must be positive integer".to_string(),
        ));
    }

    // Check ranges
    for i in 0..nx {
        if xrange[i] >= xrange[i + 1] {
            return Err(HistogramError::DomainError(
                "histogram xrange not in increasing order".to_string(),
            ));
        }
    }

    for j in 0..ny {
        if yrange[j] >= yrange[j + 1] {
            return Err(HistogramError::DomainError(
                "histogram yrange not in increasing order".to_string(),
            ));
        }
    }

    // Allocate histogram
    let mut xrange_vec = Vec::with_capacity(nx + 1);
    let mut yrange_vec = Vec::with_capacity(ny + 1);
    let mut bin_vec = Vec::with_capacity(nx * ny);

    // Initialize ranges
    xrange_vec.extend_from_slice(&xrange[..=nx]);
    yrange_vec.extend_from_slice(&yrange[..=ny]);

    // Initialize bins with zeros
    bin_vec.resize(nx * ny, 0.0);

    Ok(Histogram2D {
        nx,
        ny,
        xrange: xrange_vec,
        yrange: yrange_vec,
        bin: bin_vec,
    })
}