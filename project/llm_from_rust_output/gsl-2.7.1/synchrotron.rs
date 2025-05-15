use std::f64::consts::{PI, SQRT_3};
use std::f64::{NAN, EPSILON, MIN_POSITIVE};

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

const SYNCHROTRON1_DATA: [f64; 13] = [
    30.364682982501076273,
    17.079395277408394574,
    4.560132133545072889,
    0.549281246730419979,
    0.0372976075069301172,
    0.00161362430201041242,
    0.00000481916772120371,
    0.000000010512425288938,
    0.0000000000174638504670,
    0.000000000000022815486544,
    0.0000000000000000240443082,
    0.00000000000000000002086588,
    0.00000000000000000000015167,
];

const SYNCHROTRON1_CS: ChebSeries = ChebSeries {
    c: &SYNCHROTRON1_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const SYNCHROTRON2_DATA: [f64; 12] = [
    0.4490721623532660844,
    0.0898353677994187218,
    0.0081044573772151290,
    0.0004261716991089162,
    0.0000147609631270746,
    0.0000003628633615300,
    0.0000000066634807498,
    0.0000000000949077166,
    0.000000000001079125,
    0.000000000000010022,
    0.000000000000000077,
    0.0000000000000000005,
];

const SYNCHROTRON2_CS: ChebSeries = ChebSeries {
    c: &SYNCHROTRON2_DATA,
    order: 11,
    a: -1.0,
    b: 1.0,
    order_sp: 7,
};

const SYNCHROTRON1A_DATA: [f64; 23] = [
    2.1329305161355000985,
    0.0741352864954200240,
    0.0086968099909964198,
    0.0011703826248775692,
    0.0001645105798619192,
    0.0000240201021420640,
    0.0000035827756389389,
    0.0000005447747626984,
    0.0000000838802856196,
    0.000000013069882684,
    0.000000002053099071,
    0.000000000325187537,
    0.0000000000517914041,
    0.0000000000083002988,
    0.0000000000013352728,
    0.0000000000002159150,
    0.0000000000000349967,
    0.0000000000000056994,
    0.0000000000000009291,
    0.000000000000000152,
    0.0000000000000000249,
    0.0000000000000000041,
    0.0000000000000000007,
];

const SYNCHROTRON1A_CS: ChebSeries = ChebSeries {
    c: &SYNCHROTRON1A_DATA,
    order: 22,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

const SYNCHROTRON21_DATA: [f64; 13] = [
    38.617839923843085480,
    23.037715594963734597,
    5.3802499868335705968,
    0.6156793806995710776,
    0.0406688004668895584,
    0.0017296274552648414,
    0.000051061258836577,
    0.000000110459595022,
    0.0000000018235530206,
    0.00000000002370769803,
    0.00000000000024887296,
    0.0000000000000021529,
    0.0000000000000000156,
];

const SYNCHROTRON21_CS: ChebSeries = ChebSeries {
    c: &SYNCHROTRON21_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const SYNCHROTRON22_DATA: [f64; 13] = [
    7.9063148270660804288,
    3.1353463612853425684,
    0.4854879477453714538,
    0.0394816675827237234,
    0.0019661622334808802,
    0.0000659078932293042,
    0.0000015857561349856,
    0.0000000286865301123,
    0.0000000004041202360,
    0.0000000000045568444,
    0.0000000000000420459,
    0.0000000000000003232,
    0.0000000000000000021,
];

const SYNCHROTRON22_CS: ChebSeries = ChebSeries {
    c: &SYNCHROTRON22_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

const SYNCHROTRON2A_DATA: [f64; 17] = [
    2.020337094170713600,
    0.010956237121807404,
    0.0008542384730114676,
    0.0000723430242132822,
    0.0000063124427962699,
    0.0000005648193141174,
    0.0000000512832480138,
    0.0000000047196532914,
    0.0000000004380744214,
    0.0000000000410268149,
    0.0000000000038623072,
    0.0000000000003661323,
    0.0000000000000348023,
    0.0000000000000033301,
    0.000000000000000319,
    0.0000000000000000307,
    0.000000000000000003,
];

const SYNCHROTRON2A_CS: ChebSeries = ChebSeries {
    c: &SYNCHROTRON2A_DATA,
    order: 16,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

pub fn gsl_sf_synchrotron_1_e(x: f64, result: &mut GslSfResult) -> i32 {
    if x < 0.0 {
        result.val = NAN;
        result.err = NAN;
        return 1; // GSL_EDOM
    } else if x < 2.0 * SQRT_2 * EPSILON {
        let z = x.powf(1.0 / 3.0);
        let cf = 1.0 - 0.843812762813205 * z * z;
        result.val = 2.14952824153447863671 * z * cf;
        result.err = 2.2204460492503131e-16 * result.val;
        0 // GSL_SUCCESS
    } else if x <= 4.0 {
        let c0 = PI / SQRT_3;
        let px = x.powf(1.0 / 3.0);
        let px11 = px.powi(11);
        let t = x * x / 8.0 - 1.0;
        
        let mut result_c1 = GslSfResult { val: 0.0, err: 0.0 };
        let mut result_c2 = GslSfResult { val: 0.0, err: 0.0 };
        
        SYNCHROTRON1_CS.eval(t, &mut result_c1);
        SYNCHROTRON2_CS.eval(t, &mut result_c2);
        
        result.val = px * result_c1.val - px11 * result_c2.val - c0 * x;
        result.err = px * result_c1.err + px11 * result_c2.err + c0 * x * 2.2204460492503131e-16;
        result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    } else if x < -8.0 * -7.0839641853226408e+02 / 7.0 {
        let c0 = 0.2257913526447274323630976;
        let t = (12.0 - x) / (x + 4.0);
        let mut result_c1 = GslSfResult { val: 0.0, err: 0.0 };
        
        SYNCHROTRON1A_CS.eval(t, &mut result_c1);
        
        result.val = x.sqrt() * result_c1.val * (c0 - x).exp();
        result.err = 2.0 * 2.2204460492503131e-16 * result.val * ((c0 - x).abs() + 1.0);
        0 // GSL_SUCCESS
    } else {
        result.val = 0.0;
        result.err = MIN_POSITIVE;
        15 // GSL_EUNDRFLW
    }
}

pub fn gsl_sf_synchrotron_2_e(x: f64, result: &mut GslSfResult) -> i32 {
    if x < 0.0 {
        result.val = NAN;
        result.err = NAN;
        return 1; // GSL_EDOM
    } else if x < 2.0 * SQRT_2 * EPSILON {
        let z = x.powf(1.0 / 3.0);
        let cf = 1.0 - 1.17767156510235 * z * x;
        result.val = 1.07476412076723931836 * z * cf;
        result.err = 2.0 * 2.2204460492503131e-16 * result.val;
        0 // GSL_SUCCESS
    } else if x <= 4.0 {
        let px = x.powf(1.0 / 3.0);
        let px5 = px.powi(5);
        let t = x * x / 8.0 - 1.0;
        
        let mut cheb1 = GslSfResult { val: 0.0, err: 0.0 };
        let mut cheb2 = GslSfResult { val: 0.0, err: 0.0 };
        
        SYNCHROTRON21_CS.eval(t, &mut cheb1);
        SYNCHROTRON22_CS.eval(t, &mut cheb2);
        
        result.val = px * cheb1.val - px5 * cheb2.val;
        result.err = px * cheb1.err + px5 * cheb2.err;
        result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
        0 // GSL_SUCCESS
    } else if x < -8.0 * -7.0839641853226408e+02 / 7.0 {
        let c0 = 0.22579135264472743236;
        let t = (10.0 - x) / (x + 2.0);
        let mut cheb1 = GslSfResult { val: 0.0, err: 0.0 };
        
        SYNCHROTRON2A_CS.eval(t, &mut cheb1);
        
        result.val = x.sqrt() * (c0 - x).exp() * cheb1.val;
        result.err = 2.2204460492503131e-16 * result.val * ((c0 - x).abs() + 1.0);
        0 // GSL_SUCCESS
    } else {
        result.val = 0.0;
        result.err = MIN_POSITIVE;
        15 // GSL_EUNDRFLW
    }
}

pub fn gsl_sf_synchrotron_1(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_synchrotron_1_e(x, &mut result);
    if status != 0 {
        return result.val;
    }
    result.val
}

pub fn gsl_sf_synchrotron_2(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_synchrotron_2_e(x, &mut result);
    if status != 0 {
        return result.val;
    }
    result.val
}

const SQRT_2: f64 = 1.41421356237309504880;