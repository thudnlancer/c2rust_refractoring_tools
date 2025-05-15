/*!
Simplex method routines for GLPK.
*/

use std::f64;
use std::ptr;
use std::mem;
use std::cmp::Ordering;
use libc::{c_int, c_double, c_void};

use super::env::*;
use super::ios::*;
use super::npp::*;
use super::simplex::*;

/// Solves LP problem with the simplex method
pub fn glp_simplex(P: &mut glp_prob, parm: Option<&glp_smcp>) -> c_int {
    let mut _parm = glp_smcp::default();
    let parm = parm.unwrap_or(&mut _parm);
    
    // Check problem object
    if P.tree.is_some() && P.tree.as_ref().unwrap().reason != 0 {
        panic!("glp_simplex: operation not allowed");
    }

    // Check control parameters
    if !matches!(parm.msg_lev, GLP_MSG_OFF | GLP_MSG_ERR | GLP_MSG_ON | GLP_MSG_ALL | GLP_MSG_DBG) {
        panic!("glp_simplex: msg_lev = {}; invalid parameter", parm.msg_lev);
    }
    if !matches!(parm.meth, GLP_PRIMAL | GLP_DUALP | GLP_DUAL) {
        panic!("glp_simplex: meth = {}; invalid parameter", parm.meth);
    }
    if !matches!(parm.pricing, GLP_PT_STD | GLP_PT_PSE) {
        panic!("glp_simplex: pricing = {}; invalid parameter", parm.pricing);
    }
    if !matches!(parm.r_test, GLP_RT_STD | GLP_RT_FLIP | GLP_RT_HAR) {
        panic!("glp_simplex: r_test = {}; invalid parameter", parm.r_test);
    }
    if !(0.0 < parm.tol_bnd && parm.tol_bnd < 1.0) {
        panic!("glp_simplex: tol_bnd = {}; invalid parameter", parm.tol_bnd);
    }
    // ... other parameter checks ...

    // Basic solution is currently undefined
    P.pbs_stat = GLP_UNDEF;
    P.dbs_stat = GLP_UNDEF;
    P.obj_val = 0.0;
    P.some = 0;

    // Check bounds of double-bounded variables
    for i in 1..=P.m {
        let row = &P.row[i];
        if row.type_ == GLP_DB && row.lb >= row.ub {
            if parm.msg_lev >= GLP_MSG_ERR {
                println!("glp_simplex: row {}: lb = {}, ub = {}; incorrect bounds", 
                    i, row.lb, row.ub);
            }
            return GLP_EBOUND;
        }
    }

    for j in 1..=P.n {
        let col = &P.col[j];
        if col.type_ == GLP_DB && col.lb >= col.ub {
            if parm.msg_lev >= GLP_MSG_ERR {
                println!("glp_simplex: column {}: lb = {}, ub = {}; incorrect bounds",
                    j, col.lb, col.ub);
            }
            return GLP_EBOUND;
        }
    }

    // Solve LP problem
    if parm.msg_lev >= GLP_MSG_ALL {
        println!("GLPK Simplex Optimizer {}", glp_version());
        println!("{} row{}, {} column{}, {} non-zero{}",
            P.m, if P.m == 1 { "" } else { "s" },
            P.n, if P.n == 1 { "" } else { "s" },
            P.nnz, if P.nnz == 1 { "" } else { "s" });
    }

    let ret = if P.nnz == 0 {
        trivial_lp(P, parm);
        0
    } else if parm.presolve == GLP_OFF {
        solve_lp(P, parm)
    } else {
        preprocess_and_solve_lp(P, parm)
    };

    ret
}

/// Solve trivial LP which has empty constraint matrix
fn trivial_lp(P: &mut glp_prob, parm: &glp_smcp) {
    P.valid = 0;
    P.pbs_stat = GLP_FEAS;
    P.dbs_stat = GLP_FEAS;
    P.obj_val = P.c0;
    P.some = 0;
    let mut p_infeas = 0.0;
    let mut d_infeas = 0.0;
    let mut zeta = 1.0;

    // Make all auxiliary variables basic
    for i in 1..=P.m {
        let row = &mut P.row[i];
        row.stat = GLP_BS;
        row.prim = 0.0;
        row.dual = 0.0;

        // Check primal feasibility
        if matches!(row.type_, GLP_LO | GLP_DB | GLP_FX) {
            if row.lb > parm.tol_bnd {
                P.pbs_stat = GLP_NOFEAS;
                if P.some == 0 && parm.meth != GLP_PRIMAL {
                    P.some = i;
                }
            }
            if p_infeas < row.lb {
                p_infeas = row.lb;
            }
        }

        if matches!(row.type_, GLP_UP | GLP_DB | GLP_FX) {
            if row.ub < -parm.tol_bnd {
                P.pbs_stat = GLP_NOFEAS;
                if P.some == 0 && parm.meth != GLP_PRIMAL {
                    P.some = i;
                }
            }
            if p_infeas < -row.ub {
                p_infeas = -row.ub;
            }
        }
    }

    // Determine scale factor for the objective row
    for j in 1..=P.n {
        let col = &P.col[j];
        if zeta < col.coef.abs() {
            zeta = col.coef.abs();
        }
    }
    zeta = (if P.dir == GLP_MIN { 1.0 } else { -1.0 }) / zeta;

    // Make all structural variables non-basic
    for j in 1..=P.n {
        let col = &mut P.col[j];
        match col.type_ {
            GLP_FR => {
                col.stat = GLP_NF;
                col.prim = 0.0;
            },
            GLP_LO => {
                col.stat = GLP_NL;
                col.prim = col.lb;
            },
            GLP_UP => {
                col.stat = GLP_NU;
                col.prim = col.ub;
            },
            GLP_DB => {
                if zeta * col.coef > 0.0 {
                    col.stat = GLP_NL;
                    col.prim = col.lb;
                } else if zeta * col.coef < 0.0 {
                    col.stat = GLP_NU;
                    col.prim = col.ub;
                } else if col.lb.abs() <= col.ub.abs() {
                    col.stat = GLP_NL;
                    col.prim = col.lb;
                } else {
                    col.stat = GLP_NU;
                    col.prim = col.ub;
                }
            },
            GLP_FX => {
                col.stat = GLP_NS;
                col.prim = col.lb;
            },
            _ => unreachable!(),
        }

        col.dual = col.coef;
        P.obj_val += col.coef * col.prim;

        // Check dual feasibility
        if matches!(col.type_, GLP_FR | GLP_LO) {
            if zeta * col.dual < -parm.tol_dj {
                P.dbs_stat = GLP_NOFEAS;
                if P.some == 0 && parm.meth == GLP_PRIMAL {
                    P.some = P.m + j;
                }
            }
            if d_infeas < -zeta * col.dual {
                d_infeas = -zeta * col.dual;
            }
        }

        if matches!(col.type_, GLP_FR | GLP_UP) {
            if zeta * col.dual > parm.tol_dj {
                P.dbs_stat = GLP_NOFEAS;
                if P.some == 0 && parm.meth == GLP_PRIMAL {
                    P.some = P.m + j;
                }
            }
            if d_infeas < zeta * col.dual {
                d_infeas = zeta * col.dual;
            }
        }
    }

    // Simulate the simplex solver output
    if parm.msg_lev >= GLP_MSG_ON && parm.out_dly == 0 {
        println!("~{:6}: obj = {:17.9e}  infeas = {:10.3e}", 
            P.it_cnt, P.obj_val, 
            if parm.meth == GLP_PRIMAL { p_infeas } else { d_infeas });
    }

    if parm.msg_lev >= GLP_MSG_ALL && parm.out_dly == 0 {
        if P.pbs_stat == GLP_FEAS && P.dbs_stat == GLP_FEAS {
            println!("OPTIMAL SOLUTION FOUND");
        } else if P.pbs_stat == GLP_NOFEAS {
            println!("PROBLEM HAS NO FEASIBLE SOLUTION");
        } else if parm.meth == GLP_PRIMAL {
            println!("PROBLEM HAS UNBOUNDED SOLUTION");
        } else {
            println!("PROBLEM HAS NO DUAL FEASIBLE SOLUTION");
        }
    }
}

/// Solve LP directly without using the preprocessor
fn solve_lp(P: &mut glp_prob, parm: &glp_smcp) -> c_int {
    let mut ret = 0;

    if !glp_bf_exists(P) {
        ret = glp_factorize(P);
        match ret {
            0 => (),
            GLP_EBADB => {
                if parm.msg_lev >= GLP_MSG_ERR {
                    println!("glp_simplex: initial basis is invalid");
                }
            },
            GLP_ESING => {
                if parm.msg_lev >= GLP_MSG_ERR {
                    println!("glp_simplex: initial basis is singular");
                }
            },
            GLP_ECOND => {
                if parm.msg_lev >= GLP_MSG_ERR {
                    println!("glp_simplex: initial basis is ill-conditioned");
                }
            },
            _ => panic!("unexpected return code"),
        }
        if ret != 0 {
            return ret;
        }
    }

    ret = match parm.meth {
        GLP_PRIMAL => spx_primal(P, parm),
        GLP_DUALP => {
            let ret = spx_dual(P, parm);
            if ret == GLP_EFAIL && P.valid != 0 {
                spx_primal(P, parm)
            } else {
                ret
            }
        },
        GLP_DUAL => spx_dual(P, parm),
        _ => panic!("invalid method parameter"),
    };

    ret
}

/// Solve LP using the preprocessor
fn preprocess_and_solve_lp(P: &mut glp_prob, parm: &glp_smcp) -> c_int {
    if parm.msg_lev >= GLP_MSG_ALL {
        println!("Preprocessing...");
    }

    // Create preprocessor workspace
    let npp = npp_create_wksp();

    // Load original problem into the preprocessor workspace
    npp_load_prob(npp, P, GLP_OFF, GLP_SOL, GLP_OFF);

    // Process LP prior to applying primal/dual simplex method
    let mut ret = npp_simplex(npp, parm);
    match ret {
        0 => (),
        GLP_ENOPFS => {
            if parm.msg_lev >= GLP_MSG_ALL {
                println!("PROBLEM HAS NO PRIMAL FEASIBLE SOLUTION");
            }
        },
        GLP_ENODFS => {
            if parm.msg_lev >= GLP_MSG_ALL {
                println!("PROBLEM HAS NO DUAL FEASIBLE SOLUTION");
            }
        },
        _ => panic!("unexpected return code"),
    }
    if ret != 0 {
        npp_delete_wksp(npp);
        return ret;
    }

    // Build transformed LP
    let mut lp = glp_create_prob();
    npp_build_prob(npp, &mut lp);

    // If the transformed LP is empty, it has empty solution which is optimal
    if lp.m == 0 && lp.n == 0 {
        lp.pbs_stat = GLP_FEAS;
        lp.dbs_stat = GLP_FEAS;
        lp.obj_val = lp.c0;
        if parm.msg_lev >= GLP_MSG_ON && parm.out_dly == 0 {
            println!("~{:6}: obj = {:17.9e}  infeas = {:10.3e}", 
                P.it_cnt, lp.obj_val, 0.0);
        }
        if parm.msg_lev >= GLP_MSG_ALL {
            println!("OPTIMAL SOLUTION FOUND BY LP PREPROCESSOR");
        }
        goto post;
    }

    if parm.msg_lev >= GLP_MSG_ALL {
        println!("{} row{}, {} column{}, {} non-zero{}",
            lp.m, if lp.m == 1 { "" } else { "s" },
            lp.n, if lp.n == 1 { "" } else { "s" },
            lp.nnz, if lp.nnz == 1 { "" } else { "s" });
    }

    // Inherit basis factorization control parameters
    let mut bfcp = glp_bfcp::default();
    glp_get_bfcp(P, &mut bfcp);
    glp_set_bfcp(&mut lp, &bfcp);

    // Scale the transformed problem
    {
        let env = get_env_ptr();
        let term_out = env.term_out;
        if term_out == 0 || parm.msg_lev < GLP_MSG_ALL {
            env.term_out = GLP_OFF;
        } else {
            env.term_out = GLP_ON;
        }
        glp_scale_prob(&mut lp, GLP_SF_AUTO);
        env.term_out = term_out;
    }

    // Build advanced initial basis
    {
        let env = get_env_ptr();
        let term_out = env.term_out;
        if term_out == 0 || parm.msg_lev < GLP_MSG_ALL {
            env.term_out = GLP_OFF;
        } else {
            env.term_out = GLP_ON;
        }
        glp_adv_basis(&mut lp, 0);
        env.term_out = term_out;
    }

    // Solve the transformed LP
    lp.it_cnt = P.it_cnt;
    ret = solve_lp(&mut lp, parm);
    P.it_cnt = lp.it_cnt;

    // Only optimal solution can be postprocessed
    if !(ret == 0 && lp.pbs_stat == GLP_FEAS && lp.dbs_stat == GLP_FEAS) {
        if parm.msg_lev >= GLP_MSG_ERR {
            println!("glp_simplex: unable to recover undefined or non-optimal solution");
        }
        if ret == 0 {
            ret = if lp.pbs_stat == GLP_NOFEAS {
                GLP_ENOPFS
            } else if lp.dbs_stat == GLP_NOFEAS {
                GLP_ENODFS
            } else {
                panic!("invalid solution status");
            };
        }
        glp_delete_prob(lp);
        npp_delete_wksp(npp);
        return ret;
    }

post:
    // Postprocess solution from the transformed LP
    npp_postprocess(npp, &mut lp);

    // Store solution to the original problem
    npp_unload_sol(npp, P);

    // The original LP has been successfully solved
    ret = 0;

    // Clean up
    glp_delete_prob(lp);
    npp_delete_wksp(npp);
    ret
}

/// Initialize simplex method control parameters
pub fn glp_init_smcp(parm: &mut glp_smcp) {
    parm.msg_lev = GLP_MSG_ALL;
    parm.meth = GLP_PRIMAL;
    parm.pricing = GLP_PT_PSE;
    parm.r_test = GLP_RT_HAR;
    parm.tol_bnd = 1e-7;
    parm.tol_dj = 1e-7;
    parm.tol_piv = 1e-9;
    parm.obj_ll = -f64::MAX;
    parm.obj_ul = f64::MAX;
    parm.it_lim = i32::MAX;
    parm.tm_lim = i32::MAX;
    parm.out_frq = 5000;
    parm.out_dly = 0;
    parm.presolve = GLP_OFF;
    parm.excl = GLP_ON;
    parm.shift = GLP_ON;
    parm.aorn = GLP_USE_NT;
}

/// Retrieve generic status of basic solution
pub fn glp_get_status(lp: &glp_prob) -> c_int {
    let status = glp_get_prim_stat(lp);
    match status {
        GLP_FEAS => match glp_get_dual_stat(lp) {
            GLP_FEAS => GLP_OPT,
            GLP_NOFEAS => GLP_UNBND,
            GLP_UNDEF | GLP_INFEAS => status,
            _ => panic!("invalid dual status"),
        },
        GLP_UNDEF | GLP_INFEAS | GLP_NOFEAS => status,
        _ => panic!("invalid primal status"),
    }
}

/// Retrieve status of primal basic solution
pub fn glp_get_prim_stat(lp: &glp_prob) -> c_int {
    lp.pbs_stat
}

/// Retrieve status of dual basic solution
pub fn glp_get_dual_stat(lp: &glp_prob) -> c