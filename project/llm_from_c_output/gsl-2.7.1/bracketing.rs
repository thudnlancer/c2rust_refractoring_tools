//! Bracketing interval finding for function minimization

use std::f64;

const GOLDEN: f64 = 0.3819660; // (3 - sqrt(5))/2
const GSL_SQRT_DBL_EPSILON: f64 = f64::EPSILON.sqrt();

/// Result type for bracketing operations
#[derive(Debug, PartialEq)]
pub enum BracketResult {
    Success,
    Failure,
}

/// Function type that can be minimized
pub trait GslFunction {
    fn eval(&self, x: f64) -> f64;
}

/// Find an initial bracketing interval for a function to minimize
pub fn gsl_min_find_bracket<F: GslFunction>(
    f: &F,
    x_minimum: &mut f64,
    f_minimum: &mut f64,
    x_lower: &mut f64,
    f_lower: &mut f64,
    x_upper: &mut f64,
    f_upper: &mut f64,
    eval_max: usize,
) -> BracketResult {
    // Volatile variables to prevent extended precision issues
    let mut f_left = *f_lower;
    let mut f_right = *f_upper;
    let mut f_center;
    let mut x_left = *x_lower;
    let mut x_right = *x_upper;
    let mut x_center;
    let mut nb_eval = 0;

    if f_right >= f_left {
        x_center = (x_right - x_left) * GOLDEN + x_left;
        nb_eval += 1;
        f_center = f.eval(x_center);
    } else {
        x_center = x_right;
        f_center = f_right;
        x_right = (x_center - x_left) / GOLDEN + x_left;
        nb_eval += 1;
        f_right = f.eval(x_right);
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
                return BracketResult::Success;
            } else if f_center > f_right {
                x_left = x_center;
                f_left = f_center;
                x_center = x_right;
                f_center = f_right;
                x_right = (x_center - x_left) / GOLDEN + x_left;
                nb_eval += 1;
                f_right = f.eval(x_right);
            } else {
                // f_center == f_right
                x_right = x_center;
                f_right = f_center;
                x_center = (x_right - x_left) * GOLDEN + x_left;
                nb_eval += 1;
                f_center = f.eval(x_center);
            }
        } else {
            // f_center >= f_left
            x_right = x_center;
            f_right = f_center;
            x_center = (x_right - x_left) * GOLDEN + x_left;
            nb_eval += 1;
            f_center = f.eval(x_center);
        }

        if nb_eval >= eval_max {
            break;
        }

        let width = x_right - x_left;
        let midpoint = (x_right + x_left) * 0.5;
        if width <= GSL_SQRT_DBL_EPSILON * midpoint + GSL_SQRT_DBL_EPSILON {
            break;
        }
    }

    *x_lower = x_left;
    *x_upper = x_right;
    *x_minimum = x_center;
    *f_lower = f_left;
    *f_upper = f_right;
    *f_minimum = f_center;
    BracketResult::Failure
}