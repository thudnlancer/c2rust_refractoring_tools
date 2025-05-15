use std::f64::consts::FRAC_PI_4;

pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singularity = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

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
    fn eval(&self, x: f64) -> Result<GslSfResult, GslError> {
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
        
        Ok(GslSfResult {
            val: d,
            err: 2.2204460492503131e-16 * e + self.c[self.order as usize].abs(),
        })
    }
}

static BJ0_DATA: [f64; 13] = [
    0.100254161968939137,
    -0.665223007764405132,
    0.248983703498281314,
    -0.0332527231700357697,
    0.0023114179304694015,
    -0.0000991127741995080,
    0.0000028916708643998,
    -0.0000000612108586630,
    0.0000000009838650793,
    -0.0000000000124235515,
    0.0000000000001265433,
    -0.0000000000000010619,
    0.0000000000000000074,
];

static BJ0_CS: ChebSeries = ChebSeries {
    c: &BJ0_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

static BM0_CS: ChebSeries = ChebSeries {
    c: &[], // Actual coefficients should be filled in
    order: 0,
    a: 0.0,
    b: 0.0,
    order_sp: 0,
};

static BTH0_CS: ChebSeries = ChebSeries {
    c: &[], // Actual coefficients should be filled in
    order: 0,
    a: 0.0,
    b: 0.0,
    order_sp: 0,
};

fn bessel_cos_pi4(y: f64, eps: f64) -> Result<GslSfResult, GslError> {
    let arg = y - FRAC_PI_4 + eps;
    Ok(GslSfResult {
        val: arg.cos(),
        err: (arg.sin() * eps).abs() + 2.2204460492503131e-16 * arg.cos().abs(),
    })
}

pub fn bessel_j0_e(x: f64) -> Result<GslSfResult, GslError> {
    let y = x.abs();
    if y < 2.0 * f64::EPSILON {
        Ok(GslSfResult {
            val: 1.0,
            err: y * y,
        })
    } else if y <= 4.0 {
        BJ0_CS.eval(0.125 * y * y - 1.0)
    } else {
        let z = 32.0 / (y * y) - 1.0;
        let ca = BM0_CS.eval(z)?;
        let ct = BTH0_CS.eval(z)?;
        let cp = bessel_cos_pi4(y, ct.val / y)?;
        
        let sqrty = y.sqrt();
        let ampl = (0.75 + ca.val) / sqrty;
        let val = ampl * cp.val;
        let err = cp.val.abs() * ca.err / sqrty + ampl.abs() * cp.err;
        
        Ok(GslSfResult {
            val,
            err: err + 2.2204460492503131e-16 * val.abs(),
        })
    }
}

pub fn bessel_j0(x: f64) -> f64 {
    match bessel_j0_e(x) {
        Ok(result) => result.val,
        Err(e) => {
            // Handle error appropriately
            eprintln!("Error in bessel_j0: {:?}", e);
            f64::NAN
        }
    }
}