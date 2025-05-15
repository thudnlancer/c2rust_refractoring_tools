use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Edom = 1,
    Erange = 2,
    Edefault = 3,
    Einval = 4,
    Efailed = 5,
    Efactor = 6,
    Esanity = 7,
    Enomem = 8,
    Ebadfunc = 9,
    Erunaway = 10,
    Emaxiter = 11,
    Ezerodiv = 12,
    Ebadtol = 13,
    Etol = 14,
    Eundrflw = 15,
    Eovrflw = 16,
    Eloss = 17,
    Eround = 18,
    Ebadlen = 19,
    Enotsqr = 20,
    Esing = 21,
    Ediverge = 22,
    Eunsup = 23,
    Eunimpl = 24,
    Ecache = 25,
    Etable = 26,
    Enoprog = 27,
    Enoprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LegendreType {
    Schmidt = 0,
    SpHarm = 1,
    Full = 2,
    None = 3,
}

pub fn legendre_array_index(l: usize, m: usize) -> usize {
    (l * (l + 1)) / 2 + m
}

pub fn legendre_nlm(lmax: usize) -> usize {
    (lmax + 1) * (lmax + 2) / 2
}

pub fn legendre_array_size(lmax: usize) -> usize {
    let nlm = legendre_nlm(lmax);
    let nsqrt = 2 * lmax + 2;
    nlm + nsqrt
}

fn legendre_sqrts(lmax: usize) -> Vec<f64> {
    (0..=2 * lmax + 1).map(|l| (l as f64).sqrt()).collect()
}

pub fn legendre_array(
    norm: LegendreType,
    lmax: usize,
    x: f64,
    csphase: f64,
    result: &mut [f64],
) -> Result<(), GslError> {
    if x < -1.0 || x > 1.0 {
        return Err(GslError::Edom);
    }
    if csphase != 1.0 && csphase != -1.0 {
        return Err(GslError::Edom);
    }

    let nlm = legendre_nlm(lmax);
    let mut sqrts = legendre_sqrts(lmax);
    
    match norm {
        LegendreType::None => legendre_array_none(lmax, x, csphase, result, &mut sqrts),
        _ => legendre_array_schmidt(lmax, x, csphase, result, &mut sqrts),
    }
}

fn legendre_array_none(
    lmax: usize,
    x: f64,
    csphase: f64,
    result: &mut [f64],
    sqrts: &mut [f64],
) -> Result<(), GslError> {
    let u = (1.0 - x * x).sqrt();
    
    result[0] = 1.0;
    if lmax == 0 {
        return Ok(());
    }
    
    result[1] = x;
    
    let mut k = 1;
    for l in 2..=lmax {
        k += l;
        let plm = ((2 * l - 1) as f64 * x * result[k - l] - (l - 1) as f64 * result[k - 2 * l + 1]) / l as f64;
        result[k] = plm;
    }
    
    let mut pmm = 1.0;
    let mut twomm1 = -1.0;
    let mut idxmm = 0;
    
    for m in 1..lmax {
        idxmm += m + 1;
        twomm1 += 2.0;
        pmm *= csphase * u * twomm1;
        result[idxmm] = pmm;
        
        let mut k = idxmm + m + 1;
        let mut pm1 = x * pmm * (2 * m + 1) as f64;
        result[k] = pm1;
        
        for l in m + 2..=lmax {
            k += l;
            let plm = ((2 * l - 1) as f64 * x * pm1 - (l + m - 1) as f64 * pmm) / (l - m) as f64;
            result[k] = plm;
            pmm = pm1;
            pm1 = plm;
        }
    }
    
    if lmax > 0 {
        idxmm += lmax + 1;
        twomm1 += 2.0;
        pmm *= csphase * u * twomm1;
        result[idxmm] = pmm;
    }
    
    Ok(())
}

fn legendre_array_schmidt(
    lmax: usize,
    x: f64,
    csphase: f64,
    result: &mut [f64],
    sqrts: &mut [f64],
) -> Result<(), GslError> {
    let eps = 1.0e-280;
    let u = (1.0 - x * x).sqrt();
    
    result[0] = 1.0;
    if lmax == 0 {
        return Ok(());
    }
    
    result[1] = x;
    
    let mut k = 1;
    for l in 2..=lmax {
        k += l;
        let linv = 1.0 / l as f64;
        let plm = (2.0 - linv) * x * result[k - l] - (1.0 - linv) * result[k - 2 * l + 1];
        result[k] = plm;
    }
    
    let mut pmm = 2.0f64.sqrt() * eps;
    let mut rescalem = 1.0 / eps;
    let mut idxmm = 0;
    
    for m in 1..lmax {
        rescalem *= u;
        idxmm += m + 1;
        pmm *= csphase * sqrts[2 * m - 1] / sqrts[2 * m];
        result[idxmm] = pmm * rescalem;
        
        let mut k = idxmm + m + 1;
        let mut pm1 = x * sqrts[2 * m + 1] * pmm;
        result[k] = pm1 * rescalem;
        
        for l in m + 2..=lmax {
            k += l;
            let plm = (2 * l - 1) as f64 / sqrts[l + m] / sqrts[l - m] * x * pm1
                - sqrts[l - m - 1] * sqrts[l + m - 1] / sqrts[l + m] / sqrts[l - m] * pmm;
            result[k] = plm * rescalem;
            pmm = pm1;
            pm1 = plm;
        }
    }
    
    if lmax > 0 {
        rescalem *= u;
        idxmm += lmax + 1;
        pmm *= csphase * sqrts[2 * lmax - 1] / sqrts[2 * lmax];
        result[idxmm] = pmm * rescalem;
    }
    
    Ok(())
}

// Similar implementations for deriv, deriv_alt, deriv2, deriv2_alt functions
// would follow the same pattern, converting the C code to safe Rust
// while maintaining the same mathematical operations and error handling