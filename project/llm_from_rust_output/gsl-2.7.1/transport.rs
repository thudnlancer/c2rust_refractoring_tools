use std::f64::consts;
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

const GSL_SUCCESS: i32 = 0;
const GSL_EDOM: i32 = 1;
const GSL_EUNDRFLW: i32 = 15;

const TRANSPORT2_DATA: [f64; 18] = [
    1.671760446434538503,
    -0.147735359946794490,
    0.148213819946936338e-01,
    -0.14195330326305613e-02,
    0.1306541324415708e-03,
    -0.117155795867579e-04,
    0.10333498445756e-05,
    -0.901911304223e-07,
    0.78177169833e-08,
    -0.6744565684e-09,
    0.579946394e-10,
    -0.49747619e-11,
    0.425961e-12,
    -0.36422e-13,
    0.3111e-14,
    -0.265e-15,
    0.23e-16,
    -0.19e-17,
];

const TRANSPORT2_CS: ChebSeries = ChebSeries {
    c: &TRANSPORT2_DATA,
    order: 17,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const TRANSPORT3_DATA: [f64; 18] = [
    0.762012543243872007,
    -0.105674387705058533,
    0.119778084819657810e-01,
    -0.12144015203698307e-02,
    0.1155099769392855e-03,
    -0.105815992124423e-04,
    0.9474663385302e-06,
    -0.836221212858e-07,
    0.73109099278e-08,
    -0.6350594779e-09,
    0.549118282e-10,
    -0.47321395e-11,
    0.4067695e-12,
    -0.348971e-13,
    0.29892e-14,
    -0.256e-15,
    0.219e-16,
    -0.19e-17,
];

const TRANSPORT3_CS: ChebSeries = ChebSeries {
    c: &TRANSPORT3_DATA,
    order: 17,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const TRANSPORT4_DATA: [f64; 18] = [
    0.4807570994615110579,
    -0.8175378810321083956e-01,
    0.1002700665975162973e-01,
    -0.10599339359820151e-02,
    0.1034506245030405e-03,
    -0.96442705485899e-05,
    0.8745544408515e-06,
    -0.779321207981e-07,
    0.68649886141e-08,
    -0.5999571076e-09,
    0.521366241e-10,
    -0.45118382e-11,
    0.3892159e-12,
    -0.334936e-13,
    0.28767e-14,
    -0.2467e-15,
    0.211e-16,
    -0.18e-17,
];

const TRANSPORT4_CS: ChebSeries = ChebSeries {
    c: &TRANSPORT4_DATA,
    order: 17,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const TRANSPORT5_DATA: [f64; 18] = [
    0.347777777133910789,
    -0.66456988976050428e-01,
    0.8611072656883309e-02,
    -0.9396682223755538e-03,
    0.936324806081513e-04,
    -0.88571319340833e-05,
    0.811914989145e-06,
    -0.72957654233e-07,
    0.646971455e-08,
    -0.568490283e-09,
    0.49625598e-10,
    -0.4310940e-11,
    0.373100e-12,
    -0.32198e-13,
    0.2772e-14,
    -0.238e-15,
    0.21e-16,
    -0.18e-17,
];

const TRANSPORT5_CS: ChebSeries = ChebSeries {
    c: &TRANSPORT5_DATA,
    order: 17,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> GslSfResult {
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
    
    GslSfResult {
        val: d,
        err: f64::EPSILON * e + cs.c[cs.order as usize].abs(),
    }
}

fn transport_sumexp(numexp: i32, order: i32, t: f64, x: f64) -> f64 {
    let mut rk = numexp as f64;
    let mut sumexp = 0.0;
    
    for _ in 1..=numexp {
        let mut sum2 = 1.0;
        let xk = 1.0 / (rk * x);
        let mut xk1 = 1.0;
        
        for _ in 1..=order {
            sum2 = sum2 * xk1 * xk + 1.0;
            xk1 += 1.0;
        }
        
        sumexp *= t;
        sumexp += sum2;
        rk -= 1.0;
    }
    
    sumexp
}

pub fn gsl_sf_transport_2_e(x: f64) -> Result<GslSfResult, i32> {
    const VAL_INFINITY: f64 = 3.289868133696452873;
    
    if x < 0.0 {
        Err(GSL_EDOM)
    } else if x < 3.0 * f64::EPSILON {
        Ok(GslSfResult {
            val: x,
            err: f64::EPSILON * x.abs() + x * x / 2.0,
        })
    } else if x <= 4.0 {
        let t = x * x / 8.0 - 0.5 - 0.5;
        let result_c = cheb_eval_e(&TRANSPORT2_CS, t);
        let val = x * result_c.val;
        Ok(GslSfResult {
            val,
            err: x * result_c.err + 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let t = 2.0 * x.ln() - x;
        if t < -f64::MAX_EXP {
            Ok(GslSfResult {
                val: VAL_INFINITY,
                err: 2.0 * f64::EPSILON * VAL_INFINITY,
            })
        } else {
            let et = t.exp();
            Ok(GslSfResult {
                val: VAL_INFINITY - et,
                err: 2.0 * f64::EPSILON * (VAL_INFINITY + (t.abs() + 1.0) * et),
            })
        }
    }
}

pub fn gsl_sf_transport_2(x: f64) -> f64 {
    match gsl_sf_transport_2_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

// Similar implementations for transport_3, transport_4, transport_5
// would follow the same pattern as transport_2