use std::f64::consts::PI;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ChebSeries {
    c: &'static [f64],
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

impl ChebSeries {
    fn eval(&self, x: f64, result: &mut GslSfResult) -> i32 {
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
        
        result.val = d;
        result.err = 2.2204460492503131e-16 * e + self.c[self.order as usize].abs();
        0 // GSL_SUCCESS
    }
}

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

static F1_CS: ChebSeries = ChebSeries {
    c: &F1_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

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

static F2_CS: ChebSeries = ChebSeries {
    c: &F2_DATA,
    order: 28,
    a: -1.0,
    b: 1.0,
    order_sp: 14,
};

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

static G1_CS: ChebSeries = ChebSeries {
    c: &G1_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 13,
};

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

static G2_CS: ChebSeries = ChebSeries {
    c: &G2_DATA,
    order: 33,
    a: -1.0,
    b: 1.0,
    order_sp: 20,
};

fn fg_asymp(x: f64, f: &mut GslSfResult, g: &mut GslSfResult) {
    let xbig = 1.0 / 1.4901161193847656e-8;
    let xmaxf = 1.0 / 2.2250738585072014e-308;
    let xmaxg = 1.0 / 1.4916681462400413e-154;
    let xbnd = 7.07106781187;
    let x2 = x * x;
    
    if x <= xbnd {
        let mut result_c1 = GslSfResult { val: 0.0, err: 0.0 };
        let mut result_c2 = GslSfResult { val: 0.0, err: 0.0 };
        F1_CS.eval((1.0 / x2 - 0.04125) / 0.02125, &mut result_c1);
        G1_CS.eval((1.0 / x2 - 0.04125) / 0.02125, &mut result_c2);
        
        f.val = (1.0 + result_c1.val) / x;
        g.val = (1.0 + result_c2.val) / x2;
        
        f.err = result_c1.err / x + 2.0 * 2.2204460492503131e-16 * f.val.abs();
        g.err = result_c2.err / x2 + 2.0 * 2.2204460492503131e-16 * g.val.abs();
    } else if x <= xbig {
        let mut result_c1 = GslSfResult { val: 0.0, err: 0.0 };
        let mut result_c2 = GslSfResult { val: 0.0, err: 0.0 };
        F2_CS.eval(100.0 / x2 - 1.0, &mut result_c1);
        G2_CS.eval(100.0 / x2 - 1.0, &mut result_c2);
        
        f.val = (1.0 + result_c1.val) / x;
        g.val = (1.0 + result_c2.val) / x2;
        
        f.err = result_c1.err / x + 2.0 * 2.2204460492503131e-16 * f.val.abs();
        g.err = result_c2.err / x2 + 2.0 * 2.2204460492503131e-16 * g.val.abs();
    } else {
        f.val = if x < xmaxf { 1.0 / x } else { 0.0 };
        g.val = if x < xmaxg { 1.0 / x2 } else { 0.0 };
        
        f.err = 2.0 * 2.2204460492503131e-16 * f.val.abs();
        g.err = 2.0 * 2.2204460492503131e-16 * g.val.abs();
    }
}

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

static SI_CS: ChebSeries = ChebSeries {
    c: &SI_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

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

static CI_CS: ChebSeries = ChebSeries {
    c: &CI_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

pub fn gsl_sf_Si_e(x: f64, result: &mut GslSfResult) -> i32 {
    let ax = x.abs();
    
    if ax < 1.4901161193847656e-8 {
        result.val = x;
        result.err = 0.0;
        0 // GSL_SUCCESS
    } else if ax <= 4.0 {
        let mut result_c = GslSfResult { val: 0.0, err: 0.0 };
        SI_CS.eval((x * x - 8.0) * 0.125, &mut result_c);
        
        result.val = x * (0.75 + result_c.val);
        result.err = ax * result_c.err;
        result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    } else {
        let mut f = GslSfResult { val: 0.0, err: 0.0 };
        let mut g = GslSfResult { val: 0.0, err: 0.0 };
        fg_asymp(ax, &mut f, &mut g);
        
        result.val = 0.5 * PI - f.val * ax.cos() - g.val * ax.sin();
        result.err = f.err + g.err;
        result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
        
        if x < 0.0 {
            result.val = -result.val;
        }
        0 // GSL_SUCCESS
    }
}

pub fn gsl_sf_Ci_e(x: f64, result: &mut GslSfResult) -> i32 {
    if x <= 0.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        1 // GSL_EDOM
    } else if x <= 4.0 {
        let lx = x.ln();
        let y = (x * x - 8.0) * 0.125;
        let mut result_c = GslSfResult { val: 0.0, err: 0.0 };
        CI_CS.eval(y, &mut result_c);
        
        result.val = lx - 0.5 + result_c.val;
        result.err = 2.0 * 2.2204460492503131e-16 * (lx.abs() + 0.5) + result_c.err;
        result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    } else {
        let sin_x = x.sin();
        let cos_x = x.cos();
        
        let mut f = GslSfResult { val: 0.0, err: 0.0 };
        let mut g = GslSfResult { val: 0.0, err: 0.0 };
        fg_asymp(x, &mut f, &mut g);
        
        result.val = f.val * sin_x - g.val * cos_x;
        result.err = (f.err * sin_x).abs();
        result.err += (g.err * cos_x).abs();
        result.err += (f.val * (2.2204460492503131e-16 * sin_x.abs())).abs();
        result.err += (g.val * (2.2204460492503131e-16 * cos_x.abs())).abs();
        result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    }
}

pub fn gsl_sf_Si(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    gsl_sf_Si_e(x, &mut result);
    result.val
}

pub fn gsl_sf_Ci(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    gsl_sf_Ci_e(x, &mut result);
    result.val
}