use std::cmp::Ordering;
use std::f64::{DBL_EPSILON, INFINITY};
use std::ptr;

struct Ioscut {
    ptr: *mut Glpaij,
    type_: i32,
    lb: f64,
    ub: f64,
    name: Option<String>,
    klass: i32,
}

struct Glpaij {
    col: *mut Glpcol,
    val: f64,
    r_next: *mut Glpaij,
}

struct Glpcol {
    j: i32,
    prim: f64,
}

struct Glptree {
    curr: *mut Glpnode,
    local: *mut Iospool,
    n: i32,
    mip: *mut Glpprob,
}

struct Glpnode {
    level: i32,
}

struct Iospool {
    m: i32,
    row: Vec<*mut Ioscut>,
}

struct Glpprob {
    dir: i32,
    row: Vec<*mut Glprow>,
    col: Vec<*mut Glpcol>,
}

struct Glprow {
    origin: i32,
    klass: i32,
}

struct Info {
    cut: *mut Ioscut,
    flag: bool,
    eff: f64,
    deg: f64,
}

fn fcmp(arg1: &Info, arg2: &Info) -> Ordering {
    if arg1.deg == 0.0 && arg2.deg == 0.0 {
        arg2.eff.partial_cmp(&arg1.eff).unwrap_or(Ordering::Equal)
    } else {
        arg2.deg.partial_cmp(&arg1.deg).unwrap_or(Ordering::Equal)
    }
}

fn parallel(a: *mut Ioscut, b: *mut Ioscut, work: &mut [f64]) -> f64 {
    unsafe {
        let mut s = 0.0;
        let mut sa = 0.0;
        let mut sb = 0.0;

        let mut aij = (*a).ptr;
        while !aij.is_null() {
            let col = (*aij).col;
            work[(*col).j as usize] = (*aij).val;
            sa += (*aij).val * (*aij).val;
            aij = (*aij).r_next;
        }

        let mut aij = (*b).ptr;
        while !aij.is_null() {
            let col = (*aij).col;
            s += work[(*col).j as usize] * (*aij).val;
            sb += (*aij).val * (*aij).val;
            aij = (*aij).r_next;
        }

        let mut aij = (*a).ptr;
        while !aij.is_null() {
            let col = (*aij).col;
            work[(*col).j as usize] = 0.0;
            aij = (*aij).r_next;
        }

        let temp = sa.sqrt() * sb.sqrt();
        if temp < DBL_EPSILON * DBL_EPSILON {
            s / DBL_EPSILON
        } else {
            s / temp
        }
    }
}

fn ios_process_cuts(T: &mut Glptree) {
    unsafe {
        assert!(!T.curr.is_null());
        let pool = T.local;
        assert!(!pool.is_null());
        assert!((*pool).m > 0);

        let n = T.n as usize;
        let m = (*pool).m as usize;
        let mut info = vec![
            Info {
                cut: ptr::null_mut(),
                flag: false,
                eff: 0.0,
                deg: 0.0
            };
            m + 1
        ];
        let mut ind = vec![0; n + 1];
        let mut val = vec![0.0; n + 1];
        let mut work = vec![0.0; n + 1];

        for k in 1..=m {
            info[k].cut = (*pool).row[k];
        }

        for k in 1..=m {
            let cut = info[k].cut;
            let mut len = 0;
            let mut temp = 0.0;

            let mut aij = (*cut).ptr;
            while !aij.is_null() {
                len += 1;
                ind[len] = (*(*aij).col).j;
                val[len] = (*aij).val;
                temp += (*aij).val * (*aij).val;
                aij = (*aij).r_next;
            }

            if temp < DBL_EPSILON * DBL_EPSILON {
                temp = DBL_EPSILON;
            }

            let rhs = match (*cut).type_ {
                glp_lo => (*cut).lb,
                glp_up => (*cut).ub,
                _ => panic!("Invalid cut type"),
            };

            let (ret, dy, dz) = _glp_analyze_row(
                T.mip,
                len,
                &ind[1..=len],
                &val[1..=len],
                (*cut).type_,
                rhs,
                1e-9,
            );

            match ret {
                0 => {
                    info[k].eff = dy.abs() / temp.sqrt();
                    info[k].deg = if (*(*T).mip).dir == glp_min {
                        if dz < 0.0 { 0.0 } else { dz }
                    } else {
                        if dz > 0.0 { 0.0 } else { -dz }
                    };
                }
                1 => {
                    info[k].eff = 0.0;
                    info[k].deg = 0.0;
                }
                2 => {
                    info[k].eff = 1.0;
                    info[k].deg = INFINITY;
                }
                _ => panic!("Invalid return code"),
            }

            if info[k].deg < 0.01 {
                info[k].deg = 0.0;
            }
        }

        info[1..=m].sort_by(fcmp);

        let max_cuts = if (*(*T).curr).level == 0 { 90 } else { 10 };
        let max_cuts = if max_cuts > m { m } else { max_cuts };

        for k in 1..=max_cuts {
            if info[k].deg < 0.01 && info[k].eff < 0.01 {
                continue;
            }

            let mut skip = false;
            for kk in 1..k {
                if info[kk].flag {
                    if parallel(info[k].cut, info[kk].cut, &mut work) > 0.90 {
                        skip = true;
                        break;
                    }
                }
            }
            if skip {
                continue;
            }

            let cut = info[k].cut;
            info[k].flag = true;

            let i = glp_add_rows((*T).mip, 1);
            if let Some(name) = &(*cut).name {
                glp_set_row_name((*T).mip, i, name);
            }

            assert_eq!((*(*T).mip).row[i].origin, glp_rf_cut);
            (*(*T).mip).row[i].klass = (*cut).klass;

            let mut len = 0;
            let mut aij = (*cut).ptr;
            while !aij.is_null() {
                len += 1;
                ind[len] = (*(*aij).col).j;
                val[len] = (*aij).val;
                aij = (*aij).r_next;
            }

            glp_set_mat_row((*T).mip, i, len, &ind[1..=len], &val[1..=len]);

            let rhs = match (*cut).type_ {
                glp_lo => (*cut).lb,
                glp_up => (*cut).ub,
                _ => panic!("Invalid cut type"),
            };
            glp_set_row_bnds((*T).mip, i, (*cut).type_, rhs, rhs);
        }
    }
}

const glp_lo: i32 = 1;
const glp_up: i32 = 2;
const glp_min: i32 = 1;
const glp_max: i32 = 2;
const glp_rf_cut: i32 = 1;