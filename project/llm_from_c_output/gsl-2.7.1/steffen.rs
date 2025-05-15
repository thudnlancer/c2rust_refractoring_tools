use std::cmp::Ordering;
use std::f64;
use std::ops::{Add, Sub, Mul, Div};

struct SteffenState {
    a: Vec<f64>,       // eqs 2-5 of paper
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
    y_prime: Vec<f64>, // eq 11 of paper
}

impl SteffenState {
    fn new(size: usize) -> Result<Self, &'static str> {
        Ok(Self {
            a: vec![0.0; size],
            b: vec![0.0; size],
            c: vec![0.0; size],
            d: vec![0.0; size],
            y_prime: vec![0.0; size],
        })
    }

    fn init(&mut self, x_array: &[f64], y_array: &[f64]) -> Result<(), &'static str> {
        let size = x_array.len();
        if size < 2 {
            return Err("At least 2 points required for interpolation");
        }

        // First assign the interval and slopes for the left boundary
        let h0 = x_array[1] - x_array[0];
        let s0 = (y_array[1] - y_array[0]) / h0;
        self.y_prime[0] = s0;

        // Calculate all necessary s, h, p, and y' variables from 1 to N-2
        for i in 1..size-1 {
            // Equation 6 in the paper
            let hi = x_array[i+1] - x_array[i];
            let him1 = x_array[i] - x_array[i-1];

            // Equation 7 in the paper
            let si = (y_array[i+1] - y_array[i]) / hi;
            let sim1 = (y_array[i] - y_array[i-1]) / him1;

            // Equation 8 in the paper
            let pi = (sim1 * hi + si * him1) / (him1 + hi);

            // Equivalent of FORTRAN statement below eqn 11
            self.y_prime[i] = (steffen_copysign(1.0, sim1) + steffen_copysign(1.0, si))
                * sim1.abs().min(si.abs().min(0.5 * pi.abs()));
        }

        // y' for the rightmost boundary
        self.y_prime[size-1] = (y_array[size-1] - y_array[size-2]) 
            / (x_array[size-1] - x_array[size-2]);

        // Calculate all coefficients for the whole range
        for i in 0..size-1 {
            let hi = x_array[i+1] - x_array[i];
            let si = (y_array[i+1] - y_array[i]) / hi;

            // Equations 2-5 in the paper
            self.a[i] = (self.y_prime[i] + self.y_prime[i+1] - 2.0 * si) / (hi * hi);
            self.b[i] = (3.0 * si - 2.0 * self.y_prime[i] - self.y_prime[i+1]) / hi;
            self.c[i] = self.y_prime[i];
            self.d[i] = y_array[i];
        }

        Ok(())
    }

    fn eval(&self, x_array: &[f64], x: f64, acc: Option<usize>) -> Result<f64, &'static str> {
        let index = self.find_index(x_array, x, acc)?;
        let x_lo = x_array[index];
        let delx = x - x_lo;
        let a = self.a[index];
        let b = self.b[index];
        let c = self.c[index];
        let d = self.d[index];
        Ok(d + delx * (c + delx * (b + delx * a)))
    }

    fn eval_deriv(&self, x_array: &[f64], x: f64, acc: Option<usize>) -> Result<f64, &'static str> {
        let index = self.find_index(x_array, x, acc)?;
        let x_lo = x_array[index];
        let delx = x - x_lo;
        let a = self.a[index];
        let b = self.b[index];
        let c = self.c[index];
        Ok(c + delx * (2.0 * b + delx * 3.0 * a))
    }

    fn eval_deriv2(&self, x_array: &[f64], x: f64, acc: Option<usize>) -> Result<f64, &'static str> {
        let index = self.find_index(x_array, x, acc)?;
        let x_lo = x_array[index];
        let delx = x - x_lo;
        let a = self.a[index];
        let b = self.b[index];
        Ok(6.0 * a * delx + 2.0 * b)
    }

    fn eval_integ(
        &self,
        x_array: &[f64],
        a: f64,
        b: f64,
        acc_a: Option<usize>,
        acc_b: Option<usize>,
    ) -> Result<f64, &'static str> {
        let index_a = self.find_index(x_array, a, acc_a)?;
        let index_b = self.find_index(x_array, b, acc_b)?;

        let mut result = 0.0;

        for i in index_a..=index_b {
            let x_hi = x_array[i + 1];
            let x_lo = x_array[i];
            let dx = x_hi - x_lo;

            if dx == 0.0 {
                return Err("Consecutive x values in data");
            }

            let x1 = if i == index_a { a - x_lo } else { 0.0 };
            let x2 = if i == index_b { b - x_lo } else { x_hi - x_lo };

            result += (1.0 / 4.0) * self.a[i] * (x2.powi(4) - x1.powi(4))
                + (1.0 / 3.0) * self.b[i] * (x2.powi(3) - x1.powi(3))
                + (1.0 / 2.0) * self.c[i] * (x2.powi(2) - x1.powi(2))
                + self.d[i] * (x2 - x1);
        }

        Ok(result)
    }

    fn find_index(
        &self,
        x_array: &[f64],
        x: f64,
        acc: Option<usize>,
    ) -> Result<usize, &'static str> {
        if let Some(idx) = acc {
            if idx < x_array.len() {
                return Ok(idx);
            }
        }
        match x_array.binary_search_by(|v| v.partial_cmp(&x).unwrap_or(Ordering::Less)) {
            Ok(idx) => Ok(idx),
            Err(idx) => {
                if idx == 0 {
                    Ok(0)
                } else if idx == x_array.len() {
                    Ok(x_array.len() - 1)
                } else {
                    Ok(idx - 1)
                }
            }
        }
    }
}

fn steffen_copysign(x: f64, y: f64) -> f64 {
    if (x < 0.0 && y > 0.0) || (x > 0.0 && y < 0.0) {
        -x
    } else {
        x
    }
}

pub struct SteffenInterpolator {
    state: SteffenState,
    x_array: Vec<f64>,
    y_array: Vec<f64>,
}

impl SteffenInterpolator {
    pub fn new(x_array: Vec<f64>, y_array: Vec<f64>) -> Result<Self, &'static str> {
        if x_array.len() != y_array.len() {
            return Err("x and y arrays must have same length");
        }
        if x_array.len() < 2 {
            return Err("At least 2 points required for interpolation");
        }
        let mut state = SteffenState::new(x_array.len())?;
        state.init(&x_array, &y_array)?;
        Ok(Self {
            state,
            x_array,
            y_array,
        })
    }

    pub fn eval(&self, x: f64, acc: Option<usize>) -> Result<f64, &'static str> {
        self.state.eval(&self.x_array, x, acc)
    }

    pub fn eval_deriv(&self, x: f64, acc: Option<usize>) -> Result<f64, &'static str> {
        self.state.eval_deriv(&self.x_array, x, acc)
    }

    pub fn eval_deriv2(&self, x: f64, acc: Option<usize>) -> Result<f64, &'static str> {
        self.state.eval_deriv2(&self.x_array, x, acc)
    }

    pub fn eval_integ(&self, a: f64, b: f64, acc_a: Option<usize>, acc_b: Option<usize>) -> Result<f64, &'static str> {
        self.state.eval_integ(&self.x_array, a, b, acc_a, acc_b)
    }
}