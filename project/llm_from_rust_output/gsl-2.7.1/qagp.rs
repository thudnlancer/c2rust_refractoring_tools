use std::f64;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Edom = 1,
    Erange = 2,
    Edefault = 3,
    Einval = 4,
    Efailed = 5,
    Efactor = 6,
    Esanity = 7,
    Enomem = 8,
    Ebadfunc = 9,
    Erunaway = 10,
    Emaxiter = 11,
    Ezerodiv = 12,
    Ebadtol = 13,
    Etol = 14,
    Eundrflw = 15,
    Eovrflw = 16,
    Eloss = 17,
    Eround = 18,
    Ebadlen = 19,
    Enotsqr = 20,
    Esing = 21,
    Ediverge = 22,
    Eunsup = 23,
    Eunimpl = 24,
    Ecache = 25,
    Etable = 26,
    Enoprog = 27,
    Enoprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone, Copy)]
pub struct GslFunction {
    pub function: Option<fn(f64, &mut [u8]) -> f64>,
    pub params: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct GslIntegrationWorkspace {
    pub limit: usize,
    pub size: usize,
    pub nrmax: usize,
    pub i: usize,
    pub maximum_level: usize,
    pub alist: Vec<f64>,
    pub blist: Vec<f64>,
    pub rlist: Vec<f64>,
    pub elist: Vec<f64>,
    pub order: Vec<usize>,
    pub level: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct ExtrapolationTable {
    pub n: usize,
    pub rlist2: [f64; 52],
    pub nres: usize,
    pub res3la: [f64; 3],
}

impl GslIntegrationWorkspace {
    pub fn initialise(&mut self, a: f64, b: f64) {
        self.size = 0;
        self.nrmax = 0;
        self.i = 0;
        self.alist[0] = a;
        self.blist[0] = b;
        self.rlist[0] = 0.0;
        self.elist[0] = 0.0;
        self.order[0] = 0;
        self.level[0] = 0;
        self.maximum_level = 0;
    }

    pub fn qpsrt(&mut self) {
        let last = self.size - 1;
        let limit = self.limit;
        let elist = &mut self.elist;
        let order = &mut self.order;

        if last < 2 {
            order[0] = 0;
            order[1] = 1;
            self.i = self.order[self.nrmax];
            return;
        }

        let mut i_nrmax = self.nrmax;
        let mut i_maxerr = order[i_nrmax];
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
        while i < top && errmax < elist[order[i as usize] as usize] {
            order[(i - 1) as usize] = order[i as usize];
            i += 1;
        }

        order[(i - 1) as usize] = i_maxerr;
        let errmin = elist[last];

        let mut k = top - 1;
        while k > i - 2 && errmin >= elist[order[k as usize] as usize] {
            order[(k + 1) as usize] = order[k as usize];
            k -= 1;
        }

        order[(k + 1) as usize] = last;
        self.i = order[i_nrmax];
        self.nrmax = i_nrmax;
    }

    pub fn update(
        &mut self,
        a1: f64,
        b1: f64,
        area1: f64,
        error1: f64,
        a2: f64,
        b2: f64,
        area2: f64,
        error2: f64,
    ) {
        let i_max = self.i;
        let i_new = self.size;
        let new_level = self.level[i_max] + 1;

        if error2 > error1 {
            self.alist[i_max] = a2;
            self.rlist[i_max] = area2;
            self.elist[i_max] = error2;
            self.level[i_max] = new_level;

            self.alist[i_new] = a1;
            self.blist[i_new] = b1;
            self.rlist[i_new] = area1;
            self.elist[i_new] = error1;
            self.level[i_new] = new_level;
        } else {
            self.blist[i_max] = b1;
            self.rlist[i_max] = area1;
            self.elist[i_max] = error1;
            self.level[i_max] = new_level;

            self.alist[i_new] = a2;
            self.blist[i_new] = b2;
            self.rlist[i_new] = area2;
            self.elist[i_new] = error2;
            self.level[i_new] = new_level;
        }

        self.size += 1;
        if new_level > self.maximum_level {
            self.maximum_level = new_level;
        }

        self.qpsrt();
    }

    pub fn retrieve(&self, a: &mut f64, b: &mut f64, r: &mut f64, e: &mut f64) {
        let i = self.i;
        *a = self.alist[i];
        *b = self.blist[i];
        *r = self.rlist[i];
        *e = self.elist[i];
    }

    pub fn sum_results(&self) -> f64 {
        self.rlist[..self.size].iter().sum()
    }

    pub fn append_interval(&mut self, a1: f64, b1: f64, area1: f64, error1: f64) {
        let i_new = self.size;
        self.alist[i_new] = a1;
        self.blist[i_new] = b1;
        self.rlist[i_new] = area1;
        self.elist[i_new] = error1;
        self.order[i_new] = i_new;
        self.level[i_new] = 0;
        self.size += 1;
    }

    pub fn reset_nrmax(&mut self) {
        self.nrmax = 0;
        self.i = self.order[0];
    }

    pub fn increase_nrmax(&mut self) -> bool {
        let mut id = self.nrmax as i32;
        let mut jupbnd = if self.size - 1 > 1 + self.limit / 2 {
            (self.limit + 1 - (self.size - 1)) as i32
        } else {
            (self.size - 1) as i32
        };

        let mut k = id;
        while k <= jupbnd {
            let i_max = self.order[self.nrmax];
            self.i = i_max;
            if self.level[i_max] < self.maximum_level {
                return true;
            }
            self.nrmax += 1;
            k += 1;
        }
        false
    }

    pub fn large_interval(&self) -> bool {
        let i = self.i;
        self.level[i] < self.maximum_level
    }

    pub fn sort_results(&mut self) {
        let nint = self.size;
        for i in 0..nint {
            let mut i1 = self.order[i];
            let mut e1 = self.elist[i1];
            let mut i_max = i1;

            for j in i + 1..nint {
                let i2 = self.order[j];
                let e2 = self.elist[i2];
                if e2 >= e1 {
                    i_max = i2;
                    e1 = e2;
                }
            }

            if i_max != i1 {
                self.order[i] = self.order[i_max];
                self.order[i_max] = i1;
            }
        }
        self.i = self.order[0];
    }
}

impl ExtrapolationTable {
    pub fn new() -> Self {
        ExtrapolationTable {
            n: 0,
            rlist2: [0.0; 52],
            nres: 0,
            res3la: [0.0; 3],
        }
    }

    pub fn append_table(&mut self, y: f64) {
        self.rlist2[self.n] = y;
        self.n += 1;
    }

    pub fn qelg(&mut self, result: &mut f64, abserr: &mut f64) {
        let n = self.n - 1;
        let current = self.rlist2[n];
        let mut absolute = f64::MAX;
        let relative = 5.0 * f64::EPSILON * current.abs();

        if n < 2 {
            *result = current;
            *abserr = absolute.max(relative);
            return;
        }

        self.rlist2[n + 2] = self.rlist2[n];
        self.rlist2[n] = f64::MAX;

        let newelm = n / 2;
        let mut n_final = n;
        let n_orig = n;
        let nres_orig = self.nres;

        *result = current;
        *abserr = f64::MAX;

        let mut i = 0;
        while i < newelm {
            let res = self.rlist2[n - 2 * i + 2];
            let e0 = self.rlist2[n - 2 * i - 2];
            let e1 = self.rlist2[n - 2 * i - 1];
            let e2 = res;

            let e1abs = e1.abs();
            let delta2 = e2 - e1;
            let err2 = delta2.abs();
            let tol2 = e2.abs().max(e1abs) * f64::EPSILON;

            let delta3 = e1 - e0;
            let err3 = delta3.abs();
            let tol3 = e1abs.max(e0.abs()) * f64::EPSILON;

            if err2 <= tol2 && err3 <= tol3 {
                *result = res;
                absolute = err2 + err3;
                *abserr = absolute.max(5.0 * f64::EPSILON * res.abs());
                return;
            }

            let e3 = self.rlist2[n - 2 * i];
            self.rlist2[n - 2 * i] = e1;

            let delta1 = e1 - e3;
            let err1 = delta1.abs();
            let tol1 = e1abs.max(e3.abs()) * f64::EPSILON;

            if err1 <= tol1 || err2 <= tol2 || err3 <= tol3 {
                n_final = 2 * i;
                break;
            }

            let ss = 1.0 / delta1 + 1.0 / delta2 - 1.0 / delta3;
            if (ss * e1).abs() <= 1.0e-4 {
                n_final = 2 * i;
                break;
            } else {
                let res = e1 + 1.0 / ss;
                self.rlist2[n - 2 * i] = res;
                let error = err2 + (res - e2).abs() + err3;
                if error <= *abserr {
                    *abserr = error;
                    *result = res;
                }
                i += 1;
            }
        }

        if n_final == 49 {
            n_final = 2 * (49 / 2);
        }

        if n_orig % 2 == 1 {
            for i in 0..=newelm {
                self.rlist2[1 + 2 * i] = self.rlist2[2 * i + 3];
            }
        } else {
            for i in 0..=newelm {
                self.rlist2[2 * i] = self.rlist2[2 * i + 2];
            }
        }

        if n_orig != n_final {
            for i in 0..=n_final {
                self.rlist2[i] = self.rlist2[n_orig - n_final + i];
            }
        }

        self.n = n_final + 1;
        if nres_orig < 3 {
            self.res3la[nres_orig] = *result;
            *abserr = f64::MAX;
        } else {
            *abserr = (*result - self.res3la[2]).abs()
                + (*result - self.res3la[1]).abs()
                + (*result - self.res3la[0]).abs();
            self.res3la[0] = self.res3la[1];
            self.res3la[1] = self.res3la[2];
            self.res3la[2] = *result;
        }

        self.nres = nres_orig + 1;
        *abserr = (*abserr).max(5.0 * f64::EPSILON * result.abs());
    }
}

pub fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let e = f64::EPSILON;
    let u = f64::MIN_POSITIVE;
    let tmp = (1.0 + 100.0 * e) * (a2.abs() + 1000.0 * u);
    a1.abs() <= tmp && b2.abs() <= tmp
}

pub fn test_positivity(result: f64, resabs: f64) -> bool {
    result.abs() >= (1.0 - 50.0 * f64::EPSILON) * resabs
}

pub fn gsl_integration_qagp(
    f: &GslFunction,
    pts: &[f64],
    epsabs: f64,
    epsrel: f64,
    limit: usize,
    workspace: &mut GslIntegrationWorkspace,
) -> Result<(f64, f64), GslError> {
    qagp(f, pts, epsabs, epsrel, limit, workspace, gsl_integration_qk21)
}

fn gsl_integration_qk21(
    f: &GslFunction,
    a: f64,
    b: f64,
    result: &mut f64,
    abserr: &mut f64,
    resabs: &mut f64,
    resasc: &mut f64,
) {
    // Implementation of QK21 quadrature rule
    // This is a placeholder - actual implementation would need to be provided
    unimplemented!()
}

fn qagp(
    f: &GslFunction,
    pts: &[f64],
    epsabs: f64,
    epsrel: f64,
    limit: usize,
    workspace: &mut GslIntegrationWorkspace,
    q: fn(&GslFunction, f64, f64, &mut f64, &mut f64, &mut f64, &mut f64),
) -> Result<(f64, f64), GslError> {
    let npts = pts.len();
    if npts == 0 {
        return Err(GslError::Einval);
    }

    if limit > workspace.limit {
        return Err(GslError::Einval);
    }

    if npts > workspace.limit {
        return Err(GslError::Einval);
    }

    if epsabs <= 0.0 && (epsrel < 50.0 * f64::EPSILON || epsrel < 0.5e-28) {
        return Err(GslError::Ebadtol);
    }

    for i in 0..npts - 1 {
        if pts[i + 1] < pts[i] {
            return Err(GslError::Einval);
        }
    }

    let nint = npts - 1;
    let mut ndin = vec![0; nint];
    let mut result0 = 0.0;
    let mut abserr0 = 0.0;
    let mut resabs0 = 0.0;

    workspace.initialise(0.0, 0.0);

    for i in 0..nint {
        let a1 = pts[i];
        let b1 = pts[i + 1];
        let mut area1 = 0.0;
        let mut error1 = 0.0;
        let mut resabs1 = 0.0;
        let mut resasc1 = 0.0;

        q(f, a1, b1, &mut area1, &mut error1, &mut resabs1, &mut resasc1);

        result0 += area1;
        abserr0 += error1;
        resabs0 += resabs1;

        workspace.append_interval(a1, b1, area1, error1);

        ndin[i] = if error1 == resasc1 && error1 != 0.0 {
            1
        } else {
            0
        };
    }

    let mut errsum = 0.0;
    for i in 0..nint {
        if ndin[i] != 0 {
            workspace.elist[i] = abserr0;
        }
        errsum += workspace.elist[i];
    }

    for i in 0..nint {
        workspace.level[i] = 0;
    }

    workspace.sort_results();
    let tolerance = epsabs.max(epsrel * result0.abs());

    if abserr0 <= 100.0 * f64::EPS