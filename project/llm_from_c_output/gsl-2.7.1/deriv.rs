use std::f64;

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

pub const GSL_SUCCESS: i32 = 0;
const GSL_DBL_EPSILON: f64 = f64::EPSILON;

fn central_deriv(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, f64) {
    let fm1 = f.eval(x - h);
    let fp1 = f.eval(x + h);

    let fmh = f.eval(x - h / 2.0);
    let fph = f.eval(x + h / 2.0);

    let r3 = 0.5 * (fp1 - fm1);
    let r5 = (4.0 / 3.0) * (fph - fmh) - (1.0 / 3.0) * r3;

    let e3 = (fp1.abs() + fm1.abs()) * GSL_DBL_EPSILON;
    let e5 = 2.0 * (fph.abs() + fmh.abs()) * GSL_DBL_EPSILON + e3;

    let dy = f64::max((r3 / h).abs(), (r5 / h).abs()) * (x.abs() / h) * GSL_DBL_EPSILON;

    let result = r5 / h;
    let abserr_trunc = ((r5 - r3) / h).abs();
    let abserr_round = (e5 / h).abs() + dy;

    (result, abserr_round, abserr_trunc)
}

pub fn gsl_deriv_central(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, i32) {
    let (r_0, round, trunc) = central_deriv(f, x, h);
    let mut error = round + trunc;

    if round < trunc && (round > 0.0 && trunc > 0.0) {
        let h_opt = h * (round / (2.0 * trunc)).powf(1.0 / 3.0);
        let (r_opt, round_opt, trunc_opt) = central_deriv(f, x, h_opt);
        let error_opt = round_opt + trunc_opt;

        if error_opt < error && (r_opt - r_0).abs() < 4.0 * error {
            return (r_opt, error_opt, GSL_SUCCESS);
        }
    }

    (r_0, error, GSL_SUCCESS)
}

fn forward_deriv(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, f64) {
    let f1 = f.eval(x + h / 4.0);
    let f2 = f.eval(x + h / 2.0);
    let f3 = f.eval(x + (3.0 / 4.0) * h);
    let f4 = f.eval(x + h);

    let r2 = 2.0 * (f4 - f2);
    let r4 = (22.0 / 3.0) * (f4 - f3) - (62.0 / 3.0) * (f3 - f2) + (52.0 / 3.0) * (f2 - f1);

    let e4 = 2.0 * 20.67 * (f4.abs() + f3.abs() + f2.abs() + f1.abs()) * GSL_DBL_EPSILON;

    let dy = f64::max((r2 / h).abs(), (r4 / h).abs()) * (x.abs() / h) * GSL_DBL_EPSILON;

    let result = r4 / h;
    let abserr_trunc = ((r4 - r2) / h).abs();
    let abserr_round = (e4 / h).abs() + dy;

    (result, abserr_round, abserr_trunc)
}

pub fn gsl_deriv_forward(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, i32) {
    let (r_0, round, trunc) = forward_deriv(f, x, h);
    let mut error = round + trunc;

    if round < trunc && (round > 0.0 && trunc > 0.0) {
        let h_opt = h * (round / trunc).powf(1.0 / 2.0);
        let (r_opt, round_opt, trunc_opt) = forward_deriv(f, x, h_opt);
        let error_opt = round_opt + trunc_opt;

        if error_opt < error && (r_opt - r_0).abs() < 4.0 * error {
            return (r_opt, error_opt, GSL_SUCCESS);
        }
    }

    (r_0, error, GSL_SUCCESS)
}

pub fn gsl_deriv_backward(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, i32) {
    gsl_deriv_forward(f, x, -h)
}