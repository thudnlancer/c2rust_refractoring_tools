use std::f64::{self, EPSILON, MIN_POSITIVE};
use std::ptr::null_mut;

#[derive(Debug)]
pub struct StdControlState {
    eps_abs: f64,
    eps_rel: f64,
    a_y: f64,
    a_dydt: f64,
}

impl StdControlState {
    fn new(eps_abs: f64, eps_rel: f64, a_y: f64, a_dydt: f64) -> Result<Self, &'static str> {
        if eps_abs < 0.0 {
            return Err("eps_abs is negative");
        } else if eps_rel < 0.0 {
            return Err("eps_rel is negative");
        } else if a_y < 0.0 {
            return Err("a_y is negative");
        } else if a_dydt < 0.0 {
            return Err("a_dydt is negative");
        }

        Ok(Self {
            eps_abs,
            eps_rel,
            a_y,
            a_dydt,
        })
    }

    fn hadjust(
        &self,
        dim: usize,
        ord: u32,
        y: &[f64],
        yerr: &[f64],
        yp: &[f64],
        h: &mut f64,
    ) -> Result<Adjustment, &'static str> {
        let eps_abs = self.eps_abs;
        let eps_rel = self.eps_rel;
        let a_y = self.a_y;
        let a_dydt = self.a_dydt;

        const S: f64 = 0.9;
        let h_old = *h;

        let mut rmax = f64::MIN_POSITIVE;

        for i in 0..dim {
            let d0 = eps_rel * (a_y * y[i].abs() + a_dydt * (h_old * yp[i]).abs()) + eps_abs;
            let r = yerr[i].abs() / d0.abs();
            rmax = r.max(rmax);
        }

        if rmax > 1.1 {
            // decrease step, no more than factor of 5, but a fraction S more
            // than scaling suggests (for better accuracy)
            let mut r = S / rmax.powf(1.0 / ord as f64);

            if r < 0.2 {
                r = 0.2;
            }

            *h = r * h_old;

            Ok(Adjustment::Decrease)
        } else if rmax < 0.5 {
            // increase step, no more than factor of 5
            let mut r = S / rmax.powf(1.0 / (ord as f64 + 1.0));

            if r > 5.0 {
                r = 5.0;
            }

            if r < 1.0 {
                // don't allow any decrease caused by S<1
                r = 1.0;
            }

            *h = r * h_old;

            Ok(Adjustment::Increase)
        } else {
            // no change
            Ok(Adjustment::NoChange)
        }
    }

    fn errlevel(
        &self,
        y: f64,
        dydt: f64,
        h: f64,
        _ind: usize,
    ) -> Result<f64, &'static str> {
        let eps_abs = self.eps_abs;
        let eps_rel = self.eps_rel;
        let a_y = self.a_y;
        let a_dydt = self.a_dydt;

        let errlev = eps_rel * (a_y * y.abs() + a_dydt * (h * dydt).abs()) + eps_abs;

        if errlev <= 0.0 {
            Err("errlev <= zero")
        } else {
            Ok(errlev)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Adjustment {
    Increase,
    Decrease,
    NoChange,
}

pub struct StdControl {
    state: StdControlState,
}

impl StdControl {
    pub fn standard_new(
        eps_abs: f64,
        eps_rel: f64,
        a_y: f64,
        a_dydt: f64,
    ) -> Result<Self, &'static str> {
        let state = StdControlState::new(eps_abs, eps_rel, a_y, a_dydt)?;
        Ok(Self { state })
    }

    pub fn y_new(eps_abs: f64, eps_rel: f64) -> Result<Self, &'static str> {
        Self::standard_new(eps_abs, eps_rel, 1.0, 0.0)
    }

    pub fn yp_new(eps_abs: f64, eps_rel: f64) -> Result<Self, &'static str> {
        Self::standard_new(eps_abs, eps_rel, 0.0, 1.0)
    }

    pub fn hadjust(
        &self,
        dim: usize,
        ord: u32,
        y: &[f64],
        yerr: &[f64],
        yp: &[f64],
        h: &mut f64,
    ) -> Result<Adjustment, &'static str> {
        self.state.hadjust(dim, ord, y, yerr, yp, h)
    }

    pub fn errlevel(
        &self,
        y: f64,
        dydt: f64,
        h: f64,
        ind: usize,
    ) -> Result<f64, &'static str> {
        self.state.errlevel(y, dydt, h, ind)
    }
}