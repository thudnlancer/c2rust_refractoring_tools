use std::f64;

#[derive(Debug, Clone)]
pub struct GslSumLevinUWorkspace {
    size: usize,
    i: usize,
    terms_used: usize,
    sum_plain: f64,
    q_num: Vec<f64>,
    q_den: Vec<f64>,
    dq_num: Vec<f64>,
    dq_den: Vec<f64>,
    dsum: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
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

const SMALL: f64 = 0.01;
const EPSILON: f64 = 2.2204460492503131e-16;

impl GslSumLevinUWorkspace {
    pub fn new(size: usize) -> Self {
        let nmax = size;
        let matrix_size = nmax + 1;
        
        GslSumLevinUWorkspace {
            size,
            i: 0,
            terms_used: 0,
            sum_plain: 0.0,
            q_num: vec![0.0; size],
            q_den: vec![0.0; size],
            dq_num: vec![0.0; matrix_size * matrix_size],
            dq_den: vec![0.0; matrix_size * matrix_size],
            dsum: vec![0.0; matrix_size],
        }
    }

    pub fn gsl_sum_levin_u_accel(&mut self, array: &[f64]) -> Result<(f64, f64), GslError> {
        self.gsl_sum_levin_u_minmax(array, 0, array.len() - 1)
    }

    pub fn gsl_sum_levin_u_minmax(
        &mut self,
        array: &[f64],
        min_terms: usize,
        max_terms: usize,
    ) -> Result<(f64, f64), GslError> {
        let size = array.iter().rev().take_while(|&&x| x == 0.0).count();
        let size = array.len() - size;

        if size == 0 {
            self.sum_plain = 0.0;
            self.terms_used = 0;
            return Ok((0.0, 0.0));
        } else if size == 1 {
            self.sum_plain = array[0];
            self.terms_used = 1;
            return Ok((array[0], 0.0));
        }

        let nmax = std::cmp::min(max_terms, array.len()) - 1;
        let mut noise_n = 0.0;
        let mut noise_nm1 = 0.0;
        let mut trunc_n = 0.0;
        let mut trunc_nm1 = 0.0;
        let mut actual_trunc_n = 0.0;
        let mut actual_trunc_nm1 = 0.0;
        let mut result_n = 0.0;
        let mut result_nm1 = 0.0;
        let mut variance = 0.0;
        let mut least_trunc = f64::MAX;
        let mut least_trunc_noise = f64::MAX;
        let mut least_trunc_result = 0.0;
        let mut better = false;
        let mut before = false;
        let mut converging = false;

        let mut n = 0;
        while n < min_terms {
            let t = array[n];
            result_nm1 = result_n;
            self.gsl_sum_levin_u_step(t, n, nmax, &mut result_n)?;
            n += 1;
        }

        least_trunc_result = result_n;
        variance = 0.0;
        for i in 0..n {
            let dn = self.dsum[i] * EPSILON * array[i];
            variance += dn * dn;
        }
        noise_n = variance.sqrt();

        while n <= nmax {
            let t = array[n];
            result_nm1 = result_n;
            self.gsl_sum_levin_u_step(t, n, nmax, &mut result_n)?;

            actual_trunc_nm1 = actual_trunc_n;
            actual_trunc_n = (result_n - result_nm1).abs();
            trunc_nm1 = trunc_n;
            trunc_n = 0.5 * (actual_trunc_n + actual_trunc_nm1);

            noise_nm1 = noise_n;
            variance = 0.0;
            for i in 0..=n {
                let dn = self.dsum[i] * EPSILON * array[i];
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

                if trunc_n < 10.0 * EPSILON * result_n.abs() {
                    break;
                }
            }

            n += 1;
        }

        self.terms_used = n;

        if converging {
            Ok((least_trunc_result, f64::max(least_trunc, least_trunc_noise)))
        } else {
            Ok((result_n, f64::max(trunc_n, noise_n)))
        }
    }

    fn gsl_sum_levin_u_step(
        &mut self,
        term: f64,
        n: usize,
        nmax: usize,
        sum_accel: &mut f64,
    ) -> Result<(), GslError> {
        if n == 0 {
            *sum_accel = term;
            self.sum_plain = term;
            self.q_den[0] = 1.0 / term;
            self.q_num[0] = 1.0;
            self.dq_den[0 * (nmax + 1) + 0] = -1.0 / (term * term);
            self.dq_num[0 * (nmax + 1) + 0] = 0.0;
            self.dsum[0] = 1.0;
            return Ok(());
        }

        let mut result = 0.0;
        let mut factor = 1.0;
        let ratio = n as f64 / (n as f64 + 1.0);

        self.sum_plain += term;
        self.q_den[n] = 1.0 / (term * (n as f64 + 1.0).powi(2));
        self.q_num[n] = self.sum_plain * self.q_den[n];

        for i in 0..n {
            self.dq_den[i * (nmax + 1) + n] = 0.0;
            self.dq_num[i * (nmax + 1) + n] = self.q_den[n];
        }

        self.dq_den[n * (nmax + 1) + n] = -self.q_den[n] / term;
        self.dq_num[n * (nmax + 1) + n] = self.q_den[n] + self.sum_plain * self.dq_den[n * (nmax + 1) + n];

        for j in (0..n).rev() {
            let c = factor * (j + 1) as f64 / (n + 1) as f64;
            factor *= ratio;

            self.q_den[j] = self.q_den[j + 1] - c * self.q_den[j];
            self.q_num[j] = self.q_num[j + 1] - c * self.q_num[j];

            for i in 0..=n {
                self.dq_den[i * (nmax + 1) + j] = self.dq_den[i * (nmax + 1) + j + 1] - c * self.dq_den[i * (nmax + 1) + j];
                self.dq_num[i * (nmax + 1) + j] = self.dq_num[i * (nmax + 1) + j + 1] - c * self.dq_num[i * (nmax + 1) + j];
            }
        }

        result = self.q_num[0] / self.q_den[0];
        *sum_accel = result;

        for i in 0..=n {
            self.dsum[i] = (self.dq_num[i * (nmax + 1) + 0] - result * self.dq_den[i * (nmax + 1) + 0]) / self.q_den[0];
        }

        Ok(())
    }
}