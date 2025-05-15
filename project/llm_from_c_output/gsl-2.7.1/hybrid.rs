use std::f64;
use std::mem;
use std::ptr;
use std::slice;

use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use ndarray_linalg::{QR, Solve};

struct HybridState {
    iter: usize,
    ncfail: usize,
    ncsuc: usize,
    nslow1: usize,
    nslow2: usize,
    fnorm: f64,
    delta: f64,
    j: Array2<f64>,
    q: Array2<f64>,
    r: Array2<f64>,
    tau: Array1<f64>,
    diag: Array1<f64>,
    qtf: Array1<f64>,
    newton: Array1<f64>,
    gradient: Array1<f64>,
    x_trial: Array1<f64>,
    f_trial: Array1<f64>,
    df: Array1<f64>,
    qtdf: Array1<f64>,
    rdx: Array1<f64>,
    w: Array1<f64>,
    v: Array1<f64>,
}

impl HybridState {
    fn new(n: usize) -> Result<Self, &'static str> {
        Ok(HybridState {
            iter: 0,
            ncfail: 0,
            ncsuc: 0,
            nslow1: 0,
            nslow2: 0,
            fnorm: 0.0,
            delta: 0.0,
            j: Array2::zeros((n, n)),
            q: Array2::zeros((n, n)),
            r: Array2::zeros((n, n)),
            tau: Array1::zeros(n),
            diag: Array1::zeros(n),
            qtf: Array1::zeros(n),
            newton: Array1::zeros(n),
            gradient: Array1::zeros(n),
            x_trial: Array1::zeros(n),
            f_trial: Array1::zeros(n),
            df: Array1::zeros(n),
            qtdf: Array1::zeros(n),
            rdx: Array1::zeros(n),
            w: Array1::zeros(n),
            v: Array1::zeros(n),
        })
    }

    fn set(
        &mut self,
        func: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, &'static str>,
        x: &Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
        scale: bool,
    ) -> Result<(), &'static str> {
        *f = func(x)?;

        // Compute finite difference Jacobian
        self.j = finite_difference_jacobian(func, x, f, f64::EPSILON.sqrt())?;

        self.iter = 1;
        self.fnorm = enorm(f.view());
        self.ncfail = 0;
        self.ncsuc = 0;
        self.nslow1 = 0;
        self.nslow2 = 0;

        dx.fill(0.0);

        // Store column norms in diag
        if scale {
            compute_diag(&self.j, &mut self.diag);
        } else {
            self.diag.fill(1.0);
        }

        // Set delta to factor |D x| or to factor if |D x| is zero
        self.delta = compute_delta(&self.diag, x);

        // Factorize J into QR decomposition
        let (q, r) = self.j.view().qr()?;
        self.q = q;
        self.r = r;

        Ok(())
    }

    fn iterate(
        &mut self,
        func: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, &'static str>,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
        scale: bool,
    ) -> Result<(), &'static str> {
        let fnorm = self.fnorm;

        // Compute qtf = Q^T f
        compute_qtf(&self.q, f, &mut self.qtf);

        // Compute dogleg step
        dogleg(
            &self.r,
            &self.qtf,
            &self.diag,
            self.delta,
            &mut self.newton,
            &mut self.gradient,
            dx,
        );

        // Take a trial step
        compute_trial_step(x, dx, &mut self.x_trial);

        let pnorm = scaled_enorm(&self.diag, dx);

        if self.iter == 1 && pnorm < self.delta {
            self.delta = pnorm;
        }

        // Evaluate function at x + p
        self.f_trial = func(&self.x_trial)?;

        // Set df = f_trial - f
        compute_df(&self.f_trial, f, &mut self.df);

        // Compute the scaled actual reduction
        let fnorm1 = enorm(self.f_trial.view());
        let actred = compute_actual_reduction(fnorm, fnorm1);

        // Compute rdx = R dx
        compute_rdx(&self.r, dx, &mut self.rdx);

        // Compute the scaled predicted reduction phi1p = |Q^T f + R dx|
        let fnorm1p = enorm_sum(&self.qtf, &self.rdx);
        let prered = compute_predicted_reduction(fnorm, fnorm1p);

        // Compute the ratio of the actual to predicted reduction
        let ratio = if prered > 0.0 {
            actred / prered
        } else {
            0.0
        };

        // Update the step bound
        if ratio < 0.1 {
            self.ncsuc = 0;
            self.ncfail += 1;
            self.delta *= 0.5;
        } else {
            self.ncfail = 0;
            self.ncsuc += 1;

            if ratio >= 0.5 || self.ncsuc > 1 {
                self.delta = self.delta.max(pnorm / 0.5);
            }
            if (ratio - 1.0).abs() <= 0.1 {
                self.delta = pnorm / 0.5;
            }
        }

        // Test for successful iteration
        if ratio >= 0.0001 {
            x.assign(&self.x_trial);
            f.assign(&self.f_trial);
            self.fnorm = fnorm1;
            self.iter += 1;
        }

        // Determine the progress of the iteration
        self.nslow1 += 1;
        if actred >= 0.001 {
            self.nslow1 = 0;
        }

        if actred >= 0.1 {
            self.nslow2 = 0;
        }

        if self.ncfail == 2 {
            self.j = finite_difference_jacobian(func, x, f, f64::EPSILON.sqrt())?;

            self.nslow2 += 1;

            if self.iter == 1 {
                if scale {
                    compute_diag(&self.j, &mut self.diag);
                }
                self.delta = compute_delta(&self.diag, x);
            } else if scale {
                update_diag(&self.j, &mut self.diag);
            }

            // Factorize J into QR decomposition
            let (q, r) = self.j.view().qr()?;
            self.q = q;
            self.r = r;

            return Ok(());
        }

        // Compute qtdf = Q^T df, w = (Q^T df - R dx)/|dx|, v = D^2 dx/|dx|
        compute_qtf(&self.q, &self.df, &mut self.qtdf);
        compute_wv(
            &self.qtdf,
            &self.rdx,
            dx,
            &self.diag,
            pnorm,
            &mut self.w,
            &mut self.v,
        );

        // Rank-1 update of the jacobian Q'R' = Q(R + w v^T)
        qr_update(&mut self.q, &mut self.r, &self.w, &self.v);

        // No progress as measured by jacobian evaluations
        if self.nslow2 == 5 {
            return Err("No progress in Jacobian evaluations");
        }

        // No progress as measured by function evaluations
        if self.nslow1 == 10 {
            return Err("No progress in function evaluations");
        }

        Ok(())
    }
}

// Helper functions
fn enorm(v: ArrayView1<f64>) -> f64 {
    v.mapv(|x| x * x).sum().sqrt()
}

fn scaled_enorm(diag: &Array1<f64>, v: &Array1<f64>) -> f64 {
    (diag * v).mapv(|x| x * x).sum().sqrt()
}

fn compute_diag(j: &Array2<f64>, diag: &mut Array1<f64>) {
    for (i, mut col) in j.axis_iter(Axis(1)).enumerate() {
        diag[i] = enorm(col);
    }
}

fn compute_delta(diag: &Array1<f64>, x: &Array1<f64>) -> f64 {
    let dx_norm = scaled_enorm(diag, x);
    if dx_norm == 0.0 {
        100.0
    } else {
        dx_norm * 100.0
    }
}

fn compute_qtf(q: &Array2<f64>, f: &Array1<f64>, qtf: &mut Array1<f64>) {
    *qtf = q.t().dot(f);
}

fn dogleg(
    r: &Array2<f64>,
    qtf: &Array1<f64>,
    diag: &Array1<f64>,
    delta: f64,
    newton: &mut Array1<f64>,
    gradient: &mut Array1<f64>,
    dx: &mut Array1<f64>,
) {
    // Solve R newton = -Q^T f
    *newton = r.solve_into(qtf.mapv(|x| -x)).unwrap();

    // Compute gradient = Q^T f
    *gradient = qtf.clone();

    // Compute scaled gradient norm
    let gnorm = scaled_enorm(diag, gradient);

    // Compute scaled newton norm
    let newton_norm = scaled_enorm(diag, newton);

    // Choose step based on norms
    if newton_norm <= delta {
        *dx = newton.clone();
    } else if gnorm * delta <= 1e-4 {
        // Cauchy step
        *dx = gradient.mapv(|x| -x * delta / gnorm);
    } else {
        // Dogleg step
        let alpha = gnorm.powi(2) / gradient.dot(&r.dot(gradient));
        let cauchy = gradient.mapv(|x| -x * alpha);

        let cauchy_norm = scaled_enorm(diag, &cauchy);
        if cauchy_norm >= delta {
            *dx = cauchy.mapv(|x| x * delta / cauchy_norm);
        } else {
            let diff = newton - &cauchy;
            let tau = find_tau(&cauchy, &diff, delta, diag);
            *dx = &cauchy + &(diff.mapv(|x| x * tau));
        }
    }
}

fn find_tau(
    cauchy: &Array1<f64>,
    diff: &Array1<f64>,
    delta: f64,
    diag: &Array1<f64>,
) -> f64 {
    // Solve quadratic equation to find tau
    let a = scaled_enorm(diag, diff).powi(2);
    let b = 2.0 * (cauchy * diff * diag.mapv(|x| x.powi(2))).sum();
    let c = scaled_enorm(diag, cauchy).powi(2) - delta.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        0.0
    } else {
        (-b + discriminant.sqrt()) / (2.0 * a)
    }
}

fn compute_trial_step(x: &Array1<f64>, dx: &Array1<f64>, x_trial: &mut Array1<f64>) {
    *x_trial = x + dx;
}

fn compute_df(f_trial: &Array1<f64>, f: &Array1<f64>, df: &mut Array1<f64>) {
    *df = f_trial - f;
}

fn compute_actual_reduction(fnorm: f64, fnorm1: f64) -> f64 {
    if fnorm1 < fnorm {
        1.0 - (fnorm1 / fnorm).powi(2)
    } else {
        0.0
    }
}

fn compute_rdx(r: &Array2<f64>, dx: &Array1<f64>, rdx: &mut Array1<f64>) {
    *rdx = r.dot(dx);
}

fn enorm_sum(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    (a + b).mapv(|x| x * x).sum().sqrt()
}

fn compute_predicted_reduction(fnorm: f64, fnorm1p: f64) -> f64 {
    if fnorm1p < fnorm {
        1.0 - (fnorm1p / fnorm).powi(2)
    } else {
        0.0
    }
}

fn compute_wv(
    qtdf: &Array1<f64>,
    rdx: &Array1<f64>,
    dx: &Array1<f64>,
    diag: &Array1<f64>,
    pnorm: f64,
    w: &mut Array1<f64>,
    v: &mut Array1<f64>,
) {
    *w = (qtdf - rdx) / pnorm;
    *v = diag.mapv(|x| x.powi(2)) * dx / pnorm;
}

fn qr_update(q: &mut Array2<f64>, r: &mut Array2<f64>, w: &Array1<f64>, v: &Array1<f64>) {
    let n = w.len();
    for k in (0..n).rev() {
        let mut r_k = r.slice_mut(s![k, k..]);
        let w_k = w[k];
        let v_k = v[k];
        
        // Apply Givens rotation to zero out w[k]
        let (c, s) = givens_rotation(r_k[0], w_k);
        
        // Apply rotation to r row and w
        for j in 0..r_k.len() {
            let r_old = r_k[j];
            r_k[j] = c * r_old + s * v[j];
            v[j] = -s * r_old + c * v[j];
        }
        
        // Apply rotation to q columns
        for i in 0..q.nrows() {
            let q_ik = q[(i, k)];
            let q_ik1 = if k < n-1 { q[(i, k+1)] } else { 0.0 };
            q[(i, k)] = c * q_ik + s * q_ik1;
            if k < n-1 {
                q[(i, k+1)] = -s * q_ik + c * q_ik1;
            }
        }
    }
}

fn givens_rotation(a: f64, b: f64) -> (f64, f64) {
    if b == 0.0 {
        (1.0, 0.0)
    } else if b.abs() > a.abs() {
        let tau = -a / b;
        let s = 1.0 / (1.0 + tau.powi(2)).sqrt();
        (s * tau, s)
    } else {
        let tau = -b / a;
        let c = 1.0 / (1.0 + tau.powi(2)).sqrt();
        (c, c * tau)
    }
}

fn finite_difference_jacobian(
    func: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, &'static str>,
    x: &Array1<f64>,
    f: &Array1<f64>,
    eps: f64,
) -> Result<Array2<f64>, &'static str> {
    let n = x.len();
    let mut j = Array2::zeros((n, n));
    let mut xh = x.clone();
    let mut fh = Array1::zeros(n);
    
    for j in 0..n {
        let temp = x[j];
        let h = eps * temp.abs().max(eps);
        xh[j] = temp + h;
        fh = func(&xh)?;
        xh[j] = temp;
        
        for i in 0..n {
            j[(i, j)] = (fh[i] - f[i]) / h;
        }
    }
    
    Ok(j)
}

struct HybridSolver {
    state: HybridState,
    scale: bool,
}

impl HybridSolver {
    fn new(n: usize, scale: bool) -> Result<Self, &'static str> {
        Ok(HybridSolver {
            state: HybridState::new(n)?,
            scale,
        })
    }

    fn set(
        &mut self,
        func: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, &'static str>,
        x: &Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), &'static str> {
        self.state.set(func, x, f, dx, self.scale)
    }

    fn iterate(
        &mut self,
        func: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, &'static str>,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f