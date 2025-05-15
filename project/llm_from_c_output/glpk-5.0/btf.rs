/* btf.rs (sparse block triangular LU-factorization) */

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
use std::cmp::{min, max};
use std::ops::{Index, IndexMut};

use crate::sva::{SVA, SVAVec};
use crate::luf::LUF;
use crate::env::Env;
use crate::mc13d::mc13d;
use crate::mc21a::mc21a;

#[derive(Debug)]
pub struct BTF {
    pub n: i32,
    pub sva: *mut SVA,
    pub pp_ind: Vec<i32>,
    pub pp_inv: Vec<i32>,
    pub qq_ind: Vec<i32>,
    pub qq_inv: Vec<i32>,
    pub num: i32,
    pub beg: Vec<i32>,
    pub ar_ref: i32,
    pub ac_ref: i32,
    pub fr_ref: i32,
    pub fc_ref: i32,
    pub vr_ref: i32,
    pub vr_piv: Vec<f64>,
    pub vc_ref: i32,
    pub p1_ind: Vec<i32>,
    pub p1_inv: Vec<i32>,
    pub q1_ind: Vec<i32>,
    pub q1_inv: Vec<i32>,
}

impl BTF {
    pub fn new(n: i32, sva: *mut SVA) -> Self {
        let n = n as usize;
        BTF {
            n: n as i32,
            sva,
            pp_ind: vec![0; n + 1],
            pp_inv: vec![0; n + 1],
            qq_ind: vec![0; n + 1],
            qq_inv: vec![0; n + 1],
            num: 0,
            beg: vec![0; n + 2],
            ar_ref: 0,
            ac_ref: 0,
            fr_ref: 0,
            fc_ref: 0,
            vr_ref: 0,
            vr_piv: vec![0.0; n + 1],
            vc_ref: 0,
            p1_ind: vec![0; n + 1],
            p1_inv: vec![0; n + 1],
            q1_ind: vec![0; n + 1],
            q1_inv: vec![0; n + 1],
        }
    }

    pub fn store_a_cols(
        &mut self,
        col: &mut dyn FnMut(&mut dyn std::any::Any, i32, &mut [i32], &mut [f64]) -> i32,
        info: &mut dyn std::any::Any,
        ind: &mut [i32],
        val: &mut [f64],
    ) -> i32 {
        let n = self.n;
        let sva = unsafe { &mut *self.sva };
        let ac_ref = self.ac_ref;
        let ac_ptr = unsafe { &mut sva.ptr[ac_ref as usize - 1..] };
        let ac_len = unsafe { &mut sva.len[ac_ref as usize - 1..] };
        let mut nnz = 0;

        for j in 1..=n {
            let len = col(info, j, ind, val);
            assert!(0 <= len && len <= n);
            
            if len > 0 {
                if sva.r_ptr - sva.m_ptr < len {
                    sva.more_space(len);
                }
                sva.reserve_cap(ac_ref + j - 1, len);
            }

            let ptr = ac_ptr[j as usize];
            unsafe {
                ptr::copy_nonoverlapping(
                    ind.as_ptr().offset(1),
                    sva.ind.as_mut_ptr().offset(ptr as isize),
                    len as usize,
                );
            }
            ac_len[j as usize] = len;
            nnz += len;
        }
        nnz
    }

    pub fn make_blocks(&mut self) -> i32 {
        let n = self.n;
        let sva = unsafe { &mut *self.sva };
        let pp_ind = &mut self.pp_ind;
        let pp_inv = &mut self.pp_inv;
        let qq_ind = &mut self.qq_ind;
        let qq_inv = &mut self.qq_inv;
        let beg = &mut self.beg;
        let ac_ref = self.ac_ref;
        let ac_ptr = unsafe { &sva.ptr[ac_ref as usize - 1..] };
        let ac_len = unsafe { &sva.len[ac_ref as usize - 1..] };

        let rank = mc21a(
            n,
            &sva.ind,
            ac_ptr,
            ac_len,
            &mut self.qq_inv,
            &mut self.p1_ind,
            &mut self.p1_inv,
            &mut self.q1_ind,
            &mut self.q1_inv,
        );
        assert!(0 <= rank && rank <= n);

        if rank < n {
            return rank;
        }

        let ip = &mut self.pp_ind;
        let lenr = &mut self.qq_ind;
        for j in 1..=n {
            ip[j as usize] = ac_ptr[self.qq_inv[j as usize] as usize];
            lenr[j as usize] = ac_len[self.qq_inv[j as usize] as usize];
        }

        self.num = mc13d(
            n,
            &sva.ind,
            ip,
            lenr,
            pp_inv,
            beg,
            &mut self.p1_ind,
            &mut self.p1_inv,
            &mut self.q1_ind,
        );
        assert!(beg[1] == 1);
        beg[(self.num + 1) as usize] = n + 1;

        for j in 1..=n {
            pp_ind[pp_inv[j as usize] = j;
        }

        for i in 1..=n {
            qq_ind[i as usize] = self.qq_inv[pp_inv[i as usize] as i32;
        }

        for i in 1..=n {
            qq_inv[qq_ind[i as usize] as usize] = i;
        }

        rank
    }

    pub fn check_blocks(&self) {
        let n = self.n;
        let sva = unsafe { &*self.sva };
        let pp_ind = &self.pp_ind;
        let pp_inv = &self.pp_inv;
        let qq_ind = &self.qq_ind;
        let qq_inv = &self.qq_inv;
        let num = self.num;
        let beg = &self.beg;
        let ac_ref = self.ac_ref;
        let ac_ptr = unsafe { &sva.ptr[ac_ref as usize - 1..] };
        let ac_len = unsafe { &sva.len[ac_ref as usize - 1..] };

        assert!(n > 0);

        for k in 1..=n {
            assert!(1 <= pp_ind[k as usize] && pp_ind[k as usize] <= n);
            assert!(pp_inv[pp_ind[k as usize] as i32 == k);
            assert!(1 <= qq_ind[k as usize] && qq_ind[k as usize] <= n);
            assert!(qq_inv[qq_ind[k as usize] as i32 == k);
        }

        assert!(1 <= num && num <= n);
        assert!(beg[1] == 1);
        assert!(beg[(num + 1) as usize] == n + 1);

        for k in 1..=num {
            let size = beg[(k + 1) as usize] - beg[k as usize];
            assert!(size >= 1);

            for jj in beg[k as usize]..beg[(k + 1) as usize] {
                let mut diag = 0;
                let j = qq_ind[jj as usize];
                let ptr = ac_ptr[j as usize];
                let end = ptr + ac_len[j as usize];

                for idx in ptr..end {
                    let i = sva.ind[idx as usize];
                    let ii = pp_ind[i as usize];
                    assert!(ii < beg[(k + 1) as usize]);
                    if ii == jj {
                        diag = 1;
                    }
                }
                assert!(diag != 0);
            }
        }
    }

    pub fn build_a_rows(&mut self, len: &mut [i32]) {
        let n = self.n;
        let sva = unsafe { &mut *self.sva };
        let ar_ref = self.ar_ref;
        let ar_ptr = unsafe { &mut sva.ptr[ar_ref as usize - 1..] };
        let ar_len = unsafe { &mut sva.len[ar_ref as usize - 1..] };
        let ac_ref = self.ac_ref;
        let ac_ptr = unsafe { &sva.ptr[ac_ref as usize - 1..] };
        let ac_len = unsafe { &sva.len[ac_ref as usize - 1..] };

        let mut nnz = 0;
        for i in 1..=n {
            len[i as usize] = 0;
        }

        for j in 1..=n {
            nnz += ac_len[j as usize];
            let ptr = ac_ptr[j as usize];
            let end = ptr + ac_len[j as usize];
            for idx in ptr..end {
                len[sva.ind[idx as usize] as usize] += 1;
            }
        }

        if sva.r_ptr - sva.m_ptr < nnz {
            sva.more_space(nnz);
        }

        for i in 1..=n {
            if len[i as usize] > 0 {
                sva.reserve_cap(ar_ref + i - 1, len[i as usize]);
            }
            ar_len[i as usize] = len[i as usize];
        }

        for j in 1..=n {
            let ptr = ac_ptr[j as usize];
            let end = ptr + ac_len[j as usize];
            for idx in ptr..end {
                let i = sva.ind[idx as usize];
                let ptr1 = ar_ptr[i as usize] + (len[i as usize] - 1);
                sva.ind[ptr1 as usize] = j;
                sva.val[ptr1 as usize] = sva.val[idx as usize];
                len[i as usize] -= 1;
            }
        }
    }

    pub fn a_solve(&self, b: &mut [f64], x: &mut [f64], w1: &mut [f64], w2: &mut [f64]) {
        let sva = unsafe { &*self.sva };
        let pp_inv = &self.pp_inv;
        let qq_ind = &self.qq_ind;
        let num = self.num;
        let beg = &self.beg;
        let ac_ref = self.ac_ref;
        let ac_ptr = unsafe { &sva.ptr[ac_ref as usize - 1..] };
        let ac_len = unsafe { &sva.len[ac_ref as usize - 1..] };
        let bb = w1;
        let xx = w2;

        for k in (1..=num).rev() {
            let beg_k = beg[k as usize];
            let n_block = beg[(k + 1) as usize] - beg_k;
            
            if n_block == 1 {
                let t = x[qq_ind[beg_k as usize] as usize] =
                    b[pp_inv[beg_k as usize] as usize] / self.vr_piv[beg_k as usize];
                
                if t != 0.0 {
                    let j = qq_ind[beg_k as usize];
                    let ptr = ac_ptr[j as usize];
                    let end = ptr + ac_len[j as usize];
                    for idx in ptr..end {
                        b[sva.ind[idx as usize] as usize] -= sva.val[idx as usize] * t;
                    }
                }
            } else {
                let mut flag = false;
                for i in 1..=n_block {
                    bb[i as usize] = b[pp_inv[(i + beg_k - 1) as usize] as usize];
                    if bb[i as usize] != 0.0 {
                        flag = true;
                    }
                }

                if !flag {
                    for j in 1..=n_block {
                        x[qq_ind[(j + beg_k - 1) as usize] as usize] = 0.0;
                    }
                    continue;
                }

                let mut luf = LUF {
                    n: n_block,
                    sva: self.sva,
                    fr_ref: self.fr_ref + beg_k - 1,
                    fc_ref: self.fc_ref + beg_k - 1,
                    vr_ref: self.vr_ref + beg_k - 1,
                    vr_piv: &self.vr_piv[beg_k as usize..],
                    vc_ref: self.vc_ref + beg_k - 1,
                    pp_ind: &self.p1_ind[beg_k as usize..],
                    pp_inv: &self.p1_inv[beg_k as usize..],
                    qq_ind: &self.q1_ind[beg_k as usize..],
                    qq_inv: &self.q1_inv[beg_k as usize..],
                };

                luf.f_solve(bb);
                luf.v_solve(bb, xx);

                for j in 1..=n_block {
                    let jj = j + beg_k - 1;
                    let t = x[qq_ind[jj as usize] as usize] = xx[j as usize];
                    if t != 0.0 {
                        let ptr = ac_ptr[qq_ind[jj as usize] as usize];
                        let end = ptr + ac_len[qq_ind[jj as usize] as usize];
                        for idx in ptr..end {
                            b[sva.ind[idx as usize] as usize] -= sva.val[idx as usize] * t;
                        }
                    }
                }
            }
        }
    }

    pub fn at_solve(&self, b: &mut [f64], x: &mut [f64], w1: &mut [f64], w2: &mut [f64]) {
        let sva = unsafe { &*self.sva };
        let pp_inv = &self.pp_inv;
        let qq_ind = &self.qq_ind;
        let num = self.num;
        let beg = &self.beg;
        let ar_ref = self.ar_ref;
        let ar_ptr = unsafe { &sva.ptr[ar_ref as usize - 1..] };
        let ar_len = unsafe { &sva.len[ar_ref as usize - 1..] };
        let bb = w1;
        let xx = w2;

        for k in 1..=num {
            let beg_k = beg[k as usize];
            let n_block = beg[(k + 1) as usize] - beg_k;
            
            if n_block == 1 {
                let t = x[pp_inv[beg_k as usize] as usize] =
                    b[qq_ind[beg_k as usize] as usize] / self.vr_piv[beg_k as usize];
                
                if t != 0.0 {
                    let i = pp_inv[beg_k as usize];
                    let ptr = ar_ptr[i as usize];
                    let end = ptr + ar_len[i as usize];
                    for idx in ptr..end {
                        b[sva.ind[idx as usize] as usize] -= sva.val[idx as usize] * t;
                    }
                }
            } else {
                let mut flag = false;
                for i in 1..=n_block {
                    bb[i as usize] = b[qq_ind[(i + beg_k - 1) as usize] as usize];
                    if bb[i as usize] != 0.0 {
                        flag = true;
                    }
                }

                if !flag {
                    for j in 1..=n_block {
                        x[pp_inv[(j + beg_k - 1) as usize] as usize] = 0.0;
                    }
                    continue;
                }

                let mut luf = LUF {
                    n: n_block,
                    sva: self.sva,
                    fr_ref: self.fr_ref + beg_k - 1,
                    fc_ref: self.fc_ref + beg_k - 1,
                    vr_ref: self.vr_ref + beg_k - 1,
                    vr_piv: &self.vr_piv[beg_k as usize..],
                    vc_ref: self.vc_ref + beg_k - 1,
                    pp_ind: &self.p1_ind[beg_k as usize..],
                    pp_inv: &self.p1_inv[beg_k as usize..],
                    qq_ind: &self.q1_ind[beg_k as usize..],
                    qq_inv: &self.q1_inv[beg_k as usize..],
                };

                luf.vt_solve(bb, xx);
                luf.ft_solve(xx);

                for j in 1..=n_block {
                    let jj = j + beg_k - 1;
                    let t = x[pp_inv[jj as usize] as usize] = xx[j as usize];
                    if t != 0.0 {
                        let ptr = ar_ptr[pp_inv[jj as usize] as usize];
                        let end = ptr + ar_len[pp_inv[jj as usize] as usize];
                        for idx in ptr..end {
                            b[sva.ind[idx as usize] as usize] -= sva.val[idx as usize] * t;
                        }
                    }
                }
            }
        }
    }

   