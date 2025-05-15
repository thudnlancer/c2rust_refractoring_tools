/* glpscl.rs (problem scaling routines) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000-2013 Free Software Foundation, Inc.
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
use crate::{env::glp_prob, misc::round2n};

/***********************************************************************
*  min_row_aij - determine minimal |a[i,j]| in i-th row
*
*  This routine returns minimal magnitude of (non-zero) constraint
*  coefficients in i-th row of the constraint matrix.
*
*  If the parameter scaled is zero, the original constraint matrix A is
*  assumed. Otherwise, the scaled constraint matrix R*A*S is assumed.
*
*  If i-th row of the matrix is empty, the routine returns 1. */

fn min_row_aij(lp: &glp_prob, i: i32, scaled: bool) -> f64 {
    assert!(1 <= i && i <= lp.m);
    let mut min_aij = 1.0;
    let mut aij = lp.row[i as usize].ptr;
    while !aij.is_null() {
        let current = unsafe { &*aij };
        let mut temp = current.val.abs();
        if scaled {
            temp *= current.row.rii * current.col.sjj;
        }
        if current.r_prev.is_null() || min_aij > temp {
            min_aij = temp;
        }
        aij = current.r_next;
    }
    min_aij
}

/***********************************************************************
*  max_row_aij - determine maximal |a[i,j]| in i-th row
*
*  This routine returns maximal magnitude of (non-zero) constraint
*  coefficients in i-th row of the constraint matrix.
*
*  If the parameter scaled is zero, the original constraint matrix A is
*  assumed. Otherwise, the scaled constraint matrix R*A*S is assumed.
*
*  If i-th row of the matrix is empty, the routine returns 1. */

fn max_row_aij(lp: &glp_prob, i: i32, scaled: bool) -> f64 {
    assert!(1 <= i && i <= lp.m);
    let mut max_aij = 1.0;
    let mut aij = lp.row[i as usize].ptr;
    while !aij.is_null() {
        let current = unsafe { &*aij };
        let mut temp = current.val.abs();
        if scaled {
            temp *= current.row.rii * current.col.sjj;
        }
        if current.r_prev.is_null() || max_aij < temp {
            max_aij = temp;
        }
        aij = current.r_next;
    }
    max_aij
}

/***********************************************************************
*  min_col_aij - determine minimal |a[i,j]| in j-th column
*
*  This routine returns minimal magnitude of (non-zero) constraint
*  coefficients in j-th column of the constraint matrix.
*
*  If the parameter scaled is zero, the original constraint matrix A is
*  assumed. Otherwise, the scaled constraint matrix R*A*S is assumed.
*
*  If j-th column of the matrix is empty, the routine returns 1. */

fn min_col_aij(lp: &glp_prob, j: i32, scaled: bool) -> f64 {
    assert!(1 <= j && j <= lp.n);
    let mut min_aij = 1.0;
    let mut aij = lp.col[j as usize].ptr;
    while !aij.is_null() {
        let current = unsafe { &*aij };
        let mut temp = current.val.abs();
        if scaled {
            temp *= current.row.rii * current.col.sjj;
        }
        if current.c_prev.is_null() || min_aij > temp {
            min_aij = temp;
        }
        aij = current.c_next;
    }
    min_aij
}

/***********************************************************************
*  max_col_aij - determine maximal |a[i,j]| in j-th column
*
*  This routine returns maximal magnitude of (non-zero) constraint
*  coefficients in j-th column of the constraint matrix.
*
*  If the parameter scaled is zero, the original constraint matrix A is
*  assumed. Otherwise, the scaled constraint matrix R*A*S is assumed.
*
*  If j-th column of the matrix is empty, the routine returns 1. */

fn max_col_aij(lp: &glp_prob, j: i32, scaled: bool) -> f64 {
    assert!(1 <= j && j <= lp.n);
    let mut max_aij = 1.0;
    let mut aij = lp.col[j as usize].ptr;
    while !aij.is_null() {
        let current = unsafe { &*aij };
        let mut temp = current.val.abs();
        if scaled {
            temp *= current.row.rii * current.col.sjj;
        }
        if current.c_prev.is_null() || max_aij < temp {
            max_aij = temp;
        }
        aij = current.c_next;
    }
    max_aij
}

/***********************************************************************
*  min_mat_aij - determine minimal |a[i,j]| in constraint matrix
*
*  This routine returns minimal magnitude of (non-zero) constraint
*  coefficients in the constraint matrix.
*
*  If the parameter scaled is zero, the original constraint matrix A is
*  assumed. Otherwise, the scaled constraint matrix R*A*S is assumed.
*
*  If the matrix is empty, the routine returns 1. */

fn min_mat_aij(lp: &glp_prob, scaled: bool) -> f64 {
    let mut min_aij = 1.0;
    for i in 1..=lp.m {
        let temp = min_row_aij(lp, i, scaled);
        if i == 1 || min_aij > temp {
            min_aij = temp;
        }
    }
    min_aij
}

/***********************************************************************
*  max_mat_aij - determine maximal |a[i,j]| in constraint matrix
*
*  This routine returns maximal magnitude of (non-zero) constraint
*  coefficients in the constraint matrix.
*
*  If the parameter scaled is zero, the original constraint matrix A is
*  assumed. Otherwise, the scaled constraint matrix R*A*S is assumed.
*
*  If the matrix is empty, the routine returns 1. */

fn max_mat_aij(lp: &glp_prob, scaled: bool) -> f64 {
    let mut max_aij = 1.0;
    for i in 1..=lp.m {
        let temp = max_row_aij(lp, i, scaled);
        if i == 1 || max_aij < temp {
            max_aij = temp;
        }
    }
    max_aij
}

/***********************************************************************
*  eq_scaling - perform equilibration scaling
*
*  This routine performs equilibration scaling of rows and columns of
*  the constraint matrix.
*
*  If the parameter flag is zero, the routine scales rows at first and
*  then columns. Otherwise, the routine scales columns and then rows.
*
*  Rows are scaled as follows:
*
*                         n
*     a'[i,j] = a[i,j] / max |a[i,j]|,  i = 1,...,m.
*                        j=1
*
*  This makes the infinity (maximum) norm of each row of the matrix
*  equal to 1.
*
*  Columns are scaled as follows:
*
*                         m
*     a'[i,j] = a[i,j] / max |a[i,j]|,  j = 1,...,n.
*                        i=1
*
*  This makes the infinity (maximum) norm of each column of the matrix
*  equal to 1. */

fn eq_scaling(lp: &mut glp_prob, flag: bool) {
    for pass in 0..=1 {
        if pass == (flag as i32) {
            // scale rows
            for i in 1..=lp.m {
                let temp = max_row_aij(lp, i, true);
                lp.set_rii(i, lp.get_rii(i) / temp);
            }
        } else {
            // scale columns
            for j in 1..=lp.n {
                let temp = max_col_aij(lp, j, true);
                lp.set_sjj(j, lp.get_sjj(j) / temp);
            }
        }
    }
}

/***********************************************************************
*  gm_scaling - perform geometric mean scaling
*
*  This routine performs geometric mean scaling of rows and columns of
*  the constraint matrix.
*
*  If the parameter flag is zero, the routine scales rows at first and
*  then columns. Otherwise, the routine scales columns and then rows.
*
*  Rows are scaled as follows:
*
*     a'[i,j] = a[i,j] / sqrt(alfa[i] * beta[i]),  i = 1,...,m,
*
*  where:
*                n                        n
*     alfa[i] = min |a[i,j]|,  beta[i] = max |a[i,j]|.
*               j=1                      j=1
*
*  Columns are scaled as follows:
*
*     a'[i,j] = a[i,j] / sqrt(alfa[j] * beta[j]),  j = 1,...,n,
*
*  where:
*                m                        m
*     alfa[j] = min |a[i,j]|,  beta[j] = max |a[i,j]|.
*               i=1                      i=1
*/

fn gm_scaling(lp: &mut glp_prob, flag: bool) {
    for pass in 0..=1 {
        if pass == (flag as i32) {
            // scale rows
            for i in 1..=lp.m {
                let temp = min_row_aij(lp, i, true) * max_row_aij(lp, i, true);
                lp.set_rii(i, lp.get_rii(i) / temp.sqrt());
            }
        } else {
            // scale columns
            for j in 1..=lp.n {
                let temp = min_col_aij(lp, j, true) * max_col_aij(lp, j, true);
                lp.set_sjj(j, lp.get_sjj(j) / temp.sqrt());
            }
        }
    }
}

/***********************************************************************
*  max_row_ratio - determine worst scaling "quality" for rows
*
*  This routine returns the worst scaling "quality" for rows of the
*  currently scaled constraint matrix:
*
*              m
*     ratio = max ratio[i],
*             i=1
*  where:
*                 n              n
*     ratio[i] = max |a[i,j]| / min |a[i,j]|,  1 <= i <= m,
*                j=1            j=1
*
*  is the scaling "quality" of i-th row. */

fn max_row_ratio(lp: &glp_prob) -> f64 {
    let mut ratio = 1.0;
    for i in 1..=lp.m {
        let temp = max_row_aij(lp, i, true) / min_row_aij(lp, i, true);
        if i == 1 || ratio < temp {
            ratio = temp;
        }
    }
    ratio
}

/***********************************************************************
*  max_col_ratio - determine worst scaling "quality" for columns
*
*  This routine returns the worst scaling "quality" for columns of the
*  currently scaled constraint matrix:
*
*              n
*     ratio = max ratio[j],
*             j=1
*  where:
*                 m              m
*     ratio[j] = max |a[i,j]| / min |a[i,j]|,  1 <= j <= n,
*                i=1            i=1
*
*  is the scaling "quality" of j-th column. */

fn max_col_ratio(lp: &glp_prob) -> f64 {
    let mut ratio = 1.0;
    for j in 1..=lp.n {
        let temp = max_col_aij(lp, j, true) / min_col_aij(lp, j, true);
        if j == 1 || ratio < temp {
            ratio = temp;
        }
    }
    ratio
}

/***********************************************************************
*  gm_iterate - perform iterative geometric mean scaling
*
*  This routine performs iterative geometric mean scaling of rows and
*  columns of the constraint matrix.
*
*  The parameter it_max specifies the maximal number of iterations.
*  Recommended value of it_max is 15.
*
*  The parameter tau specifies a minimal improvement of the scaling
*  "quality" on each iteration, 0 < tau < 1. It means than the scaling
*  process continues while the following condition is satisfied:
*
*     ratio[k] <= tau * ratio[k-1],
*
*  where ratio = max |a[i,j]| / min |a[i,j]| is the scaling "quality"
*  to be minimized, k is the iteration number. Recommended value of tau
*  is 0.90. */

fn gm_iterate(lp: &mut glp_prob, it_max: i32, tau: f64) {
    let mut ratio = 0.0;
    let mut r_old;
    // if the scaling "quality" for rows is better than for columns,
    // the rows are scaled first; otherwise, the columns are scaled first
    let flag = max_row_ratio(lp) > max_col_ratio(lp);
    for k in 1..=it_max {
        // save the scaling "quality" from previous iteration
        r_old = ratio;
        // determine the current scaling "quality"
        ratio = max_mat_aij(lp, true) / min_mat_aij(lp, true);
        // if improvement is not enough, terminate scaling
        if k > 1 && ratio > tau * r_old {
            break;
        }
        // otherwise, perform another iteration
        gm_scaling(lp, flag);
    }
}

/***********************************************************************
*  scale_prob - scale problem data
*
*  The routine scale_prob performs automatic scaling of problem data
*  for the specified problem object. */

fn scale_prob(lp: &mut glp_prob, flags: i32) {
    const GLP_SF_GM: i32 = 0x01;
    const GLP_SF_EQ: i32 = 0x10;
    const GLP_SF_2N: i32 = 0x20;
    const GLP_SF_SKIP: i32 = 0x40;
    const GLP_SF_AUTO: i32 = 0x80;

    let fmt = "%s: min|aij| = %10.3e  max|aij| = %10.3e  ratio = %10.3e\n";
    println!("Scaling...");
    // cancel the current scaling effect
    lp.unscale_prob();
    // report original scaling "quality"
    let min_aij = min_mat_aij(lp, true);
    let max_aij = max_mat_aij(lp, true);
    let ratio = max_aij / min_aij;
    println!(fmt, " A", min_aij, max_aij, ratio);
    // check if the problem is well scaled
    if min_aij >= 0.10 && max_aij <= 10.0 {
        println!("Problem data seem to be well scaled");
        // skip scaling, if required
        if flags & GLP_SF_SKIP != 0 {
            return;
        }
    }
    // perform iterative geometric mean scaling, if required
    if flags & GLP_SF_GM != 0 {
        gm_iterate(lp, 15, 0.90);
        let min_aij = min_mat_aij(lp, true);
        let max_aij = max_mat_aij(lp, true);
        let ratio = max_aij / min_aij;
        println!(fmt, "GM", min_aij, max_aij, ratio);
    }
    // perform equilibration scaling, if required
    if flags & GLP_SF_EQ != 0 {
        eq_scaling(lp, max_row_ratio(lp) > max_col_ratio(lp));
        let min_aij = min_mat_aij(lp, true);
        let max_aij = max_mat_aij(lp, true);
        let ratio = max_aij / min_aij;
        println!(fmt, "EQ", min_aij, max_aij, ratio);
    }
    // round scale factors to nearest power of two, if required
    if flags & GLP_SF_2N != 0 {
        for i in 1..=lp.m {
            lp.set_rii(i, round2n(lp.get_rii(i)));
        }
        for j in 1..=lp.n {
            lp.set_sjj(j, round2n(lp.get_sjj(j)));
        }
        let min_aij = min_mat_aij(lp, true);
        let max_aij = max_mat_aij(lp, true);
        let ratio = max_aij / min_aij;
        println!(fmt, "2N", min_aij, max_aij, ratio);
    }
}

/***********************************************************************
*  glp_scale_prob - scale problem data
*
*  The routine glp_scale_prob performs automatic scaling of problem
*  data for the specified problem object.
*
*  The parameter flags specifies scaling options used by the routine.
*  Options can be combined with the bitwise OR operator and may be the
*  following:
*
*  GLP_SF_GM      perform geometric mean scaling;
*  GLP_SF_EQ      perform equilibration scaling;
*  GLP_SF_2N      round scale factors to nearest power of two;
*  GLP_SF_SKIP    skip scaling, if the problem is well scaled.
*
*  The parameter flags may be