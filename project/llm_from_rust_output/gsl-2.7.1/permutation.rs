use std::ptr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GslError {
    Success,
    Failure,
    Domain,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProg,
    NoProgJ,
    TolF,
    TolX,
    TolG,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Permutation {
    size: usize,
    data: Vec<usize>,
}

impl Permutation {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push(i);
        }
        Self { size, data }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [usize] {
        &mut self.data
    }

    pub fn swap(&mut self, i: usize, j: usize) -> Result<(), GslError> {
        if i >= self.size {
            return Err(GslError::Invalid);
        }
        if j >= self.size {
            return Err(GslError::Invalid);
        }
        if i != j {
            self.data.swap(i, j);
        }
        Ok(())
    }

    pub fn valid(&self) -> Result<(), GslError> {
        for i in 0..self.size {
            if self.data[i] >= self.size {
                return Err(GslError::Failure);
            }
            for j in 0..i {
                if self.data[i] == self.data[j] {
                    return Err(GslError::Failure);
                }
            }
        }
        Ok(())
    }

    pub fn reverse(&mut self) {
        for i in 0..self.size / 2 {
            let j = self.size - i - 1;
            self.data.swap(i, j);
        }
    }

    pub fn inverse(&mut self, p: &Permutation) -> Result<(), GslError> {
        if self.size != p.size {
            return Err(GslError::BadLen);
        }
        for i in 0..self.size {
            self.data[p.data[i]] = i;
        }
        Ok(())
    }

    pub fn next(&mut self) -> Result<(), GslError> {
        if self.size < 2 {
            return Err(GslError::Failure);
        }

        let mut i = self.size - 2;
        while i > 0 && self.data[i] > self.data[i + 1] {
            i -= 1;
        }

        if i == 0 && self.data[0] > self.data[1] {
            return Err(GslError::Failure);
        }

        let mut k = i + 1;
        for j in i + 2..self.size {
            if self.data[j] > self.data[i] && self.data[j] < self.data[k] {
                k = j;
            }
        }

        self.data.swap(i, k);

        let mut j = i + 1;
        while j <= (self.size + i) / 2 {
            let tmp = self.data[j];
            self.data[j] = self.data[self.size + i - j];
            self.data[self.size + i - j] = tmp;
            j += 1;
        }

        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), GslError> {
        if self.size < 2 {
            return Err(GslError::Failure);
        }

        let mut i = self.size - 2;
        while i > 0 && self.data[i] < self.data[i + 1] {
            i -= 1;
        }

        if i == 0 && self.data[0] < self.data[1] {
            return Err(GslError::Failure);
        }

        let mut k = i + 1;
        for j in i + 2..self.size {
            if self.data[j] < self.data[i] && self.data[j] > self.data[k] {
                k = j;
            }
        }

        self.data.swap(i, k);

        let mut j = i + 1;
        while j <= (self.size + i) / 2 {
            let tmp = self.data[j];
            self.data[j] = self.data[self.size + i - j];
            self.data[self.size + i - j] = tmp;
            j += 1;
        }

        Ok(())
    }

    pub fn mul(&mut self, pa: &Permutation, pb: &Permutation) -> Result<(), GslError> {
        if self.size != pa.size {
            return Err(GslError::Invalid);
        }
        if self.size != pb.size {
            return Err(GslError::Invalid);
        }

        for i in 0..self.size {
            self.data[i] = pb.data[pa.data[i]];
        }

        Ok(())
    }

    pub fn copy_from(&mut self, src: &Permutation) -> Result<(), GslError> {
        if self.size != src.size {
            return Err(GslError::BadLen);
        }

        self.data.copy_from_slice(&src.data);
        Ok(())
    }
}