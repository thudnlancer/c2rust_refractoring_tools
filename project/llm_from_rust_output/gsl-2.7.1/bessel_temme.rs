use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
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
pub struct GslResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone)]
pub struct ChebSeries {
    c: Vec<f64>,
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

impl ChebSeries {
    pub fn eval(&self, x: f64) -> GslResult {
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
        
        GslResult {
            val: d,
            err: 2.2204460492503131e-16 * e + self.c[self.order as usize].abs(),
        }
    }
}

lazy_static! {
    static ref G1_CS: ChebSeries = ChebSeries {
        c: vec![
            -1.14516408366268311786898152867,
            0.00636085311347084238122955495,
            0.00186245193007206848934643657,
            0.000152833085873453507081227824,
            0.000017017464011802038795324732,
            -6.4597502923347254354668326451e-07,
            -5.1819848432519380894104312968e-08,
            4.5189092894858183051123180797e-10,
            3.2433227371020873043666259180e-11,
            6.8309434024947522875432400828e-13,
            2.8353502755172101513119628130e-14,
            -7.9883905769323592875638087541e-16,
            -3.3726677300771949833341213457e-17,
            -3.6586334809210520744054437104e-20,
        ],
        order: 13,
        a: -1.0,
        b: 1.0,
        order_sp: 7,
    };

    static ref G2_CS: ChebSeries = ChebSeries {
        c: vec![
            1.882645524949671835019616975350,
            -0.077490658396167518329547945212,
            -0.018256714847324929419579340950,
            0.0006338030209074895795923971731,
            0.0000762290543508729021194461175,
            -9.5501647561720443519853993526e-07,
            -8.8927268107886351912431512955e-08,
            -1.9521334772319613740511880132e-09,
            -9.4003052735885162111769579771e-11,
            4.6875133849532393179290879101e-12,
            2.2658535746925759582447545145e-13,
            -1.1725509698488015111878735251e-15,
            -7.0441338200245222530843155877e-17,
            -2.4377878310107693650659740228e-18,
            -7.5225243218253901727164675011e-20,
        ],
        order: 14,
        a: -1.0,
        b: 1.0,
        order_sp: 8,
    };
}

pub fn temme_gamma(nu: f64) -> Result<(f64, f64, f64, f64), GslError> {
    let anu = nu.abs();
    let x = 4.0 * anu - 1.0;
    
    let r_g1 = G1_CS.eval(x);
    let r_g2 = G2_CS.eval(x);
    
    let g1 = r_g1.val;
    let g2 = r_g2.val;
    let g_1mnu = 1.0 / (g2 + nu * g1);
    let g_1pnu = 1.0 / (g2 - nu * g1);
    
    Ok((g_1pnu, g_1mnu, g1, g2))
}

pub fn bessel_y_temme(nu: f64, x: f64) -> Result<(GslResult, GslResult), GslError> {
    const MAX_ITER: i32 = 15000;
    let half_x = 0.5 * x;
    let ln_half_x = half_x.ln();
    let half_x_nu = (nu * ln_half_x).exp();
    let pi_nu = PI * nu;
    
    let sigma = -nu * ln_half_x;
    
    let sinrat = if pi_nu.abs() < 2.2204460492503131e-16 {
        1.0
    } else {
        pi_nu / pi_nu.sin()
    };
    
    let sinhrat = if sigma.abs() < 2.2204460492503131e-16 {
        1.0
    } else {
        sigma.sinh() / sigma
    };
    
    let alpha = pi_nu / 2.0;
    let sinhalf = if alpha.abs() < 2.2204460492503131e-16 {
        1.0
    } else {
        alpha.sin() / alpha
    };
    
    let sin_sqr = nu * PI * PI * 0.5 * sinhalf * sinhalf;
    
    let (g_1pnu, g_1mnu, g1, g2) = temme_gamma(nu)?;
    
    let mut fk = 2.0 / PI * sinrat * (sigma.cosh() * g1 - sinhrat * ln_half_x * g2);
    let mut pk = 1.0 / PI / half_x_nu * g_1pnu;
    let mut qk = 1.0 / PI * half_x_nu * g_1mnu;
    
    let mut hk = pk;
    let mut ck = 1.0;
    
    let mut sum0 = fk + sin_sqr * qk;
    let mut sum1 = pk;
    
    let mut k = 0;
    
    while k < MAX_ITER {
        k += 1;
        
        fk = (k as f64 * fk + pk + qk) / ((k * k) as f64 - nu * nu);
        ck *= -half_x * half_x / k as f64;
        pk /= k as f64 - nu;
        qk /= k as f64 + nu;
        
        let gk = fk + sin_sqr * qk;
        hk = -(k as f64) * gk + pk;
        
        let del0 = ck * gk;
        let del1 = ck * hk;
        
        sum0 += del0;
        sum1 += del1;
        
        if del0.abs() < 0.5 * (1.0 + sum0.abs()) * 2.2204460492503131e-16 {
            break;
        }
    }
    
    let ynu = GslResult {
        val: -sum0,
        err: (2.0 + 0.5 * k as f64) * 2.2204460492503131e-16 * sum0.abs(),
    };
    
    let ynup1 = GslResult {
        val: -sum1 * 2.0 / x,
        err: (2.0 + 0.5 * k as f64) * 2.2204460492503131e-16 * (sum1 * 2.0 / x).abs(),
    };
    
    if k >= MAX_ITER {
        Err(GslError::MaxIter)
    } else {
        Ok((ynu, ynup1))
    }
}

pub fn bessel_k_scaled_temme(nu: f64, x: f64) -> Result<(f64, f64, f64), GslError> {
    const MAX_ITER: i32 = 15000;
    let half_x = 0.5 * x;
    let ln_half_x = half_x.ln();
    let half_x_nu = (nu * ln_half_x).exp();
    let pi_nu = PI * nu;
    let sigma = -nu * ln_half_x;
    
    let sinrat = if pi_nu.abs() < 2.2204460492503131e-16 {
        1.0
    } else {
        pi_nu / pi_nu.sin()
    };
    
    let sinhrat = if sigma.abs() < 2.2204460492503131e-16 {
        1.0
    } else {
        sigma.sinh() / sigma
    };
    
    let ex = x.exp();
    
    let (g_1pnu, g_1mnu, g1, g2) = temme_gamma(nu)?;
    
    let mut fk = sinrat * (sigma.cosh() * g1 - sinhrat * ln_half_x * g2);
    let mut pk = 0.5 / half_x_nu * g_1pnu;
    let mut qk = 0.5 * half_x_nu * g_1mnu;
    
    let mut hk = pk;
    let mut ck = 1.0;
    
    let mut sum0 = fk;
    let mut sum1 = hk;
    
    let mut k = 0;
    
    while k < MAX_ITER {
        k += 1;
        
        fk = (k as f64 * fk + pk + qk) / ((k * k) as f64 - nu * nu);
        ck *= half_x * half_x / k as f64;
        pk /= k as f64 - nu;
        qk /= k as f64 + nu;
        
        hk = -(k as f64) * fk + pk;
        
        let del0 = ck * fk;
        let del1 = ck * hk;
        
        sum0 += del0;
        sum1 += del1;
        
        if del0.abs() < 0.5 * sum0.abs() * 2.2204460492503131e-16 {
            break;
        }
    }
    
    if k == MAX_ITER {
        return Err(GslError::MaxIter);
    }
    
    let k_nu = sum0 * ex;
    let k_nup1 = sum1 * 2.0 / x * ex;
    let kp_nu = -k_nup1 + nu / x * k_nu;
    
    Ok((k_nu, k_nup1, kp_nu))
}