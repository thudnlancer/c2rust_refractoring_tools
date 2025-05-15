use std::f64::consts::PI;

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

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

const ATANINT_CS: ChebSeries = ChebSeries {
    data: &ATANINT_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

pub fn gsl_sf_atanint_e(x: f64) -> Result<SfResult, ()> {
    let ax = x.abs();
    let sgn = x.signum();

    if ax == 0.0 {
        Ok(SfResult::new(0.0, 0.0))
    } else if ax < 0.5 * f64::EPSILON.sqrt() {
        Ok(SfResult::new(x, 0.0))
    } else if ax <= 1.0 {
        let t = 2.0 * (x * x - 0.5);
        let result_c = cheb_eval_e(&ATANINT_CS, t)?;
        let val = x * result_c.val;
        let err = x * result_c.err + f64::EPSILON * val.abs();
        Ok(SfResult::new(val, err))
    } else if ax < 1.0 / f64::EPSILON.sqrt() {
        let t = 2.0 * (1.0 / (x * x) - 0.5);
        let result_c = cheb_eval_e(&ATANINT_CS, t)?;
        let val = sgn * (0.5 * PI * ax.ln() + result_c.val / ax);
        let err = result_c.err / ax + val.abs() * f64::EPSILON;
        Ok(SfResult::new(val, err))
    } else {
        let val = sgn * (0.5 * PI * ax.ln() + 1.0 / ax);
        let err = 2.0 * val.abs() * f64::EPSILON;
        Ok(SfResult::new(val, err))
    }
}

pub fn gsl_sf_atanint(x: f64) -> f64 {
    match gsl_sf_atanint_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> Result<SfResult, ()> {
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

    let val = d;
    let err = f64::EPSILON * e + cs.data[cs.order].abs();
    Ok(SfResult::new(val, err))
}