/*!
 * Computes the inverse normal cumulative distribution function 
 * according to the algorithm shown in 
 *
 *      Wichura, M.J. (1988).
 *      Algorithm AS 241: The Percentage Points of the Normal Distribution.
 *      Applied Statistics, 37, 477-484.
 */

use std::f64::consts::{INFINITY, NEG_INFINITY};
use std::f64;

fn rat_eval(a: &[f64], b: &[f64], r: f64) -> f64 {
    let num = a.iter().rev().fold(0.0, |acc, &coeff| acc * r + coeff);
    let den = b.iter().rev().fold(0.0, |acc, &coeff| acc * r + coeff);
    num / den
}

fn small(q: f64) -> f64 {
    const A: [f64; 8] = [
        3.387132872796366608,
        133.14166789178437745,
        1971.5909503065514427,
        13731.693765509461125,
        45921.953931549871457,
        67265.770927008700853,
        33430.575583588128105,
        2509.0809287301226727,
    ];

    const B: [f64; 8] = [
        1.0,
        42.313330701600911252,
        687.1870074920579083,
        5394.1960214247511077,
        21213.794301586595867,
        39307.89580009271061,
        28729.085735721942674,
        5226.495278852854561,
    ];

    let r = 0.180625 - q * q;
    q * rat_eval(&A, &B, r)
}

fn intermediate(r: f64) -> f64 {
    const A: [f64; 8] = [
        1.42343711074968357734,
        4.6303378461565452959,
        5.7694972214606914055,
        3.64784832476320460504,
        1.27045825245236838258,
        0.24178072517745061177,
        0.0227238449892691845833,
        7.7454501427834140764e-4,
    ];

    const B: [f64; 8] = [
        1.0,
        2.05319162663775882187,
        1.6763848301838038494,
        0.68976733498510000455,
        0.14810397642748007459,
        0.0151986665636164571966,
        5.475938084995344946e-4,
        1.05075007164441684324e-9,
    ];

    rat_eval(&A, &B, r - 1.6)
}

fn tail(r: f64) -> f64 {
    const A: [f64; 8] = [
        6.6579046435011037772,
        5.4637849111641143699,
        1.7848265399172913358,
        0.29656057182850489123,
        0.026532189526576123093,
        0.0012426609473880784386,
        2.71155556874348757815e-5,
        2.01033439929228813265e-7,
    ];

    const B: [f64; 8] = [
        1.0,
        0.59983220655588793769,
        0.13692988092273580531,
        0.0148753612908506148525,
        7.868691311456132591e-4,
        1.8463183175100546818e-5,
        1.4215117583164458887e-7,
        2.04426310338993978564e-15,
    ];

    rat_eval(&A, &B, r - 5.0)
}

pub fn ugaussian_pinv(p: f64) -> f64 {
    let dp = p - 0.5;

    if p == 1.0 {
        return INFINITY;
    } else if p == 0.0 {
        return NEG_INFINITY;
    }

    if dp.abs() <= 0.425 {
        return small(dp);
    }

    let pp = if p < 0.5 { p } else { 1.0 - p };
    let r = (-pp.ln()).sqrt();

    let x = if r <= 5.0 {
        intermediate(r)
    } else {
        tail(r)
    };

    if p < 0.5 {
        -x
    } else {
        x
    }
}

pub fn ugaussian_qinv(q: f64) -> f64 {
    let dq = q - 0.5;

    if q == 1.0 {
        return NEG_INFINITY;
    } else if q == 0.0 {
        return INFINITY;
    }

    if dq.abs() <= 0.425 {
        return -small(dq);
    }

    let pp = if q < 0.5 { q } else { 1.0 - q };
    let r = (-pp.ln()).sqrt();

    let x = if r <= 5.0 {
        intermediate(r)
    } else {
        tail(r)
    };

    if q < 0.5 {
        x
    } else {
        -x
    }
}

pub fn gaussian_pinv(p: f64, sigma: f64) -> f64 {
    sigma * ugaussian_pinv(p)
}

pub fn gaussian_qinv(q: f64, sigma: f64) -> f64 {
    sigma * ugaussian_qinv(q)
}