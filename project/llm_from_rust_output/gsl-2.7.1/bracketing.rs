use std::f64;

pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDom = 1,
    ERange = 2,
    EDefault = 3,
    EInval = 4,
    EFailed = 5,
    EFactor = 6,
    ESanity = 7,
    ENomem = 8,
    EBadfunc = 9,
    ERunaway = 10,
    EMaxiter = 11,
    EZerodiv = 12,
    EBadtol = 13,
    ETol = 14,
    EUndrflw = 15,
    EOvrflw = 16,
    ELoss = 17,
    ERound = 18,
    EBadlen = 19,
    ENotsqr = 20,
    ESing = 21,
    EDiverge = 22,
    EUnsup = 23,
    EUnimpl = 24,
    ECache = 25,
    ETable = 26,
    ENoprog = 27,
    ENoprogj = 28,
    ETolf = 29,
    ETolx = 30,
    ETolg = 31,
    Eof = 32,
}

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn new<F: Fn(f64) -> f64 + 'static>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }
}

pub fn gsl_min_find_bracket(
    f: &GslFunction,
    x_minimum: &mut f64,
    f_minimum: &mut f64,
    x_lower: &mut f64,
    f_lower: &mut f64,
    x_upper: &mut f64,
    f_upper: &mut f64,
    eval_max: usize,
) -> Result<(), GslError> {
    let mut f_left = *f_lower;
    let mut f_right = *f_upper;
    let mut f_center = 0.0;
    let mut x_left = *x_lower;
    let mut x_right = *x_upper;
    let mut x_center = 0.0;
    let golden = 0.3819660;
    let mut nb_eval = 0;

    if f_right >= f_left {
        x_center = (x_right - x_left) * golden + x_left;
        nb_eval += 1;
        f_center = (f.function)(x_center);
        if !f_center.is_finite() {
            return Err(GslError::EBadfunc);
        }
    } else {
        x_center = x_right;
        f_center = f_right;
        x_right = (x_center - x_left) / golden + x_left;
        nb_eval += 1;
        f_right = (f.function)(x_right);
        if !f_right.is_finite() {
            return Err(GslError::EBadfunc);
        }
    }

    loop {
        if f_center < f_left {
            if f_center < f_right {
                *x_lower = x_left;
                *x_upper = x_right;
                *x_minimum = x_center;
                *f_lower = f_left;
                *f_upper = f_right;
                *f_minimum = f_center;
                return Ok(());
            } else if f_center > f_right {
                x_left = x_center;
                f_left = f_center;
                x_center = x_right;
                f_center = f_right;
                x_right = (x_center - x_left) / golden + x_left;
                nb_eval += 1;
                f_right = (f.function)(x_right);
                if !f_right.is_finite() {
                    return Err(GslError::EBadfunc);
                }
            } else {
                x_right = x_center;
                f_right = f_center;
                x_center = (x_right - x_left) * golden + x_left;
                nb_eval += 1;
                f_center = (f.function)(x_center);
                if !f_center.is_finite() {
                    return Err(GslError::EBadfunc);
                }
            }
        } else {
            x_right = x_center;
            f_right = f_center;
            x_center = (x_right - x_left) * golden + x_left;
            nb_eval += 1;
            f_center = (f.function)(x_center);
            if !f_center.is_finite() {
                return Err(GslError::EBadfunc);
            }
        }

        if !(nb_eval < eval_max
            && x_right - x_left
                > 1.4901161193847656e-08 * ((x_right + x_left) * 0.5).abs()
                + 1.4901161193847656e-08)
        {
            break;
        }
    }

    *x_lower = x_left;
    *x_upper = x_right;
    *x_minimum = x_center;
    *f_lower = f_left;
    *f_upper = f_right;
    *f_minimum = f_center;
    Err(GslError::Failure)
}