use std::f64::consts::PI;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslIntegrationQawoEnum {
    Cosine = 0,
    Sine = 1,
}

#[derive(Debug)]
pub struct GslIntegrationQawoTable {
    n: usize,
    omega: f64,
    l: f64,
    par: f64,
    sine: GslIntegrationQawoEnum,
    chebmo: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
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
    Ebadtole = 13,
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
    Continue = -2,
}

impl GslIntegrationQawoTable {
    pub fn new(
        omega: f64,
        l: f64,
        sine: GslIntegrationQawoEnum,
        n: usize,
    ) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::Edom);
        }

        let chebmo_size = 25 * n;
        let mut chebmo = vec![0.0; chebmo_size];

        let mut table = GslIntegrationQawoTable {
            n,
            omega,
            l,
            par: 0.5 * omega * l,
            sine,
            chebmo,
        };

        table.compute_moments();

        Ok(table)
    }

    pub fn set_params(&mut self, omega: f64, l: f64, sine: GslIntegrationQawoEnum) -> GslError {
        self.omega = omega;
        self.sine = sine;
        self.l = l;
        self.par = 0.5 * omega * l;
        self.compute_moments();
        GslError::Success
    }

    pub fn set_length(&mut self, l: f64) -> GslError {
        if (l - self.l).abs() < f64::EPSILON {
            return GslError::Success;
        }
        self.l = l;
        self.par = 0.5 * self.omega * l;
        self.compute_moments();
        GslError::Success
    }

    fn compute_moments(&mut self) {
        let noeq = 25;
        let par = self.par;
        let par2 = par * par;
        let par4 = par2 * par2;
        let par22 = par2 + 2.0;
        let sinpar = par.sin();
        let cospar = par.cos();

        let mut v = [0.0; 28];
        let mut d = [0.0; 25];
        let mut d1 = [0.0; 25];
        let mut d2 = [0.0; 25];

        // First set of moments
        v[0] = 2.0 * sinpar / par;
        v[1] = (8.0 * cospar + (2.0 * par2 - 8.0) * sinpar / par) / par2;
        v[2] = (32.0 * (par2 - 12.0) * cospar
            + 2.0 * ((par2 - 80.0) * par2 + 192.0) * sinpar / par)
            / par4;

        if par.abs() <= 24.0 {
            // Compute using tridiagonal system
            let mut an = 6.0;
            for k in 0..noeq - 1 {
                let an2 = an * an;
                d[k] = -2.0 * (an2 - 4.0) * (par22 - 2.0 * an2);
                d2[k] = (an - 1.0) * (an - 2.0) * par2;
                d1[k + 1] = (an + 3.0) * (an + 4.0) * par2;
                v[k + 3] = 24.0 * par * sinpar - (an2 - 4.0) * 8.0 * cospar;
                an += 2.0;
            }
            let an2 = an * an;
            d[noeq - 1] = -2.0 * (an2 - 4.0) * (par22 - 2.0 * an2);
            v[noeq + 2] = 24.0 * par * sinpar - (an2 - 4.0) * 8.0 * cospar;
            v[3] -= 56.0 * par2 * v[2];

            let ass = par * sinpar;
            let asap = ((((210.0 * par2 - 1.0) * cospar
                - (105.0 * par2 - 63.0) * ass)
                / (an * an)
                - (1.0 - 15.0 * par2) * cospar
                + 15.0 * ass)
                / (an * an)
                - cospar
                + 3.0 * ass)
                / (an * an)
                - cospar;
            v[noeq + 2] -= 2.0 * asap * par2 * (an - 1.0) * (an - 2.0);

            if dgtsl(
                noeq,
                &mut d1,
                &mut d,
                &mut d2,
                &mut v[3..3 + noeq].try_into().unwrap(),
            ) != GslError::Success
            {
                panic!("Failed to solve tridiagonal system");
            }
        } else {
            // Compute using recurrence
            let mut an = 4.0;
            for k in 3..13 {
                let an2 = an * an;
                v[k] = ((an2 - 4.0)
                    * (2.0 * (par22 - 2.0 * an2) * v[k - 1] - 8.0 * cospar)
                    + 24.0 * par * sinpar
                    - par2
                        * (an + 1.0)
                        * (an + 2.0)
                        * v[k - 2])
                    / (par2 * (an - 1.0) * (an - 2.0));
                an += 2.0;
            }
        }

        // Store first set of moments
        for i in 0..13 {
            self.chebmo[2 * i] = v[i];
        }

        // Second set of moments
        v[0] = 2.0 * (sinpar - par * cospar) / par2;
        v[1] = (18.0 - 48.0 / par2) * sinpar / par2
            + (-2.0 + 48.0 / par2) * cospar / par;
        let ac = -24.0 * par * cospar;
        let as_ = -8.0 * sinpar;

        if par.abs() <= 24.0 {
            // Compute using tridiagonal system
            let mut an = 5.0;
            for k in 0..noeq - 1 {
                let an2 = an * an;
                d[k] = -2.0 * (an2 - 4.0) * (par22 - 2.0 * an2);
                d2[k] = (an - 1.0) * (an - 2.0) * par2;
                d1[k + 1] = (an + 3.0) * (an + 4.0) * par2;
                v[k + 2] = ac + (an2 - 4.0) * as_;
                an += 2.0;
            }
            let an2 = an * an;
            d[noeq - 1] = -2.0 * (an2 - 4.0) * (par22 - 2.0 * an2);
            v[noeq + 1] = ac + (an2 - 4.0) * as_;
            v[2] -= 42.0 * par2 * v[1];

            let ass = par * cospar;
            let asap = ((((105.0 * par2 - 63.0) * ass
                - (210.0 * par2 - 1.0) * sinpar)
                / (an * an)
                + (15.0 * par2 - 1.0) * sinpar
                - 15.0 * ass)
                / (an * an)
                - sinpar
                - 3.0 * ass)
                / (an * an)
                - sinpar;
            v[noeq + 1] -= 2.0 * asap * par2 * (an - 1.0) * (an - 2.0);

            if dgtsl(
                noeq,
                &mut d1,
                &mut d,
                &mut d2,
                &mut v[2..2 + noeq].try_into().unwrap(),
            ) != GslError::Success
            {
                panic!("Failed to solve tridiagonal system");
            }
        } else {
            // Compute using recurrence
            let mut an = 3.0;
            for k in 2..12 {
                let an2 = an * an;
                v[k] = ((an2 - 4.0)
                    * (2.0 * (par22 - 2.0 * an2) * v[k - 1] + as_)
                    + ac
                    - par2 * (an + 1.0) * (an + 2.0) * v[k - 2])
                    / (par2 * (an - 1.0) * (an - 2.0));
                an += 2.0;
            }
        }

        // Store second set of moments
        for i in 0..12 {
            self.chebmo[2 * i + 1] = v[i];
        }
    }
}

fn dgtsl(
    n: usize,
    c: &mut [f64],
    d: &mut [f64],
    e: &mut [f64],
    b: &mut [f64],
) -> GslError {
    if n == 0 {
        return GslError::Success;
    }
    if n == 1 {
        b[0] /= d[0];
        return GslError::Success;
    }

    let mut c = c.to_vec();
    let mut d = d.to_vec();
    let mut e = e.to_vec();
    let mut b = b.to_vec();

    c[0] = d[0];
    d[0] = e[0];
    e[0] = 0.0;
    e[n - 1] = 0.0;

    for k in 0..n - 1 {
        let k1 = k + 1;
        if c[k1].abs() >= c[k].abs() {
            c.swap(k, k1);
            d.swap(k, k1);
            e.swap(k, k1);
            b.swap(k, k1);
        }
        if c[k] == 0.0 {
            return GslError::Failure;
        }
        let t = -c[k1] / c[k];
        c[k1] = d[k1] + t * d[k];
        d[k1] = e[k1] + t * e[k];
        e[k1] = 0.0;
        b[k1] += t * b[k];
    }

    if c[n - 1] == 0.0 {
        return GslError::Failure;
    }

    b[n - 1] /= c[n - 1];
    b[n - 2] = (b[n - 2] - d[n - 2] * b[n - 1]) / c[n - 2];

    for k in (0..n - 2).rev() {
        b[k] = (b[k] - d[k] * b[k + 1] - e[k] * b[k + 2]) / c[k];
    }

    GslError::Success
}