use std::f64::consts::{PI, SQRT_2};
use std::f64;

const SQRT_3: f64 = 1.7320508075688772;
const GAMMA_1_3: f64 = 2.6789385347077476;
const GAMMA_4_3: f64 = 0.8929795115692492;
const GSL_SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-8;
const GSL_DBL_EPSILON: f64 = 2.220446049250313e-16;
const GSL_LOG_DBL_MIN: f64 = -708.3964185322641;

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

struct SfResult {
    val: f64,
    err: f64,
}

impl SfResult {
    fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

static SYNCHROTRON1_DATA: [f64; 13] = [
    30.364682982501076273,
    17.079395277408394574,
    4.560132133545072889,
    0.549281246730419979,
    0.372976075069301172e-1,
    0.161362430201041242e-2,
    0.481916772120371e-4,
    0.10512425288938e-5,
    0.174638504670e-7,
    0.22815486544e-9,
    0.240443082e-11,
    0.2086588e-13,
    0.15167e-15,
];

static SYNCHROTRON1_CS: ChebSeries = ChebSeries {
    data: &SYNCHROTRON1_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

static SYNCHROTRON2_DATA: [f64; 12] = [
    0.4490721623532660844,
    0.898353677994187218e-1,
    0.81044573772151290e-2,
    0.4261716991089162e-3,
    0.147609631270746e-4,
    0.3628633615300e-6,
    0.66634807498e-8,
    0.949077166e-10,
    0.1079125e-11,
    0.10022e-13,
    0.77e-16,
    0.5e-18,
];

static SYNCHROTRON2_CS: ChebSeries = ChebSeries {
    data: &SYNCHROTRON2_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 7,
};

static SYNCHROTRON1A_DATA: [f64; 23] = [
    2.1329305161355000985,
    0.741352864954200240e-1,
    0.86968099909964198e-2,
    0.11703826248775692e-2,
    0.1645105798619192e-3,
    0.240201021420640e-4,
    0.35827756389389e-5,
    0.5447747626984e-6,
    0.838802856196e-7,
    0.13069882684e-7,
    0.2053099071e-8,
    0.325187537e-9,
    0.517914041e-10,
    0.83002988e-11,
    0.13352728e-11,
    0.2159150e-12,
    0.349967e-13,
    0.56994e-14,
    0.9291e-15,
    0.152e-15,
    0.249e-16,
    0.41e-17,
    0.7e-18,
];

static SYNCHROTRON1A_CS: ChebSeries = ChebSeries {
    data: &SYNCHROTRON1A_DATA,
    order: 22,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

static SYNCHROTRON21_DATA: [f64; 13] = [
    38.617839923843085480,
    23.037715594963734597,
    5.3802499868335705968,
    0.6156793806995710776,
    0.406688004668895584e-1,
    0.17296274552648414e-2,
    0.51061258836577e-4,
    0.110459595022e-5,
    0.18235530206e-7,
    0.2370769803e-9,
    0.24887296e-11,
    0.21529e-13,
    0.156e-15,
];

static SYNCHROTRON21_CS: ChebSeries = ChebSeries {
    data: &SYNCHROTRON21_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

static SYNCHROTRON22_DATA: [f64; 13] = [
    7.9063148270660804288,
    3.1353463612853425684,
    0.4854879477453714538,
    0.394816675827237234e-1,
    0.19661622334808802e-2,
    0.659078932293042e-4,
    0.15857561349856e-5,
    0.286865301123e-7,
    0.4041202360e-9,
    0.45568444e-11,
    0.420459e-13,
    0.3232e-15,
    0.21e-17,
];

static SYNCHROTRON22_CS: ChebSeries = ChebSeries {
    data: &SYNCHROTRON22_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

static SYNCHROTRON2A_DATA: [f64; 17] = [
    2.020337094170713600,
    0.10956237121807404e-1,
    0.8542384730114676e-3,
    0.723430242132822e-4,
    0.63124427962699e-5,
    0.5648193141174e-6,
    0.512832480138e-7,
    0.47196532914e-8,
    0.4380744214e-9,
    0.410268149e-10,
    0.38623072e-11,
    0.3661323e-12,
    0.348023e-13,
    0.33301e-14,
    0.319e-15,
    0.307e-16,
    0.3e-17,
];

static SYNCHROTRON2A_CS: ChebSeries = ChebSeries {
    data: &SYNCHROTRON2A_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let mut d = 0.0;
    let mut dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    for &c in cs.data.iter().rev() {
        let temp = d;
        d = y2 * d - dd + c;
        dd = temp;
    }

    SfResult::new(y * d - dd + 0.5 * cs.data[0], f64::EPSILON * d.abs())
}

fn pow_int(x: f64, n: i32) -> f64 {
    x.powi(n)
}

fn gsl_sf_synchrotron_1_e(x: f64) -> Result<SfResult, &'static str> {
    if x < 0.0 {
        Err("domain error")
    } else if x < 2.0 * SQRT_2 * GSL_SQRT_DBL_EPSILON {
        let z = x.powf(1.0 / 3.0);
        let cf = 1.0 - 0.843812762813205 * z * z;
        Ok(SfResult::new(
            2.14952824153447863671 * z * cf,
            GSL_DBL_EPSILON * 2.14952824153447863671 * z * cf,
        ))
    } else if x <= 4.0 {
        let c0 = PI / SQRT_3;
        let px = x.powf(1.0 / 3.0);
        let px11 = pow_int(px, 11);
        let t = x * x / 8.0 - 1.0;
        let result_c1 = cheb_eval_e(&SYNCHROTRON1_CS, t);
        let result_c2 = cheb_eval_e(&SYNCHROTRON2_CS, t);
        let val = px * result_c1.val - px11 * result_c2.val - c0 * x;
        let err = px * result_c1.err + px11 * result_c2.err + c0 * x * GSL_DBL_EPSILON;
        Ok(SfResult::new(val, err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x < -8.0 * GSL_LOG_DBL_MIN / 7.0 {
        let c0 = 0.2257913526447274323630976;
        let t = (12.0 - x) / (x + 4.0);
        let result_c1 = cheb_eval_e(&SYNCHROTRON1A_CS, t);
        let val = x.sqrt() * result_c1.val * (c0 - x).exp();
        Ok(SfResult::new(
            val,
            2.0 * GSL_DBL_EPSILON * val * ((c0 - x).abs() + 1.0),
        ))
    } else {
        Err("underflow error")
    }
}

fn gsl_sf_synchrotron_2_e(x: f64) -> Result<SfResult, &'static str> {
    if x < 0.0 {
        Err("domain error")
    } else if x < 2.0 * SQRT_2 * GSL_SQRT_DBL_EPSILON {
        let z = x.powf(1.0 / 3.0);
        let cf = 1.0 - 1.17767156510235 * z * x;
        Ok(SfResult::new(
            1.07476412076723931836 * z * cf,
            2.0 * GSL_DBL_EPSILON * 1.07476412076723931836 * z * cf,
        ))
    } else if x <= 4.0 {
        let px = x.powf(1.0 / 3.0);
        let px5 = pow_int(px, 5);
        let t = x * x / 8.0 - 1.0;
        let cheb1 = cheb_eval_e(&SYNCHROTRON21_CS, t);
        let cheb2 = cheb_eval_e(&SYNCHROTRON22_CS, t);
        let val = px * cheb1.val - px5 * cheb2.val;
        let err = px * cheb1.err + px5 * cheb2.err;
        Ok(SfResult::new(val, err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x < -8.0 * GSL_LOG_DBL_MIN / 7.0 {
        let c0 = 0.22579135264472743236;
        let t = (10.0 - x) / (x + 2.0);
        let cheb1 = cheb_eval_e(&SYNCHROTRON2A_CS, t);
        let val = x.sqrt() * (c0 - x).exp() * cheb1.val;
        Ok(SfResult::new(
            val,
            GSL_DBL_EPSILON * val * ((c0 - x).abs() + 1.0),
        ))
    } else {
        Err("underflow error")
    }
}

fn gsl_sf_synchrotron_1(x: f64) -> f64 {
    match gsl_sf_synchrotron_1_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

fn gsl_sf_synchrotron_2(x: f64) -> f64 {
    match gsl_sf_synchrotron_2_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}