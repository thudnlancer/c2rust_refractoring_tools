use gsl::{Vector, Matrix, EigenSymmvWorkspace};
use std::convert::TryInto;
use std::f64;
use std::ptr;

#[derive(Debug)]
pub struct MathieuWorkspace {
    size: usize,
    even_order: usize,
    odd_order: usize,
    extra_values: i32,
    qa: f64,
    qb: f64,
    aa: Vec<f64>,
    bb: Vec<f64>,
    dd: Vec<f64>,
    ee: Vec<f64>,
    tt: Vec<f64>,
    e2: Vec<f64>,
    zz: Vec<f64>,
    eval: Vector,
    evec: Matrix,
    wmat: EigenSymmvWorkspace,
}

impl MathieuWorkspace {
    pub fn new(nn: usize, qq: f64) -> Option<Self> {
        if nn == 0 {
            gsl::error("matrix dimension must be positive integer", "mathieu_workspace.c", 43, gsl::Value::Invalid);
            return None;
        }

        let mut even_order = (nn / 2 + 1) as u32;
        let mut odd_order = ((nn + 1) / 2) as u32;
        
        let extra_values = ((2.1 * qq.abs().powf(0.37)) as i32 + 9 + 20) as u32;
        
        even_order += extra_values;
        odd_order += extra_values;

        let even_order_size = even_order as usize;
        let odd_order_size = odd_order as usize;
        let nn_size = nn + 1;

        let aa = vec![0.0; nn_size];
        let bb = vec![0.0; nn_size];
        let dd = vec![0.0; even_order_size];
        let ee = vec![0.0; even_order_size];
        let tt = vec![0.0; 3 * even_order_size];
        let e2 = vec![0.0; even_order_size];
        let zz = vec![0.0; even_order_size * even_order_size];

        let eval = match Vector::new(even_order_size) {
            Some(v) => v,
            None => {
                gsl::error("failed to allocate space for eval", "mathieu_workspace.c", 150, gsl::Value::NoMemory);
                return None;
            }
        };

        let evec = match Matrix::new(even_order_size, even_order_size) {
            Some(m) => m,
            None => {
                gsl::error("failed to allocate space for evec", "mathieu_workspace.c", 166, gsl::Value::NoMemory);
                return None;
            }
        };

        let wmat = match EigenSymmvWorkspace::new(even_order_size) {
            Some(w) => w,
            None => {
                gsl::error("failed to allocate space for wmat", "mathieu_workspace.c", 183, gsl::Value::NoMemory);
                return None;
            }
        };

        Some(Self {
            size: nn,
            even_order: even_order_size,
            odd_order: odd_order_size,
            extra_values: extra_values as i32,
            qa: 0.0,
            qb: 0.0,
            aa,
            bb,
            dd,
            ee,
            tt,
            e2,
            zz,
            eval,
            evec,
            wmat,
        })
    }
}

impl Drop for MathieuWorkspace {
    fn drop(&mut self) {
        // All resources will be automatically dropped due to Vec and gsl types implementing Drop
    }
}