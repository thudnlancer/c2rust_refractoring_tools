use std::f64;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singularity = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone, Copy)]
pub struct GslFunction {
    pub function: Option<fn(f64, *mut std::ffi::c_void) -> f64>,
    pub params: *mut std::ffi::c_void,
}

#[derive(Debug, Clone, Copy)]
pub struct GslIntegrationWorkspace {
    pub limit: usize,
    pub size: usize,
    pub nrmax: usize,
    pub i: usize,
    pub maximum_level: usize,
    pub alist: *mut f64,
    pub blist: *mut f64,
    pub rlist: *mut f64,
    pub elist: *mut f64,
    pub order: *mut usize,
    pub level: *mut usize,
}

#[derive(Debug, Clone, Copy)]
pub struct GslIntegrationQawsTable {
    pub alpha: f64,
    pub beta: f64,
    pub mu: i32,
    pub nu: i32,
    pub ri: [f64; 25],
    pub rj: [f64; 25],
    pub rg: [f64; 25],
    pub rh: [f64; 25],
}

#[derive(Debug, Clone, Copy)]
struct FnQawsParams {
    function: *mut GslFunction,
    a: f64,
    b: f64,
    table: *mut GslIntegrationQawsTable,
}

fn gsl_max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn initialise(workspace: &mut GslIntegrationWorkspace, a: f64, b: f64) {
    unsafe {
        *workspace.alist = a;
        *workspace.blist = b;
        *workspace.rlist = 0.0;
        *workspace.elist = 0.0;
        *workspace.order = 0;
        *workspace.level = 0;
    }
    workspace.size = 0;
    workspace.nrmax = 0;
    workspace.i = 0;
    workspace.maximum_level = 0;
}

fn append_interval(
    workspace: &mut GslIntegrationWorkspace,
    a1: f64,
    b1: f64,
    area1: f64,
    error1: f64,
) {
    unsafe {
        *workspace.alist.add(workspace.size) = a1;
        *workspace.blist.add(workspace.size) = b1;
        *workspace.rlist.add(workspace.size) = area1;
        *workspace.elist.add(workspace.size) = error1;
        *workspace.order.add(workspace.size) = workspace.size;
        *workspace.level.add(workspace.size) = 0;
    }
    workspace.size += 1;
}

fn qpsrt(workspace: &mut GslIntegrationWorkspace) {
    let last = workspace.size - 1;
    let limit = workspace.limit;
    let elist = workspace.elist;
    let order = workspace.order;
    let mut i_nrmax = workspace.nrmax;
    let mut i_maxerr = unsafe { *order.add(i_nrmax) };

    if last < 2 {
        unsafe {
            *order = 0;
            *order.add(1) = 1;
        }
        workspace.i = i_maxerr;
        return;
    }

    let mut errmax = unsafe { *elist.add(i_maxerr) };

    while i_nrmax > 0 && errmax > unsafe { *elist.add(*order.add(i_nrmax - 1)) } {
        unsafe {
            *order.add(i_nrmax) = *order.add(i_nrmax - 1);
        }
        i_nrmax -= 1;
    }

    let top = if last < limit / 2 + 2 {
        last as i32
    } else {
        (limit - last + 1) as i32
    };

    let mut i = i_nrmax as i32 + 1;
    while i < top && errmax < unsafe { *elist.add(*order.add(i as usize)) } {
        unsafe {
            *order.add((i - 1) as usize) = *order.add(i as usize);
        }
        i += 1;
    }

    unsafe {
        *order.add((i - 1) as usize) = i_maxerr;
    }

    let errmin = unsafe { *elist.add(last) };
    let mut k = top - 1;
    while k > i - 2 && errmin >= unsafe { *elist.add(*order.add(k as usize)) } {
        unsafe {
            *order.add((k + 1) as usize) = *order.add(k as usize);
        }
        k -= 1;
    }

    unsafe {
        *order.add((k + 1) as usize) = last;
    }

    i_maxerr = unsafe { *order.add(i_nrmax) };
    workspace.i = i_maxerr;
    workspace.nrmax = i_nrmax;
}

fn update(
    workspace: &mut GslIntegrationWorkspace,
    a1: f64,
    b1: f64,
    area1: f64,
    error1: f64,
    a2: f64,
    b2: f64,
    area2: f64,
    error2: f64,
) {
    let i_max = workspace.i;
    let i_new = workspace.size;
    let new_level = unsafe { *workspace.level.add(i_max) } + 1;

    if error2 > error1 {
        unsafe {
            *workspace.alist.add(i_max) = a2;
            *workspace.rlist.add(i_max) = area2;
            *workspace.elist.add(i_max) = error2;
            *workspace.level.add(i_max) = new_level;
            *workspace.alist.add(i_new) = a1;
            *workspace.blist.add(i_new) = b1;
            *workspace.rlist.add(i_new) = area1;
            *workspace.elist.add(i_new) = error1;
            *workspace.level.add(i_new) = new_level;
        }
    } else {
        unsafe {
            *workspace.blist.add(i_max) = b1;
            *workspace.rlist.add(i_max) = area1;
            *workspace.elist.add(i_max) = error1;
            *workspace.level.add(i_max) = new_level;
            *workspace.alist.add(i_new) = a2;
            *workspace.blist.add(i_new) = b2;
            *workspace.rlist.add(i_new) = area2;
            *workspace.elist.add(i_new) = error2;
            *workspace.level.add(i_new) = new_level;
        }
    }

    workspace.size += 1;
    if new_level > workspace.maximum_level {
        workspace.maximum_level = new_level;
    }
    qpsrt(workspace);
}

fn retrieve(
    workspace: &GslIntegrationWorkspace,
    a: &mut f64,
    b: &mut f64,
    r: &mut f64,
    e: &mut f64,
) {
    let i = workspace.i;
    unsafe {
        *a = *workspace.alist.add(i);
        *b = *workspace.blist.add(i);
        *r = *workspace.rlist.add(i);
        *e = *workspace.elist.add(i);
    }
}

fn sum_results(workspace: &GslIntegrationWorkspace) -> f64 {
    let mut result_sum = 0.0;
    for k in 0..workspace.size {
        result_sum += unsafe { *workspace.rlist.add(k) };
    }
    result_sum
}

fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let e = 2.2204460492503131e-16;
    let u = 2.2250738585072014e-308;
    let tmp = (1.0 + 100.0 * e) * (a2.abs() + 1000.0 * u);
    a1.abs() <= tmp && b2.abs() <= tmp
}

fn qc25s(
    f: &mut GslFunction,
    a: f64,
    b: f64,
    a1: f64,
    b1: f64,
    t: &mut GslIntegrationQawsTable,
    result: &mut f64,
    abserr: &mut f64,
    err_reliable: &mut bool,
) {
    let mut weighted_function = GslFunction {
        function: None,
        params: ptr::null_mut(),
    };

    let mut fn_params = FnQawsParams {
        function: f,
        a,
        b,
        table: t,
    };

    weighted_function.params = &mut fn_params as *mut _ as *mut std::ffi::c_void;

    if a1 == a && (t.alpha != 0.0 || t.mu != 0) {
        let mut cheb12 = [0.0; 13];
        let mut cheb24 = [0.0; 25];
        let factor = (0.5 * (b1 - a1)).powf(t.alpha + 1.0);
        weighted_function.function = Some(fn_qaws_r);

        unsafe {
            gsl_integration_qcheb(
                &mut weighted_function,
                a1,
                b1,
                cheb12.as_mut_ptr(),
                cheb24.as_mut_ptr(),
            );
        }

        if t.mu == 0 {
            let mut res12 = 0.0;
            let mut res24 = 0.0;
            let u = factor;
            compute_result(&t.ri, &cheb12, &cheb24, &mut res12, &mut res24);
            *result = u * res24;
            *abserr = (u * (res24 - res12)).abs();
        } else {
            let mut res12a = 0.0;
            let mut res24a = 0.0;
            let mut res12b = 0.0;
            let mut res24b = 0.0;
            let u = factor * (b1 - a1).ln();
            let v = factor;
            compute_result(&t.ri, &cheb12, &cheb24, &mut res12a, &mut res24a);
            compute_result(&t.rg, &cheb12, &cheb24, &mut res12b, &mut res24b);
            *result = u * res24a + v * res24b;
            *abserr = (u * (res24a - res12a)).abs() + (v * (res24b - res12b)).abs();
        }
        *err_reliable = false;
        return;
    } else if b1 == b && (t.beta != 0.0 || t.nu != 0) {
        let mut cheb12 = [0.0; 13];
        let mut cheb24 = [0.0; 25];
        let factor = (0.5 * (b1 - a1)).powf(t.beta + 1.0);
        weighted_function.function = Some(fn_qaws_l);

        unsafe {
            gsl_integration_qcheb(
                &mut weighted_function,
                a1,
                b1,
                cheb12.as_mut_ptr(),
                cheb24.as_mut_ptr(),
            );
        }

        if t.nu == 0 {
            let mut res12 = 0.0;
            let mut res24 = 0.0;
            let u = factor;
            compute_result(&t.rj, &cheb12, &cheb24, &mut res12, &mut res24);
            *result = u * res24;
            *abserr = (u * (res24 - res12)).abs();
        } else {
            let mut res12a = 0.0;
            let mut res24a = 0.0;
            let mut res12b = 0.0;
            let mut res24b = 0.0;
            let u = factor * (b1 - a1).ln();
            let v = factor;
            compute_result(&t.rj, &cheb12, &cheb24, &mut res12a, &mut res24a);
            compute_result(&t.rh, &cheb12, &cheb24, &mut res12b, &mut res24b);
            *result = u * res24a + v * res24b;
            *abserr = (u * (res24a - res12a)).abs() + (v * (res24b - res12b)).abs();
        }
        *err_reliable = false;
        return;
    } else {
        let mut resabs = 0.0;
        let mut resasc = 0.0;
        weighted_function.function = Some(fn_qaws);

        unsafe {
            gsl_integration_qk15(
                &weighted_function,
                a1,
                b1,
                result,
                abserr,
                &mut resabs,
                &mut resasc,
            );
        }

        *err_reliable = *abserr != resasc;
        return;
    }
}

fn fn_qaws(x: f64, params: *mut std::ffi::c_void) -> f64 {
    let p = unsafe { &mut *(params as *mut FnQawsParams) };
    let f = unsafe { &mut *p.function };
    let t = unsafe { &mut *p.table };

    let mut factor = 1.0;
    if t.alpha != 0.0 {
        factor *= (x - p.a).powf(t.alpha);
    }
    if t.beta != 0.0 {
        factor *= (p.b - x).powf(t.beta);
    }
    if t.mu == 1 {
        factor *= (x - p.a).ln();
    }
    if t.nu == 1 {
        factor *= (p.b - x).ln();
    }

    if let Some(func) = f.function {
        factor * func(x, f.params)
    } else {
        0.0
    }
}

fn fn_qaws_l(x: f64, params: *mut std::ffi::c_void) -> f64 {
    let p = unsafe { &mut *(params as *mut FnQawsParams) };
    let f = unsafe { &mut *p.function };
    let t = unsafe { &mut *p.table };

    let mut factor = 1.0;
    if t.alpha != 0.0 {
        factor *= (x - p.a).powf(t.alpha);
    }
    if t.mu == 1 {
        factor *= (x - p.a).ln();
    }

    if let Some(func) = f.function {
        factor * func(x, f.params)
    } else {
        0.0
    }
}

fn fn_qaws_r(x: f64, params: *mut std::ffi::c_void) -> f64 {
    let p = unsafe { &mut *(params as *mut FnQawsParams) };
    let f = unsafe { &mut *p.function };
    let t = unsafe { &mut *p.table };

    let mut factor = 1.0;
    if t.beta != 0.0 {
        factor *= (p.b - x).powf(t.beta);
    }
    if t.nu == 1 {
        factor *= (p.b - x).ln();
    }

    if let Some(func) = f.function {
        factor * func(x, f.params)
    } else {
        0.0
    }
}

fn compute_result(
    r: &[f64],
    cheb12: &[f64],
    cheb24: &[f64],
    result12: &mut f64,
    result24: &mut f64,
) {
    *result12 = r.iter().zip(cheb12.iter()).map(|(r, c)| r * c).sum();
    *result24 = r.iter().zip(cheb24.iter()).map(|(r, c)| r * c).sum();
}

pub fn gsl_integration_qaws(
    f: &mut GslFunction,
    a: f64,
    b: f64,
    t: &mut GslIntegrationQawsTable,
    epsabs: f64,
    epsrel: f64,
    limit: usize,
    workspace: &mut GslIntegrationWorkspace,
    result: &mut f64,
    abserr: &mut f64,
) -> GslError {
    if limit > workspace.limit {
        return GslError::Invalid;
    }

    if b <= a {
        return GslError::Invalid;
    }

    if epsabs <= 0.0 && (epsrel < 50.0 * f64::EPSILON || epsrel < 0.5e-28) {
        return GslError::BadTol;
    }

    initialise(workspace, a, b);
    *result = 0.0;
    *abserr = 0.0;

    let mut area1 = 0.0;
    let mut error1 = 0.0;
    let mut err_reliable1 = false;
   