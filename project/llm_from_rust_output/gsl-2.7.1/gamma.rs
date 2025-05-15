use std::f64::consts::{PI, E};
use std::f64::{INFINITY, NAN};

const GSL_SUCCESS: i32 = 0;
const GSL_EDOM: i32 = 1;
const GSL_EUNDRFLW: i32 = 15;
const GSL_EOVRFLW: i32 = 16;
const GSL_EROUND: i32 = 18;

#[derive(Debug, Clone, Copy)]
pub struct GslSFResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
struct FactEntry {
    n: i32,
    f: f64,
    i: i64,
}

#[derive(Debug, Clone, Copy)]
struct ChebSeries {
    c: *const f64,
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

static FACT_TABLE: [FactEntry; 171] = [
    // Factorial table entries...
];

static DOUB_FACT_TABLE: [FactEntry; 298] = [
    // Double factorial table entries...
];

static GSTAR_A_DATA: [f64; 30] = [
    // gstar_a_data entries...
];

static GSTAR_A_CS: ChebSeries = ChebSeries {
    c: GSTAR_A_DATA.as_ptr(),
    order: 29,
    a: -1.0,
    b: 1.0,
    order_sp: 17,
};

static GSTAR_B_DATA: [f64; 30] = [
    // gstar_b_data entries...
];

static GSTAR_B_CS: ChebSeries = ChebSeries {
    c: GSTAR_B_DATA.as_ptr(),
    order: 29,
    a: -1.0,
    b: 1.0,
    order_sp: 18,
};

static LANCZOS_7_C: [f64; 9] = [
    // Lanczos coefficients...
];

fn lngamma_lanczos_complex(zr: f64, zi: f64, yr: &mut GslSFResult, yi: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

fn lngamma_lanczos(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

fn lngamma_sgn_0(eps: f64, lng: &mut GslSFResult, sgn: &mut f64) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

fn lngamma_sgn_sing(n: i32, eps: f64, lng: &mut GslSFResult, sgn: &mut f64) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

fn lngamma_1_pade(eps: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

fn lngamma_2_pade(eps: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

fn gammastar_ser(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

static GAMMA_5_10_DATA: [f64; 24] = [
    // gamma_5_10_data entries...
];

static GAMMA_5_10_CS: ChebSeries = ChebSeries {
    c: GAMMA_5_10_DATA.as_ptr(),
    order: 23,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

fn gamma_xgthalf(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_lngamma_e(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_lngamma_sgn_e(x: f64, result_lg: &mut GslSFResult, sgn: &mut f64) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_gamma_e(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_gammastar_e(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_gammainv_e(x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_lngamma_complex_e(zr: f64, zi: f64, lnr: &mut GslSFResult, arg: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_taylorcoeff_e(n: i32, x: f64, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_fact_e(n: u32, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_doublefact_e(n: u32, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_lnfact_e(n: u32, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_lndoublefact_e(n: u32, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_lnchoose_e(n: u32, m: u32, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

pub fn gsl_sf_choose_e(n: u32, m: u32, result: &mut GslSFResult) -> i32 {
    // Implementation...
    GSL_SUCCESS
}

// Wrapper functions...

pub fn gsl_sf_fact(n: u32) -> f64 {
    let mut result = GslSFResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_fact_e(n, &mut result);
    if status != GSL_SUCCESS {
        // Handle error
    }
    result.val
}

// Other wrapper functions...