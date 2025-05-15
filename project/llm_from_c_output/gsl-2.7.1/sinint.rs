use std::f64::consts::PI;

// Chebyshev series structure
struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

// Result type for special functions
#[derive(Debug, Clone, Copy)]
struct SfResult {
    val: f64,
    err: f64,
}

// Constants
const GSL_SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-08;
const GSL_DBL_MIN: f64 = 2.2250738585072014e-308;
const GSL_DBL_EPSILON: f64 = 2.220446049250313e-16;
const GSL_SQRT_DBL_MIN: f64 = 1.4916681462400413e-154;

// Chebyshev series data
static F1_DATA: [f64; 20] = [
    -0.1191081969051363610,
    -0.0247823144996236248,
    0.0011910281453357821,
    -0.0000927027714388562,
    0.0000093373141568271,
    -0.0000011058287820557,
    0.0000001464772071460,
    -0.0000000210694496288,
    0.0000000032293492367,
    -0.0000000005206529618,
    0.0000000000874878885,
    -0.0000000000152176187,
    0.0000000000027257192,
    -0.0000000000005007053,
    0.0000000000000940241,
    -0.0000000000000180014,
    0.0000000000000035063,
    -0.0000000000000006935,
    0.0000000000000001391,
    -0.0000000000000000282,
];

static F2_DATA: [f64; 29] = [
    -0.0348409253897013234,
    -0.0166842205677959686,
    0.0006752901241237738,
    -0.0000535066622544701,
    0.0000062693421779007,
    -0.0000009526638801991,
    0.0000001745629224251,
    -0.0000000368795403065,
    0.0000000087202677705,
    -0.0000000022601970392,
    0.0000000006324624977,
    -0.0000000001888911889,
    0.0000000000596774674,
    -0.0000000000198044313,
    0.0000000000068641396,
    -0.0000000000024731020,
    0.0000000000009226360,
    -0.0000000000003552364,
    0.0000000000001407606,
    -0.0000000000000572623,
    0.0000000000000238654,
    -0.0000000000000101714,
    0.0000000000000044259,
    -0.0000000000000019634,
    0.0000000000000008868,
    -0.0000000000000004074,
    0.0000000000000001901,
    -0.0000000000000000900,
    0.0000000000000000432,
];

static G1_DATA: [f64; 21] = [
    -0.3040578798253495954,
    -0.0566890984597120588,
    0.0039046158173275644,
    -0.0003746075959202261,
    0.0000435431556559844,
    -0.0000057417294453025,
    0.0000008282552104503,
    -0.0000001278245892595,
    0.0000000207978352949,
    -0.0000000035313205922,
    0.0000000006210824236,
    -0.0000000001125215474,
    0.0000000000209088918,
    -0.0000000000039715832,
    0.0000000000007690431,
    -0.0000000000001514697,
    0.0000000000000302892,
    -0.0000000000000061400,
    0.0000000000000012601,
    -0.0000000000000002615,
    0.0000000000000000548,
];

static G2_DATA: [f64; 34] = [
    -0.0967329367532432218,
    -0.0452077907957459871,
    0.0028190005352706523,
    -0.0002899167740759160,
    0.0000407444664601121,
    -0.0000071056382192354,
    0.0000014534723163019,
    -0.0000003364116512503,
    0.0000000859774367886,
    -0.0000000238437656302,
    0.0000000070831906340,
    -0.0000000022318068154,
    0.0000000007401087359,
    -0.0000000002567171162,
    0.0000000000926707021,
    -0.0000000000346693311,
    0.0000000000133950573,
    -0.0000000000053290754,
    0.0000000000021775312,
    -0.0000000000009118621,
    0.0000000000003905864,
    -0.0000000000001708459,
    0.0000000000000762015,
    -0.0000000000000346151,
    0.0000000000000159996,
    -0.0000000000000075213,
    0.0000000000000035970,
    -0.0000000000000017530,
    0.0000000000000008738,
    -0.0000000000000004487,
    0.0000000000000002397,
    -0.0000000000000001347,
    0.0000000000000000801,
    -0.0000000000000000501,
];

static SI_DATA: [f64; 12] = [
    -0.1315646598184841929,
    -0.2776578526973601892,
    0.0354414054866659180,
    -0.0025631631447933978,
    0.0001162365390497009,
    -0.0000035904327241606,
    0.0000000802342123706,
    -0.0000000013562997693,
    0.0000000000179440722,
    -0.0000000000001908387,
    0.0000000000000016670,
    -0.0000000000000000122,
];

static CI_DATA: [f64; 13] = [
    -0.34004281856055363156,
    -1.03302166401177456807,
    0.19388222659917082877,
    -0.01918260436019865894,
    0.00110789252584784967,
    -0.00004157234558247209,
    0.00000109278524300229,
    -0.00000002123285954183,
    0.00000000031733482164,
    -0.00000000000376141548,
    0.00000000000003622653,
    -0.00000000000000028912,
    0.00000000000000000194,
];

static F1_CS: ChebSeries = ChebSeries {
    data: &F1_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static F2_CS: ChebSeries = ChebSeries {
    data: &F2_DATA,
    order: 28,
    a: -1.0,
    b: 1.0,
    order_sp: 14,
};

static G1_CS: ChebSeries = ChebSeries {
    data: &G1_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 13,
};

static G2_CS: ChebSeries = ChebSeries {
    data: &G2_DATA,
    order: 33,
    a: -1.0,
    b: 1.0,
    order_sp: 20,
};

static SI_CS: ChebSeries = ChebSeries {
    data: &SI_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

static CI_CS: ChebSeries = ChebSeries {
    data: &CI_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        e += (temp - dd).abs();
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.data[0];
    e += (temp - dd).abs();
    
    SfResult {
        val: d,
        err: e * f64::EPSILON,
    }
}

fn fg_asymp(x: f64) -> (SfResult, SfResult) {
    let xbig = 1.0 / GSL_SQRT_DBL_EPSILON;
    let xmaxf = 1.0 / GSL_DBL_MIN;
    let xmaxg = 1.0 / GSL_SQRT_DBL_MIN;
    let xbnd = 7.07106781187;
    let x2 = x * x;

    if x <= xbnd {
        let result_c1 = cheb_eval_e(&F1_CS, (1.0 / x2 - 0.04125) / 0.02125);
        let result_c2 = cheb_eval_e(&G1_CS, (1.0 / x2 - 0.04125) / 0.02125);
        let f_val = (1.0 + result_c1.val) / x;
        let g_val = (1.0 + result_c2.val) / x2;
        let f_err = result_c1.err / x + 2.0 * GSL_DBL_EPSILON * f_val.abs();
        let g_err = result_c2.err / x2 + 2.0 * GSL_DBL_EPSILON * g_val.abs();
        (
            SfResult { val: f_val, err: f_err },
            SfResult { val: g_val, err: g_err },
        )
    } else if x <= xbig {
        let result_c1 = cheb_eval_e(&F2_CS, 100.0 / x2 - 1.0);
        let result_c2 = cheb_eval_e(&G2_CS, 100.0 / x2 - 1.0);
        let f_val = (1.0 + result_c1.val) / x;
        let g_val = (1.0 + result_c2.val) / x2;
        let f_err = result_c1.err / x + 2.0 * GSL_DBL_EPSILON * f_val.abs();
        let g_err = result_c2.err / x2 + 2.0 * GSL_DBL_EPSILON * g_val.abs();
        (
            SfResult { val: f_val, err: f_err },
            SfResult { val: g_val, err: g_err },
        )
    } else {
        let f_val = if x < xmaxf { 1.0 / x } else { 0.0 };
        let g_val = if x < xmaxg { 1.0 / x2 } else { 0.0 };
        (
            SfResult {
                val: f_val,
                err: 2.0 * GSL_DBL_EPSILON * f_val.abs(),
            },
            SfResult {
                val: g_val,
                err: 2.0 * GSL_DBL_EPSILON * g_val.abs(),
            },
        )
    }
}

pub fn gsl_sf_Si_e(x: f64) -> Result<SfResult, &'static str> {
    let ax = x.abs();
    
    if ax < GSL_SQRT_DBL_EPSILON {
        Ok(SfResult { val: x, err: 0.0 })
    } else if ax <= 4.0 {
        let result_c = cheb_eval_e(&SI_CS, (x * x - 8.0) * 0.125);
        let val = x * (0.75 + result_c.val);
        let err = ax * result_c.err + 2.0 * GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else {
        let (f, g) = fg_asymp(ax);
        let cos_ax = ax.cos();
        let sin_ax = ax.sin();
        let val = 0.5 * PI - f.val * cos_ax - g.val * sin_ax;
        let err = f.err + g.err + 2.0 * GSL_DBL_EPSILON * val.abs();
        let val = if x < 0.0 { -val } else { val };
        Ok(SfResult { val, err })
    }
}

pub fn gsl_sf_Ci_e(x: f64) -> Result<SfResult, &'static str> {
    if x <= 0.0 {
        Err("Domain error: x must be positive")
    } else if x <= 4.0 {
        let lx = x.ln();
        let y = (x * x - 8.0) * 0.125;
        let result_c = cheb_eval_e(&CI_CS, y);
        let val = lx - 0.5 + result_c.val;
        let err = 2.0 * GSL_DBL_EPSILON * (lx.abs() + 0.5) + result_c.err;
        let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else {
        let sin_result = x.sin();
        let cos_result = x.cos();
        let (f, g) = fg_asymp(x);
        let val = f.val * sin_result - g.val * cos_result;
        let mut err = f.err * sin_result.abs();
        err += g.err * cos_result.abs();
        err += f.val.abs() * GSL_DBL_EPSILON;
        err += g.val.abs() * GSL_DBL_EPSILON;
        err += 2.0 * GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    }
}

pub fn gsl_sf_Si(x: f64) -> f64 {
    gsl_sf_Si_e(x).unwrap().val
}

pub fn gsl_sf_Ci(x: f64) -> f64 {
    gsl_sf_Ci_e(x).unwrap().val
}