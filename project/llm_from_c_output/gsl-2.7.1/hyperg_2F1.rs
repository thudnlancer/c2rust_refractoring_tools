use std::f64::consts::{EULER, PI};
use std::f64::{EPSILON, MIN_POSITIVE};

const LOC_EPS: f64 = 1000.0 * EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfError {
    Domain,
    Overflow,
    Underflow,
    MaxIter,
    Unimplemented,
    Success,
}

fn hyperg_2f1_series(
    a: f64,
    b: f64,
    c: f64,
    x: f64,
) -> Result<SfResult, SfError> {
    if c.abs() < EPSILON {
        return Err(SfError::Domain);
    }

    let mut sum_pos = 1.0;
    let mut sum_neg = 0.0;
    let mut del_pos = 1.0;
    let mut del_neg = 0.0;
    let mut del = 1.0;
    let mut k = 0.0;
    let mut i = 0;

    loop {
        i += 1;
        if i > 30000 {
            let val = sum_pos - sum_neg;
            let mut err = del_pos + del_neg;
            err += 2.0 * EPSILON * (sum_pos + sum_neg);
            err += 2.0 * EPSILON * (2.0 * k.sqrt() + 1.0) * val.abs();
            return Err(SfError::MaxIter);
        }

        let del_prev = del;
        del *= (a + k) * (b + k) * x / ((c + k) * (k + 1.0));

        if del > 0.0 {
            del_pos = del;
            sum_pos += del;
        } else if del == 0.0 {
            del_pos = 0.0;
            del_neg = 0.0;
            break;
        } else {
            del_neg = -del;
            sum_neg -= del;
        }

        if (del_prev / (sum_pos - sum_neg)).abs() < EPSILON
            && (del / (sum_pos - sum_neg)).abs() < EPSILON
        {
            break;
        }

        k += 1.0;
        if ((del_pos + del_neg) / (sum_pos - sum_neg)).abs() <= EPSILON {
            break;
        }
    }

    let val = sum_pos - sum_neg;
    let mut err = del_pos + del_neg;
    err += 2.0 * EPSILON * (sum_pos + sum_neg);
    err += 2.0 * EPSILON * (2.0 * k.sqrt() + 1.0) * val.abs();

    Ok(SfResult::new(val, err))
}

fn hyperg_2f1_conj_series(
    ar: f64,
    ai: f64,
    c: f64,
    x: f64,
) -> Result<SfResult, SfError> {
    if c == 0.0 {
        return Err(SfError::Domain);
    }

    let mut sum_pos = 1.0;
    let mut sum_neg = 0.0;
    let mut del_pos = 1.0;
    let mut del_neg = 0.0;
    let mut del = 1.0;
    let mut k = 0.0;

    loop {
        del *= ((ar + k).powi(2) + ai.powi(2)) / ((k + 1.0) * (c + k)) * x;

        if del >= 0.0 {
            del_pos = del;
            sum_pos += del;
        } else {
            del_neg = -del;
            sum_neg -= del;
        }

        if k > 30000.0 {
            let val = sum_pos - sum_neg;
            let mut err = del_pos + del_neg;
            err += 2.0 * EPSILON * (sum_pos + sum_neg);
            err += 2.0 * EPSILON * (2.0 * k.sqrt() + 1.0) * val.abs();
            return Err(SfError::MaxIter);
        }

        k += 1.0;
        if ((del_pos + del_neg) / (sum_pos - sum_neg)).abs() <= EPSILON {
            break;
        }
    }

    let val = sum_pos - sum_neg;
    let mut err = del_pos + del_neg;
    err += 2.0 * EPSILON * (sum_pos + sum_neg);
    err += 2.0 * EPSILON * (2.0 * k.sqrt() + 1.0) * val.abs();

    Ok(SfResult::new(val, err))
}

fn hyperg_2f1_luke(
    a: f64,
    b: f64,
    c: f64,
    xin: f64,
) -> Result<SfResult, SfError> {
    const RECUR_BIG: f64 = 1.0e50;
    const NMAX: i32 = 20000;

    let x = -xin;
    let x3 = x.powi(3);
    let t0 = a * b / c;
    let t1 = (a + 1.0) * (b + 1.0) / (2.0 * c);
    let t2 = (a + 2.0) * (b + 2.0) / (2.0 * (c + 1.0));

    let mut n = 3;
    let mut f = 1.0;

    let mut bnm3 = 1.0;
    let mut bnm2 = 1.0 + t1 * x;
    let mut bnm1 = 1.0 + t2 * x * (1.0 + t1 / 3.0 * x);

    let mut anm3 = 1.0;
    let mut anm2 = bnm2 - t0 * x;
    let mut anm1 = bnm1 - t0 * (1.0 + t2 * x) * x + t0 * t1 * (c / (c + 1.0)) * x.powi(2);

    loop {
        let npam1 = (n + a - 1.0) as f64;
        let npbm1 = (n + b - 1.0) as f64;
        let npcm1 = (n + c - 1.0) as f64;
        let npam2 = (n + a - 2.0) as f64;
        let npbm2 = (n + b - 2.0) as f64;
        let npcm2 = (n + c - 2.0) as f64;
        let tnm1 = (2 * n - 1) as f64;
        let tnm3 = (2 * n - 3) as f64;
        let tnm5 = (2 * n - 5) as f64;
        let n2 = (n * n) as f64;

        let f1 = (3.0 * n2 + (a + b - 6.0) * n as f64 + 2.0 - a * b - 2.0 * (a + b)) / (2.0 * tnm3 * npcm1);
        let f2 = -(3.0 * n2 - (a + b + 6.0) * n as f64 + 2.0 - a * b) * npam1 * npbm1 / (4.0 * tnm1 * tnm3 * npcm2 * npcm1);
        let f3 = npam2 * npam1 * npbm2 * npbm1 * (n as f64 - a - 2.0) * (n as f64 - b - 2.0) / (8.0 * tnm3.powi(2) * tnm5 * (n as f64 + c - 3.0) * npcm2 * npcm1);
        let e = -npam1 * npbm1 * (n as f64 - c - 1.0) / (2.0 * tnm3 * npcm2 * npcm1);

        let an = (1.0 + f1 * x) * anm1 + (e + f2 * x) * x * anm2 + f3 * x3 * anm3;
        let bn = (1.0 + f1 * x) * bnm1 + (e + f2 * x) * x * bnm2 + f3 * x3 * bnm3;
        let r = an / bn;

        let prec = ((f - r) / f).abs();
        f = r;

        if prec < EPSILON || n > NMAX {
            break;
        }

        if an.abs() > RECUR_BIG || bn.abs() > RECUR_BIG {
            an /= RECUR_BIG;
            bn /= RECUR_BIG;
            anm1 /= RECUR_BIG;
            bnm1 /= RECUR_BIG;
            anm2 /= RECUR_BIG;
            bnm2 /= RECUR_BIG;
            anm3 /= RECUR_BIG;
            bnm3 /= RECUR_BIG;
        } else if an.abs() < 1.0 / RECUR_BIG || bn.abs() < 1.0 / RECUR_BIG {
            an *= RECUR_BIG;
            bn *= RECUR_BIG;
            anm1 *= RECUR_BIG;
            bnm1 *= RECUR_BIG;
            anm2 *= RECUR_BIG;
            bnm2 *= RECUR_BIG;
            anm3 *= RECUR_BIG;
            bnm3 *= RECUR_BIG;
        }

        n += 1;
        bnm3 = bnm2;
        bnm2 = bnm1;
        bnm1 = bn;
        anm3 = anm2;
        anm2 = anm1;
        anm1 = an;
    }

    let mut err = 2.0 * (prec * f).abs();
    err += 2.0 * EPSILON * (n as f64 + 1.0) * f.abs();
    err *= 8.0 * (a.abs() + b.abs() + 1.0);

    let stat = if n >= NMAX { SfError::MaxIter } else { SfError::Success };

    Ok(SfResult::new(f, err))
}

fn hyperg_2f1_conj_luke(
    ar: f64,
    ai: f64,
    c: f64,
    xin: f64,
) -> Result<SfResult, SfError> {
    const RECUR_BIG: f64 = 1.0e50;
    const NMAX: i32 = 10000;

    let x = -xin;
    let x3 = x.powi(3);
    let atimesb = ar.powi(2) + ai.powi(2);
    let apb = 2.0 * ar;
    let t0 = atimesb / c;
    let t1 = (atimesb + apb + 1.0) / (2.0 * c);
    let t2 = (atimesb + 2.0 * apb + 4.0) / (2.0 * (c + 1.0));

    let mut n = 3;
    let mut f = 1.0;

    let mut bnm3 = 1.0;
    let mut bnm2 = 1.0 + t1 * x;
    let mut bnm1 = 1.0 + t2 * x * (1.0 + t1 / 3.0 * x);

    let mut anm3 = 1.0;
    let mut anm2 = bnm2 - t0 * x;
    let mut anm1 = bnm1 - t0 * (1.0 + t2 * x) * x + t0 * t1 * (c / (c + 1.0)) * x.powi(2);

    loop {
        let nm1 = (n - 1) as f64;
        let nm2 = (n - 2) as f64;
        let npam1_npbm1 = atimesb + nm1 * apb + nm1.powi(2);
        let npam2_npbm2 = atimesb + nm2 * apb + nm2.powi(2);
        let npcm1 = nm1 + c;
        let npcm2 = nm2 + c;
        let tnm1 = (2 * n - 1) as f64;
        let tnm3 = (2 * n - 3) as f64;
        let tnm5 = (2 * n - 5) as f64;
        let n2 = (n * n) as f64;

        let f1 = (3.0 * n2 + (apb - 6.0) * n as f64 + 2.0 - atimesb - 2.0 * apb) / (2.0 * tnm3 * npcm1);
        let f2 = -(3.0 * n2 - (apb + 6.0) * n as f64 + 2.0 - atimesb) * npam1_npbm1 / (4.0 * tnm1 * tnm3 * npcm2 * npcm1);
        let f3 = npam2_npbm2 * npam1_npbm1 * (nm2.powi(2) - nm2 * apb + atimesb) / (8.0 * tnm3.powi(2) * tnm5 * (n as f64 + c - 3.0) * npcm2 * npcm1);
        let e = -npam1_npbm1 * (n as f64 - c - 1.0) / (2.0 * tnm3 * npcm2 * npcm1);

        let an = (1.0 + f1 * x) * anm1 + (e + f2 * x) * x * anm2 + f3 * x3 * anm3;
        let bn = (1.0 + f1 * x) * bnm1 + (e + f2 * x) * x * bnm2 + f3 * x3 * bnm3;
        let r = an / bn;

        let prec = ((f - r) / f).abs();
        f = r;

        if prec < EPSILON || n > NMAX {
            break;
        }

        if an.abs() > RECUR_BIG || bn.abs() > RECUR_BIG {
            an /= RECUR_BIG;
            bn /= RECUR_BIG;
            anm1 /= RECUR_BIG;
            bnm1 /= RECUR_BIG;
            anm2 /= RECUR_BIG;
            bnm2 /= RECUR_BIG;
            anm3 /= RECUR_BIG;
            bnm3 /= RECUR_BIG;
        } else if an.abs() < 1.0 / RECUR_BIG || bn.abs() < 1.0 / RECUR_BIG {
            an *= RECUR_BIG;
            bn *= RECUR_BIG;
            anm1 *= RECUR_BIG;
            bnm1 *= RECUR_BIG;
            anm2 *= RECUR_BIG;
            bnm2 *= RECUR_BIG;
            anm3 *= RECUR_BIG;
            bnm3 *= RECUR_BIG;
        }

        n += 1;
        bnm3 = bnm2;
        bnm2 = bnm1;
        bnm1 = bn;
        anm3 = anm2;
        anm2 = anm1;
        anm1 = an;
    }

    let mut err = 2.0 * (prec * f).abs();
    err += 2.0 * EPSILON * (n as f64 + 1.0) * f.abs();
    err *= 8.0 * (ar.abs() + ai.abs() + 1.0);

    let stat = if n >= NMAX { SfError::MaxIter } else { SfError::Success };

    Ok(SfResult::new(f, err))
}

fn pow_omx(x: f64, p: f64) -> Result<SfResult, SfError> {
    let ln_omx = if x.abs() < EPSILON.powf(0.2) {
        -x * (1.0 + x * (0.5 + x * (1.0 / 3.0 + x * (0.25 + x * x / 5.0))))
    } else {
        (1.0 - x).ln()
    };

    let ln_result = p * ln_omx;
    let err = EPSILON * ln_result.abs();
    Ok(SfResult::new(ln_result.exp(), err))
}

pub fn gsl_sf_hyperg_2f1_e(
    a: f64,
    b: f64,
    c: f64,
    x: f64,
) -> Result<SfResult, SfError> {
    let d = c - a - b;
    let rinta = (a + 0.5).floor();
    let rintb = (b + 0.5).floor();
    let rintc = (c + 0.5).floor();
    let a_neg_integer = a < 0.0 && (a - rinta).abs() < LOC_EPS;
    let b_neg_integer = b < 0.0 && (b - rintb).abs() < LOC_EPS;
    let c_neg_integer = c < 0.0 && (c - rintc).abs() < LOC_EPS;

    if x < -1.0 || x >= 1.0 {
        return Err(SfError::Domain);
    }

    if c_neg_integer {
        if !(a_neg_integer && a > c + 0.1)