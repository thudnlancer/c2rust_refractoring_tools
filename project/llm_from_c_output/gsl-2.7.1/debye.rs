use std::f64::consts::{LN_2, PI};
use std::f64;

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_DBL_MIN: f64 = f64::MIN_POSITIVE;
const GSL_LOG_DBL_MIN: f64 = GSL_DBL_MIN.ln();
const M_SQRT2: f64 = std::f64::consts::SQRT_2;

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

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut temp = 0.0;
    
    for j in (1..=cs.order).rev() {
        temp = d;
        d = y2 * d - dd + cs.data[j];
        dd = temp;
    }
    
    temp = y * d - dd + 0.5 * cs.data[0];
    SfResult::new(temp, cs.order as f64 * GSL_DBL_EPSILON * temp.abs())
}

static ADEB1_DATA: [f64; 17] = [
    2.4006597190381410194,
    0.1937213042189360089,
    -0.62329124554895770e-02,
    0.3511174770206480e-03,
    -0.228222466701231e-04,
    0.15805467875030e-05,
    -0.1135378197072e-06,
    0.83583361188e-08,
    -0.6264424787e-09,
    0.476033489e-10,
    -0.36574154e-11,
    0.2835431e-12,
    -0.221473e-13,
    0.17409e-14,
    -0.1376e-15,
    0.109e-16,
    -0.9e-18,
];

static ADEB1_CS: ChebSeries = ChebSeries {
    data: &ADEB1_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

static ADEB2_DATA: [f64; 18] = [
    2.5943810232570770282,
    0.2863357204530719834,
    -0.102062656158046713e-01,
    0.6049109775346844e-03,
    -0.405257658950210e-04,
    0.28633826328811e-05,
    -0.2086394303065e-06,
    0.155237875826e-07,
    -0.11731280087e-08,
    0.897358589e-10,
    -0.69317614e-11,
    0.5398057e-12,
    -0.423241e-13,
    0.33378e-14,
    -0.2645e-15,
    0.211e-16,
    -0.17e-17,
    0.1e-18,
];

static ADEB2_CS: ChebSeries = ChebSeries {
    data: &ADEB2_DATA,
    order: 17,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static ADEB3_DATA: [f64; 17] = [
    2.707737068327440945,
    0.340068135211091751,
    -0.12945150184440869e-01,
    0.7963755380173816e-03,
    -0.546360009590824e-04,
    0.39243019598805e-05,
    -0.2894032823539e-06,
    0.217317613962e-07,
    -0.16542099950e-08,
    0.1272796189e-09,
    -0.987963460e-11,
    0.7725074e-12,
    -0.607797e-13,
    0.48076e-14,
    -0.3820e-15,
    0.305e-16,
    -0.24e-17,
];

static ADEB3_CS: ChebSeries = ChebSeries {
    data: &ADEB3_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static ADEB4_DATA: [f64; 17] = [
    2.781869415020523460,
    0.374976783526892863,
    -0.14940907399031583e-01,
    0.945679811437042e-03,
    -0.66132916138933e-04,
    0.4815632982144e-05,
    -0.3588083958759e-06,
    0.271601187416e-07,
    -0.20807099122e-08,
    0.1609383869e-09,
    -0.125470979e-10,
    0.9847265e-12,
    -0.777237e-13,
    0.61648e-14,
    -0.4911e-15,
    0.393e-16,
    -0.32e-17,
];

static ADEB4_CS: ChebSeries = ChebSeries {
    data: &ADEB4_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static ADEB5_DATA: [f64; 17] = [
    2.8340269546834530149,
    0.3994098857106266445,
    -0.164566764773099646e-1,
    0.10652138340664541e-2,
    -0.756730374875418e-4,
    0.55745985240273e-5,
    -0.4190692330918e-6,
    0.319456143678e-7,
    -0.24613318171e-8,
    0.1912801633e-9,
    -0.149720049e-10,
    0.11790312e-11,
    -0.933329e-13,
    0.74218e-14,
    -0.5925e-15,
    0.475e-16,
    -0.39e-17,
];

static ADEB5_CS: ChebSeries = ChebSeries {
    data: &ADEB5_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static ADEB6_DATA: [f64; 17] = [
    2.8726727134130122113,
    0.4174375352339027746,
    -0.176453849354067873e-1,
    0.11629852733494556e-2,
    -0.837118027357117e-4,
    0.62283611596189e-5,
    -0.4718644465636e-6,
    0.361950397806e-7,
    -0.28030368010e-8,
    0.2187681983e-9,
    -0.171857387e-10,
    0.13575809e-11,
    -0.1077580e-12,
    0.85893e-14,
    -0.6872e-15,
    0.552e-16,
    -0.44e-17,
];

static ADEB6_CS: ChebSeries = ChebSeries {
    data: &ADEB6_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

fn debye_1_e(x: f64) -> Result<SfResult, &'static str> {
    const VAL_INFINITY: f64 = PI * PI / 6.0;
    let xcut = -GSL_LOG_DBL_MIN;

    if x < 0.0 {
        Err("domain error")
    } else if x < 2.0 * GSL_DBL_EPSILON.sqrt() {
        let val = 1.0 - 0.25 * x + x * x / 36.0;
        Ok(SfResult::new(val, GSL_DBL_EPSILON * val.abs()))
    } else if x <= 4.0 {
        let t = x * x / 8.0 - 1.0;
        let c = cheb_eval_e(&ADEB1_CS, t);
        let val = c.val - 0.25 * x;
        let err = c.err + 0.25 * x * GSL_DBL_EPSILON;
        Ok(SfResult::new(val, err))
    } else if x < -(LN_2 + GSL_DBL_EPSILON.ln()) {
        let nexp = (xcut / x).floor() as i32;
        let ex = (-x).exp();
        let mut sum = 0.0;
        let mut xk = nexp as f64 * x;
        let mut rk = nexp as f64;
        
        for i in (1..=nexp).rev() {
            sum *= ex;
            sum += (1.0 + 1.0 / xk) / rk;
            rk -= 1.0;
            xk -= x;
        }
        
        let val = VAL_INFINITY / x - sum * ex;
        Ok(SfResult::new(val, GSL_DBL_EPSILON * val.abs()))
    } else if x < xcut {
        let val = (VAL_INFINITY - (-x).exp() * (x + 1.0)) / x;
        Ok(SfResult::new(val, GSL_DBL_EPSILON * val.abs()))
    } else {
        let val = VAL_INFINITY / x;
        Ok(SfResult::new(val, GSL_DBL_EPSILON * val.abs()))
    }
}

// Similar implementations for debye_2_e through debye_6_e would follow
// with the same pattern but using their respective constants and ChebSeries

fn debye_1(x: f64) -> f64 {
    match debye_1_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

// Similar wrapper functions for debye_2 through debye_6 would follow