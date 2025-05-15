/* min/brent.rs

 * Translated from C to Rust while maintaining the same functionality
 * and following Rust's safety practices.
 */

use std::f64;

/// Brent's minimization algorithm state
struct BrentState {
    d: f64,
    e: f64,
    v: f64,
    w: f64,
    f_v: f64,
    f_w: f64,
}

/// Function type similar to GSL's gsl_function
pub trait GslFunction {
    fn evaluate(&self, x: f64) -> f64;
}

/// Result type for the minimization operations
#[derive(Debug, PartialEq)]
pub enum MinResult {
    Success,
    Failure,
}

const GOLDEN_RATIO: f64 = 0.3819660; // (3 - sqrt(5))/2
const SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-8; // sqrt(f64::EPSILON)

impl BrentState {
    /// Initialize the Brent minimizer state
    pub fn init<F: GslFunction>(
        &mut self,
        f: &F,
        x_minimum: f64,
        f_minimum: f64,
        x_lower: f64,
        f_lower: f64,
        x_upper: f64,
        f_upper: f64,
    ) -> MinResult {
        // Avoid unused variable warnings (keeping same behavior as C code)
        let _ = (x_minimum, f_minimum, f_lower, f_upper);

        let v = x_lower + GOLDEN_RATIO * (x_upper - x_lower);
        let w = v;

        self.v = v;
        self.w = w;
        self.d = 0.0;
        self.e = 0.0;

        let f_vw = f.evaluate(v);
        self.f_v = f_vw;
        self.f_w = f_vw;

        MinResult::Success
    }

    /// Perform one iteration of Brent's minimization algorithm
    pub fn iterate<F: GslFunction>(
        &mut self,
        f: &F,
        x_minimum: &mut f64,
        f_minimum: &mut f64,
        x_lower: &mut f64,
        f_lower: &mut f64,
        x_upper: &mut f64,
        f_upper: &mut f64,
    ) -> MinResult {
        let x_left = *x_lower;
        let x_right = *x_upper;
        let z = *x_minimum;

        let mut d = self.e;
        let mut e = self.d;
        let v = self.v;
        let w = self.w;
        let f_v = self.f_v;
        let f_w = self.f_w;
        let f_z = *f_minimum;

        let w_lower = z - x_left;
        let w_upper = x_right - z;
        let tolerance = SQRT_DBL_EPSILON * z.abs();
        let midpoint = 0.5 * (x_left + x_right);

        let mut p = 0.0;
        let mut q = 0.0;
        let mut r;

        if e.abs() > tolerance {
            // Fit parabola
            r = (z - w) * (f_z - f_v);
            q = (z - v) * (f_z - f_w);
            p = (z - v) * q - (z - w) * r;
            q = 2.0 * (q - r);

            if q > 0.0 {
                p = -p;
            } else {
                q = -q;
            }

            r = e;
            e = d;
        }

        let u = if p.abs() < (0.5 * q * r).abs() && p < q * w_lower && p < q * w_upper {
            let t2 = 2.0 * tolerance;
            d = p / q;
            let mut u = z + d;

            if (u - x_left) < t2 || (x_right - u) < t2 {
                d = if z < midpoint { tolerance } else { -tolerance };
                u = z + d;
            }
            u
        } else {
            e = if z < midpoint {
                x_right - z
            } else {
                -(z - x_left)
            };
            d = GOLDEN_RATIO * e;
            z + d
        };

        let u = if d.abs() >= tolerance {
            z + d
        } else {
            z + if d > 0.0 { tolerance } else { -tolerance }
        };

        self.e = e;
        self.d = d;

        let f_u = f.evaluate(u);

        if f_u <= f_z {
            if u < z {
                *x_upper = z;
                *f_upper = f_z;
            } else {
                *x_lower = z;
                *f_lower = f_z;
            }

            self.v = w;
            self.f_v = f_w;
            self.w = z;
            self.f_w = f_z;
            *x_minimum = u;
            *f_minimum = f_u;
        } else {
            if u < z {
                *x_lower = u;
                *f_lower = f_u;
            } else {
                *x_upper = u;
                *f_upper = f_u;
            }

            if f_u <= f_w || (w - z).abs() < f64::EPSILON {
                self.v = w;
                self.f_v = f_w;
                self.w = u;
                self.f_w = f_u;
            } else if f_u <= f_v
                || (v - z).abs() < f64::EPSILON
                || (v - w).abs() < f64::EPSILON
            {
                self.v = u;
                self.f_v = f_u;
            }
        }

        MinResult::Success
    }
}

/// Brent minimizer type similar to GSL's interface
pub struct BrentMinimizer {
    state: BrentState,
}

impl BrentMinimizer {
    pub fn new() -> Self {
        BrentMinimizer {
            state: BrentState {
                d: 0.0,
                e: 0.0,
                v: 0.0,
                w: 0.0,
                f_v: 0.0,
                f_w: 0.0,
            },
        }
    }

    pub fn init<F: GslFunction>(
        &mut self,
        f: &F,
        x_minimum: f64,
        f_minimum: f64,
        x_lower: f64,
        f_lower: f64,
        x_upper: f64,
        f_upper: f64,
    ) -> MinResult {
        self.state.init(f, x_minimum, f_minimum, x_lower, f_lower, x_upper, f_upper)
    }

    pub fn iterate<F: GslFunction>(
        &mut self,
        f: &F,
        x_minimum: &mut f64,
        f_minimum: &mut f64,
        x_lower: &mut f64,
        f_lower: &mut f64,
        x_upper: &mut f64,
        f_upper: &mut f64,
    ) -> MinResult {
        self.state.iterate(f, x_minimum, f_minimum, x_lower, f_lower, x_upper, f_upper)
    }
}