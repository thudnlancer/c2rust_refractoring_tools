use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis, s, stack};
use ndarray_linalg::{QR, Solve, Norm};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinAlgError {
    #[error("invalid matrix dimensions")]
    BadDimension,
    #[error("invalid permutation size")]
    BadPermutation,
    #[error("invalid workspace size")]
    BadWorkspace,
    #[error("rank must be <= min(M,N)")]
    InvalidRank,
    #[error("lambda must be positive")]
    InvalidLambda,
    #[error("matrix must have M >= N")]
    InvalidMatrixShape,
}

pub struct CODDecomposition {
    pub qrzt: Array2<f64>,
    pub tau_q: Array1<f64>,
    pub tau_z: Array1<f64>,
    pub perm: Vec<usize>,
    pub rank: usize,
}

impl CODDecomposition {
    pub fn new(
        a: Array2<f64>,
        tau_q: Array1<f64>,
        tau_z: Array1<f64>,
        perm: Vec<usize>,
        tol: f64,
    ) -> Result<Self, LinAlgError> {
        let (m, n) = a.dim();
        let min_mn = m.min(n);

        if tau_q.len() != min_mn {
            return Err(LinAlgError::BadDimension);
        }
        if tau_z.len() != min_mn {
            return Err(LinAlgError::BadDimension);
        }
        if perm.len() != n {
            return Err(LinAlgError::BadPermutation);
        }

        let mut qrzt = a;
        let rank = if tol >= 0.0 {
            qr_rank(&qrzt, tol)
        } else {
            min_mn
        };

        if rank < n {
            let r_upper = qrzt.slice(s![0..rank, ..]);
            cod_rz(&r_upper, &tau_z.slice(s![0..rank]))?;
        }

        Ok(Self {
            qrzt,
            tau_q,
            tau_z,
            perm,
            rank,
        })
    }

    pub fn lssolve(
        &self,
        b: &Array1<f64>,
        x: &mut Array1<f64>,
        residual: &mut Array1<f64>,
    ) -> Result<(), LinAlgError> {
        let (m, n) = self.qrzt.dim();
        if m < n {
            return Err(LinAlgError::InvalidMatrixShape);
        }
        if b.len() != m {
            return Err(LinAlgError::BadDimension);
        }
        if self.rank > min_mn(m, n) {
            return Err(LinAlgError::InvalidRank);
        }
        if x.len() != n {
            return Err(LinAlgError::BadDimension);
        }
        if residual.len() != m {
            return Err(LinAlgError::BadDimension);
        }

        let r11 = self.qrzt.slice(s![0..self.rank, 0..self.rank]);
        let mut qtb1 = residual.slice_mut(s![0..self.rank]);
        let mut x1 = x.slice_mut(s![0..self.rank]);

        x.fill(0.0);
        residual.assign(b);
        qr_qtvec(&self.qrzt, &self.tau_q, residual)?;

        x1.assign(&qtb1);
        solve_triangular(&r11, &mut x1, false, false, false)?;

        householder_zvec(&self.qrzt, &self.tau_z, self.rank, x)?;
        permute_vector_inverse(&self.perm, x);

        qtb1.fill(0.0);
        qr_qvec(&self.qrzt, &self.tau_q, residual)?;

        Ok(())
    }

    pub fn lssolve2(
        &self,
        lambda: f64,
        b: &Array1<f64>,
        x: &mut Array1<f64>,
        residual: &mut Array1<f64>,
        s: &mut Array2<f64>,
        work: &mut Array1<f64>,
    ) -> Result<(), LinAlgError> {
        let (m, n) = self.qrzt.dim();
        if m < n {
            return Err(LinAlgError::InvalidMatrixShape);
        }
        if b.len() != m {
            return Err(LinAlgError::BadDimension);
        }
        if self.rank > min_mn(m, n) {
            return Err(LinAlgError::InvalidRank);
        }
        if x.len() != n {
            return Err(LinAlgError::BadDimension);
        }
        if residual.len() != m {
            return Err(LinAlgError::BadDimension);
        }
        if s.dim() != (self.rank, self.rank) {
            return Err(LinAlgError::BadDimension);
        }
        if work.len() != self.rank {
            return Err(LinAlgError::BadWorkspace);
        }

        let r11 = self.qrzt.slice(s![0..self.rank, 0..self.rank]);
        let mut c1 = residual.slice_mut(s![0..self.rank]);
        let mut y1 = x.slice_mut(s![0..self.rank]);

        x.fill(0.0);
        residual.assign(b);
        qr_qtvec(&self.qrzt, &self.tau_q, residual)?;

        trireg_solve(&r11, lambda, &c1, s, &mut y1, work)?;
        work.assign(&y1);

        householder_zvec(&self.qrzt, &self.tau_z, self.rank, x)?;
        permute_vector_inverse(&self.perm, x);

        let mut work_mat = work.view().into_shape((self.rank, 1)).unwrap();
        let mut r11_work = r11.dot(&work_mat);
        c1 -= &r11_work.slice(s![.., 0]);
        qr_qvec(&self.qrzt, &self.tau_q, residual)?;

        Ok(())
    }

    pub fn unpack(
        &self,
        q: &mut Array2<f64>,
        r: &mut Array2<f64>,
        z: &mut Array2<f64>,
    ) -> Result<(), LinAlgError> {
        let (m, n) = self.qrzt.dim();
        let min_mn = m.min(n);

        if self.tau_q.len() != min_mn {
            return Err(LinAlgError::BadDimension);
        }
        if self.tau_z.len() != min_mn {
            return Err(LinAlgError::BadDimension);
        }
        if self.rank > min_mn {
            return Err(LinAlgError::InvalidRank);
        }
        if q.dim() != (m, m) {
            return Err(LinAlgError::BadDimension);
        }
        if r.dim() != (m, n) {
            return Err(LinAlgError::BadDimension);
        }
        if z.dim() != (n, n) {
            return Err(LinAlgError::BadDimension);
        }

        q.fill(0.0);
        for i in 0..m {
            q[[i, i]] = 1.0;
        }

        for i in (0..min_mn).rev() {
            let h = self.qrzt.slice(s![i.., i]);
            let mut m_block = q.slice_mut(s![i.., i..]);
            let tau = self.tau_q[i];
            householder_left(tau, &h, &mut m_block)?;
        }

        z.fill(0.0);
        for i in 0..n {
            z[[i, i]] = 1.0;
        }

        if self.rank < n {
            let mut work = Array1::zeros(n);
            cod_mat_z(&self.qrzt, &self.tau_z, self.rank, z, &mut work)?;
        }

        r.fill(0.0);
        let mut r11 = r.slice_mut(s![0..self.rank, 0..self.rank]);
        let qrzt11 = self.qrzt.slice(s![0..self.rank, 0..self.rank]);
        r11.assign(&qrzt11);

        Ok(())
    }
}

fn qr_rank(qr: &Array2<f64>, tol: f64) -> usize {
    let diag = qr.diag();
    let max_abs = diag.fold(0.0, |acc, &x| acc.max(x.abs()));
    let threshold = tol * max_abs;
    diag.iter().filter(|&&x| x.abs() > threshold).count()
}

fn cod_rz(a: &Array2<f64>, tau: &Array1<f64>) -> Result<(), LinAlgError> {
    let (m, n) = a.dim();
    if tau.len() != m {
        return Err(LinAlgError::BadDimension);
    }
    if n < m {
        return Err(LinAlgError::BadDimension);
    }

    for k in (0..m).rev() {
        let alpha = a[[k, k]];
        let z = a.slice(s![k, m..]);
        let tauk = householder_transform(alpha, &z.to_owned())?;
        tau[k] = tauk;

        if tauk != 0.0 && k > 0 {
            let w = tau.slice(s![0..k]);
            let b = a.slice(s![0..k, k..]);
            householder_mh(tauk, &z.to_owned(), &b.to_owned(), &w.to_owned())?;
        }
    }

    Ok(())
}

fn householder_transform(alpha: f64, v: &Array1<f64>) -> Result<f64, LinAlgError> {
    let xnorm = v.norm_l2();
    if xnorm == 0.0 {
        return Ok(0.0);
    }

    let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
    let tau = (beta - alpha) / beta;

    let s = alpha - beta;
    if s.abs() > f64::MIN_POSITIVE {
        v.mapv_inplace(|x| x / s);
    } else {
        v.mapv_inplace(|x| x * f64::EPSILON / s);
        v.mapv_inplace(|x| x / f64::EPSILON);
    }

    Ok(tau)
}

fn householder_hv(tau: f64, v: &Array1<f64>, w: &mut Array1<f64>) -> Result<(), LinAlgError> {
    if tau == 0.0 {
        return Ok(());
    }

    let m = w.len();
    let l = v.len();
    let w0 = w[0];
    let mut w1 = w.slice_mut(s![m - l..]);

    let d1 = v.dot(&w1);
    let d = w0 + d1;

    w[0] = w0 - tau * d;
    w1.scaled_add(-tau * d, v);

    Ok(())
}

fn householder_mh(
    tau: f64,
    v: &Array1<f64>,
    a: &Array2<f64>,
    work: &Array1<f64>,
) -> Result<(), LinAlgError> {
    if tau == 0.0 {
        return Ok(());
    }

    let (m, n) = a.dim();
    let l = v.len();
    let mut a1 = a.column(0);
    let c = a.slice(s![.., n - l..]);

    work.assign(&a1);
    work += &c.dot(v);
    a1.scaled_add(-tau, work);
    c.assign(&c - &work.outer(v) * tau);

    Ok(())
}

fn householder_zvec(
    qrzt: &Array2<f64>,
    tau_z: &Array1<f64>,
    rank: usize,
    v: &mut Array1<f64>,
) -> Result<(), LinAlgError> {
    let n = qrzt.shape()[1];
    if tau_z.len() != min_mn(qrzt.shape()[0], qrzt.shape()[1]) {
        return Err(LinAlgError::BadDimension);
    }
    if v.len() != n {
        return Err(LinAlgError::BadDimension);
    }

    if rank < n {
        for i in 0..rank {
            let h = qrzt.slice(s![i, rank..]);
            let mut w = v.slice_mut(s![i..]);
            let ti = tau_z[i];
            householder_hv(ti, &h.to_owned(), &mut w.to_owned())?;
        }
    }

    Ok(())
}

fn trireg_solve(
    r: &Array2<f64>,
    lambda: f64,
    b: &Array1<f64>,
    s: &mut Array2<f64>,
    x: &mut Array1<f64>,
    work: &mut Array1<f64>,
) -> Result<(), LinAlgError> {
    if lambda <= 0.0 {
        return Err(LinAlgError::InvalidLambda);
    }

    let n = r.shape()[1];
    let diag = r.diag();
    work.assign(&diag);
    x.assign(b);

    for j in 0..n {
        let mut bj = 0.0;
        s[[j, j]] = lambda;

        for k in j + 1..n {
            s[[k, k]] = 0.0;
        }

        for k in j..n {
            let xk = x[k];
            let rkk = work[k];
            let skk = s[[k, k]];

            if skk == 0.0 {
                continue;
            }

            let (sine, cosine) = if rkk.abs() < skk.abs() {
                let cotangent = rkk / skk;
                let sine = 0.5 / (0.25 + 0.25 * cotangent.powi(2)).sqrt();
                (sine, sine * cotangent)
            } else {
                let tangent = skk / rkk;
                let cosine = 0.5 / (0.25 + 0.25 * tangent.powi(2)).sqrt();
                (cosine * tangent, cosine)
            };

            let new_rkk = cosine * rkk + sine * skk;
            let new_xk = cosine * xk + sine * bj;
            bj = -sine * xk + cosine * bj;

            work[k] = new_rkk;
            s[[k, k]] = new_rkk;
            x[k] = new_xk;

            for i in k + 1..n {
                let sik = s[[i, k]];
                let sii = s[[i, i]];

                s[[i, k]] = cosine * sik + sine * sii;
                s[[i, i]] = -sine * sik + cosine * sii;
            }
        }
    }

    let s_lower = s.view().into_lower_tri();
    x.solve_triangular_inplace(&s_lower, false)?;

    Ok(())
}

fn permute_vector_inverse(perm: &[usize], v: &mut Array1<f64>) {
    let mut temp = v.to_owned();
    for (i, &p) in perm.iter().enumerate() {
        temp[p] = v[i];
    }
    v.assign(&temp);
}

fn householder_left(tau: f64, h: &ArrayView1<f64>, m: &mut ArrayViewMut2<f64>) -> Result<(), LinAlgError> {
    let mut w = Array1::zeros(m.shape()[1]);
    for j in 0..m.shape()[1] {
        let mut sum = 0.0;
        for i in 0..h.len() {
            sum += h[i] * m[[i, j]];
        }
        w[j] = sum;
    }

    for j in 0..m.shape()[1] {
        for i in 0..h.len() {
            m[[i, j]] -= tau * h[i] * w[j];
        }
    }

    Ok(())
}

fn cod_mat_z(
    qrzt: &Array2<f64>,
    tau_z: &Array1<f64>,
    rank: usize,
    a: &mut Array2<f64>,
    work: &mut Array1<f64>,
) -> Result<(), LinAlgError> {
    let n = qrzt.shape()[1];
    if tau_z.len() != min_mn(qrzt.shape()[0], qrzt.shape()[1]) {
        return Err(LinAlgError::BadDimension);
    }
    if work.len() != a.shape()[0] {
        return Err(LinAlgError::BadWorkspace);
    }

    if rank < n {
        for i in (0..rank).rev() {
            let h = qrzt.slice(s![i, rank..]);
            let mut m = a.slice_mut(s![.., i..]);
            let ti = tau_z[i];
            householder_mh(ti, &h.to_owned(), &m.to_owned(), work)?;
        }
    }

    Ok(())
}

fn qr_qtvec(qr: &Array2<f64>, tau: &Array1<f64>, v: &mut Array1<f64>) -> Result<(), LinAlgError> {
    let m = qr.shape()[0];
    let n = qr.shape()[1];
    let k = tau.len();

    for i in 0..k {
        let h = qr.slice(s![i.., i]);
        let mut v_part = v.slice_mut(s![i..]);
        let tau_val = tau[i];
        householder_hv(tau_val, &h.to_owned(), &mut v_part.to_owned())?;
    }

    Ok(())
}

fn qr_qvec(qr: &Array2