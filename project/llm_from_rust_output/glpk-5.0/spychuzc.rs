use std::cmp::Ordering;
use std::f64;

struct SPXLP {
    m: i32,
    n: i32,
    nnz: i32,
    a_ptr: Vec<i32>,
    a_ind: Vec<i32>,
    a_val: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    l: Vec<f64>,
    u: Vec<f64>,
    head: Vec<i32>,
    flag: Vec<bool>,
    valid: i32,
}

#[derive(Clone, Copy)]
struct SPYBP {
    j: i32,
    teta: f64,
    dz: f64,
}

fn glp_spy_chuzc_std(
    lp: &SPXLP,
    d: &[f64],
    r: f64,
    trow: &[f64],
    tol_piv: f64,
    tol: f64,
    tol1: f64,
) -> i32 {
    assert_ne!(r, 0.0, "r != 0.0");
    let m = lp.m;
    let n = lp.n;
    let s = if r > 0.0 { 1.0 } else { -1.0 };
    let mut q = 0;
    let mut teta_min = f64::MAX;
    let mut biga = 0.0;

    for j in 1..=(n - m) {
        let k = lp.head[(m + j) as usize];
        if lp.l[k as usize] != lp.u[k as usize] {
            let alfa = s * trow[j as usize];
            let teta = if alfa >= tol_piv && !lp.flag[j as usize] {
                let delta = tol + tol1 * lp.c[k as usize].abs();
                if d[j as usize] < delta {
                    0.0
                } else {
                    d[j as usize] / alfa
                }
            } else if alfa <= -tol_piv && (lp.l[k as usize] == f64::MIN || lp.flag[j as usize]) {
                let delta = tol + tol1 * lp.c[k as usize].abs();
                if d[j as usize] > -delta {
                    0.0
                } else {
                    d[j as usize] / alfa
                }
            } else {
                continue;
            };

            assert!(teta >= 0.0, "teta >= 0.0");
            let alfa_abs = alfa.abs();
            if teta_min > teta || (teta_min == teta && biga < alfa_abs) {
                q = j;
                teta_min = teta;
                biga = alfa_abs;
            }
        }
    }
    q
}

fn glp_spy_chuzc_harris(
    lp: &SPXLP,
    d: &[f64],
    r: f64,
    trow: &[f64],
    tol_piv: f64,
    tol: f64,
    tol1: f64,
) -> i32 {
    assert_ne!(r, 0.0, "r != 0.0");
    let m = lp.m;
    let n = lp.n;
    let s = if r > 0.0 { 1.0 } else { -1.0 };
    let mut teta_min = f64::MAX;

    for j in 1..=(n - m) {
        let k = lp.head[(m + j) as usize];
        if lp.l[k as usize] != lp.u[k as usize] {
            let alfa = s * trow[j as usize];
            let teta = if alfa >= tol_piv && !lp.flag[j as usize] {
                let delta = tol + tol1 * lp.c[k as usize].abs();
                (d[j as usize].max(0.0) + delta / alfa
            } else if alfa <= -tol_piv && (lp.l[k as usize] == f64::MIN || lp.flag[j as usize]) {
                let delta = tol + tol1 * lp.c[k as usize].abs();
                (d[j as usize].min(0.0) - delta) / alfa
            } else {
                continue;
            };

            assert!(teta >= 0.0, "teta >= 0.0");
            if teta_min > teta {
                teta_min = teta;
            }
        }
    }

    if teta_min == f64::MAX {
        0
    } else {
        let mut q = 0;
        let mut biga = 0.0;
        for j in 1..=(n - m) {
            let k = lp.head[(m + j) as usize];
            if lp.l[k as usize] != lp.u[k as usize] {
                let alfa = s * trow[j as usize];
                let teta = if alfa >= tol_piv && !lp.flag[j as usize] {
                    d[j as usize] / alfa
                } else if alfa <= -tol_piv && (lp.l[k as usize] == f64::MIN || lp.flag[j as usize]) {
                    d[j as usize] / alfa
                } else {
                    continue;
                };

                let alfa_abs = alfa.abs();
                if teta <= teta_min && biga < alfa_abs {
                    q = j;
                    biga = alfa_abs;
                }
            }
        }
        assert!((1..=(n - m)).contains(&q), "1 <= q && q <= n-m");
        q
    }
}

fn glp_spy_ls_eval_bp(
    lp: &SPXLP,
    d: &[f64],
    r: f64,
    trow: &[f64],
    tol_piv: f64,
    bp: &mut [SPYBP],
) -> i32 {
    assert_ne!(r, 0.0, "r != 0.0");
    let m = lp.m;
    let n = lp.n;
    let s = if r > 0.0 { 1.0 } else { -1.0 };
    let mut nnn = 0;
    let mut teta_max = f64::MAX;

    for j in 1..=(n - m) {
        let k = lp.head[(m + j) as usize];
        if lp.l[k as usize] != lp.u[k as usize] {
            let alfa = s * trow[j as usize];
            let teta = if alfa >= tol_piv && !lp.flag[j as usize] {
                let t = d[j as usize].max(0.0) / alfa;
                if lp.u[k as usize] == f64::MAX && teta_max > t {
                    teta_max = t;
                }
                t
            } else if alfa <= -tol_piv && (lp.l[k as usize] == f64::MIN || lp.flag[j as usize]) {
                let t = d[j as usize].min(0.0) / alfa;
                if lp.l[k as usize] == f64::MIN && teta_max > t {
                    teta_max = t;
                }
                t
            } else {
                continue;
            };

            nnn += 1;
            bp[nnn as usize] = SPYBP { j, teta, dz: 0.0 };
        }
    }

    let mut nbp = 0;
    for t in 1..=nnn {
        if bp[t as usize].teta <= teta_max + 1e-6 {
            nbp += 1;
            bp[nbp as usize] = bp[t as usize];
        }
    }
    nbp
}

fn fcmp(p1: &SPYBP, p2: &SPYBP) -> Ordering {
    p1.teta.partial_cmp(&p2.teta).unwrap_or(Ordering::Equal)
}

fn glp_spy_ls_select_bp(
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
    assert!((0..=nbp).contains(&num) && nbp <= n - m, "0 <= num && num <= nbp && nbp <= n-m");

    let mut num1 = num;
    for t in (num + 1)..=nbp {
        if bp[t as usize].teta <= teta_lim {
            num1 += 1;
            bp.swap(num1 as usize, t as usize);
        }
    }

    if num1 - num > 1 {
        bp[(num + 1) as usize..=num1 as usize].sort_by(fcmp);
    }

    for t in (num + 1)..=num1 {
        let dz = if *slope == f64::MIN {
            f64::MIN
        } else {
            *slope * (bp[t as usize].teta - if t == 1 { 0.0 } else { bp[(t - 1) as usize].teta })
        };

        bp[t as usize].dz = if dz == f64::MIN {
            f64::MIN
        } else {
            if t == 1 { 0.0 } else { bp[(t - 1) as usize].dz } + dz
        };

        if *slope != f64::MIN {
            let j = bp[t as usize].j;
            let k = lp.head[(m + j) as usize];
            if lp.l[k as usize] == f64::MIN || lp.u[k as usize] == f64::MAX {
                *slope = f64::MIN;
            } else {
                assert!(lp.l[k as usize] < lp.u[k as usize], "l[k] < u[k]");
                *slope -= trow[j as usize].abs() * (lp.u[k as usize] - lp.l[k as usize]);
            }
        }
    }
    num1
}