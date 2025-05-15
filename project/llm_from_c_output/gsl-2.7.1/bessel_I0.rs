use std::f64::consts::{E, SQRT_2};
use std::f64;

const GSL_SQRT_DBL_EPSILON: f64 = f64::EPSILON.sqrt();
const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_LOG_DBL_MAX: f64 = f64::MAX.ln();

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

struct SfResult {
    val: f64,
    err: f64,
}

impl SfResult {
    fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

const BI0_DATA: [f64; 12] = [
    -0.07660547252839144951,
    1.92733795399380827000,
    0.22826445869203013390,
    0.01304891466707290428,
    0.00043442709008164874,
    0.00000942265768600193,
    0.00000014340062895106,
    0.00000000161384906966,
    0.00000000001396650044,
    0.00000000000009579451,
    0.00000000000000053339,
    0.00000000000000000245,
];

const BI0_CS: ChebSeries = ChebSeries {
    data: &BI0_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

const AI0_DATA: [f64; 21] = [
    0.07575994494023796,
    0.00759138081082334,
    0.00041531313389237,
    0.00001070076463439,
    -0.00000790117997921,
    -0.00000078261435014,
    0.00000027838499429,
    0.00000000825247260,
    -0.00000001204463945,
    0.00000000155964859,
    0.00000000022925563,
    -0.00000000011916228,
    0.00000000001757854,
    0.00000000000112822,
    -0.00000000000114684,
    0.00000000000027155,
    -0.00000000000002415,
    -0.00000000000000608,
    0.00000000000000314,
    -0.00000000000000071,
    0.00000000000000007,
];

const AI0_CS: ChebSeries = ChebSeries {
    data: &AI0_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 13,
};

const AI02_DATA: [f64; 22] = [
    0.05449041101410882,
    0.00336911647825569,
    0.00006889758346918,
    0.00000289137052082,
    0.00000020489185893,
    0.00000002266668991,
    0.00000000339623203,
    0.00000000049406022,
    0.00000000001188914,
    -0.00000000003149915,
    -0.00000000001321580,
    -0.00000000000179419,
    0.00000000000071801,
    0.00000000000038529,
    0.00000000000001539,
    -0.00000000000004151,
    -0.00000000000000954,
    0.00000000000000382,
    0.00000000000000176,
    -0.00000000000000034,
    -0.00000000000000027,
    0.00000000000000003,
];

const AI02_CS: ChebSeries = ChebSeries {
    data: &AI02_DATA,
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut SfResult) {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        e += (y2 * temp).abs() + dd.abs() + cs.data[j].abs();
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.data[0];
    e += (y * temp).abs() + dd.abs() + 0.5 * cs.data[0].abs();
    
    result.val = d;
    result.err = GSL_DBL_EPSILON * e + (cs.order_sp as f64 + 1.0) * GSL_DBL_EPSILON * d.abs();
}

pub fn bessel_i0_scaled_e(x: f64) -> Result<SfResult, &'static str> {
    let y = x.abs();
    let mut result = SfResult::new(0.0, 0.0);

    if y < 2.0 * GSL_SQRT_DBL_EPSILON {
        result.val = 1.0 - y;
        result.err = 0.5 * y * y;
        Ok(result)
    } else if y <= 3.0 {
        let ey = (-y).exp();
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&BI0_CS, y * y / 4.5 - 1.0, &mut c);
        result.val = ey * (2.75 + c.val);
        result.err = GSL_DBL_EPSILON * result.val.abs() + ey * c.err;
        Ok(result)
    } else if y <= 8.0 {
        let sy = y.sqrt();
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&AI0_CS, (48.0 / y - 11.0) / 5.0, &mut c);
        result.val = (0.375 + c.val) / sy;
        result.err = 2.0 * GSL_DBL_EPSILON * (0.375 + c.val.abs()) / sy;
        result.err += c.err / sy;
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    } else {
        let sy = y.sqrt();
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&AI02_CS, 16.0 / y - 1.0, &mut c);
        result.val = (0.375 + c.val) / sy;
        result.err = 2.0 * GSL_DBL_EPSILON * (0.375 + c.val.abs()) / sy;
        result.err += c.err / sy;
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    }
}

pub fn bessel_i0_e(x: f64) -> Result<SfResult, &'static str> {
    let y = x.abs();
    let mut result = SfResult::new(0.0, 0.0);

    if y < 2.0 * GSL_SQRT_DBL_EPSILON {
        result.val = 1.0;
        result.err = 0.5 * y * y;
        Ok(result)
    } else if y <= 3.0 {
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&BI0_CS, y * y / 4.5 - 1.0, &mut c);
        result.val = 2.75 + c.val;
        result.err = GSL_DBL_EPSILON * (2.75 + c.val.abs());
        result.err += c.err;
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    } else if y < GSL_LOG_DBL_MAX - 1.0 {
        let ey = y.exp();
        let b_scaled = bessel_i0_scaled_e(x)?;
        result.val = ey * b_scaled.val;
        result.err = ey * b_scaled.err + y * GSL_DBL_EPSILON * result.val.abs();
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    } else {
        Err("overflow")
    }
}

pub fn bessel_i0_scaled(x: f64) -> f64 {
    match bessel_i0_scaled_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

pub fn bessel_i0(x: f64) -> f64 {
    match bessel_i0_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}