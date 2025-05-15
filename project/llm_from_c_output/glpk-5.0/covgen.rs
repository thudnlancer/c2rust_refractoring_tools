/* covgen.rs */

use std::f64::{DBL_MAX, DBL_MIN};
use std::cmp::Ordering;
use std::collections::HashMap;

const EPSILON: f64 = 1e-9;

#[derive(Debug, Clone)]
struct Bnd {
    z: i32,
    a: f64,
    b: f64,
}

#[derive(Debug)]
struct Csa {
    p: glp_prob,
    l: Vec<Bnd>,
    u: Vec<Bnd>,
    set: glp_prob,
}

#[derive(Debug)]
struct GlpCov {
    n: i32,
    set: glp_prob,
}

fn init_bounds(csa: &mut Csa) {
    for j in 1..=csa.p.n {
        csa.l[j as usize].z = 0;
        csa.u[j as usize].z = 0;
        csa.l[j as usize].a = 0.0;
        csa.u[j as usize].a = 0.0;
        csa.l[j as usize].b = csa.p.get_col_lb(j);
        csa.u[j as usize].b = csa.p.get_col_ub(j);
    }
}

fn check_vb(csa: &Csa, i: i32, x: &mut i32, z: &mut i32, a: &mut f64, b: &mut f64) -> i32 {
    let row = &csa.p.row[i as usize];
    let mut a1 = row.ptr;
    if a1.is_none() {
        return 0;
    }
    let mut a2 = a1.unwrap().r_next;
    if a2.is_none() {
        return 0;
    }
    if a2.unwrap().r_next.is_some() {
        return 0;
    }

    if csa.p.get_col_kind(a1.unwrap().col.j) == GLP_BV {
        std::mem::swap(&mut a1, &mut a2);
    }

    if a1.unwrap().col.type_ == GLP_FX || csa.p.get_col_kind(a1.unwrap().col.j) == GLP_BV {
        return 0;
    }
    if csa.p.get_col_kind(a2.unwrap().col.j) != GLP_BV {
        return 0;
    }

    let (mut type_, rhs) = match row.type_ {
        GLP_LO => {
            if a1.unwrap().val > 0.0 {
                (GLP_LO, row.lb)
            } else {
                (GLP_UP, row.lb)
            }
        }
        GLP_UP => {
            if a1.unwrap().val > 0.0 {
                (GLP_UP, row.ub)
            } else {
                (GLP_LO, row.ub)
            }
        }
        _ => unreachable!(),
    };

    *x = a1.unwrap().col.j;
    *z = a2.unwrap().col.j;
    *a = -a2.unwrap().val / a1.unwrap().val;
    *b = rhs / a1.unwrap().val;
    type_
}

fn set_vb(csa: &mut Csa, type_: i32, x: i32, z: i32, a: f64, b: f64) {
    assert!(csa.p.get_col_type(x) != GLP_FX);
    assert!(csa.p.get_col_kind(x) != GLP_BV);
    assert!(csa.p.get_col_kind(z) == GLP_BV);
    assert!(a != 0.0);

    match type_ {
        GLP_LO => {
            csa.l[x as usize].z = z;
            csa.l[x as usize].a = a;
            csa.l[x as usize].b = b;
        }
        GLP_UP => {
            csa.u[x as usize].z = z;
            csa.u[x as usize].a = a;
            csa.u[x as usize].b = b;
        }
        _ => unreachable!(),
    }
}

fn obtain_vbs(csa: &mut Csa) {
    for i in 1..=csa.p.m {
        match csa.p.row[i as usize].type_ {
            GLP_FR => {}
            GLP_LO | GLP_UP => {
                let (mut x, mut z, mut a, mut b) = (0, 0, 0.0, 0.0);
                let type_ = check_vb(csa, i, &mut x, &mut z, &mut a, &mut b);
                if type_ != 0 {
                    set_vb(csa, type_, x, z, a, b);
                }
            }
            GLP_DB | GLP_FX => {
                let save = csa.p.row[i as usize].type_;
                csa.p.row[i as usize].type_ = GLP_LO;
                let (mut x, mut z, mut a, mut b) = (0, 0, 0.0, 0.0);
                let type_ = check_vb(csa, i, &mut x, &mut z, &mut a, &mut b);
                if type_ != 0 {
                    set_vb(csa, type_, x, z, a, b);
                }
                csa.p.row[i as usize].type_ = GLP_UP;
                let type_ = check_vb(csa, i, &mut x, &mut z, &mut a, &mut b);
                if type_ != 0 {
                    set_vb(csa, type_, x, z, a, b);
                }
                csa.p.row[i as usize].type_ = save;
            }
            _ => unreachable!(),
        }
    }
}

fn add_term(v: &mut FVS, j: i32, a: f64) {
    assert!(1 <= j && j <= v.n);
    assert!(a != 0.0);
    if v.vec[j as usize] == 0.0 {
        v.nnz += 1;
        assert!(v.nnz <= v.n);
        v.ind[v.nnz as usize] = j;
    }
    v.vec[j as usize] += a;
    if v.vec[j as usize].abs() < 1e-9 * (1.0 + a.abs()) {
        v.vec[j as usize] = DBL_MIN;
    }
}

fn build_ks(csa: &Csa, n: i32, ind: &mut [i32], a: &mut [f64], b: &mut f64, v: &mut FVS) -> i32 {
    let mut new_n = 0;
    for j in 1..=n {
        let k = ind[j as usize];
        if csa.p.get_col_kind(k) == GLP_BV {
            add_term(v, k, a[j as usize]);
        } else if a[j as usize] > 0.0 {
            if csa.l[k as usize].b == -DBL_MAX {
                new_n = -1;
                break;
            } else if csa.l[k as usize].z == 0 {
                *b -= a[j as usize] * csa.l[k as usize].b;
            } else {
                add_term(v, csa.l[k as usize].z, a[j as usize] * csa.l[k as usize].a);
                *b -= a[j as usize] * csa.l[k as usize].b;
            }
        } else {
            if csa.u[k as usize].b == DBL_MAX {
                new_n = -1;
                break;
            } else if csa.u[k as usize].z == 0 {
                *b -= a[j as usize] * csa.u[k as usize].b;
            } else {
                add_term(v, csa.u[k as usize].z, a[j as usize] * csa.u[k as usize].a);
                *b -= a[j as usize] * csa.u[k as usize].b;
            }
        }
    }
    if new_n == -1 {
        v.clear();
        return -1;
    }
    v.adjust(2.0 * DBL_MIN);
    new_n = v.nnz;
    for j in 1..=new_n {
        ind[j as usize] = v.ind[j as usize];
        a[j as usize] = v.vec[ind[j as usize] as usize];
    }
    v.clear();
    new_n
}

fn can_be_active(n: i32, a: &[f64], b: f64) -> bool {
    let mut s = 0.0;
    for j in 1..=n {
        if a[j as usize] > 0.0 {
            s += a[j as usize];
        }
    }
    s > b + 0.001 * (1.0 + b.abs())
}

fn is_sos_ineq(n: i32, a: &[f64], mut b: f64) -> bool {
    assert!(n >= 2);
    for j in 1..=n {
        if a[j as usize] < 0.0 {
            b -= a[j as usize];
        }
    }
    let mut p = 1;
    for j in 2..=n {
        if a[p as usize].abs() > a[j as usize].abs() {
            p = j;
        }
    }
    let mut q = 0;
    for j in 1..=n {
        if j != p {
            if q == 0 || a[q as usize].abs() > a[j as usize].abs() {
                q = j;
            }
        }
    }
    assert!(q != 0);
    a[p as usize].abs() + a[q as usize].abs() > b + 0.001 * (1.0 + b.abs())
}

fn process_ineq(csa: &mut Csa, n: i32, ind: &mut [i32], a: &mut [f64], b: f64, v: &mut FVS) {
    let mut b = b;
    let mut n = build_ks(csa, n, ind, a, &mut b, v);
    if n <= 1 {
        return;
    }
    if !can_be_active(n, a, b) {
        return;
    }
    if is_sos_ineq(n, a, b) {
        return;
    }
    let i = csa.set.add_rows(1);
    csa.set.set_mat_row(i, n, ind, a);
    csa.set.set_row_bnds(i, GLP_UP, b, b);
}

pub fn glp_cov_init(P: glp_prob) -> Option<GlpCov> {
    let mut csa = Csa {
        p: P.clone(),
        l: vec![Bnd { z: 0, a: 0.0, b: 0.0 }; (P.n + 1) as usize],
        u: vec![Bnd { z: 0, a: 0.0, b: 0.0 }; (P.n + 1) as usize],
        set: glp_prob::new(),
    };
    csa.set.add_cols(P.n);
    init_bounds(&mut csa);
    obtain_vbs(&mut csa);
    let mut ind = vec![0; (P.n + 1) as usize];
    let mut val = vec![0.0; (P.n + 1) as usize];
    let mut fvs = FVS::new(P.n);
    for i in 1..=P.m {
        match P.row[i as usize].type_ {
            GLP_FR => {}
            GLP_LO => {
                let len = P.get_mat_row(i, &mut ind, &mut val);
                let mut rhs = P.row[i as usize].lb;
                for k in 1..=len {
                    val[k as usize] = -val[k as usize];
                }
                rhs = -rhs;
                process_ineq(&mut csa, len, &mut ind, &mut val, rhs, &mut fvs);
            }
            GLP_UP => {
                let len = P.get_mat_row(i, &mut ind, &mut val);
                let rhs = P.row[i as usize].ub;
                process_ineq(&mut csa, len, &mut ind, &mut val, rhs, &mut fvs);
            }
            GLP_DB | GLP_FX => {
                let save = P.row[i as usize].type_;
                P.row[i as usize].type_ = GLP_LO;
                let len = P.get_mat_row(i, &mut ind, &mut val);
                let mut rhs = P.row[i as usize].lb;
                for k in 1..=len {
                    val[k as usize] = -val[k as usize];
                }
                rhs = -rhs;
                process_ineq(&mut csa, len, &mut ind, &mut val, rhs, &mut fvs);
                P.row[i as usize].type_ = GLP_UP;
                let len = P.get_mat_row(i, &mut ind, &mut val);
                let rhs = P.row[i as usize].ub;
                process_ineq(&mut csa, len, &mut ind, &mut val, rhs, &mut fvs);
                P.row[i as usize].type_ = save;
            }
            _ => unreachable!(),
        }
    }
    if csa.set.m == 0 {
        None
    } else {
        Some(GlpCov {
            n: P.n,
            set: csa.set,
        })
    }
}

fn solve_ks(n: i32, a: &[i32], b: i32, c: &[i32], x: &mut [char]) -> i32 {
    if n <= 16 {
        ks_mt1(n, a, b, c, x)
    } else {
        ks_greedy(n, a, b, c, x)
    }
}

fn simple_cover(n: i32, a: &[f64], b: f64, x: &[f64], z: &mut [char]) -> f64 {
    assert!(n >= 3);
    let mut aa = vec![0; (n + 1) as usize];
    let mut cc = vec![0; (n + 1) as usize];
    let (mut max_aj, mut min_aj) = (0.0, DBL_MAX);
    for j in 1..=n {
        assert!(a[j as usize] > 0.0);
        if max_aj < a[j as usize] {
            max_aj = a[j as usize];
        }
        if min_aj > a[j as usize] {
            min_aj = a[j as usize];
        }
    }
    let mut s = 0.0;
    for j in 1..=n {
        s += a[j as usize];
        aa[j as usize] = (a[j as usize] / max_aj * 1000.0).ceil() as i32;
    }
    let bb = ((s - b) / max_aj * 1000.0).floor() as i32 - 1;
    for j in 1..=n {
        assert!(0.0 <= x[j as usize] && x[j as usize] <= 1.0);
        cc[j as usize] = ((1.0 - x[j as usize]) * 1000.0).floor() as i32;
    }
    if solve_ks(n, &aa, bb, &cc, z) == i32::MIN {
        return DBL_MAX;
    }
    for j in 1..=n {
        z[j as usize] ^= 1;
    }
    s = 0.0;
    for j in 1..=n {
        if z[j as usize] != 0 {
            s += a[j as usize];
        }
    }
    let eps = 0.01 * if min_aj >= 1.0 { min_aj } else { 1.0 };
    if !(s >= b + eps) {
        return DBL_MAX;
    }
    s = 0.0;
    for j in 1..=n {
        if z[j as usize] != 0 {
            s += 1.0 - x[j as usize];
        }
    }
    s
}

pub fn glp_cov_gen1(P: &glp_prob, cov: &GlpCov, pool: &mut glp_prob) {
    assert!(P.n == cov.n && P.n == cov.set.n);
    assert!(P.get_status() == GLP_OPT);
    let mut ind = vec![0; (P.n + 1) as usize];
    let mut val = vec![0.0; (P.n + 1) as usize];
    let mut x = vec![0.0; (P.n + 1) as usize];
    let mut z = vec![0; (P.n + 1) as usize];
    for i in 1..=cov.set.m {
        let len = cov.set.get_mat_row(i, &mut ind, &mut val);
        let mut rhs = cov.set.row[i as usize].ub;
        assert!(rhs != DBL_MAX);
        let mut new_len = 0;
        for k in 1..=len {
            if P.get_col_type(ind[k as usize]) == GLP_FX {
                rhs -= val[k as usize] * P.get_col_prim(ind[k as usize]);
            } else {
                new_len += 1;
                ind[new_len as usize] = ind[k as usize];
                val[new_len as usize] = val[k as usize];
            }
        }
        let len = new_len;
        if len <= 2 {
            continue;
        }
        for k in 1..=len {
            assert!(P.get_col_kind(ind[k as usize]) == GLP_BV);
            x[k as usize] = P.get_col_prim(ind[k as usize]);
            if x[k as usize] < 0.00001 {
                x[k as usize] = 0.0;
            } else if x[k as usize] > 0.99999 {
                x[k as usize] = 1.0;
            }
            if val[k as usize] < 0.0 {
                ind[k as usize] = -ind[k as usize];
                val[k as usize] = -val