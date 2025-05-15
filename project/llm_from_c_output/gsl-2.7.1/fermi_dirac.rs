use std::f64::consts::PI;
use std::f64::{EPSILON, MIN_POSITIVE, MAX};

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

struct GslSfResult {
    val: f64,
    err: f64,
}

const LOC_EPS: f64 = 1000.0 * EPSILON;

// Chebyshev series definitions
static FD_1_A_CS: ChebSeries = ChebSeries {
    data: &[
        1.8949340668482264365,
        0.7237719066890052793,
        0.1250000000000000000,
        0.0101065196435973942,
        0.0,
        -0.0000600615242174119,
        0.0,
        6.816528764623e-7,
        0.0,
        -9.5895779195e-9,
        0.0,
        1.515104135e-10,
        0.0,
        -2.5785616e-12,
        0.0,
        4.62270e-14,
        0.0,
        -8.612e-16,
        0.0,
        1.65e-17,
        0.0,
        -3.e-19,
    ],
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

static FD_1_B_CS: ChebSeries = ChebSeries {
    data: &[
        10.409136795234611872,
        3.899445098225161947,
        0.513510935510521222,
        0.010618736770218426,
        -0.001584468020659694,
        0.000146139297161640,
        -1.408095734499e-6,
        -2.177993899484e-6,
        3.91423660640e-7,
        -2.3860262660e-8,
        -4.138309573e-9,
        1.283965236e-9,
        -1.39695990e-10,
        -4.907743e-12,
        4.399878e-12,
        -7.17291e-13,
        2.4320e-14,
        1.4230e-14,
        -3.446e-15,
        2.93e-16,
        3.7e-17,
        -1.6e-17,
    ],
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

// ... (other Chebyshev series definitions would go here)

fn cheb_eval(cs: &ChebSeries, x: f64) -> f64 {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    for &c in cs.data.iter().rev() {
        let temp = d;
        d = y2 * d - dd + c;
        dd = temp;
    }
    
    y * d - dd + 0.5 * cs.data[0]
}

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut GslSfResult) -> i32 {
    result.val = cheb_eval(cs, x);
    result.err = cs.data[cs.order] * EPSILON;
    0 // GSL_SUCCESS
}

fn fd_whiz(term: f64, iterm: i32, qnum: &mut [f64], qden: &mut [f64], result: &mut f64, s: &mut f64) -> i32 {
    if iterm == 0 {
        *s = 0.0;
    }

    *s += term;

    let iterm_f = iterm as f64;
    qden[iterm as usize] = 1.0 / (term * (iterm_f + 1.0).powi(2));
    qnum[iterm as usize] = *s * qden[iterm as usize];

    if iterm > 0 {
        let mut factor = 1.0;
        let ratio = iterm_f / (iterm_f + 1.0);
        
        for j in (0..iterm).rev() {
            let j_f = j as f64;
            let c = factor * (j_f + 1.0) / (iterm_f + 1.0);
            factor *= ratio;
            qden[j as usize] = qden[j as usize + 1] - c * qden[j as usize];
            qnum[j as usize] = qnum[j as usize + 1] - c * qnum[j as usize];
        }
    }

    *result = qnum[0] / qden[0];
    0 // GSL_SUCCESS
}

fn fd_nint(j: i32, x: f64, result: &mut GslSfResult) -> i32 {
    if j >= -1 {
        result.val = 0.0;
        result.err = 0.0;
        return 1; // GSL_ESANITY
    } else if j < -100 {
        result.val = 0.0;
        result.err = 0.0;
        return 2; // GSL_EUNIMPL
    } else {
        let n = -(j + 1);
        let mut qcoeff = vec![0.0; (n + 1) as usize];
        qcoeff[1] = 1.0;

        for k in 2..=n {
            qcoeff[k as usize] = -qcoeff[(k - 1) as usize];
            for i in (2..k).rev() {
                qcoeff[i as usize] = i as f64 * qcoeff[i as usize] - (k - (i - 1)) as f64 * qcoeff[(i - 1) as usize];
            }
        }

        let (a, f) = if x >= 0.0 {
            let a = (-x).exp();
            let mut f = qcoeff[1];
            for i in 2..=n {
                f = f * a + qcoeff[i as usize];
            }
            (a, f)
        } else {
            let a = x.exp();
            let mut f = qcoeff[n as usize];
            for i in (1..n).rev() {
                f = f * a + qcoeff[i as usize];
            }
            (a, f)
        };

        let p = (1.0 + a).powi(j);
        result.val = f * a * p;
        result.err = 3.0 * EPSILON * (f * a * p).abs();
        0 // GSL_SUCCESS
    }
}

fn fd_neg(j: f64, x: f64, result: &mut GslSfResult) -> i32 {
    const ITMAX: usize = 100;
    const QSIZE: usize = 101;

    if x < MIN_POSITIVE.ln() {
        result.val = 0.0;
        result.err = 0.0;
        return 0; // GSL_SUCCESS
    } else if x < -1.0 && x < -(j + 1.0).abs() {
        let ex = x.exp();
        let mut term = ex;
        let mut sum = term;
        
        for n in 2..100 {
            let rat = (n as f64 - 1.0) / n as f64;
            term *= -ex * rat.powi((j + 1.0) as i32);
            sum += term;
            if (term / sum).abs() < EPSILON {
                break;
            }
        }
        
        result.val = sum;
        result.err = 2.0 * EPSILON * sum.abs();
        0 // GSL_SUCCESS
    } else {
        let mut s = 0.0;
        let mut xn = x;
        let ex = -x.exp();
        let mut enx = -ex;
        let mut f = 0.0;
        let mut f_previous;
        let mut qnum = [0.0; QSIZE];
        let mut qden = [0.0; QSIZE];
        
        for jterm in 0..=ITMAX {
            let p = (jterm as f64 + 1.0).powf(j + 1.0);
            let term = enx / p;
            f_previous = f;
            fd_whiz(term, jterm as i32, &mut qnum, &mut qden, &mut f, &mut s);
            xn += x;
            if (f - f_previous).abs() < f.abs() * 2.0 * EPSILON || xn < MIN_POSITIVE.ln() {
                break;
            }
            enx *= ex;
        }
        
        result.val = f;
        result.err = (f - f_previous).abs();
        result.err += 2.0 * EPSILON * f.abs();
        
        if ITMAX == ITMAX {
            3 // GSL_EMAXITER
        } else {
            0 // GSL_SUCCESS
        }
    }
}

// ... (other function implementations would continue here)

pub fn gsl_sf_fermi_dirac_m1_e(x: f64, result: &mut GslSfResult) -> i32 {
    if x < MIN_POSITIVE.ln() {
        result.val = 0.0;
        result.err = 0.0;
        4 // GSL_EUNDRFLW
    } else if x < 0.0 {
        let ex = x.exp();
        result.val = ex / (1.0 + ex);
        result.err = 2.0 * (x.abs() + 1.0) * EPSILON * result.val.abs();
        0 // GSL_SUCCESS
    } else {
        let ex = (-x).exp();
        result.val = 1.0 / (1.0 + ex);
        result.err = 2.0 * EPSILON * (x + 1.0) * ex;
        0 // GSL_SUCCESS
    }
}

// ... (other public function implementations would continue here)

// Evaluation macros
macro_rules! EVAL_RESULT {
    ($expr:expr) => {
        {
            let mut result = GslSfResult { val: 0.0, err: 0.0 };
            let status = $expr;
            if status != 0 {
                std::f64::NAN
            } else {
                result.val
            }
        }
    };
}

pub fn gsl_sf_fermi_dirac_m1(x: f64) -> f64 {
    EVAL_RESULT!(gsl_sf_fermi_dirac_m1_e(x, &mut result))
}

// ... (other public wrapper functions would continue here)