use std::f64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QawsError {
    #[error("alpha must be greater than -1.0")]
    AlphaInvalid,
    #[error("beta must be greater than -1.0")]
    BetaInvalid,
    #[error("mu must be 0 or 1")]
    MuInvalid,
    #[error("nu must be 0 or 1")]
    NuInvalid,
    #[error("failed to allocate space for qaws_table struct")]
    AllocationFailed,
}

pub struct QawsTable {
    pub alpha: f64,
    pub beta: f64,
    pub mu: i32,
    pub nu: i32,
    pub ri: [f64; 25],
    pub rj: [f64; 25],
    pub rg: [f64; 25],
    pub rh: [f64; 25],
}

impl QawsTable {
    pub fn new(alpha: f64, beta: f64, mu: i32, nu: i32) -> Result<Self, QawsError> {
        if alpha < -1.0 {
            return Err(QawsError::AlphaInvalid);
        }
        if beta < -1.0 {
            return Err(QawsError::BetaInvalid);
        }
        if mu != 0 && mu != 1 {
            return Err(QawsError::MuInvalid);
        }
        if nu != 0 && nu != 1 {
            return Err(QawsError::NuInvalid);
        }

        let mut table = QawsTable {
            alpha,
            beta,
            mu,
            nu,
            ri: [0.0; 25],
            rj: [0.0; 25],
            rg: [0.0; 25],
            rh: [0.0; 25],
        };

        initialise(&mut table.ri, &mut table.rj, &mut table.rg, &mut table.rh, alpha, beta);
        Ok(table)
    }

    pub fn set_params(&mut self, alpha: f64, beta: f64, mu: i32, nu: i32) -> Result<(), QawsError> {
        if alpha < -1.0 {
            return Err(QawsError::AlphaInvalid);
        }
        if beta < -1.0 {
            return Err(QawsError::BetaInvalid);
        }
        if mu != 0 && mu != 1 {
            return Err(QawsError::MuInvalid);
        }
        if nu != 0 && nu != 1 {
            return Err(QawsError::NuInvalid);
        }

        self.alpha = alpha;
        self.beta = beta;
        self.mu = mu;
        self.nu = nu;
        
        initialise(&mut self.ri, &mut self.rj, &mut self.rg, &mut self.rh, alpha, beta);
        Ok(())
    }
}

fn initialise(ri: &mut [f64; 25], rj: &mut [f64; 25], rg: &mut [f64; 25], rh: &mut [f64; 25], alpha: f64, beta: f64) {
    let alpha_p1 = alpha + 1.0;
    let beta_p1 = beta + 1.0;

    let alpha_p2 = alpha + 2.0;
    let beta_p2 = beta + 2.0;

    let r_alpha = 2.0f64.powf(alpha_p1);
    let r_beta = 2.0f64.powf(beta_p1);

    ri[0] = r_alpha / alpha_p1;
    ri[1] = ri[0] * alpha / alpha_p2;

    let mut an = 2.0;
    let mut anm1 = 1.0;

    for i in 2..25 {
        ri[i] = -(r_alpha + an * (an - alpha_p2) * ri[i - 1]) / (anm1 * (an + alpha_p1));
        anm1 = an;
        an += 1.0;
    }

    rj[0] = r_beta / beta_p1;
    rj[1] = rj[0] * beta / beta_p2;

    an = 2.0;
    anm1 = 1.0;

    for i in 2..25 {
        rj[i] = -(r_beta + an * (an - beta_p2) * rj[i - 1]) / (anm1 * (an + beta_p1));
        anm1 = an;
        an += 1.0;
    }

    rg[0] = -ri[0] / alpha_p1;
    rg[1] = -rg[0] - 2.0 * r_alpha / (alpha_p2 * alpha_p2);

    an = 2.0;
    anm1 = 1.0;

    for i in 2..25 {
        rg[i] = -(an * (an - alpha_p2) * rg[i - 1] - an * ri[i - 1] + anm1 * ri[i]) / (anm1 * (an + alpha_p1));
        anm1 = an;
        an += 1.0;
    }

    rh[0] = -rj[0] / beta_p1;
    rh[1] = -rh[0] - 2.0 * r_beta / (beta_p2 * beta_p2);

    an = 2.0;
    anm1 = 1.0;

    for i in 2..25 {
        rh[i] = -(an * (an - beta_p2) * rh[i - 1] - an * rj[i - 1] + anm1 * rj[i]) / (anm1 * (an + beta_p1));
        anm1 = an;
        an += 1.0;
    }

    for i in (1..25).step_by(2) {
        rj[i] *= -1.0;
        rh[i] *= -1.0;
    }
}