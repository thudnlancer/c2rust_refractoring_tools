/* spxlp.rs */

use std::f64;
use std::mem;
use std::ptr;

use crate::bfd::BFD;
use crate::env::FVS;

/***********************************************************************
*  The structure SPXLP describes LP problem and its current basis.
*
*  It is assumed that LP problem has the following formulation (this is
*  so called "working format"):
*
*     z = c'* x + c0 -> min                                          (1)
*
*     A * x = b                                                      (2)
*
*     l <= x <= u                                                    (3)
*
*  where:
*
*  x = (x[k]) is a n-vector of variables;
*
*  z is an objective function;
*
*  c = (c[k]) is a n-vector of objective coefficients;
*
*  c0 is a constant term of the objective function;
*
*  A = (a[i,k]) is a mxn-matrix of constraint coefficients;
*
*  b = (b[i]) is a m-vector of right-hand sides;
*
*  l = (l[k]) is a n-vector of lower bounds of variables;
*
*  u = (u[k]) is a n-vector of upper bounds of variables.
*
*  If variable x[k] has no lower (upper) bound, it is formally assumed
*  that l[k] = -inf (u[k] = +inf). Variable having no bounds is called
*  free (unbounded) variable. If l[k] = u[k], variable x[k] is assumed
*  to be fixed.
*
*  It is also assumed that matrix A has full row rank: rank(A) = m,
*  i.e. all its rows are linearly independent, so m <= n.
*
*  The (current) basis is defined by an appropriate permutation matrix
*  P of order n such that:
*
*             ( xB )
*     P * x = (    ),                                                (4)
*             ( xN )
*
*  where xB = (xB[i]) is a m-vector of basic variables, xN = (xN[j]) is
*  a (n-m)-vector of non-basic variables. If a non-basic variable xN[j]
*  has both lower and upper bounds, there is used an additional flag to
*  indicate which bound is active.
*
*  From (2) and (4) it follows that:
*
*     A * P'* P * x = b   <=>   B * xB + N * xN = b,                 (5)
*
*  where P' is a matrix transposed to P, and
*
*     A * P' = (B | N).                                              (6)
*
*  Here B is the basis matrix, which is a square non-singular matrix
*  of order m composed from columns of matrix A that correspond to
*  basic variables xB, and N is a mx(n-m) matrix composed from columns
*  of matrix A that correspond to non-basic variables xN. */

pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub A_ptr: Vec<i32>,
    pub A_ind: Vec<i32>,
    pub A_val: Vec<f64>,
    pub b: Vec<f64>,
    pub c: Vec<f64>,
    pub l: Vec<f64>,
    pub u: Vec<f64>,
    pub head: Vec<i32>,
    pub flag: Vec<u8>,
    pub valid: i32,
    pub bfd: BFD,
}

impl SPXLP {
    pub fn new(
        m: i32,
        n: i32,
        nnz: i32,
        A_ptr: Vec<i32>,
        A_ind: Vec<i32>,
        A_val: Vec<f64>,
        b: Vec<f64>,
        c: Vec<f64>,
        l: Vec<f64>,
        u: Vec<f64>,
        head: Vec<i32>,
        flag: Vec<u8>,
        valid: i32,
        bfd: BFD,
    ) -> Self {
        SPXLP {
            m,
            n,
            nnz,
            A_ptr,
            A_ind,
            A_val,
            b,
            c,
            l,
            u,
            head,
            flag,
            valid,
            bfd,
        }
    }
}

/***********************************************************************
*  spx_factorize - compute factorization of current basis matrix
*
*  This routine computes factorization of the current basis matrix B.
*
*  If the factorization has been successfully computed, the routine
*  validates it and returns zero. Otherwise, the routine invalidates
*  the factorization and returns the code provided by the factorization
*  driver (bfd_factorize). */

fn jth_col(info: &SPXLP, j: i32, ind: &mut [i32], val: &mut [f64]) -> i32 {
    let m = info.m;
    let A_ptr = &info.A_ptr;
    let head = &info.head;
    let k = head[j as usize - 1]; // x[k] = xB[j]
    assert!(1 <= j && j <= m);
    let ptr = A_ptr[k as usize - 1] as usize;
    let len = (A_ptr[k as usize] - A_ptr[k as usize - 1]) as usize;
    ind[1..1 + len].copy_from_slice(&info.A_ind[ptr - 1..ptr - 1 + len]);
    val[1..1 + len].copy_from_slice(&info.A_val[ptr - 1..ptr - 1 + len]);
    len as i32
}

pub fn spx_factorize(lp: &mut SPXLP) -> i32 {
    let ret = lp.bfd.bfd_factorize(lp.m, |j, ind, val| jth_col(lp, j, ind, val));
    lp.valid = if ret == 0 { 1 } else { 0 };
    ret
}

/***********************************************************************
*  spx_eval_beta - compute current values of basic variables
*
*  This routine computes vector beta = (beta[i]) of current values of
*  basic variables xB = (xB[i]). (Factorization of the current basis
*  matrix should be valid.) */

pub fn spx_eval_beta(lp: &SPXLP, beta: &mut [f64]) {
    let m = lp.m;
    let n = lp.n;
    let A_ptr = &lp.A_ptr;
    let A_ind = &lp.A_ind;
    let A_val = &lp.A_val;
    let b = &lp.b;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let flag = &lp.flag;
    
    // compute y = b - N * xN
    // y := b
    beta[1..=m as usize].copy_from_slice(&b[1..=m as usize]);
    
    // y := y - N * f
    for j in 1..=n - m {
        let k = head[(m + j) as usize - 1]; // x[k] = xN[j]
        // f[j] := active bound of xN[j]
        let fj = if flag[j as usize - 1] != 0 {
            u[k as usize - 1]
        } else {
            l[k as usize - 1]
        };
        
        if fj == 0.0 || fj == -f64::MAX {
            continue;
        }
        
        // y := y - N[j] * f[j]
        let ptr = A_ptr[k as usize - 1] as usize;
        let end = A_ptr[k as usize] as usize;
        for i in ptr..end {
            beta[A_ind[i - 1] as usize] -= A_val[i - 1] * fj;
        }
    }
    
    // compute beta = inv(B) * y
    assert!(lp.valid != 0);
    lp.bfd.bfd_ftran(beta);
}

/***********************************************************************
*  spx_eval_obj - compute current value of objective function
*
*  This routine computes the value of the objective function in the
*  current basic solution. */

pub fn spx_eval_obj(lp: &SPXLP, beta: &[f64]) -> f64 {
    let m = lp.m;
    let n = lp.n;
    let c = &lp.c;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let flag = &lp.flag;
    
    // compute z = cB'* beta + cN'* f + c0
    let mut z = c[0];
    
    // z := z + cB'* beta
    for i in 1..=m {
        let k = head[i as usize - 1]; // x[k] = xB[i]
        z += c[k as usize - 1] * beta[i as usize];
    }
    
    // z := z + cN'* f
    for j in 1..=n - m {
        let k = head[(m + j) as usize - 1]; // x[k] = xN[j]
        // f[j] := active bound of xN[j]
        let fj = if flag[j as usize - 1] != 0 {
            u[k as usize - 1]
        } else {
            l[k as usize - 1]
        };
        
        if fj == 0.0 || fj == -f64::MAX {
            continue;
        }
        
        z += c[k as usize - 1] * fj;
    }
    
    z
}

/***********************************************************************
*  spx_eval_pi - compute simplex multipliers in current basis
*
*  This routine computes vector pi = (pi[i]) of simplex multipliers in
*  the current basis. */

pub fn spx_eval_pi(lp: &SPXLP, pi: &mut [f64]) {
    let m = lp.m;
    let c = &lp.c;
    let head = &lp.head;
    
    // construct cB
    for i in 1..=m {
        pi[i as usize] = c[head[i as usize - 1] as usize - 1];
    }
    
    // compute pi = inv(B') * cB
    lp.bfd.bfd_btran(pi);
}

/***********************************************************************
*  spx_eval_dj - compute reduced cost of j-th non-basic variable
*
*  This routine computes reduced cost d[j] of non-basic variable
*  xN[j] = x[k], 1 <= j <= n-m. */

pub fn spx_eval_dj(lp: &SPXLP, pi: &[f64], j: i32) -> f64 {
    let m = lp.m;
    let n = lp.n;
    let A_ptr = &lp.A_ptr;
    let A_ind = &lp.A_ind;
    let A_val = &lp.A_val;
    
    assert!(1 <= j && j <= n - m);
    let k = lp.head[(m + j) as usize - 1]; // x[k] = xN[j]
    
    // dj := c[k]
    let mut dj = lp.c[k as usize - 1];
    
    // dj := dj - A'[k] * pi
    let ptr = A_ptr[k as usize - 1] as usize;
    let end = A_ptr[k as usize] as usize;
    for i in ptr..end {
        dj -= A_val[i - 1] * pi[A_ind[i - 1] as usize];
    }
    
    dj
}

/***********************************************************************
*  spx_eval_tcol - compute j-th column of simplex table
*
*  This routine computes j-th column of the current simplex table. */

pub fn spx_eval_tcol(lp: &SPXLP, j: i32, tcol: &mut [f64]) {
    let m = lp.m;
    let n = lp.n;
    let A_ptr = &lp.A_ptr;
    let A_ind = &lp.A_ind;
    let A_val = &lp.A_val;
    let head = &lp.head;
    
    assert!(1 <= j && j <= n - m);
    let k = head[(m + j) as usize - 1]; // x[k] = xN[j]
    
    // compute tcol = - inv(B) * N[j]
    tcol[1..=m as usize].fill(0.0);
    let ptr = A_ptr[k as usize - 1] as usize;
    let end = A_ptr[k as usize] as usize;
    for i in ptr..end {
        tcol[A_ind[i - 1] as usize] = -A_val[i - 1];
    }
    
    lp.bfd.bfd_ftran(tcol);
}

/***********************************************************************
*  spx_eval_rho - compute i-th row of basis matrix inverse
*
*  This routine computes i-th row of the matrix inv(B). */

pub fn spx_eval_rho(lp: &SPXLP, i: i32, rho: &mut [f64]) {
    let m = lp.m;
    assert!(1 <= i && i <= m);
    
    // compute rho = inv(B') * e[i]
    rho[1..=m as usize].fill(0.0);
    rho[i as usize] = 1.0;
    lp.bfd.bfd_btran(rho);
}

/***********************************************************************
*  spx_eval_rho_s - sparse version of spx_eval_rho */

pub fn spx_eval_rho_s(lp: &SPXLP, i: i32, rho: &mut FVS) {
    let m = lp.m;
    assert!(1 <= i && i <= m);
    assert!(rho.n == m);
    
    // compute rho = inv(B') * e[i]
    rho.clear();
    rho.nnz = 1;
    rho.ind[0] = i;
    rho.vec[i as usize - 1] = 1.0;
    lp.bfd.bfd_btran_s(rho);
}

/***********************************************************************
*  spx_eval_tij - compute element T[i,j] of simplex table
*
*  This routine computes element T[i,j] of the current simplex table. */

pub fn spx_eval_tij(lp: &SPXLP, rho: &[f64], j: i32) -> f64 {
    let m = lp.m;
    let n = lp.n;
    let A_ptr = &lp.A_ptr;
    let A_ind = &lp.A_ind;
    let A_val = &lp.A_val;
    
    assert!(1 <= j && j <= n - m);
    let k = lp.head[(m + j) as usize - 1]; // x[k] = xN[j]
    
    // compute t[i,j] = - N'[j] * rho
    let mut tij = 0.0;
    let ptr = A_ptr[k as usize - 1] as usize;
    let end = A_ptr[k as usize] as usize;
    for i in ptr..end {
        tij -= A_val[i - 1] * rho[A_ind[i - 1] as usize];
    }
    
    tij
}

/***********************************************************************
*  spx_eval_trow - compute i-th row of simplex table
*
*  This routine computes i-th row of the current simplex table. */

pub fn spx_eval_trow(lp: &SPXLP, rho: &[f64], trow: &mut [f64]) {
    let m = lp.m;
    let n = lp.n;
    for j in 1..=n - m {
        trow[j as usize] = spx_eval_tij(lp, rho, j);
    }
}

/***********************************************************************
*  spx_update_beta - update values of basic variables
*
*  This routine updates the vector beta = (beta[i]) of values of basic
*  variables xB = (xB[i]) for the adjacent basis. */

pub fn spx_update_beta(
    lp: &SPXLP,
    beta: &mut [f64],
    p: i32,
    p_flag: i32,
    q: i32,
    tcol: &[f64],
) {
    let m = lp.m;
    let n = lp.n;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;
    let flag = &lp.flag;
    
    let (delta_p, delta_q) = if p < 0 {
        // special case: xN[q] goes to its opposite bound
        assert!(1 <= q && q <= n - m);
        let k = head[(m + q) as usize - 1]; // x[k] = xN[q]
        assert!(l[k as usize - 1] != -f64::MAX && u[k as usize - 1] != f64::MAX && l[k as usize - 1] != u[k as usize - 1]);
        
        // determine delta xN[q]
        let delta_q = if flag[q as usize - 1] != 0 {
            l[k as usize - 1] - u[k as usize - 1]
        } else {
            u[k as usize - 1] - l[k as usize - 1]
        };
        
        (0.0, delta_q)
    } else {
        // xB[p] leaves the basis, xN[q] enters the basis
        assert!(1 <= p && p <= m);
        assert!(1 <= q && q <= n - m);
        
        // determine delta xB[p]
        let k = head[p as usize - 1]; // x[k] = xB[p]
        let delta_p = if p_flag != 0 {
            // xB[p] goes to its upper bound
            assert!(l[k as usize - 1] != u[k as usize - 1] && u[k as usize - 1] != f64::MAX);
            u[k as usize - 1] - beta[p as usize]
        } else if l[k as usize - 1] == -f64::MAX {
            // unbounded xB[p] becomes non-basic (unusual case)
            assert!(u[k as usize - 1] == f64::MAX);
            0.0 - beta[p as usize]
        } else {
            //