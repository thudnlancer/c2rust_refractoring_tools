use std::f64;

#[derive(Debug, Clone)]
pub struct GslSumLevinUtruncWorkspace {
    size: usize,
    i: usize,
    terms_used: usize,
    sum_plain: f64,
    q_num: Vec<f64>,
    q_den: Vec<f64>,
    dsum: Vec<f64>,
}

impl GslSumLevinUtruncWorkspace {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            i: 0,
            terms_used: 0,
            sum_plain: 0.0,
            q_num: vec![0.0; size],
            q_den: vec![0.0; size],
            dsum: vec![0.0; size],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GslError {
    Success,
    Failure,
    Continue,
    Domain,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProg,
    NoProgJ,
    TolF,
    TolX,
    TolG,
    Eof,
}

pub fn gsl_sum_levin_utrunc_accel(
    array: &[f64],
    w: &mut GslSumLevinUtruncWorkspace,
) -> Result<(f64, f64), GslError> {
    gsl_sum_levin_utrunc_minmax(array, 0, array.len() - 1, w)
}

pub fn gsl_sum_levin_utrunc_minmax(
    array: &[f64],
    min_terms: usize,
    max_terms: usize,
    w: &mut GslSumLevinUtruncWorkspace,
) -> Result<(f64, f64), GslError> {
    if array.is_empty() {
        w.sum_plain = 0.0;
        w.terms_used = 0;
        return Ok((0.0, 0.0));
    } else if array.len() == 1 {
        w.sum_plain = array[0];
        w.terms_used = 1;
        return Ok((array[0], f64::INFINITY));
    }

    const SMALL: f64 = 0.01;
    let nmax = (max_terms.min(array.len()) - 1;
    let mut trunc_n = 0.0;
    let mut trunc_nm1 = 0.0;
    let mut actual_trunc_n = 0.0;
    let mut actual_trunc_nm1 = 0.0;
    let mut result_n = 0.0;
    let mut result_nm1 = 0.0;
    let mut n = 0;
    let mut better = false;
    let mut before = false;
    let mut converging = false;
    let mut least_trunc = f64::MAX;
    let mut result_least_trunc = 0.0;

    while n < min_terms {
        let t = array[n];
        result_nm1 = result_n;
        gsl_sum_levin_utrunc_step(t, n, w, &mut result_n)?;
        n += 1;
    }

    result_least_trunc = result_n;

    while n <= nmax {
        let t = array[n];
        result_nm1 = result_n;
        gsl_sum_levin_utrunc_step(t, n, w, &mut result_n)?;

        actual_trunc_nm1 = actual_trunc_n;
        actual_trunc_n = (result_n - result_nm1).abs();
        trunc_nm1 = trunc_n;
        trunc_n = 0.5 * (actual_trunc_n + actual_trunc_nm1);

        better = trunc_n < trunc_nm1 || trunc_n < SMALL * result_n.abs();
        converging = converging || (better && before);
        before = better;

        if converging {
            if trunc_n < least_trunc {
                least_trunc = trunc_n;
                result_least_trunc = result_n;
            }
            if (trunc_n / result_n).abs() < 10.0 * f64::EPSILON {
                break;
            }
        }
        n += 1;
    }

    if converging {
        w.terms_used = n;
        Ok((result_least_trunc, least_trunc))
    } else {
        w.terms_used = n;
        Ok((result_n, trunc_n))
    }
}

fn gsl_sum_levin_utrunc_step(
    term: f64,
    n: usize,
    w: &mut GslSumLevinUtruncWorkspace,
    sum_accel: &mut f64,
) -> Result<(), GslError> {
    if term == 0.0 {
        Err(GslError::ZeroDiv)
    } else if n == 0 {
        *sum_accel = term;
        w.sum_plain = term;
        w.q_den[0] = 1.0 / term;
        w.q_num[0] = 1.0;
        Ok(())
    } else {
        let mut factor = 1.0;
        let ratio = n as f64 / (n as f64 + 1.0);
        w.sum_plain += term;
        w.q_den[n] = 1.0 / (term * (n as f64 + 1.0).powi(2));
        w.q_num[n] = w.sum_plain * w.q_den[n];

        for j in (0..n).rev() {
            let c = factor * (j + 1) as f64 / (n + 1) as f64;
            factor *= ratio;
            w.q_den[j] = w.q_den[j + 1] - c * w.q_den[j];
            w.q_num[j] = w.q_num[j + 1] - c * w.q_num[j];
        }

        *sum_accel = w.q_num[0] / w.q_den[0];
        Ok(())
    }
}