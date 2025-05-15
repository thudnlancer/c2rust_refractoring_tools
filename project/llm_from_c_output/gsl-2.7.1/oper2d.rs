//! Operations on 2D histograms
//! 
//! Provides routines for:
//! - Checking if two histograms have the same binning
//! - Adding two histograms
//! - Subtracting two histograms
//! - Multiplying two histograms
//! - Dividing two histograms
//! - Scaling histogram contents
//! - Shifting histogram contents

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HistogramError {
    DifferentBinning,
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistogramError::DifferentBinning => write!(f, "histograms have different binning"),
        }
    }
}

impl Error for HistogramError {}

pub struct Histogram2D {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

impl Histogram2D {
    /// Check if two histograms have the same binning
    pub fn equal_bins(&self, other: &Histogram2D) -> bool {
        if self.nx != other.nx || self.ny != other.ny {
            return false;
        }

        if self.xrange.len() != other.xrange.len() || self.yrange.len() != other.yrange.len() {
            return false;
        }

        for i in 0..=self.nx {
            if (self.xrange[i] - other.xrange[i]).abs() > std::f64::EPSILON {
                return false;
            }
        }

        for i in 0..=self.ny {
            if (self.yrange[i] - other.yrange[i]).abs() > std::f64::EPSILON {
                return false;
            }
        }

        true
    }

    /// Add another histogram to this one
    pub fn add(&mut self, other: &Histogram2D) -> Result<(), HistogramError> {
        if !self.equal_bins(other) {
            return Err(HistogramError::DifferentBinning);
        }

        for i in 0..(self.nx * self.ny) {
            self.bin[i] += other.bin[i];
        }

        Ok(())
    }

    /// Subtract another histogram from this one
    pub fn sub(&mut self, other: &Histogram2D) -> Result<(), HistogramError> {
        if !self.equal_bins(other) {
            return Err(HistogramError::DifferentBinning);
        }

        for i in 0..(self.nx * self.ny) {
            self.bin[i] -= other.bin[i];
        }

        Ok(())
    }

    /// Multiply this histogram by another one
    pub fn mul(&mut self, other: &Histogram2D) -> Result<(), HistogramError> {
        if !self.equal_bins(other) {
            return Err(HistogramError::DifferentBinning);
        }

        for i in 0..(self.nx * self.ny) {
            self.bin[i] *= other.bin[i];
        }

        Ok(())
    }

    /// Divide this histogram by another one
    pub fn div(&mut self, other: &Histogram2D) -> Result<(), HistogramError> {
        if !self.equal_bins(other) {
            return Err(HistogramError::DifferentBinning);
        }

        for i in 0..(self.nx * self.ny) {
            self.bin[i] /= other.bin[i];
        }

        Ok(())
    }

    /// Scale this histogram by a factor
    pub fn scale(&mut self, factor: f64) {
        for bin in &mut self.bin {
            *bin *= factor;
        }
    }

    /// Shift this histogram by an offset
    pub fn shift(&mut self, offset: f64) {
        for bin in &mut self.bin {
            *bin += offset;
        }
    }
}