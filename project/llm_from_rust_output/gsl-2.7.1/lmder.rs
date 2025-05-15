use gsl::{enums, vector, matrix, permutation, multifit};
use std::f64;

struct LmderState {
    iter: usize,
    xnorm: f64,
    fnorm: f64,
    delta: f64,
    par: f64,
    J: matrix::Matrix,
    r: matrix::Matrix,
    tau: vector::Vector,
    diag: vector::Vector,
    qtf: vector::Vector,
    newton: vector::Vector,
    gradient: vector::Vector,
    x_trial: vector::Vector,
    f_trial: vector::Vector,
    df: vector::Vector,
    sdiag: vector::Vector,
    rptdx: vector::Vector,
    weights: Option<vector::Vector>,
    w: vector::Vector,
    work1: vector::Vector,
    perm: permutation::Permutation,
}

fn enorm(f: &vector::Vector) -> f64 {
    f.norm2()
}

fn scaled_enorm(d: &vector::Vector, f: &vector::Vector) -> f64 {
    let mut e2 = 0.0;
    for i in 0..f.len() {
        let fi = f.get(i);
        let di = d.get(i);
        let u = di * fi;
        e2 += u * u;
    }
    e2.sqrt()
}

fn compute_delta(diag: &vector::Vector, x: &vector::Vector) -> f64 {
    let dx = scaled_enorm(diag, x);
    let factor = 100.0;
    if dx > 0.0 { factor * dx } else { factor }
}

fn compute_actual_reduction(fnorm: f64, fnorm1: f64) -> f64 {
    if 0.1 * fnorm1 < fnorm {
        1.0 - (fnorm1 / fnorm).powi(2)
    } else {
        -1.0
    }
}

fn compute_diag(J: &matrix::Matrix, diag: &mut vector::Vector) {
    for j in 0..J.size2() {
        let col = J.column(j);
        let norm = col.norm2();
        diag.set(j, if norm == 0.0 { 1.0 } else { norm });
    }
}

fn update_diag(J: &matrix::Matrix, diag: &mut vector::Vector) {
    for j in 0..J.size2() {
        let col = J.column(j);
        let norm = col.norm2();
        let diagj = diag.get_mut(j);
        *diagj = if *diagj > norm { *diagj } else { norm };
    }
}

fn compute_rptdx(r: &matrix::Matrix, p: &permutation::Permutation, dx: &vector::Vector, rptdx: &mut vector::Vector) {
    let n = dx.len();
    for i in 0..n {
        let mut sum = 0.0;
        for j in i..n {
            let pj = p.get(j);
            sum += r.get(i, j) * dx.get(pj);
        }
        rptdx.set(i, sum);
    }
}

fn compute_trial_step(x: &vector::Vector, dx: &vector::Vector, x_trial: &mut vector::Vector) {
    for i in 0..x.len() {
        let pi = dx.get(i);
        let xi = x.get(i);
        x_trial.set(i, xi + pi);
    }
}

fn count_nsing(r: &matrix::Matrix) -> usize {
    let n = r.size2();
    for i in 0..n {
        if r.get(i, i) == 0.0 {
            return i;
        }
    }
    n
}

fn compute_newton_direction(r: &matrix::Matrix, perm: &permutation::Permutation, qtf: &vector::Vector, x: &mut vector::Vector) {
    let n = r.size2();
    for i in 0..n {
        x.set(i, qtf.get(i));
    }
    
    let nsing = count_nsing(r);
    for i in nsing..n {
        x.set(i, 0.0);
    }
    
    if nsing > 0 {
        for j in (0..nsing).rev() {
            let rjj = r.get(j, j);
            let temp = x.get(j) / rjj;
            x.set(j, temp);
            for i in 0..j {
                let rij = r.get(i, j);
                let xi = x.get(i);
                x.set(i, xi - rij * temp);
            }
        }
    }
    
    perm.inverse_permute_vector(x);
}

fn compute_newton_correction(r: &matrix::Matrix, sdiag: &vector::Vector, p: &permutation::Permutation, 
                            x: &mut vector::Vector, dxnorm: f64, diag: &vector::Vector, w: &mut vector::Vector) {
    let n = r.size2();
    for i in 0..n {
        let pi = p.get(i);
        let dpi = diag.get(pi);
        let xpi = x.get(pi);
        w.set(i, dpi * (dpi * xpi) / dxnorm);
    }
    
    for j in 0..n {
        let sj = sdiag.get(j);
        let wj = w.get(j);
        let tj = wj / sj;
        w.set(j, tj);
        for i in j+1..n {
            let rij = r.get(i, j);
            let wi = w.get(i);
            w.set(i, wi - rij * tj);
        }
    }
}

fn compute_newton_bound(r: &matrix::Matrix, x: &vector::Vector, dxnorm: f64, 
                        perm: &permutation::Permutation, diag: &vector::Vector, w: &mut vector::Vector) {
    let n = r.size2();
    let nsing = count_nsing(r);
    if nsing < n {
        w.set_zero();
        return;
    }
    
    for i in 0..n {
        let pi = perm.get(i);
        let dpi = diag.get(pi);
        let xpi = x.get(pi);
        w.set(i, dpi * (dpi * xpi / dxnorm));
    }
    
    for j in 0..n {
        let mut sum = 0.0;
        for i in 0..j {
            sum += r.get(i, j) * w.get(i);
        }
        let rjj = r.get(j, j);
        let wj = w.get(j);
        w.set(j, (wj - sum) / rjj);
    }
}

fn compute_gradient_direction(r: &matrix::Matrix, p: &permutation::Permutation, 
                             qtf: &vector::Vector, diag: &vector::Vector, g: &mut vector::Vector) {
    let n = r.size2();
    for j in 0..n {
        let mut sum = 0.0;
        for i in 0..=j {
            sum += r.get(i, j) * qtf.get(i);
        }
        let pj = p.get(j);
        let dpj = diag.get(pj);
        g.set(j, sum / dpj);
    }
}

fn compute_gradient(r: &matrix::Matrix, qtf: &vector::Vector, g: &mut vector::Vector) {
    let n = r.size2();
    for j in 0..n {
        let mut sum = 0.0;
        for i in 0..=j {
            sum += r.get(i, j) * qtf.get(i);
        }
        g.set(j, sum);
    }
}

fn lmpar(r: &mut matrix::Matrix, perm: &permutation::Permutation, qtf: &vector::Vector, 
         diag: &vector::Vector, delta: f64, par: &mut f64, newton: &mut vector::Vector, 
         gradient: &mut vector::Vector, sdiag: &mut vector::Vector, x: &mut vector::Vector, 
         w: &mut vector::Vector) -> enums::Value {
    compute_newton_direction(r, perm, qtf, newton);
    let dxnorm = scaled_enorm(diag, newton);
    let mut fp = dxnorm - delta;
    
    if fp <= 0.1 * delta {
        x.copy(newton);
        *par = 0.0;
        return enums::Value::Success;
    }
    
    compute_newton_bound(r, newton, dxnorm, perm, diag, w);
    let wnorm = enorm(w);
    let phider = wnorm * wnorm;
    
    let par_lower = if wnorm > 0.0 { fp / (delta * phider) } else { 0.0 };
    compute_gradient_direction(r, perm, qtf, diag, gradient);
    let gnorm = enorm(gradient);
    let mut par_upper = gnorm / delta;
    
    if par_upper == 0.0 {
        par_upper = f64::MIN_POSITIVE / delta.min(0.1);
    }
    
    *par = (*par).max(par_lower).min(par_upper);
    if *par == 0.0 {
        *par = gnorm / dxnorm;
    }
    
    let mut iter = 0;
    loop {
        iter += 1;
        if *par == 0.0 {
            *par = (0.001 * par_upper).max(f64::MIN_POSITIVE);
        }
        
        let sqrt_par = par.sqrt();
        qrsolv(r, perm, sqrt_par, diag, qtf, x, sdiag, w);
        let dxnorm = scaled_enorm(diag, x);
        let fp_old = fp;
        fp = dxnorm - delta;
        
        if fp.abs() <= 0.1 * delta {
            break;
        }
        if par_lower == 0.0 && fp <= fp_old && fp_old < 0.0 {
            break;
        }
        if iter == 10 {
            break;
        }
        
        compute_newton_correction(r, sdiag, perm, x, dxnorm, diag, w);
        let wnorm = enorm(w);
        let par_c = fp / (delta * wnorm * wnorm);
        
        if fp > 0.0 {
            if *par > par_lower {
                *par = par_lower.max(*par + par_c);
            }
        } else if fp < 0.0 {
            if *par < par_upper {
                *par = par_upper.max(*par + par_c);
            }
        }
    }
    
    enums::Value::Success
}

fn qrsolv(r: &mut matrix::Matrix, p: &permutation::Permutation, lambda: f64, 
          diag: &vector::Vector, qtb: &vector::Vector, x: &mut vector::Vector, 
          sdiag: &mut vector::Vector, wa: &mut vector::Vector) -> enums::Value {
    let n = r.size2();
    
    for j in 0..n {
        let rjj = r.get(j, j);
        let qtbj = qtb.get(j);
        
        for i in j+1..n {
            let rji = r.get(j, i);
            r.set(i, j, rji);
        }
        
        x.set(j, rjj);
        wa.set(j, qtbj);
    }
    
    for j in 0..n {
        let pj = p.get(j);
        let diagpj = lambda * diag.get(pj);
        
        if diagpj != 0.0 {
            sdiag.set(j, diagpj);
            for k in j+1..n {
                sdiag.set(k, 0.0);
            }
            
            let mut qtbpj = 0.0;
            for k in j..n {
                let sdiagk = sdiag.get(k);
                if sdiagk != 0.0 {
                    let wak = wa.get(k);
                    let rkk = r.get(k, k);
                    
                    let (sine, cosine) = if rkk.abs() < sdiagk.abs() {
                        let cotangent = rkk / sdiagk;
                        let sine = 0.5 / (0.25 + 0.25 * cotangent * cotangent).sqrt();
                        (sine, sine * cotangent)
                    } else {
                        let tangent = sdiagk / rkk;
                        let cosine = 0.5 / (0.25 + 0.25 * tangent * tangent).sqrt();
                        (cosine * tangent, cosine)
                    };
                    
                    let new_rkk = cosine * rkk + sine * sdiagk;
                    let new_wak = cosine * wak + sine * qtbpj;
                    qtbpj = -sine * wak + cosine * qtbpj;
                    
                    r.set(k, k, new_rkk);
                    wa.set(k, new_wak);
                    
                    for i in k+1..n {
                        let rik = r.get(i, k);
                        let sdiagi = sdiag.get(i);
                        let new_rik = cosine * rik + sine * sdiagi;
                        let new_sdiagi = -sine * rik + cosine * sdiagi;
                        r.set(i, k, new_rik);
                        sdiag.set(i, new_sdiagi);
                    }
                }
            }
            
            let rjj = r.get(j, j);
            let xj = x.get(j);
            sdiag.set(j, rjj);
            r.set(j, j, xj);
        }
    }
    
    let nsing = (0..n).find(|&j| sdiag.get(j) == 0.0).unwrap_or(n);
    for j in nsing..n {
        wa.set(j, 0.0);
    }
    
    for k in 0..nsing {
        let j = nsing - 1 - k;
        let mut sum = 0.0;
        for i in j+1..nsing {
            sum += r.get(i, j) * wa.get(i);
        }
        let waj = wa.get(j);
        let sdiagj = sdiag.get(j);
        wa.set(j, (waj - sum) / sdiagj);
    }
    
    for j in 0..n {
        let pj = p.get(j);
        let waj = wa.get(j);
        x.set(pj, waj);
    }
    
    enums::Value::Success
}

fn set(state: &mut LmderState, swts: &vector::Vector, fdf: &mut multifit::FunctionFdf, 
       x: &mut vector::Vector, f: &mut vector::Vector, dx: &mut vector::Vector, scale: i32) -> enums::Value {
    fdf.nevalf = 0;
    fdf.nevaldf = 0;
    
    let status = multifit::eval_wf(fdf, x, swts, f);
    if status != enums::Value::Success {
        return status;
    }
    
    let status = if fdf.df.is_some() {
        multifit::eval_wdf(fdf, x, swts, &mut state.r)
    } else {
        multifit::fdfsolver_dif_df(x, swts, fdf, f, &mut state.r)
    };
    
    state.J.copy(&state.r);
    if status != enums::Value::Success {
        return status;
    }
    
    state.par = 0.0;
    state.iter = 1;
    state.fnorm = enorm(f);
    dx.set_all(0.0);
    
    if scale != 0 {
        compute_diag(&state.r, &mut state.diag);
    } else {
        state.diag.set_all(1.0);
    }
    
    state.xnorm = scaled_enorm(&state.diag, x);
    state.delta = compute_delta(&state.diag, x);
    
    let mut signum = 0;
    state.r.qrpt_decomp(&mut state.tau, &mut state.perm, &mut signum, &mut state.work1);
    state.qtf.copy(f);
    state.r.qr_qtvec(&state.tau, &mut state.qtf);
    
    state.rptdx.set_zero();
    state.w.set_zero();
    state.f_trial.set_zero();
    
    enums::Value::Success
}

fn iterate(state: &mut LmderState, swts: &vector::Vector, fdf: &mut multifit::FunctionFdf, 
           x: &mut vector::Vector, f: &mut vector::Vector, dx: &mut vector::Vector, 
           scale: i32) -> enums::Value {
    if state.fnorm == 0.0 {
        return enums::Value::Success;
    }
    
    compute_gradient_direction(&state.r, &state.perm, &state.qtf, &state.diag, &mut state.gradient);
    let iamax = state.gradient.max_index();
    let gnorm = state.gradient.get(iamax).abs() / state.fnorm;
    
    let mut iter = 0;
    loop {
        iter += 1;
        
        let status = lmpar(&mut state.r, &state.perm, &state.qtf, &state.diag, 
                          state.delta, &mut state.par, &mut state.newton, 
                          &mut state.gradient, &mut state.sdiag, dx, &mut state.w);
        if status != enums::Value::Success {
            return status;
        }
        
        dx.scale(-1.0);
        compute_trial_step(x, dx, &mut state.x_trial);
        let pnorm = scaled_enorm(&state.diag, dx);
        
        if state.iter == 1 && pnorm < state.delta {
            state.delta = pnorm;
        }
        
        let status = multifit::eval_wf(fdf, &state.x_trial, swts, &mut state.f_trial);
        if status != enums::Value::Success {
            return status;
        }
        
        let fnorm1 = enorm(&state.f_trial);
        let actred = compute_actual