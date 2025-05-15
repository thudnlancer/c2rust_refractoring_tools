use std::fmt;
use std::error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Dom,
    Range,
    Failure,
    Eof,
    // ... other error variants
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GSL error occurred")
    }
}

impl error::Error for GslError {}

#[derive(Debug, Clone)]
pub struct Combination {
    n: usize,
    k: usize,
    data: Vec<usize>,
}

impl Combination {
    pub fn new(n: usize, k: usize) -> Result<Self, GslError> {
        if k > n {
            return Err(GslError::Failure);
        }
        Ok(Self {
            n,
            k,
            data: Vec::with_capacity(k),
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

    pub fn valid(&self) -> Result<(), GslError> {
        if self.k > self.n {
            return Err(GslError::Failure);
        }

        for i in 0..self.k {
            let ci = self.data[i];
            if ci >= self.n {
                return Err(GslError::Failure);
            }

            for j in 0..i {
                if self.data[j] == ci {
                    return Err(GslError::Failure);
                }
                if self.data[j] > ci {
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
        while i > 0 && self.data[i] == self.n - self.k + i {
            i -= 1;
        }

        if i == 0 && self.data[i] == self.n - self.k {
            return Err(GslError::Failure);
        }

        self.data[i] += 1;
        while i < self.k - 1 {
            self.data[i + 1] = self.data[i] + 1;
            i += 1;
        }

        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), GslError> {
        if self.k == 0 {
            return Err(GslError::Failure);
        }

        let mut i = self.k - 1;
        while i > 0 && self.data[i] == self.data[i - 1] + 1 {
            i -= 1;
        }

        if i == 0 && self.data[i] == 0 {
            return Err(GslError::Failure);
        }

        self.data[i] -= 1;
        i += 1;
        while i < self.k {
            self.data[i] = self.n - self.k + i;
            i += 1;
        }

        Ok(())
    }

    pub fn copy_from(&mut self, src: &Combination) -> Result<(), GslError> {
        if src.n != self.n || src.k != self.k {
            return Err(GslError::BadLen);
        }

        self.data.copy_from_slice(&src.data);
        Ok(())
    }
}