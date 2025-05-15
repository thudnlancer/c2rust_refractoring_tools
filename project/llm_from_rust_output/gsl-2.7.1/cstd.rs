use std::f64;

pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

pub struct StdControlState {
    eps_abs: f64,
    eps_rel: f64,
    a_y: f64,
    a_dydt: f64,
}

pub struct GslOdeivControlType {
    name: &'static str,
}

pub struct GslOdeivControl {
    control_type: &'static GslOdeivControlType,
    state: Box<StdControlState>,
}

impl StdControlState {
    fn new(eps_abs: f64, eps_rel: f64, a_y: f64, a_dydt: f64) -> Result<Self, GslError> {
        if eps_abs < 0.0 {
            return Err(GslError::Inval);
        }
        if eps_rel < 0.0 {
            return Err(GslError::Inval);
        }
        if a_y < 0.0 {
            return Err(GslError::Inval);
        }
        if a_dydt < 0.0 {
            return Err(GslError::Inval);
        }

        Ok(Self {
            eps_abs,
            eps_rel,
            a_y,
            a_dydt,
        })
    }

    fn hadjust(
        &mut self,
        dim: usize,
        ord: u32,
        y: &[f64],
        yerr: &[f64],
        yp: &[f64],
        h: &mut f64,
    ) -> i32 {
        let eps_abs = self.eps_abs;
        let eps_rel = self.eps_rel;
        let a_y = self.a_y;
        let a_dydt = self.a_dydt;
        let s = 0.9;
        let h_old = *h;
        let mut rmax = f64::MIN_POSITIVE;

        for i in 0..dim {
            let d0 = eps_rel * (a_y * y[i].abs() + a_dydt * (h_old * yp[i]).abs()) + eps_abs;
            let r = yerr[i].abs() / d0.abs();
            rmax = r.max(rmax);
        }

        if rmax > 1.1 {
            let mut r = s / rmax.powf(1.0 / ord as f64);
            if r < 0.2 {
                r = 0.2;
            }
            *h = r * h_old;
            -1
        } else if rmax < 0.5 {
            let mut r = s / rmax.powf(1.0 / (ord as f64 + 1.0));
            if r > 5.0 {
                r = 5.0;
            }
            if r < 1.0 {
                r = 1.0;
            }
            *h = r * h_old;
            1
        } else {
            0
        }
    }
}

pub static GSL_ODEIV_CONTROL_STANDARD: GslOdeivControlType = GslOdeivControlType {
    name: "standard",
};

impl GslOdeivControl {
    pub fn standard_new(
        eps_abs: f64,
        eps_rel: f64,
        a_y: f64,
        a_dydt: f64,
    ) -> Result<Self, GslError> {
        let state = StdControlState::new(eps_abs, eps_rel, a_y, a_dydt)?;
        Ok(Self {
            control_type: &GSL_ODEIV_CONTROL_STANDARD,
            state: Box::new(state),
        })
    }

    pub fn y_new(eps_abs: f64, eps_rel: f64) -> Result<Self, GslError> {
        Self::standard_new(eps_abs, eps_rel, 1.0, 0.0)
    }

    pub fn yp_new(eps_abs: f64, eps_rel: f64) -> Result<Self, GslError> {
        Self::standard_new(eps_abs, eps_rel, 0.0, 1.0)
    }

    pub fn hadjust(
        &mut self,
        dim: usize,
        ord: u32,
        y: &[f64],
        yerr: &[f64],
        yp: &[f64],
        h: &mut f64,
    ) -> i32 {
        self.state.hadjust(dim, ord, y, yerr, yp, h)
    }
}