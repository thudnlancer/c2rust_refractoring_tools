use std::f64::consts::{PI, LN_2};
use std::i32;

const GSL_SUCCESS: i32 = 0;
const GSL_EDOM: i32 = 1;
const GSL_ELOSS: i32 = 2;
const GSL_DBL_EPSILON: f64 = std::f64::EPSILON;
const GSL_SQRT_DBL_EPSILON: f64 = std::f64::EPSILON.sqrt();
const GSL_ROOT4_DBL_EPSILON: f64 = std::f64::EPSILON.sqrt().sqrt();
const GSL_LOG_DBL_MAX: f64 = 709.78271289338397;
const GSL_DBL_MAX: f64 = std::f64::MAX;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

impl GslSfResult {
    pub fn new(val: f64, err: f64) -> Self {
        GslSfResult { val, err }
    }
}

#[inline]
fn gsl_sign(x: f64) -> f64 {
    if x < 0.0 { -1.0 } else { 1.0 }
}

#[inline]
fn gsl_is_odd(n: i32) -> bool {
    n & 1 != 0
}

#[inline]
fn gsl_min_dbl(a: f64, b: f64) -> f64 {
    a.min(b)
}

#[inline]
fn gsl_max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

#[inline]
fn sinh_series(x: f64) -> (f64, i32) {
    let y = x * x;
    let c0 = 1.0 / 6.0;
    let c1 = 1.0 / 120.0;
    let c2 = 1.0 / 5040.0;
    let c3 = 1.0 / 362880.0;
    let c4 = 1.0 / 39916800.0;
    let c5 = 1.0 / 6227020800.0;
    let c6 = 1.0 / 1307674368000.0;
    let c7 = 1.0 / 355687428096000.0;
    let result = x * (1.0 + y * (c0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * c7))))))));
    (result, GSL_SUCCESS)
}

#[inline]
fn cosh_m1_series(x: f64) -> (f64, i32) {
    let y = x * x;
    let c0 = 0.5;
    let c1 = 1.0 / 24.0;
    let c2 = 1.0 / 720.0;
    let c3 = 1.0 / 40320.0;
    let c4 = 1.0 / 3628800.0;
    let c5 = 1.0 / 479001600.0;
    let c6 = 1.0 / 87178291200.0;
    let c7 = 1.0 / 20922789888000.0;
    let c8 = 1.0 / 6402373705728000.0;
    let result = y * (c0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * (c7 + y * c8))))))));
    (result, GSL_SUCCESS)
}

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

static SINC_DATA: [f64; 17] = [
    1.133648177811747875422,
    -0.532677564732557348781,
    -0.068293048346633177859,
    0.033403684226353715020,
    0.001485679893925747818,
    -0.000734421305768455295,
    -0.000016837282388837229,
    0.000008359950146618018,
    0.000000117382095601192,
    -0.000000058413665922724,
    -0.000000000554763755743,
    0.000000000276434190426,
    0.000000000001895374892,
    -0.000000000000945237101,
    -0.000000000000004900690,
    0.000000000000002445383,
    0.000000000000000009925,
];

static SIN_DATA: [f64; 12] = [
    -0.3295190160663511504173,
    0.0025374284671667991990,
    0.0006261928782647355874,
    -4.6495547521854042157541e-06,
    -5.6917531549379706526677e-07,
    3.7283335140973803627866e-09,
    3.0267376484747473727186e-10,
    -1.7400875016436622322022e-12,
    -1.0554678305790849834462e-13,
    5.3701981409132410797062e-16,
    2.5984137983099020336115e-17,
    -1.1821555255364833468288e-19,
];

static COS_DATA: [f64; 11] = [
    0.165391825637921473505668118136,
    -0.00084852883845000173671196530195,
    -0.000210086507222940730213625768083,
    1.16582269619760204299639757584e-6,
    1.43319375856259870334412701165e-7,
    -7.4770883429007141617951330184e-10,
    -6.0969994944584252706997438007e-11,
    2.90748249201909353949854872638e-13,
    1.77126739876261435667156490461e-14,
    -7.6896421502815579078577263149e-17,
    -3.7363121133079412079201377318e-18,
];

static SINC_CS: ChebSeries = ChebSeries {
    data: &SINC_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static SIN_CS: ChebSeries = ChebSeries {
    data: &SIN_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

static COS_CS: ChebSeries = ChebSeries {
    data: &COS_DATA,
    order: 10,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> (f64, i32) {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        e += (y2 * temp).abs() + (dd).abs();
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.data[0];
    e += (y * temp).abs() + (dd).abs();
    
    let result = d;
    let abserr = e * GSL_DBL_EPSILON;
    
    (result, GSL_SUCCESS)
}

pub fn gsl_sf_sin_e(x: f64) -> (GslSfResult, i32) {
    const P1: f64 = 7.85398125648498535156e-1;
    const P2: f64 = 3.77489470793079817668e-8;
    const P3: f64 = 2.69515142907905952645e-15;

    let sgn_x = gsl_sign(x);
    let abs_x = x.abs();

    if abs_x < GSL_ROOT4_DBL_EPSILON {
        let x2 = x * x;
        let val = x * (1.0 - x2 / 6.0);
        let err = (x * x2 * x2 / 100.0).abs();
        (GslSfResult::new(val, err), GSL_SUCCESS)
    } else {
        let mut sgn_result = sgn_x;
        let y = (abs_x / (0.25 * PI)).floor();
        let octant = (y - (y / 8.0).floor() * 8.0) as i32;
        let mut octant = octant;
        let mut y = y;

        if gsl_is_odd(octant) {
            octant += 1;
            octant &= 0x7;
            y += 1.0;
        }

        if octant > 3 {
            octant -= 4;
            sgn_result = -sgn_result;
        }

        let z = ((abs_x - y * P1) - y * P2) - y * P3;

        let (val, stat_cs) = if octant == 0 {
            let t = 8.0 * z.abs() / PI - 1.0;
            let (sin_cs_val, stat) = cheb_eval_e(&SIN_CS, t);
            let val = z * (1.0 + z * z * sin_cs_val);
            (val, stat)
        } else {
            let t = 8.0 * z.abs() / PI - 1.0;
            let (cos_cs_val, stat) = cheb_eval_e(&COS_CS, t);
            let val = 1.0 - 0.5 * z * z * (1.0 - z * z * cos_cs_val);
            (val, stat)
        };

        let val = val * sgn_result;

        let err = if abs_x > 1.0 / GSL_DBL_EPSILON {
            val.abs()
        } else if abs_x > 100.0 / GSL_SQRT_DBL_EPSILON {
            2.0 * abs_x * GSL_DBL_EPSILON * val.abs()
        } else if abs_x > 0.1 / GSL_SQRT_DBL_EPSILON {
            2.0 * GSL_SQRT_DBL_EPSILON * val.abs()
        } else {
            2.0 * GSL_DBL_EPSILON * val.abs()
        };

        (GslSfResult::new(val, err), stat_cs
    }
}

pub fn gsl_sf_cos_e(x: f64) -> (GslSfResult, i32) {
    const P1: f64 = 7.85398125648498535156e-1;
    const P2: f64 = 3.77489470793079817668e-8;
    const P3: f64 = 2.69515142907905952645e-15;

    let abs_x = x.abs();

    if abs_x < GSL_ROOT4_DBL_EPSILON {
        let x2 = x * x;
        let val = 1.0 - 0.5 * x2;
        let err = (x2 * x2 / 12.0).abs();
        (GslSfResult::new(val, err), GSL_SUCCESS)
    } else {
        let mut sgn_result = 1.0;
        let y = (abs_x / (0.25 * PI)).floor();
        let octant = (y - (y / 8.0).floor() * 8.0) as i32;
        let mut octant = octant;
        let mut y = y;

        if gsl_is_odd(octant) {
            octant += 1;
            octant &= 0x7;
            y += 1.0;
        }

        if octant > 3 {
            octant -= 4;
            sgn_result = -sgn_result;
        }

        if octant > 1 {
            sgn_result = -sgn_result;
        }

        let z = ((abs_x - y * P1) - y * P2) - y * P3;

        let (val, stat_cs) = if octant == 0 {
            let t = 8.0 * z.abs() / PI - 1.0;
            let (cos_cs_val, stat) = cheb_eval_e(&COS_CS, t);
            let val = 1.0 - 0.5 * z * z * (1.0 - z * z * cos_cs_val);
            (val, stat)
        } else {
            let t = 8.0 * z.abs() / PI - 1.0;
            let (sin_cs_val, stat) = cheb_eval_e(&SIN_CS, t);
            let val = z * (1.0 + z * z * sin_cs_val);
            (val, stat)
        };

        let val = val * sgn_result;

        let err = if abs_x > 1.0 / GSL_DBL_EPSILON {
            val.abs()
        } else if abs_x > 100.0 / GSL_SQRT_DBL_EPSILON {
            2.0 * abs_x * GSL_DBL_EPSILON * val.abs()
        } else if abs_x > 0.1 / GSL_SQRT_DBL_EPSILON {
            2.0 * GSL_SQRT_DBL_EPSILON * val.abs()
        } else {
            2.0 * GSL_DBL_EPSILON * val.abs()
        };

        (GslSfResult::new(val, err), stat_cs
    }
}

pub fn gsl_sf_hypot_e(x: f64, y: f64) -> (GslSfResult, i32) {
    if x == 0.0 && y == 0.0 {
        (GslSfResult::new(0.0, 0.0), GSL_SUCCESS
    } else {
        let a = x.abs();
        let b = y.abs();
        let min = gsl_min_dbl(a, b);
        let max = gsl_max_dbl(a, b);
        let rat = min / max;
        let root_term = (1.0 + rat * rat).sqrt();

        if max < GSL_DBL_MAX / root_term {
            let val = max * root_term;
            let err = 2.0 * GSL_DBL_EPSILON * val.abs();
            (GslSfResult::new(val, err), GSL_SUCCESS)
        } else {
            (GslSfResult::new(std::f64::INFINITY, std::f64::INFINITY), GSL_EDOM
        }
    }
}

pub fn gsl_sf_complex_sin_e(zr: f64, zi: f64) -> (GslSfResult, GslSfResult, i32) {
    if zi.abs() < 1.0 {
        let (sh, _) = sinh_series(zi);
        let (ch_m1, _) = cosh_m1_series(zi);
        let szr_val = zr.sin() * (ch_m1 + 1.0);
        let szi_val = zr.cos() * sh;
        let szr = GslSfResult::new(szr_val, 2.0 * GSL_DBL_EPSILON * szr_val.abs());
        let szi = GslSfResult::new(szi_val, 2.0 * GSL_DBL_EPSILON * szi_val.abs());
        (szr, szi, GSL_SUCCESS)
    } else if zi.abs() < GSL_LOG_DBL_MAX {
        let ex = zi.exp();
        let ch = 0.5 * (ex + 1.0 / ex);
        let sh = 0.5 * (ex - 1.0 / ex);
        let szr_val = zr.sin() * ch;
        let szi_val = zr.cos() * sh;
        let szr = GslSfResult::new(szr_val, 2.0 * GSL_DBL_EPSILON * szr_val.abs());
        let szi = GslSfResult::new(szi_val, 2.0 * GSL_DBL_EPSILON * szi_val.abs());
        (szr, szi, GSL_SUCCESS)
    } else {
        (GslSfResult::new(std::f64::INFINITY, std::f64::INFINITY),
         GslSfResult::new(std::f64::INFINITY, std::f64::INFINITY),
         GSL_EDOM)
    }
}

pub fn gsl_sf_complex_cos_e(zr: f64, zi: f64) -> (GslSfResult, GslSfResult, i32) {
    if zi.abs() < 1.0 {
        let (sh, _) = sinh_series(zi);
        let (ch_m1, _) = cosh_m1_series(zi);
        let czr_val = zr.cos() * (ch_m1 + 1.0);
        let czi_val = -zr.sin() * sh;
        let czr = GslSfResult::new(czr_val, 2.0 * GSL_DBL