//! Routines to find maximum and minimum content of a 2D histogram.
//! Contains the routines:
//! - `histogram2d_max_val`: find max content values
//! - `histogram2d_min_val`: find min content values
//! - `histogram2d_max_bin`: find coordinates of max contents bin
//! - `histogram2d_min_bin`: find coordinates of min contents bin

/// Represents a 2D histogram
pub struct Histogram2D {
    nx: usize,
    ny: usize,
    bin: Vec<f64>,
}

impl Histogram2D {
    /// Creates a new 2D histogram with given dimensions
    pub fn new(nx: usize, ny: usize) -> Self {
        Histogram2D {
            nx,
            ny,
            bin: vec![0.0; nx * ny],
        }
    }

    /// Return the maximum contents value of the 2D histogram
    pub fn max_val(&self) -> f64 {
        self.bin
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Find the bin indices for maximum value of the 2D histogram
    pub fn max_bin(&self) -> (usize, usize) {
        let mut max = f64::NEG_INFINITY;
        let mut imax = 0;
        let mut jmax = 0;

        for i in 0..self.nx {
            for j in 0..self.ny {
                let val = self.bin[i * self.ny + j];
                if val > max {
                    max = val;
                    imax = i;
                    jmax = j;
                }
            }
        }

        (imax, jmax)
    }

    /// Return the minimum contents value of the 2D histogram
    pub fn min_val(&self) -> f64 {
        self.bin
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min)
    }

    /// Find the bin indices for minimum value of the 2D histogram
    pub fn min_bin(&self) -> (usize, usize) {
        let mut min = f64::INFINITY;
        let mut imin = 0;
        let mut jmin = 0;

        for i in 0..self.nx {
            for j in 0..self.ny {
                let val = self.bin[i * self.ny + j];
                if val < min {
                    min = val;
                    imin = i;
                    jmin = j;
                }
            }
        }

        (imin, jmin)
    }
}