use std::ptr;

struct BFD;

#[derive(Clone)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub a_ptr: Vec<i32>,
    pub a_ind: Vec<i32>,
    pub a_val: Vec<f64>,
    pub b: Vec<f64>,
    pub c: Vec<f64>,
    pub l: Vec<f64>,
    pub u: Vec<f64>,
    pub head: Vec<i32>,
    pub flag: Vec<i8>,
    pub valid: i32,
    pub bfd: Option<Box<BFD>>,
}

#[derive(Clone)]
pub struct SPXAT {
    pub ptr: Vec<i32>,
    pub ind: Vec<i32>,
    pub val: Vec<f64>,
    pub work: Vec<f64>,
}

impl SPXLP {
    pub fn alloc_at(&self) -> SPXAT {
        SPXAT {
            ptr: vec![0; (self.m + 2) as usize],
            ind: vec![0; (self.nnz + 1) as usize],
            val: vec![0.0; (self.nnz + 1) as usize],
            work: vec![0.0; (self.n + 1) as usize],
        }
    }

    pub fn build_at(&mut self, at: &mut SPXAT) {
        assert_eq!(at.ptr.len(), (self.m + 2) as usize);
        assert_eq!(at.ind.len(), (self.nnz + 1) as usize);
        assert_eq!(at.val.len(), (self.nnz + 1) as usize);

        // Initialize AT_ptr[2..m+1] to 0
        for i in 1..=self.m {
            at.ptr[i as usize] = 0;
        }

        // Count non-zeros per row
        for k in 1..=self.n {
            let ptr = self.a_ptr[k as usize];
            let end = self.a_ptr[(k + 1) as usize];
            for p in ptr..end {
                let row = self.a_ind[p as usize];
                at.ptr[row as usize] += 1;
            }
        }

        // Compute prefix sum
        at.ptr[1] += 1;
        for i in 2..=self.m {
            at.ptr[i as usize] += at.ptr[(i - 1) as usize];
        }

        assert_eq!(at.ptr[self.m as usize], self.nnz + 1);
        at.ptr[(self.m + 1) as usize] = self.nnz + 1;

        // Fill AT_ind and AT_val
        for k in (1..=self.n).rev() {
            let ptr = self.a_ptr[k as usize];
            let end = self.a_ptr[(k + 1) as usize];
            for p in ptr..end {
                let row = self.a_ind[p as usize];
                at.ptr[row as usize] -= 1;
                let pos = at.ptr[row as usize];
                at.ind[pos as usize] = k;
                at.val[pos as usize] = self.a_val[p as usize];
            }
        }

        assert_eq!(at.ptr[1], 1);
    }

    pub fn at_prod(&self, at: &SPXAT, y: &mut [f64], s: f64, x: &[f64]) {
        for i in 1..=self.m {
            if x[i as usize] != 0.0 {
                let t = s * x[i as usize];
                let ptr = at.ptr[i as usize];
                let end = at.ptr[(i + 1) as usize];
                for p in ptr..end {
                    let col = at.ind[p as usize];
                    y[col as usize] += at.val[p as usize] * t;
                }
            }
        }
    }

    pub fn nt_prod1(&self, at: &mut SPXAT, y: &mut [f64], ign: i32, s: f64, x: &[f64]) {
        at.work.iter_mut().skip(1).take(self.n as usize).for_each(|v| *v = 0.0);

        if ign == 0 {
            for j in 1..=(self.n - self.m) {
                let k = self.head[(self.m + j) as usize];
                at.work[k as usize] = y[j as usize];
            }
        }

        self.at_prod(at, &mut at.work, s, x);

        for j in 1..=(self.n - self.m) {
            let k = self.head[(self.m + j) as usize];
            y[j as usize] = at.work[k as usize];
        }
    }

    pub fn eval_trow1(&self, at: &mut SPXAT, rho: &[f64], trow: &mut [f64]) {
        let nnz_rho = rho.iter().skip(1).take(self.m as usize).filter(|&&v| v != 0.0).count() as i32;
        let cnt1 = (self.n - self.m) as f64 * (self.nnz as f64 / self.n as f64);
        let cnt2 = nnz_rho as f64 * (self.nnz as f64 / self.m as f64);

        if cnt1 < cnt2 {
            for j in 1..=(self.n - self.m) {
                let k = self.head[(self.m + j) as usize];
                let mut tij = 0.0;
                let ptr = self.a_ptr[k as usize];
                let end = self.a_ptr[(k + 1) as usize];
                for p in ptr..end {
                    let row = self.a_ind[p as usize];
                    tij -= self.a_val[p as usize] * rho[row as usize];
                }
                trow[j as usize] = tij;
            }
        } else {
            self.nt_prod1(at, trow, 1, -1.0, rho);
        }
    }
}

impl Drop for SPXAT {
    fn drop(&mut self) {
        // Memory is automatically managed by Vec
    }
}