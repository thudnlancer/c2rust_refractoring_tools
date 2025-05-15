use std::f64::consts::PI;
use std::f64::{NAN, INFINITY};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ChebSeries {
    c: &'static [f64],
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

impl ChebSeries {
    fn eval(&self, x: f64, result: &mut GslSfResult) -> i32 {
        let y = (2.0 * x - self.a - self.b) / (self.b - self.a);
        let y2 = 2.0 * y;
        
        let mut d = 0.0;
        let mut dd = 0.0;
        let mut e = 0.0;
        
        for j in (1..=self.order).rev() {
            let temp = d;
            d = y2 * d - dd + self.c[j as usize];
            e += (y2 * temp).abs() + dd.abs() + self.c[j as usize].abs();
            dd = temp;
        }
        
        let temp = d;
        d = y * d - dd + 0.5 * self.c[0];
        e += (y * temp).abs() + dd.abs() + 0.5 * self.c[0].abs();
        
        result.val = d;
        result.err = 2.2204460492503131e-16 * e + self.c[self.order as usize].abs();
        0 // GSL_SUCCESS
    }
}

// Debye function coefficients
const ADEB1_DATA: [f64; 17] = [
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

const ADEB2_DATA: [f64; 18] = [
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

const ADEB3_DATA: [f64; 17] = [
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

const ADEB4_DATA: [f64; 17] = [
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

const ADEB5_DATA: [f64; 17] = [
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

const ADEB6_DATA: [f64; 17] = [
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

const ADEB1_CS: ChebSeries = ChebSeries {
    c: &ADEB1_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const ADEB2_CS: ChebSeries = ChebSeries {
    c: &ADEB2_DATA,
    order: 17,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

const ADEB3_CS: ChebSeries = ChebSeries {
    c: &ADEB3_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

const ADEB4_CS: ChebSeries = ChebSeries {
    c: &ADEB4_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

const ADEB5_CS: ChebSeries = ChebSeries {
    c: &ADEB5_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

const ADEB6_CS: ChebSeries = ChebSeries {
    c: &ADEB6_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

pub fn gsl_sf_debye_1_e(x: f64, result: &mut GslSfResult) -> i32 {
    let val_infinity = PI * PI / 6.0;
    let xcut = -f64::ln(f64::MIN_POSITIVE) - f64::ln(2.0);
    
    if x < 0.0 {
        result.val = NAN;
        result.err = NAN;
        return 1; // GSL_EDOM
    } else if x < 2.0 * f64::EPSILON.sqrt() {
        result.val = 1.0 - 0.25 * x + x * x / 36.0;
        result.err = 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    } else if x <= 4.0 {
        let t = x * x / 8.0 - 1.0;
        let mut c = GslSfResult { val: 0.0, err: 0.0 };
        ADEB1_CS.eval(t, &mut c);
        result.val = c.val - 0.25 * x;
        result.err = c.err + 0.25 * x * 2.2204460492503131e-16;
        0 // GSL_SUCCESS
    } else if x < xcut {
        result.val = (val_infinity - (-x).exp() * (x + 1.0)) / x;
        result.err = 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    } else {
        result.val = val_infinity / x;
        result.err = 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    }
}

// Similar implementations for gsl_sf_debye_2_e through gsl_sf_debye_6_e
// would follow the same pattern as above...

pub fn gsl_sf_debye_1(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    gsl_sf_debye_1_e(x, &mut result);
    result.val
}

// Similar simple wrappers for gsl_sf_debye_2 through gsl_sf_debye_6
// would follow the same pattern...