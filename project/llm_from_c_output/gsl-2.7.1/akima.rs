use std::f64;
use std::ptr;
use std::mem;

pub struct AkimaState {
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
    m: Vec<f64>,
}

pub struct AkimaInterp {
    state: AkimaState,
    periodic: bool,
}

#[derive(Debug)]
pub enum InterpError {
    MemoryAllocationFailed,
    InvalidInput,
}

impl AkimaInterp {
    pub fn new(size: usize) -> Result<Self, InterpError> {
        let state = AkimaState::new(size)?;
        Ok(Self {
            state,
            periodic: false,
        })
    }

    pub fn new_periodic(size: usize) -> Result<Self, InterpError> {
        let state = AkimaState::new(size)?;
        Ok(Self {
            state,
            periodic: true,
        })
    }

    pub fn init(&mut self, x: &[f64], y: &[f64]) -> Result<(), InterpError> {
        if x.len() != y.len() {
            return Err(InterpError::InvalidInput);
        }
        
        if self.periodic {
            self.state.init_periodic(x, y)
        } else {
            self.state.init(x, y)
        }
    }

    pub fn eval(&self, x: &[f64], y: &[f64], x_val: f64) -> Result<f64, InterpError> {
        self.state.eval(x, y, x_val)
    }

    pub fn eval_deriv(&self, x: &[f64], x_val: f64) -> Result<f64, InterpError> {
        self.state.eval_deriv(x, x_val)
    }

    pub fn eval_deriv2(&self, x: &[f64], x_val: f64) -> Result<f64, InterpError> {
        self.state.eval_deriv2(x, x_val)
    }

    pub fn eval_integ(&self, x: &[f64], y: &[f64], a: f64, b: f64) -> Result<f64, InterpError> {
        self.state.eval_integ(x, y, a, b)
    }
}

impl AkimaState {
    fn new(size: usize) -> Result<Self, InterpError> {
        let b = vec![0.0; size];
        let c = vec![0.0; size];
        let d = vec![0.0; size];
        let m = vec![0.0; size + 4];

        Ok(Self { b, c, d, m })
    }

    fn init(&mut self, x: &[f64], y: &[f64]) -> Result<(), InterpError> {
        let size = x.len();
        if size < 2 {
            return Err(InterpError::InvalidInput);
        }

        let m = &mut self.m[2..]; // offset for -1,-2 components

        for i in 0..size-1 {
            m[i] = (y[i+1] - y[i]) / (x[i+1] - x[i]);
        }

        // non-periodic boundary conditions
        self.m[0] = 3.0 * m[0] - 2.0 * m[1];
        self.m[1] = 2.0 * m[0] - m[1];
        m[size-1] = 2.0 * m[size-2] - m[size-3];
        m[size] = 3.0 * m[size-2] - 2.0 * m[size-3];

        self.calc(x, size)
    }

    fn init_periodic(&mut self, x: &[f64], y: &[f64]) -> Result<(), InterpError> {
        let size = x.len();
        if size < 2 {
            return Err(InterpError::InvalidInput);
        }

        let m = &mut self.m[2..]; // offset for -1,-2 components

        for i in 0..size-1 {
            m[i] = (y[i+1] - y[i]) / (x[i+1] - x[i]);
        }

        // periodic boundary conditions
        self.m[0] = m[size-1-2];
        self.m[1] = m[size-1-1];
        m[size-1] = m[0];
        m[size] = m[1];

        self.calc(x, size)
    }

    fn calc(&mut self, x: &[f64], size: usize) -> Result<(), InterpError> {
        let m = &self.m[2..]; // offset for -1,-2 components

        for i in 0..size-1 {
            let ne = (m[i+1] - m[i]).abs() + (m[i-1] - m[i-2]).abs();
            if ne == 0.0 {
                self.b[i] = m[i];
                self.c[i] = 0.0;
                self.d[i] = 0.0;
            } else {
                let h_i = x[i+1] - x[i];
                let ne_next = (m[i+2] - m[i+1]).abs() + (m[i] - m[i-1]).abs();
                let alpha_i = (m[i-1] - m[i-2]).abs() / ne;
                
                let t_l_ip1 = if ne_next == 0.0 {
                    m[i]
                } else {
                    let alpha_ip1 = (m[i] - m[i-1]).abs() / ne_next;
                    (1.0 - alpha_ip1) * m[i] + alpha_ip1 * m[i+1]
                };

                self.b[i] = (1.0 - alpha_i) * m[i-1] + alpha_i * m[i];
                self.c[i] = (3.0 * m[i] - 2.0 * self.b[i] - t_l_ip1) / h_i;
                self.d[i] = (self.b[i] + t_l_ip1 - 2.0 * m[i]) / (h_i * h_i);
            }
        }

        Ok(())
    }

    fn eval(&self, x: &[f64], y: &[f64], x_val: f64) -> Result<f64, InterpError> {
        let index = self.find_index(x, x_val)?;
        let x_lo = x[index];
        let delx = x_val - x_lo;
        let b = self.b[index];
        let c = self.c[index];
        let d = self.d[index];
        Ok(y[index] + delx * (b + delx * (c + d * delx)))
    }

    fn eval_deriv(&self, x: &[f64], x_val: f64) -> Result<f64, InterpError> {
        let index = self.find_index(x, x_val)?;
        let x_lo = x[index];
        let delx = x_val - x_lo;
        let b = self.b[index];
        let c = self.c[index];
        let d = self.d[index];
        Ok(b + delx * (2.0 * c + 3.0 * d * delx))
    }

    fn eval_deriv2(&self, x: &[f64], x_val: f64) -> Result<f64, InterpError> {
        let index = self.find_index(x, x_val)?;
        let x_lo = x[index];
        let delx = x_val - x_lo;
        let c = self.c[index];
        let d = self.d[index];
        Ok(2.0 * c + 6.0 * d * delx)
    }

    fn eval_integ(&self, x: &[f64], y: &[f64], a: f64, b: f64) -> Result<f64, InterpError> {
        let index_a = self.find_index(x, a)?;
        let index_b = self.find_index(x, b)?;

        let mut result = 0.0;

        for i in index_a..=index_b {
            let x_hi = x[i + 1];
            let x_lo = x[i];
            let y_lo = y[i];
            let dx = x_hi - x_lo;
            if dx == 0.0 {
                return Err(InterpError::InvalidInput);
            }

            if i == index_a || i == index_b {
                let x1 = if i == index_a { a } else { x_lo };
                let x2 = if i == index_b { b } else { x_hi };
                result += self.integ_eval(y_lo, i, x_lo, x1, x2);
            } else {
                result += dx * (y_lo + dx * (0.5 * self.b[i] + 
                    dx * (self.c[i] / 3.0 + 0.25 * self.d[i] * dx)));
            }
        }

        Ok(result)
    }

    fn find_index(&self, x: &[f64], x_val: f64) -> Result<usize, InterpError> {
        // Simple binary search implementation
        let mut low = 0;
        let mut high = x.len() - 1;

        while high - low > 1 {
            let mid = (low + high) / 2;
            if x_val > x[mid] {
                low = mid;
            } else {
                high = mid;
            }
        }

        Ok(low)
    }

    fn integ_eval(&self, y_lo: f64, i: usize, x_lo: f64, x1: f64, x2: f64) -> f64 {
        let b = self.b[i];
        let c = self.c[i];
        let d = self.d[i];

        let dx1 = x1 - x_lo;
        let dx2 = x2 - x_lo;

        y_lo * (dx2 - dx1) + 
            b * (dx2.powi(2) - dx1.powi(2)) / 2.0 + 
            c * (dx2.powi(3) - dx1.powi(3)) / 3.0 + 
            d * (dx2.powi(4) - dx1.powi(4)) / 4.0
    }
}