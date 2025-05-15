use std::f64::consts::{PI, SQRT_2, FRAC_1_SQRT_2, SQRT_PI};
use std::f64;

const LOG_ROOT_PI: f64 = 0.57236494292470008706;
const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_ROOT6_DBL_EPSILON: f64 = f64::EPSILON.powf(1.0 / 6.0);

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut SfResult) {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut temp = 0.0;
    for j in (1..=cs.order).rev() {
        temp = d;
        d = y2 * d - dd + cs.data[j];
        dd = temp;
    }
    
    result.val = y * d - dd + 0.5 * cs.data[0];
    result.err = f64::EPSILON * f64::abs(result.val);
}

fn erfc8_sum(x: f64) -> f64 {
    static P: [f64; 6] = [
        2.97886562639399288862,
        7.409740605964741794425,
        6.1602098531096305440906,
        5.019049726784267463450058,
        1.275366644729965952479585264,
        0.5641895835477550741253201704,
    ];
    static Q: [f64; 7] = [
        3.3690752069827527677,
        9.608965327192787870698,
        17.08144074746600431571095,
        12.0489519278551290360340491,
        9.396034016235054150430579648,
        2.260528520767326969591866945,
        1.0,
    ];
    
    let num = P[5] + x * (P[4] + x * (P[3] + x * (P[2] + x * (P[1] + x * P[0]))));
    let den = Q[6] + x * (Q[5] + x * (Q[4] + x * (Q[3] + x * (Q[2] + x * (Q[1] + x * Q[0])))));
    
    num / den
}

fn erfc8(x: f64) -> f64 {
    let e = erfc8_sum(x);
    e * (-x * x).exp()
}

fn log_erfc8(x: f64) -> f64 {
    let e = erfc8_sum(x);
    e.ln() - x * x
}

fn erfseries(x: f64) -> SfResult {
    let mut coef = x;
    let mut e = coef;
    let mut del = 0.0;
    
    for k in 1..30 {
        coef *= -x * x / k as f64;
        del = coef / (2.0 * k as f64 + 1.0);
        e += del;
    }
    
    SfResult::new(2.0 / SQRT_PI * e, 2.0 / SQRT_PI * (del.abs() + GSL_DBL_EPSILON))
}

static ERFC_XLT1_DATA: [f64; 20] = [
    1.06073416421769980345174155056,
    -0.42582445804381043569204735291,
    0.04955262679620434040357683080,
    0.00449293488768382749558001242,
    -0.00129194104658496953494224761,
    -0.00001836389292149396270416979,
    0.00002211114704099526291538556,
    -5.23337485234257134673693179020e-7,
    -2.78184788833537885382530989578e-7,
    1.41158092748813114560316684249e-8,
    2.72571296330561699984539141865e-9,
    -2.06343904872070629406401492476e-10,
    -2.14273991996785367924201401812e-11,
    2.22990255539358204580285098119e-12,
    1.36250074650698280575807934155e-13,
    -1.95144010922293091898995913038e-14,
    -6.85627169231704599442806370690e-16,
    1.44506492869699938239521607493e-16,
    2.45935306460536488037576200030e-18,
    -9.29599561220523396007359328540e-19,
];

static ERFC_X15_DATA: [f64; 25] = [
    0.44045832024338111077637466616,
    -0.143958836762168335790826895326,
    0.044786499817939267247056666937,
    -0.013343124200271211203618353102,
    0.003824682739750469767692372556,
    -0.001058699227195126547306482530,
    0.000283859419210073742736310108,
    -0.000073906170662206760483959432,
    0.000018725312521489179015872934,
    -4.62530981164919445131297264430e-6,
    1.11558657244432857487884006422e-6,
    -2.63098662650834130067808832725e-7,
    6.07462122724551777372119408710e-8,
    -1.37460865539865444777251011793e-8,
    3.05157051905475145520096717210e-9,
    -6.65174789720310713757307724790e-10,
    1.42483346273207784489792999706e-10,
    -3.00141127395323902092018744545e-11,
    6.22171792645348091472914001250e-12,
    -1.26994639225668496876152836555e-12,
    2.55385883033257575402681845385e-13,
    -5.06258237507038698392265499770e-14,
    9.89705409478327321641264227110e-15,
    -1.90685978789192181051961024995e-15,
    3.50826648032737849245113757340e-16,
];

static ERFC_X510_DATA: [f64; 20] = [
    1.11684990123545698684297865808,
    0.003736240359381998520654927536,
    -0.000916623948045470238763619870,
    0.000199094325044940833965078819,
    -0.000040276384918650072591781859,
    7.76515264697061049477127605790e-6,
    -1.44464794206689070402099225301e-6,
    2.61311930343463958393485241947e-7,
    -4.61833026634844152345304095560e-8,
    8.00253111512943601598732144340e-9,
    -1.36291114862793031395712122089e-9,
    2.28570483090160869607683087722e-10,
    -3.78022521563251805044056974560e-11,
    6.17253683874528285729910462130e-12,
    -9.96019290955316888445830597430e-13,
    1.58953143706980770269506726000e-13,
    -2.51045971047162509999527428316e-14,
    3.92607828989125810013581287560e-15,
    -6.07970619384160374392535453420e-16,
    9.12600607264794717315507477670e-17,
];

static ERFC_XLT1_CS: ChebSeries = ChebSeries {
    data: &ERFC_XLT1_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

static ERFC_X15_CS: ChebSeries = ChebSeries {
    data: &ERFC_X15_DATA,
    order: 24,
    a: -1.0,
    b: 1.0,
    order_sp: 16,
};

static ERFC_X510_CS: ChebSeries = ChebSeries {
    data: &ERFC_X510_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

pub fn gsl_sf_erfc_e(x: f64) -> SfResult {
    let ax = x.abs();
    let (e_val, e_err) = if ax <= 1.0 {
        let t = 2.0 * ax - 1.0;
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&ERFC_XLT1_CS, t, &mut c);
        (c.val, c.err)
    } else if ax <= 5.0 {
        let ex2 = (-x * x).exp();
        let t = 0.5 * (ax - 3.0);
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&ERFC_X15_CS, t, &mut c);
        (ex2 * c.val, ex2 * (c.err + 2.0 * ax * GSL_DBL_EPSILON))
    } else if ax < 10.0 {
        let exterm = (-x * x).exp() / ax;
        let t = (2.0 * ax - 15.0) / 5.0;
        let mut c = SfResult::new(0.0, 0.0);
        cheb_eval_e(&ERFC_X510_CS, t, &mut c);
        (exterm * c.val, exterm * (c.err + 2.0 * ax * GSL_DBL_EPSILON + GSL_DBL_EPSILON))
    } else {
        let val = erfc8(ax);
        (val, (x * x + 1.0) * GSL_DBL_EPSILON * val.abs())
    };

    if x < 0.0 {
        SfResult::new(2.0 - e_val, e_err + 2.0 * GSL_DBL_EPSILON * (2.0 - e_val).abs())
    } else {
        SfResult::new(e_val, e_err + 2.0 * GSL_DBL_EPSILON * e_val.abs())
    }
}

pub fn gsl_sf_log_erfc_e(x: f64) -> SfResult {
    if x * x < 10.0 * GSL_ROOT6_DBL_EPSILON {
        let y = x / SQRT_PI;
        let c3 = (4.0 - PI) / 3.0;
        let c4 = 2.0 * (1.0 - PI / 3.0);
        let c5 = -0.001829764677455021;
        let c6 = 0.02629651521057465;
        let c7 = -0.01621575378835404;
        let c8 = 0.00125993961762116;
        let c9 = 0.00556964649138;
        let c10 = -0.0045563339802;
        let c11 = 0.0009461589032;
        let c12 = 0.0013200243174;
        let c13 = -0.00142906;
        let c14 = 0.00048204;
        
        let series = c8 + y * (c9 + y * (c10 + y * (c11 + y * (c12 + y * (c13 + c14 * y)))));
        let series = y * (1.0 + y * (1.0 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * (c7 + y * series)))))));
        
        SfResult::new(-2.0 * series, 2.0 * GSL_DBL_EPSILON * (-2.0 * series).abs())
    } else if x > 8.0 {
        let val = log_erfc8(x);
        SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs())
    } else {
        let result_erfc = gsl_sf_erfc_e(x);
        let val = result_erfc.val.ln();
        SfResult::new(
            val,
            (result_erfc.err / result_erfc.val).abs() + 2.0 * GSL_DBL_EPSILON * val.abs(),
        )
    }
}

pub fn gsl_sf_erf_e(x: f64) -> SfResult {
    if x.abs() < 1.0 {
        erfseries(x)
    } else {
        let result_erfc = gsl_sf_erfc_e(x);
        SfResult::new(
            1.0 - result_erfc.val,
            result_erfc.err + 2.0 * GSL_DBL_EPSILON * (1.0 - result_erfc.val).abs(),
        )
    }
}

pub fn gsl_sf_erf_Z_e(x: f64) -> SfResult {
    let ex2 = (-x * x / 2.0).exp();
    let val = ex2 / (SQRT_2 * SQRT_PI);
    SfResult::new(
        val,
        x.abs() * val * GSL_DBL_EPSILON + 2.0 * GSL_DBL_EPSILON * val.abs(),
    )
}

pub fn gsl_sf_erf_Q_e(x: f64) -> SfResult {
    let result_erfc = gsl_sf_erfc_e(x * FRAC_1_SQRT_2);
    SfResult::new(
        0.5 * result_erfc.val,
        0.5 * result_erfc.err + 2.0 * GSL_DBL_EPSILON * (0.5 * result_erfc.val).abs(),
    )
}

pub fn gsl_sf_hazard_e(x: f64) -> SfResult {
    if x < 25.0 {
        let result_ln_erfc = gsl_sf_log_erfc_e(x * FRAC_1_SQRT_2);
        let lnc = -0.22579135264472743236;
        let arg = lnc - 0.5 * x * x - result_ln_erfc.val;
        let mut result = SfResult::new(0.0, 0.0);
        result.val = arg.exp();
        result.err = 3.0 * (1.0 + x.abs()) * GSL_DBL_EPSILON * result.val.abs();
        result.err += result_ln_erfc.err.abs() * result.val;
        result
    } else {
        let ix2 = 1.0 / (x * x);
        let corr_b = 1.0 - 9.0 * ix2 * (1.0 - 11.0 * ix2);
        let corr_m = 1.0 - 5.0 * ix2 * (1.0 - 7.0 * ix2 * corr_b);
        let corr_t = 1.0 - ix2 * (1.0 - 3.0 * ix2 * corr_m);
        let val = x / corr_t;
        SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs())
    }
}

pub fn gsl_sf_erfc(x: f64) -> f64 {
    gsl_sf_erfc_e(x).val
}

pub fn gsl_sf_log_erfc(x: f64) -> f64 {
    gsl_sf_log_erfc_e(x).val
}

pub fn gsl_sf_erf(x: f64) -> f64 {
    gsl_sf_erf_e(x).val
}

pub fn gsl_sf_erf_Z(x: f64) -> f64 {
    gsl_sf_erf_Z_e(x).val
}

pub fn gsl_sf_erf_Q(x: f64) -> f64 {
    gsl_sf_erf_Q_e(x).val
}

pub fn gsl_sf_hazard(x: f64) -> f64 {
    gsl_sf_hazard_e(x).val
}