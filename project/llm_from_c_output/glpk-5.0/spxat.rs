/* spxat.rs */

use std::ptr;
use std::mem;
use std::ops::{AddAssign, MulAssign};

pub struct SPXAT {
    ptr: Vec<i32>,
    ind: Vec<i32>,
    val: Vec<f64>,
    work: Vec<f64>,
}

impl SPXAT {
    pub fn new() -> Self {
        SPXAT {
            ptr: Vec::new(),
            ind: Vec::new(),
            val: Vec::new(),
            work: Vec::new(),
        }
    }

    pub fn alloc_at(&mut self, lp: &SPXLP) {
        let m = lp.m;
        let n = lp.n;
        let nnz = lp.nnz;
        self.ptr = vec![0; m + 2];
        self.ind = vec![0; nnz + 1];
        self.val = vec![0.0; nnz + 1];
        self.work = vec![0.0; n + 1];
    }

    pub fn build_at(&mut self, lp: &SPXLP) {
        let m = lp.m;
        let n = lp.n;
        let nnz = lp.nnz;
        let a_ptr = &lp.a_ptr;
        let a_ind = &lp.a_ind;
        let a_val = &lp.a_val;

        // Calculate AT_ptr[i] = number of non-zeros in i-th row
        for k in 1..=n {
            let ptr = a_ptr[k] as usize;
            let end = a_ptr[k + 1] as usize;
            for p in ptr..end {
                self.ptr[a_ind[p] as usize] += 1;
            }
        }

        // Set AT_ptr[i] to position after last element in i-th row
        self.ptr[1] += 1;
        for i in 2..=m {
            self.ptr[i] += self.ptr[i - 1];
        }
        assert_eq!(self.ptr[m] as usize, nnz + 1);
        self.ptr[m + 1] = (nnz + 1) as i32;

        // Build row-wise representation and re-arrange AT_ptr[i]
        for k in (1..=n).rev() {
            let ptr = a_ptr[k] as usize;
            let end = a_ptr[k + 1] as usize;
            for p in ptr..end {
                let i = a_ind[p] as usize;
                let pos = {
                    let pos_ptr = &mut self.ptr[i];
                    *pos_ptr -= 1;
                    *pos_ptr as usize
                };
                self.ind[pos] = k as i32;
                self.val[pos] = a_val[p];
            }
        }
        assert_eq!(self.ptr[1], 1);
    }

    pub fn at_prod(&self, lp: &SPXLP, y: &mut [f64], s: f64, x: &[f64]) {
        let m = lp.m;
        for i in 1..=m {
            if x[i] != 0.0 {
                let t = s * x[i];
                let ptr = self.ptr[i] as usize;
                let end = self.ptr[i + 1] as usize;
                for p in ptr..end {
                    let j = self.ind[p] as usize;
                    y[j] += self.val[p] * t;
                }
            }
        }
    }

    pub fn nt_prod1(&self, lp: &SPXLP, y: &mut [f64], ign: bool, s: f64, x: &[f64]) {
        let m = lp.m;
        let n = lp.n;
        let head = &lp.head;
        let mut work = vec![0.0; n + 1];

        if !ign {
            for j in 1..=(n - m) {
                work[head[m + j] as usize] = y[j];
            }
        }

        self.at_prod(lp, &mut work, s, x);

        for j in 1..=(n - m) {
            y[j] = work[head[m + j] as usize];
        }
    }

    pub fn eval_trow1(&self, lp: &SPXLP, rho: &[f64], trow: &mut [f64]) {
        let m = lp.m;
        let n = lp.n;
        let nnz = lp.nnz;

        // Determine nnz(rho)
        let nnz_rho = rho[1..=m].iter().filter(|&&x| x != 0.0).count();

        // Estimate the number of operations for both ways
        let cnt1 = (n - m) as f64 * (nnz as f64 / n as f64);
        let cnt2 = nnz_rho as f64 * (nnz as f64 / m as f64);

        // Compute i-th row of simplex table
        if cnt1 < cnt2 {
            // As inner products
            let a_ptr = &lp.a_ptr;
            let a_ind = &lp.a_ind;
            let a_val = &lp.a_val;
            let head = &lp.head;

            for j in 1..=(n - m) {
                let k = head[m + j]; // x[k] = xN[j]
                // Compute t[i,j] = - N'[j] * pi
                let mut tij = 0.0;
                let ptr = a_ptr[k] as usize;
                let end = a_ptr[k + 1] as usize;
                for p in ptr..end {
                    tij -= a_val[p] * rho[a_ind[p] as usize];
                }
                trow[j] = tij;
            }
        } else {
            // As linear combination
            self.nt_prod1(lp, trow, true, -1.0, rho);
        }
    }

    pub fn free_at(&mut self) {
        self.ptr.clear();
        self.ind.clear();
        self.val.clear();
        self.work.clear();
    }
}

// Note: SPXLP struct and related functions would need to be defined elsewhere
// This is a minimal implementation focusing on the SPXAT functionality