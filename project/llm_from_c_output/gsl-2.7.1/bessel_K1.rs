use std::f64::consts;
use std::f64;

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_DBL_MIN: f64 = f64::MIN_POSITIVE;

struct SfResult {
    val: f64,
    err: f64,
}

impl SfResult {
    fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

enum SfError {
    DomainError,
    OverflowError,
    OtherError,
}

fn cheb_eval(cs: &ChebSeries, x: f64) -> f64 {
    let mut d = 0.0;
    let mut dd = 0.0;
    let x2 = 2.0 * x;

    for &c in cs.data.iter().rev() {
        let temp = d;
        d = x2 * d - dd + c;
        dd = temp;
    }

    0.5 * (d - dd)
}

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

static K1_POLY: [f64; 9] = [
    -3.0796575782920622440538935e-01,
    -8.5370719728650778045782736e-02,
    -4.6421827664715603298154971e-03,
    -1.1253607036630425931072996e-04,
    -1.5592887702110907110292728e-06,
    -1.4030163679125934402498239e-08,
    -8.8718998640336832196558868e-11,
    -4.1614323580221539328960335e-13,
    -1.5261293392975541707230366e-15,
];

static I1_POLY: [f64; 6] = [
    8.3333333333333325191635191e-02,
    6.9444444444467956461838830e-03,
    3.4722222211230452695165215e-04,
    1.1574075952009842696580084e-05,
    2.7555870002088181016676934e-07,
    4.9724386164128529514040614e-09,
];

static AK1_DATA: [f64; 25] = [
    +2.07996868001418246e-01,
    +1.62581565017881476e-01,
    -5.87070423518863640e-03,
    +4.95021520115789501e-04,
    -5.78958347598556986e-05,
    +8.18614610209334726e-06,
    -1.31604832009487277e-06,
    +2.32546031520101213e-07,
    -4.42206518311557987e-08,
    +8.92163994883100361e-09,
    -1.89046270526983427e-09,
    +4.17568808108504702e-10,
    -9.55912361791375794e-11,
    +2.25769353153867758e-11,
    -5.48128000211158482e-12,
    +1.36386122546441926e-12,
    -3.46936690565986409e-13,
    +9.00354564415705942e-14,
    -2.37950577776254432e-14,
    +6.39447503964025336e-15,
    -1.74498363492322044e-15,
    +4.82994547989290473e-16,
    -1.35460927805445606e-16,
    +3.84604274446777234e-17,
    -1.10456856122581316e-17,
];

static AK12_DATA: [f64; 14] = [
    +0.637930834373900104E-1,
    +0.283288781304972094E-1,
    -0.247537067390525035E-3,
    +0.577197245160724882E-5,
    -0.206893921953654830E-6,
    +0.973998344138180418E-8,
    -0.558533614038062498E-9,
    +0.373299663404618524E-10,
    -0.282505196102322545E-11,
    +0.237201900248414417E-12,
    -0.217667738799175398E-13,
    +0.215791416161603245E-14,
    -0.229019693071826928E-15,
    +0.258288572982327496E-16,
];

static AK1_CS: ChebSeries = ChebSeries {
    data: &AK1_DATA,
    order: 24,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

static AK12_CS: ChebSeries = ChebSeries {
    data: &AK12_DATA,
    order: 13,
    a: -1.0,
    b: 1.0,
    order_sp: 7,
};

fn poly_eval(p: &[f64], n: usize, x: f64) -> f64 {
    let mut sum = 0.0;
    for i in (0..=n).rev() {
        sum = sum * x + p[i];
    }
    sum
}

pub fn bessel_k1_scaled_e(x: f64) -> Result<SfResult, SfError> {
    if x <= 0.0 {
        Err(SfError::DomainError)
    } else if x < 2.0 * GSL_DBL_MIN {
        Err(SfError::OverflowError)
    } else if x < 1.0 {
        let lx = x.ln();
        let ex = x.exp();
        let x2 = x * x;
        let t = 0.25 * x2;
        let i1 = 0.5 * x * (1.0 + t * (0.5 + t * poly_eval(&I1_POLY, 5, t)));
        let val = ex * (x2 * poly_eval(&K1_POLY, 8, x2) + x * lx * i1 + 1.0) / x;
        let err = ex * (1.6 + lx.abs() * 0.6) * GSL_DBL_EPSILON;
        Ok(SfResult::new(val, err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x <= 8.0 {
        let sx = x.sqrt();
        let c = cheb_eval(&AK1_CS, (16.0 / x - 9.0) / 7.0);
        let val = (1.375 + c) / sx;
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        let sx = x.sqrt();
        let c = cheb_eval(&AK12_CS, 16.0 / x - 1.0);
        let val = (1.25 + c) / sx;
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    }
}

pub fn bessel_k1_e(x: f64) -> Result<SfResult, SfError> {
    if x <= 0.0 {
        Err(SfError::DomainError)
    } else if x < 2.0 * GSL_DBL_MIN {
        Err(SfError::OverflowError)
    } else if x < 1.0 {
        let lx = x.ln();
        let x2 = x * x;
        let t = 0.25 * x2;
        let i1 = 0.5 * x * (1.0 + t * (0.5 + t * poly_eval(&I1_POLY, 5, t)));
        let val = (x2 * poly_eval(&K1_POLY, 8, x2) + x * lx * i1 + 1.0) / x;
        let err = (1.6 + lx.abs() * 0.6) * GSL_DBL_EPSILON;
        Ok(SfResult::new(val, err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        let k1_scaled = bessel_k1_scaled_e(x)?;
        let ex = (-x).exp();
        let val = ex * k1_scaled.val;
        let err = val.abs() * (GSL_DBL_EPSILON * x.abs() + k1_scaled.err / k1_scaled.val);
        Ok(SfResult::new(val, err))
    }
}

pub fn bessel_k1_scaled(x: f64) -> f64 {
    bessel_k1_scaled_e(x).unwrap().val
}

pub fn bessel_k1(x: f64) -> f64 {
    bessel_k1_e(x).unwrap().val
}