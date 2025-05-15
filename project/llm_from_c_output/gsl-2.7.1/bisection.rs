/* roots/bisection.rs

   Translated from C to Rust while maintaining the same functionality
   and following Rust's safety practices.
*/

use std::f64;

/// Represents the state of the bisection solver
struct BisectionState {
    f_lower: f64,
    f_upper: f64,
}

/// Function type that the solver will operate on
type GslFunction = dyn Fn(f64) -> f64;

/// Possible error conditions
#[derive(Debug)]
enum GslError {
    EndpointsNotStraddlingZero,
}

impl std::fmt::Display for GslError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GslError::EndpointsNotStraddlingZero => 
                write!(f, "endpoints do not straddle y=0"),
        }
    }
}

impl std::error::Error for GslError {}

/// Result type for the bisection operations
type GslResult<T> = Result<T, GslError>;

/// Initializes the bisection solver
fn bisection_init(
    state: &mut BisectionState,
    f: &GslFunction,
    root: &mut f64,
    x_lower: f64,
    x_upper: f64,
) -> GslResult<()> {
    *root = 0.5 * (x_lower + x_upper);

    let f_lower = f(x_lower);
    let f_upper = f(x_upper);

    state.f_lower = f_lower;
    state.f_upper = f_upper;

    if (f_lower < 0.0 && f_upper < 0.0) || (f_lower > 0.0 && f_upper > 0.0) {
        Err(GslError::EndpointsNotStraddlingZero)
    } else {
        Ok(())
    }
}

/// Performs one iteration of the bisection solver
fn bisection_iterate(
    state: &mut BisectionState,
    f: &GslFunction,
    root: &mut f64,
    x_lower: &mut f64,
    x_upper: &mut f64,
) -> GslResult<()> {
    let x_left = *x_lower;
    let x_right = *x_upper;

    let f_lower = state.f_lower;
    let f_upper = state.f_upper;

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

    let x_bisect = (x_left + x_right) / 2.0;
    let f_bisect = f(x_bisect);

    if f_bisect == 0.0 {
        *root = x_bisect;
        *x_lower = x_bisect;
        *x_upper = x_bisect;
        return Ok(());
    }

    // Discard the half of the interval which doesn't contain the root
    if (f_lower > 0.0 && f_bisect < 0.0) || (f_lower < 0.0 && f_bisect > 0.0) {
        *root = 0.5 * (x_left + x_bisect);
        *x_upper = x_bisect;
        state.f_upper = f_bisect;
    } else {
        *root = 0.5 * (x_bisect + x_right);
        *x_lower = x_bisect;
        state.f_lower = f_bisect;
    }

    Ok(())
}

/// Bisection solver type
pub struct BisectionSolver {
    state: BisectionState,
}

impl BisectionSolver {
    /// Creates a new bisection solver
    pub fn new() -> Self {
        BisectionSolver {
            state: BisectionState {
                f_lower: 0.0,
                f_upper: 0.0,
            },
        }
    }

    /// Initializes the solver with the given function and interval
    pub fn init(
        &mut self,
        f: &GslFunction,
        root: &mut f64,
        x_lower: f64,
        x_upper: f64,
    ) -> GslResult<()> {
        bisection_init(&mut self.state, f, root, x_lower, x_upper)
    }

    /// Performs one iteration of the solver
    pub fn iterate(
        &mut self,
        f: &GslFunction,
        root: &mut f64,
        x_lower: &mut f64,
        x_upper: &mut f64,
    ) -> GslResult<()> {
        bisection_iterate(&mut self.state, f, root, x_lower, x_upper)
    }
}