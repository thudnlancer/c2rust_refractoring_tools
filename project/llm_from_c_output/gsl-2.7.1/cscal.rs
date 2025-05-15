use std::f64::{self, DBL_MIN};
use std::ptr;
use std::mem;
use std::slice;

#[derive(Debug)]
pub struct ScControlState {
    eps_abs: f64,
    eps_rel: f64,
    a_y: f64,
    a_dydt: f64,
    scale_abs: Vec<f64>,
}

#[derive(Debug)]
pub struct GslOdeiv2Control {
    state: Box<ScControlState>,
}

#[derive(Debug, PartialEq)]
pub enum GslError {
    Enomem,
    Einval,
    Esanity,
    Other(&'static str),
}

pub type GslResult<T> = Result<T, GslError>;

const GSL_SUCCESS: GslResult<()> = Ok(());
const GSL_ODEIV_HADJ_DEC: i32 = -1;
const GSL_ODEIV_HADJ_INC: i32 = 1;
const GSL_ODEIV_HADJ_NIL: i32 = 0;

impl ScControlState {
    fn new() -> GslResult<Self> {
        Ok(Self {
            eps_abs: 0.0,
            eps_rel: 0.0,
            a_y: 0.0,
            a_dydt: 0.0,
            scale_abs: Vec::new(),
        })
    }

    fn init(&mut self, eps_abs: f64, eps_rel: f64, a_y: f64, a_dydt: f64) -> GslResult<()> {
        if eps_abs < 0.0 {
            return Err(GslError::Einval);
        } else if eps_rel < 0.0 {
            return Err(GslError::Einval);
        } else if a_y < 0.0 {
            return Err(GslError::Einval);
        } else if a_dydt < 0.0 {
            return Err(GslError::Einval);
        }

        self.eps_rel = eps_rel;
        self.eps_abs = eps_abs;
        self.a_y = a_y;
        self.a_dydt = a_dydt;

        GSL_SUCCESS
    }

    fn hadjust(
        &self,
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
        let scale_abs = &self.scale_abs;

        const S: f64 = 0.9;
        let h_old = *h;

        let mut rmax = DBL_MIN;

        for i in 0..dim {
            let d0 = eps_rel * (a_y * y[i].abs() + a_dydt * (h_old * yp[i]).abs())
                + eps_abs * scale_abs[i];
            let r = yerr[i].abs() / d0.abs();
            rmax = r.max(rmax);
        }

        if rmax > 1.1 {
            let r = S / rmax.powf(1.0 / ord as f64);
            let r = if r < 0.2 { 0.2 } else { r };
            *h = r * h_old;
            GSL_ODEIV_HADJ_DEC
        } else if rmax < 0.5 {
            let r = S / rmax.powf(1.0 / (ord as f64 + 1.0));
            const MAXSCALE: f64 = 4.9;
            let r = if r > MAXSCALE { MAXSCALE } else { r };
            let r = if r < 1.0 { 1.0 } else { r };
            *h = r * h_old;
            GSL_ODEIV_HADJ_INC
        } else {
            GSL_ODEIV_HADJ_NIL
        }
    }

    fn errlevel(
        &self,
        y: f64,
        dydt: f64,
        h: f64,
        ind: usize,
        errlev: &mut f64,
    ) -> GslResult<()> {
        let eps_abs = self.eps_abs;
        let eps_rel = self.eps_rel;
        let a_y = self.a_y;
        let a_dydt = self.a_dydt;
        let scale_abs = &self.scale_abs;

        *errlev = eps_rel * (a_y * y.abs() + a_dydt * (h * dydt).abs()) + eps_abs * scale_abs[ind];

        if *errlev <= 0.0 {
            return Err(GslError::Esanity);
        }

        GSL_SUCCESS
    }
}

pub fn gsl_odeiv2_control_scaled_new(
    eps_abs: f64,
    eps_rel: f64,
    a_y: f64,
    a_dydt: f64,
    scale_abs: &[f64],
    dim: usize,
) -> GslResult<GslOdeiv2Control> {
    let mut state = Box::new(ScControlState::new()?);
    state.init(eps_abs, eps_rel, a_y, a_dydt)?;
    state.scale_abs = scale_abs.to_vec();

    Ok(GslOdeiv2Control { state })
}

impl Drop for GslOdeiv2Control {
    fn drop(&mut self) {
        // Vec will be automatically dropped
    }
}