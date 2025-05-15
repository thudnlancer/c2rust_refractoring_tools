use std::f64::consts::PI;

// Chebyshev series data
static ACLAUS_DATA: [f64; 15] = [
    2.142694363766688447e+00,
    0.723324281221257925e-01,
    0.101642475021151164e-02,
    0.3245250328531645e-04,
    0.133315187571472e-05,
    0.6213240591653e-07,
    0.313004135337e-08,
    0.16635723056e-09,
    0.919659293e-11,
    0.52400462e-12,
    0.3058040e-13,
    0.18197e-14,
    0.1100e-15,
    0.68e-17,
    0.4e-18,
];

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

static ACLAUS_CS: ChebSeries = ChebSeries {
    data: &ACLAUS_DATA,
    order: 14,
    a: -1.0,
    b: 1.0,
    order_sp: 8, // FIXME: this is a guess, correct value needed here BJG
};

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

fn angle_restrict_pos(x: f64) -> (f64, i32) {
    const TWO_PI: f64 = 2.0 * PI;
    let mut reduced = x % TWO_PI;
    if reduced < 0.0 {
        reduced += TWO_PI;
    }
    (reduced, 0)
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    // Simplified Chebyshev evaluation - for exact translation, 
    // a full implementation would be needed
    let d = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let mut y2 = 0.0;
    let mut y1 = 0.0;
    let mut y = 0.0;
    
    for j in (1..=cs.order).rev() {
        let y_temp = y;
        y = 2.0 * d * y1 - y2 + cs.data[j];
        y2 = y1;
        y1 = y_temp;
    }
    
    y = d * y1 - y2 + 0.5 * cs.data[0];
    
    SfResult {
        val: y,
        err: f64::EPSILON * y.abs(),
    }
}

pub fn clausen_e(x: f64) -> (SfResult, i32) {
    const X_CUT: f64 = PI * f64::EPSILON.sqrt();
    
    let mut x = x;
    let mut sgn = 1.0;
    let status_red;
    
    if x < 0.0 {
        x = -x;
        sgn = -1.0;
    }
    
    // Argument reduction to [0, 2pi)
    let (reduced_x, status) = angle_restrict_pos(x);
    x = reduced_x;
    status_red = status;
    
    // Further reduction to [0,pi)
    if x > PI {
        // simulated extra precision: 2PI = p0 + p1
        const P0: f64 = 6.28125;
        const P1: f64 = 0.19353071795864769253e-02;
        x = (P0 - x) + P1;
        sgn = -sgn;
    }
    
    let mut result = SfResult { val: 0.0, err: 0.0 };
    
    if x == 0.0 {
        result.val = 0.0;
        result.err = 0.0;
    } else if x < X_CUT {
        result.val = x * (1.0 - x.ln());
        result.err = x * f64::EPSILON;
    } else {
        let t = 2.0 * (x * x / (PI * PI) - 0.5;
        let result_c = cheb_eval_e(&ACLAUS_CS, t);
        result.val = x * (result_c.val - x.ln());
        result.err = x * (result_c.err + f64::EPSILON);
    }
    
    result.val *= sgn;
    
    (result, status_red)
}

pub fn clausen(x: f64) -> f64 {
    let (result, _) = clausen_e(x);
    result.val
}