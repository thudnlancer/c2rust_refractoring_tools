use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub enum GslError {
    DomainError(String),
    MemoryError(String),
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GslError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            GslError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
        }
    }
}

impl Error for GslError {}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_EDOM: i32 = 1;
pub const GSL_ENOMEM: i32 = 2;

pub struct GslHistogram2d {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

pub struct GslHistogram2dPdf {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub sum: Vec<f64>,
}

fn find(n: usize, sum: &[f64], r1: f64, k: &mut usize) -> Result<(), GslError> {
    for i in 0..n {
        if r1 >= sum[i] && r1 < sum[i + 1] {
            *k = i;
            return Ok(());
        }
    }
    Err(GslError::DomainError("cannot find r1 in cumulative pdf".into()))
}

impl GslHistogram2dPdf {
    pub fn sample(&self, r1: f64, r2: f64) -> Result<(f64, f64), GslError> {
        let mut r1 = r1;
        let mut r2 = r2;

        if r2 == 1.0 {
            r2 = 0.0;
        }
        if r1 == 1.0 {
            r1 = 0.0;
        }

        let n = self.nx * self.ny;
        let mut k = 0;
        find(n, &self.sum, r1, &mut k)?;

        let i = k / self.ny;
        let j = k - (i * self.ny);
        let delta = (r1 - self.sum[k]) / (self.sum[k + 1] - self.sum[k]);
        let x = self.xrange[i] + delta * (self.xrange[i + 1] - self.xrange[i]);
        let y = self.yrange[j] + r2 * (self.yrange[j + 1] - self.yrange[j]);

        Ok((x, y))
    }

    pub fn alloc(nx: usize, ny: usize) -> Result<Self, GslError> {
        let n = nx * ny;

        if n == 0 {
            return Err(GslError::DomainError(
                "histogram2d pdf length n must be positive integer".into(),
            ));
        }

        let xrange = vec![0.0; nx + 1];
        let yrange = vec![0.0; ny + 1];
        let sum = vec![0.0; n + 1];

        Ok(Self {
            nx,
            ny,
            xrange,
            yrange,
            sum,
        })
    }

    pub fn init(&mut self, h: &GslHistogram2d) -> Result<(), GslError> {
        if self.nx != h.nx || self.ny != h.ny {
            return Err(GslError::DomainError(
                "histogram2d size must match pdf size".into(),
            ));
        }

        for &bin in &h.bin {
            if bin < 0.0 {
                return Err(GslError::DomainError(
                    "histogram bins must be non-negative to compute a probability distribution".into(),
                ));
            }
        }

        self.xrange.copy_from_slice(&h.xrange);
        self.yrange.copy_from_slice(&h.yrange);

        let mut mean = 0.0;
        let n = self.nx * self.ny;

        for i in 0..n {
            mean += (h.bin[i] - mean) / (i + 1) as f64;
        }

        self.sum[0] = 0.0;
        let mut sum = 0.0;

        for i in 0..n {
            sum += (h.bin[i] / mean) / n as f64;
            self.sum[i + 1] = sum;
        }

        Ok(())
    }
}

impl Drop for GslHistogram2dPdf {
    fn drop(&mut self) {
        // Rust's Vec handles memory deallocation automatically
    }
}