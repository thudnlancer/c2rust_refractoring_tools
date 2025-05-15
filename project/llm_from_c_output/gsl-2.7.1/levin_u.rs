use std::f64;
use std::cmp;

pub struct LevinUWorkspace {
    sum_plain: f64,
    terms_used: usize,
    q_den: Vec<f64>,
    q_num: Vec<f64>,
    dq_den: Vec<f64>,
    dq_num: Vec<f64>,
    dsum: Vec<f64>,
}

impl LevinUWorkspace {
    pub fn new(nmax: usize) -> Self {
        let size = nmax + 1;
        LevinUWorkspace {
            sum_plain: 0.0,
            terms_used: 0,
            q_den: vec![0.0; size],
            q_num: vec![0.0; size],
            dq_den: vec![0.0; size * size],
            dq_num: vec![0.0; size * size],
            dsum: vec![0.0; size],
        }
    }

    fn index(&self, i: usize, j: usize, nmax: usize) -> usize {
        i * (nmax + 1) + j
    }
}

pub enum GslError {
    Success,
}

pub fn gsl_sum_levin_u_accel(
    array: &[f64],
    w: &mut LevinUWorkspace,
    sum_accel: &mut f64,
    abserr: &mut f64,
) -> GslError {
    gsl_sum_levin_u_minmax(array, 0, array.len() - 1, w, sum_accel, abserr)
}

pub fn gsl_sum_levin_u_minmax(
    array: &[f64],
    min_terms: usize,
    max_terms: usize,
    w: &mut LevinUWorkspace,
    sum_accel: &mut f64,
    abserr: &mut f64,
) -> GslError {
    let mut size = array.len();

    while size > 0 && array[size - 1] == 0.0 {
        size -= 1;
    }

    if size == 0 {
        *sum_accel = 0.0;
        *abserr = 0.0;
        w.sum_plain = 0.0;
        w.terms_used = 0;
        GslError::Success
    } else if size == 1 {
        *sum_accel = array[0];
        *abserr = 0.0;
        w.sum_plain = array[0];
        w.terms_used = 1;
        GslError::Success
    } else {
        const SMALL: f64 = 0.01;
        let nmax = cmp::max(max_terms, array.len()) - 1;
        let mut noise_n = 0.0;
        let mut noise_nm1 = 0.0;
        let mut trunc_n = 0.0;
        let mut trunc_nm1 = 0.0;
        let mut actual_trunc_n = 0.0;
        let mut actual_trunc_nm1 = 0.0;
        let mut result_n = 0.0;
        let mut result_nm1 = 0.0;
        let mut variance = 0.0;
        let mut n;
        let mut better = false;
        let mut before = false;
        let mut converging = false;
        let mut least_trunc = f64::MAX;
        let mut least_trunc_noise = f64::MAX;
        let mut least_trunc_result = 0.0;

        for n_ in 0..min_terms {
            let t = array[n_];
            result_nm1 = result_n;
            gsl_sum_levin_u_step(t, n_, nmax, w, &mut result_n);
        }
        n = min_terms;
        least_trunc_result = result_n;

        variance = 0.0;
        for i in 0..n {
            let dn = w.dsum[i] * f64::EPSILON * array[i];
            variance += dn * dn;
        }
        noise_n = variance.sqrt();

        for n_ in n..=nmax {
            n = n_;
            let t = array[n];

            result_nm1 = result_n;
            gsl_sum_levin_u_step(t, n, nmax, w, &mut result_n);

            actual_trunc_nm1 = actual_trunc_n;
            actual_trunc_n = (result_n - result_nm1).abs();

            trunc_nm1 = trunc_n;
            trunc_n = 0.5 * (actual_trunc_n + actual_trunc_nm1);

            noise_nm1 = noise_n;
            variance = 0.0;

            for i in 0..=n {
                let dn = w.dsum[i] * f64::EPSILON * array[i];
                variance += dn * dn;
            }

            noise_n = variance.sqrt();

            better = trunc_n < trunc_nm1 || trunc_n < SMALL * result_n.abs();
            converging = converging || (better && before);
            before = better;

            if converging {
                if trunc_n < least_trunc {
                    least_trunc_result = result_n;
                    least_trunc = trunc_n;
                    least_trunc_noise = noise_n;
                }

                if noise_n > trunc_n / 3.0 {
                    break;
                }

                if trunc_n < 10.0 * f64::EPSILON * result_n.abs() {
                    break;
                }
            }
        }

        if converging {
            *sum_accel = least_trunc_result;
            *abserr = if least_trunc > least_trunc_noise {
                least_trunc
            } else {
                least_trunc_noise
            };
            w.terms_used = n;
            GslError::Success
        } else {
            *sum_accel = result_n;
            *abserr = if trunc_n > noise_n { trunc_n } else { noise_n };
            w.terms_used = n;
            GslError::Success
        }
    }
}

pub fn gsl_sum_levin_u_step(
    term: f64,
    n: usize,
    nmax: usize,
    w: &mut LevinUWorkspace,
    sum_accel: &mut f64,
) -> GslError {
    if n == 0 {
        *sum_accel = term;
        w.sum_plain = term;

        w.q_den[0] = 1.0 / term;
        w.q_num[0] = 1.0;

        w.dq_den[w.index(0, 0, nmax)] = -1.0 / (term * term);
        w.dq_num[w.index(0, 0, nmax)] = 0.0;

        w.dsum[0] = 1.0;

        GslError::Success
    } else {
        let mut factor = 1.0;
        let ratio = n as f64 / (n as f64 + 1.0);

        w.sum_plain += term;

        w.q_den[n] = 1.0 / (term * (n as f64 + 1.0).powi(2));
        w.q_num[n] = w.sum_plain * w.q_den[n];

        for i in 0..n {
            w.dq_den[w.index(i, n, nmax)] = 0.0;
            w.dq_num[w.index(i, n, nmax)] = w.q_den[n];
        }

        w.dq_den[w.index(n, n, nmax)] = -w.q_den[n] / term;
        w.dq_num[w.index(n, n, nmax)] =
            w.q_den[n] + w.sum_plain * w.dq_den[w.index(n, n, nmax)];

        for j in (0..n).rev() {
            let c = factor * (j as f64 + 1.0) / (n as f64 + 1.0);
            factor *= ratio;
            w.q_den[j] = w.q_den[j + 1] - c * w.q_den[j];
            w.q_num[j] = w.q_num[j + 1] - c * w.q_num[j];

            for i in 0..n {
                w.dq_den[w.index(i, j, nmax)] =
                    w.dq_den[w.index(i, j + 1, nmax)] - c * w.dq_den[w.index(i, j, nmax)];
                w.dq_num[w.index(i, j, nmax)] =
                    w.dq_num[w.index(i, j + 1, nmax)] - c * w.dq_num[w.index(i, j, nmax)];
            }

            w.dq_den[w.index(n, j, nmax)] = w.dq_den[w.index(n, j + 1, nmax)];
            w.dq_num[w.index(n, j, nmax)] = w.dq_num[w.index(n, j + 1, nmax)];
        }

        let result = w.q_num[0] / w.q_den[0];
        *sum_accel = result;

        for i in 0..=n {
            w.dsum[i] = (w.dq_num[w.index(i, 0, nmax)] - result * w.dq_den[w.index(i, 0, nmax)])
                / w.q_den[0];
        }

        GslError::Success
    }
}