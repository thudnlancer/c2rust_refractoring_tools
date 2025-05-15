use std::mem;
use std::ptr;
use std::f64;

/// Sparse vector representation
pub struct SparseVector {
    /// Dimension, n >= 0
    n: usize,
    /// Number of non-zero components, 0 <= nnz <= n
    nnz: usize,
    /// pos[j] = k, 1 <= j <= n, is position of (non-zero) v[j] in the
    /// arrays ind and val, where 1 <= k <= nnz; pos[j] = 0 means that
    /// v[j] is structural zero
    pos: Vec<usize>,
    /// ind[k] = j, 1 <= k <= nnz, is index of v[j]
    ind: Vec<usize>,
    /// val[k], 1 <= k <= nnz, is a numeric value of v[j]
    val: Vec<f64>,
}

impl SparseVector {
    /// Create a new sparse vector of dimension n, initialized to zero
    pub fn new(n: usize) -> Self {
        assert!(n >= 0);
        SparseVector {
            n,
            nnz: 0,
            pos: vec![0; n + 1],
            ind: vec![0; n + 1],
            val: vec![0.0; n + 1],
        }
    }

    /// Check that sparse vector has correct representation
    pub fn check(&self) {
        assert!(self.n >= 0);
        let mut nnz = 0;
        for j in (1..=self.n).rev() {
            let k = self.pos[j];
            assert!(k <= self.nnz);
            if k != 0 {
                assert!(self.ind[k] == j);
                nnz += 1;
            }
        }
        assert!(self.nnz == nnz);
    }

    /// Retrieve component of sparse vector
    pub fn get(&self, j: usize) -> f64 {
        assert!(1 <= j && j <= self.n);
        let k = self.pos[j];
        assert!(k <= self.nnz);
        if k == 0 { 0.0 } else { self.val[k] }
    }

    /// Set/change component of sparse vector
    pub fn set(&mut self, j: usize, val: f64) {
        assert!(1 <= j && j <= self.n);
        let k = self.pos[j];
        
        if val == 0.0 {
            if k != 0 {
                // Remove j-th component
                self.pos[j] = 0;
                if k < self.nnz {
                    let last = self.nnz;
                    self.pos[self.ind[last]] = k;
                    self.ind[k] = self.ind[last];
                    self.val[k] = self.val[last];
                }
                self.nnz -= 1;
            }
        } else {
            if k == 0 {
                // Create j-th component
                self.nnz += 1;
                self.pos[j] = self.nnz;
                self.ind[self.nnz] = j;
                self.val[self.nnz] = val;
            } else {
                self.val[k] = val;
            }
        }
    }

    /// Set all components of sparse vector to zero
    pub fn clear(&mut self) {
        for k in 1..=self.nnz {
            self.pos[self.ind[k]] = 0;
        }
        self.nnz = 0;
    }

    /// Remove zero or small components from sparse vector
    pub fn clean(&mut self, eps: f64) {
        let mut nnz = 0;
        for k in 1..=self.nnz {
            if self.val[k].abs() == 0.0 || self.val[k].abs() < eps {
                // Remove component
                self.pos[self.ind[k]] = 0;
            } else {
                // Keep component
                nnz += 1;
                self.pos[self.ind[k]] = nnz;
                self.ind[nnz] = self.ind[k];
                self.val[nnz] = self.val[k];
            }
        }
        self.nnz = nnz;
    }

    /// Copy sparse vector (self := other)
    pub fn copy_from(&mut self, other: &Self) {
        assert!(self.n == other.n);
        self.clear();
        self.nnz = other.nnz;
        self.ind[1..=self.nnz].copy_from_slice(&other.ind[1..=self.nnz]);
        self.val[1..=self.nnz].copy_from_slice(&other.val[1..=self.nnz]);
        for j in 1..=self.nnz {
            self.pos[self.ind[j]] = j;
        }
    }

    /// Compute linear combination (self := self + a * other)
    pub fn linear_comb(&mut self, a: f64, other: &Self) {
        assert!(self.n == other.n);
        for k in 1..=other.nnz {
            let j = other.ind[k];
            let xj = self.get(j);
            let yj = other.val[k];
            self.set(j, xj + a * yj);
        }
    }
}

impl Drop for SparseVector {
    fn drop(&mut self) {
        // Memory is automatically managed by Vec
    }
}