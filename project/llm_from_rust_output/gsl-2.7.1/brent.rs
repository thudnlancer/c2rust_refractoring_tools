use std::f64;

pub type SizeT = usize;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDom = 1,
    ERange = 2,
    EDefault = 3,
    EInval = 4,
    EFailed = 5,
    EFactor = 6,
    ESanity = 7,
    ENomem = 8,
    EBadfunc = 9,
    ERunaway = 10,
    EMaxiter = 11,
    EZerodiv = 12,
    EBadtol = 13,
    ETol = 14,
    EUndrflw = 15,
    EOvrflw = 16,
    ELoss = 17,
    ERound = 18,
    EBadlen = 19,
    ENotsqr = 20,
    ESing = 21,
    EDiverge = 22,
    EUnsup = 23,
    EUnimpl = 24,
    ECache = 25,
    ETable = 26,
    ENoprog = 27,
    ENoprogj = 28,
    ETolf = 29,
    ETolx = 30,
    ETolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

pub struct BrentState {
    d: f64,
    e: f64,
    v: f64,
    w: f64,
    f_v: f64,
    f_w: f64,
}

pub struct BrentMinimizer {
    state: BrentState,
    golden: f64,
    tolerance: f64,
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
            golden: 0.3819660,
            tolerance: 1.4901161193847656e-08,
        }
    }

    pub fn init(
        &mut self,
        f: &GslFunction,
        x_lower: f64,
        x_upper: f64,
    ) -> Result<(), GslError> {
        let v = x_lower + self.golden * (x_upper - x_lower);
        let w = v;
        
        self.state.v = v;
        self.state.w = w;
        self.state.d = 0.0;
        self.state.e = 0.0;
        
        let f_vw = (f.function)(v);
        if !f_vw.is_finite() {
            return Err(GslError::EBadfunc);
        }
        
        self.state.f_v = f_vw;
        self.state.f_w = f_vw;
        
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
    ) -> Result<(), GslError> {
        let x_left = *x_lower;
        let x_right = *x_upper;
        let z = *x_minimum;
        let mut d = self.state.e;
        let mut e = self.state.d;
        let mut u;
        let mut f_u;
        let v = self.state.v;
        let w = self.state.w;
        let f_v = self.state.f_v;
        let f_w = self.state.f_w;
        let f_z = *f_minimum;
        
        let w_lower = z - x_left;
        let w_upper = x_right - z;
        let tolerance = self.tolerance * z.abs();
        let midpoint = 0.5 * (x_left + x_right);
        
        let mut p = 0.0;
        let mut q = 0.0;
        let mut r;
        
        if e.abs() > tolerance {
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
        
        if p.abs() < (0.5 * q * r).abs() && p < q * w_lower && p < q * w_upper {
            let t2 = 2.0 * tolerance;
            d = p / q;
            u = z + d;
            if u - x_left < t2 || x_right - u < t2 {
                d = if z < midpoint { tolerance } else { -tolerance };
            }
        } else {
            e = if z < midpoint { x_right - z } else { -(z - x_left) };
            d = self.golden * e;
        }
        
        if d.abs() >= tolerance {
            u = z + d;
        } else {
            u = z + if d > 0.0 { tolerance } else { -tolerance };
        }
        
        self.state.e = e;
        self.state.d = d;
        
        f_u = (f.function)(u);
        if !f_u.is_finite() {
            return Err(GslError::EBadfunc);
        }
        
        if f_u <= f_z {
            if u < z {
                *x_upper = z;
                *f_upper = f_z;
            } else {
                *x_lower = z;
                *f_lower = f_z;
            }
            self.state.v = w;
            self.state.f_v = f_w;
            self.state.w = z;
            self.state.f_w = f_z;
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
                self.state.v = w;
                self.state.f_v = f_w;
                self.state.w = u;
                self.state.f_w = f_u;
            } else if f_u <= f_v 
                || (v - z).abs() < f64::EPSILON 
                || (v - w).abs() < f64::EPSILON 
            {
                self.state.v = u;
                self.state.f_v = f_u;
            }
        }
        
        Ok(())
    }
}

pub fn gsl_min_fminimizer_brent() -> BrentMinimizer {
    BrentMinimizer::new()
}