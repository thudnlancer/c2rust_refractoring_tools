use std::f64;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

fn scaled_enorm(d: &ArrayView1<f64>, f: &ArrayView1<f64>) -> f64 {
    let e2 = d.iter()
        .zip(f.iter())
        .map(|(&di, &fi)| (di * fi).powi(2))
        .sum::<f64>();
    e2.sqrt()
}

fn enorm_sum(a: &ArrayView1<f64>, b: &ArrayView1<f64>) -> f64 {
    let e2 = a.iter()
        .zip(b.iter())
        .map(|(&ai, &bi)| (ai + bi).powi(2))
        .sum::<f64>();
    e2.sqrt()
}

fn compute_wv(
    qtdf: &ArrayView1<f64>,
    rdx: &ArrayView1<f64>,
    dx: &ArrayView1<f64>,
    diag: &ArrayView1<f64>,
    pnorm: f64,
    w: &mut Array1<f64>,
    v: &mut Array1<f64>,
) {
    for i in 0..qtdf.len() {
        let qtdfi = qtdf[i];
        let rdxi = rdx[i];
        let dxi = dx[i];
        let diagi = diag[i];
        
        w[i] = (qtdfi - rdxi) / pnorm;
        v[i] = diagi * diagi * dxi / pnorm;
    }
}

fn compute_df(f_trial: &ArrayView1<f64>, f: &ArrayView1<f64>, df: &mut Array1<f64>) {
    for i in 0..f.len() {
        df[i] = f_trial[i] - f[i];
    }
}

fn compute_diag(j: &ArrayView2<f64>, diag: &mut Array1<f64>) {
    for j_col in 0..diag.len() {
        let sum = j.column(j_col)
            .iter()
            .map(|&jij| jij.powi(2))
            .sum::<f64>();
        
        let sum = if sum == 0.0 { 1.0 } else { sum };
        diag[j_col] = sum.sqrt();
    }
}

fn update_diag(j: &ArrayView2<f64>, diag: &mut Array1<f64>) {
    for j_col in 0..diag.len() {
        let sum = j.column(j_col)
            .iter()
            .map(|&jij| jij.powi(2))
            .sum::<f64>();
        
        let sum = if sum == 0.0 { 1.0 } else { sum };
        let cnorm = sum.sqrt();
        let diagj = diag[j_col];
        
        if cnorm > diagj {
            diag[j_col] = cnorm;
        }
    }
}

fn compute_delta(diag: &ArrayView1<f64>, x: &ArrayView1<f64>) -> f64 {
    let dx = scaled_enorm(diag, x);
    let factor = 100.0;
    if dx > 0.0 { factor * dx } else { factor }
}

fn compute_actual_reduction(fnorm: f64, fnorm1: f64) -> f64 {
    if fnorm1 < fnorm {
        let u = fnorm1 / fnorm;
        1.0 - u * u
    } else {
        -1.0
    }
}

fn compute_predicted_reduction(fnorm: f64, fnorm1: f64) -> f64 {
    if fnorm1 < fnorm {
        let u = fnorm1 / fnorm;
        1.0 - u * u
    } else {
        0.0
    }
}

fn compute_qtf(q: &ArrayView2<f64>, f: &ArrayView1<f64>, qtf: &mut Array1<f64>) {
    for j in 0..f.len() {
        let sum = q.column(j)
            .iter()
            .zip(f.iter())
            .map(|(&qij, &fi)| qij * fi)
            .sum();
        qtf[j] = sum;
    }
}

fn compute_rdx(r: &ArrayView2<f64>, dx: &ArrayView1<f64>, rdx: &mut Array1<f64>) {
    for i in 0..dx.len() {
        let sum = r.row(i)
            .iter()
            .skip(i)
            .zip(dx.iter().skip(i))
            .map(|(&rij, &dxj)| rij * dxj)
            .sum();
        rdx[i] = sum;
    }
}

fn compute_trial_step(x: &ArrayView1<f64>, dx: &ArrayView1<f64>, x_trial: &mut Array1<f64>) {
    for i in 0..x.len() {
        x_trial[i] = x[i] + dx[i];
    }
}

fn newton_direction(r: &ArrayView2<f64>, qtf: &ArrayView1<f64>, p: &mut Array1<f64>) -> Result<(), &'static str> {
    // Assuming r is upper triangular and we can solve Rx = -qtf
    let n = r.shape()[1];
    for i in (0..n).rev() {
        let mut sum = qtf[i];
        for j in i+1..n {
            sum -= r[[i, j]] * p[j];
        }
        if r[[i, i]] == 0.0 {
            return Err("Singular matrix in newton_direction");
        }
        p[i] = -sum / r[[i, i]];
    }
    Ok(())
}

fn gradient_direction(
    r: &ArrayView2<f64>,
    qtf: &ArrayView1<f64>,
    diag: &ArrayView1<f64>,
    g: &mut Array1<f64>,
) {
    let m = r.shape()[0];
    let n = r.shape()[1];
    
    for j in 0..m {
        let sum = r.column(j)
            .iter()
            .take(n)
            .zip(qtf.iter())
            .map(|(&rij, &qtfi)| rij * qtfi)
            .sum::<f64>();
        
        let dj = diag[j];
        g[j] = -sum / dj;
    }
}

fn minimum_step(gnorm: f64, diag: &ArrayView1<f64>, g: &mut Array1<f64>) {
    for i in 0..g.len() {
        g[i] = (g[i] / gnorm) / diag[i];
    }
}

fn compute_rg(r: &ArrayView2<f64>, gradient: &ArrayView1<f64>, rg: &mut Array1<f64>) {
    let n = r.shape()[1];
    for i in 0..n {
        let sum = r.row(i)
            .iter()
            .skip(i)
            .zip(gradient.iter().skip(i))
            .map(|(&rij, &gj)| rij * gj)
            .sum();
        rg[i] = sum;
    }
}

fn scaled_addition(
    alpha: f64,
    newton: &ArrayView1<f64>,
    beta: f64,
    gradient: &ArrayView1<f64>,
    p: &mut Array1<f64>,
) {
    for i in 0..p.len() {
        p[i] = alpha * newton[i] + beta * gradient[i];
    }
}

fn dogleg(
    r: &ArrayView2<f64>,
    qtf: &ArrayView1<f64>,
    diag: &ArrayView1<f64>,
    delta: f64,
    newton: &mut Array1<f64>,
    gradient: &mut Array1<f64>,
    p: &mut Array1<f64>,
) -> Result<(), &'static str> {
    newton_direction(r, qtf, newton)?;
    
    let qnorm = scaled_enorm(diag, &newton.view());
    if qnorm <= delta {
        p.assign(newton);
        return Ok(());
    }
    
    gradient_direction(r, qtf, diag, gradient);
    let gnorm = enorm_sum(&gradient.view(), &Array1::zeros(gradient.len()).view());
    
    if gnorm == 0.0 {
        let alpha = delta / qnorm;
        let beta = 0.0;
        scaled_addition(alpha, &newton.view(), beta, &gradient.view(), p);
        return Ok(());
    }
    
    minimum_step(gnorm, diag, gradient);
    compute_rg(r, &gradient.view(), p);  // Using p as temporary space
    
    let temp = enorm_sum(&p.view(), &Array1::zeros(p.len()).view());
    let sgnorm = (gnorm / temp) / temp;
    
    if sgnorm > delta {
        let alpha = 0.0;
        let beta = delta;
        scaled_addition(alpha, &newton.view(), beta, &gradient.view(), p);
        return Ok(());
    }
    
    let bnorm = enorm_sum(qtf, &Array1::zeros(qtf.len()).view());
    let bg = bnorm / gnorm;
    let bq = bnorm / qnorm;
    let dq = delta / qnorm;
    let dq2 = dq * dq;
    let sd = sgnorm / delta;
    let sd2 = sd * sd;
    
    let t1 = bg * bq * sd;
    let u = t1 - dq;
    let t2 = t1 - dq * sd2 + (u * u + (1.0 - dq2) * (1.0 - sd2)).sqrt();
    
    let alpha = dq * (1.0 - sd2) / t2;
    let beta = (1.0 - alpha) * sgnorm;
    
    scaled_addition(alpha, &newton.view(), beta, &gradient.view(), p);
    
    Ok(())
}