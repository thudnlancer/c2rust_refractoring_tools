use std::f64;
use libc::{c_double, c_int, c_void};

pub type size_t = usize;

#[derive(Debug, Copy, Clone)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

#[derive(Clone)]
pub struct GslFunction {
    pub function: Option<fn(f64, &mut c_void) -> f64>,
    pub params: *mut c_void,
}

#[derive(Clone)]
pub struct GslIntegrationWorkspace {
    pub limit: size_t,
    pub size: size_t,
    pub nrmax: size_t,
    pub i: size_t,
    pub maximum_level: size_t,
    pub alist: Vec<f64>,
    pub blist: Vec<f64>,
    pub rlist: Vec<f64>,
    pub elist: Vec<f64>,
    pub order: Vec<size_t>,
    pub level: Vec<size_t>,
}

#[derive(Clone)]
pub struct ExtrapolationTable {
    pub n: size_t,
    pub rlist2: [f64; 52],
    pub nres: size_t,
    pub res3la: [f64; 3],
}

#[derive(Clone)]
pub struct IuParams {
    pub a: f64,
    pub f: Box<GslFunction>,
}

#[derive(Clone)]
pub struct IlParams {
    pub b: f64,
    pub f: Box<GslFunction>,
}

pub type GslIntegrationRule = fn(
    &GslFunction,
    f64,
    f64,
    &mut f64,
    &mut f64,
    &mut f64,
    &mut f64,
) -> ();

fn gsl_max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn initialise(workspace: &mut GslIntegrationWorkspace, a: f64, b: f64) {
    workspace.size = 0;
    workspace.nrmax = 0;
    workspace.i = 0;
    workspace.alist[0] = a;
    workspace.blist[0] = b;
    workspace.rlist[0] = 0.0;
    workspace.elist[0] = 0.0;
    workspace.order[0] = 0;
    workspace.level[0] = 0;
    workspace.maximum_level = 0;
}

fn set_initial_result(workspace: &mut GslIntegrationWorkspace, result: f64, error: f64) {
    workspace.size = 1;
    workspace.rlist[0] = result;
    workspace.elist[0] = error;
}

fn qpsrt(workspace: &mut GslIntegrationWorkspace) {
    let last = workspace.size - 1;
    let limit = workspace.limit;
    let elist = &mut workspace.elist;
    let order = &mut workspace.order;
    let mut i_nrmax = workspace.nrmax;
    let mut i_maxerr = order[i_nrmax];

    if last < 2 {
        order[0] = 0;
        order[1] = 1;
        workspace.i = i_maxerr;
        return;
    }

    let mut errmax = elist[i_maxerr];
    while i_nrmax > 0 && errmax > elist[order[i_nrmax - 1]] {
        order[i_nrmax] = order[i_nrmax - 1];
        i_nrmax -= 1;
    }

    let top = if last < limit / 2 + 2 {
        last as i32
    } else {
        (limit - last + 1) as i32
    };

    let mut i = i_nrmax as i32 + 1;
    while i < top && errmax < elist[order[i as usize]] {
        order[(i - 1) as usize] = order[i as usize];
        i += 1;
    }
    order[(i - 1) as usize] = i_maxerr;

    let errmin = elist[last];
    let mut k = top - 1;
    while k > i - 2 && errmin >= elist[order[k as usize]] {
        order[(k + 1) as usize] = order[k as usize];
        k -= 1;
    }
    order[(k + 1) as usize] = last;

    i_maxerr = order[i_nrmax];
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
    let new_level = workspace.level[i_max] + 1;

    if error2 > error1 {
        workspace.alist[i_max] = a2;
        workspace.rlist[i_max] = area2;
        workspace.elist[i_max] = error2;
        workspace.level[i_max] = new_level;
        workspace.alist[i_new] = a1;
        workspace.blist[i_new] = b1;
        workspace.rlist[i_new] = area1;
        workspace.elist[i_new] = error1;
        workspace.level[i_new] = new_level;
    } else {
        workspace.blist[i_max] = b1;
        workspace.rlist[i_max] = area1;
        workspace.elist[i_max] = error1;
        workspace.level[i_max] = new_level;
        workspace.alist[i_new] = a2;
        workspace.blist[i_new] = b2;
        workspace.rlist[i_new] = area2;
        workspace.elist[i_new] = error2;
        workspace.level[i_new] = new_level;
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
    *a = workspace.alist[i];
    *b = workspace.blist[i];
    *r = workspace.rlist[i];
    *e = workspace.elist[i];
}

fn sum_results(workspace: &GslIntegrationWorkspace) -> f64 {
    workspace.rlist[..workspace.size].iter().sum()
}

fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let e = 2.2204460492503131e-16;
    let u = 2.2250738585072014e-308;
    let tmp = (1.0 + 100.0 * e) * (a2.abs() + 1000.0 * u);
    a1.abs() <= tmp && b2.abs() <= tmp
}

fn reset_nrmax(workspace: &mut GslIntegrationWorkspace) {
    workspace.nrmax = 0;
    workspace.i = workspace.order[0];
}

fn increase_nrmax(workspace: &mut GslIntegrationWorkspace) -> bool {
    let mut id = workspace.nrmax as i32;
    let level = &workspace.level;
    let order = &workspace.order;
    let limit = workspace.limit;
    let last = workspace.size - 1;

    let jupbnd = if last > 1 + limit / 2 {
        (limit + 1 - last) as i32
    } else {
        last as i32
    };

    let mut k = id;
    while k <= jupbnd {
        let i_max = order[workspace.nrmax];
        workspace.i = i_max;
        if level[i_max] < workspace.maximum_level {
            return true;
        }
        workspace.nrmax += 1;
        k += 1;
    }
    false
}

fn large_interval(workspace: &GslIntegrationWorkspace) -> bool {
    let i = workspace.i;
    workspace.level[i] < workspace.maximum_level
}

fn initialise_table(table: &mut ExtrapolationTable) {
    table.n = 0;
    table.nres = 0;
}

fn append_table(table: &mut ExtrapolationTable, y: f64) {
    let n = table.n;
    table.rlist2[n] = y;
    table.n += 1;
}

fn qelg(table: &mut ExtrapolationTable, result: &mut f64, abserr: &mut f64) {
    let epstab = &mut table.rlist2;
    let res3la = &mut table.res3la;
    let n = table.n - 1;
    let current = epstab[n];
    let mut absolute = f64::MAX;
    let relative = 5.0 * 2.2204460492503131e-16 * current.abs();

    let newelm = n / 2;
    let n_orig = n;
    let mut n_final = n;
    let nres_orig = table.nres;

    *result = current;
    *abserr = f64::MAX;

    if n < 2 {
        *result = current;
        *abserr = gsl_max_dbl(absolute, relative);
        return;
    }

    epstab[n + 2] = epstab[n];
    epstab[n] = f64::MAX;

    for i in 0..newelm {
        let res = epstab[n - 2 * i + 2];
        let e0 = epstab[n - 2 * i - 2];
        let e1 = epstab[n - 2 * i - 1];
        let e2 = res;
        let e1abs = e1.abs();
        let delta2 = e2 - e1;
        let err2 = delta2.abs();
        let tol2 = gsl_max_dbl(e2.abs(), e1abs) * 2.2204460492503131e-16;
        let delta3 = e1 - e0;
        let err3 = delta3.abs();
        let tol3 = gsl_max_dbl(e1abs, e0.abs()) * 2.2204460492503131e-16;

        if err2 <= tol2 && err3 <= tol3 {
            *result = res;
            absolute = err2 + err3;
            *abserr = gsl_max_dbl(absolute, relative);
            return;
        }

        let e3 = epstab[n - 2 * i];
        epstab[n - 2 * i] = e1;
        let delta1 = e1 - e3;
        let err1 = delta1.abs();
        let tol1 = gsl_max_dbl(e1abs, e3.abs()) * 2.2204460492503131e-16;

        if err1 <= tol1 || err2 <= tol2 || err3 <= tol3 {
            n_final = 2 * i;
            break;
        } else {
            let ss = 1.0 / delta1 + 1.0 / delta2 - 1.0 / delta3;
            if (ss * e1).abs() <= 0.0001 {
                n_final = 2 * i;
                break;
            } else {
                let res = e1 + 1.0 / ss;
                epstab[n - 2 * i] = res;
                let error = err2 + (res - e2).abs() + err3;
                if error <= *abserr {
                    *abserr = error;
                    *result = res;
                }
            }
        }
    }

    let limexp = 50 - 1;
    if n_final == limexp {
        n_final = 2 * (limexp / 2);
    }

    if n_orig % 2 == 1 {
        for i in 0..=newelm {
            epstab[1 + i * 2] = epstab[i * 2 + 3];
        }
    } else {
        for i in 0..=newelm {
            epstab[i * 2] = epstab[i * 2 + 2];
        }
    }

    if n_orig != n_final {
        for i in 0..=n_final {
            epstab[i] = epstab[n_orig - n_final + i];
        }
    }

    table.n = n_final + 1;
    if nres_orig < 3 {
        res3la[nres_orig] = *result;
        *abserr = f64::MAX;
    } else {
        *abserr = (*result - res3la[2]).abs()
            + (*result - res3la[1]).abs()
            + (*result - res3la[0]).abs();
        res3la[0] = res3la[1];
        res3la[1] = res3la[2];
        res3la[2] = *result;
    }

    table.nres = nres_orig + 1;
    *abserr = gsl_max_dbl(
        *abserr,
        5.0 * 2.2204460492503131e-16 * result.abs(),
    );
}

fn test_positivity(result: f64, resabs: f64) -> bool {
    result.abs() >= (1.0 - 50.0 * 2.2204460492503131e-16) * resabs
}

pub fn gsl_integration_qags(
    f: &GslFunction,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    limit: size_t,
    workspace: &mut GslIntegrationWorkspace,
    result: &mut f64,
    abserr: &mut f64,
) -> GslError {
    qags(
        f,
        a,
        b,
        epsabs,
        epsrel,
        limit,
        workspace,
        result,
        abserr,
        gsl_integration_qk21,
    )
}

pub fn gsl_integration_qagi(
    f: &mut GslFunction,
    epsabs: f64,
    epsrel: f64,
    limit: size_t,
    workspace: &mut GslIntegrationWorkspace,
    result: &mut f64,
    abserr: &mut f64,
) -> GslError {
    let mut f_transform = GslFunction {
        function: Some(i_transform),
        params: f as *mut _ as *mut c_void,
    };

    qags(
        &f_transform,
        0.0,
        1.0,
        epsabs,
        epsrel,
        limit,
        workspace,
        result,
        abserr,
        gsl_integration_qk15,
    )
}

fn i_transform(t: f64, params: &mut c_void) -> f64 {
    let f = unsafe { &mut *(params as *mut GslFunction) };
    let x = (1.0 - t) / t;
    let y = (f.function.unwrap())(x, f.params) + (f.function.unwrap())(-x, f.params);
    y / t / t
}

pub fn gsl_integration_qagil(
    f: &mut GslFunction,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    limit: size_t,
    workspace: &mut GslIntegrationWorkspace,
    result: &mut f64,
    abserr: &mut f64,
) -> GslError {
    let mut transform_params = IlParams {
        b,
        f: Box::new(f.clone()),
    };

    let mut f_transform = GslFunction {
        function: Some(il_transform),
        params: &mut transform_params as *mut _ as *mut c_void,
    };

    qags(
        &f_transform,
        0.0,
        1.0,
        epsabs,
        epsrel,
        limit,
        workspace,
        result,
        abserr,
        gsl_integration_qk15,
    )
}

fn il_transform(t: f64, params: &mut c_void) -> f64 {
    let p = unsafe { &mut *(params as *mut IlParams) };
    let x = p.b - (1.0 - t) / t;
    let y = (p.f.function.unwrap())(x, p.f.params);
    y / t / t
}

pub fn gsl_integration_qagiu(
    f: &mut GslFunction,
    a: f64,
    epsabs: f64,
    epsrel: f64,
    limit: size_t,
    workspace: &mut GslIntegrationWorkspace,
    result: &mut f64,
    abserr: &mut f64,
) -> GslError {
    let mut transform_params = IuParams {
        a,
        f: Box::new(f.clone()),
    };

    let mut f_transform = GslFunction {
        function: Some(iu_transform),
        params: &mut transform_params as *mut _ as *mut c_void,
   