use std::f64::consts::PI;
use std::ptr::null_mut;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub enum QawoEnum {
    Sine,
    Cosine,
}

#[derive(Debug)]
pub struct QawoTable {
    n: usize,
    sine: QawoEnum,
    omega: f64,
    l: f64,
    par: f64,
    chebmo: Vec<f64>,
}

#[derive(Debug)]
pub enum IntegrationError {
    DomainError(&'static str),
    MemoryError(&'static str),
    Failure,
}

impl QawoTable {
    pub fn new(omega: f64, l: f64, sine: QawoEnum, n: usize) -> Result<Self, IntegrationError> {
        if n == 0 {
            return Err(IntegrationError::DomainError("table length n must be positive integer"));
        }

        let mut chebmo = vec![0.0; 25 * n];
        let par = 0.5 * omega * l;

        let mut table = QawoTable {
            n,
            sine,
            omega,
            l,
            par,
            chebmo,
        };

        // precompute the moments
        let mut scale = 1.0;
        for i in 0..table.n {
            compute_moments(table.par * scale, &mut table.chebmo[25*i..25*(i+1)]);
            scale *= 0.5;
        }

        Ok(table)
    }

    pub fn set(&mut self, omega: f64, l: f64, sine: QawoEnum) {
        self.omega = omega;
        self.sine = sine;
        self.l = l;
        self.par = 0.5 * omega * l;

        // recompute the moments
        let mut scale = 1.0;
        for i in 0..self.n {
            compute_moments(self.par * scale, &mut self.chebmo[25*i..25*(i+1)]);
            scale *= 0.5;
        }
    }

    pub fn set_length(&mut self, l: f64) -> Result<(), IntegrationError> {
        if l == self.l {
            return Ok(());
        }

        self.l = l;
        self.par = 0.5 * self.omega * l;

        // recompute the moments
        let mut scale = 1.0;
        for i in 0..self.n {
            compute_moments(self.par * scale, &mut self.chebmo[25*i..25*(i+1)]);
            scale *= 0.5;
        }

        Ok(())
    }
}

fn compute_moments(par: f64, chebmo: &mut [f64]) {
    let mut v = [0.0; 28];
    let mut d = [0.0; 25];
    let mut d1 = [0.0; 25];
    let mut d2 = [0.0; 25];

    let noeq = 25;
    
    let par2 = par * par;
    let par4 = par2 * par2;
    let par22 = par2 + 2.0;

    let sinpar = par.sin();
    let cospar = par.cos();

    // compute the chebyschev moments with respect to cosine
    let ac = 8.0 * cospar;
    let as_ = 24.0 * par * sinpar;

    v[0] = 2.0 * sinpar / par;
    v[1] = (8.0 * cospar + (2.0 * par2 - 8.0) * sinpar / par) / par2;
    v[2] = (32.0 * (par2 - 12.0) * cospar
          + (2.0 * ((par2 - 80.0) * par2 + 192.0) * sinpar / par) / par4;

    if par.abs() <= 24.0 {
        // compute the moments as the solution of a boundary value problem
        let mut an = 6.0;
        for k in 0..noeq-1 {
            let an2 = an * an;
            d[k] = -2.0 * (an2 - 4.0) * (par22 - 2.0 * an2);
            d2[k] = (an - 1.0) * (an - 2.0) * par2;
            d1[k + 1] = (an + 3.0) * (an + 4.0) * par2;
            v[k + 3] = as_ - (an2 - 4.0) * ac;
            an += 2.0;
        }

        let an2 = an * an;
        d[noeq - 1] = -2.0 * (an2 - 4.0) * (par22 - 2.0 * an2);
        v[noeq + 2] = as_ - (an2 - 4.0) * ac;
        v[3] = v[3] - 56.0 * par2 * v[2];

        let ass = par * sinpar;
        let asap = (((((210.0 * par2 - 1.0) * cospar - (105.0 * par2 - 63.0) * ass) / an2
                    - (1.0 - 15.0 * par2) * cospar + 15.0 * ass) / an2 
                   - cospar + 3.0 * ass) / an2 
                  - cospar) / an2;
        v[noeq + 2] = v[noeq + 2] - 2.0 * asap * par2 * (an - 1.0) * (an - 2.0);

        dgtsl(noeq, &mut d1, &mut d, &mut d2, &mut v[3..]).unwrap();
    } else {
        // compute the moments by forward recursion
        let mut an = 4.0;
        for k in 3..13 {
            let an2 = an * an;
            v[k] = ((an2 - 4.0) * (2.0 * (par22 - 2.0 * an2) * v[k - 1] - ac)
                  + as_ - par2 * (an + 1.0) * (an + 2.0) * v[k - 2]) 
                / (par2 * (an - 1.0) * (an - 2.0));
            an += 2.0;
        }
    }

    for i in 0..13 {
        chebmo[2 * i] = v[i];
    }

    // compute the chebyschev moments with respect to sine
    v[0] = 2.0 * (sinpar - par * cospar) / par2;
    v[1] = (18.0 - 48.0 / par2) * sinpar / par2 + (-2.0 + 48.0 / par2) * cospar / par;

    let ac = -24.0 * par * cospar;
    let as_ = -8.0 * sinpar;

    if par.abs() <= 24.0 {
        // compute the moments as the solution of a boundary value problem
        let mut an = 5.0;
        for k in 0..noeq-1 {
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
        v[2] = v[2] - 42.0 * par2 * v[1];

        let ass = par * cospar;
        let asap = (((((105.0 * par2 - 63.0) * ass - (210.0 * par2 - 1.0) * sinpar) / an2
                    + (15.0 * par2 - 1.0) * sinpar
                    - 15.0 * ass) / an2 - sinpar - 3.0 * ass) / an2 - sinpar) / an2;
        v[noeq + 1] = v[noeq + 1] - 2.0 * asap * par2 * (an - 1.0) * (an - 2.0);

        dgtsl(noeq, &mut d1, &mut d, &mut d2, &mut v[2..]).unwrap();
    } else {
        // compute the moments by forward recursion
        let mut an = 3.0;
        for k in 2..12 {
            let an2 = an * an;
            v[k] = ((an2 - 4.0) * (2.0 * (par22 - 2.0 * an2) * v[k - 1] + as_)
                  + ac - par2 * (an + 1.0) * (an + 2.0) * v[k - 2]) 
                / (par2 * (an - 1.0) * (an - 2.0));
            an += 2.0;
        }
    }

    for i in 0..12 {
        chebmo[2 * i + 1] = v[i];
    }
}

fn dgtsl(n: usize, c: &mut [f64], d: &mut [f64], e: &mut [f64], b: &mut [f64]) -> Result<(), IntegrationError> {
    if n == 0 {
        return Ok(());
    }

    if n == 1 {
        b[0] = b[0] / d[0];
        return Ok(());
    }

    d[0] = e[0];
    e[0] = 0.0;
    e[n - 1] = 0.0;

    for k in 0..n-1 {
        let k1 = k + 1;

        if c[k1].abs() >= c[k].abs() {
            c.swap(k, k1);
            d.swap(k, k1);
            e.swap(k, k1);
            b.swap(k, k1);
        }

        if c[k] == 0.0 {
            return Err(IntegrationError::Failure);
        }

        let t = -c[k1] / c[k];
        c[k1] = d[k1] + t * d[k];
        d[k1] = e[k1] + t * e[k];
        e[k1] = 0.0;
        b[k1] = b[k1] + t * b[k];
    }

    if c[n - 1] == 0.0 {
        return Err(IntegrationError::Failure);
    }

    b[n - 1] = b[n - 1] / c[n - 1];
    b[n - 2] = (b[n - 2] - d[n - 2] * b[n - 1]) / c[n - 2];

    for kb in (0..n-2).rev() {
        b[kb] = (b[kb] - d[kb] * b[kb + 1] - e[kb] * b[kb + 2]) / c[kb];
    }

    Ok(())
}