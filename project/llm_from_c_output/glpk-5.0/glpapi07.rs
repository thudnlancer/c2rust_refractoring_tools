/* glpapi07.rs (exact simplex solver) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2007-2017 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::f64;
use rug::{Rational, ops::AssignRound};
use rug::float::Round;
use crate::draft::*;
use crate::glpssx::*;
use crate::misc::*;
use crate::prob::*;

/***********************************************************************
*  NAME
*
*  glp_exact - solve LP problem in exact arithmetic
*
*  SYNOPSIS
*
*  int glp_exact(glp_prob *lp, const glp_smcp *parm);
*
*  DESCRIPTION
*
*  The routine glp_exact is a tentative implementation of the primal
*  two-phase simplex method based on exact (rational) arithmetic. It is
*  similar to the routine glp_simplex, however, for all internal
*  computations it uses arithmetic of rational numbers, which is exact
*  in mathematical sense, i.e. free of round-off errors unlike floating
*  point arithmetic.
*
*  Note that the routine glp_exact uses inly two control parameters
*  passed in the structure glp_smcp, namely, it_lim and tm_lim.
*
*  RETURNS
*
*  0  The LP problem instance has been successfully solved. This code
*     does not necessarily mean that the solver has found optimal
*     solution. It only means that the solution process was successful.
*
*  GLP_EBADB
*     Unable to start the search, because the initial basis specified
*     in the problem object is invalid--the number of basic (auxiliary
*     and structural) variables is not the same as the number of rows in
*     the problem object.
*
*  GLP_ESING
*     Unable to start the search, because the basis matrix correspodning
*     to the initial basis is exactly singular.
*
*  GLP_EBOUND
*     Unable to start the search, because some double-bounded variables
*     have incorrect bounds.
*
*  GLP_EFAIL
*     The problem has no rows/columns.
*
*  GLP_EITLIM
*     The search was prematurely terminated, because the simplex
*     iteration limit has been exceeded.
*
*  GLP_ETMLIM
*     The search was prematurely terminated, because the time limit has
*     been exceeded. */

fn set_d_eps(x: &mut Rational, val: f64) {
    /* convert double val to rational x obtaining a more adequate
       fraction than provided by mpq_set_d due to allowing a small
       approximation error specified by a given relative tolerance;
       for example, mpq_set_d would give the following
       1/3 ~= 0.333333333333333314829616256247391... ->
           -> 6004799503160661/18014398509481984
       while this routine gives exactly 1/3 */
    let eps = 1e-9;
    assert!(val.is_finite());
    
    if val == val.floor() {
        // if val is integral, do not approximate
        x.assign(val);
        return;
    }
    
    let s = if val > 0.0 {
        1
    } else if val < 0.0 {
        -1
    } else {
        x.assign(0);
        return;
    };
    
    let f = val.abs().frexp();
    /* |val| = f * 2^n, where 0.5 <= f < 1.0 */
    let (p, q) = fp2rat(f, 0.1 * eps);
    /* f ~= p / q, where p and q are integers */
    
    let mut temp = Rational::new();
    x.assign(p);
    temp.assign(q);
    *x /= &temp;
    
    temp.assign(1);
    for _ in 1..=f.1.abs() {
        temp += &temp;
    }
    
    if f.1 > 0 {
        *x *= &temp;
    } else if f.1 < 0 {
        *x /= &temp;
    }
    
    if s < 0 {
        *x = -x.clone();
    }
    
    /* check that the desired tolerance has been attained */
    assert!((val - x.to_f64()).abs() <= eps * (1.0 + val.abs()));
}

fn load_data(ssx: &mut SSX, lp: &glp_prob) {
    /* load LP problem data into simplex solver workspace */
    let m = ssx.m;
    let n = ssx.n;
    let nnz = ssx.A_ptr[n+1]-1;
    assert_eq!(lp.m, m);
    assert_eq!(lp.n, n);
    assert_eq!(lp.nnz, nnz);
    
    /* types and bounds of rows and columns */
    for k in 1..=m+n {
        let (type_, lb, ub) = if k <= m {
            let row = &lp.row[k];
            (row.type_, row.lb, row.ub)
        } else {
            let col = &lp.col[k-m];
            (col.type_, col.lb, col.ub)
        };
        
        let type_ = match type_ {
            GLP_FR => SSX_FR,
            GLP_LO => SSX_LO,
            GLP_UP => SSX_UP,
            GLP_DB => SSX_DB,
            GLP_FX => SSX_FX,
            _ => panic!("invalid type"),
        };
        
        ssx.type_[k] = type_;
        set_d_eps(&mut ssx.lb[k], lb);
        set_d_eps(&mut ssx.ub[k], ub);
    }
    
    /* optimization direction */
    ssx.dir = match lp.dir {
        GLP_MIN => SSX_MIN,
        GLP_MAX => SSX_MAX,
        _ => panic!("invalid direction"),
    };
    
    /* objective coefficients */
    for k in 0..=m+n {
        let coef = if k == 0 {
            lp.c0
        } else if k <= m {
            0.0
        } else {
            lp.col[k-m].coef
        };
        set_d_eps(&mut ssx.coef[k], coef);
    }
    
    /* constraint coefficients */
    let mut ind = vec![0; 1+m];
    let mut val = vec![0.0; 1+m];
    let mut loc = 0;
    
    for j in 1..=n {
        ssx.A_ptr[j] = loc + 1;
        let len = glp_get_mat_col(lp, j, &mut ind, &mut val);
        
        for k in 1..=len {
            loc += 1;
            ssx.A_ind[loc] = ind[k];
            set_d_eps(&mut ssx.A_val[loc], val[k]);
        }
    }
    
    assert_eq!(loc, nnz);
}

fn load_basis(ssx: &mut SSX, lp: &glp_prob) -> bool {
    /* load current LP basis into simplex solver workspace */
    let m = ssx.m;
    let n = ssx.n;
    let type_ = &ssx.type_;
    let stat = &mut ssx.stat;
    let Q_row = &mut ssx.Q_row;
    let Q_col = &mut ssx.Q_col;
    
    assert_eq!(lp.m, m);
    assert_eq!(lp.n, n);
    
    /* statuses of rows and columns */
    for k in 1..=m+n {
        let stat_k = if k <= m {
            lp.row[k].stat
        } else {
            lp.col[k-m].stat
        };
        
        stat[k] = match stat_k {
            GLP_BS => {
                SSX_BS
            },
            GLP_NL => {
                assert!(type_[k] == SSX_LO || type_[k] == SSX_DB);
                SSX_NL
            },
            GLP_NU => {
                assert!(type_[k] == SSX_UP || type_[k] == SSX_DB);
                SSX_NU
            },
            GLP_NF => {
                assert!(type_[k] == SSX_FR);
                SSX_NF
            },
            GLP_NS => {
                assert!(type_[k] == SSX_FX);
                SSX_NS
            },
            _ => panic!("invalid status"),
        };
    }
    
    /* build permutation matix Q */
    let mut i = 0;
    let mut j = 0;
    
    for k in 1..=m+n {
        if stat[k] == SSX_BS {
            i += 1;
            if i > m {
                return true;
            }
            Q_row[k] = i;
            Q_col[i] = k;
        } else {
            j += 1;
            if j > n {
                return true;
            }
            Q_row[k] = m + j;
            Q_col[m + j] = k;
        }
    }
    
    assert_eq!(i, m);
    assert_eq!(j, n);
    false
}

pub fn glp_exact(lp: &mut glp_prob, parm: Option<&glp_smcp>) -> i32 {
    let _parm;
    let parm = match parm {
        Some(p) => p,
        None => {
            _parm = glp_smcp::default();
            &_parm
        }
    };
    
    /* check control parameters */
    match parm.msg_lev {
        GLP_MSG_OFF | GLP_MSG_ERR | GLP_MSG_ON | GLP_MSG_ALL | GLP_MSG_DBG => (),
        _ => panic!("glp_exact: msg_lev = {}; invalid parameter", parm.msg_lev),
    }
    
    if parm.it_lim < 0 {
        panic!("glp_exact: it_lim = {}; invalid parameter", parm.it_lim);
    }
    
    if parm.tm_lim < 0 {
        panic!("glp_exact: tm_lim = {}; invalid parameter", parm.tm_lim);
    }
    
    /* the problem must have at least one row and one column */
    if !(lp.m > 0 && lp.n > 0) {
        if parm.msg_lev >= GLP_MSG_ERR {
            println!("glp_exact: problem has no rows/columns");
        }
        return GLP_EFAIL;
    }
    
    /* basic solution is currently undefined */
    lp.pbs_stat = GLP_UNDEF;
    lp.dbs_stat = GLP_UNDEF;
    lp.obj_val = 0.0;
    lp.some = 0;
    
    /* check that all double-bounded variables have correct bounds */
    for k in 1..=lp.m + lp.n {
        let (type_, lb, ub) = if k <= lp.m {
            let row = &lp.row[k];
            (row.type_, row.lb, row.ub)
        } else {
            let col = &lp.col[k - lp.m];
            (col.type_, col.lb, col.ub)
        };
        
        if type_ == GLP_DB && lb >= ub {
            if parm.msg_lev >= GLP_MSG_ERR {
                println!("glp_exact: {} {} has invalid bounds",
                    if k <= lp.m { "row" } else { "column" },
                    if k <= lp.m { k } else { k - lp.m });
            }
            return GLP_EBOUND;
        }
    }
    
    /* create the simplex solver workspace */
    if parm.msg_lev >= GLP_MSG_ALL {
        println!("glp_exact: {} rows, {} columns, {} non-zeros",
            lp.m, lp.n, lp.nnz);
        #[cfg(feature = "gmp")]
        println!("GNU MP bignum library is being used");
        #[cfg(not(feature = "gmp"))]
        {
            println!("GLPK bignum module is being used");
            println!("(Consider installing GNU MP to attain a much better performance.)");
        }
    }
    
    let mut ssx = SSX::new(lp.m, lp.n, lp.nnz);
    
    /* load LP problem data into the workspace */
    load_data(&mut ssx, lp);
    
    /* load current LP basis into the workspace */
    if load_basis(&mut ssx, lp) {
        if parm.msg_lev >= GLP_MSG_ERR {
            println!("glp_exact: initial LP basis is invalid");
        }
        return GLP_EBADB;
    }
    
    ssx.msg_lev = parm.msg_lev;
    ssx.it_lim = parm.it_lim;
    ssx.it_cnt = lp.it_cnt;
    ssx.tm_lim = parm.tm_lim as f64 / 1000.0;
    ssx.out_frq = 5.0;
    ssx.tm_beg = xtime();
    ssx.tm_lag = 0.0;
    
    /* solve LP */
    let ret = ssx_driver(&mut ssx);
    lp.it_cnt = ssx.it_cnt;
    
    /* analyze the return code */
    let (ret, pst, dst) = match ret {
        0 => (0, GLP_FEAS, GLP_FEAS),        // optimal solution found
        1 => (0, GLP_NOFEAS, GLP_INFEAS),    // no feasible solution
        2 => {                                // unbounded solution
            assert!(1 <= ssx.q && ssx.q <= lp.n);
            lp.some = ssx.Q_col[lp.m + ssx.q];
            assert!(1 <= lp.some && lp.some <= lp.m + lp.n);
            (0, GLP_FEAS, GLP_NOFEAS)
        },
        3 => (GLP_EITLIM, GLP_INFEAS, GLP_INFEAS), // iteration limit (phase I)
        4 => (GLP_EITLIM, GLP_FEAS, GLP_INFEAS),   // iteration limit (phase II)
        5 => (GLP_ETMLIM, GLP_INFEAS, GLP_INFEAS), // time limit (phase I)
        6 => (GLP_ETMLIM, GLP_FEAS, GLP_INFEAS),   // time limit (phase II)
        7 => return GLP_ESING,                     // singular basis matrix
        _ => panic!("invalid return code"),
    };
    
    /* store final basic solution components into LP object */
    lp.pbs_stat = pst;
    lp.dbs_stat = dst;
    let mut sum = lp.c0;
    
    for k in 1..=lp.m + lp.n {
        let (stat, prim, dual) = if ssx.stat[k] == SSX_BS {
            let i = ssx.Q_row[k]; // x[k] = xB[i]
            assert!(1 <= i && i <= lp.m);
            (GLP_BS, ssx.bbar[i].to_f64(), 0.0)
        } else {
            let j = ssx.Q_row[k] - lp.m; // x[k] = xN[j]
            assert!(1 <= j && j <= lp.n);
            
            let (stat, prim) = match ssx.stat[k] {
                SSX_NF => (GLP_NF, 0.0),
                SSX_NL => (GLP_NL, ssx.lb[k].to_f64()),
                SSX_NU => (GLP_NU, ssx.ub[k].to_f64()),
                SSX_NS => (GLP_NS, ssx.lb[k].to_f64()),
                _ => panic!("invalid status"),
            };
            
            (stat, prim, ssx.cbar[j].to_f64())
        };
        
        if k <= lp.m {
            lp.row[k].stat = stat;
            lp.row[k].prim = prim;
            lp.row[k].dual = dual;
        } else {
            lp.col[k - lp.m].stat = stat;
            lp.col[k - lp.m].prim = prim;
            lp.col[k - lp.m].dual = dual;
            sum += lp.col[k - lp.m].coef * prim;
        }
    }
    
    lp.obj_val = sum;
    ret
}

/* eof */