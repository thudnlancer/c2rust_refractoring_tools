use std::f64::consts::{E, PI};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
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
    fn eval(&self, x: f64) -> SfResult {
        let mut d = 0.0;
        let mut dd = 0.0;
        let y = (2.0 * x - self.a - self.b) / (self.b - self.a);
        let y2 = 2.0 * y;
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

        SfResult {
            val: d,
            err: f64::EPSILON * e + self.c[self.order as usize].abs(),
        }
    }
}

const EXPINT3_DATA: [f64; 24] = [
    1.269198414221126014,
    -0.248846446384140982,
    0.80526220717231041e-01,
    -0.25772733251968330e-01,
    0.7599878873073774e-02,
    -0.2030695581940405e-02,
    0.490834586699330e-03,
    -0.107682239142021e-03,
    0.21551726264290e-04,
    -0.3956705137384e-05,
    0.6699240933896e-06,
    -0.105132180807e-06,
    0.15362580199e-07,
    -0.20990960364e-08,
    0.2692109538e-09,
    -0.325195242e-10,
    0.37114816e-11,
    -0.4013652e-12,
    0.412334e-13,
    -0.40338e-14,
    0.3766e-15,
    -0.336e-16,
    0.29e-17,
    -0.2e-18,
];

const EXPINT3_CS: ChebSeries = ChebSeries {
    c: &EXPINT3_DATA,
    order: 23,
    a: -1.0,
    b: 1.0,
    order_sp: 15,
};

const EXPINT3A_DATA: [f64; 23] = [
    1.9270464955068273729,
    -0.349293565204813805e-01,
    0.14503383718983009e-02,
    -0.8925336718327903e-04,
    0.70542392191184e-05,
    -0.6671727454761e-06,
    0.724267589982e-07,
    -0.87825825606e-08,
    0.11672234428e-08,
    -0.1676631281e-09,
    0.257550158e-10,
    -0.41957888e-11,
    0.7201041e-12,
    -0.1294906e-12,
    0.24287e-13,
    -0.47331e-14,
    0.95531e-15,
    -0.1991e-15,
    0.428e-16,
    -0.94e-17,
    0.21e-17,
    -0.5e-18,
    0.1e-18,
];

const EXPINT3A_CS: ChebSeries = ChebSeries {
    c: &EXPINT3A_DATA,
    order: 22,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

pub fn expint_3_e(x: f64) -> Result<SfResult, &'static str> {
    const VAL_INFINITY: f64 = 0.892979511569249211;
    const X_MIN: f64 = 1.6 * 6.0554544523933429e-06;
    const X_THRESHOLD: f64 = 2.0;
    const X_MAX: f64 = (-3.6043653389117154e+01).powf(1.0 / 3.0);

    if x < 0.0 {
        Err("domain error")
    } else if x < X_MIN {
        Ok(SfResult { val: x, err: 0.0 })
    } else if x <= X_THRESHOLD {
        let t = x * x * x / 4.0 - 1.0;
        let result_c = EXPINT3_CS.eval(t);
        Ok(SfResult {
            val: x * result_c.val,
            err: x * result_c.err,
        })
    } else if x < X_MAX {
        let t = 16.0 / (x * x * x) - 1.0;
        let s = (-x * x * x).exp() / (3.0 * x * x);
        let result_c = EXPINT3A_CS.eval(t);
        Ok(SfResult {
            val: VAL_INFINITY - result_c.val * s,
            err: VAL_INFINITY * f64::EPSILON + s * result_c.err,
        })
    } else {
        Ok(SfResult {
            val: VAL_INFINITY,
            err: VAL_INFINITY * f64::EPSILON,
        })
    }
}

pub fn expint_3(x: f64) -> f64 {
    match expint_3_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}