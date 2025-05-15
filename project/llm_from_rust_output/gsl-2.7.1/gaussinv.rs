use std::f64::consts::{INFINITY, NEG_INFINITY};

pub type size_t = usize;

fn rat_eval(
    a: &[f64],
    b: &[f64],
    x: f64,
) -> f64 {
    let mut u = a[a.len() - 1];
    for &coeff in a.iter().rev().skip(1) {
        u = x * u + coeff;
    }

    let mut v = b[b.len() - 1];
    for &coeff in b.iter().rev().skip(1) {
        v = x * v + coeff;
    }

    u / v
}

fn small(q: f64) -> f64 {
    let a = [
        3.387132872796366608f64,
        133.14166789178437745f64,
        1971.5909503065514427f64,
        13731.693765509461125f64,
        45921.953931549871457f64,
        67265.770927008700853f64,
        33430.575583588128105f64,
        2509.0809287301226727f64,
    ];
    let b = [
        1.0f64,
        42.313330701600911252f64,
        687.1870074920579083f64,
        5394.1960214247511077f64,
        21213.794301586595867f64,
        39307.89580009271061f64,
        28729.085735721942674f64,
        5226.495278852854561f64,
    ];
    let r = 0.180625f64 - q * q;
    q * rat_eval(&a, &b, r)
}

fn intermediate(r: f64) -> f64 {
    let a = [
        1.42343711074968357734f64,
        4.6303378461565452959f64,
        5.7694972214606914055f64,
        3.64784832476320460504f64,
        1.27045825245236838258f64,
        0.24178072517745061177f64,
        0.0227238449892691845833f64,
        7.7454501427834140764e-4f64,
    ];
    let b = [
        1.0f64,
        2.05319162663775882187f64,
        1.6763848301838038494f64,
        0.68976733498510000455f64,
        0.14810397642748007459f64,
        0.0151986665636164571966f64,
        5.475938084995344946e-4f64,
        1.05075007164441684324e-9f64,
    ];
    rat_eval(&a, &b, r - 1.6f64)
}

fn tail(r: f64) -> f64 {
    let a = [
        6.6579046435011037772f64,
        5.4637849111641143699f64,
        1.7848265399172913358f64,
        0.29656057182850489123f64,
        0.026532189526576123093f64,
        0.0012426609473880784386f64,
        2.71155556874348757815e-5f64,
        2.01033439929228813265e-7f64,
    ];
    let b = [
        1.0f64,
        0.59983220655588793769f64,
        0.13692988092273580531f64,
        0.0148753612908506148525f64,
        7.868691311456132591e-4f64,
        1.8463183175100546818e-5f64,
        1.4215117583164458887e-7f64,
        2.04426310338993978564e-15f64,
    ];
    rat_eval(&a, &b, r - 5.0f64)
}

pub fn gsl_cdf_ugaussian_pinv(p: f64) -> f64 {
    let d_p = p - 0.5f64;
    if p == 1.0f64 {
        return INFINITY;
    } else if p == 0.0f64 {
        return NEG_INFINITY;
    }

    if d_p.abs() <= 0.425f64 {
        return small(d_p);
    }

    let pp = if p < 0.5f64 { p } else { 1.0f64 - p };
    let r = (-pp.ln()).sqrt();
    let x = if r <= 5.0f64 {
        intermediate(r)
    } else {
        tail(r)
    };

    if p < 0.5f64 { -x } else { x }
}

pub fn gsl_cdf_ugaussian_qinv(q: f64) -> f64 {
    let d_q = q - 0.5f64;
    if q == 1.0f64 {
        return NEG_INFINITY;
    } else if q == 0.0f64 {
        return INFINITY;
    }

    if d_q.abs() <= 0.425f64 {
        return -small(d_q);
    }

    let pp = if q < 0.5f64 { q } else { 1.0f64 - q };
    let r = (-pp.ln()).sqrt();
    let x = if r <= 5.0f64 {
        intermediate(r)
    } else {
        tail(r)
    };

    if q < 0.5f64 { x } else { -x }
}

pub fn gsl_cdf_gaussian_pinv(p: f64, sigma: f64) -> f64 {
    sigma * gsl_cdf_ugaussian_pinv(p)
}

pub fn gsl_cdf_gaussian_qinv(q: f64, sigma: f64) -> f64 {
    sigma * gsl_cdf_ugaussian_qinv(q)
}