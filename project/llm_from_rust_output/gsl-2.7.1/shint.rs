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

impl ChebSeries {
    fn eval(&self, x: f64) -> GslSfResult {
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

        GslSfResult {
            val: d,
            err: 2.2204460492503131e-16 * e + self.c[self.order as usize].abs(),
        }
    }
}

const SHI_DATA: [f64; 7] = [
    0.0078372685688900950695,
    0.0039227664934234563973,
    0.0000041346787887617267,
    0.0000000024707480372883,
    0.0000000000009379295591,
    0.0000000000000002451817,
    0.0000000000000000000467,
];

const SHI_CS: ChebSeries = ChebSeries {
    c: &SHI_DATA,
    order: 6,
    a: -1.0,
    b: 1.0,
    order_sp: 6,
};

pub fn gsl_sf_shi_e(x: f64) -> Result<GslSfResult, &'static str> {
    let ax = x.abs();
    let xsml = 1.4901161193847656e-08;

    if ax < xsml {
        Ok(GslSfResult { val: x, err: 0.0 })
    } else if ax <= 0.375 {
        let result_c = SHI_CS.eval(128.0 * x * x / 9.0 - 1.0);
        let val = x * (1.0 + result_c.val);
        let err = x * result_c.err + 2.0 * 2.2204460492503131e-16 * val.abs();
        Ok(GslSfResult { val, err })
    } else {
        let result_ei = gsl_sf_expint_ei_e(x)?;
        let result_e1 = gsl_sf_expint_e1_e(x)?;
        let val = 0.5 * (result_ei.val + result_e1.val);
        let err = 0.5 * (result_ei.err + result_e1.err) + 2.0 * 2.2204460492503131e-16 * val.abs();
        
        if result_ei.val == f64::NEG_INFINITY && result_e1.val == f64::NEG_INFINITY {
            Err("underflow")
        } else if result_ei.val == f64::INFINITY || result_e1.val == f64::INFINITY {
            Err("overflow")
        } else {
            Ok(GslSfResult { val, err })
        }
    }
}

pub fn gsl_sf_chi_e(x: f64) -> Result<GslSfResult, &'static str> {
    let result_ei = gsl_sf_expint_ei_e(x)?;
    let result_e1 = gsl_sf_expint_e1_e(x)?;

    if x < 0.0 {
        Err("domain error")
    } else if result_ei.val == f64::NEG_INFINITY && result_e1.val == f64::NEG_INFINITY {
        Ok(GslSfResult {
            val: 0.0,
            err: 2.2250738585072014e-308,
        })
    } else if result_ei.val == f64::INFINITY || result_e1.val == f64::INFINITY {
        Ok(GslSfResult {
            val: f64::INFINITY,
            err: f64::INFINITY,
        })
    } else {
        let val = 0.5 * (result_ei.val - result_e1.val);
        let err = 0.5 * (result_ei.err + result_e1.err) + 2.0 * 2.2204460492503131e-16 * val.abs();
        Ok(GslSfResult { val, err })
    }
}

pub fn gsl_sf_shi(x: f64) -> f64 {
    match gsl_sf_shi_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

pub fn gsl_sf_chi(x: f64) -> f64 {
    match gsl_sf_chi_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

// These would need to be implemented or wrapped from the GSL library
fn gsl_sf_expint_ei_e(x: f64) -> Result<GslSfResult, &'static str> {
    unimplemented!("Wrapper for GSL's gsl_sf_expint_Ei_e needed")
}

fn gsl_sf_expint_e1_e(x: f64) -> Result<GslSfResult, &'static str> {
    unimplemented!("Wrapper for GSL's gsl_sf_expint_E1_e needed")
}