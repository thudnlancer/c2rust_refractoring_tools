use std::f64::consts::PI;

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

impl ChebSeries {
    fn eval(&self, x: f64) -> GslSfResult {
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
        
        GslSfResult {
            val: d,
            err: 2.2204460492503131e-16 * e + self.c[self.order as usize].abs(),
        }
    }
}

const ATANINT_DATA: [f64; 21] = [
    1.91040361296235937512,
    -0.4176351437656746940e-01,
    0.275392550786367434e-02,
    -0.25051809526248881e-03,
    0.2666981285121171e-04,
    -0.311890514107001e-05,
    0.38833853132249e-06,
    -0.5057274584964e-07,
    0.681225282949e-08,
    -0.94212561654e-09,
    0.13307878816e-09,
    -0.1912678075e-10,
    0.278912620e-11,
    -0.41174820e-12,
    0.6142987e-13,
    -0.924929e-14,
    0.140387e-14,
    -0.21460e-15,
    0.3301e-16,
    -0.511e-17,
    0.79e-18,
];

static ATANINT_CS: ChebSeries = ChebSeries {
    c: &ATANINT_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

pub fn gsl_sf_atanint_e(x: f64) -> GslSfResult {
    let ax = x.abs();
    let sgn = if x >= 0.0 { 1.0 } else { -1.0 };

    if ax == 0.0 {
        GslSfResult { val: 0.0, err: 0.0 }
    } else if ax < 0.5 * 1.4901161193847656e-08 {
        GslSfResult { val: x, err: 0.0 }
    } else if ax <= 1.0 {
        let t = 2.0 * (x * x - 0.5);
        let result_c = ATANINT_CS.eval(t);
        let val = x * result_c.val;
        let mut err = x * result_c.err;
        err += 2.2204460492503131e-16 * val.abs();
        GslSfResult { val, err }
    } else if ax < 1.0 / 1.4901161193847656e-08 {
        let t = 2.0 * (1.0 / (x * x) - 0.5);
        let result_c = ATANINT_CS.eval(t);
        let val = sgn * (0.5 * PI * ax.ln() + result_c.val / ax);
        let mut err = result_c.err / ax + val.abs() * 2.2204460492503131e-16;
        err += 2.2204460492503131e-16 * val.abs();
        GslSfResult { val, err }
    } else {
        let val = sgn * (0.5 * PI * ax.ln() + 1.0 / ax);
        let err = 2.0 * val.abs() * 2.2204460492503131e-16;
        GslSfResult { val, err }
    }
}

pub fn gsl_sf_atanint(x: f64) -> f64 {
    let result = gsl_sf_atanint_e(x);
    result.val
}