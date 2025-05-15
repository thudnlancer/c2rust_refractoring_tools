use std::f64::consts::PI;
use std::cmp::{max, min};
use std::f64::EPSILON as DBL_EPSILON;

const GSL_FRANCIS_COEFF1: f64 = 0.75;
const GSL_FRANCIS_COEFF2: f64 = -0.4375;

#[derive(Debug, Clone)]
pub struct GslComplex {
    pub dat: [f64; 2],
}

impl GslComplex {
    pub fn new(re: f64, im: f64) -> Self {
        GslComplex { dat: [re, im] }
    }

    pub fn set(&mut self, re: f64, im: f64) {
        self.dat[0] = re;
        self.dat[1] = im;
    }

    pub fn re(&self) -> f64 {
        self.dat[0]
    }

    pub fn im(&self) -> f64 {
        self.dat[1]
    }
}

#[derive(Debug)]
pub struct GslMatrix {
    pub size1: usize,
    pub size2: usize,
    pub data: Vec<f64>,
    pub tda: usize,
}

impl GslMatrix {
    pub fn new(size1: usize, size2: usize) -> Self {
        let data = vec![0.0; size1 * size2];
        GslMatrix {
            size1,
            size2,
            data,
            tda: size2,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.tda + j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        self.data[i * self.tda + j] = value;
    }

    pub fn submatrix(&self, i: usize, j: usize, n1: usize, n2: usize) -> GslMatrixView {
        GslMatrixView {
            matrix: self,
            offset: i * self.tda + j,
            n1,
            n2,
            tda: self.tda,
        }
    }

    pub fn subrow(&self, i: usize, j: usize, n: usize) -> GslVectorView {
        GslVectorView {
            matrix: self,
            offset: i * self.tda + j,
            stride: 1,
            n,
        }
    }

    pub fn subcolumn(&self, j: usize, i: usize, n: usize) -> GslVectorView {
        GslVectorView {
            matrix: self,
            offset: i * self.tda + j,
            stride: self.tda,
            n,
        }
    }
}

#[derive(Debug)]
pub struct GslMatrixView<'a> {
    matrix: &'a GslMatrix,
    offset: usize,
    n1: usize,
    n2: usize,
    tda: usize,
}

impl<'a> GslMatrixView<'a> {
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.matrix.data[self.offset + i * self.tda + j]
    }

    pub fn set(&self, i: usize, j: usize, value: f64) {
        let idx = self.offset + i * self.tda + j;
        unsafe {
            *self.matrix.data.as_ptr().add(idx) as *mut f64 = value;
        }
    }
}

#[derive(Debug)]
pub struct GslVectorView<'a> {
    matrix: &'a GslMatrix,
    offset: usize,
    stride: usize,
    n: usize,
}

impl<'a> GslVectorView<'a> {
    pub fn get(&self, i: usize) -> f64 {
        self.matrix.data[self.offset + i * self.stride]
    }

    pub fn set(&self, i: usize, value: f64) {
        let idx = self.offset + i * self.stride;
        unsafe {
            *self.matrix.data.as_ptr().add(idx) as *mut f64 = value;
        }
    }
}

#[derive(Debug)]
pub struct GslVectorComplex {
    pub size: usize,
    pub data: Vec<GslComplex>,
}

impl GslVectorComplex {
    pub fn new(size: usize) -> Self {
        let data = vec![GslComplex::new(0.0, 0.0); size];
        GslVectorComplex { size, data }
    }

    pub fn set(&mut self, i: usize, value: GslComplex) {
        self.data[i] = value;
    }

    pub fn get(&self, i: usize) -> &GslComplex {
        &self.data[i]
    }
}

#[derive(Debug)]
pub struct GslEigenFrancisWorkspace {
    pub size: usize,
    pub max_iterations: usize,
    pub n_iter: usize,
    pub n_evals: usize,
    pub compute_t: bool,
    pub z: Option<GslMatrix>,
    pub h: Option<GslMatrix>,
}

impl GslEigenFrancisWorkspace {
    pub fn new() -> Self {
        GslEigenFrancisWorkspace {
            size: 0,
            max_iterations: 0,
            n_iter: 0,
            n_evals: 0,
            compute_t: false,
            z: None,
            h: None,
        }
    }

    pub fn alloc() -> Option<Self> {
        Some(Self::new())
    }

    pub fn free(&mut self) {
        // No-op in Rust, memory is managed by drop
    }

    pub fn set_compute_t(&mut self, compute_t: bool) {
        self.compute_t = compute_t;
    }
}

pub fn gsl_eigen_francis(
    h: &mut GslMatrix,
    eval: &mut GslVectorComplex,
    w: &mut GslEigenFrancisWorkspace,
) -> Result<(), &'static str> {
    if h.size1 != h.size2 {
        return Err("matrix must be square to compute eigenvalues");
    } else if eval.size != h.size1 {
        return Err("eigenvalue vector must match matrix size");
    } else {
        let n = h.size1;
        w.size = n;
        w.max_iterations = 30 * n;
        w.h = Some(h.clone());
        w.n_iter = 0;
        w.n_evals = 0;

        for j in 0..n - 3 {
            h.set(j + 2, j, 0.0);
            h.set(j + 3, j, 0.0);
        }

        if n > 2 {
            h.set(n - 1, n - 3, 0.0);
        }

        francis_schur_decomp(h, eval, w);

        if w.n_evals != n {
            return Err("maximum iterations reached without finding all eigenvalues");
        }

        Ok(())
    }
}

pub fn gsl_eigen_francis_z(
    h: &mut GslMatrix,
    eval: &mut GslVectorComplex,
    z: &mut GslMatrix,
    w: &mut GslEigenFrancisWorkspace,
) -> Result<(), &'static str> {
    w.z = Some(z.clone());
    let result = gsl_eigen_francis(h, eval, w);
    w.z = None;
    result
}

fn francis_schur_decomp(
    h: &mut GslMatrix,
    eval: &mut GslVectorComplex,
    w: &mut GslEigenFrancisWorkspace,
) {
    let n = h.size1;
    let mut m = h.submatrix(0, 0, n, n);
    let mut lambda1 = GslComplex::new(0.0, 0.0);
    let mut lambda2 = GslComplex::new(0.0, 0.0);

    while n > 2 && w.n_iter < w.max_iterations {
        w.n_iter += 1;
        let q = francis_search_subdiag_small_elements(&m);

        if q == 0 {
            francis_qrstep(&mut m, w).unwrap();
            continue;
        }

        if q == n - 1 {
            lambda1.set(m.get(q, q), 0.0);
            eval.set(w.n_evals, lambda1);
            w.n_evals += 1;
            w.n_iter = 0;
            m = h.submatrix(0, 0, n - 1, n - 1);
        } else if q == n - 2 {
            let v = h.submatrix(q, q, 2, 2);
            francis_schur_standardize(&v, &mut lambda1, &mut lambda2, w);
            eval.set(w.n_evals, lambda1);
            eval.set(w.n_evals + 1, lambda2);
            w.n_evals += 2;
            w.n_iter = 0;
            m = h.submatrix(0, 0, n - 2, n - 2);
        } else if q == 1 {
            lambda1.set(m.get(0, 0), 0.0);
            eval.set(w.n_evals, lambda1);
            w.n_evals += 1;
            w.n_iter = 0;
            m = h.submatrix(1, 1, n - 1, n - 1);
        } else if q == 2 {
            let v = h.submatrix(0, 0, 2, 2);
            francis_schur_standardize(&v, &mut lambda1, &mut lambda2, w);
            eval.set(w.n_evals, lambda1);
            eval.set(w.n_evals + 1, lambda2);
            w.n_evals += 2;
            w.n_iter = 0;
            m = h.submatrix(2, 2, n - 2, n - 2);
        } else {
            let v = h.submatrix(q, q, n - q, n - q);
            francis_schur_decomp(&mut v, eval, w);
            let v = h.submatrix(0, 0, q, q);
            francis_schur_decomp(&mut v, eval, w);
            break;
        }
    }

    if n == 1 {
        lambda1.set(h.get(0, 0), 0.0);
        eval.set(w.n_evals, lambda1);
        w.n_evals += 1;
        w.n_iter = 0;
    } else if n == 2 {
        let m = h.submatrix(0, 0, 2, 2);
        francis_schur_standardize(&m, &mut lambda1, &mut lambda2, w);
        eval.set(w.n_evals, lambda1);
        eval.set(w.n_evals + 1, lambda2);
        w.n_evals += 2;
        w.n_iter = 0;
    }
}

fn francis_qrstep(h: &mut GslMatrix, w: &mut GslEigenFrancisWorkspace) -> Result<(), &'static str> {
    let n = h.size1;
    let mut dat = [0.0; 3];
    let mut cs = 0.0;
    let mut sn = 0.0;
    let mut scale;
    let mut h_nn;
    let mut h_nm1nm1;
    let mut h_cross;
    let mut h_tmp1;
    let mut h_tmp2;

    if w.n_iter % 10 == 0 {
        let s = h.get(n - 1, n - 2).abs() + h.get(n - 2, n - 3).abs();
        h_nn = h.get(n - 1, n - 1) + GSL_FRANCIS_COEFF1 * s;
        h_nm1nm1 = h_nn;
        h_cross = GSL_FRANCIS_COEFF2 * s * s;
    } else {
        h_nn = h.get(n - 1, n - 1);
        h_nm1nm1 = h.get(n - 2, n - 2);
        h_cross = h.get(n - 1, n - 2) * h.get(n - 2, n - 1);

        let disc = 0.5 * (h_nm1nm1 - h_nn);
        let disc_sq = disc * disc + h_cross;
        if disc_sq > 0.0 {
            let disc_root = disc_sq.sqrt();
            let ave = 0.5 * (h_nm1nm1 + h_nn);
            if h_nm1nm1.abs() - h_nn.abs() > 0.0 {
                h_nm1nm1 = h_nm1nm1 * h_nn - h_cross;
                h_nn = h_nm1nm1 / (disc_root * ave.signum() + ave);
            } else {
                h_nn = disc_root * ave.signum() + ave;
            }
            h_nm1nm1 = h_nn;
            h_cross = 0.0;
        }
    }

    h_tmp1 = h_nm1nm1 - h.get(0, 0);
    h_tmp2 = h_nn - h.get(0, 0);
    dat[0] = (h_tmp1 * h_tmp2 - h_cross) / h.get(1, 0) + h.get(0, 1);
    dat[1] = h.get(1, 1) - h.get(0, 0) - h_tmp1 - h_tmp2;
    dat[2] = h.get(2, 1);

    scale = dat[0].abs() + dat[1].abs() + dat[2].abs();
    if scale != 0.0 {
        dat[0] /= scale;
        dat[1] /= scale;
        dat[2] /= scale;
    }

    for i in 0..n - 2 {
        let tau = householder_transform(&mut dat);
        if tau != 0.0 {
            let q = if 1 > i as i32 - 1 { 0 } else { i - 1 };
            let r = min(i + 3, n - 1);

            if w.compute_t {
                let h_ref = w.h.as_ref().unwrap();
                let mut m = h_ref.submatrix(i, q, 3, n - q);
                householder_hm(tau, &dat, &mut m);
                let mut m = h_ref.submatrix(0, i, r + 1, 3);
                householder_mh(tau, &dat, &mut m);
            } else {
                let mut m = h.submatrix(i, q, 3, n - q);
                householder_hm(tau, &dat, &mut m);
                let mut m = h.submatrix(0, i, r + 1, 3);
                householder_mh(tau, &dat, &mut m);
            }

            if let Some(z) = &mut w.z {
                let mut m = z.submatrix(0, i, n, 3);
                householder_mh(tau, &dat, &mut m);
            }
        }

        dat[0] = h.get(i + 1, i);
        dat[1] = h.get(i + 2, i);
        if i < n - 3 {
            dat[2] = h.get(i + 3, i);
        }

        scale = dat[0].abs() + dat[1].abs() + dat[2].abs();
        if scale != 0.0 {
            dat[0] /= scale;
            dat[1] /= scale;
            dat[2] /= scale;
        }
    }

    scale = dat[0].abs() + dat[1].abs();
    if scale != 0.0 {
        dat[0] /= scale;
        dat[1] /= scale;
    }

    let tau = householder_transform(&mut dat[..2]);
    if w.compute_t {
        let h_ref = w.h.as_ref().unwrap();
        let mut m = h_ref.submatrix(n - 2, n - 3, 2, n - n + 3);
        householder_hm(tau, &dat[..2], &mut m);
        let mut m = h_ref.submatrix(0, n - 2, n, 2);
        householder_mh(tau, &dat[..2], &mut m);
    } else {
        let mut m = h.submatrix(n - 2, n - 3, 2, 3);
        householder_hm(tau, &dat[..2], &mut m);
        let mut m = h.submatrix(0, n - 2, n, 2);
        householder_mh(tau, &dat[..2], &mut m);
    }

    if let Some(z) = &mut w.z {
        let mut m = z.submatrix(0, n - 2, n, 2);
        householder_mh(tau, &dat[..2], &mut m);
    }

    Ok(())
}

fn francis_search_subdiag_small_elements(a: &GslMatrix) -> usize {
    let n = a.size1;
    let mut dpel = a.get(n - 2, n - 2);

    for i in (1..n).rev() {
        let sel = a.get(i, i - 1);
        let del = a.get(i, i);

        if sel == 0.0 || sel.abs() < DBL_EPSILON * (del.abs() + dpel.abs()) {
            return i;
        }

        dpel = del;
    }

    0
}

fn francis_schur_standardize(
    a: &GslMatrix,
    eval1: &mut GslComplex,
    eval2: &mut GslComplex,
    w: &mut GslEigenFrancisWorkspace,
) {
    let n = w.size;
    let mut cs = 0.0;
    let mut sn = 0.0;

    francis_standard_form(a, &mut cs, &mut sn);

    eval1.set(a.get(0, 0), 0.0);
    eval2.set(a.get(1, 1), 0.0);
    if a.get(1, 0) != 0