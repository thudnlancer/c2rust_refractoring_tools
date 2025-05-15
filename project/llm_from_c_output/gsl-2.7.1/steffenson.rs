/* roots/steffenson.rs

   This is Newton's method with an Aitken "delta-squared"
   acceleration of the iterates. This can improve the convergence on
   multiple roots where the ordinary Newton algorithm is slow.

   x[i+1] = x[i] - f(x[i]) / f'(x[i])

   x_accelerated[i] = x[i] - (x[i+1] - x[i])**2 / (x[i+2] - 2*x[i+1] + x[i])

   We can only use the accelerated estimate after three iterations,
   and use the unaccelerated value until then.
*/

use std::f64;

#[derive(Debug)]
struct SteffensonState {
    f: f64,
    df: f64,
    x: f64,
    x_1: f64,
    x_2: f64,
    count: i32,
}

pub struct FunctionFdf {
    f: Box<dyn Fn(f64) -> f64>,
    df: Box<dyn Fn(f64) -> f64>,
}

impl FunctionFdf {
    pub fn eval_f(&self, x: f64) -> f64 {
        (self.f)(x)
    }

    pub fn eval_df(&self, x: f64) -> f64 {
        (self.df)(x)
    }

    pub fn eval_f_df(&self, x: f64) -> (f64, f64) {
        ((self.f)(x), (self.df)(x))
    }
}

pub struct SteffensonSolver {
    state: SteffensonState,
}

impl SteffensonSolver {
    pub fn new() -> Self {
        SteffensonSolver {
            state: SteffensonState {
                f: 0.0,
                df: 0.0,
                x: 0.0,
                x_1: 0.0,
                x_2: 0.0,
                count: 0,
            },
        }
    }

    pub fn init(&mut self, fdf: &FunctionFdf, root: f64) -> Result<(), &'static str> {
        self.state.f = fdf.eval_f(root);
        self.state.df = fdf.eval_df(root);
        self.state.x = root;
        self.state.x_1 = 0.0;
        self.state.x_2 = 0.0;
        self.state.count = 1;
        Ok(())
    }

    pub fn iterate(
        &mut self,
        fdf: &FunctionFdf,
        root: &mut f64,
    ) -> Result<(), &'static str> {
        let x_1 = self.state.x_1;
        let x = self.state.x;

        if self.state.df == 0.0 {
            return Err("derivative is zero");
        }

        let x_new = x - (self.state.f / self.state.df);
        let (f_new, df_new) = fdf.eval_f_df(x_new);

        self.state.x_2 = x_1;
        self.state.x_1 = x;
        self.state.x = x_new;
        self.state.f = f_new;
        self.state.df = df_new;

        if !f_new.is_finite() {
            return Err("function value is not finite");
        }

        if self.state.count < 3 {
            *root = x_new;
            self.state.count += 1;
        } else {
            let u = x - x_1;
            let v = x_new - 2.0 * x + x_1;

            if v == 0.0 {
                *root = x_new; // avoid division by zero
            } else {
                *root = x_1 - u * u / v; // accelerated value
            }
        }

        if !df_new.is_finite() {
            return Err("derivative value is not finite");
        }

        Ok(())
    }
}

pub const STEFFENSON_SOLVER: SteffensonSolver = SteffensonSolver {
    state: SteffensonState {
        f: 0.0,
        df: 0.0,
        x: 0.0,
        x_1: 0.0,
        x_2: 0.0,
        count: 0,
    },
};