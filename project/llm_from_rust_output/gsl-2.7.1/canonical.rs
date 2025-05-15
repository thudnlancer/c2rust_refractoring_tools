use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug, Clone)]
pub struct Permutation {
    size: usize,
    data: Vec<usize>,
}

impl Permutation {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: (0..size).collect(),
        }
    }

    pub fn linear_to_canonical(&mut self, p: &Permutation) -> Result<(), GslError> {
        if self.size != p.size {
            return Err(GslError::Inval);
        }

        let n = p.size;
        let mut t = n;
        let mut i = 0;

        while i < n {
            let mut k = p.data[i];
            let mut s = 1;

            while k > i {
                k = p.data[k];
                s += 1;
            }

            if k >= i {
                t -= s;
                self.data[t] = i;
                k = p.data[i];
                s = 1;

                while k > i {
                    self.data[t + s] = k;
                    k = p.data[k];
                    s += 1;
                }

                if t == 0 {
                    break;
                }
            }
            i += 1;
        }

        Ok(())
    }

    pub fn canonical_to_linear(&mut self, q: &Permutation) -> Result<(), GslError> {
        if self.size != q.size {
            return Err(GslError::Inval);
        }

        let n = self.size;
        self.data = (0..n).collect();

        let mut k = q.data[0];
        let mut first = self.data[k];
        let mut i = 1;

        while i < n {
            let kk = q.data[i];
            if kk > first {
                self.data[k] = self.data[kk];
                k = kk;
            } else {
                self.data[k] = first;
                k = kk;
                first = self.data[kk];
            }
            i += 1;
        }

        self.data[k] = first;
        Ok(())
    }

    pub fn inversions(&self) -> usize {
        let mut count = 0;
        let size = self.size;

        for i in 0..size - 1 {
            for j in i + 1..size {
                if self.data[i] > self.data[j] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn linear_cycles(&self) -> usize {
        let mut count = 0;
        let size = self.size;
        let mut visited = vec![false; size];

        for i in 0..size {
            if !visited[i] {
                count += 1;
                let mut k = self.data[i];
                while k != i {
                    visited[k] = true;
                    k = self.data[k];
                }
            }
        }

        count
    }

    pub fn canonical_cycles(&self) -> usize {
        if self.size == 0 {
            return 0;
        }

        let mut count = 1;
        let mut min = self.data[0];

        for &val in &self.data {
            if val < min {
                min = val;
                count += 1;
            }
        }

        count
    }
}