use std::f64::consts;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

const LOPX_DATA: [f64; 21] = [
    2.16647910664395270521272590407,
    -0.28565398551049742084877469679,
    0.01517767255690553732382488171,
    -0.00200215904941415466274422081,
    0.00019211375164056698287947962,
    -0.00002553258886105542567601400,
    2.9004512660400621301999384544e-06,
    -3.8873813517057343800270917900e-07,
    4.7743678729400456026672697926e-08,
    -6.4501969776090319441714445454e-09,
    8.2751976628812389601561347296e-10,
    -1.1260499376492049411710290413e-10,
    1.4844576692270934446023686322e-11,
    -2.0328515972462118942821556033e-12,
    2.7291231220549214896095654769e-13,
    -3.7581977830387938294437434651e-14,
    5.1107345870861673561462339876e-15,
    -7.0722150011433276578323272272e-16,
    9.7089758328248469219003866867e-17,
    -1.3492637457521938883731579510e-17,
    1.8657327910677296608121390705e-18,
];

const LOPXMX_DATA: [f64; 20] = [
    -1.12100231323744103373737274541,
    0.19553462773379386241549597019,
    -0.01467470453808083971825344956,
    0.00166678250474365477643629067,
    -0.00018543356147700369785746902,
    0.00002280154021771635036301071,
    -2.8031253116633521699214134172e-06,
    3.5936568872522162983669541401e-07,
    -4.6241857041062060284381167925e-08,
    6.0822637459403991012451054971e-09,
    -8.0339824424815790302621320732e-10,
    1.0751718277499375044851551587e-10,
    -1.4445310914224613448759230882e-11,
    1.9573912180610336168921438426e-12,
    -2.6614436796793061741564104510e-13,
    3.6402634315269586532158344584e-14,
    -4.9937495922755006545809120531e-15,
    6.8802890218846809524646902703e-16,
    -9.5034129794804273611403251480e-17,
    1.3170135013050997157326965813e-17,
];

const LOPX_CS: ChebSeries = ChebSeries {
    data: &LOPX_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

const LOPXMX_CS: ChebSeries = ChebSeries {
    data: &LOPXMX_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_ROOT6_DBL_EPSILON: f64 = f64::EPSILON.powf(1.0 / 6.0);
const GSL_ROOT5_DBL_EPSILON: f64 = f64::EPSILON.powf(1.0 / 5.0);

#[derive(Debug)]
pub enum SfError {
    DomainError,
}

pub fn gsl_sf_log_e(x: f64) -> Result<SfResult, SfError> {
    if x <= 0.0 {
        Err(SfError::DomainError)
    } else {
        let val = x.ln();
        let err = 2.0 * GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    }
}

pub fn gsl_sf_log_abs_e(x: f64) -> Result<SfResult, SfError> {
    if x == 0.0 {
        Err(SfError::DomainError)
    } else {
        let val = x.abs().ln();
        let err = 2.0 * GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    }
}

pub fn gsl_sf_complex_log_e(zr: f64, zi: f64) -> Result<(SfResult, SfResult), SfError> {
    if zr == 0.0 && zi == 0.0 {
        Err(SfError::DomainError)
    } else {
        let ax = zr.abs();
        let ay = zi.abs();
        let min = ax.min(ay);
        let max = ax.max(ay);
        let lnr_val = max.ln() + 0.5 * (1.0 + (min / max).powi(2)).ln();
        let lnr_err = 2.0 * GSL_DBL_EPSILON * lnr_val.abs();
        let theta_val = zi.atan2(zr);
        let theta_err = GSL_DBL_EPSILON * lnr_val.abs();
        Ok((
            SfResult {
                val: lnr_val,
                err: lnr_err,
            },
            SfResult {
                val: theta_val,
                err: theta_err,
            },
        ))
    }
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;

    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        e += (y2 * temp).abs() + dd.abs() + cs.data[j].abs();
        dd = temp;
    }

    let temp = d;
    d = y * d - dd + 0.5 * cs.data[0];
    e += (y * temp).abs() + dd.abs() + 0.5 * cs.data[0].abs();

    let val = d;
    let err = GSL_DBL_EPSILON * e + (cs.data[cs.order]).abs();
    SfResult { val, err }
}

pub fn gsl_sf_log_1plusx_e(x: f64) -> Result<SfResult, SfError> {
    if x <= -1.0 {
        Err(SfError::DomainError)
    } else if x.abs() < GSL_ROOT6_DBL_EPSILON {
        const C1: f64 = -0.5;
        const C2: f64 = 1.0 / 3.0;
        const C3: f64 = -1.0 / 4.0;
        const C4: f64 = 1.0 / 5.0;
        const C5: f64 = -1.0 / 6.0;
        const C6: f64 = 1.0 / 7.0;
        const C7: f64 = -1.0 / 8.0;
        const C8: f64 = 1.0 / 9.0;
        const C9: f64 = -1.0 / 10.0;
        let t = C5 + x * (C6 + x * (C7 + x * (C8 + x * C9)));
        let val = x * (1.0 + x * (C1 + x * (C2 + x * (C3 + x * (C4 + x * t))));
        let err = GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x.abs() < 0.5 {
        let t = 0.5 * (8.0 * x + 1.0) / (x + 2.0);
        let c = cheb_eval_e(&LOPX_CS, t);
        let val = x * c.val;
        let err = (x * c.err).abs();
        Ok(SfResult { val, err })
    } else {
        let val = (1.0 + x).ln();
        let err = GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    }
}

pub fn gsl_sf_log_1plusx_mx_e(x: f64) -> Result<SfResult, SfError> {
    if x <= -1.0 {
        Err(SfError::DomainError)
    } else if x.abs() < GSL_ROOT5_DBL_EPSILON {
        const C1: f64 = -0.5;
        const C2: f64 = 1.0 / 3.0;
        const C3: f64 = -1.0 / 4.0;
        const C4: f64 = 1.0 / 5.0;
        const C5: f64 = -1.0 / 6.0;
        const C6: f64 = 1.0 / 7.0;
        const C7: f64 = -1.0 / 8.0;
        const C8: f64 = 1.0 / 9.0;
        const C9: f64 = -1.0 / 10.0;
        let t = C5 + x * (C6 + x * (C7 + x * (C8 + x * C9)));
        let val = x.powi(2) * (C1 + x * (C2 + x * (C3 + x * (C4 + x * t))));
        let err = GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x.abs() < 0.5 {
        let t = 0.5 * (8.0 * x + 1.0) / (x + 2.0);
        let c = cheb_eval_e(&LOPXMX_CS, t);
        let val = x.powi(2) * c.val;
        let err = x.powi(2) * c.err;
        Ok(SfResult { val, err })
    } else {
        let lterm = (1.0 + x).ln();
        let val = lterm - x;
        let err = GSL_DBL_EPSILON * (lterm.abs() + x.abs());
        Ok(SfResult { val, err })
    }
}

pub fn gsl_sf_log(x: f64) -> f64 {
    gsl_sf_log_e(x).unwrap().val
}

pub fn gsl_sf_log_abs(x: f64) -> f64 {
    gsl_sf_log_abs_e(x).unwrap().val
}

pub fn gsl_sf_log_1plusx(x: f64) -> f64 {
    gsl_sf_log_1plusx_e(x).unwrap().val
}

pub fn gsl_sf_log_1plusx_mx(x: f64) -> f64 {
    gsl_sf_log_1plusx_mx_e(x).unwrap().val
}