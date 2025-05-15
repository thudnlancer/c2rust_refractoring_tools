use std::ptr;
use std::mem;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct SPV {
    n: i32,
    nnz: i32,
    pos: Vec<i32>,
    ind: Vec<i32>,
    val: Vec<f64>,
}

impl SPV {
    pub fn new(n: i32) -> Self {
        assert!(n >= 0, "n must be non-negative");
        
        let mut pos = vec![0; (n + 1) as usize];
        let ind = vec![0; (n + 1) as usize];
        let val = vec![0.0; (n + 1) as usize];
        
        SPV { n, nnz: 0, pos, ind, val }
    }

    pub fn check(&self) {
        assert!(self.n >= 0, "v.n must be non-negative");
        
        let mut nnz = 0;
        for j in (1..=self.n).rev() {
            let k = self.pos[j as usize];
            assert!(0 <= k && k <= self.nnz, "position out of bounds");
            
            if k != 0 {
                assert_eq!(self.ind[k as usize], j, "index mismatch");
                nnz += 1;
            }
        }
        assert_eq!(self.nnz, nnz, "nnz count mismatch");
    }

    pub fn get_vj(&self, j: i32) -> f64 {
        assert!(1 <= j && j <= self.n, "index out of bounds");
        
        let k = self.pos[j as usize];
        if k == 0 {
            0.0
        } else {
            self.val[k as usize]
        }
    }

    pub fn set_vj(&mut self, j: i32, val: f64) {
        assert!(1 <= j && j <= self.n, "index out of bounds");
        
        let k = self.pos[j as usize];
        if val == 0.0 {
            if k != 0 {
                self.pos[j as usize] = 0;
                if k < self.nnz {
                    let last = self.ind[self.nnz as usize];
                    self.pos[last as usize] = k;
                    self.ind[k as usize] = last;
                    self.val[k as usize] = self.val[self.nnz as usize];
                }
                self.nnz -= 1;
            }
        } else {
            if k == 0 {
                self.nnz += 1;
                let new_k = self.nnz;
                self.pos[j as usize] = new_k;
                self.ind[new_k as usize] = j;
                self.val[new_k as usize] = val;
            } else {
                self.val[k as usize] = val;
            }
        }
    }

    pub fn clear(&mut self) {
        for k in 1..=self.nnz {
            let j = self.ind[k as usize];
            self.pos[j as usize] = 0;
        }
        self.nnz = 0;
    }

    pub fn clean(&mut self, eps: f64) {
        let mut nnz = 0;
        for k in 1..=self.nnz {
            let val = self.val[k as usize];
            let j = self.ind[k as usize];
            
            if val.abs() == 0.0 || val.abs() < eps {
                self.pos[j as usize] = 0;
            } else {
                nnz += 1;
                self.pos[j as usize] = nnz;
                self.ind[nnz as usize] = j;
                self.val[nnz as usize] = val;
            }
        }
        self.nnz = nnz;
    }

    pub fn copy_from(&mut self, other: &Self) {
        assert!(self.n == other.n, "vector size mismatch");
        assert!(!ptr::eq(self, other), "cannot copy from self");
        
        self.clear();
        self.nnz = other.nnz;
        
        self.ind[1..=self.nnz as usize].copy_from_slice(&other.ind[1..=other.nnz as usize]);
        self.val[1..=self.nnz as usize].copy_from_slice(&other.val[1..=other.nnz as usize]);
        
        for j in 1..=self.nnz {
            let idx = self.ind[j as usize];
            self.pos[idx as usize] = j;
        }
    }

    pub fn linear_comb(&mut self, a: f64, y: &Self) {
        assert!(self.n == y.n, "vector size mismatch");
        assert!(!ptr::eq(self, y), "cannot combine with self");
        
        for k in 1..=y.nnz {
            let j = y.ind[k as usize];
            let xj = self.get_vj(j);
            let yj = y.val[k as usize];
            self.set_vj(j, xj + a * yj);
        }
    }
}

impl Drop for SPV {
    fn drop(&mut self) {
        // Vecs will automatically deallocate
    }
}