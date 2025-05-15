use std::f64;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Einval = 4,
    Enomem = 8,
    // Other error codes omitted for brevity
}

#[derive(Debug, Clone, Copy)]
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
    pub fn new(alpha: f64, beta: f64, mu: i32, nu: i32) -> Result<Self, GslError> {
        if alpha <= -1.0 {
            return Err(GslError::Einval);
        }
        if beta <= -1.0 {
            return Err(GslError::Einval);
        }
        if mu != 0 && mu != 1 {
            return Err(GslError::Einval);
        }
        if nu != 0 && nu != 1 {
            return Err(GslError::Einval);
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

        table.initialise();
        Ok(table)
    }

    pub fn set_params(&mut self, alpha: f64, beta: f64, mu: i32, nu: i32) -> Result<(), GslError> {
        if alpha <= -1.0 {
            return Err(GslError::Einval);
        }
        if beta <= -1.0 {
            return Err(GslError::Einval);
        }
        if mu != 0 && mu != 1 {
            return Err(GslError::Einval);
        }
        if nu != 0 && nu != 1 {
            return Err(GslError::Einval);
        }

        self.alpha = alpha;
        self.beta = beta;
        self.mu = mu;
        self.nu = nu;
        self.initialise();
        Ok(())
    }

    fn initialise(&mut self) {
        let alpha_p1 = self.alpha + 1.0;
        let beta_p1 = self.beta + 1.0;
        let alpha_p2 = self.alpha + 2.0;
        let beta_p2 = self.beta + 2.0;
        let r_alpha = 2.0f64.powf(alpha_p1);
        let r_beta = 2.0f64.powf(beta_p1);

        // Initialize ri
        self.ri[0] = r_alpha / alpha_p1;
        self.ri[1] = self.ri[0] * self.alpha / alpha_p2;
        
        let mut an = 2.0;
        let mut anm1 = 1.0;
        for i in 2..25 {
            self.ri[i] = -(r_alpha + an * (an - alpha_p2) * self.ri[i-1]) / (anm1 * (an + alpha_p1));
            anm1 = an;
            an += 1.0;
        }

        // Initialize rj
        self.rj[0] = r_beta / beta_p1;
        self.rj[1] = self.rj[0] * self.beta / beta_p2;
        
        an = 2.0;
        anm1 = 1.0;
        for i in 2..25 {
            self.rj[i] = -(r_beta + an * (an - beta_p2) * self.rj[i-1]) / (anm1 * (an + beta_p1));
            anm1 = an;
            an += 1.0;
        }

        // Initialize rg
        self.rg[0] = -self.ri[0] / alpha_p1;
        self.rg[1] = -self.rg[0] - 2.0 * r_alpha / (alpha_p2 * alpha_p2);
        
        an = 2.0;
        anm1 = 1.0;
        for i in 2..25 {
            self.rg[i] = -(an * (an - alpha_p2) * self.rg[i-1] - an * self.ri[i-1] + anm1 * self.ri[i]) / (anm1 * (an + alpha_p1));
            anm1 = an;
            an += 1.0;
        }

        // Initialize rh
        self.rh[0] = -self.rj[0] / beta_p1;
        self.rh[1] = -self.rh[0] - 2.0 * r_beta / (beta_p2 * beta_p2);
        
        an = 2.0;
        anm1 = 1.0;
        for i in 2..25 {
            self.rh[i] = -(an * (an - beta_p2) * self.rh[i-1] - an * self.rj[i-1] + anm1 * self.rj[i]) / (anm1 * (an + beta_p1));
            anm1 = an;
            an += 1.0;
        }

        // Adjust signs for odd indices
        for i in (1..25).step_by(2) {
            self.rj[i] *= -1.0;
            self.rh[i] *= -1.0;
        }
    }
}