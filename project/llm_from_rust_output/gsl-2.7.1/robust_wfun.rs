use std::f64::consts::E;

pub type SizeT = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
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
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Debug, Clone)]
pub struct GslBlock {
    pub size: SizeT,
    pub data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: SizeT,
    pub stride: SizeT,
    pub data: Vec<f64>,
    pub block: GslBlock,
    pub owner: bool,
}

impl GslVector {
    pub fn new(size: SizeT) -> Self {
        let data = vec![0.0; size];
        let block = GslBlock {
            size,
            data: data.clone(),
        };
        Self {
            size,
            stride: 1,
            data,
            block,
            owner: true,
        }
    }

    pub fn get(&self, i: SizeT) -> f64 {
        self.data[i * self.stride]
    }

    pub fn set(&mut self, i: SizeT, x: f64) {
        self.data[i * self.stride] = x;
    }

    pub fn set_all(&mut self, x: f64) {
        self.data.iter_mut().for_each(|v| *v = x);
    }
}

#[derive(Debug, Clone)]
pub struct GslMultifitRobustType {
    pub name: &'static str,
    pub wfun: fn(&GslVector, &mut GslVector) -> GslError,
    pub psi_deriv: fn(&GslVector, &mut GslVector) -> GslError,
    pub tuning_default: f64,
}

fn bisquare(r: &GslVector, w: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        if ri.abs() < 1.0 {
            w.set(i, (1.0 - ri * ri).powi(2));
        } else {
            w.set(i, 0.0);
        }
    }
    GslError::Success
}

fn bisquare_dpsi(r: &GslVector, dpsi: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        if ri.abs() < 1.0 {
            dpsi.set(i, (1.0 - ri * ri) * (1.0 - 5.0 * ri * ri));
        } else {
            dpsi.set(i, 0.0);
        }
    }
    GslError::Success
}

fn cauchy(r: &GslVector, w: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        w.set(i, 1.0 / (1.0 + ri * ri));
    }
    GslError::Success
}

fn cauchy_dpsi(r: &GslVector, dpsi: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        let rsq = ri * ri;
        dpsi.set(i, (1.0 - rsq) / (1.0 + rsq).powi(2));
    }
    GslError::Success
}

fn fair(r: &GslVector, w: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        w.set(i, 1.0 / (1.0 + ri.abs())));
    }
    GslError::Success
}

fn fair_dpsi(r: &GslVector, dpsi: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        dpsi.set(i, 1.0 / (1.0 + ri.abs()).powi(2));
    }
    GslError::Success
}

fn huber(r: &GslVector, w: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let absri = r.get(i).abs();
        if absri <= 1.0 {
            w.set(i, 1.0);
        } else {
            w.set(i, 1.0 / absri);
        }
    }
    GslError::Success
}

fn huber_dpsi(r: &GslVector, dpsi: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        if ri.abs() <= 1.0 {
            dpsi.set(i, 1.0);
        } else {
            dpsi.set(i, 0.0);
        }
    }
    GslError::Success
}

fn ols(_r: &GslVector, w: &mut GslVector) -> GslError {
    w.set_all(1.0);
    GslError::Success
}

fn ols_dpsi(_r: &GslVector, dpsi: &mut GslVector) -> GslError {
    dpsi.set_all(1.0);
    GslError::Success
}

fn welsch(r: &GslVector, w: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        w.set(i, E.powf(-ri * ri));
    }
    GslError::Success
}

fn welsch_dpsi(r: &GslVector, dpsi: &mut GslVector) -> GslError {
    for i in 0..r.size {
        let ri = r.get(i);
        dpsi.set(i, (1.0 - 2.0 * ri * ri) * E.powf(-ri * ri));
    }
    GslError::Success
}

pub static GSL_MULTIFIT_ROBUST_BISQUARE: GslMultifitRobustType = GslMultifitRobustType {
    name: "bisquare",
    wfun: bisquare,
    psi_deriv: bisquare_dpsi,
    tuning_default: 4.685,
};

pub static GSL_MULTIFIT_ROBUST_CAUCHY: GslMultifitRobustType = GslMultifitRobustType {
    name: "cauchy",
    wfun: cauchy,
    psi_deriv: cauchy_dpsi,
    tuning_default: 2.385,
};

pub static GSL_MULTIFIT_ROBUST_FAIR: GslMultifitRobustType = GslMultifitRobustType {
    name: "fair",
    wfun: fair,
    psi_deriv: fair_dpsi,
    tuning_default: 1.4,
};

pub static GSL_MULTIFIT_ROBUST_HUBER: GslMultifitRobustType = GslMultifitRobustType {
    name: "huber",
    wfun: huber,
    psi_deriv: huber_dpsi,
    tuning_default: 1.345,
};

pub static GSL_MULTIFIT_ROBUST_OLS: GslMultifitRobustType = GslMultifitRobustType {
    name: "ols",
    wfun: ols,
    psi_deriv: ols_dpsi,
    tuning_default: 1.0,
};

pub static GSL_MULTIFIT_ROBUST_WELSCH: GslMultifitRobustType = GslMultifitRobustType {
    name: "welsch",
    wfun: welsch,
    psi_deriv: welsch_dpsi,
    tuning_default: 2.985,
};

pub static GSL_MULTIFIT_ROBUST_DEFAULT: &GslMultifitRobustType = &GSL_MULTIFIT_ROBUST_BISQUARE;