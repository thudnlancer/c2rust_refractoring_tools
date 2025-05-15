use std::f64;

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

fn rescale_error(err: f64, result_abs: f64, result_asc: f64) -> f64 {
    let err = err.abs();
    if result_asc != 0.0 && err != 0.0 {
        let scale = (200.0 * err / result_asc).powf(1.5);
        if scale < 1.0 {
            return result_asc * scale;
        } else {
            return result_asc;
        }
    }
    if result_abs > 2.2250738585072014e-308 / (50.0 * 2.2204460492503131e-16) {
        let min_err = 50.0 * 2.2204460492503131e-16 * result_abs;
        if min_err > err {
            return min_err;
        }
    }
    err
}

pub fn gsl_integration_qk(
    n: i32,
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
    let f_center = (f.function)(center);

    let mut result_gauss = 0.0;
    let mut result_kronrod = f_center * wgk[n as usize - 1];
    let mut result_abs = result_kronrod.abs();
    let mut result_asc = 0.0;
    let mut mean;
    let mut err;

    if n % 2 == 0 {
        result_gauss = f_center * wg[n as usize / 2 - 1];
    }

    for j in 0..(n - 1) / 2 {
        let jtw = j * 2 + 1;
        let abscissa = half_length * xgk[jtw as usize];
        let fval1 = (f.function)(center - abscissa);
        let fval2 = (f.function)(center + abscissa);
        let fsum = fval1 + fval2;

        fv1[jtw as usize] = fval1;
        fv2[jtw as usize] = fval2;
        result_gauss += wg[j as usize] * fsum;
        result_kronrod += wgk[jtw as usize] * fsum;
        result_abs += wgk[jtw as usize] * (fval1.abs() + fval2.abs());
    }

    for j in 0..n / 2 {
        let jtwm1 = j * 2;
        let abscissa = half_length * xgk[jtwm1 as usize];
        let fval1 = (f.function)(center - abscissa);
        let fval2 = (f.function)(center + abscissa);

        fv1[jtwm1 as usize] = fval1;
        fv2[jtwm1 as usize] = fval2;
        result_kronrod += wgk[jtwm1 as usize] * (fval1 + fval2);
        result_abs += wgk[jtwm1 as usize] * (fval1.abs() + fval2.abs());
    }

    mean = result_kronrod * 0.5;
    result_asc = wgk[n as usize - 1] * (f_center - mean).abs();

    for j in 0..n - 1 {
        result_asc += wgk[j as usize] * ((fv1[j as usize] - mean).abs() + (fv2[j as usize] - mean).abs());
    }

    err = (result_kronrod - result_gauss) * half_length;
    result_kronrod *= half_length;
    result_abs *= abs_half_length;
    result_asc *= abs_half_length;

    (result_kronrod, rescale_error(err, result_abs, result_asc), result_abs, result_asc)
}