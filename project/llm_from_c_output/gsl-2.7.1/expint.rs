use std::f64::consts::{E, LN_2};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
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

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let d = 0.0;
    let dd = 0.0;
    
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut result = SfResult { val: 0.0, err: 0.0 };
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        dd = temp;
    }
    
    result.val = y * d - dd + 0.5 * cs.data[0];
    result.err = f64::EPSILON * (cs.order as f64 + 1.0) * result.val.abs();
    
    result
}

// Chebyshev series definitions
lazy_static! {
    static ref AE11_CS: ChebSeries = ChebSeries::new(&[
        0.121503239716065790,
        -0.065088778513550150,
        0.004897651357459670,
        -0.000649237843027216,
        0.000093840434587471,
        0.000000420236380882,
        -0.000008113374735904,
        0.000002804247688663,
        0.000000056487164441,
        -0.000000344809174450,
        0.000000058209273578,
        0.000000038711426349,
        -0.000000012453235014,
        -0.000000005118504888,
        0.000000002148771527,
        0.000000000868459898,
        -0.000000000343650105,
        -0.000000000179796603,
        0.000000000047442060,
        0.000000000040423282,
        -0.000000000003543928,
        -0.000000000008853444,
        -0.000000000000960151,
        0.000000000001692921,
        0.000000000000607990,
        -0.000000000000224338,
        -0.000000000000200327,
        -0.000000000000006246,
        0.000000000000045571,
        0.000000000000016383,
        -0.000000000000005561,
        -0.000000000000006074,
        -0.000000000000000862,
        0.000000000000001223,
        0.000000000000000716,
        -0.000000000000000024,
        -0.000000000000000201,
        -0.000000000000000082,
        0.000000000000000017
    ], 38, -1.0, 1.0, 20);

    static ref AE12_CS: ChebSeries = ChebSeries::new(&[
        0.582417495134726740,
        -0.158348850905782750,
        -0.006764275590323141,
        0.005125843950185725,
        0.000435232492169391,
        -0.000143613366305483,
        -0.000041801320556301,
        -0.000002713395758640,
        0.000001151381913647,
        0.000000420650022012,
        0.000000066581901391,
        0.000000000662143777,
        -0.000000002844104870,
        -0.000000000940724197,
        -0.000000000177476602,
        -0.000000000015830222,
        0.000000000002905732,
        0.000000000001769356,
        0.000000000000492735,
        0.000000000000093709,
        0.000000000000010707,
        -0.000000000000000537,
        -0.000000000000000716,
        -0.000000000000000244,
        -0.000000000000000058
    ], 24, -1.0, 1.0, 15);

    static ref E11_CS: ChebSeries = ChebSeries::new(&[
        -16.11346165557149402600,
        7.79407277874268027690,
        -1.95540581886314195070,
        0.37337293866277945612,
        -0.05692503191092901938,
        0.00721107776966009185,
        -0.00078104901449841593,
        0.00007388093356262168,
        -0.00000620286187580820,
        0.00000046816002303176,
        -0.00000003209288853329,
        0.00000000201519974874,
        -0.00000000011673686816,
        0.00000000000627627066,
        -0.00000000000031481541,
        0.00000000000001479904,
        -0.00000000000000065457,
        0.00000000000000002733,
        -0.00000000000000000108
    ], 18, -1.0, 1.0, 13);

    static ref E12_CS: ChebSeries = ChebSeries::new(&[
        -0.03739021479220279500,
        0.04272398606220957700,
        -0.13031820798497005440,
        0.01441912402469889073,
        -0.00134617078051068022,
        0.00010731029253063780,
        -0.00000742999951611943,
        0.00000045377325690753,
        -0.00000002476417211390,
        0.00000000122076581374,
        -0.00000000005485141480,
        0.00000000000226362142,
        -0.00000000000008635897,
        0.00000000000000306291,
        -0.00000000000000010148,
        0.00000000000000000315
    ], 15, -1.0, 1.0, 10);

    static ref AE13_CS: ChebSeries = ChebSeries::new(&[
        -0.605773246640603460,
        -0.112535243483660900,
        0.013432266247902779,
        -0.001926845187381145,
        0.000309118337720603,
        -0.000053564132129618,
        0.000009827812880247,
        -0.000001885368984916,
        0.000000374943193568,
        -0.000000076823455870,
        0.000000016143270567,
        -0.000000003466802211,
        0.000000000758754209,
        -0.000000000168864333,
        0.000000000038145706,
        -0.000000000008733026,
        0.000000000002023672,
        -0.000000000000474132,
        0.000000000000112211,
        -0.000000000000026804,
        0.000000000000006457,
        -0.000000000000001568,
        0.000000000000000383,
        -0.000000000000000094,
        0.000000000000000023
    ], 24, -1.0, 1.0, 15);

    static ref AE14_CS: ChebSeries = ChebSeries::new(&[
        -0.18929180007530170,
        -0.08648117855259871,
        0.00722410154374659,
        -0.00080975594575573,
        0.00010999134432661,
        -0.00001717332998937,
        0.00000298562751447,
        -0.00000056596491457,
        0.00000011526808397,
        -0.00000002495030440,
        0.00000000569232420,
        -0.00000000135995766,
        0.00000000033846628,
        -0.00000000008737853,
        0.00000000002331588,
        -0.00000000000641148,
        0.00000000000181224,
        -0.00000000000052538,
        0.00000000000015592,
        -0.00000000000004729,
        0.00000000000001463,
        -0.00000000000000461,
        0.00000000000000148,
        -0.00000000000000048,
        0.00000000000000016,
        -0.00000000000000005
    ], 25, -1.0, 1.0, 13);
}

fn expint_e1_impl(x: f64, scale: bool) -> Result<SfResult, &'static str> {
    let xmaxt = -f64::MIN_POSITIVE.ln();
    let xmax = xmaxt - xmaxt.ln();

    if x < -xmax && !scale {
        return Err("overflow");
    } else if x <= -10.0 {
        let s = 1.0 / x * if scale { 1.0 } else { (-x).exp() };
        let result_c = cheb_eval_e(&AE11_CS, 20.0 / x + 1.0);
        let val = s * (1.0 + result_c.val);
        let mut err = s * result_c.err;
        err += 2.0 * f64::EPSILON * (x.abs() + 1.0) * val.abs();
        Ok(SfResult { val, err })
    } else if x <= -4.0 {
        let s = 1.0 / x * if scale { 1.0 } else { (-x).exp() };
        let result_c = cheb_eval_e(&AE12_CS, (40.0 / x + 7.0) / 3.0);
        let val = s * (1.0 + result_c.val);
        let mut err = s * result_c.err;
        err += 2.0 * f64::EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x <= -1.0 {
        let ln_term = -x.abs().ln();
        let scale_factor = if scale { x.exp() } else { 1.0 };
        let result_c = cheb_eval_e(&E11_CS, (2.0 * x + 5.0) / 3.0);
        let val = scale_factor * (ln_term + result_c.val);
        let mut err = scale_factor * (result_c.err + f64::EPSILON * ln_term.abs());
        err += 2.0 * f64::EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x == 0.0 {
        Err("domain error")
    } else if x <= 1.0 {
        let ln_term = -x.abs().ln();
        let scale_factor = if scale { x.exp() } else { 1.0 };
        let result_c = cheb_eval_e(&E12_CS, x);
        let val = scale_factor * (ln_term - 0.6875 + x + result_c.val);
        let mut err = scale_factor * (result_c.err + f64::EPSILON * ln_term.abs());
        err += 2.0 * f64::EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x <= 4.0 {
        let s = 1.0 / x * if scale { 1.0 } else { (-x).exp() };
        let result_c = cheb_eval_e(&AE13_CS, (8.0 / x - 5.0) / 3.0);
        let val = s * (1.0 + result_c.val);
        let mut err = s * result_c.err;
        err += 2.0 * f64::EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x <= xmax || scale {
        let s = 1.0 / x * if scale { 1.0 } else { (-x).exp() };
        let result_c = cheb_eval_e(&AE14_CS, 8.0 / x - 1.0);
        let val = s * (1.0 + result_c.val);
        let mut err = s * (f64::EPSILON + result_c.err);
        err += 2.0 * (x + 1.0) * f64::EPSILON * val.abs();
        if val == 0.0 {
            Err("underflow")
        } else {
            Ok(SfResult { val, err })
        }
    } else {
        Err("underflow")
    }
}

fn expint_e2_impl(x: f64, scale: bool) -> Result<SfResult, &'static str> {
    let xmaxt = -f64::MIN_POSITIVE.ln();
    let xmax = xmaxt - xmaxt.ln();

    if x < -xmax && !scale {
        return Err("overflow");
    } else if x == 0.0 {
        Ok(SfResult { val: 1.0, err: 0.0 })
    } else if x < 100.0 {
        let ex = if scale { 1.0 } else { (-x).exp() };
        let result_e1 = expint_e1_impl(x, scale)?;
        let val = ex - x * result_e1.val;
        let mut err = f64::EPSILON * ex + x.abs() * result_e1.err;
        err += 2.0 * f64::EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if x < xmax || scale {
        let s = if scale { 1.0 } else { (-x).exp() };
        let c1 = -2.0;
        let c2 = 6.0;
        let c3 = -24.0;
        let c4 = 120.0;
        let c5 = -720.0;
        let c6 = 5040.0;
        let c7 = -40320.0;
        let c8 = 362880.0;
        let c9 = -3628800.0;
        let c10 = 39916800.0;
        let c11 = -479001600.0;
        let c12 = 6227020800.0;
        let c13 = -87178291200.0;
        let y = 1.0 / x;
        let sum6 = c6 + y * (c7 + y * (c8 + y * (c9 + y * (c10 + y * (c11 + y * (c12 + y * c13)))));
        let sum = y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * sum6))));
        let val = s * (1.0 + sum) / x;
        let mut err = 2.0 * (x + 1.0) * f64::EPSILON * val.abs();
        if val == 0.0 {
            Err("underflow")
        } else {
            Ok(SfResult { val, err })
        }
    } else {
        Err("underflow")
    }
}

fn expint_en_impl(n: i32, x: f64, scale: bool) -> Result<SfResult, &'static str> {
    if n < 0 {
        Err("domain error")
    } else if n == 0 {
        if x == 0.0 {
            Err("domain error")
        } else {
            let val = (if scale { 1.0 } else { (-x).exp() }) / x;
            let err = 2.0 * f64::EPSILON * val.abs();
            if val == 0.0 {
                Err("underflow")
            } else {
                Ok(SfResult { val, err })
            }
        }
    } else if n == 1 {
        expint_e1_impl(x, scale)
    } else if n == 2 {
        expint_e2_impl(x, scale)
    } else {
        if x < 0.0 {
            return Err("domain error");
        }
        if x == 0.0 {
            let val = (if scale { x.exp() } else { 1.0 }) * (1.0 / (n as f64 - 1.0));
            let err = 2.0 * f64::EPSILON * val.abs();
            if val == 0.0 {
                Err("underflow")
            } else {
                Ok(SfResult { val, err })
            }
        } else {
            let prefactor = x.powf(n as f64 - 1.0);
            // TODO: Implement gamma_inc function
            let result_g = SfResult { val: 0.0, err: 0.0 }; // Placeholder
            let scale_factor = if scale