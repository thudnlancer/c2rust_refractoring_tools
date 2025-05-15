use std::f64::consts;
use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ChebSeries {
    c: &'static [f64],
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

#[derive(Debug, PartialEq)]
pub enum GslError {
    Success = 0,
    Domain = 1,
    Range = 2,
    // ... other error codes as needed
}

const GSL_SUCCESS: GslError = GslError::Success;
const GSL_EDOM: GslError = GslError::Domain;

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut GslSfResult) -> GslError {
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.c[j as usize];
        e += (y2 * temp).abs() + dd.abs() + cs.c[j as usize].abs();
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.c[0];
    e += (y * temp).abs() + dd.abs() + 0.5 * cs.c[0].abs();
    
    result.val = d;
    result.err = 2.2204460492503131e-16 * e + cs.c[cs.order as usize].abs();
    
    GSL_SUCCESS
}

static LOPX_DATA: [f64; 21] = [
    2.1664791066439527, -0.2856539855104974, 0.015177672556905537, -0.0020021590494141547,
    0.00019211375164056698, -0.000025532588861055426, 2.900451266040062e-6, -3.8873813517057344e-7,
    4.774367872940046e-8, -6.450196977609032e-9, 8.275197662881239e-10, -1.126049937649205e-10,
    1.4844576692270934e-11, -2.032851597246212e-12, 2.7291231220549215e-13, -3.758197783038794e-14,
    5.110734587086167e-15, -7.072215001143328e-16, 9.708975832824847e-17, -1.3492637457521939e-17,
    1.8657327910677297e-18,
];

static LOPX_CS: ChebSeries = ChebSeries {
    c: &LOPX_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static LOPXMX_DATA: [f64; 20] = [
    -1.121002313237441, 0.19553462773379386, -0.01467470453808084, 0.0016667825047436548,
    -0.0001854335614770037, 0.00002280154021771635, -2.803125311663352e-6, 3.5936568872522163e-7,
    -4.624185704106206e-8, 6.0822637459404e-9, -8.03398244248158e-10, 1.0751718277499375e-10,
    -1.4445310914224613e-11, 1.9573912180610336e-12, -2.661443679679306e-13, 3.6402634315269586e-14,
    -4.993749592275501e-15, 6.880289021884681e-16, -9.503412979480428e-17, 1.3170135013050997e-17,
];

static LOPXMX_CS: ChebSeries = ChebSeries {
    c: &LOPXMX_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

pub fn gsl_sf_log_e(x: f64, result: &mut GslSfResult) -> GslError {
    if x <= 0.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        // In real code, would call error handler here
        return GSL_EDOM;
    }
    
    result.val = x.ln();
    result.err = 2.0 * 2.2204460492503131e-16 * result.val.abs();
    GSL_SUCCESS
}

pub fn gsl_sf_log_abs_e(x: f64, result: &mut GslSfResult) -> GslError {
    if x == 0.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        // In real code, would call error handler here
        return GSL_EDOM;
    }
    
    result.val = x.abs().ln();
    result.err = 2.0 * 2.2204460492503131e-16 * result.val.abs();
    GSL_SUCCESS
}

pub fn gsl_sf_complex_log_e(zr: f64, zi: f64, lnr: &mut GslSfResult, theta: &mut GslSfResult) -> GslError {
    if zr == 0.0 && zi == 0.0 {
        lnr.val = f64::NAN;
        lnr.err = f64::NAN;
        theta.val = f64::NAN;
        theta.err = f64::NAN;
        // In real code, would call error handler here
        return GSL_EDOM;
    }
    
    let ax = zr.abs();
    let ay = zi.abs();
    let min = ax.min(ay);
    let max = ax.max(ay);
    
    lnr.val = max.ln() + 0.5 * (1.0 + (min / max).powi(2)).ln();
    lnr.err = 2.0 * 2.2204460492503131e-16 * lnr.val.abs();
    
    theta.val = zi.atan2(zr);
    theta.err = 2.2204460492503131e-16 * lnr.val.abs();
    
    GSL_SUCCESS
}

pub fn gsl_sf_log_1plusx_e(x: f64, result: &mut GslSfResult) -> GslError {
    if x <= -1.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        // In real code, would call error handler here
        return GSL_EDOM;
    } else if x.abs() < 2.4607833005759251e-03 {
        let c1 = -0.5;
        let c2 = 1.0 / 3.0;
        let c3 = -1.0 / 4.0;
        let c4 = 1.0 / 5.0;
        let c5 = -1.0 / 6.0;
        let c6 = 1.0 / 7.0;
        let c7 = -1.0 / 8.0;
        let c8 = 1.0 / 9.0;
        let c9 = -1.0 / 10.0;
        
        let t = c5 + x * (c6 + x * (c7 + x * (c8 + x * c9)));
        result.val = x * (1.0 + x * (c1 + x * (c2 + x * (c3 + x * (c4 + x * t))));
        result.err = 2.2204460492503131e-16 * result.val.abs();
        GSL_SUCCESS
    } else if x.abs() < 0.5 {
        let t = 0.5 * (8.0 * x + 1.0) / (x + 2.0);
        let mut c = GslSfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&LOPX_CS, t, &mut c);
        result.val = x * c.val;
        result.err = (x * c.err).abs();
        GSL_SUCCESS
    } else {
        result.val = (1.0 + x).ln();
        result.err = 2.2204460492503131e-16 * result.val.abs();
        GSL_SUCCESS
    }
}

pub fn gsl_sf_log_1plusx_mx_e(x: f64, result: &mut GslSfResult) -> GslError {
    if x <= -1.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        // In real code, would call error handler here
        return GSL_EDOM;
    } else if x.abs() < 7.4009597974140505e-04 {
        let c1 = -0.5;
        let c2 = 1.0 / 3.0;
        let c3 = -1.0 / 4.0;
        let c4 = 1.0 / 5.0;
        let c5 = -1.0 / 6.0;
        let c6 = 1.0 / 7.0;
        let c7 = -1.0 / 8.0;
        let c8 = 1.0 / 9.0;
        let c9 = -1.0 / 10.0;
        
        let t = c5 + x * (c6 + x * (c7 + x * (c8 + x * c9)));
        result.val = x * x * (c1 + x * (c2 + x * (c3 + x * (c4 + x * t)));
        result.err = 2.2204460492503131e-16 * result.val.abs();
        GSL_SUCCESS
    } else if x.abs() < 0.5 {
        let t = 0.5 * (8.0 * x + 1.0) / (x + 2.0);
        let mut c = GslSfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&LOPXMX_CS, t, &mut c);
        result.val = x * x * c.val;
        result.err = x * x * c.err;
        GSL_SUCCESS
    } else {
        let lterm = (1.0 + x).ln();
        result.val = lterm - x;
        result.err = 2.2204460492503131e-16 * (lterm.abs() + x.abs());
        GSL_SUCCESS
    }
}

pub fn gsl_sf_log(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    match gsl_sf_log_e(x, &mut result) {
        GSL_SUCCESS => result.val,
        _ => {
            // In real code, would call error handler here
            result.val
        }
    }
}

pub fn gsl_sf_log_abs(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    match gsl_sf_log_abs_e(x, &mut result) {
        GSL_SUCCESS => result.val,
        _ => {
            // In real code, would call error handler here
            result.val
        }
    }
}

pub fn gsl_sf_log_1plusx(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    match gsl_sf_log_1plusx_e(x, &mut result) {
        GSL_SUCCESS => result.val,
        _ => {
            // In real code, would call error handler here
            result.val
        }
    }
}

pub fn gsl_sf_log_1plusx_mx(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    match gsl_sf_log_1plusx_mx_e(x, &mut result) {
        GSL_SUCCESS => result.val,
        _ => {
            // In real code, would call error handler here
            result.val
        }
    }
}