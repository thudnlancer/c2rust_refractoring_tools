/* scf.rs (sparse updatable Schur-complement-based factorization) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2013-2014 Free Software Foundation, Inc.
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

use std::mem;
use std::ptr;
use std::slice;

use crate::btf::*;
use crate::ifu::*;
use crate::luf::*;
use crate::sva::*;

/***********************************************************************
*  The structure Scf describes sparse updatable factorization based on
*  Schur complement.
*
*  The SCF-factorization has the following format:
*
*     ( A   A1~ )     ( A0  A1 )       ( R0    ) ( S0  S )
*     (         ) = P (        ) Q = P (       ) (       ) Q,        (1)
*     ( A2~ A3~ )     ( A2  A3 )       ( R   I ) (     C )
*
*  where:
*
*  A is current (unsymmetric) square matrix (not stored);
*
*  A1~, A2~, A3~ are some additional matrices (not stored);
*
*  A0 is initial (unsymmetric) square matrix (not stored);
*
*  A1, A2, A3 are some additional matrices (not stored);
*
*  R0 and S0 are matrices that define factorization of the initial
*  matrix A0 = R0 * S0 (stored in an invertable form);
*
*  R is a matrix defined from R * S0 = A2, so R = A2 * inv(S0) (stored
*  in row-wise sparse format);
*
*  S is a matrix defined from R0 * S = A1, so S = inv(R0) * A1 (stored
*  in column-wise sparse format);
*
*  C is Schur complement (to matrix A0) defined from R * S + C = A3,
*  so C = A3 - R * S = A3 - A2 * inv(A0) * A1 (stored in an invertable
*  form).
*
*  P, Q are permutation matrices (stored in both row- and column-like
*  formats). */

pub struct Scf {
    pub n: i32,
    /* order of current matrix A */
    /*--------------------------------------------------------------*/
    /* initial matrix A0 = R0 * S0 of order n0 in invertable form */
    pub n0: i32,
    /* order of matrix A0 */
    pub type_: i32,
    /* type of factorization used:
     * 1 - LU-factorization (R0 = F0, S0 = V0)
     * 2 - BT-factorization (R0 = I, S0 = A0) */
    pub a0: ScfA0,
    /* factorization of matrix A0 */
    /*--------------------------------------------------------------*/
    /* augmented matrix (A0, A1; A2, A3) of order n0+nn */
    pub nn_max: i32,
    /* maximal number of additional rows and columns in the augmented
     * matrix (this limits the number of updates) */
    pub nn: i32,
    /* current number of additional rows and columns in the augmented
     * matrix, 0 <= nn <= nn_max */
    pub sva: *mut Sva,
    /* associated sparse vector area (SVA) used to store rows of
     * matrix R and columns of matrix S */
    /*--------------------------------------------------------------*/
    /* nn*n0-matrix R in row-wise format */
    pub rr_ref: i32,
    /* reference number of sparse vector in SVA, which is the first
     * row of matrix R */
    /*--------------------------------------------------------------*/
    /* n0*nn-matrix S in column-wise format */
    pub ss_ref: i32,
    /* reference number of sparse vector in SVA, which is the first
     * column of matrix S */
    /*--------------------------------------------------------------*/
    /* Schur complement C of order nn in invertable form */
    pub ifu: Ifu,
    /* IFU-factorization of matrix C */
    /*--------------------------------------------------------------*/
    /* permutation matrix P of order n0+nn */
    pub pp_ind: *mut i32, /* int pp_ind[1+n0+nn_max]; */
    /* pp_ind[i] = j means that P[i,j] = 1 */
    pub pp_inv: *mut i32, /* int pp_inv[1+n0+nn_max]; */
    /* pp_inv[j] = i means that P[i,j] = 1 */
    /*--------------------------------------------------------------*/
    /* permutation matrix Q of order n0+nn */
    pub qq_ind: *mut i32, /* int qq_ind[1+n0+nn_max]; */
    /* qq_ind[i] = j means that Q[i,j] = 1 */
    pub qq_inv: *mut i32, /* int qq_inv[1+n0+nn_max]; */
    /* qq_inv[j] = i means that Q[i,j] = 1 */
}

pub enum ScfA0 {
    Luf(*mut Luf),
    Btf(*mut Btf),
}

#[inline]
pub fn scf_swap_q_cols(scf: &mut Scf, j1: i32, j2: i32) {
    unsafe {
        let i1 = *scf.qq_inv.offset(j1 as isize);
        let i2 = *scf.qq_inv.offset(j2 as isize);
        *scf.qq_ind.offset(i1 as isize) = j2;
        *scf.qq_inv.offset(j2 as isize) = i1;
        *scf.qq_ind.offset(i2 as isize) = j1;
        *scf.qq_inv.offset(j1 as isize) = i2;
    }
}

pub fn scf_r0_solve(scf: &Scf, tr: i32, x: &mut [f64]) {
    match scf.type_ {
        1 => {
            /* A0 = F0 * V0, so R0 = F0 */
            unsafe {
                let luf = match scf.a0 {
                    ScfA0::Luf(p) => &*p,
                    _ => unreachable!(),
                };
                if tr == 0 {
                    luf_f_solve(luf, x);
                } else {
                    luf_ft_solve(luf, x);
                }
            }
        }
        2 => {
            /* A0 = I * A0, so R0 = I */
            /* nothing to do */
        }
        _ => unreachable!(),
    }
}

pub fn scf_s0_solve(
    scf: &Scf,
    tr: i32,
    x: &mut [f64],
    w1: &mut [f64],
    w2: &mut [f64],
    w3: &mut [f64],
) {
    let n0 = scf.n0;
    match scf.type_ {
        1 => {
            /* A0 = F0 * V0, so S0 = V0 */
            unsafe {
                let luf = match scf.a0 {
                    ScfA0::Luf(p) => &*p,
                    _ => unreachable!(),
                };
                if tr == 0 {
                    luf_v_solve(luf, x, w1);
                } else {
                    luf_vt_solve(luf, x, w1);
                }
            }
        }
        2 => {
            /* A0 = I * A0, so S0 = A0 */
            unsafe {
                let btf = match scf.a0 {
                    ScfA0::Btf(p) => &*p,
                    _ => unreachable!(),
                };
                if tr == 0 {
                    btf_a_solve(btf, x, w1, w2, w3);
                } else {
                    btf_at_solve(btf, x, w1, w2, w3);
                }
            }
        }
        _ => unreachable!(),
    }
    x[1..=n0 as usize].copy_from_slice(&w1[1..=n0 as usize]);
}

pub fn scf_r_prod(scf: &Scf, y: &mut [f64], a: f64, x: &[f64]) {
    let nn = scf.nn;
    unsafe {
        let sva = &*scf.sva;
        let sv_ind = sva.ind;
        let sv_val = sva.val;
        let rr_ref = scf.rr_ref;
        let rr_ptr = sva.ptr.offset(rr_ref - 1);
        let rr_len = sva.len.offset(rr_ref - 1);

        for i in 1..=nn {
            /* t := (i-th row of R) * x */
            let mut t = 0.0;
            let ptr = *rr_ptr.offset(i as isize);
            let end = ptr + *rr_len.offset(i as isize);
            for p in ptr..end {
                t += *sv_val.offset(p as isize) * x[*sv_ind.offset(p as isize) as usize];
            }
            /* y[i] := y[i] + alpha * t */
            y[i as usize] += a * t;
        }
    }
}

pub fn scf_rt_prod(scf: &Scf, y: &mut [f64], a: f64, x: &[f64]) {
    let nn = scf.nn;
    unsafe {
        let sva = &*scf.sva;
        let sv_ind = sva.ind;
        let sv_val = sva.val;
        let rr_ref = scf.rr_ref;
        let rr_ptr = sva.ptr.offset(rr_ref - 1);
        let rr_len = sva.len.offset(rr_ref - 1);

        for i in 1..=nn {
            if x[i as usize] == 0.0 {
                continue;
            }
            /* y := y + alpha * R'[i] * x[i] */
            let t = a * x[i as usize];
            let ptr = *rr_ptr.offset(i as isize);
            let end = ptr + *rr_len.offset(i as isize);
            for p in ptr..end {
                let idx = *sv_ind.offset(p as isize) as usize;
                y[idx] += *sv_val.offset(p as isize) * t;
            }
        }
    }
}

pub fn scf_s_prod(scf: &Scf, y: &mut [f64], a: f64, x: &[f64]) {
    let nn = scf.nn;
    unsafe {
        let sva = &*scf.sva;
        let sv_ind = sva.ind;
        let sv_val = sva.val;
        let ss_ref = scf.ss_ref;
        let ss_ptr = sva.ptr.offset(ss_ref - 1);
        let ss_len = sva.len.offset(ss_ref - 1);

        for j in 1..=nn {
            if x[j as usize] == 0.0 {
                continue;
            }
            /* y := y + alpha * S[j] * x[j] */
            let t = a * x[j as usize];
            let ptr = *ss_ptr.offset(j as isize);
            let end = ptr + *ss_len.offset(j as isize);
            for p in ptr..end {
                let idx = *sv_ind.offset(p as isize) as usize;
                y[idx] += *sv_val.offset(p as isize) * t;
            }
        }
    }
}

pub fn scf_st_prod(scf: &Scf, y: &mut [f64], a: f64, x: &[f64]) {
    let nn = scf.nn;
    unsafe {
        let sva = &*scf.sva;
        let sv_ind = sva.ind;
        let sv_val = sva.val;
        let ss_ref = scf.ss_ref;
        let ss_ptr = sva.ptr.offset(ss_ref - 1);
        let ss_len = sva.len.offset(ss_ref - 1);

        for j in 1..=nn {
            /* t := (j-th column of S) * x */
            let mut t = 0.0;
            let ptr = *ss_ptr.offset(j as isize);
            let end = ptr + *ss_len.offset(j as isize);
            for p in ptr..end {
                t += *sv_val.offset(p as isize) * x[*sv_ind.offset(p as isize) as usize];
            }
            /* y[j] := y[j] + alpha * t */
            y[j as usize] += a * t;
        }
    }
}

pub fn scf_a_solve(
    scf: &Scf,
    x: &mut [f64],
    w: &mut [f64],
    work1: &mut [f64],
    work2: &mut [f64],
    work3: &mut [f64],
) {
    let n = scf.n;
    let n0 = scf.n0;
    let nn = scf.nn;
    unsafe {
        let pp_ind = scf.pp_ind;
        let qq_inv = scf.qq_inv;

        /* (u1, u2) := inv(P) * (b, 0) */
        for ii in 1..=n0 + nn {
            let i = *pp_ind.offset(ii as isize);
            debug_assert!(i == ii); /* FIXME: currently P = I */
            w[ii as usize] = if i <= n { x[i as usize] } else { 0.0 };
        }

        /* v1 := inv(R0) * u1 */
        scf_r0_solve(scf, 0, &mut w[0..=n0 as usize]);

        /* v2 := u2 - R * v1 */
        scf_r_prod(scf, &mut w[n0 as usize..], -1.0, &w[0..=n0 as usize]);

        /* w2 := inv(C) * v2 */
        ifu_a_solve(&scf.ifu, &mut w[n0 as usize..], work1);

        /* w1 := inv(S0) * (v1 - S * w2) */
        scf_s_prod(
            scf,
            &mut w[0..=n0 as usize],
            -1.0,
            &w[n0 as usize..],
        );
        scf_s0_solve(
            scf,
            0,
            &mut w[0..=n0 as usize],
            work1,
            work2,
            work3,
        );

        /* (x, x~) := inv(Q) * (w1, w2); x~ is not needed */
        for i in 1..=n {
            x[i as usize] = w[*qq_inv.offset(i as isize) as usize];
        }
    }
}

pub fn scf_at_solve(
    scf: &Scf,
    x: &mut [f64],
    w: &mut [f64],
    work1: &mut [f64],
    work2: &mut [f64],
    work3: &mut [f64],
) {
    let n = scf.n;
    let n0 = scf.n0;
    let nn = scf.nn;
    unsafe {
        let pp_inv = scf.pp_inv;
        let qq_ind = scf.qq_ind;

        /* (u1, u2) := Q * (b, 0) */
        for ii in 1..=n0 + nn {
            let i = *qq_ind.offset(ii as isize);
            w[ii as usize] = if i <= n { x[i as usize] } else { 0.0 };
        }

        /* v1 := inv(S0') * u1 */
        scf_s0_solve(scf, 1, &mut w[0..=n0 as usize], work1, work2, work3);

        /* v2 := inv(C') * (u2 - S'* v1) */
        scf_st_prod(
            scf,
            &mut w[n0 as usize..],
            -1.0,
            &w[0..=n0 as usize],
        );
        ifu_at_solve(&scf.ifu, &mut w[n0 as usize..], work1);

        /* w1 := inv(R0') * (v1 - R'* w2) */
        scf_rt_prod(
            scf,
            &mut w[0..=n0 as usize],
            -1.0,
            &w[n0 as usize..],
        );
        scf_r0_solve(scf, 1, &mut w[0..=n0 as usize]);

        /* compute (x, x~) := P * (w1, w2); x~ is not needed */
        for i in 1..=n {
            debug_assert!(*pp_inv.offset(i as isize) == i); /* FIXME: currently P = I */
            x[i as usize] = w[*pp_inv.offset(i as isize) as usize];
        }
    }
}

pub fn scf_add_r_row(scf: &mut Scf, w: &[f64]) {
    let n0 = scf.n0;
    let nn = scf.nn;
    unsafe {
        let sva = &mut *scf.sva;
        let sv_ind = sva.ind;
        let sv_val = sva.val;
