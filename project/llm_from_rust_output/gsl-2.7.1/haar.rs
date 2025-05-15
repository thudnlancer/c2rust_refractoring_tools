use std::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDom = 1,
    ERange = 2,
    EFault = 3,
    EInval = 4,
    EFailed = 5,
    EFactor = 6,
    ESanity = 7,
    ENomem = 8,
    EBadfunc = 9,
    ERunaway = 10,
    EMaxiter = 11,
    EZerodiv = 12,
    EBadTol = 13,
    ETol = 14,
    EUndrflw = 15,
    EOvrflw = 16,
    ELoss = 17,
    ERound = 18,
    EBadLen = 19,
    ENotSqr = 20,
    ESing = 21,
    EDiverge = 22,
    EUnsup = 23,
    EUnimpl = 24,
    ECache = 25,
    ETable = 26,
    ENoProg = 27,
    ENoProgJ = 28,
    ETolF = 29,
    ETolX = 30,
    ETolG = 31,
    Eof = 32,
}

#[derive(Clone, Copy)]
pub struct GslWaveletType {
    pub name: &'static CStr,
    pub init: fn(member: usize) -> Result<(WaveletCoeffs, usize), GslError>,
}

#[derive(Debug, Clone, Copy)]
pub struct WaveletCoeffs {
    pub h1: &'static [f64],
    pub g1: &'static [f64],
    pub h2: &'static [f64],
    pub g2: &'static [f64],
    pub nc: usize,
}

const CH_2: [f64; 2] = [
    0.70710678118654752440,
    0.70710678118654752440,
];
const CG_2: [f64; 2] = [
    0.70710678118654752440,
    -0.70710678118654752440,
];

fn haar_init(member: usize) -> Result<(WaveletCoeffs, usize), GslError> {
    if member != 2 {
        return Err(GslError::Failure);
    }
    let coeffs = WaveletCoeffs {
        h1: &CH_2,
        g1: &CG_2,
        h2: &CH_2,
        g2: &CG_2,
        nc: 2,
    };
    Ok((coeffs, 0))
}

fn haar_centered_init(member: usize) -> Result<(WaveletCoeffs, usize), GslError> {
    if member != 2 {
        return Err(GslError::Failure);
    }
    let coeffs = WaveletCoeffs {
        h1: &CH_2,
        g1: &CG_2,
        h2: &CH_2,
        g2: &CG_2,
        nc: 2,
    };
    Ok((coeffs, 1))
}

pub static GSL_WAVELET_HAAR: GslWaveletType = GslWaveletType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"haar\0") },
    init: haar_init,
};

pub static GSL_WAVELET_HAAR_CENTERED: GslWaveletType = GslWaveletType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"haar-centered\0") },
    init: haar_centered_init,
};