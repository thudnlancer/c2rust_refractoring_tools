use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct GslMultiset {
    n: usize,
    k: usize,
    data: Vec<usize>,
}

impl GslMultiset {
    pub fn new(n: usize, k: usize) -> Result<Self, GslError> {
        if k > n {
            return Err(GslError::Invalid);
        }
        Ok(Self {
            n,
            k,
            data: vec![0; k],
        })
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [usize] {
        &mut self.data
    }

    pub fn valid(&self) -> Result<(), GslError> {
        for i in 0..self.k {
            if self.data[i] >= self.n {
                return Err(GslError::Failure);
            }
            for j in 0..i {
                if self.data[j] > self.data[i] {
                    return Err(GslError::Failure);
                }
            }
        }
        Ok(())
    }

    pub fn next(&mut self) -> Result<(), GslError> {
        if self.k == 0 {
            return Err(GslError::Failure);
        }

        let mut i = self.k - 1;
        while i > 0 && self.data[i] == self.n - 1 {
            i -= 1;
        }

        if i == 0 && self.data[0] == self.n - 1 {
            return Err(GslError::Failure);
        }

        self.data[i] += 1;
        for j in i + 1..self.k {
            self.data[j] = self.data[i];
        }

        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), GslError> {
        if self.k == 0 {
            return Err(GslError::Failure);
        }

        let mut i = self.k - 1;
        while i > 0 && self.data[i - 1] == self.data[i] {
            i -= 1;
        }

        if i == 0 && self.data[i] == 0 {
            return Err(GslError::Failure);
        }

        self.data[i] -= 1;
        if self.data[i] < self.n - 1 {
            for j in i + 1..self.k {
                self.data[j] = self.n - 1;
            }
        }

        Ok(())
    }

    pub fn copy_from(&mut self, other: &Self) -> Result<(), GslError> {
        if self.n != other.n || self.k != other.k {
            return Err(GslError::BadLen);
        }
        self.data.copy_from_slice(&other.data);
        Ok(())
    }
}