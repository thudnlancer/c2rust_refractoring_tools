/* spychuzc.rs */

use std::cmp::Ordering;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SPYBP {
    pub j: i32,
    pub teta: f64,
    pub dz: f64,
}

#[derive(Debug)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub c: Vec<f64>,
    pub l: Vec<f64>,
    pub u: Vec<f64>,
    pub head: Vec<i32>,
    pub flag: Vec<bool>,
}

pub fn spy_chuzc_std(
    lp: &SPXLP,
    d: &[f64],
    r: f64,
    trow: &[f64],
    tol_piv: f64,
    tol: f64,
    tol1: f64,
) -> i32 {
    let m = lp.m;
    let n = lp.n;
    let c = &lp.c;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let flag = &lp.flag;

    let s = if r > 0.0 { 1.0 } else { -1.0 };
    let mut q = 0;
    let mut teta_min = f64::MAX;
    let mut biga = 0.0;

    for j in 1..=(n - m) {
        let k = head[(m + j - 1) as usize] as usize;
        if l[k] == u[k] {
            continue;
        }

        let alfa = s * trow[(j - 1) as usize];
        let teta = if alfa >= tol_piv && !flag[(j - 1) as usize] {
            let delta = tol + tol1 * if c[k] >= 0.0 { c[k] } else { -c[k] };
            if d[(j - 1) as usize] < delta { 0.0 } else { d[(j - 1) as usize] / alfa }
        } else if alfa <= -tol_piv && (l[k] == -f64::MAX || flag[(j - 1) as usize]) {
            let delta = tol + tol1 * if c[k] >= 0.0 { c[k] } else { -c[k] };
            if d[(j - 1) as usize] > -delta { 0.0 } else { d[(j - 1) as usize] / alfa }
        } else {
            continue;
        };

        assert!(teta >= 0.0);
        let abs_alfa = alfa.abs();
        if teta_min > teta || (teta_min == teta && biga < abs_alfa) {
            q = j;
            teta_min = teta;
            biga = abs_alfa;
        }
    }

    q
}

pub fn spy_chuzc_harris(
    lp: &SPXLP,
    d: &[f64],
    r: f64,
    trow: &[f64],
    tol_piv: f64,
    tol: f64,
    tol1: f64,
) -> i32 {
    let m = lp.m;
    let n = lp.n;
    let c = &lp.c;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let flag = &lp.flag;

    let s = if r > 0.0 { 1.0 } else { -1.0 };
    let mut teta_min = f64::MAX;

    for j in 1..=(n - m) {
        let k = head[(m + j - 1) as usize] as usize;
        if l[k] == u[k] {
            continue;
        }

        let alfa = s * trow[(j - 1) as usize];
        let teta = if alfa >= tol_piv && !flag[(j - 1) as usize] {
            let delta = tol + tol1 * if c[k] >= 0.0 { c[k] } else { -c[k] };
            ((if d[(j - 1) as usize] < 0.0 { 0.0 } else { d[(j - 1) as usize] }) + delta) / alfa
        } else if alfa <= -tol_piv && (l[k] == -f64::MAX || flag[(j - 1) as usize]) {
            let delta = tol + tol1 * if c[k] >= 0.0 { c[k] } else { -c[k] };
            ((if d[(j - 1) as usize] > 0.0 { 0.0 } else { d[(j - 1) as usize] }) - delta) / alfa
        } else {
            continue;
        };

        assert!(teta >= 0.0);
        if teta_min > teta {
            teta_min = teta;
        }
    }

    if teta_min == f64::MAX {
        return 0;
    }

    let mut q = 0;
    let mut biga = 0.0;

    for j in 1..=(n - m) {
        let k = head[(m + j - 1) as usize] as usize;
        if l[k] == u[k] {
            continue;
        }

        let alfa = s * trow[(j - 1) as usize];
        let teta = if alfa >= tol_piv && !flag[(j - 1) as usize] {
            d[(j - 1) as usize] / alfa
        } else if alfa <= -tol_piv && (l[k] == -f64::MAX || flag[(j - 1) as usize]) {
            d[(j - 1) as usize] / alfa
        } else {
            continue;
        };

        let abs_alfa = alfa.abs();
        if teta <= teta_min && biga < abs_alfa {
            q = j;
            biga = abs_alfa;
        }
    }

    assert!(1 <= q && q <= n - m);
    q
}

pub fn spy_ls_eval_bp(
    lp: &SPXLP,
    d: &[f64],
    r: f64,
    trow: &[f64],
    tol_piv: f64,
    bp: &mut [SPYBP],
) -> i32 {
    let m = lp.m;
    let n = lp.n;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let flag = &lp.flag;

    let s = if r > 0.0 { 1.0 } else { -1.0 };
    let mut nnn = 0;
    let mut teta_max = f64::MAX;

    for j in 1..=(n - m) {
        let k = head[(m + j - 1) as usize] as usize;
        if l[k] == u[k] {
            continue;
        }

        let alfa = s * trow[(j - 1) as usize];
        let teta = if alfa >= tol_piv && !flag[(j - 1) as usize] {
            if d[(j - 1) as usize] < 0.0 { 0.0 } else { d[(j - 1) as usize] / alfa }
        } else if alfa <= -tol_piv && (l[k] == -f64::MAX || flag[(j - 1) as usize]) {
            if d[(j - 1) as usize] > 0.0 { 0.0 } else { d[(j - 1) as usize] / alfa }
        } else {
            continue;
        };

        if alfa >= tol_piv && !flag[(j - 1) as usize] && u[k] == f64::MAX && teta_max > teta {
            teta_max = teta;
        }
        if alfa <= -tol_piv && (l[k] == -f64::MAX || flag[(j - 1) as usize]) && l[k] == -f64::MAX && teta_max > teta {
            teta_max = teta;
        }

        nnn += 1;
        bp[nnn - 1].j = j;
        bp[nnn - 1].teta = teta;
    }

    let mut nbp = 0;
    for t in 0..nnn {
        if bp[t].teta <= teta_max + 1e-6 {
            bp[nbp] = bp[t];
            nbp += 1;
        }
    }

    nbp as i32
}

fn fcmp(a: &SPYBP, b: &SPYBP) -> Ordering {
    a.teta.partial_cmp(&b.teta).unwrap_or(Ordering::Equal)
}

pub fn spy_ls_select_bp(
    lp: &SPXLP,
    trow: &[f64],
    nbp: i32,
    bp: &mut [SPYBP],
    num: i32,
    slope: &mut f64,
    teta_lim: f64,
) -> i32 {
    let m = lp.m;
    let n = lp.n;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;

    let mut num1 = num;
    for t in num..nbp {
        if bp[t as usize].teta <= teta_lim {
            bp.swap(num1 as usize, t as usize);
            num1 += 1;
        }
    }

    if num1 - num > 1 {
        bp[num as usize..num1 as usize].sort_by(fcmp);
    }

    for t in num..num1 {
        let dz = if *slope == -f64::MAX {
            -f64::MAX
        } else {
            *slope * (bp[t as usize].teta - if t == 0 { 0.0 } else { bp[(t - 1) as usize].teta })
        };

        bp[t as usize].dz = if t == 0 {
            0.0
        } else {
            bp[(t - 1) as usize].dz + dz
        };

        if *slope != -f64::MAX {
            let j = bp[t as usize].j;
            let k = head[(m + j - 1) as usize] as usize;
            if l[k] == -f64::MAX || u[k] == f64::MAX {
                *slope = -f64::MAX;
            } else {
                assert!(l[k] < u[k]);
                *slope -= trow[(j - 1) as usize].abs() * (u[k] - l[k]);
            }
        }
    }

    num1
}