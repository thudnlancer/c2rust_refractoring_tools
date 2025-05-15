use std::f64;

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

static XGK: [f64; 26] = [
    0.9992621049926098,
    0.9955569697904981,
    0.9880357945340772,
    0.9766639214595175,
    0.9616149864258425,
    0.9429745712289743,
    0.9207471152817016,
    0.8949919978782754,
    0.8658470652932756,
    0.833442628760834,
    0.7978737979985001,
    0.7592592630373576,
    0.7177664068130844,
    0.6735663684734684,
    0.6268100990103174,
    0.577662930241223,
    0.5263252843347192,
    0.473002731445715,
    0.41788538219303775,
    0.36117230580938784,
    0.30308953893110783,
    0.24386688372098843,
    0.1837189394210489,
    0.1228646926107104,
    0.06154448300568508,
    0.0,
];

static WG: [f64; 13] = [
    0.011393798501026288,
    0.026354986615032137,
    0.04093915670130631,
    0.05490469597583519,
    0.06803833381235692,
    0.08014070033500102,
    0.09102826198296365,
    0.10053594906705064,
    0.10851962447426365,
    0.11485825914571165,
    0.11945576353578477,
    0.12224244299031004,
    0.12317605372671545,
];

static WGK: [f64; 26] = [
    0.001987383892330316,
    0.005561932135356714,
    0.009473973386174152,
    0.013236229195571674,
    0.0168478177091283,
    0.020435371145882836,
    0.024009945606953216,
    0.027475317587851738,
    0.03079230016738749,
    0.03400213027432934,
    0.037116271483415544,
    0.04008382550403238,
    0.04287284502017005,
    0.04550291304992179,
    0.047982537138836714,
    0.05027767908071567,
    0.052362885806407476,
    0.05425112988854549,
    0.05595081122041231,
    0.05743711636156783,
    0.05868968002239421,
    0.05972034032417406,
    0.06053945537604586,
    0.06112850971705305,
    0.061471189871425316,
    0.061580818067832935,
];

fn gsl_integration_qk(
    n: usize,
    xgk: &[f64],
    wg: &[f64],
    wgk: &[f64],
    fv1: &mut [f64],
    fv2: &mut [f64],
    f: &GslFunction,
    a: f64,
    b: f64,
) -> (f64, f64, f64, f64) {
    let center = 0.5 * (a + b);
    let half_length = 0.5 * (b - a);
    let abs_half_length = half_length.abs();
    
    let mut result_gauss = 0.0;
    let mut result_kronrod = 0.0;
    let mut result_abs = 0.0;
    let mut result_asc = 0.0;
    
    for i in 0..(n / 2) {
        let x = half_length * xgk[2 * i + 1];
        let y = (f.function)(center + x);
        let y2 = (f.function)(center - x);
        let y_sum = y + y2;
        let y_diff = y - y2;
        
        fv1[2 * i + 1] = y;
        fv2[2 * i + 1] = y2;
        
        result_kronrod += wgk[2 * i + 1] * y_sum;
        result_abs += wgk[2 * i + 1] * (y.abs() + y2.abs());
        
        if i > 0 {
            result_gauss += wg[i] * y_sum;
        }
    }
    
    for i in 0..(n / 2) {
        let x = half_length * xgk[2 * i];
        let y = (f.function)(center + x);
        fv1[2 * i] = y;
        result_kronrod += wgk[2 * i] * y;
        result_abs += wgk[2 * i] * y.abs();
    }
    
    let mean = result_kronrod * 0.5;
    let mut asc = wgk[n - 1] * ((f.function)(center) - mean).abs();
    
    for i in 0..(n - 1) {
        asc += wgk[i] * (fv1[i] - mean).abs();
    }
    
    let result = result_kronrod * half_length;
    let abs_err = (result_kronrod - result_gauss).abs() * half_length;
    let res_abs = result_abs * abs_half_length;
    let res_asc = asc * abs_half_length;
    
    (result, abs_err, res_abs, res_asc)
}

pub fn gsl_integration_qk51(
    f: GslFunction,
    a: f64,
    b: f64,
) -> (f64, f64, f64, f64) {
    let mut fv1 = [0.0; 26];
    let mut fv2 = [0.0; 26];
    
    gsl_integration_qk(
        26,
        &XGK,
        &WG,
        &WGK,
        &mut fv1,
        &mut fv2,
        &f,
        a,
        b,
    )
}