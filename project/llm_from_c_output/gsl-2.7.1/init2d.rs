use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Histogram2D {
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
    nx: usize,
    ny: usize,
}

#[derive(Debug)]
pub enum HistogramError {
    DomainError(String),
    InvalidValue(String),
    OutOfMemory(String),
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistogramError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            HistogramError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            HistogramError::OutOfMemory(msg) => write!(f, "Out of memory: {}", msg),
        }
    }
}

impl Error for HistogramError {}

impl Histogram2D {
    pub fn new(nx: usize, ny: usize) -> Result<Self, HistogramError> {
        if nx == 0 {
            return Err(HistogramError::DomainError(
                "histogram2d length nx must be positive integer".to_string(),
            ));
        }

        if ny == 0 {
            return Err(HistogramError::DomainError(
                "histogram2d length ny must be positive integer".to_string(),
            ));
        }

        let xrange = vec![0.0; nx + 1];
        let yrange = vec![0.0; ny + 1];
        let bin = vec![0.0; nx * ny];

        Ok(Self {
            xrange,
            yrange,
            bin,
            nx,
            ny,
        })
    }

    pub fn new_uniform(
        nx: usize,
        ny: usize,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
    ) -> Result<Self, HistogramError> {
        if xmin >= xmax {
            return Err(HistogramError::InvalidValue(
                "xmin must be less than xmax".to_string(),
            ));
        }

        if ymin >= ymax {
            return Err(HistogramError::InvalidValue(
                "ymin must be less than ymax".to_string(),
            ));
        }

        let mut h = Self::new(nx, ny)?;
        Self::make_uniform(&mut h.xrange, nx, xmin, xmax);
        Self::make_uniform(&mut h.yrange, ny, ymin, ymax);

        Ok(h)
    }

    pub fn new_default(nx: usize, ny: usize) -> Result<Self, HistogramError> {
        let mut h = Self::new(nx, ny)?;

        for i in 0..=nx {
            h.xrange[i] = i as f64;
        }

        for i in 0..=ny {
            h.yrange[i] = i as f64;
        }

        for i in 0..nx * ny {
            h.bin[i] = 0.0;
        }

        Ok(h)
    }

    fn make_uniform(range: &mut [f64], n: usize, min: f64, max: f64) {
        for i in 0..=n {
            let f1 = (n - i) as f64 / n as f64;
            let f2 = i as f64 / n as f64;
            range[i] = f1 * min + f2 * max;
        }
    }

    pub fn set_ranges_uniform(
        &mut self,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
    ) -> Result<(), HistogramError> {
        if xmin >= xmax {
            return Err(HistogramError::InvalidValue(
                "xmin must be less than xmax".to_string(),
            ));
        }

        if ymin >= ymax {
            return Err(HistogramError::InvalidValue(
                "ymin must be less than ymax".to_string(),
            ));
        }

        Self::make_uniform(&mut self.xrange, self.nx, xmin, xmax);
        Self::make_uniform(&mut self.yrange, self.ny, ymin, ymax);

        for i in 0..self.nx * self.ny {
            self.bin[i] = 0.0;
        }

        Ok(())
    }

    pub fn set_ranges(
        &mut self,
        xrange: &[f64],
        yrange: &[f64],
    ) -> Result<(), HistogramError> {
        if xrange.len() != self.nx + 1 {
            return Err(HistogramError::InvalidValue(
                "size of xrange must match size of histogram".to_string(),
            ));
        }

        if yrange.len() != self.ny + 1 {
            return Err(HistogramError::InvalidValue(
                "size of yrange must match size of histogram".to_string(),
            ));
        }

        self.xrange.copy_from_slice(xrange);
        self.yrange.copy_from_slice(yrange);

        for i in 0..self.nx * self.ny {
            self.bin[i] = 0.0;
        }

        Ok(())
    }
}