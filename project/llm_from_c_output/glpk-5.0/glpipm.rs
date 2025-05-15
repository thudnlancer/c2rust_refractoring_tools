use std::f64;
use std::ptr;
use std::mem;
use std::cmp;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GlpError {
    NoConv,
    IterLimit,
    NumInstab,
    Other(String),
}

impl fmt::Display for GlpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlpError::NoConv => write!(f, "No convergence"),
            GlpError::IterLimit => write!(f, "Iteration limit exceeded"),
            GlpError::NumInstab => write!(f, "Numeric instability"),
            GlpError::Other(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for GlpError {
    fn description(&self) -> &str {
        match *self {
            GlpError::NoConv => "No convergence",
            GlpError::IterLimit => "Iteration limit exceeded",
            GlpError::NumInstab => "Numeric instability",
            GlpError::Other(ref err) => err,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GlpMsgLev {
    Off,
    Err,
    On,
    All,
    Db,
}

#[derive(Debug, Clone, Copy)]
pub enum GlpOrdAlg {
    None,
    Qmd,
    Amd,
    SymAmd,
}

#[derive(Debug)]
pub struct GlpIptcp {
    pub msg_lev: GlpMsgLev,
    pub ord_alg: GlpOrdAlg,
}

#[derive(Debug)]
pub struct GlpProb {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub dir: GlpDir,
    pub c0: f64,
    pub row: Vec<GlpRow>,
    pub col: Vec<GlpCol>,
    pub ipt_stat: GlpIptStat,
    pub ipt_obj: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum GlpDir {
    Min,
    Max,
}

#[derive(Debug, Clone, Copy)]
pub enum GlpIptStat {
    Opt,
    NoFeas,
    Infeas,
}

#[derive(Debug)]
pub struct GlpRow {
    pub typ: GlpBnd,
    pub lb: f64,
    pub ub: f64,
    pub rii: f64,
    pub ptr: Option<Box<GlpAij>>,
    pub pval: f64,
    pub dval: f64,
}

#[derive(Debug)]
pub struct GlpCol {
    pub typ: GlpBnd,
    pub lb: f64,
    pub ub: f64,
    pub coef: f64,
    pub sjj: f64,
    pub ptr: Option<Box<GlpAij>>,
    pub pval: f64,
    pub dval: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum GlpBnd {
    Free,
    Lb(f64),
    Ub(f64),
    Db(f64, f64),
    Fix(f64),
}

#[derive(Debug)]
pub struct GlpAij {
    pub row: *mut GlpRow,
    pub col: *mut GlpCol,
    pub val: f64,
    pub r_next: Option<Box<GlpAij>>,
    pub c_next: Option<Box<GlpAij>>,
}

const ITER_MAX: usize = 100;

struct Csa {
    m: i32,
    n: i32,
    a_ptr: Vec<i32>,
    a_ind: Vec<i32>,
    a_val: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    parm: GlpIptcp,
    d: Vec<f64>,
    p: Vec<i32>,
    s_ptr: Vec<i32>,
    s_ind: Vec<i32>,
    s_val: Vec<f64>,
    s_diag: Vec<f64>,
    u_ptr: Vec<i32>,
    u_ind: Vec<i32>,
    u_val: Vec<f64>,
    u_diag: Vec<f64>,
    iter: i32,
    obj: f64,
    rpi: f64,
    rdi: f64,
    gap: f64,
    phi: f64,
    mu: f64,
    rmu: f64,
    rmu0: f64,
    phi_min: Vec<f64>,
    best_iter: i32,
    best_x: Vec<f64>,
    best_y: Vec<f64>,
    best_z: Vec<f64>,
    best_obj: f64,
    dx_aff: Vec<f64>,
    dy_aff: Vec<f64>,
    dz_aff: Vec<f64>,
    alfa_aff_p: f64,
    alfa_aff_d: f64,
    mu_aff: f64,
    sigma: f64,
    dx_cc: Vec<f64>,
    dy_cc: Vec<f64>,
    dz_cc: Vec<f64>,
    dx: Vec<f64>,
    dy: Vec<f64>,
    dz: Vec<f64>,
    alfa_max_p: f64,
    alfa_max_d: f64,
}

impl Csa {
    fn new(m: i32, n: i32, a_ptr: Vec<i32>, a_ind: Vec<i32>, a_val: Vec<f64>, 
           b: Vec<f64>, c: Vec<f64>, parm: GlpIptcp) -> Self {
        Csa {
            m,
            n,
            a_ptr,
            a_ind,
            a_val,
            b,
            c,
            x: vec![0.0; (n+1) as usize],
            y: vec![0.0; (m+1) as usize],
            z: vec![0.0; (n+1) as usize],
            parm,
            d: Vec::new(),
            p: Vec::new(),
            s_ptr: Vec::new(),
            s_ind: Vec::new(),
            s_val: Vec::new(),
            s_diag: Vec::new(),
            u_ptr: Vec::new(),
            u_ind: Vec::new(),
            u_val: Vec::new(),
            u_diag: Vec::new(),
            iter: 0,
            obj: 0.0,
            rpi: 0.0,
            rdi: 0.0,
            gap: 0.0,
            phi: 0.0,
            mu: 0.0,
            rmu: 0.0,
            rmu0: 0.0,
            phi_min: vec![0.0; ITER_MAX + 1],
            best_iter: 0,
            best_x: vec![0.0; (n+1) as usize],
            best_y: vec![0.0; (m+1) as usize],
            best_z: vec![0.0; (n+1) as usize],
            best_obj: 0.0,
            dx_aff: vec![0.0; (n+1) as usize],
            dy_aff: vec![0.0; (m+1) as usize],
            dz_aff: vec![0.0; (n+1) as usize],
            alfa_aff_p: 0.0,
            alfa_aff_d: 0.0,
            mu_aff: 0.0,
            sigma: 0.0,
            dx_cc: vec![0.0; (n+1) as usize],
            dy_cc: vec![0.0; (m+1) as usize],
            dz_cc: vec![0.0; (n+1) as usize],
            dx: vec![0.0; (n+1) as usize],
            dy: vec![0.0; (m+1) as usize],
            dz: vec![0.0; (n+1) as usize],
            alfa_max_p: 0.0,
            alfa_max_d: 0.0,
        }
    }

    fn initialize(&mut self) {
        let m = self.m as usize;
        let n = self.n as usize;
        
        if matches!(self.parm.msg_lev, GlpMsgLev::All) {
            println!("Matrix A has {} non-zeros", self.a_ptr[m+1] - 1);
        }
        
        self.d = vec![0.0; n + 1];
        self.p = vec![0; 2*m + 2];
        
        for i in 1..=m {
            self.p[i] = i as i32;
            self.p[m + i] = i as i32;
        }
        
        self.s_ptr = vec![0; m + 2];
        self.s_ind = adat_symbolic(m as i32, n as i32, &self.p, &self.a_ptr, 
                                  &self.a_ind, &mut self.s_ptr);
        
        if matches!(self.parm.msg_lev, GlpMsgLev::All) {
            println!("Matrix S = A*A' has {} non-zeros (upper triangle)", 
                    self.s_ptr[m+1] - 1 + m as i32);
        }
        
        match self.parm.ord_alg {
            GlpOrdAlg::None => {
                if matches!(self.parm.msg_lev, GlpMsgLev::All) {
                    println!("Original ordering is being used");
                }
                for i in 1..=m {
                    self.p[i] = i as i32;
                    self.p[m + i] = i as i32;
                }
            },
            GlpOrdAlg::Qmd => {
                if matches!(self.parm.msg_lev, GlpMsgLev::All) {
                    println!("Minimum degree ordering (QMD)...");
                }
                min_degree(m as i32, &self.s_ptr, &self.s_ind, &mut self.p);
            },
            GlpOrdAlg::Amd => {
                if matches!(self.parm.msg_lev, GlpMsgLev::All) {
                    println!("Approximate minimum degree ordering (AMD)...");
                }
                amd_order1(m as i32, &self.s_ptr, &self.s_ind, &mut self.p);
            },
            GlpOrdAlg::SymAmd => {
                if matches!(self.parm.msg_lev, GlpMsgLev::All) {
                    println!("Approximate minimum degree ordering (SYMAMD)...");
                }
                symamd_ord(m as i32, &self.s_ptr, &self.s_ind, &mut self.p);
            },
        }
        
        self.s_ind = adat_symbolic(m as i32, n as i32, &self.p, &self.a_ptr, 
                                  &self.a_ind, &mut self.s_ptr);
        self.s_val = vec![0.0; self.s_ptr[m+1] as usize];
        self.s_diag = vec![0.0; m + 1];
        
        if matches!(self.parm.msg_lev, GlpMsgLev::All) {
            println!("Computing Cholesky factorization S = L*L'...");
        }
        
        self.u_ptr = vec![0; m + 2];
        self.u_ind = chol_symbolic(m as i32, &self.s_ptr, &self.s_ind, &mut self.u_ptr);
        
        if matches!(self.parm.msg_lev, GlpMsgLev::All) {
            println!("Matrix L has {} non-zeros", 
                    self.u_ptr[m+1] - 1 + m as i32);
        }
        
        self.u_val = vec![0.0; self.u_ptr[m+1] as usize];
        self.u_diag = vec![0.0; m + 1];
        self.iter = 0;
        self.obj = 0.0;
        self.rpi = 0.0;
        self.rdi = 0.0;
        self.gap = 0.0;
        self.phi = 0.0;
        self.mu = 0.0;
        self.rmu = 0.0;
        self.rmu0 = 0.0;
        self.phi_min = vec![0.0; ITER_MAX + 1];
        self.best_iter = 0;
        self.best_x = vec![0.0; n + 1];
        self.best_y = vec![0.0; m + 1];
        self.best_z = vec![0.0; n + 1];
        self.best_obj = 0.0;
        self.dx_aff = vec![0.0; n + 1];
        self.dy_aff = vec![0.0; m + 1];
        self.dz_aff = vec![0.0; n + 1];
        self.alfa_aff_p = 0.0;
        self.alfa_aff_d = 0.0;
        self.mu_aff = 0.0;
        self.sigma = 0.0;
        self.dx_cc = vec![0.0; n + 1];
        self.dy_cc = vec![0.0; m + 1];
        self.dz_cc = vec![0.0; n + 1];
        self.dx = self.dx_aff.clone();
        self.dy = self.dy_aff.clone();
        self.dz = self.dz_aff.clone();
        self.alfa_max_p = 0.0;
        self.alfa_max_d = 0.0;
    }

    fn a_by_vec(&self, x: &[f64], y: &mut [f64]) {
        let m = self.m as usize;
        for i in 1..=m {
            let mut temp = 0.0;
            let beg = self.a_ptr[i] as usize;
            let end = self.a_ptr[i+1] as usize;
            for t in beg..end {
                temp += self.a_val[t] * x[self.a_ind[t] as usize];
            }
            y[i] = temp;
        }
    }

    fn at_by_vec(&self, x: &[f64], y: &mut [f64]) {
        let m = self.m as usize;
        let n = self.n as usize;
        for j in 1..=n {
            y[j] = 0.0;
        }
        for i in 1..=m {
            let temp = x[i];
            if temp == 0.0 {
                continue;
            }
            let beg = self.a_ptr[i] as usize;
            let end = self.a_ptr[i+1] as usize;
            for t in beg..end {
                y[self.a_ind[t] as usize] += self.a_val[t] * temp;
            }
        }
    }

    fn decomp_ne(&mut self) {
        adat_numeric(self.m, self.n, &self.p, &self.a_ptr, &self.a_ind, 
                     &self.a_val, &self.d, &mut self.s_ptr, &mut self.s_ind, 
                     &mut self.s_val, &mut self.s_diag);
        chol_numeric(self.m, &self.s_ptr, &self.s_ind, &self.s_val, 
                    &self.s_diag, &mut self.u_ptr, &mut self.u_ind, 
                    &mut self.u_val, &mut self.u_diag);
    }

    fn solve_ne(&mut self, y: &mut [f64]) -> Result<(), GlpError> {
        let m = self.m as usize;
        let n = self.n as usize;
        let mut h = vec![0.0; m + 1];
        h[1..=m].copy_from_slice(&y[1..=m]);
        
        let mut w = vec![0.0; m + 1];
        for i in 1..=m {
            w[i] = y[self.p[i] as usize];
        }
        
        ut_solve(m as i32, &self.u_ptr, &self.u_ind, &self.u_val, 
                &self.u_diag, &mut w);
        u_solve(m as i32, &self.u_ptr, &self.u_ind, &self.u_val, 
               &self.u_diag, &mut w);
        
        for i in 1..=m {
            y[i] = w[self.p[m + i] as usize];
        }
        
        let mut r = vec![0.0; m + 1];
        let mut w2 = vec![0.0; n + 1];
        self.at_by_vec(y, &mut w2);
        
        for j in 1..=n {
            w2[j] *= self.d[j];
        }
        
        self.a_by_vec(&w2, &mut r);
        
        for i in 1..=m {
            r[i] -= h[i];
            if r[i].abs() / (1.0 + h[i].abs()) > 1e-4 {
                return Err(GlpError::NumInstab);
            }
        }
        
        Ok(())
    }

    fn solve_ns(&mut self, p: &[f64], q: &[f64], r: &[f64], 
                dx: &mut [f64], dy: &mut [f64], dz: &mut [f64]) -> Result<(), GlpError> {
        let n = self.n as usize;
        let m = self.m as usize;
        
        for j in 1..=n {
            dx[j] = (self.x[j] * q[j] - r[j]) / self.z[j];
        }
        
        self.a_by_vec(dx, dy);
        
        for i in 1..=m {
            dy[i] += p[i];
        }
        
        self.solve_ne(dy)?;
        
        self.at_by_vec(dy, dx);
        
        for j in 1..=n {
            dx[j] = (self.x[j] * (dx[j] - q[j]) + r[j]) / self.z[j];
            dz[j] = (r[j] - self.z[j] * dx[j]) / self.x[j];
        }
        
        Ok(())
    }

    fn initial