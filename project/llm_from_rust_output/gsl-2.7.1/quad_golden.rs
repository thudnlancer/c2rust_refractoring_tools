use std::f64;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

pub type GslResult = Result<(), GslError>;

pub struct GslFunction {
    function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn new<F: Fn(f64) -> f64 + 'static>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

#[derive(Clone)]
pub struct QuadGoldenState {
    step_size: f64,
    stored_step: f64,
    prev_stored_step: f64,
    x_prev_small: f64,
    f_prev_small: f64,
    x_small: f64,
    f_small: f64,
    num_iter: u32,
}

impl QuadGoldenState {
    pub fn new() -> Self {
        QuadGoldenState {
            step_size: 0.0,
            stored_step: 0.0,
            prev_stored_step: 0.0,
            x_prev_small: 0.0,
            f_prev_small: 0.0,
            x_small: 0.0,
            f_small: 0.0,
            num_iter: 0,
        }
    }

    pub fn init(
        &mut self,
        x_minimum: f64,
        f_minimum: f64,
    ) -> GslResult {
        self.x_prev_small = x_minimum;
        self.x_small = x_minimum;
        self.f_prev_small = f_minimum;
        self.f_small = f_minimum;
        self.step_size = 0.0;
        self.stored_step = 0.0;
        self.prev_stored_step = 0.0;
        self.num_iter = 0;
        Ok(())
    }

    pub fn iterate(
        &mut self,
        f: &GslFunction,
        x_minimum: &mut f64,
        f_minimum: &mut f64,
        x_lower: &mut f64,
        f_lower: &mut f64,
        x_upper: &mut f64,
        f_upper: &mut f64,
    ) -> GslResult {
        let x_m = *x_minimum;
        let f_m = *f_minimum;
        let x_l = *x_lower;
        let x_u = *x_upper;
        let x_small = self.x_small;
        let f_small = self.f_small;
        let x_prev_small = self.x_prev_small;
        let f_prev_small = self.f_prev_small;
        let mut stored_step = self.stored_step;
        let mut prev_stored_step = self.prev_stored_step;
        let mut step_size = self.step_size;
        let mut quad_step_size = prev_stored_step;

        let x_midpoint = 0.5 * (x_l + x_u);
        let tol = 1.0e-06 * f64::abs(x_m) + 1.0e-10;

        if f64::abs(stored_step) - tol > -2.0 * f64::EPSILON {
            let c3 = (x_m - x_small) * (f_m - f_prev_small);
            let c2 = (x_m - x_prev_small) * (f_m - f_small);
            let mut c1 = (x_m - x_prev_small) * c2 - (x_m - x_small) * c3;
            let mut c2 = 2.0 * (c2 - c3);

            if f64::abs(c2) > f64::EPSILON {
                if c2 > 0.0 {
                    c1 = -c1;
                }
                c2 = f64::abs(c2);
                quad_step_size = c1 / c2;
            } else {
                quad_step_size = stored_step;
            }
            prev_stored_step = stored_step;
            stored_step = step_size;
        }

        let x_trial = x_m + quad_step_size;
        if f64::abs(quad_step_size) < f64::abs(0.5 * prev_stored_step) 
            && x_trial > x_l && x_trial < x_u 
        {
            step_size = quad_step_size;
            if x_trial - x_l < 2.0 * tol || x_u - x_trial < 2.0 * tol {
                step_size = if x_midpoint >= x_m { 1.0 } else { -1.0 } * f64::abs(tol);
            }
        } else if (x_small != x_prev_small && x_small < x_m && x_prev_small < x_m)
            || (x_small != x_prev_small && x_small > x_m && x_prev_small > x_m)
        {
            let (mut outside_interval, mut inside_interval) = if x_small < x_m {
                (x_l - x_m, x_u - x_m)
            } else {
                (x_u - x_m, x_l - x_m)
            };

            if f64::abs(inside_interval) <= tol {
                std::mem::swap(&mut outside_interval, &mut inside_interval);
            }

            let step = inside_interval;
            let scale_factor = if f64::abs(outside_interval) < f64::abs(inside_interval) {
                0.5 * f64::sqrt(-outside_interval / inside_interval)
            } else {
                5.0 / 11.0 * (0.1 - inside_interval / outside_interval)
            };

            self.stored_step = step;
            step_size = scale_factor * step;
        } else {
            let step = if x_m < x_midpoint {
                x_u - x_m
            } else {
                x_l - x_m
            };
            self.stored_step = step;
            step_size = 0.3819660112501052 * step;
        }

        let x_eval = if f64::abs(step_size) > tol {
            x_m + step_size
        } else {
            x_m + if step_size >= 0.0 { 1.0 } else { -1.0 } * f64::abs(tol)
        };

        let f_eval = f.eval(x_eval);
        if !f_eval.is_finite() {
            return Err(GslError::BadFunc);
        }

        if f_eval <= f_m {
            if x_eval < x_m {
                *x_upper = x_m;
                *f_upper = f_m;
            } else {
                *x_lower = x_m;
                *f_lower = f_m;
            }
            self.x_prev_small = x_small;
            self.f_prev_small = f_small;
            self.x_small = x_m;
            self.f_small = f_m;
            *x_minimum = x_eval;
            *f_minimum = f_eval;
        } else {
            if x_eval < x_m {
                *x_lower = x_eval;
                *f_lower = f_eval;
            } else {
                *x_upper = x_eval;
                *f_upper = f_eval;
            }

            if f_eval <= f_small || f64::abs(x_small - x_m) < 2.0 * f64::EPSILON {
                self.x_prev_small = x_small;
                self.f_prev_small = f_small;
                self.x_small = x_eval;
                self.f_small = f_eval;
            } else if f_eval <= f_prev_small
                || f64::abs(x_prev_small - x_m) < 2.0 * f64::EPSILON
                || f64::abs(x_prev_small - x_small) < 2.0 * f64::EPSILON
            {
                self.x_prev_small = x_eval;
                self.f_prev_small = f_eval;
            }
        }

        self.stored_step = stored_step;
        self.prev_stored_step = prev_stored_step;
        self.step_size = step_size;
        self.num_iter += 1;

        Ok(())
    }
}

pub struct GslMinFminimizerQuadGolden {
    state: QuadGoldenState,
}

impl GslMinFminimizerQuadGolden {
    pub fn new() -> Self {
        GslMinFminimizerQuadGolden {
            state: QuadGoldenState::new(),
        }
    }

    pub fn init(
        &mut self,
        f: GslFunction,
        x_minimum: f64,
        f_minimum: f64,
        x_lower: f64,
        f_lower: f64,
        x_upper: f64,
        f_upper: f64,
    ) -> GslResult {
        self.state.init(x_minimum, f_minimum)
    }

    pub fn iterate(
        &mut self,
        f: &GslFunction,
        x_minimum: &mut f64,
        f_minimum: &mut f64,
        x_lower: &mut f64,
        f_lower: &mut f64,
        x_upper: &mut f64,
        f_upper: &mut f64,
    ) -> GslResult {
        self.state.iterate(f, x_minimum, f_minimum, x_lower, f_lower, x_upper, f_upper)
    }
}