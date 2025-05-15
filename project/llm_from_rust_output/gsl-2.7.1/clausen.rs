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

#[derive(Debug, PartialEq, Eq)]
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

impl GslError {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(GslError::Success),
            -1 => Some(GslError::Failure),
            -2 => Some(GslError::Continue),
            1 => Some(GslError::Domain),
            2 => Some(GslError::Range),
            3 => Some(GslError::Fault),
            4 => Some(GslError::Invalid),
            5 => Some(GslError::Failed),
            6 => Some(GslError::Factor),
            7 => Some(GslError::Sanity),
            8 => Some(GslError::NoMem),
            9 => Some(GslError::BadFunc),
            10 => Some(GslError::Runaway),
            11 => Some(GslError::MaxIter),
            12 => Some(GslError::ZeroDiv),
            13 => Some(GslError::BadTol),
            14 => Some(GslError::Tol),
            15 => Some(GslError::Underflow),
            16 => Some(GslError::Overflow),
            17 => Some(GslError::Loss),
            18 => Some(GslError::Round),
            19 => Some(GslError::BadLen),
            20 => Some(GslError::NotSquare),
            21 => Some(GslError::Singularity),
            22 => Some(GslError::Diverge),
            23 => Some(GslError::Unsupported),
            24 => Some(GslError::Unimplemented),
            25 => Some(GslError::Cache),
            26 => Some(GslError::Table),
            27 => Some(GslError::NoProgress),
            28 => Some(GslError::NoProgressJ),
            29 => Some(GslError::TolF),
            30 => Some(GslError::TolX),
            31 => Some(GslError::TolG),
            32 => Some(GslError::Eof),
            _ => None,
        }
    }
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> Result<GslSfResult, GslError> {
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        let cj = cs.c[j as usize];
        d = y2 * d - dd + cj;
        e += (y2 * temp).abs() + dd.abs() + cj.abs();
        dd = temp;
    }
    
    let temp = d;
    let c0 = cs.c[0];
    d = y * d - dd + 0.5 * c0;
    e += (y * temp).abs() + dd.abs() + 0.5 * c0.abs();
    
    Ok(GslSfResult {
        val: d,
        err: 2.2204460492503131e-16 * e + cs.c[cs.order as usize].abs(),
    })
}

static ACLAU_DATA: [f64; 15] = [
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

static ACLAU_CS: ChebSeries = ChebSeries {
    c: &ACLAU_DATA,
    order: 14,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

fn angle_restrict_pos(theta: f64) -> (f64, GslError) {
    const PERIOD: f64 = 2.0 * PI;
    let mut x = theta;
    
    if x < 0.0 {
        x = -x;
    }
    
    if x >= PERIOD {
        x = x % PERIOD;
    }
    
    (x, GslError::Success)
}

pub fn gsl_sf_clausen_e(x: f64) -> Result<GslSfResult, GslError> {
    let x_cut = PI * 1.4901161193847656e-08;
    let mut sgn = 1.0;
    let mut x = x;
    
    if x < 0.0 {
        x = -x;
        sgn = -1.0;
    }
    
    let (restricted_x, status_red) = angle_restrict_pos(x);
    x = restricted_x;
    
    if x > PI {
        let p0 = 6.28125;
        let p1 = 0.19353071795864769253e-02;
        x = p0 - x + p1;
        sgn = -sgn;
    }
    
    let mut result = if x == 0.0 {
        GslSfResult { val: 0.0, err: 0.0 }
    } else if x < x_cut {
        GslSfResult {
            val: x * (1.0 - x.ln()),
            err: x * 2.2204460492503131e-16,
        }
    } else {
        let t = 2.0 * (x * x / (PI * PI) - 0.5);
        let result_c = cheb_eval_e(&ACLAU_CS, t)?;
        GslSfResult {
            val: x * (result_c.val - x.ln()),
            err: x * (result_c.err + 2.2204460492503131e-16),
        }
    };
    
    result.val *= sgn;
    
    if status_red != GslError::Success {
        Err(status_red)
    } else {
        Ok(result)
    }
}

pub fn gsl_sf_clausen(x: f64) -> f64 {
    match gsl_sf_clausen_e(x) {
        Ok(result) => result.val,
        Err(e) => {
            eprintln!("Error in gsl_sf_clausen_e: {:?}", e);
            0.0
        }
    }
}