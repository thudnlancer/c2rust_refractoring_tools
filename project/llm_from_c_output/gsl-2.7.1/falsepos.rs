/* roots/falsepos.rs

   The false position algorithm uses bracketing by linear interpolation.

   If a linear interpolation step would decrease the size of the
   bracket by less than a bisection step would then the algorithm
   takes a bisection step instead.
   
   The last linear interpolation estimate of the root is used. If a
   bisection step causes it to fall outside the brackets then it is
   replaced by the bisection estimate (x_upper + x_lower)/2.
*/

use std::f64;

#[derive(Debug)]
struct FalsePosState {
    f_lower: f64,
    f_upper: f64,
}

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

#[derive(Debug)]
pub enum GslError {
    EndpointsNotStraddleY0,
}

pub type GslResult<T> = Result<T, GslError>;

pub struct FalsePosSolver {
    state: FalsePosState,
}

impl FalsePosSolver {
    pub fn new() -> Self {
        FalsePosSolver {
            state: FalsePosState {
                f_lower: 0.0,
                f_upper: 0.0,
            },
        }
    }

    pub fn init(
        &mut self,
        f: &GslFunction,
        root: &mut f64,
        x_lower: f64,
        x_upper: f64,
    ) -> GslResult<()> {
        *root = 0.5 * (x_lower + x_upper);

        let f_lower = f.eval(x_lower);
        let f_upper = f.eval(x_upper);

        self.state.f_lower = f_lower;
        self.state.f_upper = f_upper;

        if (f_lower < 0.0 && f_upper < 0.0) || (f_lower > 0.0 && f_upper > 0.0) {
            Err(GslError::EndpointsNotStraddleY0)
        } else {
            Ok(())
        }
    }

    pub fn iterate(
        &mut self,
        f: &GslFunction,
        root: &mut f64,
        x_lower: &mut f64,
        x_upper: &mut f64,
    ) -> GslResult<()> {
        let x_left = *x_lower;
        let x_right = *x_upper;

        let f_lower = self.state.f_lower;
        let f_upper = self.state.f_upper;

        if f_lower == 0.0 {
            *root = x_left;
            *x_upper = x_left;
            return Ok(());
        }

        if f_upper == 0.0 {
            *root = x_right;
            *x_lower = x_right;
            return Ok(());
        }

        // Linear interpolation
        let x_linear = x_right - (f_upper * (x_left - x_right) / (f_lower - f_upper));
        let f_linear = f.eval(x_linear);

        if f_linear == 0.0 {
            *root = x_linear;
            *x_lower = x_linear;
            *x_upper = x_linear;
            return Ok(());
        }

        // Update interval
        let w = if (f_lower > 0.0 && f_linear < 0.0) || (f_lower < 0.0 && f_linear > 0.0) {
            *root = x_linear;
            *x_upper = x_linear;
            self.state.f_upper = f_linear;
            x_linear - x_left
        } else {
            *root = x_linear;
            *x_lower = x_linear;
            self.state.f_lower = f_linear;
            x_right - x_linear
        };

        if w < 0.5 * (x_right - x_left) {
            return Ok(());
        }

        // Bisection step
        let x_bisect = 0.5 * (x_left + x_right);
        let f_bisect = f.eval(x_bisect);

        if (f_lower > 0.0 && f_bisect < 0.0) || (f_lower < 0.0 && f_bisect > 0.0) {
            *x_upper = x_bisect;
            self.state.f_upper = f_bisect;
            if *root > x_bisect {
                *root = 0.5 * (x_left + x_bisect);
            }
        } else {
            *x_lower = x_bisect;
            self.state.f_lower = f_bisect;
            if *root < x_bisect {
                *root = 0.5 * (x_bisect + x_right);
            }
        }

        Ok(())
    }
}

pub struct FalsePosSolverType {
    pub name: &'static str,
    pub size: usize,
}

pub const FALSEPOS_SOLVER_TYPE: FalsePosSolverType = FalsePosSolverType {
    name: "falsepos",
    size: std::mem::size_of::<FalsePosState>(),
};