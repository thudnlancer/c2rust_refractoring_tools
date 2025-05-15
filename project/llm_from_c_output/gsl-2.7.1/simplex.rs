use std::f64;
use ndarray::{Array1, Array2, Axis};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimplexError {
    #[error("invalid number of parameters specified")]
    InvalidParameters,
    #[error("failed to allocate memory")]
    AllocationFailed,
    #[error("incompatible size")]
    IncompatibleSize,
    #[error("non-finite function value encountered")]
    NonFiniteFunctionValue,
    #[error("vector operation failed")]
    VectorOperationFailed,
    #[error("contraction failed")]
    ContractionFailed,
}

pub struct SimplexState {
    x1: Array2<f64>,  // simplex corner points
    y1: Array1<f64>,  // function value at corner points
    ws1: Array1<f64>, // workspace 1 for algorithm
    ws2: Array1<f64>, // workspace 2 for algorithm
}

pub trait MultiminFunction {
    fn eval(&self, x: &Array1<f64>) -> f64;
}

impl SimplexState {
    fn move_corner(
        &self,
        coeff: f64,
        corner: usize,
        xc: &mut Array1<f64>,
        f: &impl MultiminFunction,
    ) -> f64 {
        let n = self.x1.shape()[1];
        for j in 0..n {
            let mp = self.x1.index_axis(Axis(0), corner)
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != corner)
                .map(|(_, &val)| val)
                .sum::<f64>() / (self.x1.shape()[0] - 1) as f64;
            let new_val = mp - coeff * (mp - self.x1[[corner, j]]);
            xc[j] = new_val;
        }
        f.eval(xc)
    }

    fn contract_by_best(
        &mut self,
        best: usize,
        xc: &mut Array1<f64>,
        f: &impl MultiminFunction,
    ) -> Result<(), SimplexError> {
        let mut status = Ok(());
        for i in 0..self.x1.shape()[0] {
            if i != best {
                for j in 0..self.x1.shape()[1] {
                    let new_val = 0.5 * (self.x1[[i, j]] + self.x1[[best, j]]);
                    self.x1[[i, j]] = new_val;
                }
                xc.assign(&self.x1.index_axis(Axis(0), i));
                let new_val = f.eval(xc);
                self.y1[i] = new_val;
                if !new_val.is_finite() {
                    status = Err(SimplexError::NonFiniteFunctionValue);
                }
            }
        }
        status
    }

    fn calc_center(&self, mp: &mut Array1<f64>) {
        let n = self.x1.shape()[1];
        for j in 0..n {
            let val = self.x1.index_axis(Axis(1), j).sum() / self.x1.shape()[0] as f64;
            mp[j] = val;
        }
    }

    fn size(&mut self) -> f64 {
        let mut ss = 0.0;
        self.calc_center(&mut self.ws2);
        for i in 0..self.x1.shape()[0] {
            self.ws1.assign(&self.x1.index_axis(Axis(0), i));
            self.ws1 -= &self.ws2;
            ss += self.ws1.dot(&self.ws1).sqrt();
        }
        ss / self.x1.shape()[0] as f64
    }

    pub fn new(n: usize) -> Result<Self, SimplexError> {
        if n == 0 {
            return Err(SimplexError::InvalidParameters);
        }
        Ok(SimplexState {
            x1: Array2::zeros((n + 1, n)),
            y1: Array1::zeros(n + 1),
            ws1: Array1::zeros(n),
            ws2: Array1::zeros(n),
        })
    }

    pub fn set(
        &mut self,
        f: &impl MultiminFunction,
        x: &Array1<f64>,
        step_size: &Array1<f64>,
    ) -> Result<f64, SimplexError> {
        if self.ws1.len() != x.len() {
            return Err(SimplexError::IncompatibleSize);
        }
        if self.ws1.len() != step_size.len() {
            return Err(SimplexError::IncompatibleSize);
        }

        // First point is the original x
        let val = f.eval(x);
        if !val.is_finite() {
            return Err(SimplexError::NonFiniteFunctionValue);
        }
        self.x1.index_axis_mut(Axis(0), 0).assign(x);
        self.y1[0] = val;

        // Following points are initialized to x + step_size
        for i in 0..x.len() {
            self.ws1.assign(x);
            self.ws1[i] += step_size[i];
            let val = f.eval(&self.ws1);
            if !val.is_finite() {
                return Err(SimplexError::NonFiniteFunctionValue);
            }
            self.x1.index_axis_mut(Axis(0), i + 1).assign(&self.ws1);
            self.y1[i + 1] = val;
        }

        // Initialize simplex size
        Ok(self.size())
    }

    pub fn iterate(
        &mut self,
        f: &impl MultiminFunction,
        x: &mut Array1<f64>,
    ) -> Result<(f64, f64), SimplexError> {
        if self.ws1.len() != x.len() {
            return Err(SimplexError::IncompatibleSize);
        }

        let n = self.y1.len();
        let mut hi = 0;
        let mut s_hi = 1;
        let mut lo = 0;
        let mut dhi = self.y1[0];
        let mut ds_hi = self.y1[1];
        let mut dlo = self.y1[0];

        // Get indices of highest, second highest and lowest points
        for i in 1..n {
            let val = self.y1[i];
            if val < dlo {
                dlo = val;
                lo = i;
            } else if val > dhi {
                ds_hi = dhi;
                s_hi = hi;
                dhi = val;
                hi = i;
            } else if val > ds_hi {
                ds_hi = val;
                s_hi = i;
            }
        }

        // Reflect the highest value
        let val = self.move_corner(-1.0, hi, &mut self.ws1, f);

        if val.is_finite() && val < self.y1[lo] {
            // Reflected point becomes lowest point, try expansion
            let val2 = self.move_corner(-2.0, hi, &mut self.ws2, f);
            if val2.is_finite() && val2 < self.y1[lo] {
                self.x1.index_axis_mut(Axis(0), hi).assign(&self.ws2);
                self.y1[hi] = val2;
            } else {
                self.x1.index_axis_mut(Axis(0), hi).assign(&self.ws1);
                self.y1[hi] = val;
            }
        } else if !val.is_finite() || val > self.y1[s_hi] {
            if val.is_finite() && val <= self.y1[hi] {
                // If trial point is better than highest point, replace it
                self.x1.index_axis_mut(Axis(0), hi).assign(&self.ws1);
                self.y1[hi] = val;
            }

            // Try one dimensional contraction
            let val2 = self.move_corner(0.5, hi, &mut self.ws2, f);
            if val2.is_finite() && val2 <= self.y1[hi] {
                self.x1.index_axis_mut(Axis(0), hi).assign(&self.ws2);
                self.y1[hi] = val2;
            } else {
                // Contract the whole simplex in respect to the best point
                self.contract_by_best(lo, &mut self.ws1, f)?;
            }
        } else {
            // Trial point is better than second highest point
            self.x1.index_axis_mut(Axis(0), hi).assign(&self.ws1);
            self.y1[hi] = val;
        }

        // Return lowest point of simplex as x
        lo = self.y1.argmin();
        x.assign(&self.x1.index_axis(Axis(0), lo));
        let fval = self.y1[lo];
        let size = self.size();

        Ok((size, fval))
    }
}