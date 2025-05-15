// integration/qk.rs

use std::f64;

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

pub fn gsl_integration_qk(
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
    let f_center = f.eval(center);

    let mut result_gauss = 0.0;
    let mut result_kronrod = f_center * wgk[n - 1];

    let mut result_abs = result_kronrod.abs();
    let mut result_asc = 0.0;
    let mean;
    let err;

    if n % 2 == 0 {
        result_gauss = f_center * wg[n / 2 - 1];
    }

    for j in 0..(n - 1) / 2 {
        let jtw = j * 2 + 1;
        let abscissa = half_length * xgk[jtw];
        let fval1 = f.eval(center - abscissa);
        let fval2 = f.eval(center + abscissa);
        let fsum = fval1 + fval2;
        fv1[jtw] = fval1;
        fv2[jtw] = fval2;
        result_gauss += wg[j] * fsum;
        result_kronrod += wgk[jtw] * fsum;
        result_abs += wgk[jtw] * (fval1.abs() + fval2.abs());
    }

    for j in 0..n / 2 {
        let jtwm1 = j * 2;
        let abscissa = half_length * xgk[jtwm1];
        let fval1 = f.eval(center - abscissa);
        let fval2 = f.eval(center + abscissa);
        fv1[jtwm1] = fval1;
        fv2[jtwm1] = fval2;
        result_kronrod += wgk[jtwm1] * (fval1 + fval2);
        result_abs += wgk[jtwm1] * (fval1.abs() + fval2.abs());
    }

    mean = result_kronrod * 0.5;
    result_asc = wgk[n - 1] * (f_center - mean).abs();

    for j in 0..n - 1 {
        result_asc += wgk[j] * ((fv1[j] - mean).abs() + (fv2[j] - mean).abs());
    }

    // scale by the width of the integration region
    err = (result_kronrod - result_gauss) * half_length;
    result_kronrod *= half_length;
    result_abs *= abs_half_length;
    result_asc *= abs_half_length;

    let abserr = rescale_error(err, result_abs, result_asc);

    (result_kronrod, abserr, result_abs, result_asc)
}

fn rescale_error(err: f64, result_abs: f64, result_asc: f64) -> f64 {
    let err = err.abs();
    
    if result_asc != 0.0 && err != 0.0 {
        let scale = (200.0 * err / result_asc).powf(1.5);
        if scale < 1.0 {
            err = err * scale;
        } else {
            err = result_asc;
        }
    }
    
    if result_abs > f64::MIN_POSITIVE / (50.0 * f64::EPSILON) {
        let min_err = 50.0 * f64::EPSILON * result_abs;
        if min_err > err {
            err = min_err;
        }
    }
    
    err
}