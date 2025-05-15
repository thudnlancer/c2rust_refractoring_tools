use std::f64::consts::{E, PI};
use std::f64::{INFINITY, NAN};

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

const GSL_SUCCESS: i32 = 0;
const GSL_EDOM: i32 = 1;
const GSL_EOVRFLW: i32 = 16;

const K1_POLY: [f64; 9] = [
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

const I1_POLY: [f64; 7] = [
    8.3333333333333325191635191e-02,
    6.9444444444467956461838830e-03,
    3.4722222211230452695165215e-04,
    1.1574075952009842696580084e-05,
    2.7555870002088181016676934e-07,
    4.9724386164128529514040614e-09,
    0.0,
];

const AK1_DATA: [f64; 25] = [
    2.07996868001418246e-01,
    1.62581565017881476e-01,
    -5.87070423518863640e-03,
    4.95021520115789501e-04,
    -5.78958347598556986e-05,
    8.18614610209334726e-06,
    -1.31604832009487277e-06,
    2.32546031520101213e-07,
    -4.42206518311557987e-08,
    8.92163994883100361e-09,
    -1.89046270526983427e-09,
    4.17568808108504702e-10,
    -9.55912361791375794e-11,
    2.25769353153867758e-11,
    -5.48128000211158482e-12,
    1.36386122546441926e-12,
    -3.46936690565986409e-13,
    9.00354564415705942e-14,
    -2.37950577776254432e-14,
    6.39447503964025336e-15,
    -1.74498363492322044e-15,
    4.82994547989290473e-16,
    -1.35460927805445606e-16,
    3.84604274446777234e-17,
    -1.10456856122581316e-17,
];

const AK1_CS: ChebSeries = ChebSeries {
    c: &AK1_DATA,
    order: 24,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const AK12_DATA: [f64; 14] = [
    0.637930834373900104E-1,
    0.283288781304972094E-1,
    -0.247537067390525035E-3,
    0.577197245160724882E-5,
    -0.206893921953654830E-6,
    0.973998344138180418E-8,
    -0.558533614038062498E-9,
    0.373299663404618524E-10,
    -0.282505196102322545E-11,
    0.237201900248414417E-12,
    -0.217667738799175398E-13,
    0.215791416161603245E-14,
    -0.229019693071826928E-15,
    0.258288572982327496E-16,
];

const AK12_CS: ChebSeries = ChebSeries {
    c: &AK12_DATA,
    order: 13,
    a: -1.0,
    b: 1.0,
    order_sp: 7,
};

fn gsl_poly_eval(c: &[f64], x: f64) -> f64 {
    let mut ans = c[c.len() - 1];
    for &coeff in c.iter().rev().skip(1) {
        ans = coeff + x * ans;
    }
    ans
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> Result<GslSfResult, i32> {
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.c[j as usize];
        e += (y2 * temp).abs() + dd.abs() + cs.c[j as usize].abs();
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.c[0];
    e += (y * temp).abs() + dd.abs() + 0.5 * cs.c[0].abs();
    
    Ok(GslSfResult {
        val: d,
        err: 2.2204460492503131e-16 * e + cs.c[cs.order as usize].abs(),
    })
}

pub fn gsl_sf_bessel_K1_scaled_e(x: f64) -> Result<GslSfResult, i32> {
    if x <= 0.0 {
        Err(GSL_EDOM)
    } else if x < 2.0 * f64::MIN_POSITIVE {
        Ok(GslSfResult {
            val: INFINITY,
            err: INFINITY,
        })
    } else if x < 1.0 {
        let lx = x.ln();
        let ex = x.exp();
        let x2 = x * x;
        let t = 0.25 * x2;
        let i1 = 0.5 * x * (1.0 + t * (0.5 + t * gsl_poly_eval(&I1_POLY, t)));
        
        let val = ex * (x2 * gsl_poly_eval(&K1_POLY, x2) + x * lx * i1 + 1.0) / x;
        let err = ex * (1.6 + lx.abs() * 0.6) * 2.2204460492503131e-16;
        
        Ok(GslSfResult {
            val,
            err: err + 2.0 * 2.2204460492503131e-16 * val.abs(),
        })
    } else if x <= 8.0 {
        let sx = x.sqrt();
        let c = cheb_eval_e(&AK1_CS, (16.0 / x - 9.0) / 7.0)?;
        
        let val = (1.375 + c.val) / sx;
        let err = c.err / sx + 2.0 * 2.2204460492503131e-16 * val.abs();
        
        Ok(GslSfResult { val, err })
    } else {
        let sx = x.sqrt();
        let c = cheb_eval_e(&AK12_CS, 16.0 / x - 1.0)?;
        
        let val = (1.25 + c.val) / sx;
        let err = c.err / sx + 2.0 * 2.2204460492503131e-16 * val.abs();
        
        Ok(GslSfResult { val, err })
    }
}

pub fn gsl_sf_bessel_K1_e(x: f64) -> Result<GslSfResult, i32> {
    if x <= 0.0 {
        Err(GSL_EDOM)
    } else if x < 2.0 * f64::MIN_POSITIVE {
        Ok(GslSfResult {
            val: INFINITY,
            err: INFINITY,
        })
    } else if x < 1.0 {
        let lx = x.ln();
        let x2 = x * x;
        let t = 0.25 * x2;
        let i1 = 0.5 * x * (1.0 + t * (0.5 + t * gsl_poly_eval(&I1_POLY, t)));
        
        let val = (x2 * gsl_poly_eval(&K1_POLY, x2) + x * lx * i1 + 1.0) / x;
        let err = (1.6 + lx.abs() * 0.6) * 2.2204460492503131e-16;
        
        Ok(GslSfResult {
            val,
            err: err + 2.0 * 2.2204460492503131e-16 * val.abs(),
        })
    } else {
        let K1_scaled = gsl_sf_bessel_K1_scaled_e(x)?;
        let exp_val = (-x).exp();
        
        let val = exp_val * K1_scaled.val;
        let err = val * (2.2204460492503131e-16 * x.abs() + K1_scaled.err / K1_scaled.val);
        
        Ok(GslSfResult { val, err })
    }
}

pub fn gsl_sf_bessel_K1_scaled(x: f64) -> f64 {
    gsl_sf_bessel_K1_scaled_e(x).unwrap().val
}

pub fn gsl_sf_bessel_K1(x: f64) -> f64 {
    gsl_sf_bessel_K1_e(x).unwrap().val
}