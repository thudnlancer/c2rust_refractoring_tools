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
    Singular = 21,
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
    pub c: &'static [f64],
    pub order: i32,
    pub a: f64,
    pub b: f64,
    pub order_sp: i32,
}

impl ChebSeries {
    pub fn eval(&self, x: f64) -> Result<GslSfResult, GslError> {
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

static BJ1_DATA: [f64; 12] = [
    -0.11726141513332787,
    -0.25361521830790640,
    0.050127080984469569,
    -0.004631514809625081,
    0.000247996229415914,
    -0.000008678948686278,
    0.000000214293917143,
    -0.000000003936093079,
    0.000000000055911823,
    -0.000000000000632761,
    0.000000000000005840,
    -0.000000000000000044,
];

static BJ1_CS: ChebSeries = ChebSeries {
    c: &BJ1_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

static BM1_CS: ChebSeries = ChebSeries {
    c: &[], // Placeholder - actual coefficients should be provided
    order: 0,
    a: 0.0,
    b: 0.0,
    order_sp: 0,
};

static BTH1_CS: ChebSeries = ChebSeries {
    c: &[], // Placeholder - actual coefficients should be provided
    order: 0,
    a: 0.0,
    b: 0.0,
    order_sp: 0,
};

fn bessel_sin_pi4(y: f64, eps: f64) -> Result<GslSfResult, GslError> {
    let x = y + eps;
    let s = (x * FRAC_PI_4).sin();
    let c = (x * FRAC_PI_4).cos();
    
    Ok(GslSfResult {
        val: s,
        err: (eps * FRAC_PI_4 * c).abs() + f64::EPSILON * s.abs(),
    })
}

pub fn bessel_j1_e(x: f64) -> Result<GslSfResult, GslError> {
    let y = x.abs();
    
    if y == 0.0 {
        return Ok(GslSfResult { val: 0.0, err: 0.0 });
    }
    
    if y < 2.0 * f64::MIN_POSITIVE {
        return Err(GslError::Underflow);
    }
    
    if y < 2.0 * std::f64::consts::SQRT_2 * f64::EPSILON.sqrt() {
        return Ok(GslSfResult { val: 0.5 * x, err: 0.0 });
    }
    
    if y < 4.0 {
        let c = BJ1_CS.eval(0.125 * y * y - 1.0)?;
        return Ok(GslSfResult {
            val: x * (0.25 + c.val),
            err: (x * c.err).abs(),
        });
    }
    
    let z = 32.0 / (y * y) - 1.0;
    let ca = BM1_CS.eval(z)?;
    let ct = BTH1_CS.eval(z)?;
    let sp = bessel_sin_pi4(y, ct.val / y)?;
    
    let sqrty = y.sqrt();
    let ampl = (0.75 + ca.val) / sqrty;
    let val = if x < 0.0 { -ampl } else { ampl } * sp.val;
    
    Ok(GslSfResult {
        val,
        err: sp.val.abs() * ca.err / sqrty + ampl.abs() * sp.err + f64::EPSILON * val.abs(),
    })
}

pub fn bessel_j1(x: f64) -> f64 {
    match bessel_j1_e(x) {
        Ok(result) => result.val,
        Err(e) => {
            eprintln!("Error in bessel_j1: {:?}", e);
            0.0
        }
    }
}