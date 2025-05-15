use std::f64;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct IntegrationFixedWorkspace {
    n: usize,
    weights: Vec<f64>,
    x: Vec<f64>,
    diag: Vec<f64>,
    subdiag: Vec<f64>,
    type_: &'static IntegrationFixedType,
}

#[derive(Debug)]
pub struct IntegrationFixedParams {
    a: f64,
    b: f64,
    alpha: f64,
    beta: f64,
    zemu: f64,
    slp: f64,
    shft: f64,
    al: f64,
    be: f64,
}

pub struct IntegrationFixedType {
    pub check: fn(usize, &IntegrationFixedParams) -> i32,
    pub init: fn(usize, &mut [f64], &mut [f64], &IntegrationFixedParams) -> i32,
}

pub struct Function<'a> {
    pub function: &'a dyn Fn(f64) -> f64,
}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_EDOM: i32 = 1;
pub const GSL_ENOMEM: i32 = 2;
pub const GSL_EINVAL: i32 = 3;
pub const GSL_EMAXITER: i32 = 4;
pub const GSL_DBL_EPSILON: f64 = f64::EPSILON;

pub fn gsl_integration_fixed_alloc(
    type_: &'static IntegrationFixedType,
    n: usize,
    a: f64,
    b: f64,
    alpha: f64,
    beta: f64,
) -> Result<IntegrationFixedWorkspace, i32> {
    if n < 1 {
        return Err(GSL_EDOM);
    }

    let mut w = IntegrationFixedWorkspace {
        n,
        weights: vec![0.0; n],
        x: vec![0.0; n],
        diag: vec![0.0; n],
        subdiag: vec![0.0; n],
        type_,
    };

    match fixed_compute(a, b, alpha, beta, &mut w) {
        GSL_SUCCESS => Ok(w),
        status => Err(status),
    }
}

pub fn gsl_integration_fixed_free(_w: IntegrationFixedWorkspace) {
    // Memory is automatically freed when the struct is dropped
}

pub fn gsl_integration_fixed_n(w: &IntegrationFixedWorkspace) -> usize {
    w.n
}

pub fn gsl_integration_fixed_nodes(w: &IntegrationFixedWorkspace) -> &[f64] {
    &w.x
}

pub fn gsl_integration_fixed_weights(w: &IntegrationFixedWorkspace) -> &[f64] {
    &w.weights
}

pub fn gsl_integration_fixed(
    func: &Function,
    result: &mut f64,
    w: &IntegrationFixedWorkspace,
) -> i32 {
    let mut sum = 0.0;
    for i in 0..w.n {
        let fi = (func.function)(w.x[i]);
        sum += w.weights[i] * fi;
    }
    *result = sum;
    GSL_SUCCESS
}

fn fixed_compute(
    a: f64,
    b: f64,
    alpha: f64,
    beta: f64,
    w: &mut IntegrationFixedWorkspace,
) -> i32 {
    let mut params = IntegrationFixedParams {
        a,
        b,
        alpha,
        beta,
        zemu: 0.0,
        slp: 0.0,
        shft: 0.0,
        al: 0.0,
        be: 0.0,
    };

    let s = (w.type_.check)(w.n, &params);
    if s != GSL_SUCCESS {
        return s;
    }

    let s = (w.type_.init)(w.n, &mut w.diag, &mut w.subdiag, &params);
    if s != GSL_SUCCESS {
        return s;
    }

    if params.zemu <= 0.0 {
        return GSL_EINVAL;
    }

    w.x.copy_from_slice(&w.diag);
    w.weights[0] = params.zemu.sqrt();

    for i in 1..w.n {
        w.weights[i] = 0.0;
    }

    let s = imtqlx(w.n, &mut w.x, &mut w.subdiag, &mut w.weights);
    if s != GSL_SUCCESS {
        return s;
    }

    for weight in &mut w.weights {
        *weight *= *weight;
    }

    let p = params.slp.powf(params.al + params.be + 1.0);
    for k in 0..w.n {
        w.x[k] = params.shft + params.slp * w.x[k];
        w.weights[k] *= p;
    }

    GSL_SUCCESS
}

fn imtqlx(n: usize, d: &mut [f64], e: &mut [f64], z: &mut [f64]) -> i32 {
    if n == 1 {
        return GSL_SUCCESS;
    }

    e[n - 1] = 0.0;

    for l in 1..=n {
        let mut j = 0;
        let mut m = l;
        loop {
            for mm in l..n {
                if (e[mm - 1].abs() <= GSL_DBL_EPSILON * (d[mm - 1].abs() + d[mm].abs()) {
                    break;
                }
                m = mm + 1;
            }
            let p = d[l - 1];
            if m == l {
                break;
            }
            if j >= 30 {
                return GSL_EMAXITER;
            }
            j += 1;
            let g = (d[l] - p) / (2.0 * e[l - 1]);
            let r = (g * g + 1.0).sqrt();
            let g = d[m - 1] - p + e[l - 1] / (g + r.abs() * g.signum());
            let mut s = 1.0;
            let mut c = 1.0;
            let mut p = 0.0;
            let mml = m - l;

            for ii in 1..=mml {
                let i = m - ii;
                let f = s * e[i - 1];
                let b = c * e[i - 1];

                if g.abs() <= f.abs() {
                    c = g / f;
                    r = (c * c + 1.0).sqrt();
                    e[i] = f * r;
                    s = 1.0 / r;
                    c *= s;
                } else {
                    s = f / g;
                    r = (s * s + 1.0).sqrt();
                    e[i] = g * r;
                    c = 1.0 / r;
                    s *= c;
                }
                g = d[i] - p;
                r = (d[i - 1] - g) * s + 2.0 * c * b;
                p = s * r;
                d[i] = g + p;
                g = c * r - b;
                let f = z[i];
                z[i] = s * z[i - 1] + c * f;
                z[i - 1] = c * z[i - 1] - s * f;
            }
            d[l - 1] -= p;
            e[l - 1] = g;
            e[m - 1] = 0.0;
        }
    }

    for ii in 2..=n {
        let i = ii - 1;
        let mut k = i;
        let mut p = d[i - 1];

        for j in ii..=n {
            if d[j - 1] < p {
                k = j;
                p = d[j - 1];
            }
        }

        if k != i {
            d.swap(k - 1, i - 1);
            z.swap(k - 1, i - 1);
        }
    }

    GSL_SUCCESS
}

fn gsl_sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}