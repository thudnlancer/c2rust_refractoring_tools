use std::ptr;

#[derive(Debug, Clone)]
pub struct FVS {
    n: i32,
    nnz: i32,
    ind: Vec<i32>,
    vec: Vec<f64>,
}

impl FVS {
    pub fn new(n: i32) -> Self {
        assert!(n >= 0, "n must be non-negative");
        FVS {
            n,
            nnz: 0,
            ind: vec![0; (n + 1) as usize],
            vec: vec![0.0; (n + 1) as usize],
        }
    }

    pub fn check_vec(&self) {
        assert!(self.n >= 0, "n must be non-negative");
        assert!(
            (0..=self.n).contains(&self.nnz),
            "nnz must be between 0 and n"
        );

        let mut map = vec![false; (self.n + 1) as usize];
        for j in 1..=self.n {
            map[j as usize] = self.vec[j as usize] != 0.0;
        }

        for k in 1..=self.nnz {
            let j = self.ind[k as usize];
            assert!((1..=self.n).contains(&j), "index out of bounds");
            assert!(map[j as usize], "map[j] must be true");
            map[j as usize] = false;
        }

        for j in 1..=self.n {
            assert!(!map[j as usize], "map[j] must be false");
        }
    }

    pub fn gather_vec(&mut self, eps: f64) {
        let mut nnz = 0;
        for j in (1..=self.n).rev() {
            if -eps < self.vec[j as usize] && self.vec[j as usize] < eps {
                self.vec[j as usize] = 0.0;
            } else {
                nnz += 1;
                self.ind[nnz as usize] = j;
            }
        }
        self.nnz = nnz;
    }

    pub fn clear_vec(&mut self) {
        for k in (1..=self.nnz).rev() {
            let j = self.ind[k as usize];
            self.vec[j as usize] = 0.0;
        }
        self.nnz = 0;
    }

    pub fn copy_vec(&mut self, other: &Self) {
        assert!(
            ptr::eq(self, other),
            "source and destination must be different"
        );
        assert!(self.n == other.n, "dimensions must match");

        self.clear_vec();
        self.nnz = other.nnz;
        for k in (1..=self.nnz).rev() {
            self.ind[k as usize] = other.ind[k as usize];
            let j = self.ind[k as usize];
            self.vec[j as usize] = other.vec[j as usize];
        }
    }

    pub fn adjust_vec(&mut self, eps: f64) {
        let mut cnt = 0;
        for k in 1..=self.nnz {
            let j = self.ind[k as usize];
            if -eps < self.vec[j as usize] && self.vec[j as usize] < eps {
                self.vec[j as usize] = 0.0;
            } else {
                cnt += 1;
                self.ind[cnt as usize] = j;
            }
        }
        self.nnz = cnt;
    }
}

impl Drop for FVS {
    fn drop(&mut self) {
        self.nnz = -1;
        self.n = self.nnz;
    }
}