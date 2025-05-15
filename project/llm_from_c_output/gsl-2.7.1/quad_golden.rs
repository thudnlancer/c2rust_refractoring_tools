use std::f64;

const REL_ERR_VAL: f64 = 1.0e-06;
const ABS_ERR_VAL: f64 = 1.0e-10;
const GOLDEN_MEAN: f64 = 0.3819660112501052; // (3 - sqrt(5))/2
const GOLDEN_RATIO: f64 = 1.6180339887498950; // (1 + sqrt(5))/2

struct QuadGoldenState {
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
    fn new() -> Self {
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
}

struct GslFunction {
    function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    fn new<F: 'static + Fn(f64) -> f64>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }

    fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

#[derive(Debug)]
enum GslError {
    Success,
    Failure,
}

fn quad_golden_init(
    state: &mut QuadGoldenState,
    _f: &GslFunction,
    x_minimum: f64,
    f_minimum: f64,
    _x_lower: f64,
    _f_lower: f64,
    _x_upper: f64,
    _f_upper: f64,
) -> GslError {
    state.x_prev_small = x_minimum;
    state.x_small = x_minimum;

    state.f_prev_small = f_minimum;
    state.f_small = f_minimum;

    state.step_size = 0.0;
    state.stored_step = 0.0;
    state.prev_stored_step = 0.0;
    state.num_iter = 0;

    GslError::Success
}

fn quad_golden_iterate(
    state: &mut QuadGoldenState,
    f: &GslFunction,
    x_minimum: &mut f64,
    f_minimum: &mut f64,
    x_lower: &mut f64,
    f_lower: &mut f64,
    x_upper: &mut f64,
    f_upper: &mut f64,
) -> GslError {
    let x_m = *x_minimum;
    let f_m = *f_minimum;

    let x_l = *x_lower;
    let x_u = *x_upper;

    let x_small = state.x_small;
    let f_small = state.f_small;

    let x_prev_small = state.x_prev_small;
    let f_prev_small = state.f_prev_small;

    let mut stored_step = state.stored_step;
    let mut prev_stored_step = state.prev_stored_step;
    let mut step_size = state.step_size;

    let mut quad_step_size = prev_stored_step;

    let x_trial;
    let x_eval;
    let f_eval;

    let x_midpoint = 0.5 * (x_l + x_u);
    let tol = REL_ERR_VAL * x_m.abs() + ABS_ERR_VAL;

    if stored_step.abs() - tol > -2.0 * f64::EPSILON {
        let c3 = (x_m - x_small) * (f_m - f_prev_small);
        let c2 = (x_m - x_prev_small) * (f_m - f_small);
        let mut c1 = (x_m - x_prev_small) * c2 - (x_m - x_small) * c3;

        let mut c2 = 2.0 * (c2 - c3);

        if c2.abs() > f64::EPSILON {
            if c2 > 0.0 {
                c1 = -c1;
            }

            c2 = c2.abs();
            quad_step_size = c1 / c2;
        } else {
            quad_step_size = stored_step;
        }

        prev_stored_step = stored_step;
        stored_step = step_size;
    }

    x_trial = x_m + quad_step_size;

    if quad_step_size.abs() < (0.5 * prev_stored_step).abs()
        && x_trial > x_l
        && x_trial < x_u
    {
        step_size = quad_step_size;

        if (x_trial - x_l) < 2.0 * tol || (x_u - x_trial) < 2.0 * tol {
            step_size = if x_midpoint >= x_m { tol } else { -tol };
        }
    } else if (x_small != x_prev_small && x_small < x_m && x_prev_small < x_m)
        || (x_small != x_prev_small && x_small > x_m && x_prev_small > x_m)
    {
        let (outside_interval, inside_interval) = if x_small < x_m {
            (x_l - x_m, x_u - x_m)
        } else {
            (x_u - x_m, x_l - x_m)
        };

        let (outside_interval, inside_interval) = if inside_interval.abs() <= tol {
            (inside_interval, outside_interval)
        } else {
            (outside_interval, inside_interval)
        };

        let step = inside_interval;
        let scale_factor = if outside_interval.abs() < inside_interval.abs() {
            0.5 * (-outside_interval / inside_interval).sqrt()
        } else {
            (5.0 / 11.0) * (0.1 - inside_interval / outside_interval)
        };

        stored_step = step;
        step_size = scale_factor * step;
    } else {
        let step = if x_m < x_midpoint {
            x_u - x_m
        } else {
            x_l - x_m
        };

        stored_step = step;
        step_size = GOLDEN_MEAN * step;
    }

    x_eval = if step_size.abs() > tol {
        x_m + step_size
    } else {
        x_m + if step_size >= 0.0 { tol } else { -tol }
    };

    f_eval = f.eval(x_eval);

    if f_eval <= f_m {
        if x_eval < x_m {
            *x_upper = x_m;
            *f_upper = f_m;
        } else {
            *x_lower = x_m;
            *f_upper = f_m;
        }

        state.x_prev_small = x_small;
        state.f_prev_small = f_small;

        state.x_small = x_m;
        state.f_small = f_m;

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

        if f_eval <= f_small || (x_small - x_m).abs() < 2.0 * f64::EPSILON {
            state.x_prev_small = x_small;
            state.f_prev_small = f_small;

            state.x_small = x_eval;
            state.f_small = f_eval;
        } else if f_eval <= f_prev_small
            || (x_prev_small - x_m).abs() < 2.0 * f64::EPSILON
            || (x_prev_small - x_small).abs() < 2.0 * f64::EPSILON
        {
            state.x_prev_small = x_eval;
            state.f_prev_small = f_eval;
        }
    }

    state.stored_step = stored_step;
    state.prev_stored_step = prev_stored_step;
    state.step_size = step_size;
    state.num_iter += 1;

    GslError::Success
}

struct QuadGoldenMinimizer {
    state: QuadGoldenState,
    function: Option<GslFunction>,
}

impl QuadGoldenMinimizer {
    fn new() -> Self {
        QuadGoldenMinimizer {
            state: QuadGoldenState::new(),
            function: None,
        }
    }

    fn init(
        &mut self,
        f: GslFunction,
        x_minimum: f64,
        f_minimum: f64,
        x_lower: f64,
        f_lower: f64,
        x_upper: f64,
        f_upper: f64,
    ) -> Result<(), GslError> {
        self.function = Some(f);
        quad_golden_init(
            &mut self.state,
            self.function.as_ref().unwrap(),
            x_minimum,
            f_minimum,
            x_lower,
            f_lower,
            x_upper,
            f_upper,
        )
    }

    fn iterate(
        &mut self,
        x_minimum: &mut f64,
        f_minimum: &mut f64,
        x_lower: &mut f64,
        f_lower: &mut f64,
        x_upper: &mut f64,
        f_upper: &mut f64,
    ) -> Result<(), GslError> {
        quad_golden_iterate(
            &mut self.state,
            self.function.as_ref().unwrap(),
            x_minimum,
            f_minimum,
            x_lower,
            f_lower,
            x_upper,
            f_upper,
        )
    }
}

const QUAD_GOLDEN: &str = "quad-golden";