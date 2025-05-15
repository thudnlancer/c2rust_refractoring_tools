use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Edom = 1,
    Erange = 2,
    Edefault = 3,
    Einval = 4,
    Efailed = 5,
    Efactor = 6,
    Esanity = 7,
    Enomem = 8,
    Ebadfunc = 9,
    Erunaway = 10,
    Emaxiter = 11,
    Ezerodiv = 12,
    Ebadtol = 13,
    Etol = 14,
    Eundrflw = 15,
    Eovrflw = 16,
    Eloss = 17,
    Eround = 18,
    Ebadlen = 19,
    Enotsqr = 20,
    Esing = 21,
    Ediverge = 22,
    Eunsup = 23,
    Eunimpl = 24,
    Ecache = 25,
    Etable = 26,
    Enoprog = 27,
    Enoprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

impl GslError {
    fn from_i32(code: i32) -> Self {
        match code {
            0 => GslError::Success,
            -1 => GslError::Failure,
            -2 => GslError::Continue,
            1 => GslError::Edom,
            2 => GslError::Erange,
            3 => GslError::Edefault,
            4 => GslError::Einval,
            5 => GslError::Efailed,
            6 => GslError::Efactor,
            7 => GslError::Esanity,
            8 => GslError::Enomem,
            9 => GslError::Ebadfunc,
            10 => GslError::Erunaway,
            11 => GslError::Emaxiter,
            12 => GslError::Ezerodiv,
            13 => GslError::Ebadtol,
            14 => GslError::Etol,
            15 => GslError::Eundrflw,
            16 => GslError::Eovrflw,
            17 => GslError::Eloss,
            18 => GslError::Eround,
            19 => GslError::Ebadlen,
            20 => GslError::Enotsqr,
            21 => GslError::Esing,
            22 => GslError::Ediverge,
            23 => GslError::Eunsup,
            24 => GslError::Eunimpl,
            25 => GslError::Ecache,
            26 => GslError::Etable,
            27 => GslError::Enoprog,
            28 => GslError::Enoprogj,
            29 => GslError::Etolf,
            30 => GslError::Etolx,
            31 => GslError::Etolg,
            32 => GslError::Eof,
            _ => GslError::Edefault,
        }
    }
}

fn gsl_max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn gsl_min_dbl(a: f64, b: f64) -> f64 {
    a.min(b)
}

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut GslSfResult) -> GslError {
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
    
    result.val = d;
    result.err = 2.2204460492503131e-16 * e + cs.c[cs.order as usize].abs();
    
    GslError::Success
}

fn sinh_series(x: f64, result: &mut f64) -> GslError {
    let y = x * x;
    *result = x * (1.0 + y * (1.0/6.0 + y * (1.0/120.0 + y * (1.0/5040.0 + y * (1.0/362880.0 + 
             y * (1.0/39916800.0 + y * (1.0/6227020800.0 + y * (1.0/1307674368000.0 + 
             y * (1.0/355687428096000.0)))))));
    GslError::Success
}

fn cosh_m1_series(x: f64, result: &mut f64) -> GslError {
    let y = x * x;
    *result = y * (0.5 + y * (1.0/24.0 + y * (1.0/720.0 + y * (1.0/40320.0 + 
             y * (1.0/3628800.0 + y * (1.0/479001600.0 + y * (1.0/87178291200.0 + 
             y * (1.0/20922789888000.0 + y * (1.0/6402373705728000.0))))))));
    GslError::Success
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

static SINC_CS: ChebSeries = ChebSeries {
    c: &SINC_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

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

static SIN_CS: ChebSeries = ChebSeries {
    c: &SIN_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

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

static COS_CS: ChebSeries = ChebSeries {
    c: &COS_DATA,
    order: 10,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

pub fn gsl_sf_sin_e(x: f64, result: &mut GslSfResult) -> GslError {
    const P1: f64 = 7.85398125648498535156e-1;
    const P2: f64 = 3.77489470793079817668e-8;
    const P3: f64 = 2.69515142907905952645e-15;
    
    let sgn_x = if x >= 0.0 { 1.0 } else { -1.0 };
    let abs_x = x.abs();
    
    if abs_x < 1.2207031250000000e-04 {
        let x2 = x * x;
        result.val = x * (1.0 - x2 / 6.0);
        result.err = (x * x2 * x2 / 100.0).abs();
        return GslError::Success;
    }
    
    let mut sgn_result = sgn_x;
    let y = (abs_x / (0.25 * PI)).floor();
    let octant = (y - (y / 8.0).floor() * 8.0) as i32;
    
    let mut octant = octant;
    let mut y = y;
    
    if octant & 1 != 0 {
        octant += 1;
        octant &= 0o7;
        y += 1.0;
    }
    
    if octant > 3 {
        octant -= 4;
        sgn_result = -sgn_result;
    }
    
    let z = abs_x - y * P1 - y * P2 - y * P3;
    let mut stat_cs;
    
    if octant == 0 {
        let mut sin_cs_result = GslSfResult { val: 0.0, err: 0.0 };
        let t = 8.0 * z.abs() / PI - 1.0;
        stat_cs = cheb_eval_e(&SIN_CS, t, &mut sin_cs_result);
        result.val = z * (1.0 + z * z * sin_cs_result.val);
    } else {
        let mut cos_cs_result = GslSfResult { val: 0.0, err: 0.0 };
        let t = 8.0 * z.abs() / PI - 1.0;
        stat_cs = cheb_eval_e(&COS_CS, t, &mut cos_cs_result);
        result.val = 1.0 - 0.5 * z * z * (1.0 - z * z * cos_cs_result.val);
    }
    
    result.val *= sgn_result;
    
    if abs_x > 1.0 / 2.2204460492503131e-16 {
        result.err = result.val.abs();
    } else if abs_x > 100.0 / 1.4901161193847656e-08 {
        result.err = 2.0 * abs_x * 2.2204460492503131e-16 * result.val.abs();
    } else if abs_x > 0.1 / 1.4901161193847656e-08 {
        result.err = 2.0 * 1.4901161193847656e-08 * result.val.abs();
    } else {
        result.err = 2.0 * 2.2204460492503131e-16 * result.val.abs();
    }
    
    GslError::from_i32(stat_cs as i32)
}

pub fn gsl_sf_cos_e(x: f64, result: &mut GslSfResult) -> GslError {
    const P1: f64 = 7.85398125648498535156e-1;
    const P2: f64 = 3.77489470793079817668e-8;
    const P3: f64 = 2.69515142907905952645e-15;
    
    let abs_x = x.abs();
    
    if abs_x < 1.2207031250000000e-04 {
        let x2 = x * x;
        result.val = 1.0 - 0.5 * x2;
        result.err = (x2 * x2 / 12.0).abs();
        return GslError::Success;
    }
    
    let mut sgn_result = 1.0;
    let y = (abs_x / (0.25 * PI)).floor();
    let octant = (y - (y / 8.0).floor() * 8.0) as i32;
    
    let mut octant = octant;
    let mut y = y;
    
    if octant & 1 != 0 {
        octant += 1;
        octant &= 0o7;
        y += 1.0;
    }
    
    if octant > 3 {
        octant -= 4;
        sgn_result = -sgn_result;
    }
    
    if octant > 1 {
        sgn_result = -sgn_result;
    }
    
    let z = abs_x - y * P1 - y * P2 - y * P3;
    let mut stat_cs;
    
    if octant == 0 {
        let mut cos_cs_result = GslSfResult { val: 0.0, err: 0.0 };
        let t = 8.0 * z.abs() / PI - 1.0;
        stat_cs = cheb_eval_e(&COS_CS, t, &mut cos_cs_result);
        result.val = 1.0 - 0.5 * z * z * (1.0 - z * z * cos_cs_result.val);
    } else {
        let mut sin_cs_result = GslSfResult { val: 0.0, err: 0.0 };
        let t = 8.0 * z.abs() / PI - 1.0;
        stat_cs = cheb_eval_e(&SIN_CS, t, &mut sin_cs_result);
        result.val = z * (1.0 + z * z * sin_cs_result.val);
    }
    
    result.val *= sgn_result;
    
    if abs_x > 1.0 / 2.2204460492503131e-16 {
        result.err = result.val.abs();
    } else if abs_x > 100.0 / 1.4901161193847656e-08 {
        result.err = 2.0 * abs_x * 2.2204460492503131e-16 * result.val.abs();
    } else if abs_x > 0.1 / 1.4901161193847656e-08 {
        result.err = 2.0 * 1.4901161193847656e-08 * result.val.abs();
    } else {
        result.err = 2.0 * 2.2204460492503131e-16 * result.val.abs();
    }
    
    GslError::from_i32(stat_cs as i32)
}

pub fn gsl_sf_hypot_e(x: f64, y: f64, result: &mut GslSfResult) -> GslError {
    if x == 0.0 && y == 0.0 {
        result.val = 0.0;
        result.err = 0.0;
        return GslError::Success;
    }
    
    let a = x.abs();
    let b = y.abs();
    let min = gsl_min_dbl(a, b);
    let max = gsl_max_dbl(a, b);
    let rat = min / max;
    let root_term = (1.0 + rat * rat).sqrt();
    
    if max < f64::MAX / root_term {
        result.val = max * root_term;
        result.err = 2.0 * 2.2204460492503131e-16 * result.val.abs();
        GslError::Success
    } else {
        result.val = INFINITY;
        result.err = INFINITY;
        GslError::Eovrflw
    }
}

pub fn gsl_sf_complex_sin_e(zr: f64, zi: f64, szr: &mut GslSfResult, szi: &mut GslSfResult) -> GslError {
    if zi.abs() < 1.0 {
        let mut ch_m1 = 0.0;
        let mut sh = 0.0;
        sinh_series(zi, &