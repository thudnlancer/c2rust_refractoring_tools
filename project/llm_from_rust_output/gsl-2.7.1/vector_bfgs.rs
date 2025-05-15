use gsl::{
    blas::{daxpy, ddot, dnrm2},
    error::{Error, Result},
    vector::Vector,
    multimin::{FunctionFdf, FdfMinimizerType},
};

pub struct VectorBfgsState {
    iter: i32,
    step: f64,
    max_step: f64,
    tol: f64,
    x1: Vector,
    dx1: Vector,
    x2: Vector,
    g0norm: f64,
    pnorm: f64,
    p: Vector,
    x0: Vector,
    g0: Vector,
    dx0: Vector,
    dg0: Vector,
}

impl VectorBfgsState {
    fn new(n: usize) -> Result<Self> {
        Ok(Self {
            iter: 0,
            step: 0.0,
            max_step: 0.0,
            tol: 0.0,
            x1: Vector::new(n)?,
            dx1: Vector::new(n)?,
            x2: Vector::new(n)?,
            g0norm: 0.0,
            pnorm: 0.0,
            p: Vector::new(n)?,
            x0: Vector::new(n)?,
            g0: Vector::new(n)?,
            dx0: Vector::new(n)?,
            dg0: Vector::new(n)?,
        })
    }
}

fn take_step(
    x: &Vector,
    p: &Vector,
    step: f64,
    lambda: f64,
    x1: &mut Vector,
    dx: &mut Vector,
) -> Result<()> {
    dx.set_zero();
    daxpy(-step * lambda, p, dx)?;
    x1.copy_from(x)?;
    daxpy(1.0, dx, x1)?;
    Ok(())
}

fn intermediate_point(
    fdf: &mut FunctionFdf,
    x: &Vector,
    p: &Vector,
    lambda: f64,
    pg: f64,
    stepa: f64,
    stepc: f64,
    fa: f64,
    fc: f64,
    x1: &mut Vector,
    dx: &mut Vector,
    gradient: &mut Vector,
    step: &mut f64,
    f: &mut f64,
) -> Result<()> {
    loop {
        let u = pg.abs() * lambda * stepc;
        let stepb = 0.5 * stepc * u / (fc - fa + u);
        
        take_step(x, p, stepb, lambda, x1, dx)?;
        
        if x == x1 {
            *step = 0.0;
            *f = fa;
            fdf.df(x1, gradient)?;
            return Ok(());
        }
        
        let fb = fdf.f(x1)?;
        if !(fb >= fa && stepb > 0.0) {
            *step = stepb;
            *f = fb;
            fdf.df(x1, gradient)?;
            return Ok(());
        }
        
        stepc = stepb;
        fc = fb;
    }
}

fn minimize(
    fdf: &mut FunctionFdf,
    x: &Vector,
    p: &Vector,
    lambda: f64,
    stepa: f64,
    stepb: f64,
    stepc: f64,
    fa: f64,
    fb: f64,
    fc: f64,
    tol: f64,
    x1: &mut Vector,
    dx1: &mut Vector,
    x2: &mut Vector,
    dx2: &mut Vector,
    gradient: &mut Vector,
    step: &mut f64,
    f: &mut f64,
    gnorm: &mut f64,
) -> Result<()> {
    let mut u = stepb;
    let mut v = stepa;
    let mut w = stepc;
    let mut fu = fb;
    let mut fv = fa;
    let mut fw = fc;
    let mut old2 = (w - v).abs();
    let mut old1 = (v - u).abs();
    let mut iter = 0;
    
    x2.copy_from(x1)?;
    dx2.copy_from(dx1)?;
    *f = fb;
    *step = stepb;
    *gnorm = dnrm2(gradient);
    
    loop {
        iter += 1;
        if iter > 10 {
            return Ok(());
        }
        
        let dw = w - u;
        let dv = v - u;
        let mut du = 0.0;
        
        let e1 = (fv - fu) * dw * dw + (fu - fw) * dv * dv;
        let e2 = 2.0 * ((fv - fu) * dw + (fu - fw) * dv);
        
        if e2 != 0.0 {
            du = e1 / e2;
        }
        
        let stepm = if du > 0.0 && du < stepc - stepb && du.abs() < 0.5 * old2 {
            u + du
        } else if du < 0.0 && du > stepa - stepb && du.abs() < 0.5 * old2 {
            u + du
        } else if stepc - stepb > stepb - stepa {
            0.38 * (stepc - stepb) + stepb
        } else {
            stepb - 0.38 * (stepb - stepa)
        };
        
        take_step(x, p, stepm, lambda, x1, dx1)?;
        let fm = fdf.f(x1)?;
        
        if fm > fb {
            if fm < fv {
                w = v;
                v = stepm;
                fw = fv;
                fv = fm;
            } else if fm < fw {
                w = stepm;
                fw = fm;
            }
            
            if stepm < stepb {
                stepa = stepm;
                fa = fm;
            } else {
                stepc = stepm;
                fc = fm;
            }
        } else {
            if fm > fb {
                break;
            }
            
            old2 = old1;
            old1 = (u - stepm).abs();
            w = v;
            v = u;
            u = stepm;
            fw = fv;
            fv = fu;
            fu = fm;
            
            x2.copy_from(x1)?;
            dx2.copy_from(dx1)?;
            fdf.df(x1, gradient)?;
            
            let mut pg = 0.0;
            ddot(p, gradient, &mut pg)?;
            let gnorm1 = dnrm2(gradient);
            
            *f = fm;
            *step = stepm;
            *gnorm = gnorm1;
            
            if (pg * lambda / gnorm1).abs() < tol {
                return Ok(());
            }
            
            if stepm < stepb {
                stepc = stepb;
                fc = fb;
                stepb = stepm;
                fb = fm;
            } else {
                stepa = stepb;
                fa = fb;
                stepb = stepm;
                fb = fm;
            }
        }
    }
    
    Ok(())
}

pub fn vector_bfgs_minimizer() -> FdfMinimizerType {
    FdfMinimizerType::new(
        "vector_bfgs",
        std::mem::size_of::<VectorBfgsState>(),
        |vstate, n| {
            let state = unsafe { &mut *(vstate as *mut VectorBfgsState) };
            *state = VectorBfgsState::new(n)?;
            Ok(())
        },
        |vstate, fdf, x, f, gradient, step_size, tol| {
            let state = unsafe { &mut *(vstate as *mut VectorBfgsState) };
            state.iter = 0;
            state.step = step_size;
            state.max_step = step_size;
            state.tol = tol;
            
            fdf.fdf(x, f, gradient)?;
            state.x0.copy_from(x)?;
            state.p.copy_from(gradient)?;
            state.g0.copy_from(gradient)?;
            
            let gnorm = dnrm2(gradient);
            state.pnorm = gnorm;
            state.g0norm = gnorm;
            
            Ok(())
        },
        |vstate| {
            let state = unsafe { &mut *(vstate as *mut VectorBfgsState) };
            state.iter = 0;
            Ok(())
        },
        |vstate, fdf, x, f, gradient, dx| {
            let state = unsafe { &mut *(vstate as *mut VectorBfgsState) };
            
            if state.pnorm == 0.0 || state.g0norm == 0.0 {
                dx.set_zero();
                return Err(Error::NoProgress);
            }
            
            let mut pg = 0.0;
            ddot(&state.p, gradient, &mut pg)?;
            
            let dir = if pg >= 0.0 { 1.0 } else { -1.0 };
            take_step(x, &state.p, state.step, dir / state.pnorm, &mut state.x1, &mut state.dx1)?;
            
            let fc = fdf.f(&state.x1)?;
            if fc < *f {
                state.step *= 2.0;
                *f = fc;
                x.copy_from(&state.x1)?;
                fdf.df(&state.x1, gradient)?;
                return Ok(());
            }
            
            let mut stepb = 0.0;
            let mut fb = 0.0;
            intermediate_point(
                fdf,
                x,
                &state.p,
                dir / state.pnorm,
                pg,
                0.0,
                state.step,
                *f,
                fc,
                &mut state.x1,
                &mut state.dx1,
                gradient,
                &mut stepb,
                &mut fb,
            )?;
            
            if stepb == 0.0 {
                return Err(Error::NoProgress);
            }
            
            let mut g1norm = 0.0;
            minimize(
                fdf,
                x,
                &state.p,
                dir / state.pnorm,
                0.0,
                stepb,
                state.step,
                *f,
                fb,
                fc,
                state.tol,
                &mut state.x1,
                &mut state.dx1,
                &mut state.x2,
                dx,
                gradient,
                &mut state.step,
                f,
                &mut g1norm,
            )?;
            
            x.copy_from(&state.x2)?;
            state.iter = (state.iter + 1) % x.len() as i32;
            
            if state.iter == 0 {
                state.p.copy_from(gradient)?;
                state.pnorm = g1norm;
            } else {
                let mut dxg = 0.0;
                let mut dgg = 0.0;
                let mut dxdg = 0.0;
                
                state.dx0.copy_from(x)?;
                daxpy(-1.0, &state.x0, &mut state.dx0)?;
                
                state.dg0.copy_from(gradient)?;
                daxpy(-1.0, &state.g0, &mut state.dg0)?;
                
                ddot(&state.dx0, gradient, &mut dxg)?;
                ddot(&state.dg0, gradient, &mut dgg)?;
                ddot(&state.dx0, &state.dg0, &mut dxdg)?;
                
                let dgnorm = dnrm2(&state.dg0);
                let (a, b) = if dxdg != 0.0 {
                    let b = dxg / dxdg;
                    let a = -(1.0 + dgnorm * dgnorm / dxdg) * b + dgg / dxdg;
                    (a, b)
                } else {
                    (0.0, 0.0)
                };
                
                state.p.copy_from(gradient)?;
                daxpy(-a, &state.dx0, &mut state.p)?;
                daxpy(-b, &state.dg0, &mut state.p)?;
                state.pnorm = dnrm2(&state.p);
            }
            
            state.g0.copy_from(gradient)?;
            state.x0.copy_from(x)?;
            state.g0norm = dnrm2(&state.g0);
            
            Ok(())
        },
        |vstate| {
            let _state = unsafe { Box::from_raw(vstate as *mut VectorBfgsState) };
        },
    )
}