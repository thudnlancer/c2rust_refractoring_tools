use std::cmp::min;
use std::f64;
use ndarray::{Array1, Array2, ArrayView1, ArrayViewMut1, Axis};
use ndarray_linalg::{Norm, Solve, Scalar};
use sprs::{CsMat, MulAcc};

/// GMRES state structure
struct GmresState {
    n: usize,               // size of linear system
    m: usize,               // dimension of Krylov subspace K_m
    r: Array1<f64>,         // residual vector r = b - A*x
    h: Array2<f64>,         // Hessenberg matrix n-by-(m+1)
    tau: Array1<f64>,       // householder scalars
    y: Array1<f64>,         // least squares rhs and solution vector
    c: Array1<f64>,         // Givens rotations cosine components
    s: Array1<f64>,         // Givens rotations sine components
    normr: f64,             // residual norm ||r||
}

impl GmresState {
    /// Allocate a new GMRES workspace
    fn new(n: usize, m: usize) -> Result<Self, &'static str> {
        if n == 0 {
            return Err("matrix dimension n must be a positive integer");
        }

        let krylov_m = if m == 0 { min(n, 10) } else { min(n, m) };

        Ok(GmresState {
            n,
            m: krylov_m,
            r: Array1::zeros(n),
            h: Array2::zeros((n, krylov_m + 1)),
            tau: Array1::zeros(krylov_m + 1),
            y: Array1::zeros(krylov_m + 1),
            c: Array1::zeros(krylov_m),
            s: Array1::zeros(krylov_m),
            normr: 0.0,
        })
    }

    /// Get current residual norm
    fn normr(&self) -> f64 {
        self.normr
    }
}

/// GMRES iterator
struct GmresIter<'a> {
    a: &'a CsMat<f64>,
    b: &'a Array1<f64>,
    tol: f64,
    x: Array1<f64>,
    state: GmresState,
}

impl<'a> GmresIter<'a> {
    fn new(
        a: &'a CsMat<f64>,
        b: &'a Array1<f64>,
        tol: f64,
        x: Array1<f64>,
        state: GmresState,
    ) -> Result<Self, &'static str> {
        let n = a.rows();
        if n != a.cols() {
            return Err("matrix must be square");
        }
        if n != b.len() {
            return Err("matrix does not match right hand side");
        }
        if n != x.len() {
            return Err("matrix does not match solution vector");
        }
        if n != state.n {
            return Err("matrix does not match workspace");
        }

        Ok(GmresIter { a, b, tol, x, state })
    }

    fn iterate(&mut self) -> Result<bool, &'static str> {
        let n = self.a.rows();
        let maxit = self.state.m;
        let normb = self.b.norm_l2();
        let reltol = self.tol * normb;
        
        let h = &mut self.state.h;
        let r = &mut self.state.r;
        let w = &mut self.state.y;
        let tau = &mut self.state.tau;
        let c = &mut self.state.c;
        let s = &mut self.state.s;

        // Step 1a: compute r = b - A*x_0
        r.assign(self.b);
        let mut ax = Array1::zeros(n);
        self.a.mul_acc_vec(&self.x.view(), &mut ax);
        *r -= &ax;

        // Initialize Hessenberg matrix
        h.fill(0.0);

        // Step 1b: householder transform of r
        let mut h0 = h.column_mut(0);
        h0.assign(r);
        let tau0 = householder_transform(&mut h0);
        tau[0] = tau0;

        // Initialize w
        w.fill(0.0);
        w[0] = h0[0];

        let mut m = 1;
        for j in 0..maxit {
            // v_m column
            let mut vm = h.column_mut(j + 1);

            // v_m(m:end)
            let mut vv = vm.slice_mut(ndarray::s![j..]);

            // householder vector u_m
            let um = h.slice(ndarray::s![j.., j]);

            // Step 2a: form v_m = P_m e_m = e_m - tau_m w_m
            vm.fill(0.0);
            vv.assign(&um);
            let taum = tau[j];
            vv *= -taum;
            vv[0] = 1.0 - taum;

            // Step 2a: v_m <- P_1 P_2 ... P_{m-1} v_m
            for k in (0..j).rev() {
                let uk = h.slice(ndarray::s![k.., k]);
                let mut vk = vm.slice_mut(ndarray::s![k..]);
                let tauk = tau[k];
                householder_hv(tauk, &uk, &mut vk);
            }

            // Step 2a: v_m <- A*v_m
            self.a.mul_acc_vec(&vm, r);
            vm.assign(r);

            // Step 2a: v_m <- P_m ... P_1 v_m
            for k in 0..=j {
                let uk = h.slice(ndarray::s![k.., k]);
                let mut vk = vm.slice_mut(ndarray::s![k..]);
                let tauk = tau[k];
                householder_hv(tauk, &uk, &mut vk);
            }

            // Steps 2c,2d: find P_{m+1} and set v_m <- P_{m+1} v_m
            if m < n {
                let mut ump1 = h.slice_mut(ndarray::s![m.., m]);
                let taump1 = householder_transform(&mut ump1);
                tau[j + 1] = taump1;
            }

            // Step 2e: v_m <- J_{m-1} ... J_1 v_m
            for k in 0..j {
                givens_gv(&mut vm, k, k + 1, c[k], s[k]);
            }

            if m < n {
                // Step 2g: find givens rotation J_m
                let (cj, sj) = givens_rotation(vm[j], vm[j + 1]);
                c[j] = cj;
                s[j] = sj;

                // Step 2h: v_m <- J_m v_m
                givens_gv(&mut vm, j, j + 1, cj, sj);

                // Step 2h: w <- J_m w
                givens_gv(w, j, j + 1, cj, sj);
            }

            // Check convergence
            let normr = w[j + 1].abs();
            if normr <= reltol {
                break;
            }

            m += 1;
        }

        // Rewind m if we exceeded maxit iterations
        if m > maxit {
            m = maxit;
        }

        // Step 3a: solve triangular system R_m y_m = w
        let rm = h.slice(ndarray::s![0..m, 1..=m]);
        let mut ym = w.slice_mut(ndarray::s![0..m]);
        rm.solve_triangular_into(ndarray::linalg::UPLO::Upper, false, false, &mut ym)
            .map_err(|_| "failed to solve triangular system")?;

        // Step 3b: update solution vector x
        let mut krylov_proj = Array1::zeros(n);
        for k in (0..m).rev() {
            let ymk = ym[k];
            let uk = h.slice(ndarray::s![k.., k]);
            let mut rk = krylov_proj.slice_mut(ndarray::s![k..]);

            krylov_proj[k] += ymk;
            let tauk = tau[k];
            householder_hv(tauk, &uk, &mut rk);
        }

        self.x += &krylov_proj;

        // Compute new residual
        r.assign(self.b);
        self.a.mul_acc_vec(&self.x.view(), &mut ax);
        *r -= &ax;
        self.state.normr = r.norm_l2();

        Ok(self.state.normr <= reltol)
    }
}

/// Householder transformation
fn householder_transform(v: &mut ArrayViewMut1<f64>) -> f64 {
    let norm = v.slice(ndarray::s![1..]).norm_l2();
    let alpha = v[0];
    let beta = -alpha.signum() * (alpha.powi(2) + norm.powi(2)).sqrt();
    let tau = (beta - alpha) / beta;
    
    v[0] = beta;
    v.slice_mut(ndarray::s![1..]) /= (alpha - beta);
    
    tau
}

/// Apply Householder transformation
fn householder_hv(tau: f64, u: &ArrayView1<f64>, v: &mut ArrayViewMut1<f64>) {
    let dot = u.dot(v);
    *v -= &(u * (tau * dot));
}

/// Givens rotation
fn givens_rotation(a: f64, b: f64) -> (f64, f64) {
    let r = a.hypot(b);
    (a / r, -b / r)
}

/// Apply Givens rotation to vector
fn givens_gv(v: &mut ArrayViewMut1<f64>, i: usize, j: usize, c: f64, s: f64) {
    let vi = v[i];
    let vj = v[j];
    v[i] = c * vi - s * vj;
    v[j] = s * vi + c * vj;
}

/// GMRES solver type
pub struct GmresSolver {
    state: GmresState,
}

impl GmresSolver {
    /// Create a new GMRES solver
    pub fn new(n: usize, m: usize) -> Result<Self, &'static str> {
        Ok(GmresSolver {
            state: GmresState::new(n, m)?,
        })
    }

    /// Solve the linear system A*x = b
    pub fn solve(
        &mut self,
        a: &CsMat<f64>,
        b: &Array1<f64>,
        tol: f64,
        x: &mut Array1<f64>,
    ) -> Result<bool, &'static str> {
        let mut iter = GmresIter::new(a, b, tol, x.clone(), self.state)?;
        let converged = iter.iterate()?;
        *x = iter.x;
        self.state = iter.state;
        Ok(converged)
    }

    /// Get current residual norm
    pub fn normr(&self) -> f64 {
        self.state.normr()
    }
}