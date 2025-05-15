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
            err: f64::EPSILON * e + self.c[self.order as usize].abs(),
        }
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
    c: &BI0_DATA,
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
    c: &AI0_DATA,
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
    c: &AI02_DATA,
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

pub fn gsl_sf_bessel_i0_scaled_e(x: f64) -> Result<GslSfResult, &'static str> {
    let y = x.abs();
    if y < 2.0 * f64::EPSILON {
        Ok(GslSfResult {
            val: 1.0 - y,
            err: 0.5 * y * y,
        })
    } else if y <= 3.0 {
        let ey = (-y).exp();
        let c = BI0_CS.eval(y * y / 4.5 - 1.0);
        Ok(GslSfResult {
            val: ey * (2.75 + c.val),
            err: f64::EPSILON * ey * (2.75 + c.val).abs() + ey * c.err,
        })
    } else if y <= 8.0 {
        let sy = y.sqrt();
        let c = AI0_CS.eval((48.0 / y - 11.0) / 5.0);
        let val = (0.375 + c.val) / sy;
        Ok(GslSfResult {
            val,
            err: 2.0 * f64::EPSILON * (0.375 + c.val.abs()) / sy + c.err / sy + 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let sy = y.sqrt();
        let c = AI02_CS.eval(16.0 / y - 1.0);
        let val = (0.375 + c.val) / sy;
        Ok(GslSfResult {
            val,
            err: 2.0 * f64::EPSILON * (0.375 + c.val.abs()) / sy + c.err / sy + 2.0 * f64::EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_bessel_i0_e(x: f64) -> Result<GslSfResult, &'static str> {
    let y = x.abs();
    if y < 2.0 * f64::EPSILON {
        Ok(GslSfResult {
            val: 1.0,
            err: 0.5 * y * y,
        })
    } else if y <= 3.0 {
        let c = BI0_CS.eval(y * y / 4.5 - 1.0);
        let val = 2.75 + c.val;
        Ok(GslSfResult {
            val,
            err: f64::EPSILON * (2.75 + c.val.abs()) + c.err + 2.0 * f64::EPSILON * val.abs(),
        })
    } else if y < 709.7827128933839 {
        let ey = y.exp();
        let b_scaled = gsl_sf_bessel_i0_scaled_e(x)?;
        let val = ey * b_scaled.val;
        Ok(GslSfResult {
            val,
            err: ey * b_scaled.err + y * f64::EPSILON * val.abs() + 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        Err("overflow")
    }
}

pub fn gsl_sf_bessel_i0_scaled(x: f64) -> f64 {
    gsl_sf_bessel_i0_scaled_e(x).unwrap().val
}

pub fn gsl_sf_bessel_i0(x: f64) -> f64 {
    gsl_sf_bessel_i0_e(x).unwrap().val
}