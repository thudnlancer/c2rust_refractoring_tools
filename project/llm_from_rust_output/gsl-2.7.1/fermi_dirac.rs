use libc::{c_double, c_int, c_uint};
use std::f64::consts::PI;

const GSL_SUCCESS: c_int = 0;
const GSL_EUNDRFLW: c_int = 15;
const GSL_EOVRFLW: c_int = 16;
const GSL_ESANITY: c_int = 7;
const GSL_EUNIMPL: c_int = 24;
const GSL_EDOM: c_int = 1;
const GSL_EMAXITER: c_int = 11;

#[derive(Clone, Copy)]
pub struct GslSfResult {
    pub val: c_double,
    pub err: c_double,
}

#[derive(Clone, Copy)]
struct ChebSeries {
    c: *const c_double,
    order: c_int,
    a: c_double,
    b: c_double,
    order_sp: c_int,
}

const ITMAX: c_uint = 100;
const QSIZE: c_uint = 101;
const NSIZE: c_uint = 101;

fn cheb_eval_e(cs: &ChebSeries, x: c_double, result: &mut GslSfResult) -> c_int {
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut e = 0.0;
    
    let mut j = cs.order;
    while j >= 1 {
        let temp = d;
        d = y2 * d - dd + unsafe { *cs.c.offset(j as isize) };
        e += (y2 * temp).abs() + dd.abs() + unsafe { (*cs.c.offset(j as isize)).abs() };
        dd = temp;
        j -= 1;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * unsafe { *cs.c.offset(0) };
    e += (y * temp).abs() + dd.abs() + 0.5 * unsafe { (*cs.c.offset(0)).abs() };
    
    result.val = d;
    result.err = 2.2204460492503131e-16 * e + unsafe { (*cs.c.offset(cs.order as isize))).abs() };
    GSL_SUCCESS
}

// Static Chebyshev series data and initialization
static FD_1_A_DATA: [c_double; 22] = [
    // ... (data values remain the same)
];

static FD_1_A_CS: ChebSeries = ChebSeries {
    c: FD_1_A_DATA.as_ptr(),
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

// ... (other static series definitions remain similar)

fn fd_whiz(
    term: c_double,
    iterm: c_int,
    qnum: &mut [c_double],
    qden: &mut [c_double],
    result: &mut c_double,
    s: &mut c_double,
) -> c_int {
    if iterm == 0 {
        *s = 0.0;
    }
    *s += term;
    
    qden[iterm as usize] = 1.0 / (term * (iterm as c_double + 1.0).powi(2));
    qnum[iterm as usize] = *s * qden[iterm as usize];
    
    if iterm > 0 {
        let mut factor = 1.0;
        let ratio = iterm as c_double / (iterm as c_double + 1.0);
        
        for j in (0..iterm).rev() {
            let c = factor * (j as c_double + 1.0) / (iterm as c_double + 1.0);
            factor *= ratio;
            
            qden[j as usize] = qden[(j + 1) as usize] - c * qden[j as usize];
            qnum[j as usize] = qnum[(j + 1) as usize] - c * qnum[j as usize];
        }
    }
    
    *result = qnum[0] / qden[0];
    GSL_SUCCESS
}

fn fd_nint(j: c_int, x: c_double, result: &mut GslSfResult) -> c_int {
    if j >= -1 {
        result.val = 0.0;
        result.err = 0.0;
        // gsl_error(...);
        return GSL_ESANITY;
    } else if j < -(NSIZE as c_int) {
        result.val = 0.0;
        result.err = 0.0;
        // gsl_error(...);
        return GSL_EUNIMPL;
    }
    
    let mut qcoeff = [0.0; 101];
    let n = -(j + 1);
    
    qcoeff[1] = 1.0;
    for k in 2..=n {
        qcoeff[k as usize] = -qcoeff[(k - 1) as usize];
        
        for i in (2..k).rev() {
            qcoeff[i as usize] = i as c_double * qcoeff[i as usize] 
                - (k - (i - 1)) as c_double * qcoeff[(i - 1) as usize];
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
    result.err = 3.0 * 2.2204460492503131e-16 * (f * a * p).abs();
    GSL_SUCCESS
}

// ... (other function implementations follow similar patterns)

pub fn gsl_sf_fermi_dirac_m1_e(x: c_double, result: &mut GslSfResult) -> c_int {
    if x < -7.0839641853226408e+02 {
        result.val = 0.0;
        result.err = 2.2250738585072014e-308;
        // gsl_error(...);
        GSL_EUNDRFLW
    } else if x < 0.0 {
        let ex = x.exp();
        result.val = ex / (1.0 + ex);
        result.err = 2.0 * (x.abs() + 1.0) * 2.2204460492503131e-16 * result.val.abs();
        GSL_SUCCESS
    } else {
        let ex = (-x).exp();
        result.val = 1.0 / (1.0 + ex);
        result.err = 2.0 * 2.2204460492503131e-16 * (x + 1.0) * ex;
        GSL_SUCCESS
    }
}

// ... (other public function implementations)

pub fn gsl_sf_fermi_dirac_m1(x: c_double) -> c_double {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_fermi_dirac_m1_e(x, &mut result);
    if status != GSL_SUCCESS {
        // Handle error
    }
    result.val
}

// ... (other convenience functions)