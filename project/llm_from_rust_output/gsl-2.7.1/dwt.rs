use thiserror::Error;

#[derive(Debug, Error)]
pub enum GslError {
    #[error("not enough workspace provided")]
    NotEnoughWorkspace,
    #[error("n is not a power of 2")]
    NotPowerOfTwo,
    #[error("2d dwt works only with square matrix")]
    NonSquareMatrix,
}

#[derive(Copy, Clone, Debug)]
pub enum GslResult {
    Success,
    Continue,
    Failure,
    DomainError,
    RangeError,
    InvalidArgument,
    Failed,
    FactorizationError,
    SanityCheckFailed,
    OutOfMemory,
    BadFunction,
    Runaway,
    MaxIterations,
    ZeroDivision,
    BadTolerance,
    Tolerance,
    Underflow,
    Overflow,
    LossOfAccuracy,
    RoundingError,
    BadLength,
    MatrixNotSquare,
    Singularity,
    Divergence,
    Unsupported,
    Unimplemented,
    CacheLimit,
    TableLimit,
    NoProgress,
    NoProgressJacobian,
    ToleranceF,
    ToleranceX,
    ToleranceG,
    EndOfFile,
}

#[derive(Copy, Clone, Debug)]
pub enum WaveletDirection {
    Forward = 1,
    Backward = -1,
}

#[derive(Copy, Clone, Debug)]
pub struct WaveletType {
    pub name: &'static str,
    pub init: Option<fn(&mut [f64], &mut [f64], &mut [f64], &mut [f64], usize) -> Result<(), GslError>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Wavelet {
    pub wavelet_type: &'static WaveletType,
    pub h1: &'static [f64],
    pub g1: &'static [f64],
    pub h2: &'static [f64],
    pub g2: &'static [f64],
    pub nc: usize,
    pub offset: usize,
}

#[derive(Debug)]
pub struct WaveletWorkspace {
    scratch: Vec<f64>,
}

impl WaveletWorkspace {
    pub fn new(n: usize) -> Self {
        WaveletWorkspace {
            scratch: vec![0.0; n],
        }
    }
}

fn binary_logn(n: usize) -> Result<usize, GslError> {
    let mut logn = 0;
    let mut k = 1;
    while k < n {
        k *= 2;
        logn += 1;
    }
    let ntest = 1 << logn;
    if n != ntest {
        Err(GslError::NotPowerOfTwo)
    } else {
        Ok(logn)
    }
}

fn dwt_step(
    w: &Wavelet,
    data: &mut [f64],
    stride: usize,
    n: usize,
    dir: WaveletDirection,
    work: &mut WaveletWorkspace,
) -> Result<(), GslError> {
    if work.scratch.len() < n {
        return Err(GslError::NotEnoughWorkspace);
    }

    work.scratch.iter_mut().for_each(|x| *x = 0.0);

    let nmod = w.nc * n - w.offset;
    let n1 = n - 1;
    let nh = n >> 1;

    match dir {
        WaveletDirection::Forward => {
            let mut ii = 0;
            for i in (0..n).step_by(2) {
                let ni = i + nmod;
                let mut h = 0.0;
                let mut g = 0.0;
                for k in 0..w.nc {
                    let jf = n1 & (ni + k);
                    h += w.h1[k] * data[stride * jf];
                    g += w.g1[k] * data[stride * jf];
                }
                work.scratch[ii] += h;
                work.scratch[ii + nh] += g;
                ii += 1;
            }
        }
        WaveletDirection::Backward => {
            let mut ii = 0;
            for i in (0..n).step_by(2) {
                let ai = data[stride * ii];
                let ai1 = data[stride * (ii + nh)];
                let ni = i + nmod;
                for k in 0..w.nc {
                    let jf = n1 & (ni + k);
                    work.scratch[jf] += w.h2[k] * ai + w.g2[k] * ai1;
                }
                ii += 1;
            }
        }
    }

    for i in 0..n {
        data[stride * i] = work.scratch[i];
    }

    Ok(())
}

pub fn wavelet_transform(
    w: &Wavelet,
    data: &mut [f64],
    stride: usize,
    n: usize,
    dir: WaveletDirection,
    work: &mut WaveletWorkspace,
) -> Result<(), GslError> {
    if work.scratch.len() < n {
        return Err(GslError::NotEnoughWorkspace);
    }

    binary_logn(n)?;

    if n < 2 {
        return Ok(());
    }

    match dir {
        WaveletDirection::Forward => {
            let mut i = n;
            while i >= 2 {
                dwt_step(w, data, stride, i, dir, work)?;
                i >>= 1;
            }
        }
        WaveletDirection::Backward => {
            let mut i = 2;
            while i <= n {
                dwt_step(w, data, stride, i, dir, work)?;
                i <<= 1;
            }
        }
    }

    Ok(())
}

pub fn wavelet_transform_forward(
    w: &Wavelet,
    data: &mut [f64],
    stride: usize,
    n: usize,
    work: &mut WaveletWorkspace,
) -> Result<(), GslError> {
    wavelet_transform(w, data, stride, n, WaveletDirection::Forward, work)
}

pub fn wavelet_transform_inverse(
    w: &Wavelet,
    data: &mut [f64],
    stride: usize,
    n: usize,
    work: &mut WaveletWorkspace,
) -> Result<(), GslError> {
    wavelet_transform(w, data, stride, n, WaveletDirection::Backward, work)
}

pub fn wavelet2d_transform(
    w: &Wavelet,
    data: &mut [f64],
    tda: usize,
    size1: usize,
    size2: usize,
    dir: WaveletDirection,
    work: &mut WaveletWorkspace,
) -> Result<(), GslError> {
    if size1 != size2 {
        return Err(GslError::NonSquareMatrix);
    }

    if work.scratch.len() < size1 {
        return Err(GslError::NotEnoughWorkspace);
    }

    binary_logn(size1)?;

    if size1 < 2 {
        return Ok(());
    }

    match dir {
        WaveletDirection::Forward => {
            for i in 0..size1 {
                wavelet_transform(
                    w,
                    &mut data[i * tda..],
                    1,
                    size1,
                    dir,
                    work,
                )?;
            }
            for i in 0..size2 {
                wavelet_transform(
                    w,
                    &mut data[i..],
                    tda,
                    size2,
                    dir,
                    work,
                )?;
            }
        }
        WaveletDirection::Backward => {
            for i in 0..size2 {
                wavelet_transform(
                    w,
                    &mut data[i..],
                    tda,
                    size2,
                    dir,
                    work,
                )?;
            }
            for i in 0..size1 {
                wavelet_transform(
                    w,
                    &mut data[i * tda..],
                    1,
                    size1,
                    dir,
                    work,
                )?;
            }
        }
    }

    Ok(())
}

// Remaining functions follow similar patterns, converting to safe Rust with proper error handling