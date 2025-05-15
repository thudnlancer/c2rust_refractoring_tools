/* 
 * gsl_histogram2d_copy.rs
 * 
 * Routine to copy a 2D histogram.
 * Translated from C to Rust with safety and idiomatic practices.
 */

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HistogramError {
    SizeMismatch,
    AllocationFailed,
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HistogramError::SizeMismatch => 
                write!(f, "histograms have different sizes, cannot copy"),
            HistogramError::AllocationFailed => 
                write!(f, "failed to allocate space for histogram struct"),
        }
    }
}

impl Error for HistogramError {}

pub struct Histogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram2D {
    pub fn memcpy(&mut self, src: &Histogram2D) -> Result<(), HistogramError> {
        if self.nx != src.nx || self.ny != src.ny {
            return Err(HistogramError::SizeMismatch);
        }

        self.xrange.copy_from_slice(&src.xrange);
        self.yrange.copy_from_slice(&src.yrange);
        self.bin.copy_from_slice(&src.bin);

        Ok(())
    }

    pub fn clone(src: &Histogram2D) -> Result<Self, HistogramError> {
        let nx = src.nx;
        let ny = src.ny;

        let mut h = Histogram2D {
            nx,
            ny,
            xrange: src.xrange.clone(),
            yrange: src.yrange.clone(),
            bin: vec![0.0; nx * ny],
        };

        h.bin.copy_from_slice(&src.bin);

        Ok(h)
    }

    // Placeholder for allocation function - would be implemented based on actual needs
    fn calloc_range(nx: usize, ny: usize, xrange: &[f64], yrange: &[f64]) -> Result<Self, HistogramError> {
        if xrange.len() != nx + 1 || yrange.len() != ny + 1 {
            return Err(HistogramError::AllocationFailed);
        }

        Ok(Histogram2D {
            nx,
            ny,
            xrange: xrange.to_vec(),
            yrange: yrange.to_vec(),
            bin: vec![0.0; nx * ny],
        })
    }
}