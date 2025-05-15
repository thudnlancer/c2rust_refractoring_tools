use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub type SizeT = usize;

#[derive(Debug)]
pub struct Histogram2D {
    pub nx: SizeT,
    pub ny: SizeT,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

#[derive(Debug)]
pub struct Histogram2DPdf {
    pub nx: SizeT,
    pub ny: SizeT,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub sum: Vec<f64>,
}

fn find(n: SizeT, range: &[f64], x: f64) -> Result<SizeT, GslError> {
    if x < range[0] {
        return Err(GslError::Domain);
    }
    if x >= range[n] {
        return Err(GslError::Range);
    }

    let u = (x - range[0]) / (range[n] - range[0]);
    let i_linear = (u * n as f64) as SizeT;

    if x >= range[i_linear] && x < range[i_linear + 1] {
        return Ok(i_linear);
    }

    let mut upper = n;
    let mut lower = 0;

    while upper - lower > 1 {
        let mid = (upper + lower) / 2;
        if x >= range[mid] {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    if x < range[lower] || x >= range[lower + 1] {
        return Err(GslError::Sanity);
    }

    Ok(lower)
}

pub fn histogram2d_pdf_sample(
    p: &Histogram2DPdf,
    r1: f64,
    r2: f64,
) -> Result<(f64, f64), GslError> {
    let r1 = if r1 == 1.0 { 0.0 } else { r1 };
    let r2 = if r2 == 1.0 { 0.0 } else { r2 };

    let k = find(p.nx * p.ny, &p.sum, r1)?;
    let i = k / p.ny;
    let j = k - i * p.ny;

    let delta = (r1 - p.sum[k]) / (p.sum[k + 1] - p.sum[k]);
    let x = p.xrange[i] + delta * (p.xrange[i + 1] - p.xrange[i]);
    let y = p.yrange[j] + r2 * (p.yrange[j + 1] - p.yrange[j]);

    Ok((x, y))
}

pub fn histogram2d_pdf_alloc(nx: SizeT, ny: SizeT) -> Result<Histogram2DPdf, GslError> {
    let n = nx * ny;
    if n == 0 {
        return Err(GslError::Domain);
    }

    Ok(Histogram2DPdf {
        nx,
        ny,
        xrange: vec![0.0; nx + 1],
        yrange: vec![0.0; ny + 1],
        sum: vec![0.0; n + 1],
    })
}

pub fn histogram2d_pdf_init(
    p: &mut Histogram2DPdf,
    h: &Histogram2D,
) -> Result<(), GslError> {
    if p.nx != h.nx || p.ny != h.ny {
        return Err(GslError::Domain);
    }

    for &bin in &h.bin {
        if bin < 0.0 {
            return Err(GslError::Domain);
        }
    }

    p.xrange.copy_from_slice(&h.xrange);
    p.yrange.copy_from_slice(&h.yrange);

    let mut mean = 0.0;
    let mut sum = 0.0;

    for (i, &bin) in h.bin.iter().enumerate() {
        mean += (bin - mean) / (i + 1) as f64;
    }

    p.sum[0] = 0.0;
    for (i, &bin) in h.bin.iter().enumerate() {
        sum += bin / mean / (h.bin.len() as f64);
        p.sum[i + 1] = sum;
    }

    Ok(())
}