use std::f64::consts::{FRAC_PI_4, PI};
use std::f64::EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Precise,
    Fast,
}

#[derive(Debug, Clone)]
struct ChebSeries {
    data: Vec<f64>,
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

impl ChebSeries {
    fn new(data: &[f64], order: usize, a: f64, b: f64, order_sp: usize) -> Self {
        ChebSeries {
            data: data.to_vec(),
            order,
            a,
            b,
            order_sp,
        }
    }
}

fn cheb_eval_mode_e(cs: &ChebSeries, x: f64, mode: Mode, result: &mut SfResult) {
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
    
    result.val = d;
    result.err = EPSILON * e + (EPSILON * (cs.order + 1)).abs() * d.abs();
}

fn airy_mod_phase(x: f64, mode: Mode, mod_: &mut SfResult, phase: &mut SfResult) -> Result<(), &'static str> {
    let mut result_m = SfResult { val: 0.0, err: 0.0 };
    let mut result_p = SfResult { val: 0.0, err: 0.0 };
    let m: f64;
    let p: f64;
    let sqx: f64;

    if x < -2.0 {
        let z = 16.0 / (x * x * x) + 1.0;
        cheb_eval_mode_e(&AM21_CS, z, mode, &mut result_m);
        cheb_eval_mode_e(&ATH1_CS, z, mode, &mut result_p);
    } else if x <= -1.0 {
        let z = (16.0 / (x * x * x) + 9.0) / 7.0;
        cheb_eval_mode_e(&AM22_CS, z, mode, &mut result_m);
        cheb_eval_mode_e(&ATH2_CS, z, mode, &mut result_p);
    } else {
        mod_.val = 0.0;
        mod_.err = 0.0;
        phase.val = 0.0;
        phase.err = 0.0;
        return Err("x is greater than 1.0");
    }

    m = 0.3125 + result_m.val;
    p = -0.625 + result_p.val;

    sqx = (-x).sqrt();

    mod_.val = (m / sqx).sqrt();
    mod_.err = mod_.val.abs() * (EPSILON + (result_m.err / result_m.val).abs());
    phase.val = FRAC_PI_4 - x * sqx * p;
    phase.err = phase.val.abs() * (EPSILON + (result_p.err / result_p.val).abs());

    Ok(())
}

fn airy_aie(x: f64, mode: Mode, result: &mut SfResult) -> Result<(), &'static str> {
    let sqx = x.sqrt();
    let z = 2.0 / (x * sqx) - 1.0;
    let y = sqx.sqrt();
    let mut result_c = SfResult { val: 0.0, err: 0.0 };
    
    cheb_eval_mode_e(&AIP_CS, z, mode, &mut result_c);
    result.val = (0.28125 + result_c.val) / y;
    result.err = result_c.err / y + EPSILON * result.val.abs();
    
    Ok(())
}

fn airy_bie(x: f64, mode: Mode, result: &mut SfResult) -> Result<(), &'static str> {
    const ATR: f64 = 8.7506905708484345;
    const BTR: f64 = -2.0938363213560543;

    if x < 4.0 {
        let sqx = x.sqrt();
        let z = ATR / (x * sqx) + BTR;
        let y = sqx.sqrt();
        let mut result_c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_mode_e(&BIP_CS, z, mode, &mut result_c);
        result.val = (0.625 + result_c.val) / y;
        result.err = result_c.err / y + EPSILON * result.val.abs();
    } else {
        let sqx = x.sqrt();
        let z = 16.0 / (x * sqx) - 1.0;
        let y = sqx.sqrt();
        let mut result_c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_mode_e(&BIP2_CS, z, mode, &mut result_c);
        result.val = (0.625 + result_c.val) / y;
        result.err = result_c.err / y + EPSILON * result.val.abs();
    }

    Ok(())
}

pub fn gsl_sf_airy_Ai_e(x: f64, mode: Mode, result: &mut SfResult) -> Result<(), &'static str> {
    if x < -1.0 {
        let mut mod_ = SfResult { val: 0.0, err: 0.0 };
        let mut theta = SfResult { val: 0.0, err: 0.0 };
        let mut cos_result = SfResult { val: 0.0, err: 0.0 };
        
        airy_mod_phase(x, mode, &mut mod_, &mut theta)?;
        cos_result.val = theta.val.cos();
        cos_result.err = theta.err * theta.val.sin().abs();
        
        result.val = mod_.val * cos_result.val;
        result.err = (mod_.val * cos_result.err).abs() + (cos_result.val * mod_.err).abs();
        result.err += EPSILON * result.val.abs();
        Ok(())
    } else if x <= 1.0 {
        let z = x * x * x;
        let mut result_c0 = SfResult { val: 0.0, err: 0.0 };
        let mut result_c1 = SfResult { val: 0.0, err: 0.0 };
        
        cheb_eval_mode_e(&AIF_CS, z, mode, &mut result_c0);
        cheb_eval_mode_e(&AIG_CS, z, mode, &mut result_c1);
        
        result.val = 0.375 + (result_c0.val - x * (0.25 + result_c1.val));
        result.err = result_c0.err + (x * result_c1.err).abs();
        result.err += EPSILON * result.val.abs();
        Ok(())
    } else {
        let x32 = x * x.sqrt();
        let s = (-2.0 * x32 / 3.0).exp();
        let mut result_aie = SfResult { val: 0.0, err: 0.0 };
        
        airy_aie(x, mode, &mut result_aie)?;
        result.val = result_aie.val * s;
        result.err = result_aie.err * s + result.val * x32 * EPSILON;
        result.err += EPSILON * result.val.abs();
        Ok(())
    }
}

// ... (其他函数类似实现)

// 定义所有ChebSeries常量
lazy_static! {
    static ref AM21_CS: ChebSeries = ChebSeries::new(&AM21_DATA, 36, -1.0, 1.0, 20);
    static ref ATH1_CS: ChebSeries = ChebSeries::new(&ATH1_DATA, 35, -1.0, 1.0, 15);
    static ref AM22_CS: ChebSeries = ChebSeries::new(&AM22_DATA, 32, -1.0, 1.0, 15);
    static ref ATH2_CS: ChebSeries = ChebSeries::new(&ATH2_DATA, 31, -1.0, 1.0, 16);
    static ref AIF_CS: ChebSeries = ChebSeries::new(&AI_DATA_F, 8, -1.0, 1.0, 8);
    static ref AIG_CS: ChebSeries = ChebSeries::new(&AI_DATA_G, 7, -1.0, 1.0, 7);
    static ref BIF_CS: ChebSeries = ChebSeries::new(&DATA_BIF, 8, -1.0, 1.0, 8);
    static ref BIG_CS: ChebSeries = ChebSeries::new(&DATA_BIG, 7, -1.0, 1.0, 7);
    static ref BIF2_CS: ChebSeries = ChebSeries::new(&DATA_BIF2, 9, -1.0, 1.0, 9);
    static ref BIG2_CS: ChebSeries = ChebSeries::new(&DATA_BIG2, 9, -1.0, 1.0, 9);
    static ref AIP_CS: ChebSeries = ChebSeries::new(&DATA_AIP, 35, -1.0, 1.0, 17);
    static ref BIP_CS: ChebSeries = ChebSeries::new(&DATA_BIP, 23, -1.0, 1.0, 14);
    static ref BIP2_CS: ChebSeries = ChebSeries::new(&DATA_BIP2, 28, -1.0, 1.0, 10);
}

// 定义所有数据数组
const AM21_DATA: [f64; 37] = [ /* ... */ ];
const ATH1_DATA: [f64; 36] = [ /* ... */ ];
const AM22_DATA: [f64; 33] = [ /* ... */ ];
const ATH2_DATA: [f64; 32] = [ /* ... */ ];
const AI_DATA_F: [f64; 9] = [ /* ... */ ];
const AI_DATA_G: [f64; 8] = [ /* ... */ ];
const DATA_BIF: [f64; 9] = [ /* ... */ ];
const DATA_BIG: [f64; 8] = [ /* ... */ ];
const DATA_BIF2: [f64; 10] = [ /* ... */ ];
const DATA_BIG2: [f64; 10] = [ /* ... */ ];
const DATA_AIP: [f64; 36] = [ /* ... */ ];
const DATA_BIP: [f64; 24] = [ /* ... */ ];
const DATA_BIP2: [f64; 29] = [ /* ... */ ];