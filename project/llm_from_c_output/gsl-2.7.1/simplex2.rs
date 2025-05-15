use std::f64;
use std::sync::atomic::{AtomicUsize, Ordering};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use ndarray_linalg::Norm;

#[derive(Debug)]
struct NMSimplexState {
    x1: Array2<f64>,       // simplex corner points
    y1: Array1<f64>,       // function value at corner points
    ws1: Array1<f64>,      // workspace 1 for algorithm
    ws2: Array1<f64>,      // workspace 2 for algorithm
    center: Array1<f64>,   // center of all points
    delta: Array1<f64>,    // current step
    xmc: Array1<f64>,      // x - center (workspace)
    s2: f64,              // squared size
    count: AtomicUsize,    // iteration count
}

type MultiminFunction = dyn Fn(&Array1<f64>) -> f64;

impl NMSimplexState {
    fn new(n: usize) -> Result<Self, &'static str> {
        if n == 0 {
            return Err("invalid number of parameters specified");
        }

        Ok(Self {
            x1: Array2::zeros((n + 1, n)),
            y1: Array1::zeros(n + 1),
            ws1: Array1::zeros(n),
            ws2: Array1::zeros(n),
            center: Array1::zeros(n),
            delta: Array1::zeros(n),
            xmc: Array1::zeros(n),
            s2: 0.0,
            count: AtomicUsize::new(0),
        })
    }

    fn compute_center(&self) -> Array1<f64> {
        let p = self.x1.shape()[0];
        let mut center = Array1::zeros(self.x1.shape()[1]);
        
        for i in 0..p {
            center += &self.x1.row(i);
        }
        
        center / (p as f64)
    }

    fn compute_size(&mut self, center: &Array1<f64>) -> f64 {
        let p = self.x1.shape()[0];
        let mut ss = 0.0;

        for i in 0..p {
            let diff = &self.x1.row(i) - center;
            let t = diff.norm();
            ss += t * t;
        }

        self.s2 = ss / (p as f64);
        (ss / (p as f64)).sqrt()
    }

    fn try_corner_move(
        &self,
        coeff: f64,
        corner: usize,
        xc: &mut Array1<f64>,
        f: &MultiminFunction,
    ) -> f64 {
        let p = self.x1.shape()[0];
        let alpha = (1.0 - coeff) * (p as f64) / ((p - 1) as f64);
        let beta = ((p as f64) * coeff - 1.0) / ((p - 1) as f64);

        *xc = &self.center * alpha + &self.x1.row(corner) * beta;
        f(xc)
    }

    fn update_point(
        &mut self,
        i: usize,
        x: &Array1<f64>,
        val: f64,
    ) {
        let p = self.x1.shape()[0];
        let x_orig = self.x1.row(i);

        // Compute delta = x - x_orig
        self.delta.assign(x);
        self.delta -= &x_orig;

        // Compute xmc = x_orig - c
        self.xmc.assign(&x_orig);
        self.xmc -= &self.center;

        // Update size: S2' = S2 + (2/P) * (x_orig - c).delta + (P-1)*(delta/P)^2
        let d = self.delta.norm();
        let xmcd = self.xmc.dot(&self.delta);
        self.s2 += (2.0 / (p as f64)) * xmcd + ((p - 1) as f64 / (p as f64)) * (d * d / (p as f64));

        // Update center: c' = c + (x - x_orig) / P
        let alpha = 1.0 / (p as f64);
        self.center.scaled_add(-alpha, &x_orig);
        self.center.scaled_add(alpha, x);

        self.x1.row_mut(i).assign(x);
        self.y1[i] = val;
    }

    fn contract_by_best(
        &mut self,
        best: usize,
        xc: &mut Array1<f64>,
        f: &MultiminFunction,
    ) -> Result<(), &'static str> {
        let p = self.x1.shape()[0];
        let mut status = Ok(());

        for i in 0..p {
            if i != best {
                for j in 0..self.x1.shape()[1] {
                    let newval = 0.5 * (self.x1[[i, j]] + self.x1[[best, j]]);
                    self.x1[[i, j]] = newval;
                }

                *xc = self.x1.row(i).to_owned();
                let newval = f(xc);
                self.y1[i] = newval;

                if !newval.is_finite() {
                    status = Err("non-finite function value encountered");
                }
            }
        }

        self.center = self.compute_center();
        self.compute_size(&self.center);

        status
    }

    fn set(
        &mut self,
        f: &MultiminFunction,
        x: &Array1<f64>,
        size: &mut f64,
        step_size: &Array1<f64>,
    ) -> Result<(), &'static str> {
        if self.ws1.len() != x.len() {
            return Err("incompatible size of x");
        }

        if self.ws1.len() != step_size.len() {
            return Err("incompatible size of step_size");
        }

        // first point is the original x0
        let val = f(x);
        if !val.is_finite() {
            return Err("non-finite function value encountered");
        }

        self.x1.row_mut(0).assign(x);
        self.y1[0] = val;

        // following points are initialized to x0 + step_size
        for i in 0..x.len() {
            self.ws1.assign(x);
            let xi = x[i];
            let si = step_size[i];
            self.ws1[i] = xi + si;

            let val = f(&self.ws1);
            if !val.is_finite() {
                return Err("non-finite function value encountered");
            }

            self.x1.row_mut(i + 1).assign(&self.ws1);
            self.y1[i + 1] = val;
        }

        self.center = self.compute_center();
        *size = self.compute_size(&self.center);
        self.count.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    fn set_rand(
        &mut self,
        f: &MultiminFunction,
        x: &Array1<f64>,
        size: &mut f64,
        step_size: &Array1<f64>,
    ) -> Result<(), &'static str> {
        if self.ws1.len() != x.len() {
            return Err("incompatible size of x");
        }

        if self.ws1.len() != step_size.len() {
            return Err("incompatible size of step_size");
        }

        // first point is the original x0
        let val = f(x);
        if !val.is_finite() {
            return Err("non-finite function value encountered");
        }

        self.x1.row_mut(0).assign(x);
        self.y1[0] = val;

        // generate a random orthonormal basis
        let mut seed = self.count.load(Ordering::Relaxed) ^ 0x12345678;
        
        // warm up the random number generator
        Self::ran_unif(&mut seed);

        let n = x.len();
        let mut m = self.x1.slice_mut(s![1.., ..]);
        m.diag_mut().fill(1.0);

        // random reflections
        for i in 0..n {
            let s = Self::ran_unif(&mut seed);
            if s > 0.5 {
                m[[i, i]] = -1.0;
            }
        }

        // random rotations
        for i in 0..n {
            for j in i + 1..n {
                let angle = 2.0 * f64::consts::PI * Self::ran_unif(&mut seed);
                let (c, s) = angle.sin_cos();
                
                let mut ci = m.column_mut(i);
                let mut cj = m.column_mut(j);
                
                for k in 0..n {
                    let temp = ci[k] * c - cj[k] * s;
                    cj[k] = ci[k] * s + cj[k] * c;
                    ci[k] = temp;
                }
            }
        }

        // scale by step_size and add to central point
        for i in 0..n {
            let xi = x[i];
            let si = step_size[i];
            let mut ci = m.column_mut(i);
            
            for j in 0..n {
                ci[j] = xi + si * ci[j];
            }
        }

        // compute function values
        for i in 0..n {
            let val = f(&m.row(i));
            if !val.is_finite() {
                return Err("non-finite function value encountered");
            }
            self.y1[i + 1] = val;
        }

        self.center = self.compute_center();
        *size = self.compute_size(&self.center);
        self.count.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    fn iterate(
        &mut self,
        f: &MultiminFunction,
        x: &mut Array1<f64>,
        size: &mut f64,
        fval: &mut f64,
    ) -> Result<(), &'static str> {
        if self.ws1.len() != x.len() {
            return Err("incompatible size of x");
        }

        let n = self.y1.len();
        let mut hi = 0;
        let mut s_hi = 1;
        let mut lo = 0;
        let mut dhi = self.y1[0];
        let mut ds_hi = self.y1[1];
        let mut dlo = self.y1[0];

        // find highest, second highest and lowest points
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

        // try reflecting the highest value point
        let val = self.try_corner_move(-1.0, hi, &mut self.ws1, f);

        if val.is_finite() && val < self.y1[lo] {
            // reflected point is lowest, try expansion
            let val2 = self.try_corner_move(-2.0, hi, &mut self.ws2, f);

            if val2.is_finite() && val2 < self.y1[lo] {
                self.update_point(hi, &self.ws2, val2);
            } else {
                self.update_point(hi, &self.ws1, val);
            }
        } else if !val.is_finite() || val > self.y1[s_hi] {
            // reflection doesn't improve things or non-finite value
            if val.is_finite() && val <= self.y1[hi] {
                self.update_point(hi, &self.ws1, val);
            }

            // try one-dimensional contraction
            let val2 = self.try_corner_move(0.5, hi, &mut self.ws2, f);

            if val2.is_finite() && val2 <= self.y1[hi] {
                self.update_point(hi, &self.ws2, val2);
            } else {
                // contract the whole simplex about the best point
                self.contract_by_best(lo, &mut self.ws1, f)?;
            }
        } else {
            // trial point is better than second highest point
            self.update_point(hi, &self.ws1, val);
        }

        // return lowest point
        lo = self.y1.argmin().unwrap();
        x.assign(&self.x1.row(lo));
        *fval = self.y1[lo];

        // update simplex size
        if self.s2 > 0.0 {
            *size = self.s2.sqrt();
        } else {
            *size = self.compute_size(&self.center);
        }

        Ok(())
    }

    fn ran_unif(seed: &mut usize) -> f64 {
        let s = *seed;
        *seed = (s * 69069 + 1) & 0xffffffff;
        (*seed as f64) / 4294967296.0
    }
}

pub struct NMSimplex2 {
    state: NMSimplexState,
}

impl NMSimplex2 {
    pub fn new(n: usize) -> Result<Self, &'static str> {
        Ok(Self {
            state: NMSimplexState::new(n)?,
        })
    }

    pub fn set(
        &mut self,
        f: &MultiminFunction,
        x: &Array1<f64>,
        size: &mut f64,
        step_size: &Array1<f64>,
    ) -> Result<(), &'static str> {
        self.state.set(f, x, size, step_size)
    }

    pub fn iterate(
        &mut self,
        f: &MultiminFunction,
        x: &mut Array1<f64>,
        size: &mut f64,
        fval: &mut f64,
    ) -> Result<(), &'static str> {
        self.state.iterate(f, x, size, fval)
    }
}

pub struct NMSimplex2Rand {
    state: NMSimplexState,
}

impl NMSimplex2Rand {
    pub fn new(n: usize) -> Result<Self, &'static str> {
        Ok(Self {
            state: NMSimplexState::new(n)?,
        })
    }

    pub fn set(
        &mut self,
        f: &MultiminFunction,
        x: &Array1<f64>,
        size: &mut f64,
        step_size: &Array1<f64>,
    ) -> Result<(), &'static str> {
        self.state.set_rand(f, x, size, step_size)
    }

    pub fn iterate(
        &mut self,
        f: &MultiminFunction,
        x: &mut Array1<f64>,
        size: &mut f64,
        fval: &mut f64,
    ) -> Result<(), &'static str> {
        self.state.iterate(f, x, size, fval)
    }
}