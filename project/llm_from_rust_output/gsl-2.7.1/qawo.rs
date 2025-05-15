use std::f64::consts::PI;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub enum IntegrationQawoEnum {
    Cosine = 0,
    Sine = 1,
}

#[derive(Debug, Clone)]
pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
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
pub struct GslIntegrationQawoTable {
    pub n: usize,
    pub omega: f64,
    pub l: f64,
    pub par: f64,
    pub sine: IntegrationQawoEnum,
    pub chebmo: Vec<f64>,
}

#[derive(Debug, Clone)]
struct ExtrapolationTable {
    n: usize,
    rlist2: [f64; 52],
    nres: usize,
    res3la: [f64; 3],
}

#[derive(Debug, Clone)]
struct FourierParams {
    function: GslFunction,
    omega: f64,
}

impl GslIntegrationWorkspace {
    fn initialise(&mut self, a: f64, b: f64) {
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

    fn set_initial_result(&mut self, result: f64, error: f64) {
        self.size = 1;
        self.rlist[0] = result;
        self.elist[0] = error;
    }

    fn reset_nrmax(&mut self) {
        self.nrmax = 0;
        self.i = self.order[0];
    }

    fn qpsrt(&mut self) {
        let last = self.size - 1;
        let limit = self.limit;
        let elist = &mut self.elist;
        let order = &mut self.order;

        if last < 2 {
            order[0] = 0;
            order[1] = 1;
            self.i = order[self.nrmax];
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
        self.i = order[i_nrmax];
        self.nrmax = i_nrmax;
    }

    fn update(
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

    fn retrieve(&self, a: &mut f64, b: &mut f64, r: &mut f64, e: &mut f64) {
        let i = self.i;
        *a = self.alist[i];
        *b = self.blist[i];
        *r = self.rlist[i];
        *e = self.elist[i];
    }

    fn sum_results(&self) -> f64 {
        self.rlist[..self.size].iter().sum()
    }
}

impl ExtrapolationTable {
    fn new() -> Self {
        Self {
            n: 0,
            rlist2: [0.0; 52],
            nres: 0,
            res3la: [0.0; 3],
        }
    }

    fn append(&mut self, y: f64) {
        self.rlist2[self.n] = y;
        self.n += 1;
    }

    fn qelg(&mut self, result: &mut f64, abserr: &mut f64) {
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
        let n_orig = n;
        let mut n_final = n;

        for i in 0..newelm {
            let k = n - 2 * i;
            let res = self.rlist2[k + 2];
            let e0 = self.rlist2[k - 2];
            let e1 = self.rlist2[k - 1];
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

            let e3 = self.rlist2[k];
            self.rlist2[k] = e1;

            let delta1 = e1 - e3;
            let err1 = delta1.abs();
            let tol1 = e1abs.max(e3.abs()) * f64::EPSILON;

            if err1 <= tol1 || err2 <= tol2 || err3 <= tol3 {
                n_final = 2 * i;
                break;
            }

            let ss = 1.0 / delta1 + 1.0 / delta2 - 1.0 / delta3;
            if (ss * e1).abs() <= 1e-4 {
                n_final = 2 * i;
                break;
            }

            let res = e1 + 1.0 / ss;
            self.rlist2[k] = res;
            let error = err2 + (res - e2).abs() + err3;
            if error <= *abserr {
                *abserr = error;
                *result = res;
            }
        }

        let limexp = 50 - 1;
        if n_final == limexp {
            n_final = 2 * (limexp / 2);
        }

        if n_orig % 2 == 1 {
            for i in 0..=newelm {
                self.rlist2[1 + 2 * i] = self.rlist2[3 + 2 * i];
            }
        } else {
            for i in 0..=newelm {
                self.rlist2[2 * i] = self.rlist2[2 + 2 * i];
            }
        }

        if n_orig != n_final {
            for i in 0..=n_final {
                self.rlist2[i] = self.rlist2[n_orig - n_final + i];
            }
        }

        self.n = n_final + 1;
        let nres_orig = self.nres;

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

fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let e = f64::EPSILON;
    let u = f64::MIN_POSITIVE;
    let tmp = (1.0 + 100.0 * e) * (a2.abs() + 1000.0 * u);
    a1.abs() <= tmp && b2.abs() <= tmp
}

fn test_positivity(result: f64, resabs: f64) -> bool {
    result.abs() >= (1.0 - 50.0 * f64::EPSILON) * resabs
}

fn qc25f(
    f: &GslFunction,
    a: f64,
    b: f64,
    wf: &mut GslIntegrationQawoTable,
    level: usize,
    result: &mut f64,
    abserr: &mut f64,
    resabs: &mut f64,
    resasc: &mut f64,
) {
    let center = 0.5 * (a + b);
    let half_length = 0.5 * (b - a);
    let omega = wf.omega;
    let par = omega * half_length;

    if par.abs() < 2.0 {
        let weighted_function = GslFunction {
            function: match wf.sine {
                IntegrationQawoEnum::Sine => Box::new(|x| {
                    let wx = omega * x;
                    let sinwx = wx.sin();
                    (f.function)(x) * sinwx
                }),
                IntegrationQawoEnum::Cosine => Box::new(|x| {
                    let wx = omega * x;
                    let coswx = wx.cos();
                    (f.function)(x) * coswx
                }),
            },
        };

        // TODO: Implement qk15 integration
        // gsl_integration_qk15(
        //     &weighted_function,
        //     a,
        //     b,
        //     result,
        //     abserr,
        //     resabs,
        //     resasc,
        // );
        unimplemented!("Need to implement qk15 integration");
    } else {
        if level >= wf.n {
            // TODO: Handle error
            // gsl_error(...);
            return;
        }

        let mut cheb12 = [0.0; 13];
        let mut cheb24 = [0.0; 25];

        // TODO: Implement qcheb integration
        // gsl_integration_qcheb(f, a, b, &mut cheb12, &mut cheb24);
        unimplemented!("Need to implement qcheb integration");

        let moment_start = 25 * level;
        let moment = &wf.chebmo[moment_start..moment_start + 25];

        let mut res12_cos = cheb12[12] * moment[12];
        let mut res12_sin = 0.0;

        for i in 0..6 {
            let k = 10 - 2 * i;
            res12_cos += cheb12[k] * moment[k];
            res12_sin += cheb12[k + 1] * moment[k + 1];
        }

        let mut res24_cos = cheb24[24] * moment[24];
        let mut res24_sin = 0.0;
        let mut result_abs = cheb24[24].abs();

        for i in 0..12 {
            let k = 22 - 2 * i;
            res24_cos += cheb24[k] * moment[k];
            res24_sin += cheb24[k + 1] * moment[k + 1];
            result_abs += cheb24[k].abs() + cheb24[k + 1].abs();
        }

        let est_cos = (res24_cos - res12_cos).abs();
        let est_sin = (res24_sin - res12_sin).abs();

        let c = half_length * (center * omega).cos();
        let s = half_length * (center * omega).sin();

        match wf.sine {
            IntegrationQawoEnum::Sine => {
                *result = c * res24_sin + s * res24_cos;
                *abserr = (c * est_sin).abs() + (s * est_cos).abs();
            }
            IntegrationQawoEnum::Cosine => {
                *result = c * res24_cos - s * res24_sin;
                *abserr = (c * est_cos).abs() + (s * est_sin).abs();
            }
        }

        *resabs = result_abs * half_length;
        *resasc = f64::MAX;
    }
}

pub fn gsl_integration_qawo(
    f: &GslFunction,
    a: f64,
    epsabs: f64,
    epsrel: f64,
    limit: usize,
    workspace: &mut GslIntegrationWorkspace,
    wf: &mut GslIntegrationQawoTable,
    result: &mut f64,
    abserr: &mut f64,
) -> i32 {
    // TODO: Implement the full integration logic
    unimplemented!("Full integration logic needs to be implemented");
}